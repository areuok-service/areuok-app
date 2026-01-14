用户需求：
1. 可以切换模式：签到模式或监督签到模式
2. 绑定其他设备以监督指定设备
3. 同意被其他设备监督
这意味着需要设计一个设备管理系统，支持：
- 设备标识（device_id）
- 设备模式（签到模式/监督模式）
- 设备间关系（谁监督谁）
- 授权机制（同意被监督）

┌─────────────┐     HTTP API     ┌─────────────┐
│  客户端 A   │ ◄──────────────► │   服务器    │
│ (签到设备)  │                  │20.41.108.70 │
└─────────────┘                  └──────┬──────┘
                                        │
┌─────────────┐     HTTP API            │
│  客户端 B   │ ◄───────────────────────┘
│ (监督设备)  │
└─────────────┘
---
【数据结构设计】
服务器需要存储：
// 设备注册
Device {
    device_id: String,           // UUID
    device_name: String,         // 全局唯一
    imei: Option<String>,       // 可选，用于设备绑定
    mode: DeviceMode,            // Signin | Supervisor
    created_at: DateTime,
    last_seen_at: DateTime,
    last_name_updated_at: Option<DateTime>, // 昵称最后修改时间
}
// 监督请求
SupervisionRequest {
    request_id: String,
    supervisor_id: String,      // 发起监督的设备
    target_id: String,          // 被监督的设备
    status: Pending | Accepted | Rejected,
    created_at: DateTime,
}
// 签到记录（核心数据）
SigninRecord {
    device_id: String,
    date: Date,
    streak: i32,
}
---
【API 设计】
POST   /devices/register          # 注册/更新设备（支持 IMEI 绑定）
GET    /devices/{id}              # 获取设备信息
PATCH  /devices/{id}/name        # 更新设备昵称（15天限制）
GET    /search/devices             # 搜索设备（按昵称）
POST   /devices/{id}/signin       # 签到
GET    /devices/{id}/status       # 获取签到状态（供监督者查询）
POST   /supervision/request       # 发起监督请求
GET    /supervision/pending/{id}  # 获取待处理的请求（被监督者收）
POST   /supervision/accept        # 接受监督
POST   /supervision/reject        # 拒绝监督
GET    /supervision/list/{id}     # 获取监督关系列表
DELETE /supervision/{rel_id}      # 解除监督关系
---
【技术选型】
服务器端：
- Rust + Axum（你已经在用 Rust，保持统一）
- PostgreSQL（如果你服务器上已有）
---
【业务规则】

### 昵称管理
1. **昵称唯一性**
   - 所有设备的昵称必须全局唯一
   - 数据库使用 UNIQUE 约束强制执行
   - 注册和更新时都会验证昵称是否已存在

2. **昵称修改冷却期**
   - 昵称修改后15天内不得再次修改
   - 通过 `last_name_updated_at` 字段跟踪
   - 首次修改昵称无冷却期限制
   - 服务端验证：`(NOW() - last_name_updated_at) >= 15天`

3. **修改流程**
   ```
   用户请求修改昵称：
   ├─ 检查昵称是否已存在
   ├─ 检查 last_name_updated_at 是否为 NULL（首次修改）
   │  └─ 允许
   └─ 检查距离上次修改是否 >= 15天
      ├─ 允许修改并更新 last_name_updated_at
      └─ 拒绝并返回剩余冷却时间
   ```

### IMEI 设备绑定
1. **绑定机制**
   - IMEI 为可选字段，注册时可提供
   - 如果提供的 IMEI 已存在于系统，返回现有设备（设备恢复）
   - 如果 IMEI 不存在，创建新设备并绑定 IMEI

2. **设备恢复场景**
   ```
   场景：用户重装应用或更换设备
   1. 应用启动时获取设备 IMEI
   2. 注册时携带 IMEI 调用 /devices/register
   3. 服务器检查 IMEI 是否已绑定设备
   4. 如果已绑定：返回原设备信息（恢复账号）
   5. 如果未绑定：创建新设备（新账号）
   ```

3. **IMEI 唯一性**
   - IMEI 在系统中必须唯一（NULL 值除外）
   - 数据库 UNIQUE 约束强制执行
   - 一个 IMEI 只能绑定一个设备

### 数据库约束
```sql
-- 设备昵称唯一
ALTER TABLE devices ADD CONSTRAINT devices_device_name_key UNIQUE (device_name);

-- IMEI 唯一（允许多个 NULL）
ALTER TABLE devices ADD CONSTRAINT devices_imei_key UNIQUE (imei);

-- 追踪昵称修改时间
ALTER TABLE devices ADD COLUMN last_name_updated_at TIMESTAMPTZ;

-- IMEI 索引（用于快速查找）
CREATE INDEX idx_devices_imei ON devices(imei);
```

### 客户端实现
1. **获取 IMEI**
   - Tauri 后端提供 `get_device_imei` 命令
   - 首次注册时调用并传递到服务器

2. **昵称编辑 UI**
   - 仪表盘右上角添加昵称编辑按钮
   - 点击后打开编辑模态框
   - 显示当前昵称和修改冷却信息
   - 如果在冷却期，禁用编辑并显示剩余天数

3. **本地验证**
   - 客户端在请求前检查冷却期
   - 通过 `localStorage.get('areuok_last_name_update')` 获取上次修改时间
   - 计算天数差，提前提示用户

4. **更新后同步**
   - 成功修改后更新本地存储
   - 记录 `last_name_updated_at` 到 localStorage
   - 更新界面显示的昵称

# Are You OK? - 移动端签到程序

一款简洁清爽的移动端签到应用，支持连续签到统计、每日一言和邮件通知功能。

## 功能特性

- ✨ **清爽界面** - 现代化设计，柔和动画，暗色模式支持
- 🔥 **签到统计** - 自动计算连续签到天数
- 💬 **每日一言** - 签到后展示励志语录
- 📧 **邮件通知** - 签到成功后发送邮件到指定邮箱
- 📱 **移动端优化** - 完美适配手机屏幕
- 🔐 **设备绑定** - 设备昵称与 IMEI 绑定，支持设备恢复
- 🏷️ **昵称管理** - 昵称全局唯一，15天内仅可修改一次

## 配置说明

### 1. 配置一言API

在 `src-tauri/config.toml` 文件中配置一言API：

```toml
[hitokoto]
id = "你的ID"
key = "你的KEY"
```

获取API凭证：访问 [https://cn.apihz.cn](https://cn.apihz.cn) 注册获取ID和KEY

### 2. 配置邮件通知

首次启动应用后：
1. 点击右上角的设置图标 ⚙️
2. 启用邮件通知开关
3. 填写SMTP邮件配置：
   - **收件人邮箱** - 接收通知的邮箱地址
   - **SMTP服务器** - 邮件服务器地址（如：smtp.gmail.com）
   - **SMTP端口** - 邮件服务器端口（如：587）
   - **SMTP用户名** - 邮件发送账户
   - **SMTP密码** - 邮件发送密码（通常需要应用专用密码）
   - **发件人邮箱** - 显示在邮件中的发件人地址
4. 点击"保存配置"

### SMTP配置参考

**Gmail:**
- 服务器：smtp.gmail.com
- 端口：587
- 需要使用应用专用密码

**QQ邮箱:**
- 服务器：smtp.qq.com
- 端口：587
- 需要开启SMTP服务并获取授权码

**163邮箱:**
- 服务器：smtp.163.com
- 端口：25或465

## 开发

### 环境要求
- Node.js 18+
- Rust 1.70+
- npm 或 pnpm

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建

```bash
npm run build
```

## 技术栈

- **前端**: Svelte 5 + SvelteKit + TypeScript
- **后端**: Tauri 2 + Rust
- **存储**: 文件系统（~/.config/areuok/）
- **邮件**: lettre 0.11
- **HTTP**: reqwest 0.12

## 数据存储

应用数据存储在系统配置目录：
- **macOS**: `~/Library/Application Support/areuok/`
- **Linux**: `~/.config/areuok/`
- **Windows**: `%APPDATA%\areuok\`

### 存储数据结构

```json
{
  "device": {
    "device_id": "uuid",
    "device_name": "设备名称",
    "imei": "设备IMEI（可选）",
    "mode": "signin|supervisor",
    "created_at": "创建时间"
  },
  "supervision_requests": [...],
  "supervision_relationships": [...]
}
```

### 本地存储

浏览器 localStorage 存储：
- `areuok_device_id` - 设备ID
- `areuok_device_name` - 设备昵称
- `areuok_device_mode` - 设备模式
- `areuok_device_imei` - 设备IMEI
- `areuok_last_name_update` - 昵称最后修改时间（用于15天限制）
- `locale` - 界面语言设置

## 运行项目

```bash
# 开发模式
npm run tauri dev

# 构建生产版本
npm run build
```

## IDE推荐

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

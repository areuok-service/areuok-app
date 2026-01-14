# 编码规范 (Coding Standards)

本文档定义了 areuok 项目的编码规范和最佳实践。

## 目录

- [前端规范 (TypeScript/Svelte)](#前端规范-typescriptsvelte)
- [后端规范 (Rust)](#后端规范-rust)
- [通用规范](#通用规范)
- [Git 提交规范](git-提交规范)

---

## 前端规范 (TypeScript/Svelte)

### 代码风格

- **使用 2 空格缩进**（不要使用制表符）
- **使用单引号**（`'` 而不是 `"`）
- **行尾使用分号**
- **最大行宽 100 字符**
- **箭头函数参数始终使用括号**：`(x) => x + 1`

### TypeScript 规范

```typescript
// ✅ 好的做法
interface UserData {
  id: string;
  name: string;
  email?: string;
}

async function fetchUserData(id: string): Promise<UserData> {
  const response = await invoke<UserData>('get_user', { id });
  return response;
}

// ❌ 避免
async function fetchUserData(id: any) {  // 不要使用 any
  return await invoke('get_user', { id });  // 缺少类型注解
}
```

**规则：**
- 优先使用 `interface` 而不是 `type`（除非需要联合类型）
- 避免使用 `any`，使用 `unknown` 或具体类型
- 异步函数始终返回 `Promise<T>`
- 使用可选链 `?.` 和空值合并 `??`
- 未使用的变量前缀加下划线：`_unusedVar`

### Svelte 5 规范

```svelte
<!-- ✅ 好的做法 -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    subtitle?: string;
  }

  let { title, subtitle = '' }: Props = $props();

  let count = $state(0);
  let doubled = $derived(count * 2);

  function increment() {
    count += 1;
  }
</script>

<h1>{title}</h1>
{#if subtitle}
  <p>{subtitle}</p>
{/if}

<button onclick={increment}>Count: {count}</button>

<style>
  h1 {
    font-size: 2rem;
    color: var(--text-main);
  }
</style>
```

**规则：**
- 使用 Svelte 5 的 runes（`$state`, `$derived`, `$props`）
- 组件 Props 使用 TypeScript 接口定义
- 事件处理器使用 `onclick` 而不是 `on:click`
- 样式使用 CSS 变量（`:root`）
- 避免在 `<script>` 中直接操作 DOM

### 文件组织

```
src/
├── routes/           # SvelteKit 路由
│   ├── +page.svelte
│   └── +layout.svelte
├── lib/              # 可复用代码
│   ├── components/   # UI 组件
│   ├── api.ts        # API 类型定义
│   └── utils/        # 工具函数
└── lib/i18n/         # 国际化文件
```

---

## 后端规范 (Rust)

### 代码风格

- **使用 4 空格缩进**
- **最大行宽 100 字符**
- **使用rustfmt格式化所有代码**
- **使用 clippy 检查代码质量**

### 命名规范

```rust
// ✅ 好的做法
struct UserInfo {  // 结构体：PascalCase
    user_id: String,  // 字段：snake_case
}

impl UserInfo {
    fn new(user_id: String) -> Self {  // 函数：snake_case
        Self { user_id }
    }

    fn get_display_name(&self) -> &str {  // 方法：snake_case
        &self.user_id
    }
}

const MAX_RETRIES: u32 = 3;  // 常量：SCREAMING_SNAKE_CASE
```

**规则：**
- **类型/结构体/枚举**：`PascalCase`
- **函数/方法/变量**：`snake_case`
- **常量**：`SCREAMING_SNAKE_CASE`
- **生命周期参数**：短小写字母：`'a`, `'T`

### 错误处理

```rust
// ✅ 好的做法：使用 Result
#[tauri::command]
fn signin(name: String) -> Result<SigninData, String> {
    let data = load_data()
        .map_err(|e| format!("Failed to load data: {}", e))?;

    Ok(data)
}

// ✅ 好的做法：使用 ? 传播错误
async fn fetch_quote() -> Result<Quote, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let quote = response.json::<HitokotoResponse>()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    Ok(quote.into())
}

// ❌ 避免：unwrap() 在生产代码中
let data = some_operation().unwrap();  // 如果出错会 panic
```

### Tauri 命令规范

```rust
// ✅ 好的做法
#[tauri::command]
fn get_user_data(user_id: String) -> Result<UserData, String> {
    // 使用 user_id 而不是 id
    let user = fetch_user(&user_id)?;
    Ok(user)
}

#[tauri::command]
fn update_device_name(device_id: String, new_name: String) -> Result<Device, String> {
    // 参数名清晰描述用途
    // 使用 new_ 前缀表示新值
}
```

**规则：**
- 所有命令返回 `Result<T, String>`
- 错误消息提供上下文信息
- 参数名描述性强，避免缩写
- 异步命令使用 `async fn`

### 数据序列化

```rust
// ✅ 好的做法
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]  // JSON 字段使用 snake_case
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub created_at: String,  // ISO 8601 格式
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceMode {
    Signin,
    Supervisor,
}
```

---

## 通用规范

### 文件编码

- **UTF-8 编码**
- **LF 换行符**（不是 CRLF）
- **文件末尾添加换行符**

### 注释规范

```typescript
// TypeScript JSDoc 注释
/**
 * 用户签到
 * @param name - 用户名称
 * @returns 签到数据，包含连续签到天数
 */
async function signin(name: string): Promise<SigninData> {
  // ...
}
```

```rust
// Rust 文档注释
/// 用户签到
///
/// # 参数
///
/// * `name` - 用户名称
///
/// # 返回
///
/// 返回签到数据，包含连续签到天数
///
/// # 错误
///
/// 如果数据加载失败，返回错误消息
#[tauri::command]
fn signin(name: String) -> Result<SigninData, String> {
    // ...
}
```

### 国际化 (i18n)

- 所有用户可见文本必须使用 i18n
- 使用 `$_()` 函数翻译文本
- 翻译键使用点号分隔：`'app.title'`

```svelte
<h1>{$_('app.title')}</h1>
<p>{$_('dashboard.greeting', { values: { name: user.name } })}</p>
```

---

## Git 提交规范

### 提交消息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

- **feat**: 新功能
- **fix**: 修复 bug
- **docs**: 文档更新
- **style**: 代码格式（不影响功能）
- **refactor**: 重构
- **perf**: 性能优化
- **test**: 测试相关
- **chore**: 构建/工具链相关

### 示例

```bash
# 功能添加
git commit -m "feat(supervision): add device search functionality"

# Bug 修复
git commit -m "fix(auth): prevent duplicate sign-in on same day"

# 重构
git commit -m "refactor(rust): extract email sending to separate module"

# 文档
git commit -m "docs: update CLAUDE.md with architecture details"
```

---

## Lint 和格式化命令

### 前端

```bash
# 运行 ESLint 检查
pnpm lint

# 自动修复 ESLint 问题
pnpm lint:fix

# 格式化代码
pnpm format

# 检查格式
pnpm format:check

# 类型检查
pnpm check
```

### 后端

```bash
cd src-tauri

# 格式化代码
cargo fmt

# 检查格式（不修改文件）
cargo fmt -- --check

# 运行 clippy
cargo clippy

# 修复 clippy 警告
cargo clippy --fix

# 运行测试
cargo test
```

---

## 代码审查清单

提交代码前检查：

- [ ] 代码已通过 `pnpm lint` 和 `pnpm format`
- [ ] Rust 代码已通过 `cargo fmt` 和 `cargo clippy`
- [ ] 所有测试通过
- [ ] 添加了必要的类型注解
- [ ] 用户可见文本已翻译
- [ ] 错误处理完善（不使用 `unwrap()`）
- [ ] Git 提交消息符合规范
- [ ] 没有调试代码（`console.log`, `dbg!`, `println!`）

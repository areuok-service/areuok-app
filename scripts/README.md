# Lint 和格式化脚本使用说明

本项目配置了完整的代码规范检查工具，包括前端（TypeScript/Svelte）和后端（Rust）的 linting 和格式化。

## 安装依赖

首次使用前，需要安装所有开发依赖：

```bash
pnpm install
```

这将安装 ESLint、Prettier 和相关插件。

## VSCode 设置

安装推荐的 VSCode 扩展：

1. 打开 VSCode
2. 按 `Cmd+Shift+P` (macOS) 或 `Ctrl+Shift+P` (Windows/Linux)
3. 输入 "Extensions: Show Recommended Extensions"
4. 安装所有推荐的扩展：
   - ESLint
   - Prettier
   - Svelte for VSCode
   - Tauri VSCode
   - rust-analyzer

安装后，VSCode 将在保存时自动格式化代码并显示 lint 错误。

## 常用命令

### 前端

```bash
# 检查 ESLint 问题
pnpm lint

# 自动修复 ESLint 问题
pnpm lint:fix

# 格式化代码
pnpm format

# 检查格式（不修改文件）
pnpm format:check

# 类型检查
pnpm check
```

### 后端（在 src-tauri 目录中）

```bash
# 格式化代码
cargo fmt

# 检查格式（不修改文件）
cargo fmt -- --check

# 运行 Clippy linter
cargo clippy

# 自动修复 Clippy 警告
cargo clippy --fix

# 运行测试
cargo test
```

### 一键检查所有

```bash
# 运行所有 lint 检查（CI 环境）
bash scripts/check-all.sh
```

## 提交代码前检查

在提交代码前，确保运行以下命令：

```bash
# 前端：自动修复和格式化
pnpm lint:fix
pnpm format

# 后端：格式化和 clippy
cd src-tauri
cargo fmt
cargo clippy --fix
```

或者使用便捷脚本：

```bash
# 修复所有可以自动修复的问题
pnpm lint:fix && pnpm format && (cd src-tauri && cargo fmt && cargo clippy --fix)
```

## 配置文件说明

### 前端

- `.eslintrc.cjs` - ESLint 配置
- `.prettierrc` - Prettier 格式化配置
- `.prettierignore` - Prettier 忽略文件列表

### 后端

- `src-tauri/.rustfmt.toml` - Rust 格式化配置
- `src-tauri/clippy.toml` - Clippy linter 配置

### 编辑器

- `.editorconfig` - 跨编辑器通用配置
- `.vscode/settings.json` - VSCode 工作区设置

## CI/CD

项目使用 GitHub Actions 在 CI 中自动运行 lint 检查：

- `.github/workflows/lint.yml` - CI 工作流配置

每次 push 或创建 PR 时，会自动运行：
- ESLint 检查
- Prettier 格式检查
- TypeScript 类型检查
- Rust 格式检查
- Clippy 检查
- Rust 测试

## 详细编码规范

请参阅 [docs/CODING_STANDARDS.md](../docs/CODING_STANDARDS.md) 了解详细的编码规范说明。

## 常见问题

### ESLint 不工作

```bash
# 清理并重新安装依赖
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

### Prettier 格式化不生效

确保 VSCode 的默认格式化程序设置为 Prettier：
1. 打开任意 `.js`/`.ts`/`.svelte` 文件
2. 按 `Cmd+Shift+P` / `Ctrl+Shift+P`
3. 输入 "Format Document With..."
4. 选择 "Prettier"

### Rust 格式化不生效

确保安装了 `rustfmt`：
```bash
rustup component add rustfmt
```

确保安装了 `clippy`：
```bash
rustup component add clippy
```

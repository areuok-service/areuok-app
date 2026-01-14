# 移动端开发脚本

本目录包含用于移动端开发的辅助脚本。

## 脚本列表

### setup-mobile.sh
设置移动端开发环境的便捷脚本。

#### 功能
- 自动检测操作系统
- 安装 Android Rust 目标架构
- 在 macOS 上安装 iOS Rust 目标架构
- 检查必要的开发工具（Rust、Xcode、Android SDK）
- 提供下一步操作指引

#### 使用方法

```bash
# 从项目根目录运行
bash scripts/setup-mobile.sh
```

#### 输出示例

```
📱 设置 areuok 移动端开发环境
================================

检测到操作系统: Mac

✅ Rust 版本: rustc 1.75.0

📦 安装 Android 目标...
info: installing component 'aarch64-linux-android'
info: installing component 'armv7-linux-androideabi'
info: installing component 'x86_64-linux-android'
✅ Android 目标安装完成

📦 安装 iOS 目标...
info: installing component 'aarch64-apple-ios'
info: installing component 'aarch64-apple-ios-sim'
✅ iOS 目标安装完成

✅ Xcode 版本: Xcode 15.0

🎉 移动端开发环境设置完成！
```

## 手动设置步骤

如果你不想使用脚本，可以手动执行以下步骤：

### Android

```bash
# 1. 安装 Rust 目标
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# 2. 设置环境变量（添加到 ~/.bashrc 或 ~/.zshrc）
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.1.10909125
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools

# 3. 验证安装
pnpm tauri android --help
```

### iOS（仅 macOS）

```bash
# 1. 安装 Rust 目标
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim

# 2. 初始化 iOS 项目
pnpm tauri ios init

# 3. 在 Xcode 中打开并配置
open src-tauri/gen/apple/areuok.xcworkspace

# 4. 验证安装
pnpm tauri ios --help
```

## 常见问题

### Q: 脚本运行失败
**A**: 确保你有 `rustup` 安装。如果没有，访问 https://rustup.rs/

### Q: Android SDK 未找到
**A**: 需要手动安装 Android Studio 或 Android SDK 命令行工具

### Q: iOS 开发只能在 macOS 上吗？
**A**: 是的，iOS 开发必须使用 macOS 和 Xcode

## 相关文档

- [MOBILE_BUILD_GUIDE.md](../docs/MOBILE_BUILD_GUIDE.md) - 详细的移动端构建指南
- [MOBILE_SETUP_SUMMARY.md](../docs/MOBILE_SETUP_SUMMARY.md) - 移动端设置总结
- [CLAUDE.md](../CLAUDE.md) - 项目文档

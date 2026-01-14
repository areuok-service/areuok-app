# 移动端构建指南

本文档说明如何为 Android 和 iOS 平台构建 areuok 应用。

> **重要提示**: Tauri 2 的移动端支持目前处于**稳定**状态，但需要额外的设置步骤。

## 目录

- [准备工作](#准备工作)
- [Android 构建指南](#android-构建指南)
- [iOS 构建指南](#ios-构建指南)
- [自动构建 (CI/CD)](#自动构建-cicd)
- [常见问题](#常见问题)

---

## 准备工作

### 系统要求

#### Android
- **开发环境**：Linux、macOS 或 Windows
- **Java**：JDK 17 或更高版本
- **Android SDK**：API Level 24+
- **Android NDK**：r26b 或更高版本
- **Rust**：稳定版本

#### iOS
- **开发环境**：macOS（必需）
- **Xcode**：15.0 或更高版本
- **Rust**：稳定版本
- **iOS SDK**：iOS 13.0+
- **Apple Developer Account**（用于签名和发布）

### 验证 Tauri 安装

首先确保 Tauri CLI 已正确安装：

```bash
pnpm tauri --version
# 应该输出: tauri-cli 2.x.x

pnpm tauri info
# 查看完整的环境信息
```

---

## Android 构建指南

### 1. 安装 Android SDK 和 NDK

#### macOS/Linux

```bash
# 使用 Android Studio 或命令行工具
# 安装 Android SDK Command-line Tools

# 设置环境变量（添加到 ~/.bashrc 或 ~/.zshrc）
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.1.10909125
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools

# 验证安装
adb --version
```

#### Windows

使用 Android Studio 安装：
1. 下载并安装 [Android Studio](https://developer.android.com/studio)
2. 在安装向导中选择：
   - Android SDK
   - Android SDK Platform-Tools
   - Android NDK (版本 26.1.10909125 或更高)

### 2. 安装 Rust 目标架构

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
```

### 3. 初始化 Android 项目

```bash
# 在项目根目录运行
pnpm tauri android init
```

这将在 `src-tauri/gen/android` 创建 Android 项目结构。

### 4. 开发和构建

```bash
# 在连接的设备或模拟器上运行应用
pnpm tauri android dev

# 构建调试版 APK
pnpm tauri android build --debug

# 构建发布版 APK/AAB
pnpm tauri android build --release
```

### 5. 构建产物位置

- **APK**: `src-tauri/gen/android/app/build/outputs/apk/`
- **AAB**: `src-tauri/gen/android/app/build/outputs/bundle/`

### 6. 签名配置（发布必需）

要发布到 Google Play，需要签名 APK/AAB：

#### 生成密钥库

```bash
keytool -genkey -v -keystore areuok-release.keystore -keyalg RSA -keysize 2048 -validity 10000 -alias areuok
```

#### 在 Gradle 中配置签名

编辑 `src-tauri/gen/android/app/build.gradle`:

```gradle
android {
    signingConfigs {
        release {
            storeFile file("../../areuok-release.keystore")
            storePassword "your-store-password"
            keyAlias "areuok"
            keyPassword "your-key-password"
        }
    }
    buildTypes {
        release {
            signingConfig signingConfigs.release
        }
    }
}
```

---

## iOS 构建指南

> **注意**: iOS 构建必须在 macOS 上进行

### 1. 安装 Xcode

从 App Store 安装 Xcode 15.0 或更高版本。

```bash
# 安装 Xcode 命令行工具
xcode-select --install

# 验证安装
xcodebuild -version
```

### 2. 安装 Rust 目标

```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

### 3. 安装 ios-deploy

```bash
npm install -g ios-deploy
```

### 4. 初始化 iOS 项目

```bash
# 在项目根目录运行
pnpm tauri ios init
```

这将创建 `src-tauri/gen/apple` 目录和 Xcode 工作空间。

### 5. 在 Xcode 中打开项目

```bash
# 打开 Xcode 工作空间
open src-tauri/gen/apple/areuok.xcworkspace
```

### 6. 配置签名和权限

在 Xcode 中：
1. 选择项目 → "Signing & Capabilities"
2. 选择你的开发团队（Team）
3. 确保 Bundle Identifier 唯一（例如：com.peng.areuok）
4. 添加必要的权限：
   - Notifications（通知权限）
   - 其他需要的权限

### 7. 开发和构建

```bash
# 在模拟器或连接的设备上运行
pnpm tauri ios dev

# 构建调试版
pnpm tauri ios build --debug

# 构建发布版
pnpm tauri ios build --release
```

### 8. 构建产物位置

- **IPA**: `src-tauri/gen/apple/build/`

---

## 自动构建 (CI/CD)

项目使用 GitHub Actions 自动构建移动端应用。

### 工作流文件

- **`.github/workflows/build-android.yml`** - Android APK/AAB 构建
- **`.github/workflows/build-ios.yml`** - iOS IPA 构建

### 触发构建

构建会在以下情况自动触发：

1. **推送到主分支**: `main` 或 `develop`
2. **创建 Pull Request**
3. **创建版本标签**: `git tag v1.0.0 && git push --tags`
4. **手动触发**: 在 GitHub Actions 页面点击 "Run workflow"

### 下载构建产物

1. 访问 GitHub Actions 页面
2. 选择对应的工作流运行
3. 在 "Artifacts" 部分下载：
   - Android: APK 和 AAB 文件
   - iOS: IPA 文件

---

## 常见问题

### Android

#### Q: `error: Android SDK not found`
**A**: 设置 `ANDROID_HOME` 环境变量指向 Android SDK 安装路径。

#### Q: `error: linker command failed`
**A**: 确保 ANDROID_NDK_ROOT 环境变量正确设置，并安装 NDK r26b 或更高版本。

#### Q: `pnpm tauri android init` 失败
**A**: 确保已安装所有必要的 Rust 目标和 Android SDK。

#### Q: 应用在设备上崩溃
**A**: 使用 `adb logcat` 查看日志：
```bash
adb logcat | grep areuok
```

### iOS

#### Q: `Code signing error`
**A**: 在 Xcode 中配置正确的开发者证书和 Provisioning Profile。

#### Q: 真机无法安装
**A**: 确保：
- 有效的 Apple Developer Account
- 设备已注册到开发者账号
- Provisioning Profile 包含该设备

#### Q: 模拟器无法运行
**A**: 确保安装了对应的 iOS SDK 版本。

#### Q: `pnpm tauri ios init` 失败
**A**: 确保在 macOS 上运行，并已安装 Xcode 命令行工具。

### 通用问题

#### Q: 移动端和桌面端代码不同步
**A**: 清理并重新构建：
```bash
pnpm tauri android clean
# 或
pnpm tauri ios clean

# 重新构建前端
pnpm build
```

#### Q: 热重载不工作
**A**: 移动端不支持 HMR（热模块替换），需要完全重新构建应用。

---

## 发布到应用商店

### Google Play（Android）

1. 构建 AAB 文件（Android App Bundle）：
   ```bash
   pnpm tauri android build --release
   ```

2. 访问 [Google Play Console](https://play.google.com/console)

3. 创建应用并上传 AAB 文件

4. 填写应用信息：
   - 应用名称
   - 描述
   - 截图
   - 图标

5. 配置定价和分发

6. 提交审核

### App Store（iOS）

1. 构建并签名 IPA：
   ```bash
   pnpm tauri ios build --release
   ```

2. 使用 [Transporter](https://apps.apple.com/app/transporter/id1450874784) 上传 IPA

3. 访问 [App Store Connect](https://appstoreconnect.apple.com)

4. 配置应用：
   - 应用信息
   - 定价和销售范围
   - 截图和预览
   - 审核信息

5. 提交审核

---

## 性能优化建议

### Android

1. **减小 APK 大小**：
   - 启用 APK 拆分
   - 压缩资源
   - 使用 ProGuard/R8

2. **优化启动时间**：
   - 延迟初始化非关键组件
   - 优化 Rust 代码启动逻辑

### iOS

1. **优化启动时间**：
   - 减少 `main.rs` 中的初始化工作
   - 使用懒加载

2. **减小应用大小**：
   - 启用 App Thinning
   - 优化资源文件

3. **电池优化**：
   - 避免频繁的后台任务
   - 使用合适的网络请求策略

---

## 相关资源

- [Tauri 移动端文档](https://v2.tauri.app/zh-cn/develop/)
- [Tauri Android 指南](https://v2.tauri.app/zh-cn/develop/android/)
- [Tauri iOS 指南](https://v2.tauri.app/zh-cn/develop/ios/)
- [Android 开发者指南](https://developer.android.com/guide)
- [iOS 人机界面指南](https://developer.apple.com/design/human-interface-guidelines/)
- [CLAUDE.md](../CLAUDE.md) - 项目架构文档
- [CODING_STANDARDS.md](./CODING_STANDARDS.md) - 编码规范

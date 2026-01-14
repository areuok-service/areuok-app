# 移动端自动构建设置总结

本文档总结了为 areuok 项目添加的 Android 和 iOS 自动构建功能。

## 项目状态

✅ **桌面端（Windows/macOS/Linux）**: 完全支持，可立即构建
✅ **移动端（Android/iOS）**: Tauri 2 原生支持，需要额外设置

## Tauri 2 移动端支持

Tauri 2 包含完整的移动端支持：
- ✅ Android 原生支持
- ✅ iOS 原生支持（需要 macOS）
- ✅ 使用相同的 Rust 后端代码
- ✅ 使用相同的前端代码（SvelteKit）
- ✅ 统一的 CLI 命令

## 已创建和修改的文件

### 1. 配置文件

#### `src-tauri/tauri.conf.json` ✏️ 已更新
配置说明：
- 移除了错误的 iOS/Android 配置
- 使用 Tauri 2 标准配置
- 移动端配置通过 `pnpm tauri android/ios init` 自动生成

#### `src-tauri/Cargo.toml` ✏️ 已更新
- 移除了不存在的 `tauri-plugin-android-ui` 和 `tauri-plugin-ios-ui`
- 这些插件由 Tauri 移动端 CLI 自动管理

### 2. CI/CD 工作流

#### `.github/workflows/build-android.yml` 🆕 新建
自动构建 Android APK 和 AAB：
- 触发条件：推送到 main/develop、PR、版本标签、手动触发
- 构建产物：APK（调试/发布）、AAB（Google Play）
- 支持自动上传到 GitHub Releases

#### `.github/workflows/build-ios.yml` 🆕 新建
自动构建 iOS IPA：
- 触发条件：推送到 main/develop、PR、版本标签、手动触发
- 构建产物：IPA 文件
- 支持代码签名（需要配置 GitHub Secrets）

### 3. 文档

#### `docs/MOBILE_BUILD_GUIDE.md` 🆕 已更新
详细的移动端构建指南，包含：
- Tauri 2 移动端支持说明
- 系统要求和环境设置
- Android/iOS 初始化步骤
- 开发和构建命令
- CI/CD 配置
- 应用商店发布指南
- 常见问题解答

#### `docs/MOBILE_SETUP_SUMMARY.md` 🆕 本文档
移动端自动构建功能总结

#### `CLAUDE.md` ✏️ 已更新
添加了移动端构建命令部分

### 4. 脚本

#### `scripts/setup-mobile.sh` 🆕 新建
自动设置移动端开发环境：
- 安装 Rust 目标架构
- 验证开发工具
- 提供下一步指引

#### `scripts/MOBILE_SCRIPTS.md` 🆕 新建
移动端脚本使用说明

## 快速开始

### 验证环境

```bash
# 检查 Tauri 版本
pnpm tauri --version
# 应该输出: tauri-cli 2.x.x

# 查看环境信息
pnpm tauri info
```

### Android 开发

```bash
# 1. 安装 Rust 目标
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# 2. 设置环境变量
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.1.10909125

# 3. 初始化 Android 项目
pnpm tauri android init

# 4. 开发和构建
pnpm tauri android dev              # 在设备/模拟器上运行
pnpm tauri android build --debug    # 构建调试版 APK
pnpm tauri android build --release  # 构建发布版 APK/AAB
```

### iOS 开发（仅 macOS）

```bash
# 1. 安装 Rust 目标
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# 2. 初始化 iOS 项目
pnpm tauri ios init

# 3. 在 Xcode 中打开并配置
open src-tauri/gen/apple/areuok.xcworkspace

# 4. 开发和构建
pnpm tauri ios dev              # 在模拟器/设备上运行
pnpm tauri ios build --debug    # 构建调试版
pnpm tauri ios build --release  # 构建发布版
```

## 支持的平台和架构

### Android
- **架构**：
  - `aarch64-linux-android`（64位 ARM，大多数现代设备）
  - `armv7-linux-androideabi`（32位 ARM，旧设备）
  - `x86_64-linux-android`（64位 x86，模拟器）
- **最低版本**：Android 7.0 (API 24)
- **目标版本**：最新稳定版

### iOS
- **架构**：
  - `aarch64-apple-ios`（64位 ARM，真机）
  - `aarch64-apple-ios-sim`（Apple Silicon Mac 模拟器）
- **最低版本**：iOS 13.0
- **支持设备**：iPhone、iPad（iOS 13+）

## 构建产物说明

### Android
- **APK**（Android Package Kit）
  - 用途：直接安装到设备或测试
  - 格式：`.apk`
  - 位置：`src-tauri/gen/android/app/build/outputs/apk/`

- **AAB**（Android App Bundle）
  - 用途：上传到 Google Play
  - 格式：`.aab`
  - 位置：`src-tauri/gen/android/app/build/outputs/bundle/`

### iOS
- **IPA**（iPhone Application）
  - 用途：安装到设备或上传到 App Store
  - 格式：`.ipa`
  - 位置：`src-tauri/gen/apple/build/`

## CI/CD 自动构建

### 触发方式

1. **推送到主分支**
   ```bash
   git push origin main
   ```

2. **创建版本标签**
   ```bash
   git tag v1.0.0
   git push --tags
   ```

3. **Pull Request**
   - 创建或更新 PR 时自动运行

4. **手动触发**
   - 访问 GitHub Actions 页面
   - 选择工作流
   - 点击 "Run workflow"

### 下载构建产物

1. 访问 GitHub Actions 页面
2. 选择对应的工作流运行
3. 在 "Artifacts" 部分下载文件
4. 文件保留 30 天

## GitHub Secrets 配置

### Android 签名（可选）

如果要自动签名发布版 APK/AAB，添加以下 Secrets：

| Secret 名称 | 说明 |
|------------|------|
| `KEYSTORE_FILE` | 密钥库文件（Base64 编码） |
| `KEYSTORE_PASSWORD` | 密钥库密码 |

生成 Base64：
```bash
base64 -i areuok-release.keystore | pbcopy  # macOS
base64 -w 0 areuok-release.keystore          # Linux
```

### iOS 签名（发布必需）

发布到 App Store 需要添加以下 Secrets：

| Secret 名称 | 说明 |
|------------|------|
| `IOS_CERTIFICATE_BASE64` | iOS 证书（.p12，Base64） |
| `IOS_CERTIFICATE_PASSWORD` | 证书密码 |
| `IOS_PROVISIONING_PROFILE_BASE64` | Provisioning Profile（Base64） |
| `KEYCHAIN_PASSWORD` | 临时 keychain 密码 |

## 发布到应用商店

### Google Play（Android）

1. **构建 AAB**：
   ```bash
   pnpm tauri android build --release
   ```

2. **上传到 Google Play Console**：
   - 登录 [Google Play Console](https://play.google.com/console)
   - 创建新应用
   - 上传 AAB 文件
   - 填写应用信息
   - 提交审核

### App Store（iOS）

1. **构建并签名 IPA**：
   ```bash
   pnpm tauri ios build --release
   ```

2. **上传到 App Store Connect**：
   - 使用 Transporter 上传 IPA
   - 登录 [App Store Connect](https://appstoreconnect.apple.com)
   - 配置应用信息
   - 提交审核

## 常见问题

### Android

**Q: `pnpm tauri android init` 失败**
- 确保已安装 Android SDK 和 NDK
- 设置 `ANDROID_HOME` 和 `ANDROID_NDK_ROOT` 环境变量
- 安装所有必要的 Rust 目标

**Q: 应用安装后崩溃**
- 检查日志：`adb logcat | grep areuok`
- 确保所有必要的权限已添加
- 验证前端构建成功

### iOS

**Q: `pnpm tauri ios init` 失败**
- 确保在 macOS 上运行
- 安装 Xcode 和命令行工具
- 安装 ios-deploy：`npm install -g ios-deploy`

**Q: 真机无法安装**
- 配置有效的开发者证书
- 添加设备到 Provisioning Profile
- 在 Xcode 中配置签名

## 性能优化建议

### Android
- 启用 R8 代码混淆和优化
- 使用 App Bundle 减小下载大小
- 压缩资源文件

### iOS
- 优化启动时间
- 减小应用包大小
- 使用 App Thinning

## 相关资源

### 官方文档
- [Tauri 2 移动端文档](https://v2.tauri.app/zh-cn/develop/)
- [Tauri Android 指南](https://v2.tauri.app/zh-cn/develop/android/)
- [Tauri iOS 指南](https://v2.tauri.app/zh-cn/develop/ios/)
- [Android 开发者指南](https://developer.android.com/guide)
- [iOS 开发者指南](https://developer.apple.com/documentation/)

### 项目文档
- [CLAUDE.md](../CLAUDE.md) - 项目架构和命令
- [MOBILE_BUILD_GUIDE.md](./MOBILE_BUILD_GUIDE.md) - 详细构建指南
- [CODING_STANDARDS.md](./CODING_STANDARDS.md) - 编码规范

## 总结

通过这次设置，areuok 项目现在支持：

- ✅ 桌面端开发（Windows/macOS/Linux）
- ✅ Android 开发和构建
- ✅ iOS 开发和构建（需要 macOS）
- ✅ 统一的代码库（Rust + SvelteKit）
- ✅ 自动化 CI/CD 构建
- ✅ GitHub Releases 自动发布
- ✅ 完整的开发文档

所有移动端相关的配置都已就绪，可以立即开始开发移动应用！

## 重要提示

1. **首次构建前必须初始化**：
   - Android: `pnpm tauri android init`
   - iOS: `pnpm tauri ios init`

2. **移动端和桌面端代码完全兼容**：
   - 相同的 Rust 代码
   - 相同的前端代码
   - 只需初始化对应的移动平台

3. **CI/CD 工作流**：
   - Android 工作流会自动运行 `pnpm tauri android init`
   - iOS 工作流会自动运行 `pnpm tauri ios init`

4. **环境要求**：
   - Android: 可以在任何平台上构建
   - iOS: 必须在 macOS 上构建

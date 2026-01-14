#!/bin/bash
set -e

echo "📱 设置 areuok 移动端开发环境"
echo "================================"
echo ""

# 检测操作系统
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    MINGW*|MSYS*|CYGWIN*)    MACHINE=Windows;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo "检测到操作系统: $MACHINE"
echo ""

# 检查 Rust 是否安装
if ! command -v rustup &> /dev/null; then
    echo "❌ 错误: 未找到 Rust。请访问 https://rustup.rs/ 安装。"
    exit 1
fi

echo "✅ Rust 版本: $(rustc --version)"
echo ""

# 安装 Android 目标
echo "📦 安装 Android 目标..."
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
echo "✅ Android 目标安装完成"
echo ""

# 如果是 macOS，也安装 iOS 目标
if [ "$MACHINE" = "Mac" ]; then
    echo "📦 安装 iOS 目标..."
    rustup target add aarch64-apple-ios
    rustup target add aarch64-apple-ios-sim
    echo "✅ iOS 目标安装完成"
    echo ""

    # 检查 Xcode
    if command -v xcodebuild &> /dev/null; then
        echo "✅ Xcode 版本: $(xcodebuild -version | head -n 1)"
    else
        echo "⚠️  警告: 未找到 Xcode。iOS 开发需要 Xcode。"
    fi
fi

# 检查 Android SDK
if [ -n "$ANDROID_HOME" ]; then
    echo "✅ ANDROID_HOME: $ANDROID_HOME"
else
    echo "⚠️  警告: ANDROID_HOME 环境变量未设置。"
    echo ""
    echo "请设置以下环境变量："
    echo "  export ANDROID_HOME=\$HOME/Android/Sdk"
    echo "  export ANDROID_NDK_ROOT=\$ANDROID_HOME/ndk/26.1.10909125"
    echo "  export PATH=\$PATH:\$ANDROID_HOME/cmdline-tools/latest/bin:\$ANDROID_HOME/platform-tools"
fi

echo ""
echo "🎉 移动端开发环境设置完成！"
echo ""
echo "下一步："
echo ""
echo "Android 开发:"
echo "  1. 确保 Android SDK 和 NDK 已安装"
echo "  2. 运行: pnpm tauri android dev"
echo ""
if [ "$MACHINE" = "Mac" ]; then
    echo "iOS 开发:"
    echo "  1. 初始化 iOS 项目: pnpm tauri ios init"
    echo "  2. 在 Xcode 中打开: open src-tauri/gen/apple/areuok.xcworkspace"
    echo "  3. 运行: pnpm tauri ios dev"
    echo ""
fi
echo "详细文档: docs/MOBILE_BUILD_GUIDE.md"

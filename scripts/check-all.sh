#!/bin/bash
set -e

echo "🔍 Running all lint checks..."

echo ""
echo "📦 Frontend lint..."
pnpm lint

echo ""
echo "🎨 Frontend format check..."
pnpm format:check

echo ""
echo "🔷 Frontend type check..."
pnpm check

echo ""
echo "🦀 Rust format check..."
cd src-tauri
cargo fmt -- --check

echo ""
echo "🔍 Rust clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "✅ All checks passed!"

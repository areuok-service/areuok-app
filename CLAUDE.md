# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **areuok** - a desktop check-in/habit tracking application with multi-device supervision capabilities. Built with Tauri 2, Svelte 5, and Rust.

### Key Features
- Daily sign-in tracking with streak counter
- Multi-device supervision: devices can supervise each other's check-in status
- Email notifications on successful check-ins with daily quotes
- System notifications using Tauri's official notification plugin
- Toast messages for error handling and user feedback
- Bilingual support (English and Simplified Chinese)
- Desktop app using Tauri with local data storage

## Development Commands

### Frontend (SvelteKit)
```bash
# Start development server
pnpm dev

# Build for production
pnpm build

# Type checking
pnpm check

# Type checking with watch mode
pnpm check:watch

# Preview production build
pnpm preview
```

### Tauri (Rust Backend)
```bash
# Run Tauri dev (starts frontend dev server automatically)
pnpm tauri dev

# Build Tauri app for production
pnpm tauri build

# Tauri CLI commands
pnpm tauri <command>
```

### Rust (Backend)
```bash
# Navigate to Rust source
cd src-tauri

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run linter
cargo clippy

# Fix linter warnings automatically
cargo clippy --fix
```

### Code Quality & Linting
```bash
# Frontend linting
pnpm lint              # Check for issues
pnpm lint:fix          # Auto-fix issues
pnpm format            # Format code with Prettier
pnpm format:check      # Check formatting without modifying

# Type checking
pnpm check             # Run svelte-check
pnpm check:watch       # Watch mode

# Rust backend (from src-tauri/)
cargo fmt              # Format Rust code
cargo clippy           # Run Rust linter
```

**Important**: Always run `pnpm lint:fix` and `cargo fmt` before committing code.

### Mobile Builds (Android/iOS)

This project supports building for Android and iOS platforms using Tauri 2.

#### Android Development
```bash
# Install Android targets first
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Set up environment (Linux/macOS)
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.1.10909125
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools

# Run on connected device/emulator
pnpm tauri android dev

# Build APK (debug)
pnpm tauri android build --debug

# Build APK (release)
pnpm tauri android build --release
```

#### iOS Development (macOS only)
```bash
# Install iOS targets first
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

# Initialize iOS project (first time only)
pnpm tauri ios init

# Run on simulator or connected device
pnpm tauri ios dev

# Build IPA (debug)
pnpm tauri ios build --debug

# Build IPA (release)
pnpm tauri ios build --release
```

**Note**: See [docs/MOBILE_BUILD_GUIDE.md](docs/MOBILE_BUILD_GUIDE.md) for detailed mobile build instructions, including signing, CI/CD setup, and troubleshooting.


## Architecture

### Frontend-Backend Communication

This is a **Tauri desktop application**, not a web app:
- Frontend (SvelteKit) calls Rust backend via `invoke()` from `@tauri-apps/api/core`
- Backend commands are defined in `src-tauri/src/lib.rs` with `#[tauri::command]`
- Commands are registered in the `invoke_handler!` macro

**Important**: The files in `src/lib/api.ts` (deviceApi, supervisionApi) use `fetch()` to call a hypothetical REST API. This is **legacy/unused code** - the actual implementation uses Tauri's `invoke()` directly in `src/routes/+page.svelte`.

### Data Storage

All data is stored locally in JSON files (platform-specific config directory):

- **`data.json`** - User's sign-in data (name, last sign-in date, streak count, history)
- **`device_config.json`** - Device info, supervision requests, and relationships
- **`email_config.json`** - Email settings for notifications

Location: `~/Library/Application Support/areuok/` on macOS (platform-specific via `dirs` crate)

### Core Data Structures (Rust)

**SigninData**: User's check-in history
- `name`, `last_signin_date`, `streak`, `signin_history`

**DeviceInfo**: Device identity
- `device_id` (UUID), `device_name`, `mode` (Signin/Supervisor), `created_at`

**SupervisionRequest**: Pending supervision requests
- `request_id`, `supervisor_device_id`, `target_device_id`, `status`

**SupervisionRelationship**: Established supervision connections
- `relationship_id`, `supervisor_device_id`, `supervised_device_id`

**DeviceStatus**: For supervisors to view supervised devices
- `device_id`, `last_signin_date`, `streak`, `is_signed_in_today`

### Tauri Commands (Backend API)

All commands are in `src-tauri/src/lib.rs`. Key ones:

**User Operations:**
- `load_signin_data()` - Load current sign-in data
- `signin(name)` - Perform daily sign-in, updates streak
- `signout()` - Clear all data

**Device Management:**
- `get_device_config()` - Get or create device config
- `set_device_mode(mode)` - Switch between Signin/Supervisor modes
- `update_device_name(name)` - Change device name

**Supervision:**
- `send_supervision_request(target_device_id)` - Request to supervise a device
- `get_pending_supervision_requests()` - Get incoming requests
- `accept_supervision_request(request_id)` - Accept supervision
- `reject_supervision_request(request_id)` - Reject supervision
- `get_supervisor_status()` - Get supervised devices status
- `remove_supervision_relationship(relationship_id)` - End supervision

**Extras:**
- `get_daily_quote()` - Fetch quote from hitokoto.cn API
- `get_email_config()` / `save_email_config_command(config)` - Email settings

### Frontend Structure

**Svelte 5 with runes (`$state`, `$derived`)** - not Svelte 4 stores

- `src/routes/+page.svelte` - Main app logic (login, dashboard, supervision)
- `src/routes/+layout.svelte` - App layout wrapper
- `src/routes/+layout.ts` - Initializes i18n locale from localStorage
- `src/lib/i18n.ts` - i18n configuration with svelte-i18n
- `src/lib/i18n/` - Translation files (en.json, zh-CN.json)

### Key Dependencies

**Rust (Cargo.toml):**
- `tauri` v2 - Desktop app framework
- `lettre` - Email sending (SMTP)
- `reqwest` - HTTP client (for quotes API)
- `chrono` - Date/time handling
- `serde`/`serde_json` - JSON serialization
- `dirs` - Platform config directories
- `uuid` - Device ID generation

**Frontend (package.json):**
- `@tauri-apps/api` v2 - Tauri frontend APIs
- `@tauri-apps/plugin-notification` - Desktop notifications
- `svelte` v5 - UI framework
- `svelte-i18n` - Internationalization
- `@sveltejs/adapter-static` - Static site generation

### External APIs

- **hitokoto.cn** (`https://v1.hitokoto.cn/`) - Free Chinese quote API for daily inspiration

### Email Feature

Optional email notifications on sign-in:
- Uses SMTP (lettre crate)
- Supports TLS (STARTTLS)
- Default: Gmail (smtp.gmail.com:587)
- User must configure SMTP credentials in app settings
- Only sends when `enabled: true` in email config

### Toast & Notification System

The application includes a comprehensive message notification system:

**Toast Messages (Frontend):**
- Location: `src/lib/components/Toast.svelte`, `src/lib/stores/toast.ts`
- Displays temporary pop-up messages for user feedback
- Supports multiple toast types: success, error, info, warning
- Auto-dismiss after configurable duration (default 3000ms)
- Uses Svelte 5 runes (`$state`, `$derived`)
- Integrated throughout the application for error handling and success feedback

**System Notifications (Backend):**
- Location: `src/lib/services/notification.ts` (frontend), `src-tauri/src/commands.rs` (backend)
- Uses Tauri's official `@tauri-apps/plugin-notification` v2
- Sends native OS notifications (desktop and mobile)
- Automatically requests notification permissions on first use
- Tauri command: `send_notification_command(title, body)`

**Usage Examples:**
```typescript
// Frontend - Show toast
import { toastStore } from '$lib/stores/toast';
toastStore.success('操作成功');
toastStore.error('操作失败');
toastStore.info('提示信息');
toastStore.warning('警告信息');

// Frontend - Send system notification
import { notificationService } from '$lib/services/notification';
await notificationService.success('签到成功', '你已经连续签到7天');
await notificationService.error('签到失败', '请稍后重试');

// Backend - Send notification via Tauri
await invoke('send_notification_command', {
  title: '签到成功',
  body: '你已经连续签到7天'
});
```

**Localization:**
All toast and notification messages are i18n-ready with translations in:
- `src/lib/i18n/en.json` - English messages
- `src/lib/i18n/zh-CN.json` - Chinese messages

Key i18n keys:
- `error.*` - Error messages (registrationFailed, signinFailed, etc.)
- `success.*` - Success messages (nameUpdated, requestAccepted, etc.)
- `notification.*` - Notification permission messages

## Important Notes

- Device IDs are auto-generated UUIDs stored in device_config.json
- Supervision requests are one-way: supervisor sends request, target accepts/rejects
- Streak logic: consecutive days only, resets if a day is missed
- The app currently has no actual network sync between devices - all supervision data is local-only
- The REST API code in `src/lib/api.ts` is not currently used

## Coding Standards

This project follows strict coding standards and linting rules. See [docs/CODING_STANDARDS.md](docs/CODING_STANDARDS.md) for detailed guidelines.

### Quick Reference

**Frontend (TypeScript/Svelte):**
- 2 spaces, single quotes, semicolons
- Max line width: 100 characters
- Use Svelte 5 runes (`$state`, `$derived`, `$props`)
- Avoid `any` type, use `unknown` instead
- Use `$_()` for all user-visible text (i18n)

**Backend (Rust):**
- 4 spaces, snake_case for functions/variables
- Max line width: 100 characters
- Use `Result<T, String>` for error handling
- Avoid `.unwrap()`, use `?` operator
- Add `#[serde(rename_all = "snake_case")]` for API types

**Before committing:**
```bash
# Frontend
pnpm lint:fix
pnpm format

# Backend
cd src-tauri && cargo fmt && cargo clippy --fix
```

# Are U OK? (areuok) - Open Source DAMUMU Alternative

English | [简体中文](./README.md)

A clean and elegant cross-platform check-in application with streak tracking, multi-device supervision, daily quotes, and email notifications.

> 🎯 **areuok** is an open-source alternative to the "DAMUMU" (formerly known as "死了么" / "Are You Dead?") App, providing complete offline functionality with optional cloud service integration for multi-device sync and supervision.

## Features

- ✨ **Clean UI** - Modern design with smooth animations and dark mode support
- 🔥 **Streak Tracking** - Automatic consecutive check-in counting
- 💬 **Daily Quotes** - Inspirational quotes displayed after check-in
- 📧 **Email Notifications** - Send email notifications on successful check-in
- 📱 **Cross-Platform** - Supports macOS, Windows, Linux, Android, and iOS
- 🔐 **Device Binding** - Device nickname bound to IMEI for device recovery
- 🏷️ **Nickname Management** - Globally unique nicknames with 15-day change cooldown
- 👀 **Multi-Device Supervision** - Devices can supervise each other's check-in status (requires cloud service)
- ☁️ **Local + Cloud** - Works fully offline or with optional cloud sync

## Architecture

This project uses a **local client + optional cloud service** architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                    areuok Client                            │
│         (Tauri 2 + SvelteKit + Rust)                        │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   macOS     │    │   Windows   │    │   Linux     │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│  ┌─────────────┐    ┌─────────────┐                        │
│  │   Android   │    │     iOS     │                        │
│  └─────────────┘    └─────────────┘                        │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTP API (optional)
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                 areuok-server Cloud Service                 │
│              (Rust + Axum + PostgreSQL)                     │
│                                                             │
│  • Device registration        • Check-in data sync         │
│  • Supervision relationships  • Streak calculation         │
│  • IMEI device binding        • RESTful API                │
└─────────────────────────────────────────────────────────────┘
```

### Usage Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| **Offline Mode** | All data stored locally, no internet required | Personal use, privacy-focused |
| **Cloud Sync Mode** | Connect to cloud service for multi-device sync | Multi-device use, supervision features |

### Related Repositories

| Repository | Description |
|------------|-------------|
| [areuok](https://github.com/nicepeng/areuok) | 📱 Client application (this repo) |
| [areuok-server](https://github.com/nicepeng/areuok-server) | ☁️ Cloud server |

## Cloud Service Deployment

To use multi-device supervision features, deploy your own areuok-server:

### Quick Deployment (Docker)

```bash
# Clone the server repository
git clone https://github.com/nicepeng/areuok-server.git
cd areuok-server

# One-click start (includes PostgreSQL + server)
./start-docker.sh

# Server will run at http://localhost:3000
```

### Server API Overview

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/devices/register` | POST | Register new device |
| `/devices/{id}` | GET | Get device information |
| `/devices/{id}/signin` | POST | Device check-in |
| `/devices/{id}/status` | GET | Get check-in status |
| `/supervision/request` | POST | Create supervision request |
| `/supervision/accept` | POST | Accept supervision request |
| `/supervision/list/{id}` | GET | Get supervision relationships |

For detailed API documentation, see [areuok-server docs](https://github.com/nicepeng/areuok-server#api-documentation).

### Client Cloud Configuration

To configure the cloud service in the client app:

1. Open app settings
2. Find "Cloud Service" configuration
3. Enter your server address (e.g., `https://your-server.com`)
4. Save and test connection

## Configuration

### 1. Daily Quote API

Configure the quote API in `src-tauri/config.toml`:

```toml
[hitokoto]
id = "your-id"
key = "your-key"
```

Get API credentials: Visit [https://cn.apihz.cn](https://cn.apihz.cn) to register

### 2. Email Notifications

After first launch:
1. Click the settings icon ⚙️ in the top right
2. Enable email notifications
3. Fill in SMTP configuration:
   - **Recipient Email** - Email address to receive notifications
   - **SMTP Server** - Mail server address (e.g., smtp.gmail.com)
   - **SMTP Port** - Mail server port (e.g., 587)
   - **SMTP Username** - Email account for sending
   - **SMTP Password** - Email password (usually requires app-specific password)
   - **Sender Email** - Sender address shown in emails
4. Click "Save Configuration"

### SMTP Configuration Reference

**Gmail:**
- Server: smtp.gmail.com
- Port: 587
- Requires app-specific password

**Outlook/Hotmail:**
- Server: smtp-mail.outlook.com
- Port: 587

**Yahoo:**
- Server: smtp.mail.yahoo.com
- Port: 587

## Development

### Requirements
- Node.js 18+
- Rust 1.70+
- npm or pnpm

### Install Dependencies

```bash
npm install
```

### Development Mode

```bash
npm run tauri dev
```

### Build

```bash
npm run build
```

## Tech Stack

- **Frontend**: Svelte 5 + SvelteKit + TypeScript
- **Backend**: Tauri 2 + Rust
- **Storage**: File system (~/.config/areuok/)
- **Email**: lettre 0.11
- **HTTP**: reqwest 0.12

## Data Storage

App data is stored in system config directory:
- **macOS**: `~/Library/Application Support/areuok/`
- **Linux**: `~/.config/areuok/`
- **Windows**: `%APPDATA%\areuok\`

### Data Structure

```json
{
  "device": {
    "device_id": "uuid",
    "device_name": "Device Name",
    "imei": "Device IMEI (optional)",
    "mode": "signin|supervisor",
    "created_at": "Creation time"
  },
  "supervision_requests": [...],
  "supervision_relationships": [...]
}
```

### Local Storage

Browser localStorage:
- `areuok_device_id` - Device ID
- `areuok_device_name` - Device nickname
- `areuok_device_mode` - Device mode
- `areuok_device_imei` - Device IMEI
- `areuok_last_name_update` - Last nickname update time (for 15-day limit)
- `locale` - UI language setting

## Running the Project

```bash
# Development mode
npm run tauri dev

# Build for production
npm run build
```

## Recommended IDE

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Documentation

- [Contributing Guide](./CONTRIBUTING.md)
- [Changelog](./docs/CHANGELOG.md)
- [License](./LICENSE)
- [Mobile Build Guide](./docs/MOBILE_BUILD_GUIDE.md)
- [Coding Standards](./docs/CODING_STANDARDS.md)

## License

This project is licensed under the [GNU General Public License v2.0 (GPLv2)](./LICENSE).

## Acknowledgements

### Inspiration

This project is an open-source alternative to "DAMUMU" (formerly known as "死了么" / "Are You Dead?" in Chinese communities). Thanks to the original creator for the creative inspiration!

### Tech Stack Credits

This project wouldn't be possible without these amazing open-source projects:

**Core Frameworks**
- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop and mobile apps with web technologies
- [Svelte](https://svelte.dev/) & [SvelteKit](https://kit.svelte.dev/) - Compile-time frontend framework
- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software

**Rust Ecosystem**
- [lettre](https://github.com/lettre/lettre) - Email client for Rust
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for Rust
- [serde](https://serde.rs/) - Serialization/deserialization framework for Rust
- [chrono](https://github.com/chronotope/chrono) - Date and time library for Rust

**Frontend Tools**
- [TypeScript](https://www.typescriptlang.org/) - JavaScript with syntax for types
- [svelte-i18n](https://github.com/kaisermann/svelte-i18n) - Internationalization library for Svelte
- [Vite](https://vitejs.dev/) - Next generation frontend tooling

**External Services**
- [Hitokoto API](https://hitokoto.cn/) - Provides daily inspirational quotes

Thanks to all open-source contributors for their hard work!

## Contributing

Issues and Pull Requests are welcome! Please read the [Contributing Guide](./CONTRIBUTING.md) first.

# TopDock - Setup Guide

## Prerequisites

### 1. Install Rust
Download dan install Rust dari: https://rustup.rs/

Untuk Windows, download `rustup-init.exe` dan jalankan.

Setelah instalasi, restart terminal dan verifikasi:
```powershell
cargo --version
rustc --version
```

### 2. Install Build Tools
TopDock memerlukan Windows SDK dan MSVC build tools.

Install **Visual Studio Build Tools** dari:
https://visualstudio.microsoft.com/downloads/

Pilih workload:
- Desktop development with C++
- Windows 10/11 SDK

## Quick Start

Setelah Rust terinstall, jalankan:

```powershell
# Initialize project
cargo init --name topdock

# Build project
cargo build

# Run project
cargo run
```

## Project Structure

```
topdock/
├── Cargo.toml              # Dependencies dan konfigurasi
├── Cargo.lock              # Lock file dependencies
├── src/
│   ├── main.rs            # Entry point
│   ├── app.rs             # Main application logic
│   ├── config/            # Configuration management
│   ├── ui/                # UI components dan rendering
│   ├── platform/          # Windows-specific code
│   ├── features/          # Feature modules (launcher, todo, etc)
│   ├── storage/           # Database dan persistence
│   └── services/          # Background services
├── migrations/            # Database migrations
├── assets/                # Icons, images, resources
├── tests/                 # Integration tests
└── README.md
```

## Next Steps

Setelah setup selesai, lanjut ke implementasi window management dan UI.

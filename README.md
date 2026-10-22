# TopDock

> Dynamic Island untuk Windows - Floating productivity bar yang kecil, ringkas, dan powerful

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 📖 Tentang

TopDock adalah aplikasi desktop Windows native berbasis Rust yang menyediakan floating bar di bagian atas layar. Bar ini berfungsi sebagai pusat akses cepat untuk aplikasi, file, musik, notifikasi, pengingat, dan daftar tugas.

**Konsep:** Terinspirasi dari Dynamic Island - antarmuka tetap kecil saat tidak digunakan, kemudian melebar atau menampilkan panel kontekstual ketika ada aktivitas atau interaksi pengguna.

## ✨ Fitur

### MVP (P0 - Prioritas Tertinggi)
- ✅ **Floating Bar Native** - Window transparan tanpa dekorasi di bagian atas layar
- ⚡ **Mode Compact & Expanded** - Transisi smooth antara mode idle dan aktif
- 🎨 **Hover Magnification** - Animasi halus saat interaksi
- 🚀 **Launcher** - Quick access untuk aplikasi, file, dan folder
- 📂 **Drag & Drop** - Seret file dari File Explorer dengan menu aksi
- 🎵 **Media Control** - Deteksi dan kontrol musik via Windows Media Session
- ✅ **To-Do List** - Daftar tugas dengan penyimpanan lokal
- ⏰ **Reminders** - Pengingat dengan snooze dan notifikasi
- 🔔 **System Tray** - Minimize ke tray dan startup options

### Roadmap (P1)
- 🎤 **Lirik Tersinkronisasi** - Tampilan lirik real-time dari sumber eksternal
- 📢 **Notifikasi Pihak Ketiga** - Integrasi notifikasi Windows
- ⚙️ **Settings Lengkap** - Customization UI, privasi, dan behavior
- 🖥️ **Multi-Monitor** - Dukungan beberapa monitor

### Future (P2)
- 📊 **Riwayat Aktivitas** - Log notifikasi dan reminder
- ☁️ **Cloud Sync** - Sinkronisasi data antar perangkat

## 🛠️ Teknologi

- **Bahasa:** Rust 2021 Edition
- **Window Management:** winit
- **Rendering:** wgpu
- **Database:** SQLite (rusqlite)
- **Windows API:** windows-rs
- **Async Runtime:** tokio

## 🚀 Quick Start

### Option A: Build di Debian Server (Recommended)

**✨ Tidak perlu install Rust di Windows!**

```bash
# 1. SSH ke server Debian
ssh user@your-server

# 2. Clone repository
git clone https://github.com/yourusername/topdock.git
cd topdock

# 3. Setup build environment (run once)
make setup
# atau: chmod +x build-server-setup.sh && ./build-server-setup.sh

# 4. Build untuk Windows
make build
# atau: ./build.sh

# 5. Transfer ke Windows
make deploy IP=192.168.1.100
# atau: scp target/x86_64-pc-windows-gnu/release/topdock.exe user@windows:~/Desktop/

# 6. Di Windows, double-click topdock.exe untuk run
```

📖 **Panduan lengkap:** [BUILD_SERVER.md](BUILD_SERVER.md)

### Option B: Build di Windows

1. **Prerequisites**
   - Rust - Install dari [rustup.rs](https://rustup.rs/)
   - Visual Studio Build Tools

2. **Build**
   ```powershell
   # Clone repository
   git clone https://github.com/yourusername/topdock.git
   cd topdock

   # Build project
   cargo build --release

   # Run
   cargo run --release
   ```

Lihat [SETUP.md](SETUP.md) untuk panduan lengkap.

### Development Commands

```bash
# Build
make build              # Release build
make build-debug        # Debug build (faster)
make quick             # Quick build untuk testing

# Testing
make test              # Run tests
make check             # Lint dengan clippy
make fmt               # Format code

# Deploy
make deploy IP=192.168.1.100

# Clean
make clean             # Hapus build artifacts
make rebuild           # Clean + build
```

## 📁 Struktur Project

```
topdock/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application logic
│   ├── config/              # Configuration management
│   ├── ui/                  # UI state dan rendering
│   ├── platform/windows/    # Windows-specific APIs
│   ├── features/            # Feature modules
│   │   ├── launcher/
│   │   ├── music/
│   │   ├── todo/
│   │   ├── reminders/
│   │   ├── lyrics/
│   │   └── notifications/
│   ├── storage/             # Database dan persistence
│   └── services/            # Background services
├── migrations/              # Database migrations
├── assets/                  # Resources (icons, images)
├── tests/                   # Integration tests
└── Cargo.toml              # Dependencies
```

## ⚙️ Konfigurasi

File konfigurasi disimpan di: `%USERPROFILE%\.topdock\config.toml`

```toml
compact_width = 200
compact_height = 40
expanded_width = 400
expanded_height = 300
top_margin = 10
alignment = "center"  # "left", "center", "right"
animations_enabled = true
animation_duration = 200
always_on_top = true
start_on_login = false
theme = "dark"  # "dark", "light", "system"
monitor_index = 0
```

## 🎯 Prinsip Desain

- **Compact by default** - Bar sekecil mungkin saat idle
- **Expand on intent** - Membesar saat cursor mendekat atau ada event
- **Context-aware** - Tampilan berubah sesuai aktivitas
- **Keyboard-friendly** - Shortcut keyboard untuk aksi penting
- **Non-destructive** - Operasi aman, tidak merusak data sumber
- **Privacy-first** - Kontrol penuh atas data sensitif
- **Local-first** - Data tersimpan lokal secara default

## 🗺️ Development Roadmap

- [x] Phase 1: Setup Project & Basic Window
- [ ] Phase 2: Floating Bar & Window Management
- [ ] Phase 3: UI Framework & Animation
- [ ] Phase 4: Database & Storage
- [ ] Phase 5: Launcher & Drag-Drop
- [ ] Phase 6: Media Integration
- [ ] Phase 7: To-Do List
- [ ] Phase 8: Reminders & Scheduler
- [ ] Phase 9: Settings & Configuration
- [ ] Phase 10: Lyrics Integration (P1)
- [ ] Phase 11: Notifications (P1)
- [ ] Phase 12: Polish & Testing

## 📝 License

MIT License - lihat [LICENSE](LICENSE) untuk detail

## 🤝 Contributing

Contributions welcome! Silakan baca [CONTRIBUTING.md](CONTRIBUTING.md) untuk panduan kontribusi.

## 📧 Kontak

- GitHub Issues: [Report bugs atau request features](https://github.com/yourusername/topdock/issues)
- Documentation: Lihat [topdock-prd.md](topdock-prd.md) untuk spesifikasi lengkap

---

**Status:** 🚧 In Active Development - Phase 1 Complete

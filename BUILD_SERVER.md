# Build Server Setup Guide

## 🎯 Tujuan
Build TopDock di server Debian, transfer `.exe` ke Windows untuk dijalankan. **Tidak perlu install Rust di Windows**.

---

## 📋 Prerequisites

- Server Debian/Ubuntu dengan sudo access
- SSH access ke server
- (Opsional) SSH access dari server ke Windows untuk auto-deploy

---

## 🚀 Setup Build Server

### 1. Clone Repository di Server

```bash
# SSH ke server Debian
ssh user@your-debian-server

# Clone repository
cd ~
git clone https://github.com/yourusername/topdock.git
# atau jika sudah clone: git pull origin main

cd topdock
```

### 2. Run Setup Script

```bash
# Make executable
chmod +x build-server-setup.sh

# Run setup (akan install Rust, MinGW, dll)
./build-server-setup.sh
```

Script ini akan:
- ✅ Update system packages
- ✅ Install build tools (gcc, make, dll)
- ✅ Install MinGW untuk cross-compile ke Windows
- ✅ Install Rust via rustup
- ✅ Add Windows target (x86_64-pc-windows-gnu)
- ✅ Configure cargo untuk cross-compilation

**⏱️ Waktu: ~5-10 menit** (tergantung koneksi internet)

### 3. Verify Setup

```bash
# Check Rust version
rustc --version
# Output: rustc 1.xx.x

# Check cargo
cargo --version
# Output: cargo 1.xx.x

# Check MinGW
x86_64-w64-mingw32-gcc --version
# Output: x86_64-w64-mingw32-gcc 10.x.x

# Check Windows target
rustup target list | grep windows-gnu
# Output: x86_64-pc-windows-gnu (installed)
```

---

## 🔨 Build Process

### Option A: Simple Build

```bash
# Make executable
chmod +x build.sh

# Build release version
./build.sh

# Build debug version (lebih cepat, file lebih besar)
./build.sh debug

# Clean build (hapus cache dulu)
./build.sh clean
```

Output: `target/x86_64-pc-windows-gnu/release/topdock.exe`

### Option B: Manual Build

```bash
# Release build (optimized, kecil)
cargo build --release --target x86_64-pc-windows-gnu

# Debug build (cepat, besar, untuk development)
cargo build --target x86_64-pc-windows-gnu

# Check output
ls -lh target/x86_64-pc-windows-gnu/release/topdock.exe
```

---

## 📦 Transfer ke Windows

### Option 1: Auto Deploy (Recommended)

```bash
# Setup SSH di Windows dulu (lihat section Windows SSH Setup)

# Deploy
chmod +x deploy.sh
./deploy.sh 192.168.1.100              # dengan username saat ini
./deploy.sh 192.168.1.100 Administrator # dengan username spesifik
```

### Option 2: Manual SCP

```bash
# Copy ke Windows
scp target/x86_64-pc-windows-gnu/release/topdock.exe user@windows-ip:~/Desktop/

# Contoh:
scp target/x86_64-pc-windows-gnu/release/topdock.exe john@192.168.1.100:~/Desktop/
```

### Option 3: Download via Web

```bash
# Serve file via HTTP (simple)
cd target/x86_64-pc-windows-gnu/release
python3 -m http.server 8000

# Di Windows, buka browser:
# http://server-ip:8000/
# Download topdock.exe
```

### Option 4: Shared Folder / SMB

Jika server dan Windows ada di network yang sama:

```bash
# Mount Windows share di Linux
sudo apt install cifs-utils
sudo mount -t cifs //windows-ip/SharedFolder /mnt/windows -o username=user

# Copy
cp target/x86_64-pc-windows-gnu/release/topdock.exe /mnt/windows/
```

---

## 🪟 Windows SSH Setup (untuk Auto Deploy)

Install OpenSSH Server di Windows:

### Via PowerShell (Windows 10/11):

```powershell
# Run as Administrator
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0

# Start service
Start-Service sshd

# Set to start automatically
Set-Service -Name sshd -StartupType 'Automatic'

# Allow firewall
New-NetFirewallRule -Name sshd -DisplayName 'OpenSSH Server (sshd)' -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22
```

### Test dari Server:

```bash
# Test SSH connection
ssh your-windows-username@windows-ip

# Jika berhasil, setup SSH key untuk auto-login
ssh-copy-id your-windows-username@windows-ip
```

---

## 🔄 Development Workflow

### 1. Edit Code di Windows

```bash
# Di Windows (VSCode, Notepad++, dll)
# Edit files di d:\dock-island\
```

### 2. Commit & Push

```powershell
# Di Windows
cd d:\dock-island
git add .
git commit -m "Add new feature"
git push origin main
```

### 3. Build di Server

```bash
# Di server Debian
cd ~/topdock
git pull origin main
./build.sh
```

### 4. Deploy ke Windows

```bash
# Auto deploy
./deploy.sh 192.168.1.100

# Atau manual copy (lihat Option 2-4 di atas)
```

### 5. Test di Windows

```
Double-click topdock.exe
Test fitur baru
```

---

## 📊 Build Times & Sizes

**First Build:**
- Debug: ~5-10 menit, ~15-20 MB
- Release: ~10-20 menit, ~5-8 MB

**Incremental Build** (setelah edit kecil):
- Debug: ~30 detik - 2 menit
- Release: ~1-5 menit

**Tips untuk build lebih cepat:**
```bash
# Install sccache (cache compiler)
cargo install sccache

# Set environment variable
echo 'export RUSTC_WRAPPER=sccache' >> ~/.bashrc
source ~/.bashrc

# Parallel jobs
export CARGO_BUILD_JOBS=4
```

---

## 🐛 Troubleshooting

### Error: "linker `x86_64-w64-mingw32-gcc` not found"

```bash
sudo apt install mingw-w64
```

### Error: "target may not be installed"

```bash
rustup target add x86_64-pc-windows-gnu
```

### Build error: "cannot find -lwindows"

Windows crate issue. Fix:
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --target x86_64-pc-windows-gnu
```

### exe crashes di Windows

Mungkin missing DLL. Build static:
```bash
# Edit Cargo.toml, tambahkan:
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"

# Rebuild
cargo clean
./build.sh
```

### Permission denied saat transfer

```bash
# Check SSH
ssh user@windows-ip "echo test"

# Check target directory writable
ssh user@windows-ip "mkdir -p ~/Desktop && echo test > ~/Desktop/test.txt"
```

---

## 🎨 IDE Setup (Optional)

### VSCode dengan Remote SSH

1. Install extension "Remote - SSH"
2. Connect ke server Debian
3. Open folder: `~/topdock`
4. Install Rust extension di remote
5. Edit dan build langsung dari VSCode

### Build dari VSCode:

```json
// .vscode/tasks.json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Build Windows Release",
      "type": "shell",
      "command": "./build.sh",
      "group": {
        "kind": "build",
        "isDefault": true
      }
    },
    {
      "label": "Deploy to Windows",
      "type": "shell",
      "command": "./deploy.sh",
      "args": ["${input:windowsIP}"]
    }
  ],
  "inputs": [
    {
      "id": "windowsIP",
      "type": "promptString",
      "description": "Windows IP address"
    }
  ]
}
```

---

## 🔐 Security Notes

1. **SSH Keys**: Setup SSH key authentication, jangan simpan password di script
2. **Git Credentials**: Use SSH keys atau personal access tokens
3. **Secrets**: Jangan commit API keys ke repository
4. **Firewall**: Only open SSH port (22) yang diperlukan

---

## 📚 Useful Commands

```bash
# Check exe info
file target/x86_64-pc-windows-gnu/release/topdock.exe

# List dependencies
x86_64-w64-mingw32-objdump -x topdock.exe | grep DLL

# Strip debug symbols (reduce size)
x86_64-w64-mingw32-strip topdock.exe

# Compress exe
upx --best topdock.exe  # Install: apt install upx-ucl
```

---

## ✅ Quick Reference

```bash
# Setup (run once)
./build-server-setup.sh

# Daily workflow
git pull                    # Update code
./build.sh                  # Build exe
./deploy.sh 192.168.1.100   # Deploy to Windows

# Or manual
cargo build --release --target x86_64-pc-windows-gnu
scp target/x86_64-pc-windows-gnu/release/topdock.exe user@windows:/destination/
```

---

**Next:** Setelah setup selesai, lanjut implementasi Phase 2! 🚀

# 🚀 Quick Start Guide

## TL;DR

```bash
# Di server Debian:
make setup && make build && make deploy IP=192.168.1.100

# Di Windows:
# Double-click topdock.exe → Done! ✅
```

---

## Setup Lengkap (15 menit)

### 1️⃣ Di Server Debian

```bash
# Clone repo
git clone <your-repo-url>
cd topdock

# Setup environment (auto-install Rust, MinGW, dll)
chmod +x build-server-setup.sh
./build-server-setup.sh

# Tunggu ~5-10 menit...
# ✅ Setup complete!
```

### 2️⃣ Build

```bash
# Build release version
make build

# Output: target/x86_64-pc-windows-gnu/release/topdock.exe
# Size: ~5-8 MB
# Time: ~10-20 menit (first build), ~1-5 menit (incremental)
```

### 3️⃣ Transfer ke Windows

**Option A: Auto Deploy** (Butuh SSH di Windows)
```bash
make deploy IP=192.168.1.100
# atau dengan username:
./deploy.sh 192.168.1.100 YourUsername
```

**Option B: Manual SCP**
```bash
scp target/x86_64-pc-windows-gnu/release/topdock.exe user@192.168.1.100:~/Desktop/
```

**Option C: HTTP Download**
```bash
cd target/x86_64-pc-windows-gnu/release
python3 -m http.server 8000
# Buka di Windows: http://server-ip:8000/topdock.exe
```

### 4️⃣ Run di Windows

```
1. Buka File Explorer
2. Navigate ke Desktop (atau tempat kamu copy)
3. Double-click topdock.exe
4. Window kecil muncul di top-center screen
5. Press ESC untuk exit (untuk sementara)
```

---

## Daily Workflow

### Edit → Build → Deploy → Test

```bash
# 1. Edit code di Windows (VSCode, dll)
# Edit files...

# 2. Commit & push
git add .
git commit -m "Update feature"
git push

# 3. Build di server
ssh user@server
cd ~/topdock
git pull
make build

# 4. Deploy
make deploy IP=192.168.1.100

# 5. Test di Windows
# Double-click exe baru
```

---

## Makefile Commands

```bash
make help          # Show all commands
make setup         # Setup environment (once)
make build         # Build release
make build-debug   # Build debug (faster)
make clean         # Clean artifacts
make deploy IP=x   # Deploy to Windows
make test          # Run tests
make check         # Lint with clippy
make fmt           # Format code
make rebuild       # Clean + build
make info          # Show build info
```

---

## Manual Build (tanpa Makefile)

```bash
# Full command
cargo build --release --target x86_64-pc-windows-gnu

# Debug build (lebih cepat)
cargo build --target x86_64-pc-windows-gnu

# Clean
cargo clean

# Run tests
cargo test --target x86_64-pc-windows-gnu
```

---

## Verify Setup

```bash
# Check Rust
rustc --version
cargo --version

# Check Windows target
rustup target list | grep windows-gnu
# Should show: x86_64-pc-windows-gnu (installed)

# Check MinGW
x86_64-w64-mingw32-gcc --version

# Test build
make build
# atau: ./build.sh
```

---

## Troubleshooting

### ❌ "rustc: command not found"
```bash
# Run setup script
./build-server-setup.sh

# Or reload environment
source $HOME/.cargo/env
```

### ❌ "linker x86_64-w64-mingw32-gcc not found"
```bash
sudo apt install mingw-w64
```

### ❌ "target may not be installed"
```bash
rustup target add x86_64-pc-windows-gnu
```

### ❌ exe tidak jalan di Windows
```bash
# Rebuild dengan optimasi berbeda
cargo clean
cargo build --release --target x86_64-pc-windows-gnu

# Atau check dependencies
x86_64-w64-mingw32-objdump -x topdock.exe | grep DLL
```

### ❌ SSH ke Windows gagal
```powershell
# Di Windows (Run as Admin):
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
Start-Service sshd
Set-Service -Name sshd -StartupType 'Automatic'
```

---

## Tips & Tricks

### 🚀 Faster Builds
```bash
# Install sccache (compiler cache)
cargo install sccache
echo 'export RUSTC_WRAPPER=sccache' >> ~/.bashrc
source ~/.bashrc
```

### 📦 Smaller exe
```bash
# Strip debug symbols
x86_64-w64-mingw32-strip topdock.exe

# Compress with UPX
sudo apt install upx-ucl
upx --best topdock.exe
```

### 🔄 Auto Build on Git Push
Setup GitHub Actions atau GitLab CI (lihat `.github/workflows/build.yml`)

### 📝 VSCode Remote
```bash
# Install "Remote - SSH" extension
# Connect to server
# Edit & build directly
```

---

## What's Next?

✅ **Setup done!** Sekarang kamu bisa:

1. **Start Phase 2**: Implement window management
   ```bash
   # Lihat DEVELOPMENT.md untuk checklist
   ```

2. **Setup CI/CD**: Auto-build on push
   ```bash
   # Push to GitHub → Auto-build → Download artifact
   ```

3. **Development**: Edit code & iterate
   ```bash
   # Edit → commit → build → deploy → test
   ```

---

**Questions?** Check [BUILD_SERVER.md](BUILD_SERVER.md) untuk detail lengkap!

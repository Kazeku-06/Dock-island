# TopDock Build Options

## 🎯 Pilihan Build Method

Ada **4 cara** untuk build TopDock tanpa install Rust di Windows:

| Method | Pros | Cons | Setup Time |
|--------|------|------|------------|
| **1. Native Cross-Compile** | Paling cepat (incremental builds) | Perlu setup Rust di server | ~10 menit |
| **2. Docker** | Tidak perlu install Rust sama sekali | Build lebih lambat (no cache) | ~2 menit |
| **3. GitHub Actions** | Auto-build on push, zero setup | Perlu repo public/private GitHub | ~5 menit |
| **4. GitLab CI** | Self-hosted, full control | Perlu GitLab instance | ~10 menit |

---

## 1️⃣ Native Cross-Compile (Recommended)

### Setup (run once)

```bash
# SSH ke Debian server
ssh user@your-server

# Clone repo
git clone <repo-url>
cd topdock

# Auto setup
make setup
# atau: ./build-server-setup.sh
```

### Build

```bash
# Release build (optimized)
make build

# Debug build (faster compilation)
make build-debug

# Clean rebuild
make rebuild
```

### Output
- **File:** `target/x86_64-pc-windows-gnu/release/topdock.exe`
- **Size:** ~5-8 MB (release), ~15-20 MB (debug)
- **Time:** 10-20 min (first), 1-5 min (incremental)

### Deploy

```bash
# Auto deploy via SSH
make deploy IP=192.168.1.100

# Manual SCP
scp target/x86_64-pc-windows-gnu/release/topdock.exe user@windows:~/Desktop/

# Via HTTP
cd target/x86_64-pc-windows-gnu/release
python3 -m http.server 8000
# Download dari: http://server-ip:8000/topdock.exe
```

### Pros
✅ Fastest incremental builds\
✅ Full control over build process\
✅ Easy debugging\
✅ Works offline (after setup)

### Cons
❌ Requires initial Rust installation\
❌ Takes ~500MB disk space

---

## 2️⃣ Docker Build

**Zero Rust install needed!** Hanya perlu Docker.

### Setup

```bash
# Install Docker (jika belum)
# Debian/Ubuntu:
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add user to docker group
sudo usermod -aG docker $USER
# Logout & login again
```

### Build

```bash
# Clone repo
git clone <repo-url>
cd topdock

# Build dengan Docker
make build-docker
# atau: chmod +x docker-build.sh && ./docker-build.sh
```

### Output
- **File:** `./topdock.exe` (di root directory)
- **Size:** ~5-8 MB
- **Time:** 15-30 min (first), 10-20 min (rebuilds)

### Pros
✅ No Rust installation needed\
✅ Reproducible builds\
✅ Isolated environment

### Cons
❌ Slower than native (no incremental compilation)\
❌ Requires Docker (~1GB disk space)\
❌ No build cache between runs

---

## 3️⃣ GitHub Actions (Auto CI/CD)

Auto-build saat push ke GitHub. Zero server setup!

### Setup

1. **Push repo ke GitHub**
   ```bash
   git remote add origin https://github.com/yourusername/topdock.git
   git push -u origin main
   ```

2. **GitHub Actions sudah configured** (file `.github/workflows/build.yml`)

3. **Every push triggers build**

### Download exe

```bash
# Via GitHub web:
1. Go to: https://github.com/yourusername/topdock/actions
2. Click latest workflow run
3. Download artifact: "topdock-windows-x64"
4. Extract topdock.exe
```

### Or use GitHub CLI

```bash
# Install gh CLI
# https://cli.github.com/

# Download latest artifact
gh run list --limit 1
gh run download <run-id> -n topdock-windows-x64
```

### Pros
✅ Zero server setup\
✅ Automatic builds\
✅ Free for public repos\
✅ Build on multiple OS (Windows native + Linux cross-compile)

### Cons
❌ Requires GitHub account\
❌ ~5-10 min build time per push\
❌ Limited minutes on free tier (2000/month)

---

## 4️⃣ GitLab CI (Self-Hosted)

Similar ke GitHub Actions, tapi self-hosted.

### Setup

1. **Push to GitLab**
   ```bash
   git remote add origin https://gitlab.com/yourusername/topdock.git
   git push -u origin main
   ```

2. **Create `.gitlab-ci.yml`**
   ```yaml
   image: rust:latest

   stages:
     - build

   build:windows:
     stage: build
     before_script:
       - apt-get update
       - apt-get install -y mingw-w64
       - rustup target add x86_64-pc-windows-gnu
     script:
       - cargo build --release --target x86_64-pc-windows-gnu
     artifacts:
       paths:
         - target/x86_64-pc-windows-gnu/release/topdock.exe
   ```

3. **Download from Pipeline artifacts**

### Pros
✅ Self-hosted option\
✅ Unlimited build minutes\
✅ Full control

### Cons
❌ Requires GitLab setup\
❌ More complex configuration

---

## 🆚 Comparison

### Build Speed

```
First Build:
├─ Native:        10-20 min  ⭐⭐⭐⭐⭐
├─ Docker:        15-30 min  ⭐⭐⭐
├─ GitHub:        5-10 min   ⭐⭐⭐⭐
└─ GitLab:        5-15 min   ⭐⭐⭐⭐

Incremental Build:
├─ Native:        1-5 min    ⭐⭐⭐⭐⭐
├─ Docker:        10-20 min  ⭐⭐
├─ GitHub:        5-10 min   ⭐⭐⭐⭐
└─ GitLab:        5-10 min   ⭐⭐⭐⭐
```

### Disk Space

```
├─ Native:        ~500 MB (Rust + build cache)
├─ Docker:        ~1 GB (Docker image)
├─ GitHub:        0 MB (builds in cloud)
└─ GitLab:        0 MB (builds in cloud) atau ~2GB (self-hosted)
```

### Setup Complexity

```
├─ Native:        ⭐⭐⭐ (run 1 script)
├─ Docker:        ⭐⭐⭐⭐ (install Docker)
├─ GitHub:        ⭐⭐⭐⭐⭐ (push to repo)
└─ GitLab:        ⭐⭐⭐ (setup runner)
```

---

## 💡 Recommendation

### For Development (daily builds)
→ **Use Native Cross-Compile** (Method 1)
- Fastest incremental builds
- Best for active development

### For Occasional Builds
→ **Use Docker** (Method 2)
- No Rust install
- Simple & clean

### For Team/Open Source
→ **Use GitHub Actions** (Method 3)
- Auto-build on every commit
- Easy artifact download
- Free for public repos

### For Enterprise
→ **Use GitLab CI** (Method 4) with self-hosted runner
- Full control
- Unlimited builds
- Custom infrastructure

---

## 🔧 Hybrid Approach

Combine methods untuk best result:

```
Developer Machine (Windows):
└─ Edit code with VSCode
   ↓
Git Push
   ↓
GitHub Actions:
├─ Auto-build on push
├─ Run tests
├─ Create release artifact
└─ Notify on Slack/Discord
   ↓
Debian Server (for quick testing):
├─ git pull
├─ make build (native, fast)
└─ make deploy IP=windows-test-machine
   ↓
Windows Test Machine:
└─ Run & test locally
```

---

## 📚 Quick Commands

```bash
# Method 1: Native
make setup && make build && make deploy IP=x

# Method 2: Docker
make build-docker && scp topdock.exe user@windows:~/

# Method 3: GitHub
git push origin main
# Wait for Actions, download artifact

# Method 4: GitLab
git push gitlab main
# Wait for Pipeline, download artifact
```

---

## ✅ Checklist: Choose Your Method

**Pick Method 1 (Native) if:**
- [ ] You have a Debian/Ubuntu server
- [ ] You'll build frequently (daily)
- [ ] You want fastest builds
- [ ] Disk space not an issue

**Pick Method 2 (Docker) if:**
- [ ] You don't want to install Rust
- [ ] You build occasionally
- [ ] You already have Docker
- [ ] You want clean isolation

**Pick Method 3 (GitHub) if:**
- [ ] You use GitHub
- [ ] You want auto-builds
- [ ] You don't have a build server
- [ ] Open source or free tier OK

**Pick Method 4 (GitLab) if:**
- [ ] You use GitLab
- [ ] You need self-hosted
- [ ] You want unlimited builds
- [ ] Enterprise environment

---

**Next:** Pilih method, setup, dan build! 🚀

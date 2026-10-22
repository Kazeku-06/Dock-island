#!/bin/bash
# Setup script untuk build server Debian
# Run: chmod +x build-server-setup.sh && ./build-server-setup.sh

set -e

echo "================================"
echo "TopDock Build Server Setup"
echo "================================"
echo ""

# 1. Update system
echo "[1/6] Updating system..."
sudo apt update && sudo apt upgrade -y

# 2. Install dependencies
echo "[2/6] Installing build dependencies..."
sudo apt install -y curl build-essential mingw-w64 git

# 3. Install Rust
echo "[3/6] Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust already installed"
fi

# 4. Add Windows target
echo "[4/6] Adding Windows target..."
rustup target add x86_64-pc-windows-gnu

# 5. Configure cargo for cross-compilation
echo "[5/6] Configuring cargo..."
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[build]
jobs = 4
EOF

# 6. Verify installation
echo "[6/6] Verifying installation..."
rustc --version
cargo --version
x86_64-w64-mingw32-gcc --version

echo ""
echo "================================"
echo "✅ Setup Complete!"
echo "================================"
echo ""
echo "Next steps:"
echo "  1. Clone/update your TopDock repository"
echo "  2. Run: ./build.sh"
echo "  3. Copy target/x86_64-pc-windows-gnu/release/topdock.exe to Windows"
echo ""

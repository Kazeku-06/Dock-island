#!/bin/bash
# Build script untuk TopDock
# Cross-compile dari Linux ke Windows

set -e

echo "================================"
echo "Building TopDock for Windows"
echo "================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Run ./build-server-setup.sh first"
    exit 1
fi

# Check if Windows target is installed
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo -e "${YELLOW}Installing Windows target...${NC}"
    rustup target add x86_64-pc-windows-gnu
fi

# Parse arguments
BUILD_TYPE="release"
if [[ "$1" == "debug" ]]; then
    BUILD_TYPE="debug"
fi

echo -e "${GREEN}Build type: ${BUILD_TYPE}${NC}"
echo ""

# Clean old builds (optional)
if [[ "$1" == "clean" ]] || [[ "$2" == "clean" ]]; then
    echo "Cleaning old builds..."
    cargo clean
fi

# Build
echo "Building..."
if [[ "$BUILD_TYPE" == "release" ]]; then
    cargo build --release --target x86_64-pc-windows-gnu
    OUTPUT_DIR="target/x86_64-pc-windows-gnu/release"
else
    cargo build --target x86_64-pc-windows-gnu
    OUTPUT_DIR="target/x86_64-pc-windows-gnu/debug"
fi

# Check if build succeeded
if [ -f "${OUTPUT_DIR}/topdock.exe" ]; then
    echo ""
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}✅ Build Successful!${NC}"
    echo -e "${GREEN}================================${NC}"
    echo ""
    echo "Output: ${OUTPUT_DIR}/topdock.exe"
    echo "Size: $(du -h ${OUTPUT_DIR}/topdock.exe | cut -f1)"
    echo ""
    echo "To transfer to Windows:"
    echo "  scp ${OUTPUT_DIR}/topdock.exe user@windows-ip:~/Desktop/"
    echo ""
    echo "Or use the deploy script:"
    echo "  ./deploy.sh windows-ip"
    echo ""
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

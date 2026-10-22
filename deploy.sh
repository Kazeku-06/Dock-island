#!/bin/bash
# Deploy script - transfer built exe to Windows machine

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

if [ -z "$1" ]; then
    echo -e "${RED}Usage: ./deploy.sh <windows-ip-or-hostname> [windows-username]${NC}"
    echo ""
    echo "Examples:"
    echo "  ./deploy.sh 192.168.1.100"
    echo "  ./deploy.sh 192.168.1.100 Administrator"
    echo "  ./deploy.sh my-windows-pc.local john"
    exit 1
fi

WINDOWS_HOST="$1"
WINDOWS_USER="${2:-$USER}"
OUTPUT_DIR="target/x86_64-pc-windows-gnu/release"
EXE_FILE="${OUTPUT_DIR}/topdock.exe"

# Check if exe exists
if [ ! -f "$EXE_FILE" ]; then
    echo -e "${RED}Error: topdock.exe not found!${NC}"
    echo "Run ./build.sh first"
    exit 1
fi

echo "================================"
echo "Deploying TopDock to Windows"
echo "================================"
echo ""
echo "Target: ${WINDOWS_USER}@${WINDOWS_HOST}"
echo "File: ${EXE_FILE}"
echo "Size: $(du -h ${EXE_FILE} | cut -f1)"
echo ""

# Deploy using scp
echo "Transferring..."
scp "$EXE_FILE" "${WINDOWS_USER}@${WINDOWS_HOST}:~/Desktop/topdock.exe"

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}✅ Deploy Successful!${NC}"
    echo -e "${GREEN}================================${NC}"
    echo ""
    echo "File location on Windows:"
    echo "  C:\\Users\\${WINDOWS_USER}\\Desktop\\topdock.exe"
    echo ""
    echo "Next steps:"
    echo "  1. On Windows, double-click topdock.exe to run"
    echo "  2. Press ESC to exit (for now)"
    echo "  3. Check logs at: %USERPROFILE%\\.topdock\\"
    echo ""
else
    echo -e "${RED}Deploy failed!${NC}"
    exit 1
fi

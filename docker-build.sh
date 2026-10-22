#!/bin/bash
# Build TopDock using Docker (no local Rust install needed!)

set -e

echo "================================"
echo "Building TopDock with Docker"
echo "================================"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed"
    echo "Install from: https://docs.docker.com/engine/install/"
    exit 1
fi

# Build Docker image
echo "Building Docker image..."
docker build -f Dockerfile.cross -t topdock-builder .

# Extract exe from container
echo ""
echo "Extracting exe..."
docker create --name topdock-temp topdock-builder
docker cp topdock-temp:/app/target/x86_64-pc-windows-gnu/release/topdock.exe ./topdock.exe
docker rm topdock-temp

echo ""
echo "================================"
echo "✅ Build Complete!"
echo "================================"
echo ""
echo "Output: ./topdock.exe"
echo "Size: $(du -h topdock.exe | cut -f1)"
echo ""
echo "Transfer to Windows:"
echo "  scp topdock.exe user@windows-ip:~/Desktop/"
echo ""

# Makefile untuk TopDock
# Simplify build commands

.PHONY: help setup build build-debug build-docker clean deploy test check fmt clippy

# Default target
help:
	@echo "TopDock Build Commands:"
	@echo ""
	@echo "  make setup        - Setup build environment (run once)"
	@echo "  make build        - Build release version for Windows"
	@echo "  make build-debug  - Build debug version"
	@echo "  make build-docker - Build using Docker (no Rust install needed)"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make deploy IP=<ip> - Deploy to Windows machine"
	@echo "  make test         - Run tests"
	@echo "  make check        - Run clippy linter"
	@echo "  make fmt          - Format code"
	@echo ""

# Setup build environment
setup:
	@chmod +x build-server-setup.sh
	@./build-server-setup.sh

# Build release version
build:
	@echo "Building release version..."
	@cargo build --release --target x86_64-pc-windows-gnu
	@echo ""
	@echo "✅ Build complete!"
	@echo "Output: target/x86_64-pc-windows-gnu/release/topdock.exe"
	@ls -lh target/x86_64-pc-windows-gnu/release/topdock.exe

# Build debug version (faster)
build-debug:
	@echo "Building debug version..."
	@cargo build --target x86_64-pc-windows-gnu
	@echo ""
	@echo "✅ Build complete!"
	@echo "Output: target/x86_64-pc-windows-gnu/debug/topdock.exe"
	@ls -lh target/x86_64-pc-windows-gnu/debug/topdock.exe

# Build using Docker (no Rust install needed)
build-docker:
	@echo "Building with Docker..."
	@chmod +x docker-build.sh
	@./docker-build.sh

# Clean build artifacts
clean:
	@echo "Cleaning..."
	@cargo clean
	@echo "✅ Clean complete!"

# Deploy to Windows (use: make deploy IP=192.168.1.100)
deploy:
	@if [ -z "$(IP)" ]; then \
		echo "Error: IP not specified"; \
		echo "Usage: make deploy IP=192.168.1.100"; \
		exit 1; \
	fi
	@chmod +x deploy.sh
	@./deploy.sh $(IP) $(USER)

# Run tests
test:
	@cargo test --target x86_64-pc-windows-gnu

# Run clippy (linter)
check:
	@cargo clippy --target x86_64-pc-windows-gnu -- -D warnings

# Format code
fmt:
	@cargo fmt --all

# Full rebuild (clean + build)
rebuild: clean build

# Quick build (for development)
quick:
	@cargo build --target x86_64-pc-windows-gnu

# Show build info
info:
	@echo "Build Information:"
	@echo "=================="
	@rustc --version
	@cargo --version
	@echo ""
	@rustup target list | grep windows-gnu
	@echo ""
	@echo "Build configuration:"
	@cat ~/.cargo/config.toml 2>/dev/null || echo "No custom config"

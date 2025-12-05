#!/bin/bash
set -e

# MoveForge Installation Script
# This script installs MoveForge CLI on Unix-based systems

REPO="moveforge/moveforge"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.moveforge}"
BIN_DIR="${BIN_DIR:-$HOME/.local/bin}"

echo "⚡ MoveForge Installer"
echo "====================="
echo ""

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     OS_TYPE="linux";;
    Darwin*)    OS_TYPE="macos";;
    *)          echo "Unsupported OS: $OS"; exit 1;;
esac

case "$ARCH" in
    x86_64)     ARCH_TYPE="x86_64";;
    arm64)      ARCH_TYPE="aarch64";;
    aarch64)    ARCH_TYPE="aarch64";;
    *)          echo "Unsupported architecture: $ARCH"; exit 1;;
esac

echo "Detected: $OS_TYPE ($ARCH_TYPE)"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found"
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "✓ Rust/Cargo installed"
echo ""

# Create installation directory
mkdir -p "$INSTALL_DIR"
mkdir -p "$BIN_DIR"

# Clone or download MoveForge
echo "Installing MoveForge..."

if [ -d "$INSTALL_DIR/src" ]; then
    echo "Updating existing installation..."
    cd "$INSTALL_DIR/src"
    git pull
else
    echo "Cloning repository..."
    git clone "https://github.com/$REPO" "$INSTALL_DIR/src"
    cd "$INSTALL_DIR/src"
fi

# Build MoveForge
echo "Building MoveForge (this may take a few minutes)..."
cargo build --release

# Install binary
echo "Installing binary to $BIN_DIR..."
cp target/release/moveforge "$BIN_DIR/"
chmod +x "$BIN_DIR/moveforge"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo ""
    echo "⚠️  Add $BIN_DIR to your PATH:"
    echo ""
    echo "  For bash: echo 'export PATH=\"\$PATH:$BIN_DIR\"' >> ~/.bashrc"
    echo "  For zsh:  echo 'export PATH=\"\$PATH:$BIN_DIR\"' >> ~/.zshrc"
    echo ""
fi

# Verify installation
if command -v moveforge &> /dev/null; then
    echo ""
    echo "✅ MoveForge installed successfully!"
    echo ""
    moveforge --version
    echo ""
    echo "Quick start:"
    echo "  moveforge init my-project"
    echo "  moveforge node --fork testnet"
    echo "  moveforge sim --parallel --count 100"
    echo ""
    echo "Documentation: https://github.com/moveforge/moveforge"
else
    echo ""
    echo "❌ Installation failed. Please check the errors above."
    exit 1
fi

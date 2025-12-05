#!/bin/bash
set -e

# ParaForge Installation Script
# This script installs ParaForge CLI on Unix-based systems

REPO="paraforge/paraforge"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.paraforge}"
BIN_DIR="${BIN_DIR:-$HOME/.local/bin}"

echo "⚡ ParaForge Installer"
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

# Clone or download ParaForge
echo "Installing ParaForge..."

if [ -d "$INSTALL_DIR/src" ]; then
    echo "Updating existing installation..."
    cd "$INSTALL_DIR/src"
    git pull
else
    echo "Cloning repository..."
    git clone "https://github.com/$REPO" "$INSTALL_DIR/src"
    cd "$INSTALL_DIR/src"
fi

# Build ParaForge
echo "Building ParaForge (this may take a few minutes)..."
cargo build --release

# Install binary
echo "Installing binary to $BIN_DIR..."
cp target/release/paraforge "$BIN_DIR/"
chmod +x "$BIN_DIR/paraforge"

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
if command -v paraforge &> /dev/null; then
    echo ""
    echo "✅ ParaForge installed successfully!"
    echo ""
    paraforge --version
    echo ""
    echo "Quick start:"
    echo "  paraforge init my-project"
    echo "  paraforge node --fork testnet"
    echo "  paraforge sim --parallel --count 100"
    echo ""
    echo "Documentation: https://github.com/paraforge/paraforge"
else
    echo ""
    echo "❌ Installation failed. Please check the errors above."
    exit 1
fi

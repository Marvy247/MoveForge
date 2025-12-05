#!/bin/bash
set -e

echo "⚡ Building MoveForge"
echo "===================="
echo ""

# Build Rust CLI
echo "📦 Building Rust components..."
cargo build --release
echo "✓ Rust build complete"
echo ""

# Build SDK
echo "📦 Building TypeScript SDK..."
cd sdk
npm install
npm run build
cd ..
echo "✓ SDK build complete"
echo ""

# Build Frontend
echo "📦 Building Frontend..."
cd frontend
npm install
npm run build
cd ..
echo "✓ Frontend build complete"
echo ""

echo "✅ All components built successfully!"
echo ""
echo "Artifacts:"
echo "  - CLI binary: target/release/moveforge"
echo "  - SDK package: sdk/dist/"
echo "  - Frontend: frontend/.next/"

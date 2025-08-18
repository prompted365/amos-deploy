#!/bin/bash
# Test build script for AMOS backend

echo "Testing AMOS backend build..."
echo "=============================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the backend directory."
    exit 1
fi

# Run cargo check first (faster than full build)
echo "Running cargo check..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Cargo check passed!"
    echo ""
    echo "Running full build..."
    cargo build
    
    if [ $? -eq 0 ]; then
        echo "✅ Build successful!"
        echo ""
        echo "You can now run the server with:"
        echo "  cargo run"
        echo ""
        echo "Or run the release build with:"
        echo "  cargo build --release"
        echo "  ./target/release/amos-deploy-server"
    else
        echo "❌ Build failed!"
        exit 1
    fi
else
    echo "❌ Cargo check failed!"
    exit 1
fi
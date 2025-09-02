#!/bin/bash

# Demo script for Qapish AI Colocation Platform
set -e

echo "🚀 Starting Qapish AI Colocation Platform Demo"
echo "============================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install Rust first:"
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "📦 Installing trunk for web development..."
    cargo install trunk
fi

echo "🔧 Building web frontend..."
cd web
trunk build --release
cd ..

echo "✅ Web frontend built successfully!"
echo ""

# Set demo environment variables
export DATABASE_URL=""
export PORT=8080
export DEMO_MODE=true

echo "🌐 Starting demo server on http://localhost:8080"
echo ""
echo "📦 Available AI Colocation Packages:"
echo "   • Midrange Consumer: $3,000 setup + $200/mo USDC"
echo "   • Top Consumer: $20,000 setup + $500/mo USDC"
echo "   • Pro Server: $100,000 setup + $1,000/mo USDC"
echo ""
echo "✨ Features showcased:"
echo "   • Marketing-style landing page"
echo "   • Dynamic package loading from API"
echo "   • Post-quantum security messaging"
echo "   • Responsive design with animations"
echo ""
echo "🛑 Press Ctrl+C to stop the demo"
echo ""

# Try to open browser (works on most systems)
sleep 2 && (
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:8080"
    elif command -v open &> /dev/null; then
        open "http://localhost:8080"
    elif command -v start &> /dev/null; then
        start "http://localhost:8080"
    else
        echo "🌐 Please open http://localhost:8080 in your browser"
    fi
) &

# Start the API server
echo "🚀 Launching API server..."
cargo run --bin api

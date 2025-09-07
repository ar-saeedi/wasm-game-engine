#!/bin/bash

# WebAssembly Game Engine Build Script
echo "🎮 Building WebAssembly Game Engine..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed"
    echo "📥 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed"
    echo "📥 Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "🔧 Building WebAssembly module..."
wasm-pack build --target web --out-dir pkg --release

if [ $? -eq 0 ]; then
    echo "✅ WebAssembly build successful!"
    
    # Install npm dependencies if package.json exists
    if [ -f "package.json" ]; then
        echo "📦 Installing npm dependencies..."
        npm install
    fi
    
    echo ""
    echo "🚀 Build complete! You can now:"
    echo "   1. Start development server: npm run dev"
    echo "   2. Open examples/basic-demo.html in your browser"
    echo "   3. Or serve with: python -m http.server 8000"
    echo ""
    echo "🌐 Then visit: http://localhost:8000/examples/basic-demo.html"
else
    echo "❌ Build failed!"
    exit 1
fi

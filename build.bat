@echo off
REM WebAssembly Game Engine Build Script for Windows

echo 🎮 Building WebAssembly Game Engine...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ wasm-pack is not installed
    echo 📥 Please install wasm-pack from: https://rustwasm.github.io/wasm-pack/installer/
    echo    Or run: cargo install wasm-pack
    pause
    exit /b 1
)

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed
    echo 📥 Please install Rust from: https://rustup.rs/
    pause
    exit /b 1
)

echo 🔧 Building WebAssembly module...
wasm-pack build --target web --out-dir pkg --release

if %errorlevel% equ 0 (
    echo ✅ WebAssembly build successful!
    
    REM Install npm dependencies if package.json exists
    if exist "package.json" (
        echo 📦 Installing npm dependencies...
        npm install
    )
    
    echo.
    echo 🚀 Build complete! You can now:
    echo    1. Start development server: npm run dev
    echo    2. Open examples/basic-demo.html in your browser
    echo    3. Or serve with: python -m http.server 8000
    echo.
    echo 🌐 Then visit: http://localhost:8000/examples/basic-demo.html
    echo.
    pause
) else (
    echo ❌ Build failed!
    pause
    exit /b 1
)

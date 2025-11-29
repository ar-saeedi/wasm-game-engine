@echo off
echo ========================================
echo   Deploying Game Engine to Vercel
echo   (Simple version - no WASM build)
echo ========================================
echo.

cd /d "%~dp0"

echo The game engine has JavaScript fallback!
echo No Rust compilation needed for demo.
echo.

echo Deploying to Vercel...
vercel --prod

echo.
echo ========================================
echo   Deployment Complete!
echo ========================================
echo.
echo Copy the deployment URL above!
echo.
echo Note: This uses JavaScript fallback engine.
echo For full WASM version, build locally first.
echo.
pause


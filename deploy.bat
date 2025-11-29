@echo off
echo ========================================
echo   Deploying Game Engine to Vercel
echo ========================================
echo.

cd /d "%~dp0"

echo Installing dependencies...
call npm install

echo.
echo Deploying to Vercel...
call vercel --prod

echo.
echo ========================================
echo   Deployment Complete!
echo ========================================
echo.
echo Copy the deployment URL above and save it!
echo.
pause


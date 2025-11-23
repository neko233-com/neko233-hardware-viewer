@echo off
echo ==========================================
echo      NEKO233 HARDWARE VIEWER DEV
echo ==========================================
echo.
echo Installing dependencies...
call npm install

echo.
echo Starting Tauri Dev Environment...
call npm run tauri dev
pause

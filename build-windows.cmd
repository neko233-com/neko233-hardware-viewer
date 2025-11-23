@echo off
chcp 65001 >nul
echo ========================================
echo  Neko233 Hardware Viewer - Windows Build
echo ========================================
echo.

REM Check Node.js
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo [Error] Node.js not found. Please install Node.js.
    pause
    exit /b 1
)

echo [Step 1/3] Installing dependencies...
if not exist "node_modules\" (
    call npm install
    if %errorlevel% neq 0 (
        echo [Error] npm install failed.
        pause
        exit /b 1
    )
) else (
    echo Dependencies already installed.
)

echo [Step 2/3] Building Rust backend and Frontend...
call npm run build:win
if %errorlevel% neq 0 (
    echo [Error] Build failed.
    pause
    exit /b 1
)

echo [Step 3/3] Done!
echo Output directory: src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi
pause

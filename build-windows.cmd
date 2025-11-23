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

echo [Step 3/3] Copying artifacts to release/...
if not exist "release" mkdir release

echo Copying MSI installers...
if exist "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\*.msi" (
    copy /Y "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\*.msi" "release\"
) else (
    echo [Warning] No MSI files found.
)

echo Copying NSIS installers...
if exist "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\*.exe" (
    copy /Y "src-tauri\target\x86_64-pc-windows-msvc\release\bundle\nsis\*.exe" "release\"
) else (
    echo [Warning] No NSIS files found.
)

echo.
echo ========================================
echo Build success! 
echo Artifacts are in the 'release' folder.
echo ========================================
pause

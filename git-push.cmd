@echo off
chcp 65001 >nul
echo ========================================
echo  Neko233 Hardware Viewer - Git Push
echo ========================================
echo.

set /p msg="Enter commit message (default: 'Update'): "
if "%msg%"=="" set msg=Update

echo.
echo [Step 1/3] Adding files...
git add .

echo.
echo [Step 2/3] Committing...
git commit -m "%msg%"

echo.
echo [Step 3/3] Pushing to GitHub...
git push

if %errorlevel% neq 0 (
    echo.
    echo [Error] Push failed.
    pause
    exit /b 1
)

echo.
echo [Success] Pushed to GitHub!
echo This should trigger the GitHub Action workflow.
pause

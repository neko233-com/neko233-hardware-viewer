@echo off
setlocal
chcp 65001 > nul

:: 设置日志文件
set LOG_FILE=neko233-hardware-viewer.log

:: 开始记录日志
echo ======================================== >> %LOG_FILE%
echo  发布流程开始: %DATE% %TIME% >> %LOG_FILE%
echo ======================================== >> %LOG_FILE%

:: 检查未提交的更改
git status --porcelain > nul
if %errorlevel% neq 0 (
    echo [信息] 检测到未提交的更改。 >> %LOG_FILE%
    echo [信息] 正在添加所有文件... >> %LOG_FILE%
    git add . >> %LOG_FILE% 2>&1
    
    set /p COMMIT_MSG="请输入提交信息 (默认: 'Release update'): "
    if "%COMMIT_MSG%"=="" set COMMIT_MSG=Release update
    
    echo [信息] 正在提交，信息: %COMMIT_MSG% >> %LOG_FILE%
    git commit -m "%COMMIT_MSG%" >> %LOG_FILE% 2>&1
) else (
    echo [信息] 没有需要提交的更改。 >> %LOG_FILE%
)

:: 升级版本号 (Patch)
echo [信息] 正在升级补丁版本号... >> %LOG_FILE%
call npm version patch --no-git-tag-version >> %LOG_FILE% 2>&1

:: 读取新版本号
for /f "tokens=2 delims=:, " %%a in ('findstr "version" package.json') do set NEW_VERSION=%%~a
echo [信息] 新版本: %NEW_VERSION% >> %LOG_FILE%

:: 提交版本号变更
git add package.json package-lock.json >> %LOG_FILE% 2>&1
git commit -m "chore: bump version to %NEW_VERSION%" >> %LOG_FILE% 2>&1

:: 本地构建
echo [信息] 正在进行本地构建... >> %LOG_FILE%
call npm run tauri build >> %LOG_FILE% 2>&1
if %errorlevel% neq 0 (
    echo [错误] 构建失败。请检查日志文件。
    echo [错误] 构建失败。 >> %LOG_FILE%
    exit /b 1
)

:: 推送到 GitHub
echo [信息] 正在推送到 GitHub... >> %LOG_FILE%
git push origin main >> %LOG_FILE% 2>&1

echo [成功] 发布流程已完成。 >> %LOG_FILE%
echo 发布完成！详情请查看 %LOG_FILE%。
pause

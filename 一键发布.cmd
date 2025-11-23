@echo off
setlocal
chcp 65001 > nul

:: 设置日志文件
set LOG_FILE=neko233-hardware-viewer.log

:: 开始记录日志
echo ========================================
echo ======================================== >> %LOG_FILE%
echo  发布流程开始: %DATE% %TIME%
echo  发布流程开始: %DATE% %TIME% >> %LOG_FILE%
echo ========================================
echo ======================================== >> %LOG_FILE%

:: 检查未提交的更改
git status --porcelain > nul
if %errorlevel% neq 0 (
    echo [信息] 检测到未提交的更改。
    echo [信息] 检测到未提交的更改。 >> %LOG_FILE%
    echo [信息] 正在添加所有文件...
    echo [信息] 正在添加所有文件... >> %LOG_FILE%
    git add . >> %LOG_FILE% 2>&1
    
    set /p COMMIT_MSG="请输入提交信息 (默认: 'Release update'): "
    if "%COMMIT_MSG%"=="" set COMMIT_MSG=Release update
    
    echo [信息] 正在提交，信息: %COMMIT_MSG%
    echo [信息] 正在提交，信息: %COMMIT_MSG% >> %LOG_FILE%
    git commit -m "%COMMIT_MSG%" >> %LOG_FILE% 2>&1
) else (
    echo [信息] 没有需要提交的更改。
    echo [信息] 没有需要提交的更改。 >> %LOG_FILE%
)

:: 版本号管理
for /f "tokens=2 delims=:, " %%a in ('findstr "version" package.json') do set CURRENT_VERSION=%%~a
echo 当前版本: %CURRENT_VERSION%

set /p TARGET_VERSION="请输入新版本号 (回车默认自动+1 Patch): "

if "%TARGET_VERSION%"=="" (
    echo [信息] 正在自动升级补丁版本号...
    echo [信息] 正在自动升级补丁版本号... >> %LOG_FILE%
    call npm version patch --no-git-tag-version >> %LOG_FILE% 2>&1
) else (
    echo [信息] 正在设置版本号为 %TARGET_VERSION%...
    echo [信息] 正在设置版本号为 %TARGET_VERSION%... >> %LOG_FILE%
    call npm version %TARGET_VERSION% --no-git-tag-version >> %LOG_FILE% 2>&1
)

:: 读取新版本号
for /f %%v in ('node -p "require('./package.json').version"') do set NEW_VERSION=%%v
echo [信息] 新版本: %NEW_VERSION%
echo [信息] 新版本: %NEW_VERSION% >> %LOG_FILE%

:: 同步版本号到 tauri.conf.json
echo [信息] 正在同步版本号到 tauri.conf.json...
echo [信息] 正在同步版本号到 tauri.conf.json... >> %LOG_FILE%
node -e "const fs = require('fs'); const tauriPath = 'src-tauri/tauri.conf.json'; const pkg = require('./package.json'); const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf8')); tauri.package.version = pkg.version; fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2));" >> %LOG_FILE% 2>&1

:: 提交版本号变更
git add package.json package-lock.json src-tauri\tauri.conf.json >> %LOG_FILE% 2>&1
git commit -m "chore: bump version to %NEW_VERSION%" >> %LOG_FILE% 2>&1

:: 设置签名密钥 (从文件读取)
if exist tauri.key (
    set /p TAURI_PRIVATE_KEY=<tauri.key
    echo [信息] 已加载签名密钥。
    echo [信息] 已加载签名密钥。 >> %LOG_FILE%
) else (
    echo [警告] 未找到 tauri.key，构建可能失败或无法签名。
    echo [警告] 未找到 tauri.key，构建可能失败或无法签名。 >> %LOG_FILE%
)
set TAURI_KEY_PASSWORD=

:: 本地构建
echo [信息] 正在进行本地构建... (输出将显示在控制台)
echo [信息] 正在进行本地构建... >> %LOG_FILE%
call npm run tauri build
if %errorlevel% neq 0 (
    echo [错误] 构建失败。
    echo [错误] 构建失败。 >> %LOG_FILE%
    exit /b 1
)

:: 复制构建产物到 release 目录
echo [信息] 正在复制构建产物到 release 目录...
echo [信息] 正在复制构建产物到 release 目录... >> %LOG_FILE%

if not exist "release" mkdir "release"

:: 复制 NSIS 安装包 (.exe)
if exist "src-tauri\target\release\bundle\nsis\*.exe" (
    copy /Y "src-tauri\target\release\bundle\nsis\*.exe" "release\" >> %LOG_FILE% 2>&1
)

:: 复制 MSI 安装包 (.msi)
if exist "src-tauri\target\release\bundle\msi\*.msi" (
    copy /Y "src-tauri\target\release\bundle\msi\*.msi" "release\" >> %LOG_FILE% 2>&1
)

:: 推送到 GitHub
echo [信息] 正在推送到 GitHub...
echo [信息] 正在推送到 GitHub... >> %LOG_FILE%
git push origin main >> %LOG_FILE% 2>&1

:: 创建并推送 Tag
echo [信息] 正在创建并推送 Tag v%NEW_VERSION%...
echo [信息] 正在创建并推送 Tag v%NEW_VERSION%... >> %LOG_FILE%
git tag v%NEW_VERSION% >> %LOG_FILE% 2>&1
git push origin v%NEW_VERSION% >> %LOG_FILE% 2>&1

:: 尝试创建 GitHub Release
where gh >nul 2>nul
if %errorlevel% equ 0 (
    echo [信息] 检测到 GitHub CLI，正在创建 Release 并上传附件...
    echo [信息] 检测到 GitHub CLI，正在创建 Release 并上传附件... >> %LOG_FILE%
    
    gh release create v%NEW_VERSION% ^
        "src-tauri\target\release\bundle\nsis\*.exe" ^
        "src-tauri\target\release\bundle\msi\*.msi" ^
        --title "v%NEW_VERSION%" ^
        --notes "Release v%NEW_VERSION%" >> %LOG_FILE% 2>&1
        
    if %errorlevel% equ 0 (
        echo [成功] Release 创建成功！
        echo [成功] Release 创建成功！ >> %LOG_FILE%
    ) else (
        echo [错误] Release 创建失败，请检查日志。
        echo [错误] Release 创建失败，请检查日志。 >> %LOG_FILE%
    )
) else (
    echo [警告] 未找到 GitHub CLI ^(gh^)，跳过 Release 创建。
    echo [警告] 未找到 GitHub CLI ^(gh^)，跳过 Release 创建。 >> %LOG_FILE%
    echo [建议] 请安装 GitHub CLI 以启用自动发布 Release 功能: https://cli.github.com/
)

echo [成功] 发布流程已完成。
echo [成功] 发布流程已完成。 >> %LOG_FILE%
echo 发布完成！详情请查看 %LOG_FILE%。

echo.
echo ========================================
echo  快速访问信息
echo ========================================
echo [安装包目录]
echo %CD%\src-tauri\target\release\bundle\nsis
echo.
echo [GitHub Tags]
echo https://github.com/neko233-com/neko233-hardware-viewer/tags
echo.
echo [GitHub Actions]
echo https://github.com/neko233-com/neko233-hardware-viewer/actions
echo ========================================
echo.
echo 正在打开安装包目录...
start "" "%CD%\src-tauri\target\release\bundle\nsis"

pause

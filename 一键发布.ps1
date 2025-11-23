# 一键发布.ps1
$ErrorActionPreference = "Stop"
$LogFile = "neko233-hardware-viewer.log"

# 辅助函数：记录日志
function Log-Message {
    param([string]$Message, [string]$Level="INFO")
    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $LogLine = "[$Timestamp] [$Level] $Message"
    Write-Host $LogLine -ForegroundColor Cyan
    $LogLine | Out-File -FilePath $LogFile -Append -Encoding utf8
}

# 初始化日志
"========================================" | Out-File -FilePath $LogFile -Append -Encoding utf8
" 发布流程开始: $(Get-Date)" | Out-File -FilePath $LogFile -Append -Encoding utf8
"========================================" | Out-File -FilePath $LogFile -Append -Encoding utf8

# 1. 检查 Git 状态
$gitStatus = git status --porcelain
if ($gitStatus) {
    Log-Message "检测到未提交的更改。"
    Log-Message "正在添加所有文件..."
    git add . 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
    
    $commitMsg = Read-Host "请输入提交信息 (默认: 'Release update')"
    if ([string]::IsNullOrWhiteSpace($commitMsg)) { $commitMsg = "Release update" }
    
    Log-Message "正在提交，信息: $commitMsg"
    git commit -m "$commitMsg" 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
} else {
    Log-Message "没有需要提交的更改。"
}

# 2. 版本号管理
# 使用 Node 读取版本号以确保准确
$currentVersion = node -p "require('./package.json').version"
Write-Host "当前版本: $currentVersion" -ForegroundColor Yellow

$targetVersion = Read-Host "请输入新版本号 (回车默认自动+1 Patch)"

if ([string]::IsNullOrWhiteSpace($targetVersion)) {
    Log-Message "正在自动升级补丁版本号..."
    npm version patch --no-git-tag-version 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
} else {
    Log-Message "正在设置版本号为 $targetVersion..."
    npm version $targetVersion --no-git-tag-version 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
}

# 读取新版本号
$newVersion = node -p "require('./package.json').version"
Log-Message "新版本: $newVersion"

# 3. 同步版本号到 tauri.conf.json
Log-Message "正在同步版本号到 tauri.conf.json..."
# 使用 Node.js 处理 JSON 以避免 PowerShell 编码问题 (BOM)
node -e "const fs = require('fs'); const tauriPath = 'src-tauri/tauri.conf.json'; const pkg = require('./package.json'); const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf8')); tauri.package.version = pkg.version; fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2));" 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8

# 4. 提交版本号变更
git add package.json package-lock.json src-tauri/tauri.conf.json 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
git commit -m "chore: bump version to $newVersion" 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8

# 5. 设置签名密钥
if (Test-Path "tauri.key") {
    $env:TAURI_PRIVATE_KEY = Get-Content "tauri.key" -Raw
    $env:TAURI_KEY_PASSWORD = ""
    Log-Message "已加载签名密钥。"
} else {
    Log-Message "未找到 tauri.key，构建可能失败或无法签名。" "WARNING"
}

# 6. 本地构建
Log-Message "正在进行本地构建... (输出将显示在控制台)"
try {
    # 直接调用 npm，让输出流向控制台
    npm run tauri build
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }
} catch {
    Log-Message "构建失败。" "ERROR"
    exit 1
}

# 7. 复制构建产物
Log-Message "正在复制构建产物到 release 目录..."
if (-not (Test-Path "release")) { New-Item -ItemType Directory -Path "release" | Out-Null }

$nsisPath = "src-tauri\target\release\bundle\nsis"
$msiPath = "src-tauri\target\release\bundle\msi"

if (Test-Path "$nsisPath\*.exe") { Copy-Item "$nsisPath\*.exe" -Destination "release\" -Force }
if (Test-Path "$msiPath\*.msi") { Copy-Item "$msiPath\*.msi" -Destination "release\" -Force }

# 8. 推送到 GitHub
Log-Message "正在推送到 GitHub..."
git push origin main 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8

# 9. 创建并推送 Tag
Log-Message "正在创建并推送 Tag v$newVersion..."
git tag "v$newVersion" 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8
git push origin "v$newVersion" 2>&1 | Out-File -FilePath $LogFile -Append -Encoding utf8

# 10. GitHub Release
if (Get-Command "gh" -ErrorAction SilentlyContinue) {
    Log-Message "检测到 GitHub CLI，正在创建 Release 并上传附件..."
    
    $files = @()
    if (Test-Path "$nsisPath\*.exe") { $files += "$nsisPath\*.exe" }
    if (Test-Path "$msiPath\*.msi") { $files += "$msiPath\*.msi" }
    
    # 展开文件路径
    $expandedFiles = @()
    foreach ($f in $files) {
        $expandedFiles += Get-Item $f | Select-Object -ExpandProperty FullName
    }

    if ($expandedFiles.Count -gt 0) {
        try {
            # 注意：PowerShell 中调用外部命令传参有时需要注意引号
            gh release create "v$newVersion" $expandedFiles --title "v$newVersion" --notes "Release v$newVersion"
            Log-Message "Release 创建成功！"
        } catch {
            Log-Message "Release 创建失败 (可能是网络问题或已存在): $_" "ERROR"
        }
    } else {
        Log-Message "未找到构建产物，跳过上传附件。" "WARNING"
    }
} else {
    Log-Message "未找到 GitHub CLI (gh)，跳过 Release 创建。" "WARNING"
    Write-Host "[建议] 请安装 GitHub CLI 以启用自动发布 Release 功能: https://cli.github.com/" -ForegroundColor Yellow
}

Log-Message "发布流程已完成。"

# 快速访问信息
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host " 快速访问信息" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "[安装包目录]"
Write-Host "$PWD\src-tauri\target\release\bundle\nsis"
Write-Host ""
Write-Host "[GitHub Tags]"
Write-Host "https://github.com/neko233-com/neko233-hardware-viewer/tags"
Write-Host ""
Write-Host "[GitHub Actions]"
Write-Host "https://github.com/neko233-com/neko233-hardware-viewer/actions"
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "正在打开安装包目录..."
Invoke-Item "$PWD\src-tauri\target\release\bundle\nsis"

Read-Host "按回车键退出..."

import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import readline from 'readline';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

function runCommand(command, ignoreError = false) {
    try {
        console.log(`> ${command}`);
        execSync(command, { stdio: 'inherit' });
        return true;
    } catch (error) {
        if (!ignoreError) {
            console.error(`[错误] 命令执行失败: ${command}`);
            process.exit(1);
        }
        return false;
    }
}

function getPackageVersion() {
    const pkgPath = path.join(__dirname, 'package.json');
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
    return pkg.version;
}

function updateVersion(newVersion) {
    // 更新 package.json
    const pkgPath = path.join(__dirname, 'package.json');
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
    pkg.version = newVersion;
    fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
    console.log(`[信息] 已更新 package.json 版本至 ${newVersion}`);

    // 更新 tauri.conf.json
    const tauriPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');
    const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf-8'));
    tauri.version = newVersion; // Tauri v2 根级别版本号
    fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + '\n');
    console.log(`[信息] 已更新 tauri.conf.json 版本至 ${newVersion}`);
}

function incrementVersion(version, type = 'patch') {
    const parts = version.split('.').map(Number);
    if (type === 'major') {
        parts[0]++;
        parts[1] = 0;
        parts[2] = 0;
    } else if (type === 'minor') {
        parts[1]++;
        parts[2] = 0;
    } else {
        parts[2]++;
    }
    return parts.join('.');
}

console.log('========================================');
console.log(' Neko233 Hardware Viewer - 发布工具');
console.log('========================================');
console.log('');

// 主执行流程
(async () => {
    const ask = (query) => new Promise((resolve) => rl.question(query, resolve));

    // 1. 检查 Git 状态
    try {
        execSync('git status --porcelain');
    } catch (e) {
        // 如果输出不为空（或命令失败），可能存在更改
    }
    
    // 我们无法轻易检查 execSync 的输出（因为 stdio: inherit），所以直接尝试添加/提交
    // 或者总是执行 git add .
    
    console.log('[步骤 1] 检查 Git 状态...');
    runCommand('git add .');
    // 尝试提交，如果没有内容提交则忽略错误
    const commitMsg = await ask("请输入当前更改的提交信息 (默认: 'Release update'): ") || 'Release update';
    runCommand(`git commit -m "${commitMsg}"`, true);

    // 2. 版本管理
    const currentVersion = getPackageVersion();
    console.log(`\n当前版本: ${currentVersion}`);
    
    const nextPatch = incrementVersion(currentVersion, 'patch');
    const versionInput = await ask(`请输入新版本号 (默认: ${nextPatch}): `);
    const newVersion = versionInput.trim() || nextPatch;

    if (newVersion !== currentVersion) {
        updateVersion(newVersion);
        // 提交版本号变更
        runCommand('git add package.json src-tauri/tauri.conf.json');
        runCommand(`git commit -m "chore: bump version to ${newVersion}"`);
        
        // 立即推送版本变更，防止后续构建失败导致忘记推送
        console.log('[信息] 正在推送版本号变更...');
        runCommand('git push');
    }

    // 3. 加载签名密钥
    const keyPath = path.join(__dirname, 'tauri.key');
    if (fs.existsSync(keyPath)) {
        try {
            const keyContent = fs.readFileSync(keyPath, 'utf-8').trim();
            process.env.TAURI_PRIVATE_KEY = keyContent;
            process.env.TAURI_KEY_PASSWORD = ''; 
            console.log('[信息] 已从 tauri.key 加载签名密钥');
        } catch (e) {
            console.warn('[警告] 读取 tauri.key 失败:', e.message);
        }
    } else {
        console.warn('[警告] 未找到 tauri.key。如果启用了自动更新，构建可能会失败。');
    }

    // 4. 构建
    console.log('\n[步骤 4] 正在构建应用程序...');
    const startTime = Date.now();
    runCommand('npm run build:win');

    // 5. 复制构建产物
    console.log('\n[步骤 5] 处理构建产物...');
    const releaseDir = path.join(__dirname, 'release');
    if (!fs.existsSync(releaseDir)) {
        fs.mkdirSync(releaseDir);
    }

    const targetBase = path.join(__dirname, 'src-tauri', 'target', 'x86_64-pc-windows-msvc', 'release', 'bundle');
    const msiDir = path.join(targetBase, 'msi');
    const nsisDir = path.join(targetBase, 'nsis');

    let copiedCount = 0;

    // 复制 MSI
    if (fs.existsSync(msiDir)) {
        const files = fs.readdirSync(msiDir).filter(f => f.endsWith('.msi'));
        files.forEach(file => {
            fs.copyFileSync(path.join(msiDir, file), path.join(releaseDir, file));
            console.log(`已复制: ${file}`);
            copiedCount++;
        });
    }

    // 复制 NSIS (Exe)
    if (fs.existsSync(nsisDir)) {
        const files = fs.readdirSync(nsisDir).filter(f => f.endsWith('.exe'));
        files.forEach(file => {
            fs.copyFileSync(path.join(nsisDir, file), path.join(releaseDir, file));
            console.log(`已复制: ${file}`);
            copiedCount++;
        });
    }

    if (copiedCount === 0) {
        console.warn('[警告] 未找到可复制的构建产物。');
    }

    // 6. Git 推送和打标签
    console.log('\n[步骤 6] Git 推送和打标签...');
    try {
        runCommand('git push');
        
        if (newVersion !== currentVersion) {
            const tagName = `v${newVersion}`;
            console.log(`正在创建标签 ${tagName}...`);
            runCommand(`git tag ${tagName}`);
            runCommand(`git push origin ${tagName}`);
        }
    } catch (e) {
        console.error('[错误] Git 操作失败。');
    }

    const endTime = Date.now();
    const duration = ((endTime - startTime) / 1000).toFixed(1);

    console.log('\n========================================');
    console.log(`[成功] 发布流程在 ${duration}秒内完成！`);
    console.log('构建产物位于 "release/" 文件夹中。');
    console.log('代码和标签已推送到 GitHub。');
    console.log('========================================');

    console.log('\n[相关链接]');
    console.log('GitHub 仓库: https://github.com/neko233-com/neko233-hardware-viewer');
    console.log('GitHub Actions: https://github.com/neko233-com/neko233-hardware-viewer/actions');
    console.log(`本地发布目录: ${releaseDir}`);

    console.log('\n[信息] 正在打开发布目录...');
    try {
        // Windows 下打开文件夹
        execSync(`start "" "${releaseDir}"`);
    } catch (e) {
        console.warn('[警告] 无法自动打开文件夹:', e.message);
    }
    
    rl.close();
})();


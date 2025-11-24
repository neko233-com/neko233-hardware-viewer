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
            console.error(`[Error] Command failed: ${command}`);
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
    // Update package.json
    const pkgPath = path.join(__dirname, 'package.json');
    const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
    pkg.version = newVersion;
    fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
    console.log(`[Info] Updated package.json to ${newVersion}`);

    // Update tauri.conf.json
    const tauriPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');
    const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf-8'));
    tauri.version = newVersion; // Tauri v2 root level version
    fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + '\n');
    console.log(`[Info] Updated tauri.conf.json to ${newVersion}`);
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
console.log(' Neko233 Hardware Viewer - Release Tool');
console.log('========================================');
console.log('');

// Main execution flow
(async () => {
    const ask = (query) => new Promise((resolve) => rl.question(query, resolve));

    // 1. Check Git Status
    try {
        execSync('git status --porcelain');
    } catch (e) {
        // If output is not empty (or command fails), we might have changes
    }
    
    // We can't easily check output of execSync with stdio: inherit, so let's just try to add/commit if user wants
    // Or just always add .
    
    console.log('[Step 1] Checking Git status...');
    runCommand('git add .');
    // Try commit, ignore error if nothing to commit
    const commitMsg = await ask("Enter commit message for current changes (default: 'Release update'): ") || 'Release update';
    runCommand(`git commit -m "${commitMsg}"`, true);

    // 2. Version Management
    const currentVersion = getPackageVersion();
    console.log(`\nCurrent Version: ${currentVersion}`);
    
    const nextPatch = incrementVersion(currentVersion, 'patch');
    const versionInput = await ask(`Enter new version (default: ${nextPatch}): `);
    const newVersion = versionInput.trim() || nextPatch;

    if (newVersion !== currentVersion) {
        updateVersion(newVersion);
        // Commit version bump
        runCommand('git add package.json src-tauri/tauri.conf.json');
        runCommand(`git commit -m "chore: bump version to ${newVersion}"`);
    }

    // 3. Load Signing Key
    const keyPath = path.join(__dirname, 'tauri.key');
    if (fs.existsSync(keyPath)) {
        try {
            const keyContent = fs.readFileSync(keyPath, 'utf-8').trim();
            process.env.TAURI_PRIVATE_KEY = keyContent;
            process.env.TAURI_KEY_PASSWORD = ''; 
            console.log('[Info] Loaded signing key from tauri.key');
        } catch (e) {
            console.warn('[Warning] Failed to read tauri.key:', e.message);
        }
    } else {
        console.warn('[Warning] tauri.key not found. Build may fail if updater is enabled.');
    }

    // 4. Build
    console.log('\n[Step 4] Building Application...');
    const startTime = Date.now();
    runCommand('npm run build:win');

    // 5. Copy Artifacts
    console.log('\n[Step 5] Processing Artifacts...');
    const releaseDir = path.join(__dirname, 'release');
    if (!fs.existsSync(releaseDir)) {
        fs.mkdirSync(releaseDir);
    }

    const targetBase = path.join(__dirname, 'src-tauri', 'target', 'x86_64-pc-windows-msvc', 'release', 'bundle');
    const msiDir = path.join(targetBase, 'msi');
    const nsisDir = path.join(targetBase, 'nsis');

    let copiedCount = 0;

    // Copy MSI
    if (fs.existsSync(msiDir)) {
        const files = fs.readdirSync(msiDir).filter(f => f.endsWith('.msi'));
        files.forEach(file => {
            fs.copyFileSync(path.join(msiDir, file), path.join(releaseDir, file));
            console.log(`Copied: ${file}`);
            copiedCount++;
        });
    }

    // Copy NSIS (Exe)
    if (fs.existsSync(nsisDir)) {
        const files = fs.readdirSync(nsisDir).filter(f => f.endsWith('.exe'));
        files.forEach(file => {
            fs.copyFileSync(path.join(nsisDir, file), path.join(releaseDir, file));
            console.log(`Copied: ${file}`);
            copiedCount++;
        });
    }

    if (copiedCount === 0) {
        console.warn('[Warning] No artifacts found to copy.');
    }

    // 6. Git Push & Tag
    console.log('\n[Step 6] Git Push & Tag...');
    try {
        runCommand('git push');
        
        if (newVersion !== currentVersion) {
            const tagName = `v${newVersion}`;
            console.log(`Creating tag ${tagName}...`);
            runCommand(`git tag ${tagName}`);
            runCommand(`git push origin ${tagName}`);
        }
    } catch (e) {
        console.error('[Error] Git operations failed.');
    }

    const endTime = Date.now();
    const duration = ((endTime - startTime) / 1000).toFixed(1);

    console.log('\n========================================');
    console.log(`[Success] Release completed in ${duration}s!`);
    console.log('Artifacts are in the "release/" folder.');
    console.log('Code and Tags pushed to GitHub.');
    console.log('========================================');
    
    rl.close();
})();


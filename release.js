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

function runCommand(command) {
    try {
        execSync(command, { stdio: 'inherit' });
    } catch (error) {
        console.error(`[Error] Command failed: ${command}`);
        process.exit(1);
    }
}

console.log('========================================');
console.log(' Neko233 Hardware Viewer - Release Tool');
console.log('========================================');
console.log('');

rl.question("Enter commit message (default: 'Release update'): ", (answer) => {
    const commitMsg = answer.trim() || 'Release update';
    rl.close();

    const startTime = Date.now();

    // 1. Build
    console.log('\n[Step 1/4] Building Application...');
    // Ensure dependencies are installed? Maybe skip to save time, assume dev env.
    // runCommand('npm install'); 
    runCommand('npm run build:win');

    // 2. Copy Artifacts
    console.log('\n[Step 2/4] Processing Artifacts...');
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

    // 3. Git Operations
    console.log('\n[Step 3/4] Git Operations...');
    try {
        runCommand('git add .');
        runCommand(`git commit -m "${commitMsg}"`);
        
        console.log('\n[Step 4/4] Pushing to GitHub...');
        runCommand('git push');
    } catch (e) {
        console.error('[Error] Git operations failed.');
        process.exit(1);
    }

    const endTime = Date.now();
    const duration = ((endTime - startTime) / 1000).toFixed(1);

    console.log('\n========================================');
    console.log(`[Success] Release completed in ${duration}s!`);
    console.log('Artifacts are in the "release/" folder.');
    console.log('Code pushed to GitHub.');
    console.log('========================================');
});

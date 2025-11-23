import fs from 'fs';
import pngToIco from 'png-to-ico';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Source image - ensure this exists and is a valid PNG
const source = path.join(__dirname, 'src-tauri/icons/128x128.png');
const dest = path.join(__dirname, 'src-tauri/icons/icon.ico');

console.log(`Generating icon.ico from ${source}...`);

if (!fs.existsSync(source)) {
    console.error(`Source file not found: ${source}`);
    process.exit(1);
}

try {
    const buf = await pngToIco(source);
    fs.writeFileSync(dest, buf);
    console.log('Successfully generated src-tauri/icons/icon.ico');
} catch (e) {
    console.error('Error generating icon:', e);
    process.exit(1);
}

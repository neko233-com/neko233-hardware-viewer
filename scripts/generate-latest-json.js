const fs = require('fs');
const path = require('path');

const targetDir = process.argv[2];
const version = process.argv[3];
const notes = process.argv[4] || "Update";
const pubDate = new Date().toISOString();

if (!targetDir || !version) {
  console.error("Usage: node generate-latest-json.js <target-dir> <version> [notes]");
  process.exit(1);
}

// Find signature files
const files = fs.readdirSync(targetDir);
const sigFile = files.find(f => f.endsWith('.msi.zip.sig'));

if (!sigFile) {
  console.error("No .msi.zip.sig file found in " + targetDir);
  // Fallback or exit? For now, let's try to find any sig
}

// We need to construct the JSON structure expected by Tauri updater
// {
//   "version": "v1.0.0",
//   "notes": "...",
//   "pub_date": "...",
//   "platforms": {
//     "windows-x86_64": {
//       "signature": "CONTENT_OF_SIG_FILE",
//       "url": "https://github.com/.../download/v1.0.0/app.msi.zip"
//     }
//   }
// }

const platforms = {};

// Windows x64
const msiZip = files.find(f => f.endsWith('.msi.zip'));
const msiSig = files.find(f => f.endsWith('.msi.zip.sig'));

if (msiZip && msiSig) {
  const sigContent = fs.readFileSync(path.join(targetDir, msiSig), 'utf8');
  platforms['windows-x86_64'] = {
    signature: sigContent,
    url: `https://github.com/neko233-com/neko233-hardware-viewer/releases/download/v${version}/${msiZip}`
  };
}

const latestJson = {
  version: `v${version}`,
  notes: notes,
  pub_date: pubDate,
  platforms: platforms
};

fs.writeFileSync(path.join(targetDir, 'latest.json'), JSON.stringify(latestJson, null, 2));
console.log("Generated latest.json");

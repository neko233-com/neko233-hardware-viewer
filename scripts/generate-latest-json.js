import fs from 'fs';
import path from 'path';

const targetDir = process.argv[2];
const version = process.argv[3];
const notes = process.argv[4] || "Update";
const pubDate = new Date().toISOString();

if (!targetDir || !version) {
  console.error("Usage: node generate-latest-json.js <target-dir> <version> [notes]");
  process.exit(1);
}

// Find signature files
let files;
try {
  files = fs.readdirSync(targetDir);
  console.log(`Files in ${targetDir}:`, files);
} catch (e) {
  console.error(`Failed to read directory ${targetDir}: ${e.message}`);
  process.exit(1);
}

const platforms = {};

// Helper to find artifacts
function findArtifact(ext) {
  return files.find(f => f.endsWith(ext));
}

// Try MSI first, then NSIS
let installerZip = findArtifact('.msi.zip');
let installerSig = findArtifact('.msi.zip.sig');

if (!installerZip || !installerSig) {
  console.log("MSI artifacts not found (zip/sig), checking NSIS...");
  // If MSI not found, try to look in sibling 'nsis' directory if we are in 'msi'
  if (targetDir.endsWith('msi')) {
    const nsisDir = path.join(path.dirname(targetDir), 'nsis');
    console.log(`Checking NSIS directory: ${nsisDir}`);
    if (fs.existsSync(nsisDir)) {
      const nsisFiles = fs.readdirSync(nsisDir);
      console.log(`Files in NSIS dir:`, nsisFiles);
      const nsisZip = nsisFiles.find(f => f.endsWith('.nsis.zip'));
      const nsisSig = nsisFiles.find(f => f.endsWith('.nsis.zip.sig'));
      if (nsisZip && nsisSig) {
        installerZip = nsisZip;
        installerSig = nsisSig;
        // Update targetDir to read the sig file correctly
        // But wait, we need to read the file content.
        // Let's just read it here.
        const sigContent = fs.readFileSync(path.join(nsisDir, nsisSig), 'utf8');
        const cleanVersion = version.startsWith('v') ? version.substring(1) : version;
        platforms['windows-x86_64'] = {
          signature: sigContent,
          url: `https://raw.githubusercontent.com/neko233-com/neko233-hardware-viewer/main/release/neko233-hardware-viewer_${cleanVersion}_x64-setup.nsis.zip`
        };
        console.log(`Found NSIS artifact: ${nsisZip}`);
      } else {
          console.log("NSIS zip or sig not found.");
      }
    } else {
        console.log("NSIS directory does not exist.");
    }
  }
} else {
  const sigContent = fs.readFileSync(path.join(targetDir, installerSig), 'utf8');
  const cleanVersion = version.startsWith('v') ? version.substring(1) : version;
  platforms['windows-x86_64'] = {
    signature: sigContent,
    url: `https://raw.githubusercontent.com/neko233-com/neko233-hardware-viewer/main/release/neko233-hardware-viewer_${cleanVersion}_x64_en-US.msi.zip`
  };
  console.log(`Found MSI artifact: ${installerZip}`);
}

if (Object.keys(platforms).length === 0) {
  console.error("Error: No Windows artifacts (MSI or NSIS) found with signatures.");
  process.exit(1);
}

const latestJson = {
  version: version.startsWith('v') ? version : `v${version}`,
  notes: notes,
  pub_date: pubDate,
  platforms: platforms
};

fs.writeFileSync(path.join(targetDir, 'latest.json'), JSON.stringify(latestJson, null, 2));
console.log("Generated latest.json");

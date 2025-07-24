#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const releaseDir = path.resolve(__dirname, '../src-tauri/target/release');
const updaterDir = path.resolve(releaseDir, 'bundle/updater');

if (!fs.existsSync(updaterDir)) {
    console.log('Updater bundle directory not found. Skipping manifest generation.');
    process.exit(0);
}

const platforms = {};
const files = fs.readdirSync(updaterDir);

const tauriConfPath = path.resolve(__dirname, '../src-tauri/tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));
const version = tauriConf.version;
const now = new Date();

files.forEach(file => {
    if (file.endsWith('.json')) {
        const filePath = path.join(updaterDir, file);
        const content = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
        const platformName = file.replace('.json', '');
        
        const assetName = content.url.split('/').pop();

        platforms[platformName] = {
            signature: content.signature,
            url: `https://github.com/o0x1024/mytips/releases/download/v${version}/${assetName}`
        };
    }
});

const manifest = {
    version: `v${version}`,
    notes: `Release version v${version}`,
    pub_date: now.toISOString(),
    platforms: platforms
};

const manifestPath = path.join(releaseDir, 'latest.json');
fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));

console.log(`Update manifest generated at: ${manifestPath}`); 
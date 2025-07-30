#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// 确保输出目录存在
const distDir = path.resolve(__dirname, '../dist');
const updateManifestsDir = path.resolve(distDir, 'update-manifests');

if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir);
}

if (!fs.existsSync(updateManifestsDir)) {
    fs.mkdirSync(updateManifestsDir);
}

const releaseDir = path.resolve(__dirname, '../src-tauri/target/release');
const updaterDir = path.resolve(releaseDir, 'bundle/updater');

if (!fs.existsSync(updaterDir)) {
    console.log('Updater bundle directory not found. Creating empty manifest.');
    
    // 获取版本信息
    const tauriConfPath = path.resolve(__dirname, '../src-tauri/tauri.conf.json');
    const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));
    const version = process.env.npm_package_version || tauriConf.package.version;
    
    // 创建空的清单文件
    const emptyManifest = {
        version,
        notes: `Release version ${version}`,
        pub_date: new Date().toISOString(),
        platforms: {}
    };
    
    // 写入空的清单文件
    const latestJsonPath = path.join(updateManifestsDir, 'latest.json');
    fs.writeFileSync(latestJsonPath, JSON.stringify(emptyManifest, null, 2));
    
    console.log(`Empty update manifest generated at: ${latestJsonPath}`);
    process.exit(0);
}

// 处理各平台的更新清单
const platforms = {};
const files = fs.readdirSync(updaterDir);

const tauriConfPath = path.resolve(__dirname, '../src-tauri/tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));
const version = process.env.npm_package_version || tauriConf.package.version;
const now = new Date();

// 读取GitHub仓库信息
const repoName = process.env.GITHUB_REPOSITORY || 'o0x1024/mytips';
const releaseNotes = process.env.RELEASE_NOTES || 'See the release notes for details.';

files.forEach(file => {
    if (file.endsWith('.json')) {
        const filePath = path.join(updaterDir, file);
        const content = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
        const platformName = file.replace('.json', '');
        
        // 确保URL使用正确的版本号
        const assetName = content.url.split('/').pop();
        
        platforms[platformName] = {
            signature: content.signature,
            url: `https://github.com/${repoName}/releases/download/v${version}/${assetName}`
        };
        
        // 同时保存单独的平台清单文件，以兼容旧版本
        const platformManifest = {
            version,
            notes: releaseNotes,
            pub_date: now.toISOString(),
            platforms: {
                [platformName]: platforms[platformName]
            }
        };
        
        const platformManifestPath = path.join(updateManifestsDir, `${platformName}.json`);
        fs.writeFileSync(platformManifestPath, JSON.stringify(platformManifest, null, 2));
        console.log(`Platform manifest generated at: ${platformManifestPath}`);
    }
});

// 创建包含所有平台的统一清单
const manifest = {
    version,
    notes: releaseNotes,
    pub_date: now.toISOString(),
    platforms: platforms
};

// 保存统一的清单文件
const latestJsonPath = path.join(updateManifestsDir, 'latest.json');
fs.writeFileSync(latestJsonPath, JSON.stringify(manifest, null, 2));

console.log(`Combined update manifest generated at: ${latestJsonPath}`); 
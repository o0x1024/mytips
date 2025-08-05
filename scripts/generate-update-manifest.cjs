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
    console.log('Updater bundle directory not found. Creating manifest from existing bundles.');
    
    // 获取版本信息
    const tauriConfPath = path.resolve(__dirname, '../src-tauri/tauri.conf.json');
    const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));
    const version = process.env.npm_package_version || tauriConf.version || '1.1.4';
    
    // 读取GitHub仓库信息
    const repoName = process.env.GITHUB_REPOSITORY || 'o0x1024/mytips';
    const releaseNotes = process.env.RELEASE_NOTES || 'See the release notes for details.';
    
    // 根据截图中的文件列表创建平台清单
    const platforms = {
        'darwin-aarch64': {
            signature: 'placeholder-signature',
            url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-aarch64.app.tar.gz`
        },
        'darwin-x86_64': {
            signature: 'placeholder-signature', 
            url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-x86_64.dmg`
        },
        'linux-x86_64': {
            signature: 'placeholder-signature',
            url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-amd64.AppImage`
        },
        'windows-x86_64': {
            signature: 'placeholder-signature',
            url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-x64-setup.exe`
        }
    };
    
    // 检查本地构建文件以更新实际存在的平台
    const bundleDir = path.resolve(releaseDir, 'bundle');
    const localPlatforms = {};
    
    // 平台映射表
    const platformMapping = {
        'aarch64': 'darwin-aarch64',
        'amd64': 'linux-x86_64', 
        'x86_64': 'windows-x86_64',
        'x86_64.AppImage': 'linux-x86_64',
        'android': 'android-arm64'
    };
    
    // 检查 macOS 构建文件
    const macosDir = path.join(bundleDir, 'macos');
    if (fs.existsSync(macosDir)) {
        const tarGzFile = path.join(macosDir, 'MyTips.app.tar.gz');
        if (fs.existsSync(tarGzFile)) {
            localPlatforms['darwin-aarch64'] = {
                signature: 'placeholder-signature',
                url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-aarch64.app.tar.gz`
            };
        }
    }
    
    // 检查 DMG 文件
    const dmgDir = path.join(bundleDir, 'dmg');
    if (fs.existsSync(dmgDir)) {
        const dmgFiles = fs.readdirSync(dmgDir).filter(f => f.endsWith('.dmg'));
        dmgFiles.forEach(dmgFile => {
            if (dmgFile.includes('aarch64')) {
                localPlatforms['darwin-aarch64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${dmgFile}`
                };
            } else if (dmgFile.includes('x86_64')) {
                localPlatforms['darwin-x86_64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${dmgFile}`
                };
            }
        });
    }
    
    // 检查 AppImage 文件
    const appimageDir = path.join(bundleDir, 'appimage');
    if (fs.existsSync(appimageDir)) {
        const appimageFiles = fs.readdirSync(appimageDir).filter(f => f.endsWith('.AppImage'));
        appimageFiles.forEach(appimageFile => {
            if (appimageFile.includes('amd64')) {
                localPlatforms['linux-x86_64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${appimageFile}`
                };
            }
        });
    }
    
    // 检查 DEB 文件
    const debDir = path.join(bundleDir, 'deb');
    if (fs.existsSync(debDir)) {
        const debFiles = fs.readdirSync(debDir).filter(f => f.endsWith('.deb'));
        debFiles.forEach(debFile => {
            if (debFile.includes('amd64')) {
                localPlatforms['linux-x86_64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${debFile}`
                };
            }
        });
    }
    
    // 检查 RPM 文件
    const rpmDir = path.join(bundleDir, 'rpm');
    if (fs.existsSync(rpmDir)) {
        const rpmFiles = fs.readdirSync(rpmDir).filter(f => f.endsWith('.rpm'));
        rpmFiles.forEach(rpmFile => {
            if (rpmFile.includes('x86_64')) {
                localPlatforms['linux-x86_64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${rpmFile}`
                };
            }
        });
    }
    
    // 检查 Windows 安装包
    const nsis = path.join(bundleDir, 'nsis');
    if (fs.existsSync(nsis)) {
        const exeFiles = fs.readdirSync(nsis).filter(f => f.endsWith('.exe'));
        exeFiles.forEach(exeFile => {
            if (exeFile.includes('x64')) {
                localPlatforms['windows-x86_64'] = {
                    signature: 'placeholder-signature',
                    url: `https://github.com/${repoName}/releases/download/v${version}/${exeFile}`
                };
            }
        });
    }
    
    // 检查 Android APK
    const androidDir = path.join(bundleDir, 'android');
    if (fs.existsSync(androidDir)) {
        const apkFiles = fs.readdirSync(androidDir).filter(f => f.endsWith('.apk'));
        apkFiles.forEach(apkFile => {
            localPlatforms['android-arm64'] = {
                signature: 'placeholder-signature',
                url: `https://github.com/${repoName}/releases/download/v${version}/${apkFile}`
            };
        });
    }
    
    // 合并默认平台和本地平台配置（本地文件优先）
    const finalPlatforms = { ...platforms, ...localPlatforms };
    
    // 创建清单文件
    const manifest = {
        version,
        notes: releaseNotes,
        pub_date: new Date().toISOString(),
        platforms: finalPlatforms
    };
    
    // 写入清单文件
    const latestJsonPath = path.join(updateManifestsDir, 'latest.json');
    fs.writeFileSync(latestJsonPath, JSON.stringify(manifest, null, 2));
    
    console.log(`Update manifest generated at: ${latestJsonPath}`);
    console.log('Platforms found:', Object.keys(finalPlatforms));
    console.log('Local platforms detected:', Object.keys(localPlatforms));
    console.log('Default platforms included:', Object.keys(platforms).filter(p => !localPlatforms[p]));
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
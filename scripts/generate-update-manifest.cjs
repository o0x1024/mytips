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
    const releaseNotes = process.env.RELEASE_NOTES || 'editor instead use codemirror and fix bug';
    
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
    
    // 读取签名文件的辅助函数
    function readSignatureFile(sigPath) {
        try {
            if (fs.existsSync(sigPath)) {
                const signature = fs.readFileSync(sigPath, 'utf-8').trim();
                console.log(`Successfully read signature from ${sigPath}: ${signature.substring(0, 50)}...`);
                return signature;
            } else {
                console.warn(`Signature file not found: ${sigPath}`);
            }
        } catch (error) {
            console.warn(`Failed to read signature file ${sigPath}:`, error.message);
        }
        return 'placeholder-signature';
    }
    
    // 从已移动的文件中读取签名的辅助函数
    function readSignatureFromMovedFiles(targetDir, fileName) {
        const sigFileName = fileName + '.sig';
        const possiblePaths = [
            path.join(targetDir, sigFileName),
            path.join(targetDir, '..', sigFileName),
            path.join(targetDir, '..', '..', sigFileName)
        ];
        
        for (const sigPath of possiblePaths) {
            if (fs.existsSync(sigPath)) {
                try {
                    const signature = fs.readFileSync(sigPath, 'utf-8').trim();
                    console.log(`Found signature file at ${sigPath}: ${signature.substring(0, 50)}...`);
                    return signature;
                } catch (error) {
                    console.warn(`Failed to read signature file ${sigPath}:`, error.message);
                }
            }
        }
        
        console.warn(`No signature file found for ${fileName}`);
        return 'placeholder-signature';
    }
    
    // 检查本地构建文件以更新实际存在的平台
    const targetDir = path.resolve(__dirname, '../src-tauri/target');
    const localPlatforms = {};
    
    // 查找所有可能的目标架构目录
    const targetTriples = [
        'aarch64-apple-darwin',
        'x86_64-apple-darwin', 
        'x86_64-unknown-linux-gnu',
        'x86_64-pc-windows-msvc',
        'aarch64-linux-android'
    ];
    
    // 为每个目标架构检查构建文件
    const allBundleDirs = [];
    targetTriples.forEach(triple => {
        const bundleDir = path.join(targetDir, triple, 'release', 'bundle');
        if (fs.existsSync(bundleDir)) {
            allBundleDirs.push(bundleDir);
        }
    });
    
    // 检查 GitHub Actions 下载的构建产物目录
    const actionArtifactDirs = [
        'tauri-build-darwin-aarch64',
        'tauri-build-darwin-x86_64',
        'tauri-build-linux-x86_64', 
        'tauri-build-windows-x86_64',
        'tauri-build-android'
    ];
    
    actionArtifactDirs.forEach(artifactDir => {
        const artifactPath = path.join(targetDir, artifactDir);
        if (fs.existsSync(artifactPath)) {
            // 查找 bundle 目录
            const findBundleDir = (dir) => {
                const items = fs.readdirSync(dir);
                for (const item of items) {
                    const itemPath = path.join(dir, item);
                    if (fs.statSync(itemPath).isDirectory()) {
                        if (item === 'bundle') {
                            return itemPath;
                        }
                        const subBundle = findBundleDir(itemPath);
                        if (subBundle) return subBundle;
                    }
                }
                return null;
            };
            
            const bundleDir = findBundleDir(artifactPath);
            if (bundleDir) {
                allBundleDirs.push(bundleDir);
            }
        }
    });
    
    // 如果没有找到特定架构的目录，回退到通用目录
    const fallbackBundleDir = path.resolve(releaseDir, 'bundle');
    if (allBundleDirs.length === 0 && fs.existsSync(fallbackBundleDir)) {
        allBundleDirs.push(fallbackBundleDir);
    }
    
    // 平台映射表
    const platformMapping = {
        'aarch64': 'darwin-aarch64',
        'amd64': 'linux-x86_64', 
        'x86_64': 'windows-x86_64',
        'x86_64.AppImage': 'linux-x86_64',
        'android': 'android-arm64'
    };
    
    // 遍历所有找到的 bundle 目录
    allBundleDirs.forEach(bundleDir => {
        console.log(`Checking bundle directory: ${bundleDir}`);
        
        // 检查 macOS 构建文件
        const macosDir = path.join(bundleDir, 'macos');
        if (fs.existsSync(macosDir)) {
            const tarGzFile = path.join(macosDir, 'MyTips.app.tar.gz');
            if (fs.existsSync(tarGzFile)) {
                const sigFile = tarGzFile + '.sig';
                localPlatforms['darwin-aarch64'] = {
                    signature: readSignatureFile(sigFile),
                    url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-aarch64.app.tar.gz`
                };
            }
        }
        
        // 检查 DMG 文件
        const dmgDir = path.join(bundleDir, 'dmg');
        if (fs.existsSync(dmgDir)) {
            const dmgFiles = fs.readdirSync(dmgDir).filter(f => f.endsWith('.dmg'));
            dmgFiles.forEach(dmgFile => {
                const dmgPath = path.join(dmgDir, dmgFile);
                const sigFile = dmgPath + '.sig';
                if (dmgFile.includes('aarch64')) {
                    localPlatforms['darwin-aarch64'] = {
                        signature: readSignatureFile(sigFile),
                        url: `https://github.com/${repoName}/releases/download/v${version}/${dmgFile}`
                    };
                } else if (dmgFile.includes('x86_64')) {
                    localPlatforms['darwin-x86_64'] = {
                        signature: readSignatureFile(sigFile),
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
                    const appImagePath = path.join(appimageDir, appimageFile);
                    const sigFile = appImagePath + '.sig';
                    localPlatforms['linux-x86_64'] = {
                        signature: readSignatureFile(sigFile),
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
                    const debPath = path.join(debDir, debFile);
                    const sigFile = debPath + '.sig';
                    localPlatforms['linux-x86_64'] = {
                        signature: readSignatureFile(sigFile),
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
                    const rpmPath = path.join(rpmDir, rpmFile);
                    const sigFile = rpmPath + '.sig';
                    localPlatforms['linux-x86_64'] = {
                        signature: readSignatureFile(sigFile),
                        url: `https://github.com/${repoName}/releases/download/v${version}/${rpmFile}`
                    };
                }
            });
        }
        
        // 检查 Windows 安装包
        const nsisDir = path.join(bundleDir, 'nsis');
        if (fs.existsSync(nsisDir)) {
            const exeFiles = fs.readdirSync(nsisDir).filter(f => f.endsWith('.exe'));
            exeFiles.forEach(exeFile => {
                if (exeFile.includes('x64')) {
                    const exePath = path.join(nsisDir, exeFile);
                    const sigFile = exePath + '.sig';
                    console.log(`Found Windows exe: ${exeFile}, checking sig: ${sigFile}`);
                    localPlatforms['windows-x86_64'] = {
                        signature: readSignatureFile(sigFile),
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
                const apkPath = path.join(androidDir, apkFile);
                const sigFile = apkPath + '.sig';
                localPlatforms['android-arm64'] = {
                    signature: readSignatureFile(sigFile),
                    url: `https://github.com/${repoName}/releases/download/v${version}/${apkFile}`
                };
            });
        }
    });
    
    // 特殊处理 Android 构建产物（因为路径结构不同）
    const androidArtifactDir = path.join(targetDir, 'tauri-build-android');
    if (fs.existsSync(androidArtifactDir)) {
        console.log(`Checking Android artifact directory: ${androidArtifactDir}`);
        
        // 递归查找 APK 和 AAB 文件
        const findAndroidFiles = (dir) => {
            const items = fs.readdirSync(dir);
            for (const item of items) {
                const itemPath = path.join(dir, item);
                if (fs.statSync(itemPath).isDirectory()) {
                    findAndroidFiles(itemPath);
                } else if (item.endsWith('.apk')) {
                    const sigFile = itemPath + '.sig';
                    console.log(`Found Android APK: ${item}, checking sig: ${sigFile}`);
                    localPlatforms['android-arm64'] = {
                        signature: readSignatureFile(sigFile),
                        url: `https://github.com/${repoName}/releases/download/v${version}/mytips-${version}-android-release.apk`
                    };
                } else if (item.endsWith('.aab')) {
                    const sigFile = itemPath + '.sig';
                    console.log(`Found Android AAB: ${item}, checking sig: ${sigFile}`);
                    // AAB 文件通常用于 Google Play Store，这里可以根据需要添加处理逻辑
                }
            }
        };
        
        findAndroidFiles(androidArtifactDir);
    }
    
    // 检查是否有已移动的签名文件（在 GitHub Actions 环境中）
    const uploadDir = process.env.UPLOAD_DIR;
    console.log(`Environment UPLOAD_DIR: ${uploadDir}`);
    
    if (uploadDir && fs.existsSync(uploadDir)) {
        console.log(`Checking for moved signature files in: ${uploadDir}`);
        console.log(`Directory contents:`, fs.readdirSync(uploadDir));
        
        // 检查各平台的签名文件
        const platformFiles = {
            'darwin-aarch64': `mytips-${version}-aarch64.app.tar.gz`,
            'darwin-x86_64': `mytips-${version}-x86_64.dmg`,
            'linux-x86_64': `mytips-${version}-amd64.AppImage`,
            'windows-x86_64': `mytips-${version}-x64-setup.exe`,
            'android-arm64': `mytips-${version}-android-release.apk`
        };
        
        Object.entries(platformFiles).forEach(([platform, fileName]) => {
            const sigFile = path.join(uploadDir, fileName + '.sig');
            console.log(`Looking for signature file: ${sigFile}`);
            
            if (fs.existsSync(sigFile)) {
                try {
                    const signature = fs.readFileSync(sigFile, 'utf-8').trim();
                    console.log(`✓ Found moved signature for ${platform}: ${signature.substring(0, 50)}...`);
                    localPlatforms[platform] = {
                        signature: signature,
                        url: `https://github.com/${repoName}/releases/download/v${version}/${fileName}`
                    };
                } catch (error) {
                    console.warn(`Failed to read moved signature file ${sigFile}:`, error.message);
                }
            } else {
                console.warn(`✗ Signature file not found: ${sigFile}`);
            }
        });
    } else {
        console.log(`UPLOAD_DIR not set or doesn't exist, using original signature detection method`);
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
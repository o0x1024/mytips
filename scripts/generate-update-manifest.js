#!/usr/bin/env node

import fs from 'fs'
import path from 'path'
import crypto from 'crypto'
import { fileURLToPath } from 'url'

// 获取当前文件的目录路径（ES 模块中的 __dirname 替代方案）
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// 配置
const config = {
  // 从 package.json 读取版本
  version: process.env.npm_package_version || '0.1.0',
  // 发布时间
  pub_date: new Date().toISOString(),
  // 更新说明
  notes: process.env.RELEASE_NOTES || '修复编辑器代码块主题切换问题',
  // GitHub 仓库信息
  repo: process.env.GITHUB_REPOSITORY || 'o0x1024/mytips',
  // 是否为预发布版本
  prerelease: process.env.PRERELEASE === 'true'
}

// 检查是否为 debug 构建
const isDebug = process.argv.includes('--debug');
if (isDebug) {
  console.log('💡 检测到 Debug 构建模式，将跳过签名。');
}

// 支持的平台
const platforms = {
  'darwin-x86_64': {
    signature: '',
    url: `https://github.com/${config.repo}/releases/download/v${config.version}/mytips-${config.version}-x86_64.app.tar.gz`
  },
  'darwin-aarch64': {
    signature: '',
    url: `https://github.com/${config.repo}/releases/download/v${config.version}/mytips-${config.version}-aarch64.app.tar.gz`
  },
  'linux-x86_64': {
    signature: '',
    url: `https://github.com/${config.repo}/releases/download/v${config.version}/mytips-${config.version}-amd64.AppImage.tar.gz`
  },
  'windows-x86_64': {
    signature: '',
    url: `https://github.com/${config.repo}/releases/download/v${config.version}/mytips-${config.version}-x64-setup.nsis.zip`
  }
}

// 读取签名文件
function readSignature(platform) {
  let sigPath
  let targetDir = ''

  // 根据平台确定 Rust 目标三元组
  switch (platform) {
    case 'darwin-x86_64':
      targetDir = 'x86_64-apple-darwin';
      break;
    case 'darwin-aarch64':
      targetDir = 'aarch64-apple-darwin';
      break;
    case 'linux-x86_64':
      targetDir = 'x86_64-unknown-linux-gnu';
      break;
    case 'windows-x86_64':
      targetDir = 'x86_64-pc-windows-msvc';
      break;
  }

  // 定义原生和交叉编译的路径
  const crossCompilePath = targetDir ? path.join(__dirname, '..', 'src-tauri', 'target', targetDir, 'release', 'bundle') : null;
  const nativePath = path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle');

  // 优先使用交叉编译路径（在CI环境中更常见），否则回退到原生路径
  const bundlePath = crossCompilePath && fs.existsSync(crossCompilePath) ? crossCompilePath : nativePath;

  // 根据平台确定签名文件的具体路径
  const version = config.version;
  
  const findSigFile = (dir, extension) => {
    if (!fs.existsSync(dir)) return null;
    const files = fs.readdirSync(dir);
    const sigFile = files.find(f => f.endsWith(extension));
    return sigFile ? path.join(dir, sigFile) : null;
  };

  if (platform.startsWith('darwin-')) {
    // Tauri 通常不会在 macos 的 .app 包名中加入版本号
    sigPath = path.join(bundlePath, 'macos', `mytips.app.tar.gz.sig`);
  } else if (platform === 'linux-x86_64') {
    sigPath = path.join(bundlePath, 'appimage', `mytips_${version}_amd64.AppImage.tar.gz.sig`);
  } else if (platform === 'windows-x86_64') {
    // 动态查找 NSIS 或 MSI 签名文件，优先使用 NSIS
    const nsisSigPath = findSigFile(path.join(bundlePath, 'nsis'), '.nsis.zip.sig');
    const msiSigPath = findSigFile(path.join(bundlePath, 'msi'), '.msi.zip.sig');
    sigPath = nsisSigPath || msiSigPath;
  } else {
    console.warn(`警告: 未知平台 ${platform}`);
    return null;
  }
  
  try {
    if (sigPath && fs.existsSync(sigPath)) {
      return fs.readFileSync(sigPath, 'utf8').trim()
    } else {
      console.warn(`警告: 签名文件不存在于预期路径: ${sigPath || '未找到'}`)
      return null
    }
  } catch (error) {
    console.error(`读取签名文件失败 ${sigPath}:`, error.message)
    return null
  }
}

// 生成统一清单
function generateUnifiedManifest(outputDir) {
  const manifest = {
    version: config.version,
    notes: config.notes,
    pub_date: config.pub_date,
    platforms: {}
  }
  
  Object.keys(platforms).forEach(platform => {
    const signature = readSignature(platform)
    const platformData = {
      url: platforms[platform].url
    }
    
    // 只有当签名存在且不是 debug 构建时才添加签名字段
    if (signature && !isDebug) {
      platformData.signature = signature
    }
    
    manifest.platforms[platform] = platformData
  })
  
  const manifestPath = path.join(outputDir, 'latest.json')
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2))
  console.log(`✅ 已生成统一清单: ${manifestPath}`)
}

// 主函数
function main() {
  const outputDir = path.join(__dirname, '..', 'dist', 'update-manifests')
  
  // 确保输出目录存在
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true })
  }

  console.log('正在生成更新清单...')
  console.log(`版本: ${config.version}`)
  console.log(`发布时间: ${config.pub_date}`)

  try {
    // 生成统一清单（包含所有平台）
    generateUnifiedManifest(outputDir)

    // 生成各平台特定清单
    Object.keys(platforms).forEach(platform => {
      const signature = readSignature(platform)
      const platformManifest = {
        version: config.version,
        notes: config.notes,
        pub_date: config.pub_date,
        url: platforms[platform].url
      }
      
      // 只有当签名存在且不是 debug 构建时才添加签名字段
      if (signature && !isDebug) {
        platformManifest.signature = signature
      }
      
      const platformPath = path.join(outputDir, `${platform}.json`)
      fs.writeFileSync(platformPath, JSON.stringify(platformManifest, null, 2))
      console.log(`✅ 已生成 ${platform} 清单: ${platformPath}`)
    })

    // 生成部署脚本
    generateDeployScript(outputDir)

    console.log('\n🎉 更新清单生成完成!')
    console.log('\n📝 下一步操作:')
    console.log('1. 构建应用: yarn tauri build')
    console.log('2. 创建 GitHub Release')
    console.log('3. 上传构建产物到 Release')
    console.log('4. 上传更新清单到 Release')

  } catch (error) {
    console.error('❌ 生成更新清单失败:', error.message)
    process.exit(1)
  }
}

// 生成部署脚本
function generateDeployScript(outputDir) {
  const projectRoot = path.join(__dirname, '..', '..');

  const findBundleDir = (platform) => {
    const BUNDLE_DIR = path.join(projectRoot, 'src-tauri', 'target', 'release', 'bundle');
    let crossBundleDir = '';
    switch (platform) {
      case 'darwin-aarch64':
        crossBundleDir = path.join(projectRoot, 'src-tauri', 'target', 'aarch64-apple-darwin', 'release', 'bundle');
        break;
      case 'darwin-x86_64':
        crossBundleDir = path.join(projectRoot, 'src-tauri', 'target', 'x86_64-apple-darwin', 'release', 'bundle');
        break;
      case 'windows-x86_64':
        crossBundleDir = path.join(projectRoot, 'src-tauri', 'target', 'x86_64-pc-windows-msvc', 'release', 'bundle');
        break;
      case 'linux-x86_64':
        crossBundleDir = path.join(projectRoot, 'src-tauri', 'target', 'x86_64-unknown-linux-gnu', 'release', 'bundle');
        break;
    }
    return crossBundleDir && fs.existsSync(crossBundleDir) ? crossBundleDir : BUNDLE_DIR;
  };

  const getAssetPaths = (platform) => {
    const bundleDir = findBundleDir(platform);
    const assets = [];
    
    const findFiles = (dir, pattern) => {
      if (!fs.existsSync(dir)) return [];
      return fs.readdirSync(dir)
        .filter(f => f.match(pattern))
        .map(f => path.relative(projectRoot, path.join(dir, f)));
    };
    
    switch (platform) {
      case 'darwin-aarch64':
      case 'darwin-x86_64':
        assets.push(...findFiles(path.join(bundleDir, 'macos'), /^mytips\.app\.tar\.gz(\.sig)?$/));
        assets.push(...findFiles(path.join(bundleDir, 'dmg'), /^mytips_.*\.dmg$/));
        break;
      case 'linux-x86_64':
        assets.push(...findFiles(path.join(bundleDir, 'appimage'), /^mytips_.*_amd64\.AppImage(\.tar\.gz(\.sig)?)?$/));
        break;
      case 'windows-x86_64':
        assets.push(...findFiles(path.join(bundleDir, 'nsis'), /^mytips_.*_x64-setup(\.exe|\.nsis\.zip(\.sig)?)?$/));
        assets.push(...findFiles(path.join(bundleDir, 'msi'), /^mytips_.*_x64_en-US(\.msi|\.msi\.zip(\.sig)?)?$/));
        break;
    }
    return assets;
  };

  const allAssets = Object.keys(platforms).flatMap(getAssetPaths);
  const uniqueAssets = [...new Set(allAssets)];
  const manifestPath = path.relative(projectRoot, path.join(outputDir, 'latest.json'));

  const deployScript = `#!/bin/bash

# GitHub Release 部署脚本
# 使用 GitHub CLI 创建 release 并上传文件

set -e

VERSION="${config.version}"
REPO="${config.repo}"
TAG="v$VERSION"
RELEASE_NOTES="${config.notes}"
PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "正在创建 GitHub Release..."

# 创建 release
gh release create "$TAG" \\
  --repo "$REPO" \\
  --title "v$VERSION" \\
  --notes "$RELEASE_NOTES" \\
  ${config.prerelease ? '--prerelease' : ''}

echo "正在上传构建产物..."

# List of assets to upload (paths are relative to project root)
ASSETS=(
${uniqueAssets.map(p => `  "${p.replace(/\\/g, '/')}"`).join('\n')}
)

for asset_relative_path in "\${ASSETS[@]}"; do
  asset_full_path="\$PROJECT_ROOT/\$asset_relative_path"
  if [ -f "\$asset_full_path" ]; then
    echo "Uploading \$asset_relative_path..."
    gh release upload "$TAG" --repo "$REPO" "\$asset_full_path"
  else
    echo "Warning: Asset not found, skipping: \$asset_full_path"
  fi
done

echo "正在上传更新清单..."

# 上传更新清单
manifest_full_path="\$PROJECT_ROOT/${manifestPath.replace(/\\/g, '/')}"
gh release upload "$TAG" \\
  --repo "$REPO" \\
  "\$manifest_full_path"

echo "✅ GitHub Release 创建完成!"
echo "📍 Release URL: https://github.com/$REPO/releases/tag/$TAG"

echo ""
echo "📦 已上传的文件:"
gh release view "$TAG" --repo "$REPO" --json assets --jq '.assets[].name' | sort
`

  const scriptPath = path.join(outputDir, 'deploy.sh')
  fs.writeFileSync(scriptPath, deployScript)
  
  // 使脚本可执行
  try {
    fs.chmodSync(scriptPath, '755')
  } catch (error) {
    console.warn('警告: 无法设置部署脚本的执行权限')
  }
  
  console.log(`✅ 已生成部署脚本: ${scriptPath}`)
}

// 检查是否为直接运行（ES 模块中的 require.main === module 替代方案）
if (import.meta.url === `file://${process.argv[1]}`) {
  main()
}

// 导出函数
export {
  generateUnifiedManifest,
  config
} 
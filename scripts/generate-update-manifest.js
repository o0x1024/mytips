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
  if (platform.startsWith('darwin-')) {
    // Tauri 通常不会在 macos 的 .app 包名中加入版本号
    sigPath = path.join(bundlePath, 'macos', `mytips.app.tar.gz.sig`);
  } else if (platform === 'linux-x86_64') {
    sigPath = path.join(bundlePath, 'appimage', `mytips_${version}_amd64.AppImage.tar.gz.sig`);
  } else if (platform === 'windows-x86_64') {
    // 检查 NSIS 和 MSI 两种可能
    const nsisSigPath = path.join(bundlePath, 'nsis', `mytips_${version}_x64-setup.nsis.zip.sig`);
    const msiSigPath = path.join(bundlePath, 'msi', `mytips_${version}_x64_en-US.msi.zip.sig`);
    sigPath = fs.existsSync(nsisSigPath) ? nsisSigPath : msiSigPath;
  } else {
    console.warn(`警告: 未知平台 ${platform}`);
    return null;
  }
  
  try {
    if (fs.existsSync(sigPath)) {
      return fs.readFileSync(sigPath, 'utf8').trim()
    } else {
      console.warn(`警告: 签名文件不存在: ${sigPath}`)
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
    
    // 只有当签名存在时才添加签名字段
    if (signature) {
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
      
      // 只有当签名存在时才添加签名字段
      if (signature) {
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
  const deployScript = `#!/bin/bash

# GitHub Release 部署脚本
# 使用 GitHub CLI 创建 release 并上传文件

set -e

VERSION="${config.version}"
REPO="${config.repo}"
TAG="v\$VERSION"
RELEASE_NOTES="${config.notes}"
PROJECT_ROOT="\$(cd "\$(dirname "\$0")/../.." && pwd)"

echo "正在创建 GitHub Release..."

# 创建 release
gh release create "\$TAG" \\
  --repo "\$REPO" \\
  --title "v\$VERSION" \\
  --notes "\$RELEASE_NOTES" \\
  ${config.prerelease ? '--prerelease' : ''}

echo "正在上传构建产物..."

# 定义构建产物路径
BUNDLE_DIR="\$PROJECT_ROOT/src-tauri/target/release/bundle"
WINDOWS_CROSS_BUNDLE_DIR="\$PROJECT_ROOT/src-tauri/target/x86_64-pc-windows-msvc/release/bundle"

# 检查并上传 macOS 构建产物
echo "检查 macOS 构建产物..."
if [ -f "\$BUNDLE_DIR/macos/MyTips.app.tar.gz" ]; then
  echo "上传 macOS 更新包..."
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$BUNDLE_DIR/macos/MyTips.app.tar.gz"
fi

if [ -f "\$BUNDLE_DIR/macos/MyTips.app.tar.gz.sig" ]; then
  echo "上传 macOS 签名文件..."
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$BUNDLE_DIR/macos/MyTips.app.tar.gz.sig"
fi

# 动态查找 DMG 文件（支持不同版本号）
DMG_FILE=\$(find "\$BUNDLE_DIR/dmg" -name "MyTips_*_aarch64.dmg" -type f | head -1)
if [ -n "\$DMG_FILE" ] && [ -f "\$DMG_FILE" ]; then
  echo "上传 macOS DMG 安装包: \$(basename "\$DMG_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$DMG_FILE"
fi

# 检查并上传 Windows 构建产物（支持交叉编译）
echo "检查 Windows 构建产物..."

# 检查交叉编译路径
if [ -d "\$WINDOWS_CROSS_BUNDLE_DIR" ]; then
  echo "检测到 Windows 交叉编译构建产物..."
  WINDOWS_BUNDLE_DIR="\$WINDOWS_CROSS_BUNDLE_DIR"
else
  echo "使用本地 Windows 构建产物..."
  WINDOWS_BUNDLE_DIR="\$BUNDLE_DIR"
fi

# 动态查找 MSI 文件
MSI_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/msi" -name "MyTips_*_x64_en-US.msi" -type f 2>/dev/null | head -1)
if [ -n "\$MSI_FILE" ] && [ -f "\$MSI_FILE" ]; then
  echo "上传 Windows MSI 安装包: \$(basename "\$MSI_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$MSI_FILE"
fi

MSI_ZIP_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/msi" -name "MyTips_*_x64_en-US.msi.zip" -type f 2>/dev/null | head -1)
if [ -n "\$MSI_ZIP_FILE" ] && [ -f "\$MSI_ZIP_FILE" ]; then
  echo "上传 Windows MSI 更新包: \$(basename "\$MSI_ZIP_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$MSI_ZIP_FILE"
fi

MSI_SIG_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/msi" -name "MyTips_*_x64_en-US.msi.zip.sig" -type f 2>/dev/null | head -1)
if [ -n "\$MSI_SIG_FILE" ] && [ -f "\$MSI_SIG_FILE" ]; then
  echo "上传 Windows MSI 签名文件: \$(basename "\$MSI_SIG_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$MSI_SIG_FILE"
fi

# 动态查找 NSIS 文件
NSIS_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/nsis" -name "MyTips_*_x64-setup.exe" -type f 2>/dev/null | head -1)
if [ -n "\$NSIS_FILE" ] && [ -f "\$NSIS_FILE" ]; then
  echo "上传 Windows NSIS 安装包: \$(basename "\$NSIS_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$NSIS_FILE"
fi

NSIS_ZIP_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/nsis" -name "MyTips_*_x64-setup.nsis.zip" -type f 2>/dev/null | head -1)
if [ -n "\$NSIS_ZIP_FILE" ] && [ -f "\$NSIS_ZIP_FILE" ]; then
  echo "上传 Windows NSIS 更新包: \$(basename "\$NSIS_ZIP_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$NSIS_ZIP_FILE"
fi

NSIS_SIG_FILE=\$(find "\$WINDOWS_BUNDLE_DIR/nsis" -name "MyTips_*_x64-setup.nsis.zip.sig" -type f 2>/dev/null | head -1)
if [ -n "\$NSIS_SIG_FILE" ] && [ -f "\$NSIS_SIG_FILE" ]; then
  echo "上传 Windows NSIS 签名文件: \$(basename "\$NSIS_SIG_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$NSIS_SIG_FILE"
fi

# 检查并上传 Linux 构建产物
echo "检查 Linux 构建产物..."
# 动态查找 AppImage 文件
APPIMAGE_FILE=\$(find "\$BUNDLE_DIR/appimage" -name "mytips_*_amd64.AppImage" -type f | head -1)
if [ -n "\$APPIMAGE_FILE" ] && [ -f "\$APPIMAGE_FILE" ]; then
  echo "上传 Linux AppImage 安装包: \$(basename "\$APPIMAGE_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$APPIMAGE_FILE"
fi

APPIMAGE_TAR_FILE=\$(find "\$BUNDLE_DIR/appimage" -name "mytips_*_amd64.AppImage.tar.gz" -type f | head -1)
if [ -n "\$APPIMAGE_TAR_FILE" ] && [ -f "\$APPIMAGE_TAR_FILE" ]; then
  echo "上传 Linux AppImage 更新包: \$(basename "\$APPIMAGE_TAR_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$APPIMAGE_TAR_FILE"
fi

APPIMAGE_SIG_FILE=\$(find "\$BUNDLE_DIR/appimage" -name "mytips_*_amd64.AppImage.tar.gz.sig" -type f | head -1)
if [ -n "\$APPIMAGE_SIG_FILE" ] && [ -f "\$APPIMAGE_SIG_FILE" ]; then
  echo "上传 Linux AppImage 签名文件: \$(basename "\$APPIMAGE_SIG_FILE")"
  gh release upload "\$TAG" \\
    --repo "\$REPO" \\
    "\$APPIMAGE_SIG_FILE"
fi

echo "正在上传更新清单..."

# 上传更新清单
gh release upload "\$TAG" \\
  --repo "\$REPO" \\
  "${outputDir}/latest.json" \\
  "${outputDir}/darwin-x86_64.json" \\
  "${outputDir}/darwin-aarch64.json" \\
  "${outputDir}/linux-x86_64.json" \\
  "${outputDir}/windows-x86_64.json"

echo "✅ GitHub Release 创建完成!"
echo "📍 Release URL: https://github.com/\$REPO/releases/tag/\$TAG"

echo ""
echo "📦 已上传的文件:"
gh release view "\$TAG" --repo "\$REPO" --json assets --jq '.assets[].name' | sort
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
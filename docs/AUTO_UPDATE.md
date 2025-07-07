# MyTips 自动更新功能

本文档介绍如何配置和使用 MyTips 应用的自动更新功能。

## 功能特性

- ✅ 自动检查 GitHub Release 更新
- ✅ 支持静默检查和手动检查
- ✅ 下载进度显示
- ✅ 安装状态监控
- ✅ 支持代理设置
- ✅ 错误处理和重试机制
- ✅ 多平台支持 (Windows, macOS, Linux)

## 配置步骤

### 1. 生成签名密钥

首次配置需要生成 Tauri 更新签名密钥：

```bash
# 生成密钥对
yarn tauri signer generate

# 这会生成两个文件：
# ~/.tauri/mytips.key (私钥，用于签名，不要提交到代码库)
# ~/.tauri/mytips.pub (公钥，需要添加到配置文件)
```

### 2. 配置 tauri.conf.json

将公钥内容添加到 `src-tauri/tauri.conf.json`：

```json
{
  "plugins": {
    "updater": {
      "pubkey": "你的公钥内容",
      "endpoints": [
        "https://github.com/your-username/mytips/releases/latest/download/{{target}}-{{arch}}.json"
      ]
    }
  },
  "bundle": {
    "createUpdaterArtifacts": true
  }
}
```

### 3. 更新 GitHub 仓库信息

修改生成脚本中的仓库信息：

```javascript
// scripts/generate-update-manifest.js
const config = {
  repo: process.env.GITHUB_REPOSITORY || 'your-username/mytips',
  // 其他配置...
}
```

## 构建和发布流程

### 1. 构建应用

```bash
# 构建应用并生成更新清单
yarn update:build
```

这个命令会：
- 编译前端代码
- 构建 Tauri 应用
- 生成各平台的安装包
- 生成更新清单文件

### 2. 创建 GitHub Release

#### 方式一：使用自动化脚本

```bash
# 设置环境变量
export GITHUB_REPOSITORY="your-username/mytips"
export RELEASE_NOTES="修复了一些已知问题，提升了用户体验。"

# 部署到 GitHub Release
yarn update:deploy
```

#### 方式二：手动创建

1. 在 GitHub 仓库中创建新的 Release
2. 上传构建产物：
   - Windows: `*.exe`, `*.msi`, `*.nsis.zip`
   - macOS: `*.dmg`, `*.app.tar.gz`
   - Linux: `*.deb`, `*.AppImage.tar.gz`
3. 上传更新清单文件：
   - `latest.json` (通用清单)
   - `darwin-x86_64.json`
   - `darwin-aarch64.json`
   - `linux-x86_64.json`
   - `windows-x86_64.json`

### 3. 验证更新

发布后，可以通过以下方式验证更新功能：

1. 降低本地版本号进行测试
2. 在开发模式下点击"检查更新"按钮
3. 查看控制台日志确认更新检测正常

## 前端使用

### 基本用法

UpdateManager 组件已集成到主布局中，会自动运行：

```vue
<template>
  <UpdateManager />
</template>
```

### 高级配置

```javascript
// 获取组件引用
const updateManager = ref()

// 设置自动检查（每30分钟检查一次）
updateManager.value?.setAutoCheck(true, 30)

// 设置代理
updateManager.value?.setProxy(true, 'http://proxy.example.com:8080')

// 手动检查更新
updateManager.value?.checkForUpdates()
```

### 监听更新事件

```javascript
import { listen } from '@tauri-apps/api/event'

// 监听下载进度
const unlistenProgress = await listen('update-progress', (event) => {
  console.log(`下载进度: ${event.payload}%`)
})

// 监听安装开始
const unlistenInstalling = await listen('update-installing', () => {
  console.log('开始安装更新...')
})

// 监听更新完成
const unlistenCompleted = await listen('update-completed', () => {
  console.log('更新安装完成')
})
```

## 后端 API

### 检查更新

```javascript
import { invoke } from '@tauri-apps/api/core'

// 基本检查
const updateInfo = await invoke('check_for_updates')

// 带配置的检查
const updateInfo = await invoke('check_for_updates_with_config', {
  timeoutSeconds: 30,
  proxy: 'http://proxy.example.com:8080'
})

console.log('更新信息:', updateInfo)
// {
//   version: "0.2.0",
//   date: "2024-01-15T10:30:00Z",
//   body: "修复了一些已知问题...",
//   available: true
// }
```

### 开始自动更新

```javascript
// 启动自动更新流程
await invoke('start_auto_update')
```

### 获取当前版本

```javascript
const version = await invoke('get_current_version')
```

### 设置更新端点

```javascript
// 动态切换更新源（如从稳定版切换到测试版）
await invoke('set_update_endpoints', {
  endpoints: [
    'https://github.com/user/repo/releases/latest/download/beta-{{target}}-{{arch}}.json'
  ]
})
```

## 故障排除

### 常见问题

1. **检查更新失败**
   - 检查网络连接
   - 确认 GitHub Release 存在
   - 验证更新清单文件格式

2. **签名验证失败**
   - 确认公钥配置正确
   - 检查签名文件是否正确生成
   - 验证私钥用于签名构建产物

3. **下载失败**
   - 检查文件 URL 是否可访问
   - 确认文件大小和格式正确
   - 尝试设置代理或更换网络

### 调试模式

在开发环境中启用详细日志：

```javascript
// 在控制台中启用调试
localStorage.setItem('debug-updater', 'true')

// 查看更新检查详情
console.log('检查更新...')
```

### 日志位置

- Windows: `%APPDATA%\com.mytips.app\logs\`
- macOS: `~/Library/Logs/com.mytips.app/`
- Linux: `~/.local/share/com.mytips.app/logs/`

## 安全注意事项

1. **私钥安全**：
   - 私钥文件不要提交到代码库
   - 使用环境变量或安全的密钥管理服务
   - 定期轮换签名密钥

2. **更新端点**：
   - 只使用 HTTPS 端点
   - 验证 SSL 证书
   - 考虑使用 CDN 加速下载

3. **权限管理**：
   - 限制 GitHub Release 的访问权限
   - 使用专用的部署账户
   - 启用二步验证

## GitHub Actions 集成

创建 `.github/workflows/release.yml` 自动化发布流程：

```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build-and-release:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          
      - name: Install dependencies
        run: yarn install
        
      - name: Build and generate manifests
        run: yarn update:build
        env:
          GITHUB_REPOSITORY: ${{ github.repository }}
          RELEASE_NOTES: ${{ github.event.head_commit.message }}
          
      - name: Create Release
        if: matrix.os == 'ubuntu-latest'
        uses: softprops/action-gh-release@v1
        with:
          files: |
            dist/update-manifests/*.json
            src-tauri/target/release/bundle/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## 总结

通过以上配置，MyTips 应用将具备完整的自动更新功能：

- 🚀 用户可以及时获得新版本
- 🔒 更新过程安全可靠
- 📱 支持多平台部署
- 🛠️ 开发者可以便捷发布

如果遇到问题，请查看故障排除部分或提交 Issue。 
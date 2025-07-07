# MyTips - 本地优先的笔记与代码片段管理利器

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="MyTips Logo" width="128" height="128">
</p>

<p align="center">
  <strong>一款为开发者和知识工作者打造的，集笔记、AI 助手、剪贴板管理于一体的跨平台桌面应用。</strong>
</p>

<p align="center">
  <img alt="License" src="https://img.shields.io/badge/license-GPL--3.0-blue.svg"/>
  <img alt="Version" src="https://img.shields.io/badge/version-1.0.1-brightgreen.svg"/>
  <img alt="Tauri" src="https://img.shields.io/badge/built%20with-Tauri-3C3C3C.svg?logo=tauri"/>
  <img alt="Vue.js" src="https://img.shields.io/badge/made%20with-Vue.js-4FC08D.svg?logo=vue.js"/>
  <img alt="Rust" src="https://img.shields.io/badge/backend-Rust-DEA584.svg?logo=rust"/>
</p>

---

## 🌟 简介

MyTips 是一款基于 Tauri 框架构建的现代化桌面应用，旨在提供一个**安全、高效、功能丰富**的个人知识管理中心。它将数据完全存储在本地，确保您的隐私安全，同时集成了强大的笔记系统、智能的 AI 助手和便捷的剪贴板历史工具，帮助您无缝捕捉、组织和运用信息。

## 📸 应用截图

![MyTips 主界面](./public/placeholder-for-main-ui.png)

## ✨ 主要特性

### 📝 强大的笔记系统
- 🗂️ **多层级笔记本**: 像文件系统一样组织您的笔记
- 🏷️ **标签系统**: 使用标签进行灵活的交叉分类
- ✍️ **实时 Markdown 编辑**: 支持所见即所得 (WYSIWYG) 和分屏预览模式
- ✨ **语法高亮**: 内置 Prism.js，支持超过100种语言的代码高亮
- 🔍 **全文搜索**: 毫秒级响应，快速定位笔记、标签或内容
- 🔒 **端到端加密**: 支持对单个笔记或整个笔记本进行加密
- 🔄 **导入/导出**: 轻松导入导出 Markdown 文件
- 🖼️ **图片点击放大**: 支持笔记内图片的预览和放大功能
- 📋 **复制粘贴增强**: 支持代码块一键复制，图片拖拽上传

### 🤖 智能 AI 助手
- 🌊 **流式输出**: 实时显示AI回复内容，提升交互体验
- 🔗 **笔记引用**: 使用 `#` 快速引用笔记内容，为 AI 提供上下文
- 🎭 **自定义角色**: 创建您的专属 AI 角色，适应不同工作场景
- 🤖 **多模型支持**: 集成 OpenAI, Gemini, Claude, DeepSeek, 通义千问, 豆包, Grok 等多种大语言模型
- 🖼️ **多模态对话**: 支持上传图片、文档与 AI 交互
- 💾 **对话管理**: 自动保存对话历史，方便回顾和导出
- 🎯 **反向布局聊天**: 最新消息在顶部显示，符合现代聊天习惯
- 🌐 **浮动AI助手**: 独立的浮动窗口，可在任何位置使用AI对话
- ❌ **流式取消**: 支持实时取消正在生成的AI回复
- 📤 **对话导出**: 支持将AI对话记录导出为文档

### 🎯 浮动AI助手
- 🪟 **独立窗口**: 独立的浮动聊天窗口，不影响主应用使用
- 📍 **自由拖拽**: 可自由拖拽到屏幕任意位置
- 🔄 **自动隐藏**: 智能边缘吸附和半透明显示
- 📏 **大小调整**: 支持窗口大小调整，适应不同使用场景
- 💾 **历史保存**: 自动保存浮动聊天历史，重启后恢复

### 📋 高效的剪贴板管理
- 📊 **历史记录**: 自动记录剪贴板历史，轻松找回复制过的内容
- 🛡️ **智能过滤**: 自定义规则，防止记录密码等敏感信息
- ⌨️ **快捷粘贴**: 快速搜索并粘贴历史记录
- 🔍 **内容预览**: 支持文本、图片等多种格式的内容预览

### 🎨 卓越的用户体验
- 🎨 **丰富主题**: 内置多种明亮和暗色主题，并支持自定义
- ⚡ **原生性能**: 基于 Rust 和 Tauri，提供流畅的跨平台体验
- ⌨️ **快捷键**: 丰富的快捷键支持，提升操作效率
- ⚙️ **系统集成**: 支持系统托盘、开机自启和全局快捷键
- 📱 **响应式设计**: 适配不同屏幕尺寸，提供最佳视觉体验
- 🔔 **状态反馈**: 完善的加载状态和操作反馈


## 📦 安装

您可以从下面的地址下载适用于您操作系统的最新版本：

[**➡️ 前往 GitHub Releases 下载**](https://github.com/o0x1024/mytips/releases)

- **Windows**: 下载 `.msi或exe` 安装包
- **macOS**: 下载 `.dmg` 文件 (支持 Apple Silicon 和 Intel 芯片)
- **Linux**: 下载 `.AppImage` 或 `.deb` 文件

## 🚀 快速开始

1. **创建笔记**: 启动应用后，点击侧边栏的 `新建笔记` 按钮开始您的第一篇笔记
2. **配置 AI**: 前往 `设置` -> `AI 助手` 页面，填入您拥有的大语言模型的 API 密钥
3. **与 AI 对话**: 进入 `AI 助手` 页面，开始与您的智能助手对话。在对话中输入 `#` 可以快速引用您的笔记
4. **使用浮动AI**: 点击浮动AI按钮，在独立窗口中使用AI助手
5. **查看剪贴板历史**: 默认情况下，剪贴板历史是开启的。您可以在 `剪贴板` 页面查看和搜索历史记录

## 💡 使用技巧

### AI助手使用技巧
- 💬 **笔记引用**: 在AI对话中输入 `#` 会弹出笔记选择器，选择相关笔记为AI提供上下文
- 🌊 **流式体验**: AI回复支持实时显示，您可以随时点击停止按钮取消生成
- 🪟 **浮动窗口**: 使用浮动AI助手可以在处理其他任务时随时咨询AI
- 📤 **对话导出**: 重要的AI对话可以导出保存，便于后续参考

### 笔记管理技巧
- 🔒 **加密保护**: 敏感笔记可以设置加密，保护您的隐私信息
- 🏷️ **标签分类**: 合理使用标签可以让笔记检索更加高效
- 📋 **快速复制**: 代码块和重要内容支持一键复制功能
- 🖼️ **图片管理**: 支持拖拽上传图片，点击图片可以放大查看

## 🛠️ 技术栈

| 类别       | 技术                                                                                             |
| ---------- | ------------------------------------------------------------------------------------------------ |
| **前端**     | `Vue 3` (Composition API), `TypeScript`, `Vite`, `Pinia`, `Vue Router`, `Tailwind CSS`, `DaisyUI` |
| **Markdown** | `Marked`, `DOMPurify`, `Prism.js`                                                            |
| **后端**     | `Rust`, `Tauri`, `Tokio`                                                                         |
| **数据库**   | `SQLite` (via `rusqlite`)                                                                        |
| **API 通信** | `Reqwest`                                                                                        |

## 💻 开发

### 环境要求
- **Node.js** `16+` (推荐 `18+`)
- **Rust** `1.70+` (stable toolchain)
- **系统依赖**: 请参考 [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites) 完成安装

### 安装与启动

```bash
# 1. 克隆项目
git clone <repository-url>
cd mytips

# 2. 安装前端依赖 (推荐使用 bun 或 pnpm)
bun install

# 3. 启动开发模式
bun tauri dev
```

## 🤝 贡献

我们非常欢迎社区的贡献！如果您有任何想法或建议，请随时提交 [Issue](https://github.com/your-username/mytips/issues) 或 [Pull Request](https://github.com/your-username/mytips/pulls)。

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 将分支推送到远程 (`git push origin feature/AmazingFeature`)
5. 打开一个 Pull Request

## 🔐 隐私与安全

- **本地优先**: 所有数据默认存储在本地，绝不上传至任何服务器
- **API 密钥安全**: 您的 API 密钥经过加密存储在本地
- **开源透明**: 代码完全开源，接受社区的审查
- **加密保护**: 支持笔记本和单个笔记的端到端加密

## 📝 许可证

本项目基于 [GPL](LICENSE) 许可证开源。

## 🙏 致谢

MyTips 的发展离不开以下优秀开源项目的支持：

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)
- 以及所有在 `package.json` 和 `Cargo.toml` 中列出的依赖库

---

**MyTips - 让知识管理更简单、更智能、更安全。** 🚀

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

*此处可以放置应用的主界面截图*
![MyTips 主界面](placeholder-for-main-ui.png)

## ✨ 主要特性

- **强大的笔记系统**:
    - 🗂️ **多层级笔记本**: 像文件系统一样组织您的笔记。
    - 🏷️ **标签系统**: 使用标签进行灵活的交叉分类。
    - ✍️ **实时 Markdown 编辑**: 支持所见即所得 (WYSIWYG) 和分屏预览模式。
    - ✨ **语法高亮**: 内置 Prism.js，支持超过100种语言的代码高亮。
    - 🔍 **全文搜索**: 毫秒级响应，快速定位笔记、标签或内容。
    - 🔒 **端到端加密**: 支持对单个笔记或整个笔记本进行加密。
    - 🔄 **导入/导出**: 轻松导入导出 Markdown 文件。

- **智能 AI 助手**:
    - 🤖 **多模型支持**: 集成 OpenAI, Gemini, Claude, DeepSeek, 通义千问, 豆包, Grok 等多种大语言模型。
    - 🎭 **自定义角色**: 创建您的专属 AI 角色，适应不同工作场景。
    - 🔗 **笔记引用**: 使用 `#` 快速引用笔记内容，为 AI 提供上下文。
    - 🖼️ **多模态对话**: 支持上传图片、文档与 AI 交互。
    - 💾 **对话管理**: 自动保存对话历史，方便回顾和导出。

- **高效的剪贴板管理**:
    - 📋 **历史记录**: 自动记录剪贴板历史，轻松找回复制过的内容。
    - 🗑️ **智能过滤**: 自定义规则，防止记录密码等敏感信息。
    - ⌨️ **快捷粘贴**: 快速搜索并粘贴历史记录。

- **卓越的用户体验**:
    - 🎨 **丰富主题**: 内置多种明亮和暗色主题，并支持自定义。
    - ⚡ **原生性能**: 基于 Rust 和 Tauri，提供流畅的跨平台体验。
    - ⌨️ **快捷键**: 丰富的快捷键支持，提升操作效率。
    - ⚙️ **系统集成**: 支持系统托盘、开机自启和全局快捷键。

## 📦 安装

您可以从下面的地址下载适用于您操作系统的最新版本：

[**➡️ 前往 GitHub Releases 下载**](https://github.com/your-username/mytips/releases)

- **Windows**: 下载 `.msi` 安装包。
- **macOS**: 下载 `.dmg` 文件 (支持 Apple Silicon 和 Intel 芯片)。
- **Linux**: 下载 `.AppImage` 或 `.deb` 文件。

## 🚀 快速开始

1.  **创建笔记**: 启动应用后，点击侧边栏的 `新建笔记` 按钮开始您的第一篇笔记。
2.  **配置 AI**: 前往 `设置` -> `AI 助手` 页面，填入您拥有的大语言模型的 API 密钥。
3.  **与 AI 对话**: 进入 `AI 助手` 页面，开始与您的智能助手对话。在对话中输入 `#` 可以快速引用您的笔记。
4.  **查看剪贴板历史**: 默认情况下，剪贴板历史是开启的。您可以在 `剪贴板` 页面查看和搜索历史记录。

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
- **系统依赖**: 请参考 [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites) 完成安装。

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

1.  Fork 本仓库
2.  创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3.  提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4.  将分支推送到远程 (`git push origin feature/AmazingFeature`)
5.  打开一个 Pull Request

## 🔐 隐私与安全

- **本地优先**: 所有数据默认存储在本地，绝不上传至任何服务器。
- **API 密钥安全**: 您的 API 密钥经过加密存储在本地。
- **开源透明**: 代码完全开源，接受社区的审查。

## 📝 许可证

本项目基于 [GPL-3.0](LICENSE) 许可证开源。

## 🙏 致谢

MyTips 的发展离不开以下优秀开源项目的支持：

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Vditor](https://github.com/Vanessa219/vditor)
- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)
- 以及所有在 `package.json` 和 `Cargo.toml` 中列出的依赖库。

---

**MyTips - 让知识管理更简单、更智能、更安全。** 🚀

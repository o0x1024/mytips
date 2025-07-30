<div align="center">

# MyTips

**Local-first intelligent note-taking and knowledge management platform**

*A modern desktop application designed for developers and knowledge workers*

<img src="src-tauri/icons/128x128.png" alt="MyTips Logo" width="120" height="120">

[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.1.0-brightgreen.svg)](https://github.com/o0x1024/mytips/releases)
[![Tauri](https://img.shields.io/badge/built%20with-Tauri-3C3C3C.svg?logo=tauri)](https://tauri.app/)
[![Vue.js](https://img.shields.io/badge/frontend-Vue.js-4FC08D.svg?logo=vue.js)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/backend-Rust-DEA584.svg?logo=rust)](https://www.rust-lang.org/)
[![Downloads](https://img.shields.io/github/downloads/o0x1024/mytips/total.svg)](https://github.com/o0x1024/mytips/releases)

[Download](https://github.com/o0x1024/mytips/releases) • [Documentation](#quick-start) • [Issues](https://github.com/o0x1024/mytips/issues) • [Discussions](https://github.com/o0x1024/mytips/discussions)

[中文](README.zh.md) | English

</div>

## Why Choose MyTips?

**Privacy First** - All data stored locally, no privacy concerns  
**Lightweight & Efficient** - Built with Rust + Tauri, fast startup and low resource usage  
**AI Enhanced** - Integrated with multiple LLMs, making AI your knowledge assistant  
**Native Markdown** - Standard Markdown support, no vendor lock-in  
**Real-time Sync** - Multi-device data synchronization, access notes anywhere  
**Modern Interface** - Beautiful UI with multiple theme support

MyTips is committed to creating a **secure, intelligent, and efficient** personal knowledge management center, perfectly integrating note-taking, AI conversations, and clipboard management to help you build your own knowledge system.

## Screenshots

![MyTips Main Interface](./public/placeholder-for-main-ui.png)

## Core Features

### Intelligent Note System


- **Hierarchical Management** - Multi-level notebook structure, intuitive organization like file system
- **Tag Classification** - Flexible tagging system with cross-classification and quick filtering
- **Markdown Editing** - WYSIWYG editor with real-time preview and split-screen mode
- **Syntax Highlighting** - Built-in support for 100+ programming languages
- **Full-text Search** - Millisecond search response, quickly locate any content
- **Encryption Protection** - End-to-end encryption for sensitive notes security
- **Import/Export** - Batch import and export of Markdown files
- **Media Support** - Drag-and-drop image upload with click-to-zoom preview
 
### AI Assistant


- **Streaming Conversations** - Real-time AI responses with interruption support
- **Note Integration** - Use `#` to reference note content, providing context for AI
- **Role Customization** - Create custom AI personas for different work scenarios
- **Multi-model Support** - Integrated with OpenAI, Gemini, Claude, DeepSeek, Qwen and other mainstream models
- **Multimodal Interaction** - Support image and document uploads for rich conversations
- **Floating Assistant** - Independent floating window for AI access anywhere
- **History Management** - Auto-save conversation records with export and sharing support

### Clipboard Management

- **History Records** - Automatically record clipboard content, never lose important information
- **Smart Filtering** - Custom rules to protect sensitive information security
- **Quick Operations** - Global shortcuts for fast search and paste
- **Multi-format Support** - Preview text, images, files and other formats

### Modern Experience

- **Theme Customization** - Multiple built-in themes with dark/light mode switching
- **Native Performance** - Rust + Tauri architecture, fast startup and low resource usage
- **Quick Operations** - Rich keyboard shortcuts support for improved efficiency
- **System Integration** - System tray, auto-start, global shortcuts
- **Responsive Interface** - Adaptive to various screen sizes with perfect display


## Download & Installation


### System Requirements

- **Windows** 10/11 (x64)
- **macOS** 10.15+ (Intel/Apple Silicon)
- **Linux** Ubuntu 18.04+ / Debian 10+ / Fedora 32+

### Download Packages

| Platform | Download Link | File Format |
|----------|---------------|-------------|
| Windows | [Download](https://github.com/o0x1024/mytips/releases) | `.msi` / `.exe` |
| macOS | [Download](https://github.com/o0x1024/mytips/releases) | `.dmg` |
| Linux | [Download](https://github.com/o0x1024/mytips/releases) | `.AppImage` / `.deb` |

## Quick Start

### 1. Create Your First Note
```
Launch App → Click "New Note" → Start Writing
```

### 2. Configure AI Assistant
```
Settings → AI Assistant → Enter API Key → Select Model
```

### 3. Start AI Conversation
```
AI Assistant Page → Enter Question → Use # to Reference Notes
```

## Tips & Tricks

### Advanced AI Assistant
- **Note References** - Type `#` in conversations to quickly reference note content
- **Streaming Control** - View responses in real-time, stop generation anytime
- **Floating Mode** - Independent window for multitasking AI usage
- **Conversation Export** - Save important conversations for future reference

### Advanced Note Management
- **Encryption Protection** - Set password protection for sensitive content
- **Tagging System** - Build personal tag classification system
- **Quick Operations** - One-click code block copying, drag-and-drop image upload
- **Search Tips** - Full-text search across titles, content, and tags

## Tech Stack


| Layer | Technology |
|-------|------------|
| **Frontend Framework** | Vue 3 + TypeScript + Vite |
| **UI Components** | Tailwind CSS + DaisyUI |
| **State Management** | Pinia + Vue Router |
| **Markdown** | Marked + Prism.js + KaTeX |
| **Desktop Framework** | Tauri 2.0 + Rust |
| **Data Storage** | SQLite + Local File System |

## Development

### Requirements
- **Node.js** `16+` (recommended `18+`)
- **Rust** `1.70+` (stable toolchain)
- **System Dependencies**: Please refer to [Tauri Official Documentation](https://tauri.app/v1/guides/getting-started/prerequisites) for installation

### Installation & Setup

```bash
# 1. Clone the repository
git clone https://github.com/o0x1024/mytips.git
cd mytips

# 2. Install frontend dependencies (recommended using yarn)
yarn install

# 3. Start development mode
yarn tauri dev
```

## Contributing

We welcome community contributions! If you have any ideas or suggestions, please feel free to submit an [Issue](https://github.com/o0x1024/mytips/issues) or [Pull Request](https://github.com/o0x1024/mytips/pulls).

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Privacy & Security

- **Local First**: All data stored locally by default, never uploaded to any server
- **API Key Security**: Your API keys are encrypted and stored locally
- **Open Source Transparency**: Code is completely open source, subject to community review
- **Encryption Protection**: Support end-to-end encryption for notebooks and individual notes

## License

This project is open source under the [GPL-3.0](LICENSE) license.

## Acknowledgments

MyTips development is supported by the following excellent open source projects:

- [Tauri](https://tauri.app/)
- [Vue.js](https://vuejs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/)
- And all the dependency libraries listed in `package.json` and `Cargo.toml`

---

**MyTips - Making knowledge management simpler, smarter, and more secure.**

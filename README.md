# MyTips - 本地优先的笔记和代码片段管理工具

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="MyTips Logo" width="128" height="128">
</p>

MyTips 是一款基于 Tauri 框架开发的跨平台桌面应用程序，专为开发者和知识工作者设计。它提供了一个强大且灵活的本地优先的笔记和代码片段管理系统，支持 Markdown 编辑、语法高亮、AI 助手、剪贴板历史等功能。

## ✨ 主要特性

- 📝 **富文本编辑器** - 基于 Vditor 的强大 Markdown 编辑器，支持实时预览
- 🎨 **语法高亮** - 使用 highlight.js 提供多语言代码语法高亮
- 📂 **笔记本管理** - 灵活的层级笔记本组织结构
- 🏷️ **标签系统** - 强大的标签分类和筛选功能
- 🤖 **AI 助手** - 集成 AI 功能，提供智能写作辅助
- 📋 **剪贴板历史** - 自动记录和管理剪贴板内容
- 🌙 **主题切换** - 支持明暗主题自由切换
- 🔍 **全文搜索** - 快速搜索笔记内容和代码片段
- 💾 **本地存储** - 数据完全存储在本地，保护隐私
- ⚡ **快速启动** - 支持系统托盘和快捷键操作

## 🛠️ 技术栈

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Vue Router** - 官方路由管理器
- **Pinia** - 现代状态管理
- **Tailwind CSS** - 原子化 CSS 框架
- **DaisyUI** - 基于 Tailwind CSS 的组件库
- **Vditor** - 所见即所得的 Markdown 编辑器
- **VueUse** - Vue 组合式 API 工具集

### 后端 (Rust)
- **Tauri** - 现代桌面应用框架
- **SQLite** - 轻量级数据库 (rusqlite)
- **Serde** - 序列化/反序列化框架
- **Tokio** - 异步运行时
- **Chrono** - 日期时间处理
- **Reqwest** - HTTP 客户端

### 开发工具
- **Vite** - 快速构建工具
- **TypeScript** - 静态类型检查
- **Vue TSC** - Vue TypeScript 编译器

## 📦 项目结构

```
mytips/
├── src/                      # 前端源码
│   ├── components/           # Vue 组件
│   │   ├── MainLayout.vue    # 主布局组件
│   │   ├── NoteEditor.vue    # 笔记编辑器
│   │   ├── NoteList.vue      # 笔记列表
│   │   ├── SideNavBar.vue    # 侧边导航栏
│   │   └── ...
│   ├── views/                # 页面组件
│   │   ├── Home.vue          # 首页
│   │   ├── Editor.vue        # 编辑器页面
│   │   ├── Settings.vue      # 设置页面
│   │   ├── AIAssistant.vue   # AI 助手页面
│   │   └── ClipboardHistory.vue # 剪贴板历史页面
│   ├── stores/               # Pinia 状态管理
│   ├── assets/               # 静态资源
│   └── main.ts               # 应用入口
├── src-tauri/                # Tauri 后端源码
│   ├── src/                  # Rust 源码
│   ├── Cargo.toml            # Rust 依赖配置
│   └── tauri.conf.json       # Tauri 配置
├── public/                   # 公共资源
├── dist/                     # 构建输出
└── package.json              # 前端依赖配置
```

## 🚀 开发环境搭建

### 系统要求
- Node.js 16+ 
- Rust 1.70+
- 系统相关依赖（详见 [Tauri 文档](https://tauri.app/v1/guides/getting-started/prerequisites)）

### 安装依赖

```bash
# 安装前端依赖
npm install
# 或使用 yarn
yarn install
# 或使用 bun
bun install

# Tauri CLI 会自动处理 Rust 依赖
```

### 开发命令

```bash
# 启动开发服务器
npm run tauri dev
# 或
yarn tauri dev

# 构建生产版本
npm run tauri build
# 或
yarn tauri build

# 仅启动前端开发服务器
npm run dev

# 类型检查和构建前端
npm run build
```

## 📋 功能说明

### 笔记管理
- 支持 Markdown 格式的富文本编辑
- 层级笔记本结构，方便内容组织
- 实时保存，防止数据丢失
- 支持代码块语法高亮

### 标签系统
- 灵活的标签分类管理
- 支持标签筛选和搜索
- 可视化标签管理界面

### AI 助手
- 集成 AI 模型，提供智能写作建议
- 支持代码片段生成和优化
- 自定义 AI 提示词

### 剪贴板历史
- 自动记录剪贴板内容
- 支持快速搜索和使用历史记录
- 可配置记录规则和保留时间

### 设置和个性化
- 主题切换（明暗模式）
- 编辑器个性化配置
- 快捷键自定义
- 数据导入导出

## 🔧 配置说明

应用的主要配置文件：

- `src-tauri/tauri.conf.json` - Tauri 应用配置
- `package.json` - 前端依赖和脚本
- `src-tauri/Cargo.toml` - Rust 依赖配置
- `tailwind.config.js` - Tailwind CSS 配置
- `vite.config.ts` - Vite 构建配置

## 📝 许可证

本项目采用 GPL 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 联系方式

如有问题或建议，请通过以下方式联系：

- 提交 GitHub Issue
- 发送邮件至项目维护者

---

**MyTips** - 让知识管理更简单高效！

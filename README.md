# MyTips - 本地优先的笔记和代码片段管理工具

<p align="center">
  <img src="src-tauri/icons/128x128.png" alt="MyTips Logo" width="128" height="128">
</p>

MyTips 是一款基于 Tauri 框架开发的跨平台桌面应用程序，专为开发者和知识工作者设计。它提供了一个强大且灵活的本地优先的笔记和代码片段管理系统，支持 Markdown 编辑、语法高亮、AI 助手、剪贴板历史等功能。

## ✨ 主要特性

### 📝 笔记管理
- **语法高亮** - 使用 Prism.js 提供多语言代码语法高亮，支持超过100种编程语言
- **笔记本管理** - 灵活的层级笔记本组织结构，支持分类和标签系统
- **全文搜索** - 快速搜索笔记内容和代码片段，支持标题和内容搜索

### 🤖 AI 助手
- **多模型支持** - 集成多个主流AI模型：OpenAI ChatGPT、Google Gemini、DeepSeek、通义千问、Claude、豆包、Grok等
- **角色扮演** - 创建和管理自定义AI角色，支持不同专业领域的对话场景
- **流式输出** - 实时流式显示AI响应，提供更好的交互体验
- **多模态输入** - 支持文本、图片、文档等多种格式的文件上传
- **笔记引用** - 在对话中快速引用已有笔记内容，使用#符号快速选择
- **对话管理** - 保存和管理历史对话，支持对话标题编辑和导出

### 📋 剪贴板历史
- **自动记录** - 自动捕获和保存剪贴板内容历史
- **智能过滤** - 可配置的内容过滤规则，避免记录敏感信息
- **快速搜索** - 支持关键词搜索历史剪贴板内容
- **一键复用** - 快速复制历史内容到当前剪贴板

### 🎨 界面与体验
- **主题切换** - 支持明暗主题及多种预设主题自由切换
- **响应式设计** - 自适应不同屏幕尺寸，提供最佳使用体验
- **快捷键支持** - 丰富的快捷键操作，提高使用效率
- **系统集成** - 支持系统托盘和开机自启动

### 🔐 隐私与安全
- **本地存储** - 数据完全存储在本地SQLite数据库，保护隐私安全
- **API密钥管理** - 安全存储和管理各种AI服务的API密钥
- **代理支持** - 支持HTTP代理配置，适应不同网络环境

## 🛠️ 技术栈

### 前端技术
- **Vue 3** - 渐进式JavaScript框架，使用Composition API开发
- **TypeScript** - 类型安全的JavaScript超集，提供更好的开发体验
- **Vue Router** - 官方路由管理器，支持单页面应用导航
- **Pinia** - 现代化的Vue状态管理库
- **Tailwind CSS** - 原子化CSS框架，快速构建响应式界面
- **DaisyUI** - 基于Tailwind CSS的组件库，提供丰富的UI组件
- **Vditor** - 所见即所得的Markdown编辑器
- **VueUse** - Vue组合式API工具集
- **Prism.js** - 轻量级语法高亮库
- **Marked** - 高性能Markdown解析器
- **DOMPurify** - HTML清理库，防止XSS攻击

### 后端技术 (Rust)
- **Tauri** - 现代桌面应用框架，提供原生性能
- **SQLite** - 轻量级嵌入式数据库 (rusqlite)
- **Serde** - Rust序列化/反序列化框架
- **Tokio** - 异步运行时，支持高并发操作
- **Reqwest** - HTTP客户端库，用于AI API调用
- **Chrono** - 日期时间处理库
- **Tauri-plugin-clipboard** - 剪贴板操作插件
- **Tauri-plugin-autostart** - 开机自启动插件

### 开发工具
- **Vite** - 快速构建工具和开发服务器
- **Vue TSC** - Vue TypeScript编译器
- **ESLint** - JavaScript/TypeScript代码质量检查
- **Prettier** - 代码格式化工具

## 📦 项目结构

```
mytips/
├── src/                      # 前端源码
│   ├── components/           # Vue 组件
│   │   ├── MainLayout.vue    # 主布局组件
│   │   ├── NoteEditor.vue    # 笔记编辑器组件
│   │   ├── NoteList.vue      # 笔记列表组件
│   │   ├── SideNavBar.vue    # 侧边导航栏组件
│   │   └── ...               # 其他组件
│   ├── views/                # 页面组件
│   │   ├── Home.vue          # 首页 - 笔记管理
│   │   ├── Editor.vue        # 编辑器页面
│   │   ├── Settings.vue      # 设置页面
│   │   ├── AIAssistant.vue   # AI助手页面
│   │   └── ClipboardHistory.vue # 剪贴板历史页面
│   ├── stores/               # Pinia 状态管理
│   │   ├── tipsStore.ts      # 笔记数据状态管理
│   │   ├── uiStore.ts        # UI状态管理
│   │   └── ...               # 其他状态管理
│   ├── services/             # API服务层
│   │   ├── ai.ts             # AI相关API
│   │   ├── tips.ts           # 笔记相关API
│   │   └── ...               # 其他服务
│   ├── assets/               # 静态资源
│   └── main.ts               # 应用入口
├── src-tauri/                # Tauri 后端源码
│   ├── src/                  # Rust 源码
│   │   ├── main.rs           # 主程序入口
│   │   ├── commands/         # Tauri命令处理
│   │   ├── database/         # 数据库操作
│   │   ├── ai/               # AI功能模块
│   │   └── ...               # 其他模块
│   ├── Cargo.toml            # Rust 依赖配置
│   └── tauri.conf.json       # Tauri 配置
├── public/                   # 公共资源
├── dist/                     # 构建输出
└── package.json              # 前端依赖配置
```

## 🚀 开发环境搭建

### 系统要求
- **Node.js** 16+ (推荐使用18或20)
- **Rust** 1.70+ (需要stable工具链)
- **系统依赖** 根据平台安装对应依赖：
  - **Windows**: Microsoft Visual Studio C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: 开发工具包 (build-essential, webkit2gtk等)

详细系统依赖请参考 [Tauri 文档](https://tauri.app/v1/guides/getting-started/prerequisites)

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd mytips

# 安装前端依赖
npm install
# 或使用其他包管理器
yarn install
# pnpm install
# bun install

# Rust依赖会在首次运行时自动安装
```

### 开发命令

```bash
# 启动开发服务器 (热重载)
npm run tauri dev
# 或
yarn tauri dev

# 构建生产版本
npm run tauri build
# 或
yarn tauri build

# 仅启动前端开发服务器
npm run dev

# 类型检查
npm run type-check

# 代码格式化
npm run format

# 代码质量检查
npm run lint
```

## 📋 功能详细说明

### 笔记管理系统
- **创建和编辑**: 支持Markdown格式的富文本编辑，提供实时预览
- **分类管理**: 创建层级分类结构，便于组织和查找笔记
- **标签系统**: 为笔记添加多个标签，支持标签筛选和管理
- **搜索功能**: 全文搜索笔记标题和内容，快速定位所需信息
- **导入导出**: 支持Markdown文件的批量导入和导出

### AI助手功能
- **模型配置**: 支持配置多个AI模型的API密钥和参数
- **角色系统**: 创建专业领域的AI角色，如编程导师、文案写手等
- **对话管理**: 自动保存对话历史，支持对话标题自定义
- **多模态交互**: 上传图片、文档等文件与AI互动
- **流式响应**: 实时显示AI生成内容，可随时取消生成
- **内容引用**: 在对话中引用已有笔记，增强AI回答的准确性

### 剪贴板历史
- **自动监控**: 实时捕获剪贴板内容变化
- **内容过滤**: 配置过滤规则，忽略密码等敏感信息
- **历史管理**: 设置历史记录保留时间和数量限制
- **快速搜索**: 通过关键词快速查找历史内容

### 系统设置
- **外观设置**: 主题切换、字体大小调整
- **AI配置**: API密钥管理、模型参数设置
- **代理设置**: HTTP代理配置，支持企业网络环境
- **自启动**: 配置应用开机自动启动
- **数据管理**: 数据备份和恢复功能

## 🔧 配置文件说明

### 主要配置文件
- `src-tauri/tauri.conf.json` - Tauri应用主配置
- `package.json` - 前端依赖和脚本配置
- `src-tauri/Cargo.toml` - Rust依赖配置
- `tailwind.config.js` - Tailwind CSS样式配置
- `vite.config.ts` - Vite构建工具配置
- `tsconfig.json` - TypeScript编译配置

### 运行时数据存储
- 数据库文件: `%APPDATA%/com.mytips.app/` (Windows) 或 `~/Library/Application Support/com.mytips.app/` (macOS)
- 配置文件: 存储在系统标准配置目录
- 日志文件: 存储在系统日志目录

## 🎯 使用说明

### 快速开始
1. **首次使用**: 启动应用后会自动创建数据库和基础配置
2. **创建笔记**: 点击"新建笔记"开始记录内容
3. **配置AI**: 前往设置页面配置AI模型API密钥
4. **使用AI助手**: 在AI助手页面与人工智能对话

### 高级功能
- **笔记引用**: 在AI对话中使用`#笔记标题`快速引用已有内容
- **角色扮演**: 创建专业角色，获得更专业的AI回答
- **批量操作**: 支持笔记的批量导入、导出和管理
- **主题定制**: 选择合适的主题和字体，个性化使用体验

## 🔐 隐私和安全

- **本地优先**: 所有数据存储在本地，不上传到任何服务器
- **API安全**: API密钥加密存储，仅在需要时解密使用
- **内容过滤**: 剪贴板监控可配置过滤规则，保护敏感信息
- **开源透明**: 完全开源，代码公开透明，可自行审查

## 📝 许可证

本项目采用 GPL-3.0 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 贡献方式
1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 开发规范
- 遵循项目的代码风格和TypeScript类型规范
- 确保所有测试通过
- 更新相关文档
- 提交信息使用清晰的描述

## 📞 联系方式

如有问题或建议，可通过以下方式联系：

- 🐛 **Bug报告**: [GitHub Issues](https://github.com/your-username/mytips/issues)
- 💡 **功能建议**: [GitHub Discussions](https://github.com/your-username/mytips/discussions)
- 📧 **邮件联系**: 发送邮件至项目维护者

## 🙏 致谢

感谢以下开源项目的支持：
- [Tauri](https://tauri.app/) - 跨平台应用框架
- [Vue.js](https://vuejs.org/) - 渐进式JavaScript框架
- [Vditor](https://github.com/Vanessa219/vditor) - Markdown编辑器
- [Tailwind CSS](https://tailwindcss.com/) - CSS框架
- [DaisyUI](https://daisyui.com/) - UI组件库

---

**MyTips** - 让知识管理更简单高效！ 🚀

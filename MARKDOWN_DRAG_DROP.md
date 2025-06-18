# Markdown 拖拽导入功能

## 功能概述

MyTips 现在支持将外部 Markdown 文件直接拖拽到应用窗口中进行预览和导入。

## 使用方法

### 1. 拖拽文件
- 从文件管理器中选择一个或多个 Markdown 文件
- 将文件拖拽到 MyTips 应用窗口中
- 当检测到 Markdown 文件时，会显示拖拽覆盖层

### 2. 预览内容
- 松开鼠标后，会自动打开预览模态框
- 预览框显示 Markdown 文件的渲染效果
- 可以查看文件名和完整内容

### 3. 导入选项
- 点击"导入为笔记"按钮
- 设置笔记标题（默认为文件名）
- 选择要导入到的笔记本
- 点击"确认导入"完成导入

## 支持的文件格式

支持以下 Markdown 文件扩展名：
- .md
- .markdown
- .mdown
- .mkdn
- .mkd
- .mdwn
- .mdtxt
- .mdtext
- .text

## 功能特点

### 实时预览
- 使用 marked.js 进行 Markdown 渲染
- 使用 DOMPurify 进行内容安全处理
- 支持完整的 Markdown 语法（表格、代码块、链接等）

### 智能检测
- 自动检测拖拽的文件类型
- 只对 Markdown 文件显示导入界面
- 支持多文件拖拽（选择第一个 Markdown 文件）

### 灵活导入
- 可自定义笔记标题
- 可选择目标笔记本
- 保留原始 Markdown 格式

## 技术实现

### 组件结构
`MarkdownDropPreview.vue` - 主要组件
- 集成到 `MainLayout.vue` 中
- 使用全局拖拽事件监听

### 依赖库
- `marked` - Markdown 解析和渲染
- `dompurify` - HTML 内容安全处理
- Vue 3 组合式 API

### 事件处理
- `dragenter` - 检测拖拽进入
- `dragover` - 维持拖拽状态
- `dragleave` - 检测拖拽离开
- `drop` - 处理文件放置

## 注意事项

1. **文件大小**: 建议单个文件不超过 10MB
2. **文件编码**: 支持 UTF-8 编码的文本文件
3. **安全性**: 所有 HTML 内容都经过 DOMPurify 处理
4. **兼容性**: 需要现代浏览器支持拖拽 API

## 使用示例

1. 创建一个测试 Markdown 文件：
```markdown
# 我的笔记

这是一个测试笔记。

## 代码示例
\`\`\`javascript
console.log('Hello, World!');
\`\`\`
```

2. 将文件拖拽到 MyTips 窗口
3. 在预览界面查看渲染效果
4. 设置标题和笔记本后导入

## 故障排除

### 拖拽无响应
- 确保文件是支持的 Markdown 格式
- 检查文件是否可读
- 尝试重新拖拽

### 预览显示异常
- 检查 Markdown 语法是否正确
- 确认文件编码为 UTF-8
- 查看浏览器控制台错误信息

### 导入失败
- 确保已选择目标笔记本
- 检查笔记标题是否为空
- 确认网络连接正常 
# 文件拖拽预览功能完善

## 改进概述

根据Tauri WebView文档，我们完全重写了文件拖拽预览功能，使用Tauri的原生`onDragDropEvent` API替换了HTML5拖拽事件，提供了更可靠和高性能的文件拖拽体验。

## 主要改进

### 1. 使用Tauri原生拖拽API

**之前**：使用HTML5的`dragenter`、`dragover`、`dragleave`、`drop`事件
```javascript
// 旧方式 - HTML5 拖拽事件
window.addEventListener('dragenter', handleDragEnter, true)
window.addEventListener('dragover', handleDragOver, true)
window.addEventListener('dragleave', handleDragLeave, true)
window.addEventListener('drop', handleDrop, true)
```

**现在**：使用Tauri的`onDragDropEvent` API
```javascript
// 新方式 - Tauri原生拖拽API
const webview = getCurrentWebview()
unlistenDragDrop = await webview.onDragDropEvent(handleTauriDragDrop)
```

### 2. 改进的事件处理

#### 支持的拖拽事件类型：
- **`over`**：文件悬停在窗口上时触发
- **`drop`**：文件放置时触发
- **`cancelled`**：拖拽取消时触发

#### 实时位置跟踪：
```javascript
if (event.payload.type === 'over') {
  isDragOver.value = true
  dragPosition.value = { x: event.payload.position.x, y: event.payload.position.y }
}
```

### 3. 使用Tauri文件系统API

**文件读取**：
```javascript
// 使用Tauri的readTextFile替代FileReader
const content = await readTextFile(filePath)
```

**文件存在检查**：
```javascript
// 检查文件是否存在
const fileExists = await exists(path)
```

### 4. 增强的文件类型检测

支持更多Markdown文件扩展名：
- `.md`, `.markdown`, `.mdown`, `.mkdn`, `.mkd`
- `.mdwn`, `.mdtxt`, `.mdtext`, `.text`

```javascript
async function isMarkdownFile(path: string): Promise<boolean> {
  const fileName = path.split('/').pop()?.toLowerCase() || ''
  const markdownExtensions = [
    '.md', '.markdown', '.mdown', '.mkdn', '.mkd', 
    '.mdwn', '.mdtxt', '.mdtext', '.text'
  ]
  
  const isMarkdown = markdownExtensions.some(ext => fileName.endsWith(ext))
  
  if (isMarkdown) {
    return await exists(path) // 确保文件真实存在
  }
  
  return false
}
```

### 5. 改进的用户界面

#### 拖拽覆盖层增强：
- 显示实时拖拽位置坐标
- 更清晰的视觉反馈
- 支持的文件格式提示

#### 错误处理改进：
- 详细的错误信息显示
- 文件不存在检测
- 读取失败的友好提示

## 技术优势

### 1. 性能提升
- **原生API**：直接使用Tauri的原生拖拽事件，避免了HTML5事件的兼容性问题
- **更少的事件监听器**：只需要一个监听器而不是四个
- **更高效的文件读取**：使用Rust后端的文件系统API

### 2. 可靠性提升
- **事件冲突解决**：避免了与其他组件的拖拽事件冲突
- **文件路径处理**：直接获取文件的绝对路径，避免了File对象的限制
- **错误处理**：完善的try-catch错误处理机制

### 3. 功能增强
- **实时位置跟踪**：显示拖拽的实时坐标
- **文件存在验证**：确保文件在预览前真实存在
- **更好的调试**：详细的控制台日志和调试信息

## 使用方法

### 1. 基本拖拽操作
1. 从文件管理器选择Markdown文件
2. 拖拽到MyTips应用窗口
3. 观察拖拽覆盖层和位置信息
4. 松开鼠标查看预览

### 2. 调试功能
在浏览器控制台中运行：
```javascript
// 手动测试覆盖层显示/隐藏
testDragFunction()
```

### 3. 监控拖拽事件
控制台会显示详细的拖拽事件日志：
```
Tauri拖拽事件: {type: "over", position: {x: 100, y: 200}}
文件悬停在位置: {x: 100, y: 200}
Tauri拖拽事件: {type: "drop", paths: ["/path/to/file.md"]}
文件放置，路径: ["/path/to/file.md"]
```

## 兼容性说明

### Tauri要求
- 需要Tauri 2.0+
- 需要启用`dragDropEnabled: true`配置
- 需要`@tauri-apps/api/webview`和`@tauri-apps/plugin-fs`

### 配置检查
确保`src-tauri/tauri.conf.json`中包含：
```json
{
  "app": {
    "windows": [
      {
        "dragDropEnabled": true
      }
    ]
  }
}
```

## 故障排除

### 1. 拖拽无响应
- 检查控制台是否有Tauri API错误
- 确认`dragDropEnabled`配置已启用
- 验证文件格式是否支持

### 2. 文件读取失败
- 检查文件路径是否正确
- 确认文件权限
- 验证文件编码（推荐UTF-8）

### 3. 预览显示异常
- 检查Markdown语法
- 查看Prism语法高亮错误
- 确认DOMPurify配置

## 开发者注意事项

### 1. 类型安全
所有拖拽事件处理都使用TypeScript类型检查：
```typescript
async function handleTauriDragDrop(event: any) {
  // event.payload 包含类型化的拖拽数据
}
```

### 2. 内存管理
确保在组件卸载时清理事件监听器：
```javascript
onUnmounted(() => {
  if (unlistenDragDrop) {
    unlistenDragDrop()
    unlistenDragDrop = null
  }
})
```

### 3. 错误处理
所有异步操作都包含适当的错误处理：
```javascript
try {
  const content = await readTextFile(filePath)
  // 处理成功情况
} catch (error) {
  console.error('读取文件失败:', error)
  // 处理错误情况
}
```

## 测试建议

1. **功能测试**：
   - 拖拽不同格式的Markdown文件
   - 测试大文件和小文件
   - 验证多文件拖拽行为

2. **边界测试**：
   - 拖拽非Markdown文件
   - 拖拽不存在的文件
   - 拖拽空文件

3. **UI测试**：
   - 验证覆盖层显示
   - 检查位置坐标更新
   - 测试预览模态框

通过这些改进，文件拖拽预览功能现在更加可靠、高效，并且完全符合Tauri应用的最佳实践。 
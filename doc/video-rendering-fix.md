# 视频渲染问题修复

## 问题描述

用户报告 markdown 预览区显示不支持视频播放。根据截图显示：

```
🎬 视频文件: 027-agent-loop-animation.mp4 您的浏览器不支持视频播放。

<video controls style="max-width: 100%; height: auto;">
  <source src="local://media_1754383646746_773" type="video/mp4">
  您的浏览器不支持视频播放。
</video>
```

## 问题分析

1. 视频 HTML 标签生成正确
2. `local://` 协议引用正确
3. 问题可能在于：
   - `local://` 协议没有被正确替换为 base64 数据
   - images 对象中缺少对应的媒体文件数据
   - 渲染过程中数据丢失

## 当前实现

### NoteEditor.vue 中的视频处理

```typescript
// 生成视频 Markdown
else if (file.type.startsWith('video/')) {
  mediaMarkdown = `\n\n🎬 **视频文件: ${file.name || '视频'}**\n<video controls style="max-width: 100%; height: auto;">\n  <source src="local://${mediaId}" type="${file.type}">\n  您的浏览器不支持视频播放。\n</video>\n\n`
}

// 保存到本地状态
localNote.value.images[mediaId] = base64Data
```

### markdownService.ts 中的处理

```typescript
// 处理视频源
if (node.tagName === 'source' && typeof node.properties?.src === 'string' && node.properties.src.startsWith('local://')) {
  const mediaId = node.properties.src.replace('local://', '')
  if (images[mediaId]) {
    node.properties.src = images[mediaId]
  } else {
    console.warn(`Media file with id "${mediaId}" not found in provided images map.`)
    node.properties.src = ''
  }
}
```

## 调试信息

已添加调试日志：
- NoteEditor.vue render 函数中记录可用的 images keys
- markdownService.ts 中记录视频处理过程

## 下一步

1. 在 Tauri 应用中测试视频上传和渲染
2. 检查控制台输出的调试信息
3. 验证 images 对象是否包含正确的数据
4. 确认 base64 数据格式是否正确

## 可能的修复方案

如果发现问题，可能需要：
1. 修复 images 对象的数据传递
2. 确保视频文件正确保存到数据库
3. 验证从数据库加载的视频数据格式
4. 检查 rehype 插件的执行顺序
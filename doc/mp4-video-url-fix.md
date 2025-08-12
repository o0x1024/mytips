# MP4 视频 URL 格式错误修复报告

## 问题描述
用户报告在 Markdown 预览区点击视频播放按钮无法播放已上传的 MP4 视频，控制台报错：
```
[Error] Failed to load resource: unsupported URL (audio_1b3b465ebd204124a40b9f7324bb6cf8, line 0)
```

## 根本原因分析

### 问题根源
错误信息显示视频标签的 `src` 属性直接是 `audio_1b3b465ebd204124a40b9f7324bb6cf8`，缺少协议前缀（如 `local://` 或 `audio://`）。

### 技术分析
1. **正常情况**：视频标签应该是 `<source src="local://audio_1b3b465ebd204124a40b9f7324bb6cf8" type="video/mp4">`
2. **异常情况**：视频标签变成了 `<source src="audio_1b3b465ebd204124a40b9f7324bb6cf8" type="video/mp4">`
3. **处理逻辑**：`rehypeLocalAudio` 插件只处理带有 `audio://` 或 `local://` 协议前缀的 URL

## 修复方案

### 修复文件
**文件**: `src/services/markdownService.ts`

### 修复内容
修改 `rehypeLocalAudio` 函数，增加对没有协议前缀的 `audio_id` 格式的处理：

**修改前**:
```javascript
visit(tree, 'element', (node: Element) => {
  if (
    node.tagName === 'source' &&
    typeof node.properties?.src === 'string' &&
    (node.properties.src.startsWith('audio://') || node.properties.src.startsWith('local://'))
  ) {
    const audioId = node.properties.src.replace(/^(audio:\/\/|local:\/\/)/, '')
    nodesToProcess.push({ node, audioId })
  }
})
```

**修改后**:
```javascript
visit(tree, 'element', (node: Element) => {
  if (
    node.tagName === 'source' &&
    typeof node.properties?.src === 'string'
  ) {
    let audioId = ''
    
    // 处理带协议前缀的URL
    if (node.properties.src.startsWith('audio://') || node.properties.src.startsWith('local://')) {
      audioId = node.properties.src.replace(/^(audio:\/\/|local:\/\/)/, '')
    }
    // 处理直接的audio_id格式（没有协议前缀）
    else if (node.properties.src.match(/^audio_[a-f0-9]+$/)) {
      audioId = node.properties.src
    }
    
    if (audioId) {
      nodesToProcess.push({ node, audioId })
    }
  }
})
```

### 修复逻辑
1. **扩展检测范围**：不再仅限于带协议前缀的 URL
2. **增加格式匹配**：使用正则表达式 `/^audio_[a-f0-9]+$/` 匹配直接的 audio_id 格式
3. **统一处理**：无论是否有协议前缀，都能正确提取 audio_id 并处理

## 修复效果

### 预期结果
- ✅ 修复 "Failed to load resource: unsupported URL" 错误
- ✅ 支持处理没有协议前缀的 audio_id 格式
- ✅ 保持对现有格式的兼容性
- ✅ MP4 视频能够正常播放

### 兼容性
- ✅ 向后兼容：仍然支持 `local://audio_id` 和 `audio://audio_id` 格式
- ✅ 向前兼容：新增支持直接的 `audio_id` 格式
- ✅ 错误恢复：对于无效格式会优雅降级

## 测试建议

1. **测试现有视频**：检查之前无法播放的 MP4 视频是否现在可以正常播放
2. **测试新上传视频**：上传新的 MP4 文件并验证播放功能
3. **测试不同格式**：验证其他视频格式（webm、avi 等）是否仍然正常工作
4. **测试音频文件**：确保音频文件播放功能没有受到影响

## 修复时间
- **修复日期**: 2025-01-06
- **修复状态**: ✅ 已完成
- **测试状态**: ⏳ 待用户测试
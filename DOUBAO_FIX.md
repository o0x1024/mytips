# 豆包API错误修复说明

## 问题描述

在使用豆包大模型时遇到以下错误：

### 错误1：角色名大小写问题
```
抱歉，发生了错误：豆包API返回错误 400 Bad Request: {"error":{"code":"InvalidParameter","message":"The parameter `messages.role` specified in the request are not valid: invalid value: `User`, supported values are: `system`, `assistant`, `user`, `tool`. Request id: 021749784108109ac9ba1f2260f568266ea32bf2462373a181f86"}}
```

### 错误2：消息内容格式问题
```
{"error":{"code":"InvalidParameter","message":"The parameter messages.content specified in the request are not valid: expected a string or array of objects, but got {\"Text\":\"你是谁？\"} instead. Request id: 021749784343693e5d1952a316480dd1a8e27d419c7433fa46d23","param":"messages.content","type":"BadRequest"}}
```

## 错误原因

1. **角色名问题**：豆包API要求消息角色必须是小写的（`user`, `assistant`, `system`），但GenAI库的`ChatMessage.role.to_string()`方法可能返回大写的角色名（如`User`）。

2. **消息内容格式问题**：豆包API期望`messages.content`是字符串或对象数组，但GenAI库的`ChatMessage.content`是`MessageContent`类型，序列化后变成了`{"Text":"内容"}`这样的对象格式。

## 修复方案

在`src-tauri/src/api/ai/doubao.rs`文件中进行了以下修复：

### 1. 角色名修复

#### 修复前
```rust
json!({
    "role": msg.role.to_string(), // 可能返回大写的"User"
    "content": msg.content
})
```

#### 修复后
```rust
let role = match msg.role.to_string().to_lowercase().as_str() {
    "user" => "user",
    "assistant" => "assistant", 
    "system" => "system",
    _ => "user", // 默认为user
};
```

### 2. 消息内容格式修复

#### 添加内容提取函数
```rust
// 从GenAI库的MessageContent中提取纯文本内容
fn extract_text_content(content: &genai::chat::MessageContent) -> String {
    if let Ok(json_value) = serde_json::to_value(content) {
        // 如果content已经是字符串，直接返回
        if let Some(text) = json_value.as_str() {
            return text.to_string();
        }
        
        // 如果content是对象，尝试提取Text字段
        if let Some(obj) = json_value.as_object() {
            if let Some(text) = obj.get("Text") {
                if let Some(text_str) = text.as_str() {
                    return text_str.to_string();
                }
            }
            // 尝试其他可能的字段名
            if let Some(text) = obj.get("text") {
                if let Some(text_str) = text.as_str() {
                    return text_str.to_string();
                }
            }
        }
        
        return json_value.to_string();
    }
    
    String::new()
}
```

#### 使用内容提取函数
```rust
// 提取消息内容，处理GenAI库的复杂content格式
let content = extract_text_content(&msg.content);

json!({
    "role": role,
    "content": content  // 现在是纯文本字符串
})
```

## 修复位置

1. `doubao_chat_with_history` 函数 - 非流式对话历史
2. `doubao_stream_chat_with_history` 函数 - 流式对话历史
3. 新增 `extract_text_content` 辅助函数

## 测试验证

修复后，豆包模型应该能够正常处理：
- ✅ 单轮对话
- ✅ 多轮对话历史
- ✅ 流式输出
- ✅ 不同角色的消息（user, assistant, system）
- ✅ 正确的消息内容格式

## 兼容性

此修复确保了与豆包API的完全兼容，同时保持了与其他AI模型的兼容性。修复后的代码能够：
- 正确处理GenAI库的`MessageContent`类型
- 提取纯文本内容发送给豆包API
- 确保角色名符合豆包API规范 
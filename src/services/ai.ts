import { invoke } from '@tauri-apps/api/core'

// 对话相关API
export async function sendAIMessage(_conversationId: string, message: string, model: string) {
  return await invoke<string>('send_ai_message', { 
    modelId: model,
    message
  })
}

export async function listAIConversations() {
  return await invoke('list_ai_conversations')
}

export async function getAIConversationMessages(conversationId: string) {
  return await invoke('list_ai_messages', { conversation_id: conversationId })
}

export async function deleteAIConversation(conversationId: string) {
  return await invoke('delete_ai_conversation', { conversation_id: conversationId })
}

export async function createAIConversation(model: string, title?: string) {
  return await invoke('create_ai_conversation', { model, title })
}

export async function clearAIConversation(conversationId: string) {
  return await invoke('clear_ai_conversation', { conversation_id: conversationId })
}

export async function addAIMessage(conversationId: string, role: string, content: string) {
  return await invoke('add_ai_message', { 
    conversation_id: conversationId, 
    role, 
    content 
  })
}

export async function exportAIConversations() {
  return await invoke('export_ai_conversations')
}

export async function updateAIConversationTitle(conversationId: string, newTitle: string) {
  return await invoke('update_ai_conversation_title', { 
    conversation_id: conversationId, 
    new_title: newTitle 
  })
}

// 角色相关API
export async function listAIRoles() {
  return await invoke('list_ai_roles')
}

export async function createAIRole(name: string, description: string) {
  return await invoke('create_ai_role', { name, description })
}

export async function updateAIRole(roleId: string, name: string, description: string) {
  return await invoke('update_ai_role', { 
    role_id: roleId, 
    name, 
    description 
  })
}

export async function deleteAIRole(roleId: string) {
  return await invoke('delete_ai_role', { role_id: roleId })
}

// 默认AI模型相关API
export async function saveDefaultAIModel(modelId: string) {
  return await invoke('save_default_ai_model', { modelId })
}

export async function getDefaultAIModel() {
  return await invoke('get_default_ai_model')
}

// 添加别名函数以兼容现有代码
export async function listAIMessages(conversationId: string) {
  return await getAIConversationMessages(conversationId)
} 

// 总结剪贴板内容
export async function summarizeClipboardEntries(days: number, prompt: string) {
  return await invoke<string>('summarize_clipboard_entries', { days, prompt });
} 
import { invokeSafe as invoke } from './api'

// AI服务提供商信息
export interface AIProvider {
  id: string
  provider: string
  name: string
  api_key?: string
  api_base?: string
  organization?: string
  enabled: boolean
  default_model: string
  models: any[]
}

// AI模型信息
export interface AIModel {
  name: string
  provider: string
  is_chat: boolean
  is_embedding: boolean
}

// 测试连接请求
export interface TestConnectionRequest {
  provider: string
  api_key?: string
  api_base?: string
  organization?: string
  model?: string
}

// 测试连接响应
export interface TestConnectionResponse {
  success: boolean
  message: string
  models?: string[]
}

// AI服务状态
export interface AIServiceStatus {
  provider: string
  is_available: boolean
  models_count: number
  active_conversations: number
}

// AI使用统计
export interface AIUsageStats {
  conversations: number
  messages: number
  tokens: {
    input: number
    output: number
    total: number
  }
  providers: Record<string, any>
}

// 获取AI配置
export async function getAIConfig(): Promise<{ providers: Record<string, AIProvider> } | null> {
  return invoke('get_ai_config')
}

// 获取聊天模型列表
export async function getChatModels(): Promise<AIModel[]> {
  return invoke('get_ai_chat_models') as Promise<AIModel[]>
}

// 获取嵌入模型列表
export async function getEmbeddingModels(): Promise<AIModel[]> {
  return invoke('get_ai_embedding_models') as Promise<AIModel[]>
}

// 获取默认AI模型
export async function getDefaultAIModel(modelType: string): Promise<AIModel | null> {
  return invoke('get_default_ai_model', { modelType }) as Promise<AIModel | null>
}

// 设置默认AI模型
export async function setDefaultAIModel(modelType: string, provider: string, modelName: string): Promise<void> {
  return invoke('set_default_ai_model', { modelType, provider, modelName }) as Promise<void>
}

// 测试AI连接
export async function testAIConnection(request: TestConnectionRequest): Promise<TestConnectionResponse> {
  return invoke('test_ai_connection', { request }) as Promise<TestConnectionResponse>
}

// 保存AI配置
export async function saveAIConfig(providers: Record<string, AIProvider>): Promise<void> {
  return invoke('save_ai_config', { config: { providers } }) as Promise<void>
}

// 获取AI使用统计
export async function getAIUsageStats(): Promise<AIUsageStats> {
  return invoke('get_ai_usage_stats') as Promise<AIUsageStats>
}

// 重新加载AI服务
export async function reloadAIServices(): Promise<void> {
  return invoke('reload_ai_services') as Promise<void>
}

// 获取AI服务状态
export async function getAIServiceStatus(): Promise<AIServiceStatus[]> {
  return invoke('get_ai_service_status') as Promise<AIServiceStatus[]>
}

// 自定义模型配置
export interface CustomModelConfig {
  id: string
  name: string
  endpoint: string
  model_name: string
  adapter_type: string // openai, ollama, etc.
  api_key?: string
}

// 获取自定义模型配置列表
export async function listCustomModelConfigs(): Promise<CustomModelConfig[]> {
  return invoke('list_custom_model_configs') as Promise<CustomModelConfig[]>
}

// 添加自定义模型配置
export async function addCustomModelConfig(config: CustomModelConfig): Promise<void> {
  return invoke('add_custom_model_config', { config }) as Promise<void>
}

// 更新自定义模型配置
export async function updateCustomModelConfig(config: CustomModelConfig): Promise<void> {
  return invoke('update_custom_model_config', { config }) as Promise<void>
}

// 删除自定义模型配置
export async function deleteCustomModelConfig(modelId: string): Promise<void> {
  return invoke('delete_custom_model_config', { modelId }) as Promise<void>
}

// 默认的AI提供商列表
export const defaultProviders: Record<string, AIProvider> = {
  openai: {
    id: 'openai',
    provider: 'openai',
    name: 'OpenAI',
    enabled: true,
    default_model: 'gpt-3.5-turbo',
    models: []
  },
  anthropic: {
    id: 'anthropic',
    provider: 'anthropic',
    name: 'Anthropic',
    enabled: true,
    default_model: 'claude-3.5-sonnet',
    models: []
  },
  gemini: {
    id: 'gemini',
    provider: 'gemini',
    name: 'Gemini',
    enabled: true,
    default_model: 'gemini-2.0-flash',
    models: []
  },
  deepseek: {
    id: 'deepseek',
    provider: 'deepseek',
    name: 'DeepSeek',
    enabled: true,
    default_model: 'deepseek-chat',
    models: []
  },
  qwen: {
    id: 'qwen',
    provider: 'qwen',
    name: 'Qwen',
    enabled: true,
    default_model: 'qwen3-coder-plus',
    models: []
  },
  doubao: {
    id: 'doubao',
    provider: 'doubao',
    name: 'Doubao',
    enabled: true,
    default_model: 'doubao-seed-1.6',
    models: []
  },
  xai: {
    id: 'xai',
    provider: 'xai',
    name: 'Grok',
    enabled: true,
    default_model: 'grok-3',
    models: []
  },
  custom: {
    id: 'custom',
    provider: 'custom',
    name: 'Custom',
    enabled: true,
    default_model: '',
    models: []
  }
} 
<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">AI助手</h2>
      
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">全局AI默认模型</span>
        </label>
        <select v-model="defaultAIModel" class="select select-bordered w-full" @change="saveDefaultAIModel">
          <option value="openai">OpenAI ChatGPT</option>
          <option value="gemini">Gemini</option>
          <option value="anthropic">Anthropic Claude</option>
          <option value="deepseek">DeepSeek</option>
          <option value="ali">通义千问</option>
          <option value="doubao">字节豆包</option>
          <option value="grok">xAI Grok</option>
          <option value="custom">自定义模型</option>
        </select>
      </div>
      
      <div class="tabs tabs-boxed mb-4 flex-wrap gap-2">
        <a 
          v-for="(provider, id) in aiProviders" 
          :key="id"
          class="tab" 
          :class="{ 'tab-active': selectedConfigModel === id }"
          @click="selectedConfigModel = id"
        >
          {{ provider.name }}
        </a>
      </div>
      
      <div v-if="selectedConfigModel" class="border rounded-lg p-4 mb-4">
        <h3 class="font-bold mb-2">{{ aiProviders[selectedConfigModel]?.name }} 配置</h3>
        
        <div class="form-control mb-2">
          <label class="label">
            <span class="label-text">API密钥</span>
          </label>
          <input 
            type="password" 
            v-model="aiProviders[selectedConfigModel].api_key" 
            placeholder="输入API密钥" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai' || selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">API端点</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].api_base" 
            placeholder="API端点URL" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai'">
          <label class="label">
            <span class="label-text">组织ID (可选)</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].organization" 
            placeholder="组织ID" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">模型名称</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].default_model" 
            placeholder="模型名称" 
            class="input input-bordered w-full"
          />
        </div>
        
        <!-- 输入框 + datalist，同时支持手动输入和下拉选择 -->
        <div class="form-control mb-4" v-else>
          <label class="label">
            <span class="label-text">默认模型</span>
          </label>
          <input
            type="text"
            v-model="aiProviders[selectedConfigModel].default_model"
            :list="`models-${selectedConfigModel}`"
            placeholder="输入模型名称或选择"
            class="input input-bordered w-full"
          />
          <datalist :id="`models-${selectedConfigModel}`">
            <option
              v-for="model in aiProviders[selectedConfigModel].models"
              :key="model.name"
              :value="model.name"
            />
          </datalist>
        </div>
        
        <div class="flex justify-between">
          <button 
            class="btn btn-primary" 
            :disabled="isTestingApi" 
            @click="() => testApiConnection(selectedConfigModel)"
          >
            <span v-if="isTestingApi">测试中...</span>
            <span v-else>测试连接</span>
          </button>
          
          <button 
            class="btn btn-accent" 
            :disabled="isSavingApiConfig" 
            @click="saveAIProviderConfig"
          >
            <span v-if="isSavingApiConfig">保存中...</span>
            <span v-else>保存配置</span>
          </button>
        </div>
      </div>
      
      <div class="divider">使用统计</div>
      
      <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
        <div class="stat">
          <div class="stat-title">对话总数</div>
          <div class="stat-value">{{ aiStats.conversations }}</div>
        </div>
        
        <div class="stat">
          <div class="stat-title">消息总数</div>
          <div class="stat-value">{{ aiStats.messages }}</div>
        </div>
        
        <div class="stat">
          <div class="stat-title">Token总数</div>
          <div class="stat-value">{{ aiStats.tokens.total }}</div>
          <div class="stat-desc">输入: {{ aiStats.tokens.input }} / 输出: {{ aiStats.tokens.output }}</div>
        </div>
      </div>
      
      <button class="btn btn-sm btn-ghost mt-2" @click="refreshAIStats">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        刷新统计
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { showMessage } from '../../services/dialog'
import {
  AIProvider, TestConnectionRequest,
  getChatModels, getDefaultAIModel,
  testAIConnection, saveAIConfig, getAIConfig, getAIUsageStats, reloadAIServices,
  getAIServiceStatus, defaultProviders
} from '../../services/aiService'

// AI模型和提供商映射关系
const providerMapping: Record<string, string> = {
  'chatgpt': 'openai',
  'gemini': 'gemini',
  'deepseek': 'deepseek',
  'qwen': 'ali',
  'claude': 'anthropic',
  'doubao': 'doubao',
  'grok': 'xai',
  'custom': 'custom'
}

const defaultAIModel = ref('chatgpt')
const selectedConfigModel = ref('openai')
const aiProviders = ref<Record<string, AIProvider>>(structuredClone(defaultProviders))
const isTestingApi = ref(false)
const isSavingApiConfig = ref(false)
const aiStats = ref({
  conversations: 0,
  messages: 0,
  tokens: {
    input: 0,
    output: 0,
    total: 0
  },
  providers: {}
})

// 保存默认AI模型
async function saveDefaultAIModel() {
  try {
    const providerId = defaultAIModel.value
    const providerConfig = aiProviders.value[providerId]
    
    if (!providerConfig) {
      console.error(`Provider config not found for: ${providerId}`)
      showMessage(`未找到提供商配置: ${providerId}`, { title: '错误' })
      return
    }

    const providerForBackend = providerMapping[providerId]
    const modelName = providerConfig.default_model
    
    if (!providerForBackend || !modelName) {
        console.error('Provider or model name is missing for the selected default AI model.')
        showMessage('提供商或模型名称丢失', { title: '错误' })
        return
    }
    
    // 调用后端将全局默认模型存入数据库
    await invoke('set_default_ai_model', {
      modelType: 'chat',
      provider: providerForBackend,
      modelName: modelName
    })
    
    // 同时保存到localStorage，用于快速加载
    localStorage.setItem('defaultAIModel', providerId)
    
    // 发出全局设置变更通知
    await emit('global-settings-changed', { key: 'defaultAIModel' })
    
    showMessage('默认AI模型已保存', { title: '成功' })

  } catch (error) {
    console.error('保存默认AI模型失败:', error)
    showMessage('保存默认AI模型失败: ' + error, { title: '错误' })
  }
}

// 加载默认AI模型
async function loadDefaultAIModel() {
  try {
    // 先尝试从新API获取默认模型
    try {
      const defaultModel = await invoke('get_default_ai_model', { modelType: 'chat' }) as any
      if (defaultModel && defaultModel.provider && defaultModel.name) {
        // 找到对应的本地模型ID
        for (const [id, provider] of Object.entries(providerMapping)) {
          if (provider === defaultModel.provider) {
            defaultAIModel.value = id
            break
          }
        }
        console.log('从API加载了默认AI模型:', defaultAIModel.value)
        return
      }
    } catch (error) {
      console.warn('从API获取默认AI模型失败，尝试使用localStorage:', error)
    }

    // 回退到localStorage
    const model = localStorage.getItem('defaultAIModel')
    if (model) {
      defaultAIModel.value = model
    }
  } catch (error) {
    console.error('获取默认AI模型失败:', error)
  }
}

// 测试API连接
async function testApiConnection(providerId: string): Promise<void> {
  if (!aiProviders.value[providerId]) return
  
  const provider = aiProviders.value[providerId]
  
  // 对于非自定义模型，至少需要API Key
  if (providerId !== 'custom' && !provider.api_key) {
    showMessage('请输入API密钥后再测试', { title: '提示' })
    return
  }
  
  // 对于自定义模型，至少需要Endpoint和Model Name
  if (providerId === 'custom' && (!provider.api_base || !provider.default_model)) {
    showMessage('请输入API端点和模型标识符后再测试', { title: '提示' })
    return
  }
  
  const request: TestConnectionRequest = {
    provider: provider.provider,
    api_key: provider.api_key,
    api_base: provider.api_base,
    model: provider.default_model
  }
  
  isTestingApi.value = true
  try {
    const response = await testAIConnection(request)
    
    if (response.success) {
      showMessage(response.message, { title: '连接成功' })
      
      // 如果返回了模型列表，更新提供商的模型列表
      if (response.models && response.models.length > 0) {
        provider.models = response.models.map(name => ({
          name,
          provider: providerId
        }))
      }
    } else {
      showMessage(response.message, { title: '连接失败' })
    }
  } catch (error) {
    console.error('API连接测试失败:', error)
    showMessage('API连接测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingApi.value = false
  }
}

// 保存AI配置
async function saveAIProviderConfig() {
  isSavingApiConfig.value = true
  try {
    // 保存所有提供商配置
    await saveAIConfig(aiProviders.value)
    
    showMessage('AI配置已保存', { title: '成功' })
    
    // 重新加载AI服务
    await reloadAIServices()
  } catch (error) {
    console.error('保存AI配置失败:', error)
    showMessage('保存AI配置失败: ' + error, { title: '错误' })
  } finally {
    isSavingApiConfig.value = false
  }
}

// 加载AI提供商配置
async function loadAIProvidersConfig() {
  try {
    // 一次性获取所有AI提供商的配置
    const config = await getAIConfig()
    if (config && config.providers) {
      // 合并数据库中的配置和前端的默认配置
      for (const providerId in defaultProviders) {
        if (config.providers[providerId]) {
          aiProviders.value[providerId] = {
            ...defaultProviders[providerId],
            ...config.providers[providerId],
          }
        }
      }
    }

    // 加载聊天模型列表
    await getChatModels()
    
    // 加载默认AI模型
    const defaultModel = await getDefaultAIModel('chat')
    if (defaultModel) {
      // 从返回的 provider 'openai' 找到对应的本地id 'chatgpt'
      for (const key in providerMapping) {
        if (providerMapping[key] === defaultModel.provider) {
          defaultAIModel.value = key
          break
        }
      }
    }
    
    // 获取服务状态
    const statuses = await getAIServiceStatus()
    for (const status of statuses) {
      if (aiProviders.value[status.provider]) {
        aiProviders.value[status.provider].enabled = status.is_available
      }
    }
  } catch (error) {
    console.error('加载AI配置失败:', error)
    showMessage('加载AI配置失败: ' + error, { title: '错误' })
  }
}

// 刷新AI使用统计
async function refreshAIStats() {
  try {
    const stats = await getAIUsageStats()
    aiStats.value = stats
  } catch (error) {
    console.error('获取AI使用统计失败:', error)
    showMessage('获取AI使用统计失败: ' + error, { title: '错误' })
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  await loadDefaultAIModel()
  await loadAIProvidersConfig()
  await refreshAIStats()
})
</script> 
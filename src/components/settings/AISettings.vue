<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('aiSettings.title') }}</h2>
      
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('aiSettings.defaultModel') }}</span>
        </label>
        <select v-model="selectedModel" class="select select-bordered w-full" @change="saveDefaultAIModel">
          <option value="openai">{{ $t('aiSettings.models.openai') }}</option>
          <option value="gemini">{{ $t('aiSettings.models.gemini') }}</option>
          <option value="anthropic">{{ $t('aiSettings.models.anthropic') }}</option>
          <option value="deepseek">{{ $t('aiSettings.models.deepseek') }}</option>
          <option value="qwen">{{ $t('aiSettings.models.qwen') }}</option>
          <option value="doubao">{{ $t('aiSettings.models.doubao') }}</option>
          <option value="grok">{{ $t('aiSettings.models.grok') }}</option>
          <option value="custom">{{ $t('aiSettings.models.custom') }}</option>
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
        <h3 class="font-bold mb-2">{{ $t('aiSettings.configTitle', { name: aiProviders[selectedConfigModel]?.name }) }}</h3>
        
        <div class="form-control mb-2">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.apiKey') }}</span>
          </label>
          <input 
            type="password" 
            v-model="aiProviders[selectedConfigModel].api_key" 
            :placeholder="$t('aiSettings.apiKeyPlaceholder')" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai' || selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.apiEndpoint') }}</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].api_base" 
            :placeholder="$t('aiSettings.apiEndpointPlaceholder')" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.orgId') }}</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].organization" 
            :placeholder="$t('aiSettings.orgIdPlaceholder')" 
            class="input input-bordered w-full"
          />
        </div>
        
        <div class="form-control mb-2" v-if="selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.modelName') }}</span>
          </label>
          <input 
            type="text" 
            v-model="aiProviders[selectedConfigModel].default_model" 
            :placeholder="$t('aiSettings.modelNamePlaceholder')" 
            class="input input-bordered w-full"
          />
        </div>
        
        <!-- 输入框 + datalist，同时支持手动输入和下拉选择 -->
        <div class="form-control mb-4" v-else>
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.defaultModelLabel') }}</span>
          </label>
          <input
            type="text"
            v-model="aiProviders[selectedConfigModel].default_model"
            :list="`models-${selectedConfigModel}`"
            :placeholder="$t('aiSettings.defaultModelPlaceholder')"
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
            <span v-if="isTestingApi">{{ $t('aiSettings.testing') }}</span>
            <span v-else>{{ $t('aiSettings.testConnection') }}</span>
          </button>
          
          <button 
            class="btn btn-accent" 
            :disabled="isSavingApiConfig" 
            @click="saveAIProviderConfig"
          >
            <span v-if="isSavingApiConfig">{{ $t('aiSettings.saving') }}</span>
            <span v-else>{{ $t('aiSettings.saveConfig') }}</span>
          </button>
        </div>
      </div>
      
      <div class="divider">{{ $t('aiSettings.usageStats') }}</div>
      
      <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
        <div class="stat">
          <div class="stat-title">{{ $t('aiSettings.totalConversations') }}</div>
          <div class="stat-value">{{ aiStats.conversations }}</div>
        </div>
        
        <div class="stat">
          <div class="stat-title">{{ $t('aiSettings.totalMessages') }}</div>
          <div class="stat-value">{{ aiStats.messages }}</div>
        </div>
        
        <div class="stat">
          <div class="stat-title">{{ $t('aiSettings.totalTokens') }}</div>
          <div class="stat-value">{{ aiStats.tokens.total }}</div>
          <div class="stat-desc">{{ $t('aiSettings.tokenUsage', { input: aiStats.tokens.input, output: aiStats.tokens.output }) }}</div>
        </div>
      </div>
      
      <button class="btn btn-sm btn-ghost mt-2" @click="refreshAIStats">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        {{ $t('aiSettings.refreshStats') }}
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
  getChatModels,
  testAIConnection, saveAIConfig, getAIConfig, getAIUsageStats, reloadAIServices,
  getAIServiceStatus, defaultProviders
} from '../../services/aiService'
import { useI18n } from 'vue-i18n'
import { useAIStore } from '../../stores/aiStore';
import { storeToRefs } from 'pinia';

const { t } = useI18n()
const aiStore = useAIStore()
const { selectedModel } = storeToRefs(aiStore)

// AI模型和提供商映射关系
const providerMapping: Record<string, string> = {
  'chatgpt': 'openai',
  'gemini': 'gemini',
  'deepseek': 'deepseek',
  'qwen': 'qwen',
  'claude': 'anthropic',
  'doubao': 'doubao',
  'grok': 'xai',
  'custom': 'custom'
}

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
    const providerId = selectedModel.value
    aiStore.setSelectedModel(providerId);

    const providerConfig = aiProviders.value[providerId]
    
    if (!providerConfig) {
      console.error(`Provider config not found for: ${providerId}`)
      showMessage(t('aiSettings.notifications.providerNotFound', { providerId }), { title: t('aiSettings.notifications.error') })
      return
    }

    const providerForBackend = providerMapping[providerId]
    const modelName = providerConfig.default_model
    
    if (!providerForBackend || !modelName) {
        console.error('Provider or model name is missing for the selected default AI model.')
        showMessage(t('aiSettings.notifications.providerOrModelMissing'), { title: t('aiSettings.notifications.error') })
        return
    }
    
    // 调用后端将全局默认模型存入数据库
    await invoke('set_default_ai_model', {
      modelType: 'chat',
      provider: providerForBackend,
      modelName: modelName
    })
    
    // 发出全局设置变更通知
    await emit('global-settings-changed', { key: 'defaultAIModel' })
    
    showMessage(t('aiSettings.notifications.defaultModelSaved'), { title: t('aiSettings.notifications.success') })

  } catch (error) {
    console.error('保存默认AI模型失败:', error)
    showMessage(t('aiSettings.notifications.defaultModelSaveFailed', { error }), { title: t('aiSettings.notifications.error') })
  }
}

// 加载默认AI模型
async function loadDefaultAIModel() {
  // The store is now the source of truth, so we don't need to load from backend/localStorage here.
  // The store is persisted and will be loaded automatically.
}

// 测试API连接
async function testApiConnection(providerId: string): Promise<void> {
  if (!aiProviders.value[providerId]) return
  
  const provider = aiProviders.value[providerId]
  
  // 对于非自定义模型，至少需要API Key
  if (providerId !== 'custom' && !provider.api_key) {
    showMessage(t('aiSettings.notifications.apiKeyRequired'), { title: t('aiSettings.notifications.tip') })
    return
  }
  
  // 对于自定义模型，至少需要Endpoint和Model Name
  if (providerId === 'custom' && (!provider.api_base || !provider.default_model)) {
    showMessage(t('aiSettings.notifications.customModelParamsRequired'), { title: t('aiSettings.notifications.tip') })
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
      showMessage(response.message, { title: t('aiSettings.notifications.connectionSuccess') })
      
      // 如果返回了模型列表，更新提供商的模型列表
      if (response.models && response.models.length > 0) {
        provider.models = response.models.map(name => ({
          name,
          provider: providerId
        }))
      }
    } else {
      showMessage(response.message, { title: t('aiSettings.notifications.connectionFailed') })
    }
  } catch (error) {
    console.error('API连接测试失败:', error)
    showMessage(t('aiSettings.notifications.apiTestFailed', { error }), { title: t('aiSettings.notifications.error') })
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
    
    showMessage(t('aiSettings.notifications.configSaved'), { title: t('aiSettings.notifications.success') })
    
    // 重新加载AI服务
    await reloadAIServices()

    // 刷新当前页面的配置信息
    await loadAIProvidersConfig()

    // 发出全局设置变更通知，通知其他组件配置已更新
    await emit('global-settings-changed', { key: 'aiConfig' })

  } catch (error) {
    console.error('保存AI配置失败:', error)
    showMessage(t('aiSettings.notifications.configSaveFailed', { error }), { title: t('aiSettings.notifications.error') })
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
    
    // 获取服务状态
    const statuses = await getAIServiceStatus()
    for (const status of statuses) {
      const providerKey = Object.keys(providerMapping).find(key => providerMapping[key] === status.provider);
      if (providerKey && aiProviders.value[providerKey]) {
        aiProviders.value[providerKey].enabled = status.is_available
      }
    }
  } catch (error) {
    console.error('加载AI配置失败:', error)
    showMessage(t('aiSettings.notifications.loadConfigFailed', { error }), { title: t('aiSettings.notifications.error') })
  }
}

// 刷新AI使用统计
async function refreshAIStats() {
  try {
    const stats = await getAIUsageStats()
    aiStats.value = stats
  } catch (error) {
    console.error('获取AI使用统计失败:', error)
    showMessage(t('aiSettings.notifications.refreshStatsFailed', { error }), { title: t('aiSettings.notifications.error') })
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  await loadDefaultAIModel()
  await loadAIProvidersConfig()
  await refreshAIStats()
})
</script> 
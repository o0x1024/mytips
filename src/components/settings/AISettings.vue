<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('aiSettings.title') }}</h2>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('aiSettings.defaultModel') }}</span>
        </label>
        <div class="relative">
          <select v-model="globalDefaultModelName" class="select select-bordered w-full pr-10"
            @change="saveDefaultAIModel">
            <option disabled value="">{{ $t('aiSettings.defaultModelPlaceholder') }}</option>
            <option v-for="model in availableModelOptions" :key="model" :value="model">{{ model }}</option>
          </select>
          <button class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-500 hover:text-primary"
            @click="saveDefaultAIModel">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 提供商选择网格 -->
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2 mb-4">
        <button v-for="(provider, id) in aiProviders" :key="id" 
          class="btn btn-sm text-xs h-auto py-2 px-3"
          :class="selectedConfigModel === id ? 'btn-primary' : 'btn-outline'"
          @click="selectedConfigModel = id">
          <span class="truncate">{{ provider.name }}</span>
        </button>
      </div>

      <!-- 配置区域 -->
      <div v-if="selectedConfigModel" class="bg-base-50 border border-base-300 rounded-lg p-4 mb-4">
        <div class="flex items-center gap-2 mb-4">
          <div class="w-3 h-3 rounded-full bg-primary"></div>
          <h3 class="font-bold text-lg">{{ $t('aiSettings.configTitle', { name: aiProviders[selectedConfigModel]?.name }) }}</h3>
        </div>

        <!-- 使用紧凑的网格布局 -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
          <!-- API密钥 -->
          <div class="form-control" v-if="selectedConfigModel !== 'ollama'">
            <label class="label py-1">
              <span class="label-text text-sm">{{ $t('aiSettings.apiKey') }}</span>
            </label>
            <input type="password" v-model="aiProviders[selectedConfigModel].api_key"
              :placeholder="$t('aiSettings.apiKeyPlaceholder')" 
              class="input input-bordered input-sm w-full" />
          </div>

          <!-- API端点 -->
          <div class="form-control" v-if="selectedConfigModel === 'openai' || selectedConfigModel === 'ollama' || selectedConfigModel === 'custom'">
            <label class="label py-1">
              <span class="label-text text-sm">{{ $t('aiSettings.apiEndpoint') }}</span>
            </label>
            <input type="text" v-model="aiProviders[selectedConfigModel].api_base"
              :placeholder="selectedConfigModel === 'ollama' ? 'http://localhost:11434' : $t('aiSettings.apiEndpointPlaceholder')" 
              class="input input-bordered input-sm w-full" />
          </div>

          <!-- 组织ID -->
          <div class="form-control" v-if="selectedConfigModel === 'openai'">
            <label class="label py-1">
              <span class="label-text text-sm">{{ $t('aiSettings.orgId') }}</span>
            </label>
            <input type="text" v-model="aiProviders[selectedConfigModel].organization"
              :placeholder="$t('aiSettings.orgIdPlaceholder')" 
              class="input input-bordered input-sm w-full" />
          </div>

          <!-- 模型名称 -->
          <div class="form-control">
            <label class="label py-1">
              <span class="label-text text-sm">
                {{ selectedConfigModel === 'custom' ? $t('aiSettings.modelName') : $t('aiSettings.defaultModelLabel') }}
              </span>
            </label>
            <input type="text" v-model="aiProviders[selectedConfigModel].default_model"
              :list="selectedConfigModel !== 'custom' ? `models-${selectedConfigModel}` : undefined"
              :placeholder="selectedConfigModel === 'custom' ? $t('aiSettings.modelNamePlaceholder') : $t('aiSettings.defaultModelPlaceholder')"
              class="input input-bordered input-sm w-full" />
            <datalist v-if="selectedConfigModel !== 'custom'" :id="`models-${selectedConfigModel}`">
              <option v-for="model in aiProviders[selectedConfigModel].models" :key="model.name" :value="model.name" />
            </datalist>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex flex-wrap gap-2 justify-end">
          <button class="btn btn-primary btn-sm" :disabled="isTestingApi"
            @click="() => testApiConnection(selectedConfigModel)">
            <svg v-if="isTestingApi" class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span v-if="isTestingApi">{{ $t('aiSettings.testing') }}</span>
            <span v-else>{{ $t('aiSettings.testConnection') }}</span>
          </button>

          <button class="btn btn-accent btn-sm" :disabled="isSavingApiConfig" @click="saveAIProviderConfig">
            <svg v-if="isSavingApiConfig" class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span v-if="isSavingApiConfig">{{ $t('aiSettings.saving') }}</span>
            <span v-else>{{ $t('aiSettings.saveConfig') }}</span>
          </button>
        </div>
      </div>

      <!-- 使用统计 -->
      <div class="bg-base-50 border border-base-300 rounded-lg p-4">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-bold text-base">{{ $t('aiSettings.usageStats') }}</h3>
          <button class="btn btn-xs btn-ghost" @click="refreshAIStats">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span class="ml-1 hidden sm:inline">{{ $t('aiSettings.refreshStats') }}</span>
          </button>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <div class="bg-white rounded-lg p-3 border border-base-200">
            <div class="text-xs text-base-content/60 mb-1">{{ $t('aiSettings.totalConversations') }}</div>
            <div class="text-xl font-bold text-primary">{{ aiStats.conversations }}</div>
          </div>

          <div class="bg-white rounded-lg p-3 border border-base-200">
            <div class="text-xs text-base-content/60 mb-1">{{ $t('aiSettings.totalMessages') }}</div>
            <div class="text-xl font-bold text-secondary">{{ aiStats.messages }}</div>
          </div>

          <div class="bg-white rounded-lg p-3 border border-base-200">
            <div class="text-xs text-base-content/60 mb-1">{{ $t('aiSettings.totalTokens') }}</div>
            <div class="text-xl font-bold text-accent">{{ aiStats.tokens.total }}</div>
            <div class="text-xs text-base-content/50 mt-1">{{ $t('aiSettings.tokenUsage', {
              input: aiStats.tokens.input, output: aiStats.tokens.output }) }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 自定义模型管理部分 -->
  <div v-if="selectedConfigModel === 'custom'" class="space-y-4 mt-4">
    <div class="flex justify-between items-center">
      <h3 class="font-medium">{{ $t('aiSettings.customModels') }}</h3>
      <button class="btn btn-sm btn-primary" @click="openCustomModelForm()">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
          stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        {{ $t('aiSettings.addCustomModel') }}
      </button>
    </div>

    <!-- 自定义模型列表 -->
    <div v-if="customModels.length === 0" class="text-center py-4 text-base-content/70">
      {{ $t('aiSettings.noCustomModels') }}
    </div>

    <div v-else class="space-y-3">
      <div v-for="model in customModels" :key="model.id" class="card bg-base-200 shadow-sm border border-base-300">
        <div class="card-body p-4">
          <div class="flex justify-between">
            <h3 class="card-title text-base">{{ model.name }}</h3>
            <div class="flex gap-1">
              <button class="btn btn-xs btn-ghost" @click="editCustomModel(model)">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button class="btn btn-xs btn-error" @click="confirmDeleteCustomModel(model.id)">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
          <div class="grid grid-cols-2 gap-2 text-sm">
            <div>
              <span class="font-semibold">{{ $t('aiSettings.endpoint') }}:</span>
              <span class="text-base-content/70">{{ model.endpoint }}</span>
            </div>
            <div>
              <span class="font-semibold">{{ $t('aiSettings.modelName') }}:</span>
              <span class="text-base-content/70">{{ model.model_name }}</span>
            </div>
            <div>
              <span class="font-semibold">{{ $t('aiSettings.adapterType') }}:</span>
              <span class="text-base-content/70">{{ model.adapter_type }}</span>
            </div>
            <div>
              <span class="font-semibold">{{ $t('aiSettings.apiKey') }}:</span>
              <span class="text-base-content/70">{{ model.api_key ? '******' : $t('aiSettings.notSet') }}</span>
            </div>
          </div>
          <div class="card-actions justify-end mt-2">
            <button class="btn btn-xs btn-outline" @click="testCustomModelConnection(model)">
              {{ $t('aiSettings.testConnection') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- 自定义模型表单弹窗 -->
  <dialog ref="customModelModal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-4">
        {{ editingCustomModel ? $t('aiSettings.editCustomModel') : $t('aiSettings.addCustomModel') }}
      </h3>

      <div class="space-y-4">
        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.modelName') }}</span>
          </label>
          <input type="text" v-model="customModelForm.name" class="input input-bordered w-full"
            :placeholder="$t('aiSettings.modelNamePlaceholder')" />
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.endpoint') }}</span>
          </label>
          <input type="text" v-model="customModelForm.endpoint" class="input input-bordered w-full"
            :placeholder="$t('aiSettings.endpointPlaceholder')" />
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.modelName') }}</span>
          </label>
          <input type="text" v-model="customModelForm.model_name" class="input input-bordered w-full"
            :placeholder="$t('aiSettings.modelIdentifierPlaceholder')" />
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.adapterType') }}</span>
          </label>
          <select v-model="customModelForm.adapter_type" class="select select-bordered w-full">
            <option value="openai">OpenAI</option>
            <option value="ollama">Ollama</option>
            <option value="custom">{{ $t('aiSettings.custom') }}</option>
          </select>
        </div>

        <div class="form-control">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.apiKey') }}</span>
          </label>
          <input type="password" v-model="customModelForm.api_key" class="input input-bordered w-full"
            :placeholder="$t('aiSettings.apiKeyOptionalPlaceholder')" />
        </div>
      </div>

      <div class="modal-action">
        <button class="btn" @click="closeCustomModelForm">{{ $t('aiSettings.cancel') }}</button>
        <button class="btn btn-primary" @click="saveCustomModel"
          :disabled="!customModelForm.name || !customModelForm.endpoint">
          {{ $t('aiSettings.save') }}
        </button>
      </div>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
// import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { showToast } from '../../services/notification'
import {
  AIProvider, TestConnectionRequest,
  getChatModels,
  testAIConnection, saveAIConfig, getAIConfig, getAIUsageStats, reloadAIServices,
  getAIServiceStatus, defaultProviders, CustomModelConfig,
  listCustomModelConfigs, addCustomModelConfig, updateCustomModelConfig, deleteCustomModelConfig
} from '../../services/aiService'
import { useI18n } from 'vue-i18n'
import { useAIStore } from '../../stores/aiStore';

const { t } = useI18n()
const aiStore = useAIStore()

// AI模型和提供商映射关系
const providerMapping: Record<string, string> = {
  'chatgpt': 'openai',
  'gemini': 'gemini',
  'deepseek': 'deepseek',
  'qwen': 'qwen',
  'claude': 'anthropic',
  'doubao': 'doubao',
  'grok': 'xai',
  'ollama': 'ollama',
  'custom': 'custom'
}

const selectedConfigModel = ref('openai')
const aiProviders = ref<Record<string, AIProvider>>(structuredClone(defaultProviders))
const isTestingApi = ref(false)
const isSavingApiConfig = ref(false)
const isInitialLoad = ref(true)
const chatModels = ref<ReturnType<typeof Array.prototype.slice> & { [index: number]: any }>([])
// 全局默认模型名（仅用于顶部下拉显示和修改）
const globalDefaultModelName = ref('')
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
    // 使用当前配置面板选择的提供商作为最终 providerId
    const modelName = globalDefaultModelName.value
    if (!modelName) {
      showToast(t('aiSettings.notifications.defaultModelSaveFailed', { error: 'model is empty' }), 'error')
      return
    }
    // 先以当前tab指示的 provider 作为候选
    const providerId = selectedConfigModel.value
    console.log('saveDefaultAIModel - selectedConfigModel.value:', providerId)
    console.log('saveDefaultAIModel - modelName:', modelName)
    let providerForBackend = providerId // 默认按当前选中的提供商

    // 智能推断用户想要的提供商
    // 1. 检查是否为ollama模型格式 (包含冒号，如 qwen3:4b)
    if (modelName.includes(':') && !modelName.startsWith('http')) {
      providerForBackend = 'ollama'
      console.log(`模型名称 ${modelName} 包含冒号，推断为ollama模型`)
    }
    // 2. 如果当前是custom选项卡，查找自定义模型
    else if (providerId === 'custom') {
      const matchedCustom = customModels.value.find(m => m.model_name === modelName)
      if (matchedCustom) {
        providerForBackend = `custom_${matchedCustom.id}`
        console.log(`在custom选项卡找到自定义模型: ${matchedCustom.id}`)
      }
    }
    // 3. 从后端返回的聊天模型列表中查找匹配的提供商
    else {
      const matchedModel = chatModels.value.find((m: any) => m?.name === modelName)
      if (matchedModel && matchedModel.provider) {
        providerForBackend = matchedModel.provider
        console.log(`从聊天模型列表找到模型 ${modelName} 属于提供商: ${matchedModel.provider}`)
      } else {
        // 4. 最后使用当前选项卡对应的提供商
        const backendProviderId = providerMapping[providerId] || providerId
        providerForBackend = backendProviderId
        console.log(`使用当前选项卡 ${providerId} 对应的提供商: ${providerForBackend}`)
      }
    }

    if (!providerForBackend) {
      console.error('Provider is missing for the selected default AI model.')
      showToast(t('aiSettings.notifications.providerMissing'), 'error')
      return
    }

    // 调用后端将全局默认模型存入数据库
    console.log(`保存默认AI模型: provider=${providerForBackend}, model=${modelName}`)
    await aiStore.updateDefaultChatModel(providerForBackend, modelName)

    // 发出全局设置变更通知
    await emit('global-settings-changed', { key: 'defaultAIModel' })

    showToast(t('aiSettings.notifications.defaultModelSaved'), 'success')
    console.log(`默认AI模型已保存: ${providerForBackend} / ${modelName}`)

  } catch (error) {
    console.error('Failed to save default AI model:', error)
    showToast(t('aiSettings.notifications.defaultModelSaveFailed', { error }), 'error')
  }
}

// 加载默认AI模型

// 测试API连接
async function testApiConnection(providerId: string): Promise<void> {
  if (!aiProviders.value[providerId]) return

  const provider = aiProviders.value[providerId]

  // Ollama不需要API Key，但需要endpoint
  if (providerId === 'ollama') {
    if (!provider.api_base) {
      provider.api_base = 'http://localhost:11434'
    }
  }
  // 对于非自定义模型和非Ollama，至少需要API Key
  else if (providerId !== 'custom' && !provider.api_key) {
    showToast(t('aiSettings.notifications.apiKeyRequired'), 'warning')
    return
  }

  // 对于自定义模型，至少需要Endpoint和Model Name
  if (providerId === 'custom' && (!provider.api_base || !provider.default_model)) {
    showToast(t('aiSettings.notifications.customModelParamsRequired'), 'warning')
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
      showToast(response.message, 'success')

      // 如果返回了模型列表，更新提供商的模型列表
      if (response.models && response.models.length > 0) {
        provider.models = response.models.map(name => ({ name, provider: providerId }))
      } else {
        provider.models = []
      }
      updateAvailableModelOptions()
    } else {
      showToast(response.message, 'error')
      // 连接失败，清空模型列表
      provider.models = []
      updateAvailableModelOptions()
    }
  } catch (error) {
    console.error('API connection test failed:', error)
    showToast(t('aiSettings.notifications.apiTestFailed', { error }), 'error')
    // 测试异常，清空模型列表
    provider.models = []
    updateAvailableModelOptions()
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

    showToast(t('aiSettings.notifications.configSaved'), 'success')

    // 重新加载AI服务
    await reloadAIServices()

    // 刷新当前页面的配置信息
    await loadAIProvidersConfig()

    // 发出全局设置变更通知，通知其他组件配置已更新
    await emit('global-settings-changed', { key: 'aiConfig' })

  } catch (error) {
    console.error('Failed to save AI config:', error)
    showToast(t('aiSettings.notifications.configSaveFailed', { error }), 'error')
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
    try {
      chatModels.value = await getChatModels()
    } catch (e) {
      console.warn('Failed to load chat models, using empty list:', e)
      chatModels.value = []
    }

    // 获取服务状态
    const statuses = await getAIServiceStatus()
    for (const status of statuses) {
      const providerKey = Object.keys(providerMapping).find(key => providerMapping[key] === status.provider);
      if (providerKey && aiProviders.value[providerKey]) {
        aiProviders.value[providerKey].enabled = status.is_available
      }
    }
    // 同步全局默认模型显示
    try {
      await aiStore.loadDefaultChatModel()
      globalDefaultModelName.value = aiStore.defaultChatModel || ''
      
      // 只在首次加载时自动设置选项卡，后续保存操作不要自动切换
      if (isInitialLoad.value) {
        // 当 provider 是 custom_<id> 时，高亮 custom 页签
        selectedConfigModel.value = aiStore.defaultChatProvider?.startsWith('custom_') ? 'custom' : (aiStore.defaultChatProvider || 'openai')
        isInitialLoad.value = false
      }
    } catch (e) {
      console.warn('Failed to sync default chat model from store:', e)
    }
    updateAvailableModelOptions()
  } catch (error) {
    console.error('Failed to load AI config:', error)
    showToast(t('aiSettings.notifications.loadConfigFailed', { error }), 'error')
  }
}

// 刷新AI使用统计
async function refreshAIStats() {
  try {
    const stats = await getAIUsageStats()
    aiStats.value = stats
  } catch (error) {
    console.error('Failed to fetch AI usage stats:', error)
    showToast(t('aiSettings.notifications.refreshStatsFailed', { error }), 'error')
  }
}

// 自定义模型相关数据和函数
const customModels = ref<CustomModelConfig[]>([])
const customModelModal = ref<HTMLDialogElement | null>(null)
const editingCustomModel = ref(false)
const customModelForm = ref<CustomModelConfig>({
  id: '',
  name: '',
  endpoint: '',
  model_name: '',
  adapter_type: 'openai'
})

// 加载自定义模型列表
async function loadCustomModels() {
  try {
    customModels.value = await listCustomModelConfigs()
    updateAvailableModelOptions()
  } catch (error) {
    console.error('Failed to load custom models:', error)
    showToast(t('aiSettings.notifications.loadCustomModelsFailed', { error }), 'error')
  }
}

// 打开自定义模型表单
function openCustomModelForm(model?: CustomModelConfig) {
  editingCustomModel.value = !!model

  if (model) {
    customModelForm.value = { ...model }
  } else {
    customModelForm.value = {
      id: generateUniqueId(),
      name: '',
      endpoint: '',
      model_name: '',
      adapter_type: 'openai'
    }
  }

  if (customModelModal.value) {
    customModelModal.value.showModal()
  }
}

// 关闭自定义模型表单
function closeCustomModelForm() {
  if (customModelModal.value) {
    customModelModal.value.close()
  }
}

// 编辑自定义模型
function editCustomModel(model: CustomModelConfig) {
  openCustomModelForm(model)
}

// 确认删除自定义模型
async function confirmDeleteCustomModel(modelId: string) {

  try {
    await deleteCustomModelConfig(modelId)
    await loadCustomModels()
    showToast(t('aiSettings.notifications.customModelDeleted'), 'success')
  } catch (error) {
    console.error('Failed to delete custom model:', error)
    showToast(t('aiSettings.notifications.deleteCustomModelFailed', { error }), 'error')
  }

}

// 保存自定义模型
async function saveCustomModel() {
  try {
    if (editingCustomModel.value) {
      await updateCustomModelConfig(customModelForm.value)
    } else {
      await addCustomModelConfig(customModelForm.value)
    }

    closeCustomModelForm()
    await loadCustomModels()
    showToast(t('aiSettings.notifications.customModelSaved'), 'success')
  } catch (error) {
    console.error('Failed to save custom model:', error)
    showToast(t('aiSettings.notifications.saveCustomModelFailed', { error }), 'error')
  }
}

// 测试自定义模型连接
async function testCustomModelConnection(model: CustomModelConfig) {
  try {
    const request: TestConnectionRequest = {
      provider: 'custom',
      api_key: model.api_key,
      api_base: model.endpoint,
      model: model.adapter_type
    }

    const response = await testAIConnection(request)

    if (response.success) {
      showToast(response.message, 'success')
    } else {
      showToast(response.message, 'error')
    }
  } catch (error) {
    console.error('Failed to test custom model connection:', error)
    showToast(t('aiSettings.notifications.testConnectionFailed', { error }), 'error')
  }
}

// 生成唯一ID
function generateUniqueId(): string {
  return Date.now().toString(36) + Math.random().toString(36).substring(2, 9)
}

// 获取所有可用的模型名称，用于默认模型下拉框
const availableModelOptions = ref<string[]>([])

// 更新全局默认AI模型下拉选项，只包含 getAIConfig 和 listCustomModelConfigs 返回的模型
function updateAvailableModelOptions() {
  // 合并所有来源，允许选择任意可用模型
  const providerModelsAll = Object.values(aiProviders.value)
    .flatMap(p => Array.isArray(p.models) ? p.models.map((m: any) => m?.name) : [])
    .filter(Boolean)
  const backendModelsAll = (chatModels.value || []).map((m: any) => m?.name).filter(Boolean)
  const customModelNames = customModels.value.map(m => m.model_name).filter(Boolean)
  availableModelOptions.value = [...new Set([...providerModelsAll, ...backendModelsAll, ...customModelNames])].sort()
}

// 切换提供商时，刷新可选模型
watch(selectedConfigModel, () => {
  updateAvailableModelOptions()
})

// 组件挂载时加载自定义模型
onMounted(async () => {
  await aiStore.loadDefaultChatModel()
  await loadAIProvidersConfig()
  await refreshAIStats()
  await loadCustomModels()
  updateAvailableModelOptions()
})
</script>
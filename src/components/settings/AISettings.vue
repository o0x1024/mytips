<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('aiSettings.title') }}</h2>

      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('aiSettings.defaultModel') }}</span>
        </label>
        <div class="relative">
          <select v-model="aiProviders[selectedConfigModel].default_model" class="select select-bordered w-full pr-10"
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

      <div class="tabs tabs-boxed mb-4 flex-wrap gap-2">
        <a v-for="(provider, id) in aiProviders" :key="id" class="tab"
          :class="{ 'tab-active': selectedConfigModel === id }" @click="selectedConfigModel = id">
          {{ provider.name }}
        </a>
      </div>

      <div v-if="selectedConfigModel" class="border rounded-lg p-4 mb-4">
        <h3 class="font-bold mb-2">{{ $t('aiSettings.configTitle', { name: aiProviders[selectedConfigModel]?.name }) }}
        </h3>

        <div class="form-control mb-2">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.apiKey') }}</span>
          </label>
          <input type="password" v-model="aiProviders[selectedConfigModel].api_key"
            :placeholder="$t('aiSettings.apiKeyPlaceholder')" class="input input-bordered w-full" />
        </div>

        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai' || selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.apiEndpoint') }}</span>
          </label>
          <input type="text" v-model="aiProviders[selectedConfigModel].api_base"
            :placeholder="$t('aiSettings.apiEndpointPlaceholder')" class="input input-bordered w-full" />
        </div>

        <div class="form-control mb-2" v-if="selectedConfigModel === 'openai'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.orgId') }}</span>
          </label>
          <input type="text" v-model="aiProviders[selectedConfigModel].organization"
            :placeholder="$t('aiSettings.orgIdPlaceholder')" class="input input-bordered w-full" />
        </div>

        <div class="form-control mb-2" v-if="selectedConfigModel === 'custom'">
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.modelName') }}</span>
          </label>
          <input type="text" v-model="aiProviders[selectedConfigModel].default_model"
            :placeholder="$t('aiSettings.modelNamePlaceholder')" class="input input-bordered w-full" />
        </div>

        <!-- 输入框 + datalist，同时支持手动输入和下拉选择 -->
        <div class="form-control mb-4" v-else>
          <label class="label">
            <span class="label-text">{{ $t('aiSettings.defaultModelLabel') }}</span>
          </label>
          <input type="text" v-model="aiProviders[selectedConfigModel].default_model"
            :list="`models-${selectedConfigModel}`" :placeholder="$t('aiSettings.defaultModelPlaceholder')"
            class="input input-bordered w-full" />
          <datalist :id="`models-${selectedConfigModel}`">
            <option v-for="model in aiProviders[selectedConfigModel].models" :key="model.name" :value="model.name" />
          </datalist>
        </div>

        <div class="flex justify-between">
          <button class="btn btn-primary" :disabled="isTestingApi"
            @click="() => testApiConnection(selectedConfigModel)">
            <span v-if="isTestingApi">{{ $t('aiSettings.testing') }}</span>
            <span v-else>{{ $t('aiSettings.testConnection') }}</span>
          </button>

          <button class="btn btn-accent" :disabled="isSavingApiConfig" @click="saveAIProviderConfig">
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
          <div class="stat-desc">{{ $t('aiSettings.tokenUsage', {
            input: aiStats.tokens.input, output:
              aiStats.tokens.output }) }}</div>
        </div>
      </div>

      <button class="btn btn-sm btn-ghost mt-2" @click="refreshAIStats">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        {{ $t('aiSettings.refreshStats') }}
      </button>
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
import { ref, onMounted } from 'vue'
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
    // 使用当前配置面板选择的提供商作为最终 providerId
    const providerId = selectedConfigModel.value
    const providerConfig = aiProviders.value[providerId]
    if (!providerConfig) {
      console.error(`Provider config not found for: ${providerId}`)
      showToast(t('aiSettings.notifications.providerNotFound', { providerId }), 'error')
      return
    }
    // 从对应提供商配置中读取默认模型
    let providerForBackend = providerId // 默认按当前选中的提供商
    const modelName = providerConfig.default_model

    // 若所选模型来自自定义模型（如 Ollama: qwen3:4b），将 provider 映射为 custom_<id>
    const matchedCustom = customModels.value.find(m => m.model_name === modelName)
    if (matchedCustom) {
      providerForBackend = `custom_${matchedCustom.id}`
    }

    if (!providerForBackend) {
      console.error('Provider is missing for the selected default AI model.')
      showToast(t('aiSettings.notifications.providerMissing'), 'error')
      return
    }

    // 调用后端将全局默认模型存入数据库
    await aiStore.updateDefaultChatModel(providerForBackend, modelName)

    // 发出全局设置变更通知
    await emit('global-settings-changed', { key: 'defaultAIModel' })

    showToast(t('aiSettings.notifications.defaultModelSaved'), 'success')

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

  // 对于非自定义模型，至少需要API Key
  if (providerId !== 'custom' && !provider.api_key) {
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
    await getChatModels()

    // 获取服务状态
    const statuses = await getAIServiceStatus()
    for (const status of statuses) {
      const providerKey = Object.keys(providerMapping).find(key => providerMapping[key] === status.provider);
      if (providerKey && aiProviders.value[providerKey]) {
        aiProviders.value[providerKey].enabled = status.is_available
      }
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
  // provider.models 里的 name
  const providerModels = Object.values(aiProviders.value)
    .flatMap(p => Array.isArray(p.models) ? p.models.map((m: any) => m.name) : [])
    .filter(Boolean)
  // customModels 里的 model_name
  const customModelNames = customModels.value.map(m => m.model_name).filter(Boolean)
  availableModelOptions.value = [...new Set([...providerModels, ...customModelNames])].sort()
}

// 组件挂载时加载自定义模型
onMounted(async () => {
  await aiStore.loadDefaultChatModel()
  await loadAIProvidersConfig()
  await refreshAIStats()
  await loadCustomModels()
  updateAvailableModelOptions()
})
</script>
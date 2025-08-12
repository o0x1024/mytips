import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getDefaultAIModel, setDefaultAIModel } from '../services/aiService'

export const useAIStore = defineStore('ai', () => {
  const selectedModel = ref('chatgpt')
  // 全局默认聊天模型（供 TIP/翻译/解释 等非会话功能使用）
  const defaultChatProvider = ref<string>('gemini')
  const defaultChatModel = ref<string>('gemini-2.0-flash')

  function setSelectedModel(modelId: string) {
    selectedModel.value = modelId
  }

  async function loadDefaultChatModel() {
    try {
      const dm = await getDefaultAIModel('chat')
      if (dm) {
        defaultChatProvider.value = dm.provider
        defaultChatModel.value = dm.name
      }
    } catch (e) {
      console.error('AIStore: failed to load default chat model:', e)
    }
  }

  async function updateDefaultChatModel(provider: string, modelName: string) {
    try {
      await setDefaultAIModel('chat', provider, modelName)
      defaultChatProvider.value = provider
      defaultChatModel.value = modelName
    } catch (e) {
      console.error('AIStore: failed to update default chat model:', e)
      throw e
    }
  }

  return {
    selectedModel,
    setSelectedModel,
    defaultChatProvider,
    defaultChatModel,
    loadDefaultChatModel,
    updateDefaultChatModel,
  }
}, {
  persist: true,
}) 
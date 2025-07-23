import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAIStore = defineStore('ai', () => {
  const selectedModel = ref('chatgpt')

  function setSelectedModel(modelId: string) {
    selectedModel.value = modelId
  }

  return {
    selectedModel,
    setSelectedModel,
  }
}, {
  persist: true,
}) 
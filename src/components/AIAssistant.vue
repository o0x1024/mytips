<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 可用的AI服务
const aiServices = [
  { id: 'chatgpt', name: 'ChatGPT', icon: '🤖' },
  { id: 'gemini', name: 'Google Gemini', icon: '🧠' },
  { id: 'deepseek', name: 'DeepSeek', icon: '🔍' }
]

// 已打开的AI服务
const openServices = ref<string[]>([])

// 加载已打开的AI服务
const loadOpenServices = async () => {
  try {
    openServices.value = await invoke('get_open_ai_assistants')
  } catch (error) {
    console.error('获取打开的AI助手失败:', error)
  }
}

// 打开AI助手
const openAIAssistant = async (serviceId: string) => {
  try {
    await invoke('open_ai_assistant', { aiService: serviceId })
    await loadOpenServices()
  } catch (error) {
    console.error(`打开AI助手(${serviceId})失败:`, error)
  }
}

// 关闭AI助手
const closeAIAssistant = async (serviceId: string) => {
  try {
    await invoke('close_ai_assistant', { aiService: serviceId })
    await loadOpenServices()
  } catch (error) {
    console.error(`关闭AI助手(${serviceId})失败:`, error)
  }
}

// 初始化
onMounted(async () => {
  await loadOpenServices()
})
</script>

<template>
  <div class="ai-assistant-container">
    <h2 class="text-xl font-bold mb-4">AI 助手</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div 
        v-for="service in aiServices" 
        :key="service.id"
        class="card bg-base-200 shadow-md hover:shadow-lg transition-shadow"
      >
        <div class="card-body">
          <h3 class="card-title">
            <span class="text-2xl mr-2">{{ service.icon }}</span>
            {{ service.name }}
          </h3>
          
          <div class="card-actions justify-end mt-4">
            <button 
              v-if="openServices.includes(service.id)"
              @click="closeAIAssistant(service.id)"
              class="btn btn-sm btn-error"
            >
              关闭窗口
            </button>
            <button 
              v-else
              @click="openAIAssistant(service.id)"
              class="btn btn-sm btn-primary"
            >
              打开窗口
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ai-assistant-container {
  padding: 1rem;
}
</style> 
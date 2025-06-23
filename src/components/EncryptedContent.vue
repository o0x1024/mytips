<template>
  <div class="encrypted-content-container h-full flex flex-col items-center justify-center p-8 bg-base-200/50">
    <div class="max-w-md w-full text-center">
      <!-- 加密图标 -->
      <div class="mb-6">
        <div class="w-24 h-24 mx-auto bg-warning/20 rounded-full flex items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
        </div>
      </div>

      <!-- 标题和描述 -->
      <h3 class="text-2xl font-bold text-base-content mb-2">
        {{ title }}
      </h3>
      <p class="text-base-content/70 mb-6">
        {{ description }}
      </p>

      <!-- 解锁按钮 -->
      <button 
        class="btn btn-warning btn-wide gap-2 mb-3"
        @click="$emit('unlock')"
        :disabled="loading">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
        </svg>
        {{ loading ? '解锁中...' : '输入密码解锁' }}
      </button>

      <!-- 取消加密按钮 -->
      <button 
        class="btn btn-outline btn-error btn-wide gap-2"
        @click="$emit('decrypt')"
        :disabled="loading">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-11.83 1.82M15 7a6 6 0 00-11.83 1.82M15 7H9m6 0V1m0 6v6m0 0H9m6 0v6" />
        </svg>
        取消加密
      </button>

      <!-- 加密信息 -->
      <div class="mt-8 p-4 bg-base-100 rounded-lg border border-base-300">
        <div class="text-sm text-base-content/60 space-y-2">
          <div class="flex items-center justify-between">
            <span>加密状态:</span>
            <span class="badge badge-warning badge-sm">已加密</span>
          </div>
          <div v-if="encryptedAt" class="flex items-center justify-between">
            <span>加密时间:</span>
            <span class="text-xs">{{ formatDate(encryptedAt) }}</span>
          </div>
          <div class="flex items-center justify-between">
            <span>加密算法:</span>
            <span class="text-xs">AES-256</span>
          </div>
        </div>
      </div>

      <!-- 安全提示 -->
      <div class="mt-4 text-xs text-base-content/50">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 inline mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        此内容已加密保护，需要正确的密码才能查看
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 组件属性
defineProps({
  title: {
    type: String,
    default: '内容已加密'
  },
  description: {
    type: String,
    default: '此内容受密码保护，请输入正确的密码来查看内容。'
  },
  loading: {
    type: Boolean,
    default: false
  },
  encryptedAt: {
    type: Number,
    default: null
  }
})

// 组件事件
defineEmits(['unlock', 'decrypt'])

// 格式化日期
function formatDate(timestamp: number): string {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN')
}
</script>

<style scoped>
.encrypted-content-container {
  background-image: 
    linear-gradient(45deg, transparent 40%, rgba(255, 255, 255, 0.1) 50%, transparent 60%),
    linear-gradient(-45deg, transparent 40%, rgba(255, 255, 255, 0.05) 50%, transparent 60%);
  background-size: 20px 20px;
  animation: shimmer 3s ease-in-out infinite;
}

@keyframes shimmer {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.95; }
}

.btn-warning:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
</style> 
<template>
  <div v-if="showDialog" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-base-100 rounded-lg shadow-xl max-w-md w-full mx-4">
      <div class="p-6">
        <div class="flex items-center mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-primary mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          <h3 class="text-lg font-semibold">{{ $t('updateDialog.newVersionTitle') }}</h3>
        </div>
        
        <div class="mb-4">
          <p class="text-base-content mb-2">
            <strong>{{ $t('updateDialog.version') }} {{ updateInfo?.version }}</strong>
          </p>
          <p class="text-sm text-base-content/80 mb-2">
            {{ $t('updateDialog.releaseDate') }}: {{ formatDate(updateInfo?.pub_date) }}
          </p>
          <div v-if="updateInfo?.body" class="bg-base-200 p-3 rounded text-sm max-h-32 overflow-y-auto">
            <div v-html="formatReleaseNotes(updateInfo.body)"></div>
          </div>
        </div>

        <!-- 下载进度 -->
        <div v-if="isDownloading" class="mb-4">
          <div class="flex justify-between text-sm mb-2">
            <span>{{ $t('updateDialog.downloadProgress') }}</span>
            <span>{{ downloadProgress }}%</span>
          </div>
          <progress class="progress progress-primary w-full" :value="downloadProgress" max="100"></progress>
          <div class="text-xs text-base-content/60 mt-1">
            {{ formatBytes(downloadedBytes) }} / {{ formatBytes(totalBytes) }}
          </div>
        </div>

        <!-- 安装状态 -->
        <div v-if="isInstalling" class="mb-4">
          <div class="flex items-center justify-center">
            <span class="loading loading-spinner loading-md mr-2"></span>
            <span>{{ $t('updateDialog.installing') }}</span>
          </div>
        </div>

        <!-- 错误信息 -->
        <div v-if="errorMessage" class="mb-4 p-3 bg-error/10 border border-error/20 rounded text-error text-sm">
          {{ errorMessage }}
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-3 justify-end">
          <button 
            v-if="!isDownloading && !isInstalling" 
            class="btn btn-ghost" 
            @click="closeDialog"
          >
            {{ $t('updateDialog.remindLater') }}
          </button>
          <button 
            v-if="!isDownloading && !isInstalling" 
            class="btn btn-primary" 
            @click="downloadAndInstall"
          >
            {{ $t('updateDialog.updateNow') }}
          </button>
          <button 
            v-if="isDownloading" 
            class="btn btn-ghost" 
            @click="cancelDownload"
          >
            {{ $t('updateDialog.cancelDownload') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { relaunch } from '@tauri-apps/plugin-process'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useUpdateStore } from '../stores/updateStore'
import { showConfirm } from '../services/dialog'

const { t, locale } = useI18n()
const updateStore = useUpdateStore()

// Props
interface Props {
  modelValue: boolean
}

const props = defineProps<Props>()

// Emits
const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

// 状态管理
const showDialog = ref(false)
const isDownloading = ref(false)
const isInstalling = ref(false)
const errorMessage = ref('')
const downloadProgress = ref(0)
const downloadedBytes = ref(0)
const totalBytes = ref(0)

// 计算属性
const updateInfo = computed(() => updateStore.updateInfo)

let currentUpdate: any = null
let unlistenProgress: (() => void) | null = null
let unlistenInstalling: (() => void) | null = null
let unlistenCompleted: (() => void) | null = null

// 监听外部控制
watch(() => props.modelValue, (newValue) => {
  showDialog.value = newValue
  if (newValue) {
    setupEventListeners()
  } else {
    cleanupEventListeners()
  }
})

// 监听对话框状态变化
watch(showDialog, (newValue) => {
  emit('update:modelValue', newValue)
})

// 设置事件监听器
async function setupEventListeners() {
  try {
    // 监听下载进度
    unlistenProgress = await listen('update-progress', (event) => {
      downloadProgress.value = Math.round(event.payload as number)
    })
    
    // 监听安装开始
    unlistenInstalling = await listen('update-installing', () => {
      isDownloading.value = false
      isInstalling.value = true
    })
    
    // 监听更新完成
    unlistenCompleted = await listen('update-completed', () => {
      isInstalling.value = false
      showRestartPrompt()
    })
  } catch (error) {
    console.error('设置事件监听器失败:', error)
  }
}

// 清理事件监听器
function cleanupEventListeners() {
  if (unlistenProgress) unlistenProgress()
  if (unlistenInstalling) unlistenInstalling()
  if (unlistenCompleted) unlistenCompleted()
}

// 下载并安装更新
async function downloadAndInstall() {
  if (!updateStore.updateInfo?.available && !currentUpdate) return
  
  isDownloading.value = true
  errorMessage.value = ''
  downloadProgress.value = 0
  downloadedBytes.value = 0
  totalBytes.value = 0
  
  try {
    // 优先使用后端自动更新
    if (updateStore.updateInfo?.available) {
      await invoke('start_auto_update')
      return
    }
    
    // 后备方案：使用前端 API
    if (currentUpdate) {
      await currentUpdate.downloadAndInstall((event: any) => {
        switch (event.event) {
          case 'Started':
            totalBytes.value = event.data.contentLength || 0
            console.log(`开始下载 ${formatBytes(totalBytes.value)}`)
            break
            
          case 'Progress':
            downloadedBytes.value += event.data.chunkLength || 0
            if (totalBytes.value > 0) {
              downloadProgress.value = Math.round((downloadedBytes.value / totalBytes.value) * 100)
            }
            break
            
          case 'Finished':
            console.log('下载完成')
            isDownloading.value = false
            isInstalling.value = true
            break
        }
      })
      
      console.log('更新安装完成')
      showRestartPrompt()
    }
    
  } catch (error) {
    console.error('更新下载/安装失败:', error)
    errorMessage.value = `更新失败: ${error}`
    isDownloading.value = false
    isInstalling.value = false
  }
}

// 取消下载
function cancelDownload() {
  closeDialog()
}

// 显示重启提示
async function showRestartPrompt() {
  const confirmed = await showConfirm(t('updateDialog.restartPromptMessage'), {
    title: t('updateDialog.restartPromptTitle'),
    confirmText: t('updateDialog.restartNow'),
    cancelText: t('updateDialog.restartLater')
  })
  
  if (confirmed) {
    relaunch()
  } else {
    closeDialog()
    // 清除更新状态，因为用户选择稍后重启
    updateStore.clearUpdate()
  }
}

// 关闭对话框
function closeDialog() {
  showDialog.value = false
  isDownloading.value = false
  isInstalling.value = false
  errorMessage.value = ''
  currentUpdate = null
  cleanupEventListeners()
}

// 格式化日期
function formatDate(dateString?: string): string {
  if (!dateString) return t('updateDialog.unknownDate')
  
  try {
    // 尝试解析日期字符串
    const date = new Date(dateString)
    
    // 检查日期是否有效
    if (isNaN(date.getTime())) {
      console.warn('无效的日期格式:', dateString)
      return dateString // 如果无法解析，返回原始字符串
    }
    
    // 返回本地化的日期格式
    return date.toLocaleDateString(locale.value, {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    })
  } catch (error) {
    console.error('日期格式化错误:', error, '原始日期:', dateString)
    return dateString // 出错时返回原始字符串
  }
}

// 格式化字节大小
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化发布说明
function formatReleaseNotes(notes: string): string {
  // 简单的 Markdown 格式化
  return notes
    .replace(/\n/g, '<br>')
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
    .replace(/#{1,6}\s*(.*)/g, '<strong>$1</strong>')
}
</script>

<style scoped>
.progress {
  transition: all 0.3s ease;
}

code {
  background-color: hsl(var(--b2));
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
}
</style> 
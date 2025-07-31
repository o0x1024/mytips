<template>
  <div class="sync-status-indicator">
    <!-- 同步状态指示器 -->
    <div class="flex items-center gap-2 text-sm">
      <!-- 在线状态 -->
      <div class="flex items-center gap-1">
        <div 
          class="w-2 h-2 rounded-full transition-colors duration-300"
          :class="{
            'bg-success animate-pulse': isOnline && !isError,
            'bg-error': isError,
            'bg-warning': !isOnline && !isError
          }"
        ></div>
        <span class="text-xs opacity-70">
          {{ isError ? $t('syncStatus.error') : (isOnline ? $t('syncStatus.online') : $t('syncStatus.offline')) }}
        </span>
      </div>

      <!-- 同步状态 -->
      <div v-if="supportsSync" class="flex items-center gap-1">
        <div v-if="isSyncing" class="flex items-center gap-1">
          <span class="loading loading-spinner loading-xs"></span>
          <span class="text-xs opacity-70">{{ $t('syncStatus.syncing') }}</span>
        </div>
        <div v-else-if="lastSyncTime" class="flex items-center gap-1">
          <svg class="w-3 h-3 opacity-70" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="text-xs opacity-70">{{ formatLastSync(lastSyncTime) }}</span>
        </div>
      </div>

      <!-- 错误指示 -->
      <div v-if="errorCount > 0" class="flex items-center gap-1">
        <svg class="w-3 h-3 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z" />
        </svg>
        <span class="text-xs text-error">{{ errorCount }}</span>
      </div>

      <!-- 操作按钮 -->
      <div class="flex items-center gap-1">
        <!-- 手动同步按钮 -->
        <button 
          v-if="supportsSync && !isSyncing"
          class="btn btn-ghost btn-xs p-1 min-h-0 h-auto"
          @click="triggerSync"
          :title="$t('syncStatus.manualSync')"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>

        <!-- 详细状态按钮 -->
        <button 
          class="btn btn-ghost btn-xs p-1 min-h-0 h-auto"
          @click="showDetails = !showDetails"
          :title="$t('syncStatus.showDetails')"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>

    <!-- 详细状态面板 -->
    <div v-if="showDetails" class="mt-2 p-3 bg-base-200 rounded-lg text-xs space-y-2">
      <div class="flex justify-between">
        <span class="opacity-70">{{ $t('syncStatus.mode') }}:</span>
        <span class="font-medium">{{ $t(`dataSettings.modes.${currentMode}.label`) }}</span>
      </div>
      
      <div v-if="supportsSync" class="flex justify-between">
        <span class="opacity-70">{{ $t('syncStatus.lastSync') }}:</span>
        <span class="font-medium">{{ lastSyncTime ? formatFullDate(lastSyncTime) : $t('syncStatus.never') }}</span>
      </div>
      
      <div v-if="syncStatus" class="space-y-1">
        <div class="flex justify-between">
          <span class="opacity-70">{{ $t('syncStatus.isSyncing') }}:</span>
          <span class="font-medium">{{ syncStatus.is_syncing ? $t('common.yes') : $t('common.no') }}</span>
        </div>
        <div v-if="syncStatus.pending_changes" class="flex justify-between">
          <span class="opacity-70">{{ $t('syncStatus.pendingChanges') }}:</span>
          <span class="font-medium text-warning">{{ syncStatus.pending_changes }}</span>
        </div>
        <div v-if="syncStatus.next_sync_time" class="flex justify-between">
          <span class="opacity-70">{{ $t('syncStatus.nextSync') }}:</span>
          <span class="font-medium">{{ formatFullDate(syncStatus.next_sync_time) }}</span>
        </div>
      </div>

      <div v-if="errorMessage" class="text-error text-xs">
        <span class="opacity-70">{{ $t('syncStatus.lastError') }}:</span>
        <div class="mt-1 font-mono bg-error/10 p-2 rounded">{{ errorMessage }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useDatabaseStore } from '../stores/databaseStore'
import { storeToRefs } from 'pinia'
// import type { SyncStats } from '../types/database'

const { t } = useI18n()
const databaseStore = useDatabaseStore()

// 从store获取响应式数据
const { 
  currentMode, 
  // isConnected, 
  supportsSync, 
  isOnline, 
  syncStatus,
  isLoading 
} = storeToRefs(databaseStore)

// 本地状态
const showDetails = ref(false)
// const syncStats = ref<SyncStats | null>(null)
const errorMessage = ref('')
const refreshInterval = ref<NodeJS.Timeout | null>(null)

// 计算属性
const isSyncing = computed(() => isLoading.value)
const isError = computed(() => !!errorMessage.value)
const lastSyncTime = computed(() => syncStatus.value?.last_sync_time || 0)
const errorCount = computed(() => syncStatus.value?.sync_error_count || 0)

// 方法
const triggerSync = async () => {
  try {
    errorMessage.value = ''
    await databaseStore.sync()
    // 同步后重新获取状态来更新统计信息
    await databaseStore.loadStatus(true)
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    errorMessage.value = errorMsg
    console.error('Manual sync failed:', error)
  }
}

const formatLastSync = (timestamp: number) => {
  if (!timestamp) return t('syncStatus.never')
  
  const now = Date.now()
  const diff = now - timestamp
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)
  
  if (minutes < 1) return t('syncStatus.justNow')
  if (minutes < 60) return t('syncStatus.minutesAgo', { minutes })
  if (hours < 24) return t('syncStatus.hoursAgo', { hours })
  return t('syncStatus.daysAgo', { days })
}

const formatFullDate = (timestamp: number) => {
  if (!timestamp) return t('syncStatus.never')
  return new Date(timestamp).toLocaleString()
}

const startAutoRefresh = () => {
  // 每30秒刷新一次状态
  refreshInterval.value = setInterval(async () => {
    try {
      await databaseStore.loadStatus()
    } catch (error) {
      console.error('Failed to refresh sync status:', error)
    }
  }, 30000) as NodeJS.Timeout
}

const stopAutoRefresh = () => {
  if (refreshInterval.value) {
    clearInterval(refreshInterval.value)
    refreshInterval.value = null
  }
}

// 生命周期
onMounted(() => {
  startAutoRefresh()
})

onUnmounted(() => {
  stopAutoRefresh()
})
</script>

<style scoped>
.sync-status-indicator {
  @apply transition-all duration-300;
}
</style>
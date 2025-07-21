<template>
  <div class="space-y-6">
    <!-- 初始化加载状态 -->
    <div v-if="isInitializing" class="flex justify-center items-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
      <span class="ml-3 text-base-content/70">正在加载设置...</span>
    </div>
    
    <template v-else>
      <!-- 数据库配置卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
            </svg>
            数据库配置
          </h2>

          <!-- 数据库模式选择 -->
          <div class="form-control mb-6">
            <label class="label">
              <span class="label-text font-medium">数据库模式</span>
              <span v-if="isLoadingStatus" class="loading loading-spinner loading-sm"></span>
            </label>
            
            <!-- 当前模式状态 -->
            <div v-if="databaseStatus" class="alert mb-4">
              <div class="flex items-center gap-2">
                <span class="text-lg">📊</span>
                <div>
                  <div class="font-semibold">
                    当前模式: {{ availableModes.find(m => m.value === currentDatabaseMode)?.label || currentDatabaseMode }}
                  </div>
                  <div class="text-sm opacity-70">
                    {{ availableModes.find(m => m.value === currentDatabaseMode)?.description }}
                  </div>
                </div>
              </div>
              <div class="flex gap-2 ml-auto">
                <div class="badge" :class="databaseStatus.is_connected ? 'badge-success' : 'badge-error'">
                  {{ databaseStatus.is_connected ? '已连接' : '未连接' }}
                </div>
                <div v-if="databaseStore.supportsSync" 
                     class="badge" 
                     :class="databaseStatus?.sync_status?.is_online ? 'badge-primary' : 'badge-warning'">
                  {{ databaseStatus?.sync_status?.is_online ? '同步可用' : '同步离线' }}
                </div>
              </div>
            </div>

            <!-- 模式选择卡片 -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              <div v-for="mode in availableModes.filter(m => m.supported)" 
                   :key="mode.value"
                   class="card bg-base-200 hover:bg-base-300 cursor-pointer transition-all"
                   :class="{ 'border-2 border-primary': currentDatabaseMode === mode.value }"
                   @click="() => currentDatabaseMode === mode.value ? null : switchDatabaseMode(mode.value)">
                <div class="card-body p-4">
                  <div class="flex items-center gap-3">
                    <div class="flex-1">
                      <h3 class="font-semibold">{{ mode.label }}</h3>
                      <p class="text-sm opacity-70">{{ mode.description }}</p>
                    </div>
                    <div v-if="currentDatabaseMode === mode.value" class="badge badge-primary">当前</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 快速操作按钮 -->
            <div class="flex gap-2 flex-wrap">
              <button 
                v-if="currentDatabaseMode !== 'embedded_replica' && hasRemoteConfig"
                class="btn btn-primary btn-sm"
                @click="switchToEmbeddedReplicaMode"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                切换到嵌入式副本模式（推荐）
              </button>
              
              <button 
                v-if="databaseStore.supportsSync"
                class="btn btn-outline btn-sm"
                @click="performDatabaseSync"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                立即同步
              </button>
              
              <button 
                class="btn btn-outline btn-sm"
                @click="testCurrentDatabaseConnection"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                测试连接
              </button>
              
              <button 
                v-if="currentDatabaseMode === 'embedded_replica' || currentDatabaseMode === 'local'"
                class="btn btn-outline btn-info btn-sm"
                @click="optimizeDatabaseWAL"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                优化WAL文件
              </button>
              
              <button 
                v-if="currentDatabaseMode === 'local'"
                class="btn btn-outline btn-warning btn-sm"
                @click="cleanupLocalDatabaseFiles"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                清理数据库文件
              </button>
            </div>
          </div>

          <!-- 远程数据库配置 -->
          <div class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">远程数据库URL</span>
              </label>
              <input 
                type="text" 
                v-model="syncConfig.remote_url" 
                placeholder="libsql://your-database.turso.io"
                class="input input-bordered"
                :disabled="isOperationInProgress"
                v-if="syncConfig"
              />
              <label class="label">
                <span class="label-text-alt">远程Turso数据库的连接URL</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">认证令牌</span>
                <span v-if="isLocalDevUrl" class="label-text-alt text-info">
                  (本地环境可选)
                </span>
              </label>
              <div class="join">
                <input 
                  :type="showSyncToken ? 'text' : 'password'"
                  v-model="syncConfig.auth_token" 
                  :placeholder="isLocalDevUrl ? '本地开发环境可以为空' : '输入认证令牌'"
                  class="input input-bordered join-item flex-1"
                  :disabled="isOperationInProgress"
                  v-if="syncConfig"
                />
                <button 
                  type="button" 
                  class="btn btn-outline join-item"
                  @click="showSyncToken = !showSyncToken"
                >
                  <svg v-if="showSyncToken" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464M9.878 9.878a3 3 0 104.243 4.243M4.929 19.071L19.071 4.929" />
                  </svg>
                  <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </button>
              </div>
            </div>

            <div class="flex gap-2">
              <button 
                class="btn btn-primary btn-sm" 
                @click="testAndSaveRemoteConfig"
                :disabled="!canTestSync || isTestingSync"
              >
                <span v-if="isTestingSync" class="loading loading-spinner loading-sm mr-2"></span>
                {{ isTestingSync ? '测试中...' : '测试并保存配置' }}
              </button>
            </div>
          </div>

          <!-- 数据库信息显示 -->
          <div v-if="databaseInfo" class="mt-4 space-y-4">
            <!-- 基本信息统计 -->
            <div class="stats shadow bg-base-200">
              <div class="stat">
                <div class="stat-title">数据库大小</div>
                <div class="stat-value text-sm">{{ databaseInfo.size }}</div>
                <div v-if="databaseInfo.mode_description" class="stat-desc">{{ databaseInfo.mode_description }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">笔记数量</div>
                <div class="stat-value text-sm">{{ databaseInfo.noteCount }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">分类数量</div>
                <div class="stat-value text-sm">{{ databaseInfo.categoryCount }}</div>
              </div>
              <div v-if="databaseInfo.isRemote" class="stat">
                <div class="stat-title">在线状态</div>
                <div class="stat-value text-sm">
                  <div class="badge" :class="databaseInfo.isOnline ? 'badge-success' : 'badge-error'">
                    {{ databaseInfo.isOnline ? '在线' : '离线' }}
                  </div>
                </div>
              </div>
            </div>

            <!-- 数据库路径信息 -->
            <div v-if="databaseInfo.database_path" class="alert alert-info">
              <div class="flex-1 min-w-0">
                <div class="font-semibold text-sm mb-2 flex items-center gap-2">
                  <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
                  </svg>
                  数据库路径
                </div>
                <div class="bg-base-100 rounded-lg border p-3 space-y-2">
                  <div class="text-xs font-mono break-all leading-relaxed select-all 
                              bg-base-200 p-2 rounded border-l-4 border-primary
                              hover:bg-base-300 transition-colors cursor-text
                              min-h-[2.5rem] flex items-center"
                       :title="'点击选择全部 • ' + databaseInfo.database_path">
                    {{ databaseInfo.database_path }}
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center text-xs opacity-70 gap-1">
                    <span>最后修改: {{ databaseInfo.lastModified }}</span>
                    <span class="text-primary font-medium">{{ databaseInfo.size }}</span>
                  </div>
                </div>
              </div>
              <div class="flex-none ml-3">
                <button 
                  class="btn btn-ghost btn-xs tooltip tooltip-left"
                  @click="copyDatabasePath"
                  data-tip="复制路径"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 数据备份与恢复卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
            </svg>
            数据备份与恢复
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
            <button 
              class="btn btn-outline" 
              @click="backupDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
              </svg>
              {{ isOperationInProgress ? '备份中...' : '本地备份' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="restoreDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 17l4 4 4-4m-4-5v9m-8-6h16" />
              </svg>
              {{ isOperationInProgress ? '恢复中...' : '本地恢复' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="exportAsMarkdown"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
              </svg>
              {{ isOperationInProgress ? '导出中...' : '导出Markdown' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="migrateConfigToDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
              </svg>
              {{ isOperationInProgress ? '迁移中...' : '迁移配置' }}
            </button>
          </div>
        </div>
      </div>
    </template>

    <!-- 冲突解决对话框 -->
    <ConflictResolutionDialog v-if="showConflictResolutionDialog" @close="showConflictResolutionDialog = false" />
    
    <!-- 同步历史对话框 -->
    <SyncHistoryDialog v-if="showSyncHistoryDialogVisible" @close="showSyncHistoryDialogVisible = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { showMessage, showConfirm } from '../../services/dialog'
import ConflictResolutionDialog from '../dialogs/ConflictResolutionDialog.vue'
import SyncHistoryDialog from '../dialogs/SyncHistoryDialog.vue'
import { useDatabaseStore } from '../../stores/databaseStore'
import { DatabaseService } from '../../services/databaseService'
import type { LegacySyncConfig } from '../../types/database'

// 使用数据库store
const databaseStore = useDatabaseStore()

// 数据库模式管理（从store获取）
const currentDatabaseMode = computed(() => databaseStore.currentMode)
const availableModes = computed(() => databaseStore.availableModes)
const databaseStatus = computed(() => databaseStore.databaseStatus)

// 操作状态管理
const isOperationInProgress = ref(false)
const isInitializing = ref(true)
const isLoadingStatus = ref(false)
const isTestingSync = ref(false)
const showSyncToken = ref(false)
const showConflictResolutionDialog = ref(false)
const showSyncHistoryDialogVisible = ref(false)

// 数据库信息
const databaseInfo = ref<{
  size: string
  noteCount: number
  categoryCount: number
  lastModified: string
  database_path?: string
  mode_description?: string
  isRemote?: boolean
  isOnline?: boolean
} | null>(null)

// 同步配置
const syncConfig = ref<LegacySyncConfig>({
  remote_url: '',
  auth_token: '',
  sync_mode: 'OFFLINE' as const,
  sync_interval: 300,
  auto_sync_enabled: true,
  is_online: false
})

// 计算属性
const canTestSync = computed(() => {
  return syncConfig.value?.remote_url?.trim() !== ''
})

const isLocalDevUrl = computed(() => {
  const url = syncConfig.value?.remote_url?.trim() || ''
  return url.startsWith('http://127.0.0.1') || 
         url.startsWith('http://localhost') ||
         url.startsWith('https://127.0.0.1') ||
         url.startsWith('https://localhost')
})

const hasRemoteConfig = computed(() => {
  const hasUrl = syncConfig.value?.remote_url?.trim() !== ''
  if (!hasUrl) return false
  
  const hasToken = syncConfig.value?.auth_token?.trim() !== ''
  
  // 本地开发环境允许空token
  if (isLocalDevUrl.value) {
    return true
  }
  
  // 生产环境需要token
  return hasToken
})

// === 数据库状态管理 ===

/**
 * 加载数据库状态
 */
async function loadDatabaseStatus() {
  isLoadingStatus.value = true
  try {
    const status = await databaseStore.loadStatus(true)
    
    // 更新数据库信息
    if (status?.database_info) {
      databaseInfo.value = {
        size: status.database_info.size,
        noteCount: status.database_info.note_count,
        categoryCount: status.database_info.category_count,
        lastModified: status.database_info.last_modified,
        database_path: status.database_info.database_path,
        mode_description: status.database_info.mode_description,
        isRemote: databaseStore.supportsSync,
        isOnline: status.is_connected
      }
    }
    
    console.log('Database status loaded:', status)
  } catch (error) {
    console.error('Failed to load database status:', error)
    showMessage('加载数据库状态失败: ' + error, { title: '错误' })
  } finally {
    isLoadingStatus.value = false
  }
}

/**
 * 切换数据库模式
 */
async function switchDatabaseMode(mode: string, params?: any) {
  if (isOperationInProgress.value) return
  
  const modeOption = availableModes.value.find(m => m.value === mode)
  if (!modeOption) {
    showMessage('不支持的数据库模式', { title: '错误' })
    return
  }
  
  const confirmed = await showConfirm(
    `确定要切换到${modeOption.label}吗？\n\n${modeOption.description}`,
    {
      title: '切换数据库模式',
      confirmText: '切换',
      cancelText: '取消'
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await databaseStore.switchMode(mode, params || syncConfig.value)
    showMessage(`${result}\n\n笔记本和笔记数据已自动刷新。`, { title: '切换成功' })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Failed to switch database mode:', error)
    showMessage('切换数据库模式失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 切换到嵌入式副本模式（推荐）
 */
async function switchToEmbeddedReplicaMode() {
  
  // 检查是否有远程配置信息
  if (!hasRemoteConfig.value) {
    const url = syncConfig.value?.remote_url?.trim() || ''
    
    let message = '请先配置远程数据库信息：\n\n1. 输入远程数据库URL'
    
    if (url === '') {
      message += ' (例如: libsql://your-db.turso.io 或 http://127.0.0.1:8888)'
    }
    
    if (!isLocalDevUrl.value) {
      message += '\n2. 输入认证令牌'
      message += '\n3. 点击"测试并保存配置"'
      message += '\n4. 再尝试切换到嵌入式副本模式'
    } else {
      message += '\n2. 对于本地开发环境，认证令牌可以为空'
      message += '\n3. 点击"测试并保存配置"（可选）'
      message += '\n4. 再尝试切换到嵌入式副本模式'
    }
    
    showMessage(message, { title: '需要先配置远程数据库' })
    return
  }
  
  // 构建完整的嵌入式副本模式参数
  const embeddedReplicaParams = {
    remote_url: syncConfig.value.remote_url,
    auth_token: syncConfig.value.auth_token,
    sync_interval: syncConfig.value.sync_interval || 300,
    sync_interval_seconds: syncConfig.value.sync_interval || 300,
    local_path: undefined, // 使用默认本地路径
    read_your_writes: true, // 启用读写一致性
    auto_sync_enabled: true
  }
  
  try {
    await switchDatabaseMode('embedded_replica', embeddedReplicaParams)
  } catch (error) {
    // 如果错误提到WAL索引，提供清理选项
    const errorMessage = String(error)
    if (errorMessage.includes('wal_index') || errorMessage.includes('WAL') || errorMessage.includes('schema verification failure')) {
      const shouldCleanup = await showConfirm(
        '切换失败，可能由于数据库文件状态不一致导致。\n\n是否要自动清理数据库文件后重试？\n\n清理操作是安全的，通常能解决此类问题。',
        {
          title: '数据库切换失败',
          confirmText: '清理并重试',
          cancelText: '取消'
        }
      )
      
      if (shouldCleanup) {
        isOperationInProgress.value = true
        try {
          // 执行清理
          const cleanupResult = await DatabaseService.cleanupLocalDatabaseFiles()
          console.log('Cleanup result:', cleanupResult)
          
          // 短暂延时后重试
          await new Promise(resolve => setTimeout(resolve, 500))
          
          // 清理后重试
          await switchDatabaseMode('embedded_replica', embeddedReplicaParams)
        } catch (retryError) {
          console.error('Retry after cleanup failed:', retryError)
          showMessage('重试失败: ' + retryError, { title: '错误' })
        } finally {
          isOperationInProgress.value = false
        }
      }
    } else {
      showMessage('切换数据库模式失败: ' + error, { title: '错误' })
    }
  }
}

/**
 * 执行数据库同步
 */
async function performDatabaseSync() {
  if (isOperationInProgress.value) return
  
  if (!databaseStore.supportsSync) {
    showMessage('当前数据库模式不支持同步', { title: '提示' })
    return
  }
  
  isOperationInProgress.value = true
  try {
    const result = await databaseStore.sync()
    showMessage(`${result}\n\n笔记本和笔记数据已自动刷新。`, { title: '同步成功' })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Database sync failed:', error)
    showMessage('数据库同步失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 测试数据库连接
 */
async function testCurrentDatabaseConnection() {
  if (isOperationInProgress.value) return
  
  isOperationInProgress.value = true
  try {
    const connected = await databaseStore.testConnection()
    if (connected) {
      showMessage('数据库连接正常', { title: '连接测试' })
    } else {
      showMessage('数据库连接失败', { title: '连接测试' })
    }
  } catch (error) {
    console.error('Connection test failed:', error)
    showMessage('连接测试失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 清理本地数据库文件
 */
async function cleanupLocalDatabaseFiles() {
  if (isOperationInProgress.value) return
  
  const confirmed = await showConfirm(
    '确定要清理本地数据库文件吗？\n\n这将删除WAL和SHM等临时文件，可以解决数据库切换时的WAL索引错误。\n\n操作是安全的，不会删除主数据库文件中的数据。',
    {
      title: '清理数据库文件',
      confirmText: '清理',
      cancelText: '取消'
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await DatabaseService.cleanupLocalDatabaseFiles()
    showMessage(result, { title: '清理完成' })
  } catch (error) {
    console.error('Cleanup failed:', error)
    showMessage('清理失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 优化WAL文件
 */
async function optimizeDatabaseWAL() {
  if (isOperationInProgress.value) return
  
  const confirmed = await showConfirm(
    '确定要优化WAL文件吗？\n\n优化WAL文件可以提高数据库性能，但需要较长时间。\n\n操作是安全的，不会删除主数据库文件中的数据。',
    {
      title: '优化WAL文件',
      confirmText: '优化',
      cancelText: '取消'
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
         const result = await DatabaseService.optimizeDatabaseWAL()
    showMessage(result, { title: '优化完成' })
  } catch (error) {
    console.error('WAL optimization failed:', error)
    showMessage('WAL优化失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

// === 远程配置管理 ===

/**
 * 测试并保存远程配置
 */
async function testAndSaveRemoteConfig() {
  if (!canTestSync.value || !syncConfig.value) return
  
  isTestingSync.value = true
  try {
    const result = await invoke('test_remote_connection', {
      remote_url: syncConfig.value.remote_url,
      auth_token: syncConfig.value.auth_token
    })
    
    if (result) {
      // 保存配置
      await invoke('save_sync_config', { config: syncConfig.value })
      showMessage('远程数据库连接测试成功，配置已保存！', { title: '测试成功' })
      
      // 刷新状态
      await loadDatabaseStatus()
    } else {
      showMessage('远程数据库连接测试失败，请检查URL和令牌！', { title: '测试失败' })
    }
  } catch (error) {
    console.error('Test connection failed:', error)
    showMessage('测试连接失败: ' + error, { title: '错误' })
  } finally {
    isTestingSync.value = false
  }
}

/**
 * 加载同步配置
 */
async function loadSyncConfig() {
  try {
    const config = await invoke('get_sync_config') as any
    
    if (config) {
      syncConfig.value = config
    } else {
      // 使用默认配置
      syncConfig.value = {
        remote_url: '',
        auth_token: '',
        sync_mode: 'OFFLINE',
        sync_interval: 300,
        auto_sync_enabled: true,
        is_online: false
      }
    }
    
  } catch (error) {
    console.error('Load sync config failed:', error)
    // 使用默认配置作为回退
    syncConfig.value = {
      remote_url: '',
      auth_token: '',
      sync_mode: 'OFFLINE',
      sync_interval: 300,
      auto_sync_enabled: true,
      is_online: false
    }
  }
}

// === 辅助函数 ===



/**
 * 复制数据库路径到剪贴板
 */
async function copyDatabasePath() {
  if (!databaseInfo.value?.database_path) {
    showMessage('数据库路径为空', { title: '提示' })
    return
  }
  try {
    await invoke('copy_to_clipboard', { text: databaseInfo.value.database_path })
    showMessage('数据库路径已复制到剪贴板！', { title: '复制成功' })
  } catch (error) {
    console.error('Failed to copy database path:', error)
    showMessage('复制数据库路径失败: ' + error, { title: '错误' })
  }
}



// === 数据操作方法 ===

async function backupDatabase() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('backup_database') as string
    showMessage(result, { title: '备份成功' })
  } catch (error) {
    console.error('Backup failed:', error)
    showMessage('备份失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function restoreDatabase() {
  const confirmed = await showConfirm(
    '恢复数据库将覆盖当前所有数据，此操作不可撤销！确定要继续吗？',
    { title: '确认恢复', confirmText: '确认恢复', cancelText: '取消' }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('restore_database') as string
    showMessage(result, { title: '恢复成功' })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Restore failed:', error)
    showMessage('恢复失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function exportAsMarkdown() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('export_as_markdown') as string
    showMessage(result, { title: '导出成功' })
  } catch (error) {
    console.error('Export failed:', error)
    showMessage('导出失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function migrateConfigToDatabase() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('migrate_config_to_database') as string
    showMessage(result, { title: '迁移成功' })
  } catch (error) {
    console.error('Migration failed:', error)
    showMessage('迁移失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  try {
    // 加载数据库状态和配置
    await Promise.all([
      loadDatabaseStatus(),
      loadSyncConfig()
    ])
  
    
  } catch (error) {
    console.error('[DataSettings] Initialization failed:', error)
  } finally {
    isInitializing.value = false
  }
})
</script>

<style scoped>
/* 卡片悬停效果 */
.card:hover {
  transform: translateY(-2px);
  transition: transform 0.2s ease;
}

/* 模式选择卡片样式 */
.card.bg-base-200 {
  transition: all 0.3s ease;
}

.card.bg-base-200:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

/* 选中状态的卡片样式 */
.card.border-primary {
  box-shadow: 0 4px 12px rgba(var(--p) / 0.3);
}

.card.border-warning {
  box-shadow: 0 4px 12px rgba(var(--wa) / 0.3);
}

.card.border-success {
  box-shadow: 0 4px 12px rgba(var(--su) / 0.3);
}

/* 统计信息样式 */
.stats .stat {
  padding: 1rem;
}

.stats .stat-value {
  font-size: 1rem;
}

/* 数据库路径显示优化 */
.alert.alert-info {
  @apply border-info/20 bg-info/5;
}

/* 路径文本框样式 */
.select-all {
  user-select: all;
  -webkit-user-select: all;
  -moz-user-select: all;
  -ms-user-select: all;
}

/* 工具提示样式 */
.tooltip:before {
  @apply text-xs;
}

/* 响应式文本 */
@media (max-width: 640px) {
  .font-mono {
    word-break: break-all;
    overflow-wrap: break-word;
  }
  
  .text-xs {
    line-height: 1.4;
  }
}

/* 复制按钮动画 */
.btn:active {
  transform: scale(0.95);
}

/* 路径容器悬停效果 */
.cursor-text:hover {
  @apply shadow-sm;
}
</style> 
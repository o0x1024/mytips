import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { DatabaseService, DatabaseModeManager } from '../services/databaseService'
import type { DatabaseStatusResponse } from '../types/database'

/**
 * 数据库状态管理Store
 */
export const useDatabaseStore = defineStore('database', () => {
  // 状态
  const databaseStatus = ref<DatabaseStatusResponse | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const lastUpdated = ref<number>(0)
  
  // 数据库切换事件计数器，用于通知其他组件刷新数据
  const databaseChangeCounter = ref(0)

  // 计算属性
  const currentMode = computed(() => databaseStatus.value?.current_mode || 'local')
  const isConnected = computed(() => databaseStatus.value?.is_connected || false)
  const supportsSync = computed(() => DatabaseModeManager.supportsSync(currentMode.value))
  const isOnline = computed(() => databaseStatus.value?.sync_status?.is_online || false)
  const availableModes = computed(() => DatabaseModeManager.getDatabaseModeOptions())

  // 数据库信息
  const databaseInfo = computed(() => databaseStatus.value?.database_info)
  const syncStatus = computed(() => databaseStatus.value?.sync_status)

  // Actions

  /**
   * 加载数据库状态
   */
  async function loadStatus(force = false) {
    // 避免频繁刷新（5秒内不重复请求）
    if (!force && Date.now() - lastUpdated.value < 5000) {
      return databaseStatus.value
    }

    isLoading.value = true
    error.value = null

    try {
      const status = await DatabaseService.getDatabaseStatus()
      databaseStatus.value = status
      lastUpdated.value = Date.now()
      return status
    } catch (err) {
      error.value = (err as Error).message
      console.error('[DatabaseStore] Failed to load database status:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 切换数据库模式
   */
  async function switchMode(mode: string, params: any = {}) {
    isLoading.value = true
    error.value = null

    try {
      let result: string

      switch (mode) {
        case 'local':
          result = await DatabaseService.switchToLocalMode(params.path)
          break
        case 'remote':
          result = await DatabaseService.switchToRemoteMode(params.url, params.auth_token)
          break
        case 'embedded_replica':
          console.log('[DatabaseStore] switchToEmbeddedReplicaMode params:', params)
          result = await DatabaseService.switchToEmbeddedReplicaMode(
            params.local_path || undefined,
            params.remote_url,
            params.auth_token,
            params.sync_interval_seconds || params.sync_interval || 300
          )
          break
        default:
          throw new Error(`Unsupported database mode: ${mode}`)
      }

      // 切换成功后立即刷新状态
      await loadStatus(true)
      
      // 触发数据库切换事件，通知其他组件刷新数据
      databaseChangeCounter.value++
      console.log('[DatabaseStore] Database switched successfully, notifying other stores to refresh data')
      
      return result
    } catch (err) {
      error.value = (err as Error).message
      console.error('[DatabaseStore] Failed to switch database mode:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 执行同步
   */
  async function sync() {
    if (!supportsSync.value) {
      throw new Error('Current database mode does not support sync')
    }

    isLoading.value = true
    error.value = null

    try {
      const result = await DatabaseService.syncDatabase()
      // 同步后刷新状态
      await loadStatus(true)
      
      // 触发数据库同步事件，通知其他组件刷新数据
      databaseChangeCounter.value++
      console.log('[DatabaseStore] Database synced successfully, notifying other stores to refresh data')
      
      return result
    } catch (err) {
      error.value = (err as Error).message
      console.error('[DatabaseStore] Database sync failed:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 测试连接
   */
  async function testConnection() {
    isLoading.value = true
    error.value = null

    try {
      const connected = await DatabaseService.testDatabaseConnection()
      // 测试后刷新状态
      await loadStatus(true)
      return connected
    } catch (err) {
      error.value = (err as Error).message
      console.error('[DatabaseStore] Connection test failed:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 清除错误状态
   */
  function clearError() {
    error.value = null
  }

  /**
   * 重置状态
   */
  function reset() {
    databaseStatus.value = null
    isLoading.value = false
    error.value = null
    lastUpdated.value = 0
  }

  return {
    // 状态
    databaseStatus,
    isLoading,
    error,
    lastUpdated,
    databaseChangeCounter,

    // 计算属性
    currentMode,
    isConnected,
    supportsSync,
    isOnline,
    availableModes,
    databaseInfo,
    syncStatus,

    // Actions
    loadStatus,
    switchMode,
    sync,
    testConnection,
    clearError,
    reset
  }
})

export default useDatabaseStore 
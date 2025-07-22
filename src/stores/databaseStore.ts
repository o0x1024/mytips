import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { DatabaseService, DatabaseModeManager } from '../services/databaseService'
import type { DatabaseStatusResponse } from '../types/database'
import { useTipsStore } from '../stores/tipsStore'

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
  const currentMode = ref('local'); // 默认值
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
      currentMode.value = status.current_mode
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
   * @param mode 要切换到的模式
   * @param params 特定模式需要的参数
   */
  async function switchMode(mode: string, params: any) {
    console.log(`[DatabaseStore] Attempting to switch to ${mode} mode with params:`, params);
    try {
        let command;
        let payload: any = { ...params };

        switch (mode) {
            case 'local':
                command = 'switch_to_local_mode';
                break;
            case 'remote':
                command = 'switch_to_remote_mode';
                if (payload.remote_url) {
                    payload.url = payload.remote_url;
                    delete payload.remote_url;
                }
                break;
            case 'embedded_replica':
                command = 'switch_to_embedded_replica_mode';
                if (payload.remote_url) {
                  payload.url = payload.remote_url
                  delete payload.remote_url
                }
                break;
            default:
                throw new Error(`Unsupported database mode: ${mode}`);
        }

        console.log(`[DatabaseStore] Invoking command: ${command} with payload:`, payload);
        const result = await invoke(command, { config: payload });

        // 切换成功后更新当前模式
        currentMode.value = mode;
        console.log(`[DatabaseStore] Switched to ${mode} mode successfully.`);
        
        // 重新加载状态
        await loadStatus(true);
        
        // 重新加载笔记和分类
        const tipsStore = useTipsStore();
        await tipsStore.initializeData();

        return result;
    } catch (error) {
        console.error(`[DatabaseStore] Failed to switch to ${mode} mode:`, error);
        throw error;
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
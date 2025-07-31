import { useDatabaseStore } from '../stores/databaseStore'

/**
 * 状态回调函数类型
 */
export type StatusCallback = (message: string) => void

/**
 * 应用初始化服务
 */
export class AppInitializer {
  private static initialized = false

  /**
   * 初始化应用
   */
  static async initialize(statusCallback?: StatusCallback): Promise<void> {
    if (this.initialized) {
      console.log('[AppInitializer] Already initialized, skipping...')
      statusCallback?.('应用已初始化')
      return
    }

    console.log('[AppInitializer] Starting application initialization...')
    statusCallback?.('正在初始化应用...')

    try {
      await this.initializeDatabase(statusCallback)
      this.initialized = true
      console.log('[AppInitializer] Application initialization completed successfully')
      statusCallback?.('初始化完成')
    } catch (error) {
      console.error('[AppInitializer] Application initialization failed:', error)
      statusCallback?.('初始化失败')
      throw error
    }
  }

  /**
   * 初始化数据库
   */
  private static async initializeDatabase(statusCallback?: StatusCallback): Promise<void> {
    console.log('[AppInitializer] Starting database initialization...')
    statusCallback?.('正在初始化数据库...')
    const databaseStore = useDatabaseStore()

    try {
      // 加载数据库状态
      console.log('[AppInitializer] Loading database status...')
      statusCallback?.('正在加载数据库状态...')
      await databaseStore.loadStatus(true)
      
      const status = databaseStore.databaseStatus
      if (!status) {
        console.warn('[AppInitializer] Could not load database status')
        statusCallback?.('无法加载数据库状态')
        return
      }

      console.log(`[AppInitializer] Database initialization results:`)
      console.log(`  - Database mode: ${status.current_mode}`)
      console.log(`  - Database connected: ${status.is_connected}`)
      console.log(`  - Supports sync: ${databaseStore.supportsSync}`)
      console.log(`  - Is online: ${databaseStore.isOnline}`)
      
      if (status.database_info) {
        console.log(`  - Database info:`, {
          path: status.database_info.database_path,
          size: status.database_info.size,
          noteCount: status.database_info.note_count,
          categoryCount: status.database_info.category_count
        })
        statusCallback?.(`已加载 ${status.database_info.note_count} 条笔记`)
      }

      // 如果数据库未连接，尝试重新连接
      if (!status.is_connected) {
        console.log('[AppInitializer] Database not connected, attempting to reconnect...')
        statusCallback?.('正在重新连接数据库...')
        
        try {
          const connected = await databaseStore.testConnection()
          if (connected) {
            console.log('[AppInitializer] Database reconnection successful')
            statusCallback?.('数据库连接成功')
            // 重新加载状态以获取最新信息
            await databaseStore.loadStatus(true)
          } else {
            console.warn('[AppInitializer] Database reconnection failed')
            statusCallback?.('数据库连接失败')
          }
        } catch (error) {
          console.error('[AppInitializer] Database reconnection error:', error)
          statusCallback?.('数据库连接错误')
        }
      } else {
        statusCallback?.('数据库连接正常')
      }

      // 如果支持同步且在线，检查同步状态
      if (databaseStore.supportsSync && databaseStore.isOnline) {
        console.log('[AppInitializer] Sync supported and online, sync capabilities available')
        statusCallback?.('正在检查同步状态...')
        
        // 检查同步状态
        if (status.sync_status) {
          console.log('[AppInitializer] Sync status:', {
            isOnline: status.sync_status.is_online,
            lastSyncTime: status.sync_status.last_sync_time,
            isSyncing: status.sync_status.is_syncing
          })
          statusCallback?.('同步功能可用')
        }
      } else {
        console.log('[AppInitializer] Sync not available or offline')
        statusCallback?.('离线模式')
      }

      console.log('[AppInitializer] Database initialization completed successfully')
      statusCallback?.('数据库初始化完成')

    } catch (error) {
      console.error('[AppInitializer] Database initialization failed:', error)
      // 提供更详细的错误信息
      if (error instanceof Error) {
        console.error('[AppInitializer] Error details:', {
          name: error.name,
          message: error.message,
          stack: error.stack
        })
      }
      // 不抛出错误，让应用继续运行，但记录问题
    }
  }

  /**
   * 重置初始化状态（仅用于开发/测试）
   */
  static reset(): void {
    this.initialized = false
  }

  /**
   * 检查是否已初始化
   */
  static isInitialized(): boolean {
    return this.initialized
  }
}

/**
 * 便捷的初始化函数
 */
export async function initializeApp(statusCallback?: StatusCallback): Promise<void> {
  return AppInitializer.initialize(statusCallback)
}

export default AppInitializer
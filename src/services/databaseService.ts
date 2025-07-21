import { invoke } from '@tauri-apps/api/core'
import type {
    DatabaseStatusResponse,
    SwitchModeRequest,
    SyncStats,
    LegacySyncConfig
} from '../types/database'

/**
 * 数据库服务 - 封装统一数据库管理API
 */
export class DatabaseService {

    /**
     * 获取数据库状态
     */
    static async getDatabaseStatus(): Promise<DatabaseStatusResponse> {
        return await invoke('get_database_status')
    }

    /**
     * 切换数据库模式
     */
    static async switchDatabaseMode(request: SwitchModeRequest): Promise<string> {
        return await invoke('switch_database_mode', { request })
    }

    /**
     * 切换到本地模式
     */
    static async switchToLocalMode(path?: string): Promise<string> {
        return await invoke('switch_to_local_mode', { path })
    }

    /**
     * 切换到远程模式
     */
    static async switchToRemoteMode(url: string, authToken: string): Promise<string> {
        return await invoke('switch_to_remote_mode', { url, auth_token: authToken })
    }

    /**
     * 切换到嵌入式副本模式（推荐模式）
     * 参数顺序必须与后端API匹配：localPath, remoteUrl, authToken, syncIntervalSeconds
     */
    static async switchToEmbeddedReplicaMode(
        localPath: string | undefined,
        remoteUrl: string,
        authToken: string,
        syncIntervalSeconds?: number
    ): Promise<string> {
        console.log('[DatabaseService] switchToEmbeddedReplicaMode called with:', {
            localPath, remoteUrl, authToken, syncIntervalSeconds
        })
        
        const params = {
            local_path: localPath,
            remote_url: remoteUrl,
            auth_token: authToken,
            sync_interval_seconds: syncIntervalSeconds
        }
        
        console.log('[DatabaseService] Invoking with params:', params)
        
        return await invoke('switch_to_embedded_replica_mode', params)
    }

    /**
     * 执行数据库同步
     */
    static async syncDatabase(): Promise<string> {
        return await invoke('sync_database')
    }

    /**
     * 测试数据库连接
     */
    static async testDatabaseConnection(): Promise<boolean> {
        return await invoke('test_database_connection')
    }

    /**
     * 获取支持的数据库模式
     */
    static async getSupportedDatabaseModes(): Promise<string[]> {
        return await invoke('get_supported_database_modes')
    }

    // Legacy API兼容性方法

    /**
     * 获取同步状态 (Legacy)
     */
    static async getLegacySyncStatus(): Promise<any> {
        return await invoke('get_sync_status')
    }

    /**
     * 保存同步配置 (Legacy)
     */
    static async saveLegacySyncConfig(config: LegacySyncConfig): Promise<void> {
        return await invoke('save_sync_config', { config })
    }

    /**
     * 手动同步 (Legacy)
     */
    static async manualSync(): Promise<SyncStats> {
        return await invoke('manual_sync')
    }

    /**
     * 测试远程连接 (Legacy)
     */
    static async testRemoteConnection(remoteUrl: string, authToken: string): Promise<boolean> {
        return await invoke('test_remote_connection', {
            remote_url: remoteUrl,
            auth_token: authToken
        })
    }

    /**
     * 获取数据库信息 (Legacy)
     */
    static async getLegacyDatabaseInfo(): Promise<any> {
        return await invoke('get_database_info')
    }

    /**
     * 选择数据库文件 (Legacy)
     */
    static async selectDatabaseFile(): Promise<any> {
        return await invoke('select_database_file')
    }

    /**
     * 创建新数据库 (Legacy)
     */
    static async createNewDatabase(): Promise<any> {
        return await invoke('create_new_database')
    }

    /**
     * 重置到默认数据库 (Legacy)
     */
    static async resetToDefaultDatabase(): Promise<any> {
        return await invoke('reset_to_default_database')
    }

    /**
     * 清理本地数据库文件（解决WAL索引问题）
     */
    static async cleanupLocalDatabaseFiles(path?: string): Promise<string> {
        return await invoke<string>('cleanup_local_database_files', { path })
    }

    /**
     * 优化WAL文件大小（减少WAL索引文件大小）
     */
    static async optimizeDatabaseWAL(): Promise<string> {
        return await invoke<string>('optimize_database_wal')
    }
}

/**
 * 数据库模式管理器
 */
export class DatabaseModeManager {

    /**
     * 获取可用的数据库模式选项
     */
    static getDatabaseModeOptions() {
        return [
            {
                value: 'local',
                label: '本地模式',
                description: '数据存储在本地SQLite文件中，无网络依赖',
                supported: true
            },
            {
                value: 'remote',
                label: '远程模式',
                description: '直接连接到远程Turso数据库，需要网络连接',
                supported: true
            },
            {
                value: 'embedded_replica',
                label: '嵌入式副本模式（推荐）',
                description: '本地副本+远程同步，最佳性能和可靠性',
                supported: true
            },
            {
                value: 'in_memory',
                label: '内存模式',
                description: '数据存储在内存中，仅用于测试',
                supported: false // 通常不在生产环境中使用
            }
        ]
    }

    /**
     * 检查模式是否支持同步
     */
    static supportsSync(mode: string): boolean {
        return ['remote', 'embedded_replica'].includes(mode)
    }

    /**
     * 获取模式的推荐配置
     */
    static getRecommendedConfig(mode: string) {
        switch (mode) {
            case 'local':
                return {
                    description: '适合离线使用，数据安全存储在本地',
                    features: ['离线可用', '快速访问', '无网络依赖', '数据隐私']
                }
            case 'remote':
                return {
                    description: '适合多设备同步，需要稳定网络连接',
                    features: ['实时同步', '多设备访问', '云端备份', '团队协作']
                }
            case 'embedded_replica':
                return {
                    description: '最佳选择，结合本地性能和远程同步的优势',
                    features: ['本地性能', '自动同步', '离线可用', '数据安全']
                }
            case 'in_memory':
                return {
                    description: '临时存储，仅用于测试和开发',
                    features: ['极快速度', '临时存储', '无持久化', '开发测试']
                }
            default:
                return null
        }
    }
}

export default DatabaseService 
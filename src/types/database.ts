// 数据库管理相关类型定义

export interface DatabaseInfo {
  size: string
  note_count: number
  category_count: number
  last_modified: string
  database_path?: string
  mode_description?: string
  version?: string
  path?: string
}

export interface SyncStatus {
  is_syncing: boolean
  last_sync_time: number
  sync_error_count: number
  is_online: boolean
  pending_changes?: number
  next_sync_time?: number
}

export interface DatabaseStatusResponse {
  current_mode: string
  database_info: DatabaseInfo
  sync_status?: SyncStatus
  is_connected: boolean
}

// 数据库模式参数类型
export interface LocalModeParams {
  path: string
}

export interface RemoteModeParams {
  url: string
  auth_token: string
}

export interface EmbeddedReplicaModeParams {
  local_path: string
  remote_url: string
  auth_token: string
  sync_interval_seconds?: number
  read_your_writes?: boolean
}

export type InMemoryModeParams = {}

export type ModeParams = 
  | { type: 'local'; params: LocalModeParams }
  | { type: 'remote'; params: RemoteModeParams }
  | { type: 'embedded_replica'; params: EmbeddedReplicaModeParams }
  | { type: 'in_memory'; params: InMemoryModeParams }

export interface SwitchModeRequest {
  mode: string
  params: ModeParams['params']
}

// 数据库模式选项
export interface DatabaseModeOption {
  value: string
  label: string
  description: string
  icon: string
  supported: boolean
}

// 配置管理相关类型
export interface DatabaseConfig {
  current_mode: string
  auto_backup_enabled: boolean
  backup_interval_hours: number
  max_backup_files: number
  sync_enabled: boolean
  sync_interval_seconds: number
}

// 操作结果类型
export interface DatabaseOperationResult {
  success: boolean
  message: string
  data?: any
  restart_required?: boolean
}

// 连接测试结果
export interface ConnectionTestResult {
  connected: boolean
  latency_ms?: number
  error?: string
}

// 同步统计
export interface SyncStats {
  total_records: number
  synced_records: number
  pending_records: number
  failed_records: number
  last_sync_time: number
  is_online: boolean
}

// Legacy兼容性类型
export interface LegacySyncConfig {
  remote_url: string
  auth_token: string
  sync_mode: 'OFFLINE' | 'MANUAL' | 'AUTO'
  sync_interval: number
  auto_sync_enabled: boolean
  is_online: boolean
  updated_at?: number
} 
import { invoke } from '@tauri-apps/api/core'
import { showToast } from './notification'

/**
 * 带统一错误提示的安全 invoke 调用
 */
export async function invokeSafe<T = any>(command: string, args: Record<string, any> = {}): Promise<T> {
  try {
    return await invoke<T>(command, args)
  } catch (err: any) {
    console.error('Invoke error:', err)
    const message = err?.message ?? 'Unknown error'
    showToast(message, 'error')
    throw err
  }
} 
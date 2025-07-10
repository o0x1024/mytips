import { ToastType, useNotificationStore } from '../stores/notificationStore'

/**
 * 触发一个 Toast 通知
 * @param message 文本内容
 * @param type 通知类型，默认为 info
 * @param duration 显示时长（毫秒），默认 3s
 */
export function showToast(message: string, type: ToastType = 'info', duration = 3000) {
  const store = useNotificationStore()
  store.addToast(message, type, duration)
} 
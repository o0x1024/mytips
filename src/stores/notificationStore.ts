import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ToastType = 'success' | 'error' | 'info' | 'warning'

export interface Toast {
  id: string
  message: string
  type: ToastType
  duration: number
}

export const useNotificationStore = defineStore('notification', () => {
  // 所有待显示的 toast
  const toasts = ref<Toast[]>([])

  /**
   * 新增一条 Toast，并在 duration 后自动移除
   */
  function addToast(message: string, type: ToastType = 'info', duration = 3000) {
    const id = `${Date.now()}_${Math.random().toString(36).slice(2, 8)}`
    const toast: Toast = { id, message, type, duration }
    toasts.value.push(toast)

    // 自动移除
    setTimeout(() => removeToast(id), duration)
  }

  /** 手动移除 */
  function removeToast(id: string) {
    const idx = toasts.value.findIndex(t => t.id === id)
    if (idx !== -1) {
      toasts.value.splice(idx, 1)
    }
  }

  return { toasts, addToast, removeToast }
}) 
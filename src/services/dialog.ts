import { createApp, type App } from 'vue'
import CommonDialog from '../components/CommonDialog.vue'

interface DialogOptions {
  title?: string
  confirmText?: string
  cancelText?: string
}

interface AlertOptions {
  title?: string
  confirmText?: string
}

class DialogService {
  private app: App | null = null
  private container: HTMLElement | null = null

  /**
   * 显示确认对话框
   */
  async confirm(message: string, options: DialogOptions = {}): Promise<boolean> {
    return new Promise((resolve) => {
      this.createDialog({
        type: 'confirm',
        message,
        title: options.title,
        confirmText: options.confirmText || '确定',
        cancelText: options.cancelText || '取消',
        onConfirm: () => resolve(true),
        onCancel: () => resolve(false)
      })
    })
  }

  /**
   * 显示提示对话框
   */
  async alert(message: string, options: AlertOptions = {}): Promise<void> {
    return new Promise((resolve) => {
      this.createDialog({
        type: 'alert',
        message,
        title: options.title,
        confirmText: options.confirmText || '确定',
        onConfirm: () => resolve(),
        onCancel: () => resolve()
      })
    })
  }

  /**
   * 创建对话框实例
   */
  private createDialog(config: {
    type: 'confirm' | 'alert'
    message: string
    title?: string
    confirmText?: string
    cancelText?: string
    onConfirm: () => void
    onCancel: () => void
  }) {
    // 创建容器
    this.container = document.createElement('div')
    document.body.appendChild(this.container)

    // 创建Vue应用实例
    this.app = createApp(CommonDialog, {
      type: config.type,
      message: config.message,
      title: config.title,
      confirmText: config.confirmText,
      cancelText: config.cancelText,
      onConfirm: () => {
        config.onConfirm()
        this.cleanup()
      },
      onCancel: () => {
        config.onCancel()
        this.cleanup()
      }
    })

    // 挂载组件
    const instance = this.app.mount(this.container)
    
    // 显示对话框
    setTimeout(() => {
      (instance as any).show()
    }, 10)
  }

  /**
   * 清理资源
   */
  private cleanup() {
    if (this.app) {
      this.app.unmount()
      this.app = null
    }
    if (this.container && this.container.parentNode) {
      this.container.parentNode.removeChild(this.container)
      this.container = null
    }
  }
}

// 创建全局实例
const dialogService = new DialogService()

// 导出便捷方法
export const showConfirm = (message: string, options?: DialogOptions) => 
  dialogService.confirm(message, options)

export const showAlert = (message: string, options?: AlertOptions) => 
  dialogService.alert(message, options)

export default dialogService 
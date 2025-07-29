import { createApp, type App } from 'vue'
import CommonDialog from '../components/CommonDialog.vue'
import i18n from '../i18n' // 导入i18n实例

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
        confirmText: options.confirmText || (i18n.global as any).t('common.confirm'),
        cancelText: options.cancelText || (i18n.global as any).t('common.cancel'),
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
        confirmText: options.confirmText || (i18n.global as any).t('common.confirm'),
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
      visible: true,
      title: config.title,
      content: config.message,
      confirmText: config.confirmText,
      cancelText: config.cancelText,
      showCancel: config.type === 'confirm',
      onConfirm: () => {
        config.onConfirm()
        this.cleanup()
      },
      onCancel: () => {
        config.onCancel()
        this.cleanup()
      },
      onClose: () => {
        config.onCancel()
        this.cleanup()
      }
    })

    this.app.use(i18n)
    // 挂载组件
    this.app.mount(this.container)
    
    // 不需要调用show方法，因为我们已经设置了visible为true
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

export const showMessage = (message: string, options?: AlertOptions) =>
  dialogService.alert(message, { title: options?.title || (i18n.global as any).t('common.tip'), ...options })

export default dialogService 
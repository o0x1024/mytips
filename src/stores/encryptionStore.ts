import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface EncryptionStatus {
  noteId?: string
  notebookId?: string
  isEncrypted: boolean
  isUnlocked: boolean
}

export const useEncryptionStore = defineStore('encryption', () => {
  // 状态
  const unlockedItems = ref<Set<string>>(new Set()) // 已解锁的笔记/笔记本ID
  const encryptionStatuses = ref<Map<string, EncryptionStatus>>(new Map())
  const currentPassword = ref<string>('')
  const isLoading = ref(false)
  const unlockedPasswords = ref<Map<string, string>>(new Map()) // 存储已解锁项目的密码

  // 检查项目是否已解锁
  function isItemUnlocked(id: string): boolean {
    return unlockedItems.value.has(id)
  }

  // 检查项目是否加密
  function isItemEncrypted(id: string): boolean {
    const status = encryptionStatuses.value.get(id)
    return status?.isEncrypted || false
  }

  // 解锁项目
  function unlockItem(id: string, password?: string) {
    unlockedItems.value.add(id)
    if (password) {
      unlockedPasswords.value.set(id, password)
    }
  }

  // 锁定项目
  function lockItem(id: string) {
    unlockedItems.value.delete(id)
    unlockedPasswords.value.delete(id)
  }

  // 锁定所有项目
  async function lockAllItems() {
    unlockedItems.value.clear()
    unlockedPasswords.value.clear()
    currentPassword.value = ''
    
    // 同时清除后端数据库中的解锁状态
    try {
      await invoke<boolean>('clear_session_unlocks')
      console.log('已清除前端和后端的所有解锁状态')
    } catch (error) {
      console.error('清除后端解锁状态失败:', error)
    }
  }

  // 设置项目加密状态
  function setEncryptionStatus(id: string, status: EncryptionStatus) {
    encryptionStatuses.value.set(id, status)
  }

  // 获取已解锁笔记的解密内容
  async function getUnlockedNoteContent(noteId: string): Promise<string | null> {
    try {
      console.log('正在获取已解锁笔记的解密内容，noteId:', noteId)
      
      const password = unlockedPasswords.value.get(noteId)
      if (!password) {
        console.error('未找到笔记的密码，noteId:', noteId)
        return null
      }
      
      console.log('找到密码，正在调用后端API...')
      
      const content = await invoke<string>('get_unlocked_note_content', { 
        noteId, 
        password 
      })
      
      console.log('后端API调用成功，内容长度:', content.length)
      return content
    } catch (error) {
      console.error('获取解密内容失败:', error)
      // 提供更详细的错误信息
      if (typeof error === 'string') {
        console.error('错误详情:', error)
      }
      return null
    }
  }

  // 验证密码并解锁笔记
  async function unlockNote(noteId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('unlock_note', { noteId, password })
      if (result) {
        unlockItem(noteId, password)
        currentPassword.value = password
      }
      return result
    } catch (error) {
      console.error('解锁笔记失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 验证密码并解锁笔记本
  async function unlockNotebook(notebookId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('unlock_notebook', { notebookId, password })
      if (result) {
        unlockItem(notebookId, password)
        currentPassword.value = password
      }
      return result
    } catch (error) {
      console.error('解锁笔记本失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 加密笔记
  async function encryptNote(noteId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('encrypt_note', { noteId, password })
      if (result) {
        setEncryptionStatus(noteId, { noteId, isEncrypted: true, isUnlocked: false })
        lockItem(noteId)
      }
      return result
    } catch (error) {
      console.error('加密笔记失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 加密笔记本
  async function encryptNotebook(notebookId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('encrypt_notebook', { notebookId, password })
      if (result) {
        setEncryptionStatus(notebookId, { notebookId, isEncrypted: true, isUnlocked: false })
        lockItem(notebookId)
      }
      return result
    } catch (error) {
      console.error('加密笔记本失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 解密笔记
  async function decryptNote(noteId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('decrypt_note', { noteId, password })
      if (result) {
        setEncryptionStatus(noteId, { noteId, isEncrypted: false, isUnlocked: true })
        unlockItem(noteId)
      }
      return result
    } catch (error) {
      console.error('解密笔记失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 解密笔记本
  async function decryptNotebook(notebookId: string, password: string): Promise<boolean> {
    try {
      isLoading.value = true
      const result = await invoke<boolean>('decrypt_notebook', { notebookId, password })
      if (result) {
        setEncryptionStatus(notebookId, { notebookId, isEncrypted: false, isUnlocked: true })
        unlockItem(notebookId)
      }
      return result
    } catch (error) {
      console.error('解密笔记本失败:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 获取加密项目列表
  async function fetchEncryptionStatuses(): Promise<void> {
    try {
      isLoading.value = true
      const statuses = await invoke<EncryptionStatus[]>('get_encryption_statuses')
      
      // 清空现有状态
      encryptionStatuses.value.clear()
      
      // 重新构建加密状态映射
      statuses.forEach(status => {
        const id = status.noteId || status.notebookId
        if (id) {
          encryptionStatuses.value.set(id, status)
          
          // 重要：确保会话状态同步
          // 如果数据库中显示为已解锁，但前端状态中未解锁，则清理前端状态
          if (status.isUnlocked) {
            // 数据库显示已解锁，但这可能是因为应用重启前的遗留状态
            // 由于应用启动时会清理数据库中的解锁状态，这里也需要同步清理前端状态
            if (unlockedItems.value.has(id)) {
              lockItem(id) // 清理前端的解锁状态
            }
          }
        }
      })
      
      console.log(`加密状态加载完成：共 ${encryptionStatuses.value.size} 个项目`)
    } catch (error) {
      console.error('获取加密状态失败:', error)
    } finally {
      isLoading.value = false
    }
  }

  // 初始化方法：在应用启动时调用，确保状态正确
  function initialize() {
    // 清理所有会话级别的前端状态
    lockAllItems()
    console.log('加密存储已初始化，清理了所有会话状态')
  }

  return {
    // 状态
    unlockedItems,
    encryptionStatuses,
    currentPassword,
    isLoading,
    unlockedPasswords,

    // 方法
    isItemUnlocked,
    isItemEncrypted,
    unlockItem,
    lockItem,
    lockAllItems,
    setEncryptionStatus,
    getUnlockedNoteContent,
    unlockNote,
    unlockNotebook,
    encryptNote,
    encryptNotebook,
    decryptNote,
    decryptNotebook,
    fetchEncryptionStatuses,
    initialize
  }
}) 
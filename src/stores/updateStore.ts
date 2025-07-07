import { ref } from 'vue'
import { defineStore } from 'pinia'

export interface UpdateInfo {
  version: string
  pub_date: string
  body: string
  available: boolean
}

export const useUpdateStore = defineStore('update', () => {
  // 状态
  const hasUpdate = ref(false)
  const updateInfo = ref<UpdateInfo | null>(null)
  const isChecking = ref(false)
  const lastCheckTime = ref<Date | null>(null)
  const currentVersion = ref<string>('')
  
  // 设置
  const autoCheck = ref(true)
  const checkInterval = ref(3600000) // 1小时
  
  // 操作
  function setUpdateInfo(info: UpdateInfo | null) {
    updateInfo.value = info
    hasUpdate.value = info?.available || false
    lastCheckTime.value = new Date()
  }
  
  function setCurrentVersion(version: string) {
    currentVersion.value = version
  }
  
  function setChecking(checking: boolean) {
    isChecking.value = checking
  }
  
  function setAutoCheck(enabled: boolean, interval?: number) {
    autoCheck.value = enabled
    if (interval) {
      checkInterval.value = interval
    }
  }
  
  function clearUpdate() {
    hasUpdate.value = false
    updateInfo.value = null
  }
  
  return {
    // 状态
    hasUpdate,
    updateInfo,
    isChecking,
    lastCheckTime,
    currentVersion,
    autoCheck,
    checkInterval,
    
    // 操作
    setUpdateInfo,
    setCurrentVersion,
    setChecking,
    setAutoCheck,
    clearUpdate
  }
}) 
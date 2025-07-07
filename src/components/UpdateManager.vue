<template>
  <!-- 不显示任何UI，作为后台服务运行 -->
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useUpdateStore } from '../stores/updateStore'

const updateStore = useUpdateStore()

let checkTimer: number | null = null
let unlistenProgress: (() => void) | null = null
let unlistenInstalling: (() => void) | null = null
let unlistenCompleted: (() => void) | null = null
let unlistenManualUpdate: (() => void) | null = null

// 启动更新服务
onMounted(async () => {
  // 监听后端更新事件
  setupEventListeners()
  
  // 获取当前版本信息
  await getCurrentVersion()
  
  // 启动后自动检查更新
  setTimeout(() => {
    checkForUpdatesQuiet()
  }, 5000) // 5秒后检查
  
  // 设置定时检查
  startPeriodicCheck()
})

onUnmounted(() => {
  // 清理事件监听器和定时器
  if (unlistenProgress) unlistenProgress()
  if (unlistenInstalling) unlistenInstalling()
  if (unlistenCompleted) unlistenCompleted()
  if (unlistenManualUpdate) unlistenManualUpdate()
  if (checkTimer) clearInterval(checkTimer)
})

// 设置事件监听器
async function setupEventListeners() {
  try {
    // 监听下载进度
    unlistenProgress = await listen('update-progress', (event) => {
      console.log(`更新下载进度: ${event.payload}%`)
    })
    
    // 监听安装开始
    unlistenInstalling = await listen('update-installing', () => {
      console.log('开始安装更新')
    })
    
    // 监听更新完成
    unlistenCompleted = await listen('update-completed', () => {
      console.log('更新安装完成')
    })
    
    // 监听需要手动更新的事件
    unlistenManualUpdate = await listen('manual_update_required', async () => {
      console.log('收到手动更新请求')
      try {
        const confirmed = await invoke('show_confirm_dialog', {
          message: '由于签名验证失败，无法自动更新。这可能是由于网络问题或配置错误导致的。您想前往发布页面手动下载最新版本吗？',
          title: '需要手动更新'
        }) as boolean
        
        if (confirmed) {
          await invoke('open_url', {
            url: 'https://github.com/o0x1024/mytips/releases/latest'
          })
        }
      } catch (error) {
        console.error('处理手动更新请求失败:', error)
      }
    })
  } catch (error) {
    console.error('设置事件监听器失败:', error)
  }
}

// 开始定期检查
function startPeriodicCheck() {
  if (checkTimer) clearInterval(checkTimer)
  
  if (updateStore.autoCheck) {
    checkTimer = window.setInterval(() => {
      if (updateStore.autoCheck) {
        checkForUpdatesQuiet()
      }
    }, updateStore.checkInterval)
  }
}

// 获取当前版本
async function getCurrentVersion() {
  try {
    const version = await invoke('get_current_version') as string
    updateStore.setCurrentVersion(version)
  } catch (error) {
    console.error('获取当前版本失败:', error)
  }
}

// 静默检查更新
async function checkForUpdatesQuiet() {
  if (updateStore.isChecking) return
  
  updateStore.setChecking(true)
  
  try {
    const updateResult = await invoke('check_for_updates_with_config', {
      timeoutSeconds: 30,
      proxy: null
    }) as any
    
    if (updateResult.available) {
      updateStore.setUpdateInfo({
        version: updateResult.version,
        pub_date: updateResult.pub_date || '',
        body: updateResult.body || '',
        available: true
      })
      console.log(`发现新版本: ${updateResult.version}`)
    } else {
      updateStore.setUpdateInfo(null)
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setUpdateInfo(null)
  } finally {
    updateStore.setChecking(false)
  }
}

// 暴露给其他组件的方法
defineExpose({
  checkForUpdatesQuiet,
  startPeriodicCheck
})
</script> 
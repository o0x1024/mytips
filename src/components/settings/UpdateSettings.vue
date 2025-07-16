<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">更新设置</h2>
      
      <!-- 当前版本信息 -->
      <div class="mb-6">
        <h3 class="text-lg font-medium mb-2">版本信息</h3>
        <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
          <div>
            <p class="font-medium">当前版本: {{ updateStore.currentVersion || '获取中...' }}</p>
            <p class="text-sm text-base-content/80 mt-1">
              上次检查: {{ formatLastCheckTime() }}
            </p>
          </div>
          <div v-if="updateStore.hasUpdate" class="badge badge-error">
            有新版本
          </div>
          <div v-else class="badge badge-success">
            已是最新
          </div>
        </div>
      </div>

      <!-- 新版本信息 -->
      <div v-if="updateStore.hasUpdate && updateStore.updateInfo" class="mb-6">
        <h3 class="text-lg font-medium mb-2">新版本可用</h3>
        <div class="p-4 bg-accent/10 border border-accent/20 rounded-lg">
          <p class="font-medium text-accent mb-2">
            版本 {{ updateStore.updateInfo.version }}
          </p>
          <p class="text-sm text-base-content/80 mb-2">
            发布时间: {{ formatDate(updateStore.updateInfo.pub_date) }}
          </p>
          <div v-if="updateStore.updateInfo.body" class="text-sm bg-base-200 p-3 rounded max-h-32 overflow-y-auto">
            <div v-html="formatReleaseNotes(updateStore.updateInfo.body)"></div>
          </div>
          <button 
            class="btn btn-accent btn-sm mt-3" 
            @click="showUpdateDialog = true"
          >
            立即更新
          </button>
        </div>
      </div>

      <!-- 更新设置 -->
      <div class="mb-6">
        <h3 class="text-lg font-medium mb-4">更新设置</h3>
        
        <!-- 自动检查更新 -->
        <div class="form-control mb-4">
          <label class="label cursor-pointer">
            <span class="label-text">自动检查更新</span>
            <input 
              type="checkbox" 
              class="checkbox checkbox-primary" 
              v-model="updateStore.autoCheck"
              @change="updateAutoCheckSetting"
            />
          </label>
          <div class="label">
            <span class="label-text-alt">启用后，应用会定期检查新版本</span>
          </div>
        </div>

        <!-- 检查间隔 -->
        <div v-if="updateStore.autoCheck" class="form-control mb-4">
          <label class="label">
            <span class="label-text">检查间隔</span>
          </label>
          <select 
            class="select select-bordered w-full max-w-xs" 
            v-model="checkIntervalHours"
            @change="updateCheckInterval"
          >
            <option value="1">每小时</option>
            <option value="6">每6小时</option>
            <option value="12">每12小时</option>
            <option value="24">每天</option>
            <option value="168">每周</option>
          </select>
        </div>
      </div>

      <!-- 检查更新按钮 -->
      <div class="flex flex-wrap gap-3">
        <button 
          class="btn btn-primary" 
          @click="checkForUpdates"
          :disabled="updateStore.isChecking"
        >
          <span v-if="updateStore.isChecking" class="loading loading-spinner loading-sm mr-2"></span>
          {{ updateStore.isChecking ? '检查中...' : '检查更新' }}
        </button>
        
        <button 
          class="btn btn-outline btn-primary" 
          @click="checkForUpdatesNoSignature"
          :disabled="updateStore.isChecking"
          title="跳过签名验证检查更新（用于解决签名问题）"
        >
          <span v-if="updateStore.isChecking" class="loading loading-spinner loading-sm mr-2"></span>
          {{ updateStore.isChecking ? '检查中...' : '检查更新(无签名)' }}
        </button>
        
        <button 
          v-if="updateStore.hasUpdate" 
          class="btn btn-accent" 
          @click="showUpdateDialog = true"
        >
          立即更新
        </button>

        <button 
          class="btn btn-outline btn-info" 
          @click="testWindowsUpdate"
          :disabled="isTestingWindowsUpdate"
        >
          <span v-if="isTestingWindowsUpdate" class="loading loading-spinner loading-sm mr-2"></span>
          {{ isTestingWindowsUpdate ? '测试中...' : 'Windows更新测试' }}
        </button>

        <button 
          class="btn btn-outline btn-secondary" 
          @click="showPlatformInfo"
        >
          平台信息
        </button>

        <button 
          class="btn btn-outline btn-warning" 
          @click="testWindowsUpdateNoSignature"
          :disabled="isTestingWindowsUpdateNoSig"
          title="测试无签名验证的Windows更新（用于网络问题诊断）"
        >
          <span v-if="isTestingWindowsUpdateNoSig" class="loading loading-spinner loading-sm mr-2"></span>
          {{ isTestingWindowsUpdateNoSig ? '测试中...' : '网络诊断测试' }}
        </button>
      </div>
    </div>
  </div>

  <!-- 更新对话框 -->
  <UpdateDialog v-model="showUpdateDialog" />
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { showMessage } from '../../services/dialog'
import { useUpdateStore } from '../../stores/updateStore'
import UpdateDialog from '../UpdateDialog.vue'

const updateStore = useUpdateStore()

const checkIntervalHours = ref(1)
const isTestingWindowsUpdate = ref(false)
const isTestingWindowsUpdateNoSig = ref(false)
const showUpdateDialog = ref(false)

// 格式化时间相关函数
function formatLastCheckTime(): string {
  if (!updateStore.lastCheckTime) return '从未检查'
  const now = new Date()
  const diff = now.getTime() - updateStore.lastCheckTime.getTime()
  const minutes = Math.floor(diff / (1000 * 60))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  
  const days = Math.floor(hours / 24)
  return `${days}天前`
}

function formatDate(dateString?: string): string {
  if (!dateString) return '未知'
  
  try {
    const date = new Date(dateString)
    
    if (isNaN(date.getTime())) {
      console.warn('无效的日期格式:', dateString)
      return dateString
    }
    
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    })
  } catch (error) {
    console.error('日期格式化错误:', error, '原始日期:', dateString)
    return dateString
  }
}

function formatReleaseNotes(notes: string): string {
  // 简单的 Markdown 格式化
  return notes
    .replace(/\n/g, '<br>')
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
    .replace(/#{1,6}\s*(.*)/g, '<strong>$1</strong>')
}

// 更新检查相关函数
async function checkForUpdates(): Promise<void> {
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
      showMessage(`发现新版本 ${updateResult.version}！`, { title: '更新检查' })
    } else {
      updateStore.setUpdateInfo(null)
      showMessage('当前已是最新版本！', { title: '更新检查' })
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setUpdateInfo(null)
    showMessage('检查更新失败: ' + error, { title: '错误' })
  } finally {
    updateStore.setChecking(false)
  }
}

async function checkForUpdatesNoSignature(): Promise<void> {
  if (updateStore.isChecking) return
  
  updateStore.setChecking(true)
  
  try {
    const updateResult = await invoke('check_for_updates_no_signature', {
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
      showMessage(`发现新版本 ${updateResult.version}！（无签名验证模式）`, { title: '更新检查' })
    } else {
      updateStore.setUpdateInfo(null)
      showMessage('当前已是最新版本！（无签名验证模式）', { title: '更新检查' })
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setUpdateInfo(null)
    showMessage('检查更新失败: ' + error, { title: '错误' })
  } finally {
    updateStore.setChecking(false)
  }
}

function updateAutoCheckSetting(): void {
  console.log('自动检查设置更新:', updateStore.autoCheck)
}

function updateCheckInterval(): void {
  const hours = parseInt(checkIntervalHours.value.toString())
  const milliseconds = hours * 60 * 60 * 1000
  updateStore.setAutoCheck(updateStore.autoCheck, milliseconds)
  console.log('检查间隔更新:', hours, '小时')
}

// 测试函数
async function testWindowsUpdate(): Promise<void> {
  if (isTestingWindowsUpdate.value) return
  
  isTestingWindowsUpdate.value = true
  
  try {
    const result = await invoke('test_windows_update_with_proxy') as string
    showMessage(result, { title: 'Windows 更新测试' })
  } catch (error) {
    console.error('Windows 更新测试失败:', error)
    showMessage('Windows 更新测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingWindowsUpdate.value = false
  }
}

async function testWindowsUpdateNoSignature(): Promise<void> {
  if (isTestingWindowsUpdateNoSig.value) return
  
  isTestingWindowsUpdateNoSig.value = true
  
  try {
    const result = await invoke('test_windows_update_no_signature') as string
    showMessage(result, { title: '网络诊断测试' })
  } catch (error) {
    console.error('网络诊断测试失败:', error)
    showMessage('网络诊断测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingWindowsUpdateNoSig.value = false
  }
}

async function showPlatformInfo(): Promise<void> {
  try {
    const platformInfo = await invoke('get_platform_info') as string
    showMessage(platformInfo, { title: '平台信息' })
  } catch (error) {
    console.error('获取平台信息失败:', error)
    showMessage('获取平台信息失败: ' + error, { title: '错误' })
  }
}

// 组件挂载时初始化
onMounted(async () => {
  // 初始化更新设置
  checkIntervalHours.value = Math.floor(updateStore.checkInterval / (1000 * 60 * 60))
  
  // 获取当前版本
  try {
    const version = await invoke('get_current_version') as string
    updateStore.setCurrentVersion(version)
  } catch (error) {
    console.error('获取当前版本失败:', error)
  }
})
</script> 
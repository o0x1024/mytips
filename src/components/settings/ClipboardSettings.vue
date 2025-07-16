<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">临时笔记设置</h2>

      <div class="divider">内容捕获设置</div>

      <!-- 启用剪贴板监听 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">启用剪贴板监听</span>
          <input type="checkbox" class="toggle toggle-primary"
            v-model="clipboardSettings.enableMonitoring" @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          启用后，系统将自动监听并捕获剪贴板变化。关闭后，只能通过快捷键手动添加内容
        </p>
      </div>

      <!-- 应用白名单设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">启用应用白名单</span>
          <input type="checkbox" class="toggle toggle-primary"
            v-model="clipboardSettings.enableAppWhitelist" @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          启用后，来自白名单应用的剪贴板内容将不会被添加到临时笔记区
        </p>
      </div>

      <!-- 捕获图片设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">捕获图片内容</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureImages"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          启用后，临时笔记区将同时捕获剪贴板中的图片内容
        </p>
      </div>

      <!-- 捕获来源信息设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">记录来源信息</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureSourceInfo"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          启用后，将记录剪贴板内容的来源应用或窗口名称
        </p>
      </div>

      <div class="divider">应用白名单管理</div>

      <!-- 白名单应用管理 -->
      <div v-if="clipboardSettings.enableAppWhitelist" class="form-control mb-4">
        <label class="label">
          <span class="label-text">白名单应用管理</span>
        </label>
        
        <!-- 添加新应用 -->
        <div class="flex gap-2 mb-3">
          <input 
            type="text" 
            v-model="newWhitelistApp" 
            placeholder="输入应用名称（如：chrome.exe, notepad.exe）" 
            class="input input-bordered flex-1"
            @keydown.enter="addWhitelistApp"
          />
          <button 
            class="btn btn-primary btn-sm" 
            @click="addWhitelistApp"
            :disabled="!newWhitelistApp.trim()"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            添加
          </button>
        </div>

        <!-- 预设应用快速添加 -->
        <div class="mb-3">
          <p class="text-sm text-base-content/80 mb-2">常用应用：</p>
          <div class="flex flex-wrap gap-2">
            <button 
              v-for="preset in presetWhitelistApps" 
              :key="preset.name"
              class="btn btn-xs btn-outline"
              @click="addPresetApp(preset.name)"
              :disabled="clipboardSettings.whitelistApps.includes(preset.name)"
            >
              {{ preset.label }}
            </button>
          </div>
        </div>

        <!-- 白名单应用列表 -->
        <div class="bg-base-200 rounded-lg p-3 max-h-48 overflow-y-auto">
          <div v-if="clipboardSettings.whitelistApps.length === 0" class="text-center py-4 text-base-content/60">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mx-auto mb-2 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <p class="text-sm">暂无白名单应用</p>
          </div>
          
          <div v-else class="space-y-2">
            <div 
              v-for="(app, index) in clipboardSettings.whitelistApps" 
              :key="index"
              class="flex items-center justify-between p-2 bg-base-100 rounded border border-base-300"
            >
              <div class="flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-base-content/70" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                <span class="text-sm font-mono">{{ app }}</span>
              </div>
              <button 
                class="btn btn-xs btn-error btn-outline"
                @click="removeWhitelistApp(index)"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <div class="mt-2 text-xs text-base-content/60">
          <p>💡 提示：</p>
          <ul class="list-disc list-inside ml-2 space-y-1">
            <li>Windows: 使用进程名称（如 chrome.exe, notepad.exe）</li>
            <li>macOS: 使用应用名称（如 Google Chrome, TextEdit）</li>
            <li>Linux: 使用进程名称（如 chrome, gedit）</li>
          </ul>
        </div>
      </div>

      <!-- 白名单功能未启用时的提示 -->
      <div v-else class="alert alert-info">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span>请先启用应用白名单功能，然后管理白名单应用列表</span>
      </div>

      <div class="divider">数据保留设置</div>

      <!-- 保留时间设置 -->
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">临时笔记保留时间</span>
        </label>
        <select v-model="clipboardSettings.retentionDays" class="select select-bordered w-full"
          @change="updateClipboardSettings">
          <option v-for="option in uiStore.clipboardRetentionOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <p class="text-xs text-base-content/80 mt-1">
          设置临时笔记区自动清理数据的时间，选择"永久保留"则不会自动清理
        </p>

        <button class="btn btn-outline btn-sm mt-2" @click="cleanExpiredEntries" :disabled="isCleaningEntries">
          <span v-if="isCleaningEntries">
            <span class="loading loading-spinner loading-xs mr-2"></span>
            清理中...
          </span>
          <span v-else>
            <div class="flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              立即清理过期内容
            </div>
          </span>
        </button>
      </div>

      <!-- 加密存储设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">加密存储临时笔记</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.encryptStorage"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          启用后，临时笔记将使用加密方式存储在数据库中
        </p>
      </div>

      <div class="divider">快捷键设置</div>

      <!-- 全局快捷键设置 -->
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">选中文本添加到临时笔记区</span>
        </label>
        <div class="flex items-center gap-2">
          <div class="join">
            <button class="btn join-item" 
              :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('meta')}"
              @click="toggleShortcutModifier('meta')">
              {{ isMac ? '⌘ Command' : 'Ctrl' }}
            </button>
            <button class="btn join-item" 
              :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('shift')}"
              @click="toggleShortcutModifier('shift')">
              Shift
            </button>
            <button class="btn join-item" 
              :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('alt')}"
              @click="toggleShortcutModifier('alt')">
              {{ isMac ? 'Option' : 'Alt' }}
            </button>
          </div>
          <span class="mx-2">+</span>
          <select v-model="clipboardSettings.shortcut.key" class="select select-bordered" @change="updateClipboardSettings">
            <option v-for="key in shortcutKeys" :key="key.value" :value="key.value">{{ key.label }}</option>
          </select>
        </div>
        <p class="text-xs text-base-content/80 mt-2">
          选择修饰键和按键组合，用于将选中文本快速添加到临时笔记区
        </p>
        <div class="alert alert-warning mt-3" v-if="isShortcutInvalid">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
          <span>快捷键必须至少包含一个修饰键和一个字符键</span>
        </div>
        <button class="btn btn-sm btn-outline mt-3" @click="applyShortcutChanges" :disabled="isShortcutInvalid || isApplyingShortcut">
          <span v-if="isApplyingShortcut">
            <span class="loading loading-spinner loading-xs mr-2"></span>
            应用中...
          </span>
          <span v-else>应用快捷键更改</span>
        </button>
        <div class="alert alert-info mt-3">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          <span>修改快捷键后需要点击"应用快捷键更改"并重启应用才能生效</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useUIStore } from '../../stores/uiStore'
import { showMessage } from '../../services/dialog'

const uiStore = useUIStore()

// 剪贴板设置
const clipboardSettings = ref({
  enableMonitoring: true,
  enableAppWhitelist: false,
  whitelistApps: [] as string[],
  ignoreSensitiveContent: false,
  captureImages: false,
  captureSourceInfo: false,
  retentionDays: 30,
  encryptStorage: false,
  shortcut: {
    modifiers: ['meta', 'shift'],
    key: 'c'
  }
})

// 白名单相关变量
const newWhitelistApp = ref('')
const isCleaningEntries = ref(false)
const isApplyingShortcut = ref(false)

// 预设的常用应用
const presetWhitelistApps = [
  // Windows
  { name: 'chrome.exe', label: 'Chrome' },
  { name: 'firefox.exe', label: 'Firefox' },
  { name: 'edge.exe', label: 'Edge' },
  { name: 'notepad.exe', label: '记事本' },
  { name: 'winword.exe', label: 'Word' },
  { name: 'excel.exe', label: 'Excel' },
  { name: 'powerpnt.exe', label: 'PowerPoint' },
  { name: 'code.exe', label: 'VS Code' },
  { name: 'devenv.exe', label: 'Visual Studio' },
  { name: 'slack.exe', label: 'Slack' },
  { name: 'discord.exe', label: 'Discord' },
  { name: 'teams.exe', label: 'Teams' },
  // macOS
  { name: 'Google Chrome', label: 'Chrome (Mac)' },
  { name: 'Firefox', label: 'Firefox (Mac)' },
  { name: 'Safari', label: 'Safari' },
  { name: 'TextEdit', label: 'TextEdit' },
  { name: 'Microsoft Word', label: 'Word (Mac)' },
  { name: 'Microsoft Excel', label: 'Excel (Mac)' },
  { name: 'Visual Studio Code', label: 'VS Code (Mac)' },
  { name: 'Slack', label: 'Slack (Mac)' },
  { name: 'Discord', label: 'Discord (Mac)' },
  // Linux
  { name: 'chrome', label: 'Chrome (Linux)' },
  { name: 'firefox', label: 'Firefox (Linux)' },
  { name: 'gedit', label: 'Gedit' },
  { name: 'code', label: 'VS Code (Linux)' },
  { name: 'slack', label: 'Slack (Linux)' }
]

// 快捷键相关
const shortcutKeys = [
  { value: 'a', label: 'A' },
  { value: 'b', label: 'B' },
  { value: 'c', label: 'C' },
  { value: 'd', label: 'D' },
  { value: 'e', label: 'E' },
  { value: 'f', label: 'F' },
  { value: 'g', label: 'G' },
  { value: 'h', label: 'H' },
  { value: 'i', label: 'I' },
  { value: 'j', label: 'J' },
  { value: 'k', label: 'K' },
  { value: 'l', label: 'L' },
  { value: 'm', label: 'M' },
  { value: 'n', label: 'N' },
  { value: 'o', label: 'O' },
  { value: 'p', label: 'P' },
  { value: 'q', label: 'Q' },
  { value: 'r', label: 'R' },
  { value: 's', label: 'S' },
  { value: 't', label: 'T' },
  { value: 'u', label: 'U' },
  { value: 'v', label: 'V' },
  { value: 'w', label: 'W' },
  { value: 'x', label: 'X' },
  { value: 'y', label: 'Y' },
  { value: 'z', label: 'Z' },
  { value: '1', label: '1' },
  { value: '2', label: '2' },
  { value: '3', label: '3' },
  { value: '4', label: '4' },
  { value: '5', label: '5' },
  { value: '6', label: '6' },
  { value: '7', label: '7' },
  { value: '8', label: '8' },
  { value: '9', label: '9' },
  { value: '0', label: '0' }
]

// 判断当前快捷键是否有效
const isShortcutInvalid = computed(() => {
  return clipboardSettings.value.shortcut.modifiers.length === 0 || 
         !clipboardSettings.value.shortcut.key
})

// 判断是否为Mac系统
const isMac = computed(() => {
  return navigator.platform.toUpperCase().indexOf('MAC') >= 0
})

// 初始化时合并用户设置
if (uiStore.settings.clipboard) {
  Object.assign(clipboardSettings.value, uiStore.settings.clipboard)
}

// 同步剪贴板设置到后端
async function syncClipboardSettingsToBackend() {
  try {
    const backendSettings = {
      ignore_sensitive_content: clipboardSettings.value.ignoreSensitiveContent,
      capture_images: clipboardSettings.value.captureImages,
      capture_source_info: clipboardSettings.value.captureSourceInfo,
      retention_days: clipboardSettings.value.retentionDays,
      encrypt_storage: clipboardSettings.value.encryptStorage,
      enable_monitoring: clipboardSettings.value.enableMonitoring,
      enable_app_whitelist: clipboardSettings.value.enableAppWhitelist,
      whitelist_apps: clipboardSettings.value.whitelistApps
    }

    await invoke('save_clipboard_settings', { settings: backendSettings })
    console.log('剪贴板设置已同步到后端')
    
    if (clipboardSettings.value.enableMonitoring) {
      await invoke('start_clipboard_monitoring')
    } else {
      await invoke('stop_clipboard_monitoring')
    }
    
  } catch (error) {
    console.error('同步剪贴板设置失败:', error)
  }
}

// 从后端加载剪贴板设置
async function loadClipboardSettingsFromBackend() {
  try {
    const backendSettings = await invoke<any>('get_clipboard_settings')
    if (backendSettings) {
      clipboardSettings.value = {
        ignoreSensitiveContent: backendSettings.ignore_sensitive_content,
        captureImages: backendSettings.capture_images,
        captureSourceInfo: backendSettings.capture_source_info,
        retentionDays: backendSettings.retention_days,
        encryptStorage: backendSettings.encrypt_storage,
        enableMonitoring: backendSettings.enable_monitoring !== undefined 
                        ? backendSettings.enable_monitoring 
                        : true,
        enableAppWhitelist: backendSettings.enable_app_whitelist || false,
        whitelistApps: backendSettings.whitelist_apps || [],
        shortcut: clipboardSettings.value.shortcut
      }

      updateClipboardSettings()
    }
    
    const shortcutConfig = await invoke<any>('get_global_shortcut_config')
    if (shortcutConfig) {
      clipboardSettings.value.shortcut = {
        modifiers: shortcutConfig.modifiers || ['meta', 'shift'],
        key: shortcutConfig.key || 'c'
      }
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

// 更新剪贴板设置
function updateClipboardSettings() {
  uiStore.setClipboardSettings(clipboardSettings.value)
  syncClipboardSettingsToBackend()
}

// 切换修饰键
function toggleShortcutModifier(modifier: string) {
  const modifiers = clipboardSettings.value.shortcut.modifiers
  const index = modifiers.indexOf(modifier)
  
  if (index === -1) {
    modifiers.push(modifier)
  } else {
    if (modifiers.length > 1) {
      modifiers.splice(index, 1)
    }
  }
}

// 应用快捷键更改
async function applyShortcutChanges() {
  if (isShortcutInvalid.value) return
  
  isApplyingShortcut.value = true
  try {
    const shortcutConfig = {
      modifiers: clipboardSettings.value.shortcut.modifiers,
      key: clipboardSettings.value.shortcut.key
    }
    
    await invoke('update_global_shortcut', { config: shortcutConfig })
    showMessage('快捷键已更新，重启应用后生效', { title: '成功' })
  } catch (error) {
    console.error('更新快捷键失败:', error)
    showMessage('更新快捷键失败: ' + error, { title: '错误' })
  } finally {
    isApplyingShortcut.value = false
  }
}

// 清理过期条目
async function cleanExpiredEntries() {
  isCleaningEntries.value = true
  try {
    await invoke('clean_expired_clipboard_entries')
    showMessage('过期剪贴板条目已清理', { title: '清理成功' })
  } catch (error) {
    console.error('清理过期剪贴板条目失败:', error)
    showMessage('清理过期剪贴板条目失败: ' + error, { title: '错误' })
  } finally {
    isCleaningEntries.value = false
  }
}

// 白名单应用管理方法
function addWhitelistApp() {
  const appName = newWhitelistApp.value.trim()
  if (!appName) return
  
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    showMessage(`应用 "${appName}" 已在白名单中`, { title: '提示' })
    return
  }
  
  clipboardSettings.value.whitelistApps.push(appName)
  newWhitelistApp.value = ''
  updateClipboardSettings()
  
  showMessage(`已添加 "${appName}" 到白名单`, { title: '成功' })
}

function removeWhitelistApp(index: number) {
  const appName = clipboardSettings.value.whitelistApps[index]
  clipboardSettings.value.whitelistApps.splice(index, 1)
  updateClipboardSettings()
  
  showMessage(`已从白名单中移除 "${appName}"`, { title: '成功' })
}

function addPresetApp(appName: string) {
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    return
  }
  
  clipboardSettings.value.whitelistApps.push(appName)
  updateClipboardSettings()
  
  showMessage(`已添加 "${appName}" 到白名单`, { title: '成功' })
}

// 组件挂载时加载设置
onMounted(async () => {
  await loadClipboardSettingsFromBackend()
})
</script> 
<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('clipboardSettings.title') }}</h2>

      <div class="divider">{{ $t('clipboardSettings.contentCapture.title') }}</div>

      <!-- 启用剪贴板监听 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('clipboardSettings.contentCapture.enableMonitoring') }}</span>
          <input type="checkbox" class="toggle toggle-primary"
            v-model="clipboardSettings.enableMonitoring" @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.contentCapture.enableMonitoringDescription') }}
        </p>
      </div>

      <!-- 应用白名单设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('clipboardSettings.contentCapture.enableAppWhitelist') }}</span>
          <input type="checkbox" class="toggle toggle-primary"
            v-model="clipboardSettings.enableAppWhitelist" @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.contentCapture.enableAppWhitelistDescription') }}
        </p>
      </div>

      <!-- 捕获图片设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('clipboardSettings.contentCapture.captureImages') }}</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureImages"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.contentCapture.captureImagesDescription') }}
        </p>
      </div>

      <!-- 捕获来源信息设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('clipboardSettings.contentCapture.captureSourceInfo') }}</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureSourceInfo"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.contentCapture.captureSourceInfoDescription') }}
        </p>
      </div>

      <div class="divider">{{ $t('clipboardSettings.whitelist.title') }}</div>

      <!-- 白名单应用管理 -->
      <div v-if="clipboardSettings.enableAppWhitelist" class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('clipboardSettings.whitelist.title') }}</span>
        </label>
        
        <!-- 添加新应用 -->
        <div class="flex gap-2 mb-3">
          <input 
            type="text" 
            v-model="newWhitelistApp" 
            :placeholder="$t('clipboardSettings.whitelist.inputPlaceholder')" 
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
            {{ $t('clipboardSettings.whitelist.add') }}
          </button>
        </div>

        <!-- 预设应用快速添加 -->
        <div class="mb-3">
          <p class="text-sm text-base-content/80 mb-2">{{ $t('clipboardSettings.whitelist.presetApps') }}</p>
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
            <p class="text-sm">{{ $t('clipboardSettings.whitelist.noApps') }}</p>
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
          <p>{{ $t('clipboardSettings.whitelist.tipTitle') }}</p>
          <ul class="list-disc list-inside ml-2 space-y-1">
            <li>{{ $t('clipboardSettings.whitelist.tipWindows') }}</li>
            <li>{{ $t('clipboardSettings.whitelist.tipMacos') }}</li>
            <li>{{ $t('clipboardSettings.whitelist.tipLinux') }}</li>
          </ul>
        </div>
      </div>

      <!-- 白名单功能未启用时的提示 -->
      <div v-else class="alert alert-info">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span>{{ $t('clipboardSettings.whitelist.disabledInfo') }}</span>
      </div>

      <div class="divider">{{ $t('clipboardSettings.dataRetention.title') }}</div>

      <!-- 保留时间设置 -->
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('clipboardSettings.dataRetention.retentionTime') }}</span>
        </label>
        <select v-model="clipboardSettings.retentionDays" class="select select-bordered w-full"
          @change="updateClipboardSettings">
          <option v-for="option in uiStore.clipboardRetentionOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.dataRetention.retentionTimeDescription') }}
        </p>

        <button class="btn btn-outline btn-sm mt-2" @click="cleanExpiredEntries" :disabled="isCleaningEntries">
          <span v-if="isCleaningEntries">
            <span class="loading loading-spinner loading-xs mr-2"></span>
            {{ $t('clipboardSettings.dataRetention.cleaning') }}
          </span>
          <span v-else>
            <div class="flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              {{ $t('clipboardSettings.dataRetention.cleanNow') }}
            </div>
          </span>
        </button>
      </div>

      <!-- 加密存储设置 -->
      <div class="form-control mb-4">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('clipboardSettings.dataRetention.encryptStorage') }}</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.encryptStorage"
            @change="updateClipboardSettings" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('clipboardSettings.dataRetention.encryptStorageDescription') }}
        </p>
      </div>

      <div class="divider">{{ $t('clipboardSettings.shortcuts.title') }}</div>

      <!-- 全局快捷键设置 -->
      <div class="form-control mb-4">
        <label class="label">
          <span class="label-text">{{ $t('clipboardSettings.shortcuts.addSelectedText') }}</span>
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
          {{ $t('clipboardSettings.shortcuts.description') }}
        </p>
        <div class="alert alert-warning mt-3" v-if="isShortcutInvalid">
          <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
          <span>{{ $t('clipboardSettings.shortcuts.invalidWarning') }}</span>
        </div>
        <button class="btn btn-sm btn-outline mt-3" @click="applyShortcutChanges" :disabled="isShortcutInvalid || isApplyingShortcut">
          <span v-if="isApplyingShortcut">
            <span class="loading loading-spinner loading-xs mr-2"></span>
            {{ $t('clipboardSettings.shortcuts.applying') }}
          </span>
          <span v-else>{{ $t('clipboardSettings.shortcuts.applyChanges') }}</span>
        </button>
        <div class="alert alert-info mt-3">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          <span>{{ $t('clipboardSettings.shortcuts.restartNote') }}</span>
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
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
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
  { name: 'chrome.exe', label: t('clipboardSettings.presets.chrome') },
  { name: 'firefox.exe', label: t('clipboardSettings.presets.firefox') },
  { name: 'edge.exe', label: t('clipboardSettings.presets.edge') },
  { name: 'notepad.exe', label: t('clipboardSettings.presets.notepad') },
  { name: 'winword.exe', label: t('clipboardSettings.presets.word') },
  { name: 'excel.exe', label: t('clipboardSettings.presets.excel') },
  { name: 'powerpnt.exe', label: t('clipboardSettings.presets.powerpoint') },
  { name: 'code.exe', label: t('clipboardSettings.presets.vscode') },
  { name: 'devenv.exe', label: t('clipboardSettings.presets.visualstudio') },
  { name: 'slack.exe', label: t('clipboardSettings.presets.slack') },
  { name: 'discord.exe', label: t('clipboardSettings.presets.discord') },
  { name: 'teams.exe', label: t('clipboardSettings.presets.teams') },
  // macOS
  { name: 'Google Chrome', label: t('clipboardSettings.presets.chromeMac') },
  { name: 'Firefox', label: t('clipboardSettings.presets.firefoxMac') },
  { name: 'Safari', label: t('clipboardSettings.presets.safari') },
  { name: 'TextEdit', label: t('clipboardSettings.presets.textedit') },
  { name: 'Microsoft Word', label: t('clipboardSettings.presets.wordMac') },
  { name: 'Microsoft Excel', label: t('clipboardSettings.presets.excelMac') },
  { name: 'Visual Studio Code', label: t('clipboardSettings.presets.vscodeMac') },
  { name: 'Slack', label: t('clipboardSettings.presets.slackMac') },
  { name: 'Discord', label: t('clipboardSettings.presets.discordMac') },
  // Linux
  { name: 'chrome', label: t('clipboardSettings.presets.chromeLinux') },
  { name: 'firefox', label: t('clipboardSettings.presets.firefoxLinux') },
  { name: 'gedit', label: t('clipboardSettings.presets.gedit') },
  { name: 'code', label: t('clipboardSettings.presets.vscodeLinux') },
  { name: 'slack', label: t('clipboardSettings.presets.slackLinux') }
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
    showMessage(t('clipboardSettings.notifications.shortcutUpdated'), { title: t('clipboardSettings.notifications.success') })
  } catch (error) {
    console.error('更新快捷键失败:', error)
    showMessage(t('clipboardSettings.notifications.shortcutUpdateFailed', { error }), { title: t('clipboardSettings.notifications.error') })
  } finally {
    isApplyingShortcut.value = false
  }
}

// 清理过期条目
async function cleanExpiredEntries() {
  isCleaningEntries.value = true
  try {
    await invoke('clean_expired_clipboard_entries')
    showMessage(t('clipboardSettings.notifications.cleanupSuccess'), { title: t('clipboardSettings.notifications.success') })
  } catch (error) {
    console.error('清理过期剪贴板条目失败:', error)
    showMessage(t('clipboardSettings.notifications.cleanupFailed', { error }), { title: t('clipboardSettings.notifications.error') })
  } finally {
    isCleaningEntries.value = false
  }
}

// 白名单应用管理方法
function addWhitelistApp() {
  const appName = newWhitelistApp.value.trim()
  if (!appName) return
  
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    showMessage(t('clipboardSettings.notifications.appExists', { appName }), { title: t('clipboardSettings.notifications.tip') })
    return
  }
  
  clipboardSettings.value.whitelistApps.push(appName)
  newWhitelistApp.value = ''
  updateClipboardSettings()
  
  showMessage(t('clipboardSettings.notifications.appAdded', { appName }), { title: t('clipboardSettings.notifications.success') })
}

function removeWhitelistApp(index: number) {
  const appName = clipboardSettings.value.whitelistApps[index]
  clipboardSettings.value.whitelistApps.splice(index, 1)
  updateClipboardSettings()
  
  showMessage(t('clipboardSettings.notifications.appRemoved', { appName }), { title: t('clipboardSettings.notifications.success') })
}

function addPresetApp(appName: string) {
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    return
  }
  
  clipboardSettings.value.whitelistApps.push(appName)
  updateClipboardSettings()
  
  showMessage(t('clipboardSettings.notifications.appAdded', { appName }), { title: t('clipboardSettings.notifications.success') })
}

// 组件挂载时加载设置
onMounted(async () => {
  await loadClipboardSettingsFromBackend()
})
</script> 
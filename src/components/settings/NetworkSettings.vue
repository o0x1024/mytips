<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('networkSettings.title') }}</h2>

      <!-- 代理设置 -->
      <div class="form-control">
        <div class="flex items-center mb-4">
          <label class="label cursor-pointer">
            <span class="label-text text-base mr-4">{{ $t('networkSettings.proxy.enable') }}</span>
            <input type="checkbox" v-model="proxySettings.enabled" class="toggle toggle-primary" />
          </label>
        </div>

        <div v-if="proxySettings.enabled" class="grid gap-4">
          <!-- 代理协议 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">{{ $t('networkSettings.proxy.protocol') }}</span>
            </label>
            <select v-model="proxySettings.protocol" class="select select-bordered w-full">
              <option value="http">{{ $t('networkSettings.proxy.http') }}</option>
              <option value="socks5">{{ $t('networkSettings.proxy.socks5') }}</option>
            </select>
          </div>

          <!-- 代理地址和端口 -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('networkSettings.proxy.host') }}</span>
              </label>
              <input type="text" v-model="proxySettings.host" :placeholder="$t('networkSettings.proxy.hostPlaceholder')"
                class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('networkSettings.proxy.port') }}</span>
              </label>
              <input type="number" v-model="proxySettings.port" :placeholder="$t('networkSettings.proxy.portPlaceholder')"
                class="input input-bordered" />
            </div>
          </div>

          <!-- 测试按钮 -->
          <div class="form-control mt-2">
            <button class="btn btn-outline btn-primary" @click="testProxyConnection" :disabled="isTestingProxy">
              <span v-if="isTestingProxy">
                <span class="loading loading-spinner loading-xs mr-2"></span>
                {{ $t('networkSettings.proxy.testing') }}
              </span>
              <span v-else>{{ $t('networkSettings.proxy.testConnection') }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { showMessage } from '../../services/dialog'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// 代理设置接口
interface ProxySettings {
  enabled: boolean
  protocol: string
  host: string
  port: number
}

const proxySettings = ref<ProxySettings>({
  enabled: false,
  protocol: 'http',
  host: '127.0.0.1',
  port: 10809,
})

const isTestingProxy = ref(false)

// 保存代理设置
async function saveProxySettings() {
  try {
    await invoke('save_proxy_settings', { proxySettings: proxySettings.value })
  } catch (error) {
    console.error('保存代理设置失败:', error)
    showMessage(t('networkSettings.notifications.saveFailed', { error }), { title: t('networkSettings.notifications.error') })
  }
}

// 测试代理连接
async function testProxyConnection() {
  isTestingProxy.value = true

  try {
    // 先保存设置
    await saveProxySettings()

    // 测试连接
    const result = await invoke<string>('test_proxy_connection', {
      proxySettings: proxySettings.value
    })

    showMessage(result, { title: t('networkSettings.notifications.connectionTest') })
  } catch (error) {
    console.error('代理测试失败:', error)
    showMessage(t('networkSettings.notifications.testFailed', { error }), { title: t('networkSettings.notifications.error') })
  } finally {
    isTestingProxy.value = false
  }
}

// 加载代理设置
async function loadProxySettings() {
  try {
    const settings = await invoke<ProxySettings>('get_proxy_settings')
    if (settings) {
      proxySettings.value = settings
    }
  } catch (error) {
    console.error('获取代理设置失败:', error)
  }
}

// 监听代理设置变化
watch(proxySettings, async () => {
  await saveProxySettings()
}, { deep: true })

// 组件挂载时加载设置
onMounted(async () => {
  await loadProxySettings()
})
</script> 
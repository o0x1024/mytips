<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">网络设置</h2>

      <!-- 代理设置 -->
      <div class="form-control">
        <div class="flex items-center mb-4">
          <label class="label cursor-pointer">
            <span class="label-text text-base mr-4">启用代理</span>
            <input type="checkbox" v-model="proxySettings.enabled" class="toggle toggle-primary" />
          </label>
        </div>

        <div v-if="proxySettings.enabled" class="grid gap-4">
          <!-- 代理类型 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">代理类型</span>
            </label>
            <select v-model="proxySettings.type" class="select select-bordered w-full">
              <option value="http">HTTP</option>
              <option value="socks5">SOCKS5</option>
            </select>
          </div>

          <!-- 代理地址和端口 -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">主机</span>
              </label>
              <input type="text" v-model="proxySettings.host" placeholder="例如：127.0.0.1"
                class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label">
                <span class="label-text">端口</span>
              </label>
              <input type="number" v-model="proxySettings.port" placeholder="例如：7890"
                class="input input-bordered" />
            </div>
          </div>

          <!-- 代理认证 -->
          <div class="form-control">
            <div class="flex items-center">
              <label class="label cursor-pointer">
                <span class="label-text mr-4">需要认证</span>
                <input type="checkbox" v-model="proxySettings.auth" class="checkbox checkbox-primary" />
              </label>
            </div>
          </div>

          <!-- 认证信息 -->
          <div v-if="proxySettings.auth" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">用户名</span>
              </label>
              <input type="text" v-model="proxySettings.username" class="input input-bordered" />
            </div>
            <div class="form-control">
              <label class="label">
                <span class="label-text">密码</span>
              </label>
              <input type="password" v-model="proxySettings.password" class="input input-bordered" />
            </div>
          </div>

          <!-- 测试按钮 -->
          <div class="form-control mt-2">
            <button class="btn btn-outline btn-primary" @click="testProxyConnection" :disabled="isTestingProxy">
              <span v-if="isTestingProxy">
                <span class="loading loading-spinner loading-xs mr-2"></span>
                测试中...
              </span>
              <span v-else>测试连接</span>
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

// 代理设置接口
interface ProxySettings {
  enabled: boolean
  type: string
  host: string
  port: number
  auth: boolean
  username: string
  password: string
}

const proxySettings = ref<ProxySettings>({
  enabled: false,
  type: 'http',
  host: '127.0.0.1',
  port: 10809,
  auth: false,
  username: '',
  password: ''
})

const isTestingProxy = ref(false)

// 保存代理设置
async function saveProxySettings() {
  try {
    await invoke('save_proxy_settings', { proxySettings: proxySettings.value })
  } catch (error) {
    console.error('保存代理设置失败:', error)
    showMessage('保存代理设置失败: ' + error, { title: '错误' })
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

    showMessage(result, { title: '连接测试' })
  } catch (error) {
    console.error('代理测试失败:', error)
    showMessage('代理测试失败: ' + error, { title: '错误' })
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
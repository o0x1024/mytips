<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('appSettings.title') }}</h2>

      <div class="form-control">
        <label class="label cursor-pointer justify-start gap-4">
          <span class="label-text">{{ $t('appSettings.autoStart') }}</span>
          <input type="checkbox" class="toggle toggle-primary" v-model="autoStartEnabled"
            @change="toggleAutoStart" />
        </label>
        <p class="text-xs text-base-content/80 mt-1">
          {{ $t('appSettings.autoStartDescription') }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'

const autoStartEnabled = ref(false)

// 切换自动启动
async function toggleAutoStart() {
  if (autoStartEnabled.value) {
    autoStartEnabled.value = true
    await enable()
  } else {
    autoStartEnabled.value = false
    await disable()
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  try {
    const enabled = await isEnabled()
    autoStartEnabled.value = enabled
  } catch (error) {
    console.error('Failed to check auto-start status:', error)
  }
})
</script> 
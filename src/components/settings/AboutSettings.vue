<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">{{ $t('aboutSettings.title') }}</h2>
      <p>MyTips v{{ currentVersion || '0.2.0' }}</p>
      <p class="text-sm text-base-content/80 mt-2">
        {{ $t('aboutSettings.description') }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const currentVersion = ref('')

onMounted(async () => {
  try {
    const version = await invoke('get_current_version') as string
    currentVersion.value = version
  } catch (error) {
    console.error('Failed to get current version:', error)
  }
})
</script> 
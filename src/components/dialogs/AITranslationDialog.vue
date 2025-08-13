<template>
  <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[1000] ai-dialog-overlay" @click.self="closeDialog">
    <div class="bg-base-100 rounded-lg shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col ai-dialog-content">
      <div class="p-4 border-b border-base-300 flex items-center justify-between">
        <h3 class="font-bold text-lg">{{ t('aiTranslationDialog.title') }}</h3>
        <button class="btn btn-sm btn-ghost btn-square" @click="closeDialog">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
        </button>
      </div>

      <div class="p-6 flex-1 overflow-y-auto relative">
        <div v-if="(!content || content.length === 0) && loading" class="flex items-center justify-center h-full">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
        <div v-if="content && content.length > 0" class="prose max-w-none" v-html="content"></div>
        <div v-if="loading && content && content.length > 0" class="absolute top-2 right-2 opacity-70 text-xs">
          <span class="loading loading-spinner loading-sm mr-1"></span>
          Streaming...
        </div>
      </div>

      <div class="p-4 border-t border-base-300 flex justify-end gap-2">
        <button class="btn btn-sm" @click="emit('copy')" :disabled="loading">{{ t('common.copy') }}</button>
        <button class="btn btn-sm btn-primary" @click="emit('insert')" :disabled="loading">{{ t('aiTranslationDialog.insertIntoNote') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, watch, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  visible: Boolean,
  loading: Boolean,
  content: String,
})

const emit = defineEmits(['close', 'copy', 'insert'])
const { t } = useI18n()

const closeDialog = () => {
  emit('close')
}

// Enter 触发插入
function handleKeydown(e: KeyboardEvent) {
  if (e.key !== 'Enter') return
  if (!props.visible || props.loading) return
  if ((e.target as HTMLElement)?.closest('textarea, [contenteditable="true"], input')) return
  e.preventDefault()
  emit('insert')
}

watch(() => props.visible, (v) => {
  if (v) {
    window.addEventListener('keydown', handleKeydown)
  } else {
    window.removeEventListener('keydown', handleKeydown)
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.prose {
  max-width: none;
}

:deep(pre) {
  white-space: pre-wrap;      /* keep newlines but allow wrapping */
  word-wrap: break-word;      /* legacy compatibility */
  overflow-wrap: anywhere;    /* modern wrapping for long words */
  max-width: 100%;
}

</style> 
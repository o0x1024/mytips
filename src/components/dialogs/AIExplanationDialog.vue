<template>
  <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[1000] ai-dialog-overlay" @click.self="closeDialog">
    <div class="bg-base-100 rounded-lg shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col ai-dialog-content">
      <h3 class="font-bold text-lg mb-2">{{ t('aiExplanationDialog.title') }}</h3>
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
      <div class="mt-4 flex justify-end gap-2">
        <button class="btn btn-sm" @click="copy">{{ t('common.copy') }}</button>
        <button class="btn btn-sm" @click="insert">{{ t('aiExplanationDialog.insertIntoNote') }}</button>
        <button class="btn btn-sm btn-error" @click="closeDialog">{{ t('common.close') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import { useI18n } from 'vue-i18n'


defineProps({
  visible: {
    type: Boolean,
    required: true,
  },
  loading: {
    type: Boolean,
    required: true,
  },
  content: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['close', 'copy', 'insert'])
const { t } = useI18n()

const closeDialog = () => {
  emit('close')
}




const copy = () => emit('copy');
const insert = () => emit('insert');
</script>

<style scoped>
.prose {
  max-width: none;
}
</style> 
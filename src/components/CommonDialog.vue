<template>
  <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[1000]" @click.self="close">
    <div class="bg-base-100 rounded-lg shadow-xl p-6 w-full max-w-lg">
      <h3 class="font-bold text-lg mb-4">{{ title }}</h3>
      <div class="prose max-w-none max-h-[60vh] overflow-y-auto" v-html="renderedContent"></div>
      <div class="mt-6 flex justify-end gap-2">
        <button v-if="showCancel" class="btn" @click="cancel">{{ cancelText || t('common.cancel') }}</button>
        <button class="btn btn-primary" @click="confirm">{{ confirmText || t('common.confirm') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { renderInlineMarkdown } from '../services/markdownService'

const props = defineProps({
  visible: Boolean,
  title: String,
  content: String,
  confirmText: String,
  cancelText: String,
  showCancel: {
    type: Boolean,
    default: true,
  },
})

const emit = defineEmits(['confirm', 'cancel', 'close'])
const { t } = useI18n()
const renderedContent = ref('')

watchEffect(async () => {
  if (props.content) {
    renderedContent.value = await renderInlineMarkdown(props.content)
  }
})

const confirm = () => emit('confirm')
const cancel = () => emit('cancel')
const close = () => emit('close')
</script>

<style scoped>
/* 确保对话框在最顶层 */
.modal {
  z-index: 9999;
}

/* 消息内容样式 */
.modal-box p {
  line-height: 1.6;
  word-wrap: break-word;
}

/* 按钮间距 */
.modal-action {
  gap: 0.5rem;
}

:deep(.prose code) {
  background-color: rgba(125, 125, 125, 0.15);
  padding: 0.1em 0.3em;
  border-radius: 6px;
  color: #f87171; /* A bright color for visibility in both light and dark modes */
}

:deep(.prose pre) {
  background-color: #f4f4f4;
  padding: 1em;
  border-radius: 6px;
  overflow-x: auto;
}

[data-theme="dark"] :deep(.prose pre),
[data-theme="night"] :deep(.prose pre) {
   background-color: #2d2d2d;
}

[data-theme="dark"] :deep(.prose code),
[data-theme="night"] :deep(.prose code) {
  color: #f87171; /* More vibrant red for dark themes */
  background-color: rgba(255, 255, 255, 0.1);
}
</style> 
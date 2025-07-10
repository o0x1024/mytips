<template>
  <div class="modal" :class="{ 'modal-open': visible }">
    <div :class="['modal-box', 'prose', type === 'confirm' ? 'max-w-md' : '']">
      <h3 v-if="title" class="font-bold text-lg">{{ title }}</h3>
      <div class="py-4" v-html="renderedMessage"></div>
      <div class="modal-action">
        <button v-if="type === 'confirm'" class="btn" @click="onCancel">{{ cancelText }}</button>
        <button class="btn btn-primary" @click="onConfirm">{{ confirmText }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import DOMPurify from 'dompurify'
import Prism from 'prismjs'

interface Props {
  type?: 'confirm' | 'alert'
  title?: string
  message: string
  confirmText?: string
  cancelText?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'alert',
  title: '',
  confirmText: '确定',
  cancelText: '取消'
})

const emit = defineEmits(['confirm', 'cancel'])

const visible = ref(false)

const show = () => {
  visible.value = true
}

const hide = () => {
  visible.value = false
}

defineExpose({ show, hide })

const onConfirm = () => {
  emit('confirm')
  hide()
}

const onCancel = () => {
  emit('cancel')
  hide()
}

const renderedMessage = computed(() => {
  try {
    const marked = new Marked(
      markedHighlight({
        langPrefix: 'language-',
        highlight(code, lang) {
          const language = Prism.languages[lang] || Prism.languages.plaintext;
          return Prism.highlight(code, language, lang);
        }
      })
    );
    const dirty = marked.parse(props.message) as string;
    return DOMPurify.sanitize(dirty);
  } catch (e) {
    console.error("Markdown parsing error:", e);
    return props.message;
  }
});
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
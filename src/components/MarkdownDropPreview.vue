<template>
  <!-- 只有在拖拽或预览时才渲染 -->
  <div
    v-if="isDragging || showPreview"
    class="fixed inset-0 z-[9999] flex items-center justify-center"
    :class="{ 'pointer-events-none': showPreview }"
    @dragenter.prevent
    @dragover.prevent
    @dragleave.prevent
    @drop.prevent
    @drop="handleFileDrop"
  >
    <!-- 半透明背景，点击无效 -->
    <div v-if="isDragging && !showPreview" class="absolute inset-0 bg-base-300/60 backdrop-blur-sm"></div>

    <!-- 拖拽提示 -->
    <div
      v-if="isDragging && !showPreview"
      class="relative border-2 border-dashed border-primary bg-primary/10 rounded-lg p-8 text-center text-base-content max-w-lg w-full mx-4"
    >
      <p>{{ t('markdownDropPreview.dragAndDrop') }}</p>
    </div>

    <!-- 预览卡片 -->
    <div v-if="showPreview" class="relative p-6 border rounded-lg bg-base-200 text-left max-w-2xl w-full mx-4 shadow-xl">
      <h3 class="font-bold mb-2">
        {{ t('markdownDropPreview.preview') }}: {{ previewFile?.name }}
      </h3>
      <div class="prose max-w-none max-h-60 overflow-y-auto" v-html="renderedHtml"></div>
      <div class="mt-4">
        <label class="label">
          <span class="label-text">{{ t('markdownDropPreview.importTitle') }}</span>
        </label>
        <input v-model="importTitle" type="text" class="input input-bordered w-full" />
      </div>
      <div class="mt-4 flex justify-end gap-2">
        <button class="btn btn-sm" @click="cancelImport">{{ t('common.cancel') }}</button>
        <button class="btn btn-sm btn-primary" @click="confirmImport" :disabled="!importTitle.trim()">
          {{ t('common.import') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { renderInlineMarkdown } from '../services/markdownService'

const { t } = useI18n()
const emit = defineEmits(['import'])

const isDragging = ref(false)
// Counter 解决嵌套 dragenter / dragleave 导致的闪烁
let dragCounter = 0

const previewFile = ref<{ name: string; path: string } | null>(null)
const markdownContent = ref('')
const renderedHtml = ref('')
const showPreview = ref(false)
const importTitle = ref('')

watch(markdownContent, async (newContent) => {
  if (newContent) {
    renderedHtml.value = await renderInlineMarkdown(newContent)
  } else {
    renderedHtml.value = ''
  }
})

// -------- 全局拖拽监听 --------
function handleWindowDragEnter(e: DragEvent) {
  dragCounter++
  if (e.dataTransfer?.types.includes('Files')) {
    isDragging.value = true
  }
}

function handleWindowDragLeave() {
  dragCounter = Math.max(0, dragCounter - 1)
  if (dragCounter === 0) {
    isDragging.value = false
  }
}

function handleWindowDrop() {
  dragCounter = 0
  isDragging.value = false
}

onMounted(() => {
  window.addEventListener('dragenter', handleWindowDragEnter)
  window.addEventListener('dragover', handleWindowDragEnter)
  window.addEventListener('dragleave', handleWindowDragLeave)
  window.addEventListener('drop', handleWindowDrop)
})

onBeforeUnmount(() => {
  window.removeEventListener('dragenter', handleWindowDragEnter)
  window.removeEventListener('dragover', handleWindowDragEnter)
  window.removeEventListener('dragleave', handleWindowDragLeave)
  window.removeEventListener('drop', handleWindowDrop)
})

const handleFileDrop = (event: DragEvent) => {
  isDragging.value = false
  const file = event.dataTransfer?.files[0]
  if (file && file.type === 'text/markdown') {
    const reader = new FileReader()
    reader.onload = (e) => {
      const content = e.target?.result as string
      if (content) {
        markdownContent.value = content
        previewFile.value = { name: file.name, path: (file as any).path } // 'path' is a Tauri-specific property
        importTitle.value = file.name.replace(/\.md$/, '')
        showPreview.value = true
      }
    }
    reader.readAsText(file)
  }
}

const confirmImport = () => {
  if (importTitle.value.trim() && markdownContent.value) {
    emit('import', {
      title: importTitle.value.trim(),
      content: markdownContent.value,
    })
    resetState()
  }
}

const cancelImport = () => {
  resetState()
}

const resetState = () => {
  showPreview.value = false
  previewFile.value = null
  markdownContent.value = ''
  importTitle.value = ''
}
</script>

<style scoped>
/* 基础排版 */
.prose {
  color: inherit;
  font-size: var(--base-font-size); /* 继承全局字体大小 */
}

/* 确保prose内的所有元素都能正确继承字体大小 */
.prose * {
  font-size: inherit;
}

/* 为不同标题级别设置相对大小 */
.prose h1 {
  font-size: calc(var(--base-font-size) * 2.25); /* 相当于 text-4xl */
}

.prose h2 {
  font-size: calc(var(--base-font-size) * 1.875); /* 相当于 text-3xl */
}

.prose h3 {
  font-size: calc(var(--base-font-size) * 1.5); /* 相当于 text-2xl */
}

.prose h4 {
  font-size: calc(var(--base-font-size) * 1.25); /* 相当于 text-xl */
}

.prose h5 {
  font-size: calc(var(--base-font-size) * 1.125); /* 相当于 text-lg */
}

.prose h6 {
  font-size: var(--base-font-size); /* 相当于 text-base */
}

.prose h1,
.prose h2,
.prose h3,
.prose h4,
.prose h5,
.prose h6 {
  color: inherit;
}

.prose code {
  border-radius: 0.25rem;
  padding: 0.125rem 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  /* 使用CSS变量确保在所有主题下都有合适的颜色 */
  background-color: hsl(var(--b3));
  color: hsl(var(--bc));
  border: 1px solid hsl(var(--b3));
}

/* 暗色主题下的行内代码特殊优化 */
:deep(.prose code) {
  border-radius: 0.25rem;
  padding: 0.125rem 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  background-color: hsl(var(--b3));
  color: hsl(var(--bc));
  border: 1px solid hsl(var(--b3));
}

[data-theme="dark"] :deep(.prose code),
[data-theme="night"] :deep(.prose code),
[data-theme="black"] :deep(.prose code),
[data-theme="dracula"] :deep(.prose code),
[data-theme="halloween"] :deep(.prose code),
[data-theme="business"] :deep(.prose code),
[data-theme="luxury"] :deep(.prose code),
[data-theme="coffee"] :deep(.prose code),
[data-theme="forest"] :deep(.prose code),
[data-theme="synthwave"] :deep(.prose code) {
  background-color: rgba(255, 255, 255, 0.1) !important;
  color: rgb(251, 191, 36) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
}

/* 亮色主题下的行内代码优化 */
[data-theme="light"] :deep(.prose code),
[data-theme="cupcake"] :deep(.prose code),
[data-theme="bumblebee"] :deep(.prose code),
[data-theme="emerald"] :deep(.prose code),
[data-theme="corporate"] :deep(.prose code),
[data-theme="retro"] :deep(.prose code),
[data-theme="cyberpunk"] :deep(.prose code),
[data-theme="valentine"] :deep(.prose code),
[data-theme="garden"] :deep(.prose code),
[data-theme="aqua"] :deep(.prose code),
[data-theme="lofi"] :deep(.prose code),
[data-theme="pastel"] :deep(.prose code),
[data-theme="fantasy"] :deep(.prose code),
[data-theme="wireframe"] :deep(.prose code),
[data-theme="cmyk"] :deep(.prose code),
[data-theme="autumn"] :deep(.prose code),
[data-theme="acid"] :deep(.prose code),
[data-theme="lemonade"] :deep(.prose code),
[data-theme="winter"] :deep(.prose code) {
  background-color: rgba(0, 0, 0, 0.08) !important;
  color: rgb(124, 58, 237) !important;
  border: 1px solid rgba(0, 0, 0, 0.15) !important;
}

.prose pre {
  background-color: hsl(var(--b2));
  border-radius: 0.5rem;
  padding: 1rem;
  overflow-x: auto;
}

.prose pre code {
  background-color: transparent;
  padding: 0;
}

.prose blockquote {
  border-left: 4px solid hsl(var(--p));
  padding-left: 1rem;
  margin-left: 0;
  font-style: italic;
  color: hsl(var(--bc) / 0.8);
}

.prose table {
  border-collapse: collapse;
  width: 100%;
}

.prose th,
.prose td {
  border: 1px solid hsl(var(--b3));
  padding: 0.5rem;
  text-align: left;
}

.prose th {
  background-color: hsl(var(--b2));
  font-weight: 600;
}
</style> 
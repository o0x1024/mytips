<template>
  <div class="flex-1 flex overflow-hidden relative">
    <!-- Markdown编辑器 -->
    <textarea v-if="!isPreviewMode || isSplitMode"
      class="flex-1 p-4 h-full resize-none focus:outline-none font-mono text-base overflow-auto"
      :class="{ 'w-1/2': isSplitMode }"
      placeholder="开始输入内容..."
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      @contextmenu.prevent="emit('contextmenu', $event)"
      @paste="emit('paste', $event)"
      @keydown="emit('keydown', $event)"
      @scroll="handleEditorScroll"
      @wheel="setUserScrollingPane('editor')"
      @touchstart="setUserScrollingPane('editor')"
      ref="editorTextarea"
      @blur="emit('blur')"
    ></textarea>

    <!-- Markdown预览 -->
    <div v-if="isPreviewMode || isSplitMode"
      ref="previewDiv"
      class="flex-1 p-4 overflow-auto prose dark:prose-invert max-w-none"
      :class="{ 'w-1/2': isSplitMode, 'markdown-body': true, 'markdown-preview': true }"
      v-html="renderedContent"
      @scroll="handlePreviewScroll"
      @wheel="setUserScrollingPane('preview')"
      @touchstart="setUserScrollingPane('preview')"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  renderedContent: {
    type: String,
    required: true,
  },
  isSplitMode: {
    type: Boolean,
    default: true,
  },
  isPreviewMode: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'contextmenu', 'paste', 'keydown', 'blur', 'editor-scroll', 'preview-scroll']);

const editorTextarea = ref<HTMLTextAreaElement | null>(null);
const previewDiv = ref<HTMLDivElement | null>(null);

const userScrollingPane = ref<'editor' | 'preview' | null>(null);
let scrollEndTimer: number | null = null;

const setUserScrollingPane = (pane: 'editor' | 'preview') => {
  userScrollingPane.value = pane;
  if (scrollEndTimer) {
    clearTimeout(scrollEndTimer);
  }
  scrollEndTimer = window.setTimeout(() => {
    userScrollingPane.value = null;
  }, 200);
};

const handleEditorScroll = (event: Event) => {
  if (userScrollingPane.value !== 'editor') return;

  const editor = editorTextarea.value;
  const preview = previewDiv.value;
  if (!editor || !preview || !props.isSplitMode) return;

  if (editor.scrollHeight <= editor.clientHeight) return;

  const scrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight);
  preview.scrollTop = scrollRatio * (preview.scrollHeight - preview.clientHeight);
  emit('editor-scroll', event);
};

const handlePreviewScroll = (event: Event) => {
  if (userScrollingPane.value !== 'preview') return;

  const editor = editorTextarea.value;
  const preview = previewDiv.value;
  if (!editor || !preview || !props.isSplitMode) return;

  if (preview.scrollHeight <= preview.clientHeight) return;

  const scrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight);
  editor.scrollTop = scrollRatio * (editor.scrollHeight - editor.clientHeight);
  emit('preview-scroll', event);
};


// Expose the textarea element to the parent
defineExpose({
  editorTextarea,
  previewDiv,
});
</script>


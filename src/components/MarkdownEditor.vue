<template>
  <div class="flex-1 flex overflow-hidden relative">
    <!-- Markdown编辑器 -->
    <textarea v-if="!isPreviewMode || isSplitMode"
      class="p-4 h-full resize-none focus:outline-none font-mono text-base overflow-auto"
      :class="{ 'basis-[55%]': isSplitMode, 'w-full': !isSplitMode }"
      :placeholder="t('markdownEditor.placeholder')"
      :value="modelValue"
      @input="handleInput"
      @contextmenu.prevent="emit('contextmenu', $event)"
      @paste="emit('paste', $event)"
      @keydown="handleKeyDown"
      @scroll="handleEditorScroll"
      @wheel="setUserScrollingPane('editor')"
      @touchstart="setUserScrollingPane('editor')"
      ref="editorTextarea"
      @blur="emit('blur')"
    ></textarea>

    <!-- Markdown预览 -->
    <div v-if="isPreviewMode || isSplitMode"
      ref="previewDiv"
      class="p-4 overflow-auto prose dark:prose-invert max-w-none"
      :class="{ 'basis-[45%]': isSplitMode, 'w-full': !isSplitMode, 'markdown-body': true, 'markdown-preview': true }"
      v-html="renderedContent"
      @scroll="handlePreviewScroll"
      @wheel="setUserScrollingPane('preview')"
      @touchstart="setUserScrollingPane('preview')"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

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

// --- Undo / Redo state ---
const undoStack = ref<string[]>([]);
const redoStack = ref<string[]>([]);
const maxHistory = 100;

function pushHistory(snapshot: string) {
  if (undoStack.value.length === 0 || undoStack.value[undoStack.value.length - 1] !== snapshot) {
    undoStack.value.push(snapshot);
    if (undoStack.value.length > maxHistory) undoStack.value.shift();
  }
}

function handleInput(event: Event) {
  const value = (event.target as HTMLTextAreaElement).value;
  // 新输入产生新历史，清空重做栈
  redoStack.value = [];
  pushHistory(value);
  emit('update:modelValue', value);
}

function applySnapshot(value: string) {
  const textarea = editorTextarea.value;
  if (!textarea) return;
  textarea.value = value;
  emit('update:modelValue', value);
  // 将光标放到文本末尾（简单实现）
  const end = value.length;
  textarea.setSelectionRange(end, end);
}

function undo() {
  // 当前值等于栈顶时，先弹出当前，再取上一条
  const current = editorTextarea.value?.value ?? '';
  if (undoStack.value.length === 0) return;
  if (undoStack.value[undoStack.value.length - 1] === current) {
    undoStack.value.pop();
  }
  if (undoStack.value.length === 0) return;
  const prev = undoStack.value.pop() as string;
  redoStack.value.push(current);
  applySnapshot(prev);
}

function redo() {
  if (redoStack.value.length === 0) return;
  const next = redoStack.value.pop() as string;
  // 将当前值压入撤销栈
  const current = editorTextarea.value?.value ?? '';
  pushHistory(current);
  applySnapshot(next);
}

function handleKeyDown(event: KeyboardEvent) {
  const isMac = navigator.platform.toUpperCase().includes('MAC');
  const mod = isMac ? event.metaKey : event.ctrlKey;
  if (mod && event.key.toLowerCase() === 'z') {
    event.preventDefault();
    if (event.shiftKey) {
      redo();
    } else {
      undo();
    }
    return;
  }
  if (!isMac && event.ctrlKey && event.key.toLowerCase() === 'y') {
    event.preventDefault();
    redo();
    return;
  }
  emit('keydown', event);
}

// 初始化历史：以初始值入栈
watch(
  () => props.modelValue,
  (val) => {
    // 当外部变更且与栈顶不同，更新历史栈顶
    if (typeof val === 'string' && val !== (undoStack.value[undoStack.value.length - 1] ?? undefined)) {
      pushHistory(val);
    }
  },
  { immediate: true }
);

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
  undo,
  redo,
});
</script>


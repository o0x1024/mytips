<template>
  <CodeMirrorEditor
    :model-value="modelValue"
    :rendered-content="renderedContent"
    :is-split-mode="isSplitMode"
    :is-preview-mode="isPreviewMode"

    @update:model-value="emit('update:modelValue', $event)"
    @contextmenu="emit('contextmenu', $event)"
    @paste="emit('paste', $event)"
    @keydown="emit('keydown', $event)"
    @blur="emit('blur', $event)"
    @editor-scroll="handleEditorScroll"
    @preview-scroll="handlePreviewScroll"
    @ready="onCodeMirrorReady"

    ref="codeMirrorEditor"
  />
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import CodeMirrorEditor from './CodeMirrorEditor.vue';

defineProps({
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

const emit = defineEmits(['update:modelValue', 'contextmenu', 'paste', 'keydown', 'blur', 'editor-scroll', 'preview-scroll', 'ready']);

// 处理编辑器滚动事件，确保传递所有必要信息
const handleEditorScroll = (event: any) => {
  emit('editor-scroll', event);
};

// 处理预览区域滚动事件，确保传递所有必要信息
const handlePreviewScroll = (event: any) => {
  emit('preview-scroll', event);
};

const onCodeMirrorReady = () => {
  emit('ready');
};

const codeMirrorEditor = ref<InstanceType<typeof CodeMirrorEditor> | null>(null);

// 创建兼容的 editorTextarea 对象
const editorTextarea = computed(() => {
  if (!codeMirrorEditor.value) {
    return null;
  }
  const editorView = codeMirrorEditor.value.editorView;
  if (!editorView) {
    return null;
  }
  
  // 创建一个兼容HTMLTextAreaElement接口的对象
  const scrollDOM = editorView.scrollDOM;
  return {
    // 滚动相关属性（用于滚动同步）- 使用 getter 确保动态获取
    get scrollTop() { return scrollDOM.scrollTop; },
    set scrollTop(value: number) { scrollDOM.scrollTop = value; },
    get scrollHeight() { return scrollDOM.scrollHeight; },
    get clientHeight() { return scrollDOM.clientHeight; },
    get scrollLeft() { return scrollDOM.scrollLeft; },
    set scrollLeft(value: number) { scrollDOM.scrollLeft = value; },
    get scrollWidth() { return scrollDOM.scrollWidth; },
    get clientWidth() { return scrollDOM.clientWidth; },
    
    // 文本选择相关属性（兼容textarea API）
    get selectionStart() {
      const selection = editorView.state.selection.main;
      return selection.from;
    },
    get selectionEnd() {
      const selection = editorView.state.selection.main;
      return selection.to;
    },
    
    // 文本内容属性
    get value() {
      return editorView.state.doc.toString();
    },
    set value(newValue: string) {
      editorView.dispatch({
        changes: { from: 0, to: editorView.state.doc.length, insert: newValue }
      });
    },
    
    // 文本选择方法
    setSelectionRange(start: number, end: number) {
      editorView.dispatch({
        selection: { anchor: start, head: end }
      });
      editorView.focus();
    },
    
    // 焦点方法
    focus() {
      editorView.focus();
    },
    
    // 滚动方法
    scrollTo(x: number, y: number) {
      scrollDOM.scrollTo(x, y);
    },
    
    // DOM元素引用（用于事件监听等）
    addEventListener: scrollDOM.addEventListener.bind(scrollDOM),
    removeEventListener: scrollDOM.removeEventListener.bind(scrollDOM),
    
    // 其他可能需要的属性
    get offsetHeight() { return scrollDOM.offsetHeight; },
    get offsetWidth() { return scrollDOM.offsetWidth; },
  };
});

const previewDiv = computed(() => {
  if (!codeMirrorEditor.value) return null;
  return codeMirrorEditor.value.previewDiv || null;
});

// Expose the editor instance to the parent
defineExpose({
  codeMirrorEditor,
  editorTextarea,
  previewDiv,
});
</script>


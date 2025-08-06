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
    @editor-scroll="emit('editor-scroll', $event)"
    @preview-scroll="emit('preview-scroll', $event)"

    ref="codeMirrorEditor"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';
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

const emit = defineEmits(['update:modelValue', 'contextmenu', 'paste', 'keydown', 'blur', 'editor-scroll', 'preview-scroll']);

const codeMirrorEditor = ref<InstanceType<typeof CodeMirrorEditor> | null>(null);

// Expose the editor instance to the parent
defineExpose({
  codeMirrorEditor,
});
</script>


<template>
  <div v-if="visible" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40" @click.self="close">
    <div class="bg-base-100 rounded-lg shadow-lg w-1/2 p-4">
      <h3 class="font-bold text-lg mb-2">TIP一下</h3>
      <!-- 模板选择 -->
      <div class="mb-2 flex items-center gap-2">
        <select v-model="selectedTemplate" class="select select-sm select-bordered w-48" @change="applyTemplate">
          <option value="" disabled selected>选择模板</option>
          <option v-for="tpl in templates" :key="tpl.name" :value="tpl.name">{{ tpl.name }}</option>
        </select>
        <span v-if="isLoading" class="loading loading-spinner loading-xs"></span>
      </div>
      <textarea v-model="localPrompt" class="textarea textarea-bordered w-full h-32 tip-prompt-textarea"></textarea>
      <div class="mt-2 text-xs text-right text-base-content/60">{{ localPrompt.length }} 字符</div>
      <div class="mt-4 flex justify-end gap-2">
        <button class="btn btn-sm btn-secondary" @click="reset">重置</button>
        <button class="btn btn-sm btn-primary" @click="confirm">确认</button>
        <button class="btn btn-sm btn-error" @click="close">取消</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, defineProps, defineEmits, onMounted } from 'vue';
import { useTipTemplateStore } from '../../stores/tipTemplateStore';

const props = defineProps({
  visible: Boolean,
  prompt: String,
  selectedText: String,
});

const emit = defineEmits(['close', 'confirm', 'set-template', 'reset', 'save-template']);

const localPrompt = ref(props.prompt ?? '');

// 模板 store
const { templates, loadTemplates, isLoading } = useTipTemplateStore();
const selectedTemplate = ref<string>('');

onMounted(() => {
  loadTemplates();
});

function applyTemplate() {
  const tpl = templates.value.find(t => t.name === selectedTemplate.value);
  if (tpl) {
    const text = tpl.content.replace('{{SELECTED_TEXT}}', props.selectedText || '');
    localPrompt.value = text;
  }
}

watch(() => props.prompt, (newVal) => {
  localPrompt.value = newVal ?? '';
});

const close = () => emit('close');
const confirm = () => emit('confirm', localPrompt.value);

const reset = () => emit('reset');

</script>

<style scoped>
.tip-prompt-textarea {
  transition: border-color 0.2s ease;
  resize: vertical;
  min-height: 120px;
  max-height: 300px;
}
</style> 
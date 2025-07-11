import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface TipTemplate {
  name: string;
  content: string;
}

const templates = ref<TipTemplate[]>([]);
const isLoaded = ref(false);
const isLoading = ref(false);

async function loadTemplates(force = false) {
  if (isLoaded.value && !force) return;
  try {
    isLoading.value = true;
    const list: TipTemplate[] = await invoke('get_tip_templates');
    templates.value = list;
    isLoaded.value = true;
  } catch (err) {
    console.error('加载提示词模板失败:', err);
  } finally {
    isLoading.value = false;
  }
}

async function addTemplate(name: string, content: string) {
  const tpl: TipTemplate = { name, content };
  await invoke('save_tip_template', { template: tpl });
  await loadTemplates(true);
}

async function deleteTemplate(name: string) {
  await invoke('delete_tip_template', { name });
  await loadTemplates(true);
}

export function useTipTemplateStore() {
  return {
    templates,
    isLoaded,
    isLoading,
    loadTemplates,
    addTemplate,
    deleteTemplate,
  };
} 
<template>
  <div v-if="visible" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40" @click.self="close">
    <div class="bg-base-100 rounded-lg shadow-lg w-1/2 p-4">
      <h3 class="font-bold text-lg mb-2">{{ t('aiExplanationDialog.title') }}</h3>
      <div class="prose max-h-80 overflow-y-auto">
        <div v-if="loading && !content" class="flex justify-center items-center h-24">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
        <div v-else v-html="sanitizedContent"></div>
      </div>
      <div class="mt-4 flex justify-end gap-2">
        <button class="btn btn-sm" @click="copy">{{ t('common.copy') }}</button>
        <button class="btn btn-sm" @click="insert">{{ t('aiExplanationDialog.insertIntoNote') }}</button>
        <button class="btn btn-sm btn-error" @click="close">{{ t('common.close') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import DOMPurify from 'dompurify';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const props = defineProps({
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

const emit = defineEmits(['close', 'copy', 'insert']);

const sanitizedContent = computed(() => {
  return DOMPurify.sanitize(props.content, {
    ADD_ATTR: ['target', 'class', 'href'],
    ALLOW_DATA_ATTR: true
  });
});

const close = () => emit('close');
const copy = () => emit('copy');
const insert = () => emit('insert');
</script>

<style scoped>
.prose {
  max-width: none;
}
</style> 
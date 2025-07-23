<template>
  <div class="border-t border-base-100 p-4 bg-base-200">
    <div class="flex flex-wrap w-full gap-4 items-center justify-between">
      <!-- 标签选择器组件 -->
      <div class="flex-1">
        <TagSelector 
          :model-value="tags"
          :content-text="contentText"
          :title-text="titleText"
          @update:modelValue="emit('update:tags', $event)"
          @saveNote="emit('saveNote')"
        />
      </div>

      <!-- 统计信息和状态指示器 -->
      <div class="text-xs text-base-content/80 flex items-center gap-4 shrink-0">
        <!-- 图片加载状态指示器 -->
        <div v-if="isLoadingImages" class="flex items-center gap-1 text-info" :title="$t('editorFooter.loadingImagesTitle')">
          <span class="loading loading-spinner loading-xs"></span>
          <span>{{ $t('editorFooter.loadingImages') }}</span>
        </div>
        <span :title="$t('editorFooter.wordCountTitle')">{{ $t('editorFooter.wordCount', { count: wordCount }) }}</span>
        <span :title="$t('editorFooter.createdAtTitle')">{{ $t('editorFooter.createdAt', { time: formatDateTime(createdAt) }) }}</span>
        <span :title="$t('editorFooter.updatedAtTitle')">{{ $t('editorFooter.updatedAt', { time: formatDateTime(updatedAt) }) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import TagSelector from './TagSelector.vue';
import { formatDateTime } from '../utils/formatters';

const { t } = useI18n();

interface Tag {
  id: string;
  name: string;
}

const props = defineProps({
  tags: {
    type: Array as () => Tag[],
    required: true,
  },
  contentText: {
    type: String,
    required: true,
  },
  titleText: {
    type: String,
    required: true,
  },
  createdAt: {
    type: Number,
    required: true,
  },
  updatedAt: {
    type: Number,
    required: true,
  },
  isLoadingImages: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:tags', 'saveNote']);

const wordCount = computed(() => {
  return props.contentText.length;
});
</script> 
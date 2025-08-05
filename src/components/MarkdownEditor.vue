<template>
  <div class="flex-1 flex overflow-hidden relative">
    <!-- 搜索框 -->
    <div v-if="showSearch" class="absolute top-2 right-2 z-10 bg-base-100 border border-base-300 rounded-lg p-3 shadow-lg min-w-80">
      <!-- 搜索行 -->
      <div class="flex items-center gap-2 mb-2">
        <input
          ref="searchInput"
          v-model="searchQuery"
          type="text"
          :placeholder="t('markdownEditor.searchPlaceholder')"
          class="input input-xs flex-1"
          @keydown.enter="findNext"
          @keydown.escape="closeSearch"
          @input="performSearch"
        />
        <div class="flex items-center gap-1">
          <button @click="findPrevious" class="btn btn-xs btn-ghost" :disabled="!searchResults.length" :title="t('markdownEditor.findPrevious')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"></polyline></svg>
          </button>
          <button @click="findNext" class="btn btn-xs btn-ghost" :disabled="!searchResults.length" :title="t('markdownEditor.findNext')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"></polyline></svg>
          </button>
          <span class="text-xs text-base-content/70 min-w-12">
            {{ searchResults.length > 0 ? `${currentSearchIndex + 1}/${searchResults.length}` : t('markdownEditor.noResults') }}
          </span>
          <button @click="showReplace = !showReplace" class="btn btn-xs btn-ghost" :title="t('markdownEditor.toggleReplace')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14,2 14,8 20,8"></polyline><line x1="16" y1="13" x2="8" y2="21"></line><line x1="12" y1="18" x2="12" y2="12"></line></svg>
          </button>
          <button @click="closeSearch" class="btn btn-xs btn-ghost" :title="t('markdownEditor.closeSearch')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
          </button>
        </div>
      </div>
      
      <!-- 替换行 -->
      <div v-if="showReplace" class="flex items-center gap-2">
        <input
          v-model="replaceQuery"
          type="text"
          :placeholder="t('markdownEditor.replacePlaceholder')"
          class="input input-xs flex-1"
          @keydown.enter="replaceNext"
          @keydown.escape="closeSearch"
        />
        <div class="flex items-center gap-1">
          <button @click="replaceNext" class="btn btn-xs btn-primary" :disabled="!searchResults.length" :title="t('markdownEditor.replaceNext')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path></svg>
          </button>
          <button @click="replaceAll" class="btn btn-xs btn-secondary" :disabled="!searchResults.length" :title="t('markdownEditor.replaceAll')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path><path d="M9 12l2 2 4-4"></path></svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Markdown编辑器 -->
    <textarea v-if="!isPreviewMode || isSplitMode"
      class="flex-1 p-4 h-full resize-none focus:outline-none font-mono text-base overflow-auto"
      :class="{ 'w-1/2': isSplitMode }"
      :placeholder="t('markdownEditor.placeholder')"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      @contextmenu.prevent="emit('contextmenu', $event)"
      @paste="emit('paste', $event)"
      @keydown="emit('keydown', $event)"
      @scroll="handleEditorScroll"
      @wheel="setUserScrollingPane('editor')"
      @touchstart="setUserScrollingPane('editor')"
      ref="editorTextarea"
      @blur="emit('blur')" dx
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
import { ref, nextTick, watch } from 'vue';
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
  showSearch: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'contextmenu', 'paste', 'keydown', 'blur', 'editor-scroll', 'preview-scroll', 'close-search']);

const editorTextarea = ref<HTMLTextAreaElement | null>(null);
const previewDiv = ref<HTMLDivElement | null>(null);
const searchInput = ref<HTMLInputElement | null>(null);

// 搜索相关状态
const searchQuery = ref('');
const replaceQuery = ref('');
const searchResults = ref<{start: number, end: number}[]>([]);
const currentSearchIndex = ref(-1);
const showReplace = ref(false);

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

// 搜索相关方法
const performSearch = () => {
  if (!searchQuery.value || !editorTextarea.value) {
    searchResults.value = [];
    currentSearchIndex.value = -1;
    return;
  }

  const content = props.modelValue;
  const query = searchQuery.value.toLowerCase();
  const results: {start: number, end: number}[] = [];
  
  let index = 0;
  while (index < content.length) {
    const foundIndex = content.toLowerCase().indexOf(query, index);
    if (foundIndex === -1) break;
    
    results.push({
      start: foundIndex,
      end: foundIndex + query.length
    });
    index = foundIndex + 1;
  }
  
  searchResults.value = results;
  currentSearchIndex.value = results.length > 0 ? 0 : -1;
  
  if (results.length > 0) {
    highlightSearchResult(0);
  }
};

const highlightSearchResult = (index: number) => {
  if (!editorTextarea.value || index < 0 || index >= searchResults.value.length) return;
  
  const result = searchResults.value[index];
  editorTextarea.value.focus();
  editorTextarea.value.setSelectionRange(result.start, result.end);
  
  // 滚动到选中位置
  const textarea = editorTextarea.value;
  const textBeforeSelection = props.modelValue.substring(0, result.start);
  const lines = textBeforeSelection.split('\n');
  const lineHeight = 20; // 估算行高
  const scrollTop = (lines.length - 1) * lineHeight;
  textarea.scrollTop = Math.max(0, scrollTop - textarea.clientHeight / 2);
};

const findNext = () => {
  if (searchResults.value.length === 0) return;
  
  currentSearchIndex.value = (currentSearchIndex.value + 1) % searchResults.value.length;
  highlightSearchResult(currentSearchIndex.value);
};

const findPrevious = () => {
  if (searchResults.value.length === 0) return;
  
  currentSearchIndex.value = currentSearchIndex.value <= 0 
    ? searchResults.value.length - 1 
    : currentSearchIndex.value - 1;
  highlightSearchResult(currentSearchIndex.value);
};

const closeSearch = () => {
  searchQuery.value = '';
  replaceQuery.value = '';
  searchResults.value = [];
  currentSearchIndex.value = -1;
  showReplace.value = false;
  emit('close-search');
};

// 替换当前选中的结果
const replaceNext = () => {
  if (searchResults.value.length === 0 || currentSearchIndex.value === -1) return;
  
  const result = searchResults.value[currentSearchIndex.value];
  const newContent = props.modelValue.substring(0, result.start) + 
                    replaceQuery.value + 
                    props.modelValue.substring(result.end);
  
  emit('update:modelValue', newContent);
  
  // 重新搜索以更新结果
  setTimeout(() => {
    performSearch();
  }, 50);
};

// 替换所有匹配的结果
const replaceAll = () => {
  if (searchResults.value.length === 0 || !searchQuery.value) return;
  
  const regex = new RegExp(searchQuery.value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');
  const newContent = props.modelValue.replace(regex, replaceQuery.value);
  
  emit('update:modelValue', newContent);
  
  // 重新搜索以更新结果
  setTimeout(() => {
    performSearch();
  }, 50);
};

// 监听搜索框显示状态
watch(() => props.showSearch, (newValue) => {
  if (newValue) {
    nextTick(() => {
      searchInput.value?.focus();
    });
  } else {
    searchQuery.value = '';
    searchResults.value = [];
    currentSearchIndex.value = -1;
  }
});

// Expose the textarea element to the parent
defineExpose({
  editorTextarea,
  previewDiv,
});
</script>

<style scoped>
.search-box {
  position: absolute;
  top: 10px;
  right: 10px;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 8px;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  z-index: 10;
  min-width: 300px;
}

.dark .search-box {
  background: #374151;
  border-color: #4b5563;
  color: white;
}

.search-input {
  border: 1px solid #d1d5db;
  border-radius: 4px;
  padding: 4px 8px;
  font-size: 14px;
  width: 180px;
}

.dark .search-input {
  background: #4b5563;
  border-color: #6b7280;
  color: white;
}

.search-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 1px #3b82f6;
}

.search-button {
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  padding: 4px 8px;
  margin: 0 2px;
  cursor: pointer;
  font-size: 12px;
  transition: background-color 0.2s;
}

.dark .search-button {
  background: #4b5563;
  border-color: #6b7280;
  color: white;
}

.search-button:hover {
  background: #e5e7eb;
}

.dark .search-button:hover {
  background: #6b7280;
}

.search-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.search-count {
  font-size: 12px;
  color: #6b7280;
  margin: 0 8px;
}

.dark .search-count {
  color: #9ca3af;
}

.close-button {
  background: none;
  border: none;
  font-size: 16px;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  color: #6b7280;
}

.dark .close-button {
  color: #9ca3af;
}

.close-button:hover {
  background: #f3f4f6;
  color: #374151;
}

.dark .close-button:hover {
  background: #4b5563;
  color: white;
}
</style>


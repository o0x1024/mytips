<template>
  <div class="ai-assistant-page flex flex-col h-screen">
    <!-- 顶部标题栏 -->
    <div class="page-header p-4 bg-base-200 flex items-center justify-between">
      <div class="flex items-center">
        <button class="btn btn-ghost mr-2" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <div>
          <h1 class="text-2xl font-bold">临时笔记区</h1>
          <p class="text-base-content/70">在这里管理您的临时笔记，可以合并为正式笔记或进行其他操作</p>
        </div>
      </div>
      <div class="flex gap-2">
        <button @click="mergeToNote" :disabled="selectedIds.length === 0" class="btn btn-primary btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          创建笔记
        </button>
        <button @click="deleteSelected" :disabled="selectedIds.length === 0" class="btn btn-error btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          删除选中
        </button>
        <button @click="confirmClearAll" :disabled="history.length === 0" class="btn btn-error btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          清空全部
        </button>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="flex-1 overflow-hidden flex">
      <!-- 左侧列表 -->
      <div class="w-full h-full flex flex-col p-4 overflow-hidden" :class="{ 'md:w-1/2': previewItem }">
        <!-- 搜索框 -->
        <div class="mb-4 flex gap-2">
          <div class="relative flex-1">
            <input 
              type="text" 
              v-model="searchQuery" 
              placeholder="搜索临时笔记..." 
              class="input input-bordered w-full pr-10"
              @input="handleSearch"
            />
            <button 
              v-if="searchQuery" 
              @click="clearSearch" 
              class="absolute right-3 top-1/2 transform -translate-y-1/2 text-base-content/50 hover:text-base-content"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>

        <!-- 搜索结果统计 -->
        <div v-if="searchQuery && filteredHistory.length > 0" class="text-sm text-base-content/70 mb-2">
          找到 {{ filteredHistory.length }} 条匹配结果
        </div>

        <!-- 笔记列表 -->
        <div class="flex-1 overflow-y-auto pr-2">
          <div v-if="filteredHistory.length === 0"
            class="flex flex-col items-center justify-center h-full text-base-content/50">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <p>{{ searchQuery ? '未找到匹配的临时笔记' : '临时笔记区为空' }}</p>
            <p class="text-sm mt-2">{{ searchQuery ? '请尝试其他搜索关键词' : '复制文本到剪贴板将自动添加到此处' }}</p>
          </div>
          <div v-else class="space-y-3">
            <div v-for="item in filteredHistory" :key="item.id"
              class="card bg-base-200 cursor-pointer transition-colors duration-200 hover:bg-base-300 border-l-4"
              :class="{ 'border-primary': selectedIds.includes(item.id), 'border-transparent': !selectedIds.includes(item.id) }"
              @click="toggleSelection(item.id)">
              <div class="card-body p-4">
                <div class="flex items-start gap-3">
                  <input type="checkbox" :checked="selectedIds.includes(item.id)" class="checkbox checkbox-primary mt-1"
                    @click.stop @change="toggleSelection(item.id)" />
                  <div class="flex-grow">
                    <div class="text-sm whitespace-pre-wrap"
                      :class="{ 'max-h-32 overflow-y-auto': !expandedItems.includes(item.id) }">
                      <span v-if="searchQuery" v-html="highlightText(item.content, searchQuery)"></span>
                      <span v-else-if="isImageContent(item.content)" ><img :src="item.content" alt="图片预览" /></span>
                      <span v-else>{{ item.content }}</span>
                    </div>
                    <div v-if="isContentLong(item.content)" class="text-xs mt-1">
                      <button class="text-primary hover:underline flex items-center gap-1"
                        @click.stop="toggleExpand(item.id)">
                        <span v-if="expandedItems.includes(item.id)">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
                          </svg>
                          收起
                        </span>
                        <span v-else>
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                          </svg>
                          展开
                        </span>
                      </button>
                    </div>

                    <div class="flex justify-between items-center mt-2">
                      <div class="flex items-center gap-2">
                        <div class="text-xs text-base-content/60">{{ formatDateTime(item.created_at) }}</div>
                        <div v-if="item.source" class="text-xs bg-base-300 px-2 py-0.5 rounded-full">
                          来源: {{ item.source }}
                        </div>
                      </div>

                      <div class="flex gap-1">
                        <button class="btn btn-xs btn-ghost" @click.stop="copyToClipboard(item.content)" title="复制">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
                          </svg>
                        </button>
                        <button class="btn btn-xs btn-ghost" @click.stop="previewContent(item)" title="预览">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                          </svg>
                        </button>
                        <button class="btn btn-xs btn-ghost" @click.stop="deleteItem(item.id)" title="删除">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                          </svg>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 右侧预览区 -->
      <div v-if="previewItem" class="hidden md:flex md:w-1/2 h-full flex-col p-4 bg-base-100 border-l border-base-300 overflow-hidden">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-lg font-medium">内容预览</h3>
          <button @click="closePreview" class="btn btn-sm btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        
        <div v-if="previewItem.source" class="text-sm bg-base-300 px-2 py-1 rounded-md inline-block mb-2">
          来源: {{ previewItem.source }}
        </div>
        
        <div class="text-xs text-base-content/60 mb-4">
          创建时间: {{ formatDateTime(previewItem.created_at) }}
        </div>
        
        <!-- 渲染预览内容 -->
        <div class="flex-1 overflow-auto p-4 bg-base-200 rounded-lg markdown-preview">
          <!-- 图片内容 -->
          <div v-if="isImageContent(previewItem.content)" class="text-center">
            <img :src="previewItem.content" alt="图片预览" class="max-w-full" />
          </div>
          <!-- 代码内容 -->
          <pre v-else-if="isCodeContent(previewItem.content)" class="p-4 bg-base-300 rounded-lg overflow-auto"><code>{{ previewItem.content }}</code></pre>
          <!-- 普通文本，支持Markdown -->
          <div v-else v-html="renderMarkdown(previewItem.content)"></div>
        </div>
        
        <div class="mt-4 flex gap-2">
          <button @click="copyToClipboard(previewItem.content)" class="btn btn-sm">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
            </svg>
            复制内容
          </button>
          <button @click="createNoteFromPreview" class="btn btn-sm btn-primary">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            创建笔记
          </button>
        </div>
      </div>
    </div>
    
    <!-- 确认清空全部对话框 -->
    <div class="modal" :class="{ 'modal-open': showClearAllConfirm }">
      <div class="modal-box">
        <h3 class="font-bold text-lg">确认清空</h3>
        <p class="py-4">您确定要清空所有临时笔记吗？此操作不可撤销。</p>
        <div class="modal-action">
          <button class="btn" @click="showClearAllConfirm = false">取消</button>
          <button class="btn btn-error" @click="clearAllEntries">确认清空</button>
        </div>
      </div>
    </div>

    <!-- 复制成功通知 -->
    <div v-if="showCopyNotification"
      class="fixed bottom-4 right-4 bg-success text-success-content px-4 py-2 rounded shadow-lg animate-fade-in-out">
      已复制到剪贴板
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, onActivated, onDeactivated } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';

interface ClipboardHistory {
  id: number;
  content: string;
  created_at: number;
  source?: string;
}

const history = ref<ClipboardHistory[]>([]);
const selectedIds = ref<number[]>([]);
const router = useRouter();
const showCopyNotification = ref(false);
const expandedItems = ref<number[]>([]);
const searchQuery = ref('');
const searchTimeout = ref<number | null>(null);
const showClearAllConfirm = ref(false);
const previewItem = ref<ClipboardHistory | null>(null);

// 高亮显示搜索结果
const highlightText = (text: string, query: string): string => {
  if (!query) return text;
  
  // 如果是图片内容，直接返回原内容以避免破坏格式
  if (isImageContent(text)) {
    return text;
  }
  
  // 转义特殊字符
  const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = new RegExp(`(${escapedQuery})`, 'gi');
  
  return text.replace(regex, '<span class="bg-warning/50">$1</span>');
};

// 全局监听器
let unlisten: (() => void) | null = null;

// 设置事件监听器
async function setupEventListeners() {
  unlisten = await listen('new-clipboard-entry', async () => {
    console.log('New clipboard entry detected, refreshing history...');
    await fetchHistory();
  });
}

// 移除事件监听器
function removeEventListeners() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}

const fetchHistory = async () => {
  try {
    const result = await invoke<ClipboardHistory[]>('get_clipboard_history');
    history.value = result;
  } catch (error) {
    console.error('Failed to fetch clipboard history:', error);
  }
};

const toggleSelection = (id: number) => {
  const index = selectedIds.value.indexOf(id);
  if (index > -1) {
    selectedIds.value.splice(index, 1);
  } else {
    selectedIds.value.push(id);
  }
};

const deleteSelected = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await invoke('delete_clipboard_entries', { ids: selectedIds.value });
    selectedIds.value = [];
    await fetchHistory();
  } catch (error) {
    console.error('Failed to delete clipboard entries:', error);
  }
};

const deleteItem = async (id: number) => {
  try {
    await invoke('delete_clipboard_entries', { ids: [id] });
    const index = selectedIds.value.indexOf(id);
    if (index > -1) {
      selectedIds.value.splice(index, 1);
    }
    await fetchHistory();
  } catch (error) {
    console.error('Failed to delete clipboard entry:', error);
  }
};

const copyToClipboard = async (content: string) => {
  try {
    await invoke('copy_to_clipboard', { text: content });
    // 显示通知
    showCopyNotification.value = true;
    setTimeout(() => {
      showCopyNotification.value = false;
    }, 2000);
  } catch (error) {
    console.error('Failed to copy to clipboard:', error);
  }
};

const mergeToNote = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    // 使用明确的类型或进行类型断言
    interface TipResult {
      id: string;
      title: string;
      content: string;
      [key: string]: any; // 其他可能的字段
    }

    const result = await invoke<TipResult>('create_note_from_history', { ids: selectedIds.value });
    selectedIds.value = [];
    await fetchHistory();

    // 跳转到编辑页面，附加from=clipboard查询参数表示来源
    if (result && result.id) {
      router.push({
        path: `/editor/${result.id}`,
        query: { from: 'clipboard' }
      });
    }
  } catch (error) {
    console.error('Failed to create note from history:', error);
  }
};

const goBack = () => {
  router.back();
};

// 格式化日期时间
function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

// 检查内容是否足够长需要展开/收起
const isContentLong = (content: string) => {
  return content.length > 200 || content.split('\n').length > 5;
};

// 切换内容展开/收起状态
const toggleExpand = (id: number) => {
  const index = expandedItems.value.indexOf(id);
  if (index > -1) {
    expandedItems.value.splice(index, 1);
  } else {
    expandedItems.value.push(id);
  }
};

// 预览内容
const previewContent = (item: ClipboardHistory) => {
  previewItem.value = item;
};

// 关闭预览
const closePreview = () => {
  previewItem.value = null;
};

// 从预览创建笔记
const createNoteFromPreview = async () => {
  if (!previewItem.value) return;
  try {
    interface TipResult {
      id: string;
      title: string;
      content: string;
      [key: string]: any;
    }

    const result = await invoke<TipResult>('create_note_from_history', { 
      ids: [previewItem.value.id] 
    });
    
    // 跳转到编辑页面
    if (result && result.id) {
      router.push({
        path: `/editor/${result.id}`,
        query: { from: 'clipboard' }
      });
    }
  } catch (error) {
    console.error('Failed to create note from preview:', error);
  }
};

// 确认清空全部
const confirmClearAll = () => {
  showClearAllConfirm.value = true;
};

// 清空所有临时笔记
const clearAllEntries = async () => {
  try {
    await invoke('clear_all_clipboard_entries');
    showClearAllConfirm.value = false;
    selectedIds.value = [];
    previewItem.value = null;
    await fetchHistory();
  } catch (error) {
    console.error('Failed to clear all clipboard entries:', error);
  }
};

// 渲染Markdown内容（简化版）
const renderMarkdown = (content: string): string => {
  // 简单的HTML转义和格式处理
  const htmlEscaped = content
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
    
  // 简单的Markdown转HTML
  const markdownFormatted = htmlEscaped
    // 处理图片
    .replace(/!\[(.*?)\]\((.*?)\)/g, '<img alt="$1" src="$2" style="max-width:100%;">')
    // 处理链接
    .replace(/\[(.*?)\]\((.*?)\)/g, '<a href="$2" target="_blank">$1</a>')
    // 处理标题
    .replace(/^# (.*?)$/gm, '<h1>$1</h1>')
    .replace(/^## (.*?)$/gm, '<h2>$1</h2>')
    .replace(/^### (.*?)$/gm, '<h3>$1</h3>')
    // 处理列表
    .replace(/^- (.*?)$/gm, '<li>$1</li>')
    // 处理粗体
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    // 处理斜体
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    // 处理代码
    .replace(/`(.*?)`/g, '<code>$1</code>')
    // 处理换行
    .replace(/\n/g, '<br>');
    
  return markdownFormatted;
};

// 检查是否为图片内容
const isImageContent = (content: string): boolean => {
  // 检查是否包含Base64数据
  return content.startsWith('data:image') && content.includes('base64');
};

// 检查是否为代码内容
const isCodeContent = (content: string): boolean => {
  // 简单判断是否为代码：包含多行且每行都是代码风格
  const lines = content.split('\n');
  return lines.length > 3 && 
         (content.includes('function ') || 
          content.includes('class ') || 
          content.includes('import ') || 
          content.includes('const ') || 
          content.includes('var ') ||
          content.includes(';') ||
          content.includes('{') && content.includes('}'));
};

// 防抖处理搜索
const handleSearch = () => {
  if (searchTimeout.value) {
    clearTimeout(searchTimeout.value);
  }
  
  searchTimeout.value = setTimeout(() => {
    // 搜索逻辑已通过computed实现
    searchTimeout.value = null;
  }, 300) as unknown as number;
};

// 清除搜索
const clearSearch = () => {
  searchQuery.value = '';
};

// 根据搜索关键词过滤历史记录
const filteredHistory = computed(() => {
  if (!searchQuery.value) {
    return history.value;
  }
  
  const query = searchQuery.value.toLowerCase();
  return history.value.filter(item => {
    return item.content.toLowerCase().includes(query) || 
           (item.source && item.source.toLowerCase().includes(query));
  });
});

// 组件生命周期钩子
onMounted(async () => {
  await fetchHistory();
  setupEventListeners();
});

onActivated(async () => {
  console.log('ClipboardHistory组件被激活');
  await fetchHistory();
});

onDeactivated(() => {
  console.log('ClipboardHistory组件被停用');
});

onUnmounted(() => {
  removeEventListeners();
});
</script>

<style scoped>
.card {
  transition: all 0.2s ease;
}

.animate-fade-in-out {
  animation: fadeInOut 2s ease-in-out;
}

@keyframes fadeInOut {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }

  10% {
    opacity: 1;
    transform: translateY(0);
  }

  90% {
    opacity: 1;
    transform: translateY(0);
  }

  100% {
    opacity: 0;
    transform: translateY(-10px);
  }
}

:deep(.bg-warning\/50) {
  background-color: rgba(255, 193, 7, 0.5);
}

img {
  max-width: 100%;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  object-fit: contain;
  max-height: 200px;
}

.markdown-preview img {
  max-height: 500px;
  margin: 0 auto;
}
</style>
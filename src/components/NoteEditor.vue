<template>
  <div class="h-full flex flex-col" v-if="note">
    <!-- 加密内容视图 -->
    <div v-if="isNoteEncrypted && !isNoteUnlocked" class="h-full">
      <EncryptedContent 
        :title="`笔记已加密: ${note.title}`"
        :description="'此笔记受密码保护，请输入正确的密码来查看内容。'"
        :loading="encryptionStore.isLoading"
        :encrypted-at="note.updated_at"
        @unlock="handleUnlockRequest"
        @decrypt="handleDecryptRequest"
      />
    </div>
    
    <!-- 正常编辑器视图 -->
    <div v-else class="h-full flex flex-col">
      <div 
        ref="fullscreenContainer"
        class="h-full flex flex-col" 
        :class="{ 'fullscreen-editor': isFullscreen }"
        tabindex="0" 
        @focusout="onEditorBlur"
        @keydown="handleFullscreenKeyDown">
        
        <!-- 顶部栏 -->
        <EditorTopBar
          v-model:title="localNote.title"
          :is-fullscreen="isFullscreen"
          @input="autoSave"
          @command="handleTopBarCommand"
        />

        <!-- 工具栏 -->
        <EditorToolbar 
          :is-fullscreen="isFullscreen"
          :is-edit-only="isEditOnly"
          :is-preview-mode="isPreviewMode"
          :is-split-mode="isSplitMode"
          :show-toc="showToc"
          :current-highlight-theme="currentHighlightTheme"
          :current-markdown-theme="currentMarkdownTheme"
          @command="handleToolbarCommand"
        />

        <!-- 主要编辑区域容器 -->
        <div class="flex-1 flex overflow-hidden relative">
          <!-- Markdown 编辑器核心组件 -->
          <MarkdownEditor
            :key="note.id"
            v-model="localNote.content"
            :rendered-content="renderedContent"
            :is-split-mode="isSplitMode"
            :is-preview-mode="isPreviewMode"
            ref="markdownEditor"
            @contextmenu="handleContextMenu"
            @paste="handlePaste"
            @keydown="handleKeyDown"
            @preview-scroll="handlePreviewScroll"
          />

          <!-- 右键菜单 -->
          <div v-if="showContextMenu"
            class="context-menu absolute bg-base-200 text-base-content rounded-md shadow-lg p-2 z-30"
            :style="{ top: contextMenuY + 'px', left: contextMenuX + 'px' }">
            <ul class="menu menu-sm p-0">
              <li><a @click="copySelectedText" :class="{ 'disabled': !hasSelectedText }">复制</a></li>
              <li><a @click="pasteFromClipboard">粘贴</a></li>
              <li class="menu-title"><span></span></li>
              <li><a @click="explainWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">AI解释</a></li>
              <li><a @click="translateWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">AI翻译</a></li>
              <li><a @click="tipWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">TIP一下</a></li>
            </ul>
          </div>
            
            <!-- 悬浮目录 -->
            <div v-if="showToc && tocItems.length > 0" 
                 ref="tocContainer"
                class="toc-container fixed bg-base-100 rounded-lg p-4 max-w-xs shadow-lg border border-base-300"
                 :class="{ 'dragging': isDragging }"
                :style="{ right: '20px', top: tocPosition.y + 'px', maxHeight: '70vh', zIndex: 100 }"
                 @mousedown="startDrag"
                 @touchstart="startDrag">
              <div class="toc-header flex items-center justify-between mb-3 pb-2 border-b border-base-300">
                <h3 class="text-sm font-bold text-base-content flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" /></svg>
                  目录
                </h3>
              <button @click="showToc = false" class="btn btn-xs btn-ghost btn-square" @mousedown.stop @touchstart.stop>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>
              <div class="overflow-y-auto overflow-x-hidden" style="max-height: 320px;">
                <ul class="space-y-1 w-full">
                <li v-for="(item, index) in tocItems" :key="index" :style="{ paddingLeft: (item.level - 1) * 12 + 'px' }" class="text-sm overflow-hidden">
                  <a @click="scrollToHeading(item.id)" @mousedown.stop @touchstart.stop class="toc-item block py-1 px-2 text-base-content/80 cursor-pointer" :class="{ 'active': item.id === activeHeadingId }" :title="item.text">
                      {{ item.text }}
                    </a>
                  </li>
                </ul>
            </div>
          </div>
        </div>

        <!-- 底部元数据区域 -->
        <EditorFooter
          v-model:tags="localNote.tags"
          :content-text="localNote.content"
          :title-text="localNote.title"
          :created-at="localNote.created_at"
          :updated-at="localNote.updated_at"
          :is-loading-images="isLoadingImages"
          @saveNote="saveNoteToList"
        />
            </div>
    </div>
  </div>

  <!-- AI 解释弹窗 -->
  <AIExplanationDialog
    :visible="showExplanationBox"
    :loading="isExplaining"
    :content="explanationContent"
    @close="showExplanationBox = false"
    @copy="copyExplanation"
    @insert="insertExplanationToContent"
  />

  <!-- AI 翻译弹窗 -->
  <AITranslationDialog
    :visible="showTranslationBox"
    :loading="isTranslating"
    :content="translationContent"
    @close="showTranslationBox = false"
    @copy="copyTranslation"
    @insert="insertTranslationToContent"
  />

  <!-- TIP 对话框 -->
  <TipInputDialog
    :visible="showTipDialog"
    :prompt="tipPrompt"
    :selected-text="selectedTextForTip"
    @close="closeTipDialog"
    @confirm="confirmTip"
    @set-template="setTipTemplate"
    @reset="resetTipPrompt"
    @save-template="saveCurrentAsTemplate"
  />

  <!-- TIP结果弹窗 -->
  <TipResultDialog
    :visible="showTipResultBox"
    :loading="isTipProcessing"
    :content="tipResultContent"
    @close="closeTipResultBox"
    @copy="copyTipResult"
    @insert="insertTipResultToContent"
  />
</template>

<script setup lang="ts">
import { ref, computed, watch, defineProps, defineEmits, nextTick, onMounted, onActivated, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Marked } from 'marked'
import DOMPurify from 'dompurify'
import EncryptedContent from './EncryptedContent.vue'
import EditorToolbar from './EditorToolbar.vue'
import EditorTopBar from './EditorTopBar.vue'
import EditorFooter from './EditorFooter.vue'
import MarkdownEditor from './MarkdownEditor.vue'
import AIExplanationDialog from './dialogs/AIExplanationDialog.vue'
import AITranslationDialog from './dialogs/AITranslationDialog.vue'
import TipInputDialog from './dialogs/TipInputDialog.vue'
import TipResultDialog from './dialogs/TipResultDialog.vue'
import { showAlert } from '../services/dialog'
import { useEncryptionStore } from '../stores/encryptionStore'
import { getDefaultAIModel } from '../services/aiService'
import Prism from 'prismjs'
// Import prism styles and plugins
import 'prismjs/plugins/line-numbers/prism-line-numbers.css'
import 'prismjs/plugins/line-numbers/prism-line-numbers'
import 'prismjs/plugins/toolbar/prism-toolbar.css'
import 'prismjs/plugins/toolbar/prism-toolbar'
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard'
// Import prism languages
import 'prismjs/components/prism-markup-templating'
import 'prismjs/components/prism-markup'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-java'
import 'prismjs/components/prism-go'
import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-sql'
import 'prismjs/components/prism-yaml'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-php'
import 'prismjs/components/prism-csharp'
import { diff_match_patch as DiffMatchPatch, patch_obj } from 'diff-match-patch';
import { LRUCache } from 'lru-cache'
import { useTipTemplateStore } from '../stores/tipTemplateStore'

// 预定义主题样式映射，避免动态加载CSS
const PRISM_THEMES = {
  default: {
    background: '#f5f2f0',
    color: '#728fcb',
    selectionBackground: '#b3d4fc'
  },
  okaidia: {
    background: '#272822',
    color: '#f8f8f2',
    selectionBackground: '#49483e'
  },
  twilight: {
    background: '#141414',
    color: '#f7f7f7',
    selectionBackground: '#444'
  },
  'solarized-light': {
    background: '#fdf6e3',
    color: '#657b83',
    selectionBackground: '#eee8d5'
  },
  'tomorrow-night': {
    background: '#2d2d2d',
    color: '#ccc',
    selectionBackground: '#515151'
  }
}

const providerMapping: Record<string, string> = {
  'chatgpt': 'openai',
  'gemini': 'gemini',
  'deepseek': 'deepseek',
  'qwen': 'ali',
  'claude': 'anthropic',
  'doubao': 'doubao',
  'grok': 'xai',
  'custom': 'custom'
};

// 安全检查 Prism 语言是否可用
function isPrismLanguageAvailable(lang: string): boolean {
  try {
    // plaintext 总是可用的，因为它不需要特殊的语法高亮
    if (lang === 'plaintext' || lang === 'text' || lang === 'plain') {
      return true;
    }
    
    return !!(
      typeof Prism !== 'undefined' && 
      Prism.languages && 
      typeof Prism.languages === 'object' &&
      Prism.languages[lang] &&
      typeof Prism.highlight === 'function'
    );
  } catch (error) {
    console.warn(`检查 Prism 语言 ${lang} 时出错:`, error);
    return false;
  }
}

// 简化的语言组件初始化函数
async function loadPrismLanguages() {
  try {
    // 检查已加载的语言
    console.log('Prism语言组件加载完成')
  } catch (error) {
    console.error('检查 Prism 语言组件失败:', error);
  }
}

interface Tag {
  id: string;
  name: string;
}

interface Note {
  id: string;
  title: string;
  content: string;
  created_at: number;
  updated_at: number;
  tags: Tag[];
  isPinned?: boolean;
  category_id?: string;
  images?: { [key: string]: string }; // 添加图片存储
}

// 组件属性
const props = defineProps({
  note: {
    type: Object as () => Note,
    required: true
  }
})

// 组件事件
const emit = defineEmits([
  'update',
  'delete-note',
  'duplicate-note',
  'add-tag',
  'remove-tag',
  'toggle-pin',
  'unlock-note',
  'decrypt-note'
])

// 加密store
const encryptionStore = useEncryptionStore()

// 状态
const localNote = ref<Note>({ ...props.note })
const isPreviewMode = ref(false)
const markdownEditor = ref<{ editorTextarea: HTMLTextAreaElement | null; previewDiv: HTMLDivElement | null; } | null>(null);
const editorTextarea = computed(() => markdownEditor.value?.editorTextarea || null);
const previewDiv = computed(() => markdownEditor.value?.previewDiv || null);
const autoSaveTimeout = ref<number | null>(null)
const renderTimeout = ref<number | null>(null)
const renderedContent = ref('')
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const isAIProcessing = ref(false)
const isEditOnly = ref(false)
const isSplitMode = ref(true)
const isSwitchingNote = ref(false) // 用于区分笔记切换和用户输入
const streamingContent = ref('')  // 用于存储流式输出的内容
const isStreaming = ref(false)    // 是否正在流式输出
const currentStreamingId = ref<string | null>(null)  // 当前流式输出的ID
const imageObserver = ref<IntersectionObserver | null>(null)

// 添加全屏模式状态
const isFullscreen = ref(false)
const fullscreenContainer = ref<HTMLElement | null>(null)

// 添加图片加载相关状态
const isLoadingImages = ref(false)
// 将 imageLoadCache 从 Map 更改为 LRUCache
const imageLoadCache = ref(new LRUCache<string, Record<string, string>>({ max: 50 }))
const imageLoadTimeouts = ref<Map<string, number>>(new Map())

const hasSelectedText = computed(() => {
  const textarea = editorTextarea.value
  if (!textarea) return false

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  return start !== end
})
const showExplanationBox = ref(false)
const explanationContent = ref('')
const selectedTextForExplanation = ref('')
const isExplaining = ref(false)
const showTranslationBox = ref(false)
const translationContent = ref('')
const selectedTextForTranslation = ref('')
const isTranslating = ref(false)
// 添加TIP对话框相关状态
const showTipDialog = ref(false)
const tipPrompt = ref('')
const selectedTextForTip = ref('')
const originalTipPrompt = ref('')
// TIP结果弹窗相关状态
const showTipResultBox = ref(false)
const tipResultContent = ref('')
const isTipProcessing = ref(false)

// 动态响应式工具栏相关状态
const toolbarContainer = ref<HTMLElement | null>(null)
const toolbarLeft = ref<HTMLElement | null>(null)
const toolbarRight = ref<HTMLElement | null>(null)
const hiddenItems = ref<any[]>([])

// 目录相关状态
const showToc = ref(false)
const tocItems = ref<{ id: string; level: number; text: string }[]>([]);
const activeHeadingId = ref('');
const tocPosition = ref({ x: window.innerWidth - 320, y: 200 })
const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })
const tocContainer = ref<HTMLElement | null>(null)

const resizeObserver = ref<ResizeObserver | null>(null)
let globalUnlisten: (() => void) | null = null; // 全局事件监听器引用

// 优化撤销/重做堆栈
const dmp = new DiffMatchPatch()
const undoStack = ref<any[]>([])
const redoStack = ref<any[]>([])
const lastSavedContent = ref<string>('')

// 动态响应式工具栏相关函数
function initResponsiveToolbar() {
  if (!toolbarContainer.value || !toolbarLeft.value || !toolbarRight.value) return
  
  // 创建ResizeObserver来监听工具栏容器大小变化
  resizeObserver.value = new ResizeObserver(() => {
    updateToolbarLayout()
  })
  
  resizeObserver.value.observe(toolbarContainer.value)
  
  // 初始化时也检查一次
  nextTick(() => {
    updateToolbarLayout()
  })
}

function updateToolbarLayout() {
  if (!toolbarContainer.value || !toolbarLeft.value || !toolbarRight.value) return
  
  const containerWidth = toolbarContainer.value.offsetWidth
  const rightWidth = toolbarRight.value.offsetWidth
  const availableWidth = containerWidth - rightWidth - 32 // 32px for padding and gaps
  
  // 获取所有工具栏项目
  const allItems = Array.from(toolbarLeft.value.querySelectorAll('.toolbar-item')) as HTMLElement[]
  
  // 重置所有项目的显示状态
  allItems.forEach(item => {
    item.style.display = ''
  })
  
  // 清空隐藏项目列表
  hiddenItems.value = []
  
  // 计算当前显示项目的总宽度
  let currentWidth = 0
  const visibleItems: HTMLElement[] = []
  
  for (const item of allItems) {
    const itemWidth = item.offsetWidth
    
    if (currentWidth + itemWidth <= availableWidth) {
      currentWidth += itemWidth + 8 // 8px for gap
      visibleItems.push(item)
    } else {
      // 隐藏超出的项目
      item.style.display = 'none'
      
      // 添加到隐藏项目列表
      const priority = parseInt(item.dataset.priority || '999')
      const action = getItemAction(item)
      const content = getItemContent(item)
      
      if (action && content) {
        hiddenItems.value.push({
          priority,
          action,
          content
        })
      }
    }
  }
  
  // 按优先级排序隐藏项目
  hiddenItems.value.sort((a, b) => a.priority - b.priority)
  
  // 隐藏文本标签如果空间不足
  updateTextVisibility(availableWidth, currentWidth)
}

function updateTextVisibility(availableWidth: number, currentWidth: number) {
  const textElements = toolbarLeft.value?.querySelectorAll('.toolbar-text') as NodeListOf<HTMLElement>
  
  if (!textElements) return
  
  // 如果空间紧张，隐藏文本标签
  const shouldHideText = currentWidth > availableWidth * 0.8
  
  textElements.forEach(element => {
    element.style.display = shouldHideText ? 'none' : ''
  })
}

function getItemAction(item: HTMLElement): (() => void) | null {
  const priority = parseInt(item.dataset.priority || '999')
  
  // 根据优先级返回对应的操作函数
  switch (priority) {
    case 3: return () => insertMarkdown('~~', '~~')
    case 4: return () => insertMarkdown('- ')
    case 5: return () => insertMarkdown('[', '](https://)')
    case 6: return () => insertMarkdown('> ')
    case 7: return () => insertMarkdown('```\n', '\n```')
    case 8: return () => insertMarkdown('![', '](图片URL)')
    case 9: return () => insertTable()
    case 10: return () => {} // 代码高亮主题
    case 11: return () => {} // Markdown主题
    case 12: return () => {} // 全屏提示
    default: return null
  }
}

function getItemContent(item: HTMLElement): string {
  const priority = parseInt(item.dataset.priority || '999')
  
  // 根据优先级返回对应的HTML内容
  switch (priority) {
    case 3: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M17 9V5H7v4"></path>
        <path d="M7 13v6h10v-6"></path>
        <line x1="4" y1="12" x2="20" y2="12"></line>
      </svg>
      删除线`
    case 4: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="8" y1="6" x2="21" y2="6"></line>
        <line x1="8" y1="12" x2="21" y2="12"></line>
        <line x1="8" y1="18" x2="21" y2="18"></line>
        <line x1="3" y1="6" x2="3.01" y2="6"></line>
        <line x1="3" y1="12" x2="3.01" y2="12"></line>
        <line x1="3" y1="18" x2="3.01" y2="18"></line>
      </svg>
      无序列表`
    case 5: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
        <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
      </svg>
      插入链接`
    case 6: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
        <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
      </svg>
      引用块`
    case 7: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="16 18 22 12 16 6"></polyline>
        <polyline points="8 6 2 12 8 18"></polyline>
      </svg>
      代码块`
    case 8: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <circle cx="8.5" cy="8.5" r="1.5"></circle>
        <polyline points="21 15 16 10 5 21"></polyline>
      </svg>
      插入图片`
    case 9: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <line x1="3" y1="9" x2="21" y2="9"></line>
        <line x1="3" y1="15" x2="21" y2="15"></line>
        <line x1="9" y1="3" x2="9" y2="21"></line>
        <line x1="15" y1="3" x2="15" y2="21"></line>
      </svg>
      插入表格`
    default: return ''
  }
}

// 初始化时保存初始内容到撤销栈
onMounted(() => {
  lastSavedContent.value = localNote.value.content
  
  // 初始化响应式工具栏
  nextTick(() => {
    initResponsiveToolbar()
  })

  markdownWorker.value = new Worker(new URL('../workers/markdown.worker.ts', import.meta.url), { type: 'module' });

  markdownWorker.value.onmessage = (event: MessageEvent<{html?: string, error?: string}>) => {
    if (event.data.error) {
      console.error('Markdown rendering error:', event.data.error);
      renderedContent.value = `<div class="text-error">Markdown rendering error: ${event.data.error}</div>`;
      return;
    }
    if(event.data.html) {
        // 在主线程进行 HTML 清洗，避免在 Worker 中因缺少 `document` 报错
        const safeHtml = DOMPurify.sanitize(event.data.html, {
          ADD_ATTR: ['target', 'class', 'href'],
          ALLOW_DATA_ATTR: true
        });
        renderedContent.value = safeHtml;
        nextTick(() => {
            highlightCode();
            updateToc();
        });
    }
  };
})

onBeforeUnmount(() => {
  if (markdownWorker.value) {
    markdownWorker.value.terminate();
  }
})

// 优化的图片加载函数
async function loadNoteImages(noteId: string, timeout: number = 5000): Promise<Record<string, string> | null> {
  // 检查缓存
  if (imageLoadCache.value.has(noteId)) {
    console.log(`从缓存加载笔记(${noteId})的图片`)
    return imageLoadCache.value.get(noteId) || null
  }

  // 检查是否已有相同请求在进行
  if (imageLoadTimeouts.value.has(noteId)) {
    console.log(`笔记(${noteId})的图片正在加载中，跳过重复请求`)
    return null
  }

  try {
    isLoadingImages.value = true

    // 创建超时Promise
    const timeoutPromise = new Promise<never>((_, reject) => {
      const timeoutId = setTimeout(() => {
        reject(new Error(`图片加载超时(${timeout}ms)`))
      }, timeout) as unknown as number

      imageLoadTimeouts.value.set(noteId, timeoutId)
    })

    // 先获取图片总数
    const countPromise = invoke('get_tip_images_count', { tip_id: noteId }) as Promise<number>
    const totalCount = await Promise.race([countPromise, timeoutPromise])

    if (totalCount === 0) {
      imageLoadCache.value.set(noteId, {})
      return {}
    }

    // 分批加载图片，每次最多加载5张
    const batchSize = 5
    const allImages: Record<string, string> = {}

    for (let offset = 0; offset < totalCount; offset += batchSize) {
      const batchPromise = invoke('get_tip_images', {
        tip_id: noteId,
        limit: batchSize,
        offset: offset
      }) as Promise<Record<string, string>>

      const batchImages = await Promise.race([batchPromise, timeoutPromise])
      Object.assign(allImages, batchImages)

      console.log(`加载笔记(${noteId})图片批次 ${Math.floor(offset / batchSize) + 1}/${Math.ceil(totalCount / batchSize)}`)

      // 如果是第一批，立即更新界面显示
      if (offset === 0 && Object.keys(batchImages).length > 0) {
        // 检查当前笔记是否还是目标笔记
        if (localNote.value.id === noteId) {
          localNote.value.images = { ...batchImages }
          console.log(`首批图片已显示，笔记(${noteId})`)
        }
      }
    }

    console.log(`获取到笔记(${noteId})的图片总数: ${Object.keys(allImages).length}`)

    // 缓存完整结果
    imageLoadCache.value.set(noteId, allImages)

    return allImages
  } catch (error) {
    console.error(`加载笔记(${noteId})图片失败:`, error)

    // 如果是超时错误，缓存空结果避免重复请求
    if (error instanceof Error && error.message.includes('超时')) {
      imageLoadCache.value.set(noteId, {})
      console.warn(`笔记(${noteId})图片加载超时，已缓存空结果`)
    }

    return null
  } finally {
    // 清理超时定时器
    const timeoutId = imageLoadTimeouts.value.get(noteId)
    if (timeoutId) {
      clearTimeout(timeoutId)
      imageLoadTimeouts.value.delete(noteId)
    }

    isLoadingImages.value = false
  }
}

// 异步加载图片，不阻塞界面
function loadImagesAsync(noteId: string) {
  // 使用nextTick确保在下一个事件循环中执行
  nextTick(async () => {
    const images = await loadNoteImages(noteId, 3000) // 3秒超时

    // 检查当前笔记是否还是目标笔记（避免切换过快导致的状态错乱）
    if (localNote.value.id === noteId && images && Object.keys(images).length > 0) {
      localNote.value.images = images
      console.log(`异步加载完成，笔记(${noteId})图片已更新到本地状态，触发重新渲染`)
      // 图片加载完成后，再次渲染以显示图片
      renderMarkdown()
    }
  })
}



// 监听外部note变化 - 优化版本
watch(() => props.note, async (newNote, oldNote) => {
  // 如果是初始化（oldNote为undefined）或者笔记ID发生变化（切换到不同的笔记），才完全重新设置localNote
  if (!oldNote || oldNote.id !== newNote.id) {
    isSwitchingNote.value = true;
    // 使用深拷贝，确保localNote是完全独立的副本
    localNote.value = JSON.parse(JSON.stringify(newNote));

    // 检查笔记是否为已解锁的加密笔记
    if (encryptionStore.isItemEncrypted(newNote.id) && encryptionStore.isItemUnlocked(newNote.id)) {
      // 获取解密后的内容
      const decryptedContent = await encryptionStore.getUnlockedNoteContent(newNote.id);
      if (decryptedContent !== null) {
        localNote.value.content = decryptedContent;
      }
    }

    // 清除可能存在的延迟渲染
    if (renderTimeout.value) {
        clearTimeout(renderTimeout.value);
    }
    // 立即渲染
    renderMarkdown();

    // 如果笔记有ID但没有images数据，异步加载图片（不阻塞界面）
    if (newNote.id && !newNote.images) {
      // 立即显示笔记内容，图片稍后异步加载
      loadImagesAsync(newNote.id)
    }

    // 等待DOM更新后再重置标志
    await nextTick();
    isSwitchingNote.value = false;
  }
  // 如果是同一个笔记的更新，只更新非编辑相关的字段（如category_id等）
  else {
    // 只更新非内容相关的字段，避免覆盖用户正在编辑的内容
    if (newNote.category_id !== localNote.value.category_id) {
      localNote.value.category_id = newNote.category_id;
    }
    if (newNote.tags && JSON.stringify(newNote.tags) !== JSON.stringify(localNote.value.tags)) {
      localNote.value.tags = newNote.tags;
    }
    
    // 重要：检查内容是否从加密状态变为解密状态
    // 如果内容发生变化且笔记已解锁，则更新本地内容
    // 或者如果当前显示的是占位符，而新内容不是占位符，也要更新
    if (newNote.content !== localNote.value.content) {
      const isCurrentPlaceholder = localNote.value.content === "[此笔记已加密，请解锁后查看]"
      const isNewContentDecrypted = newNote.content !== "[此笔记已加密，请解锁后查看]" && 
                                   !newNote.content.includes('"salt"') && 
                                   !newNote.content.includes('"encrypted_data"')
      
      // 如果当前是占位符，新内容是解密后的内容，或者笔记已解锁，则更新
      if (isCurrentPlaceholder && isNewContentDecrypted) {
        console.log('检测到内容从占位符变为解密内容，更新本地内容')
        localNote.value.content = newNote.content;
      } else if (encryptionStore.isItemEncrypted(newNote.id) && encryptionStore.isItemUnlocked(newNote.id)) {
        console.log('检测到已解锁笔记内容变化，更新本地内容')
        localNote.value.content = newNote.content;
      }
    }
  }
}, { immediate: true, deep: true })

// 添加对解锁状态变化的监听
watch(
  () => ({
    noteId: props.note.id,
    isEncrypted: encryptionStore.isItemEncrypted(props.note.id),
    isUnlocked: encryptionStore.isItemUnlocked(props.note.id),
    noteContent: props.note.content // 添加内容监听
  }),
  async (newState, oldState) => {
    console.log('NoteEditor: 检测到状态变化', {
      noteId: newState.noteId,
      isEncrypted: newState.isEncrypted,
      isUnlocked: newState.isUnlocked,
      contentLength: newState.noteContent.length,
      contentPreview: newState.noteContent.substring(0, 50),
      currentLocalContent: localNote.value.content.substring(0, 50)
    })
    
    // 如果笔记刚刚被解锁（之前未解锁，现在已解锁）
    if (newState.isEncrypted && 
        newState.isUnlocked && 
        oldState && 
        !oldState.isUnlocked) {
      console.log('NoteEditor: 检测到笔记解锁状态变化，获取解密内容')
      const decryptedContent = await encryptionStore.getUnlockedNoteContent(newState.noteId);
      if (decryptedContent !== null) {
        console.log('NoteEditor: 成功获取解密内容，长度:', decryptedContent.length)
        localNote.value.content = decryptedContent;
        console.log('NoteEditor: 已更新为解密后的内容')
      } else {
        console.error('NoteEditor: 获取解密内容失败')
      }
    }
    
    // 如果内容从占位符变为真实内容，也要更新
    if (oldState && 
        oldState.noteContent === "[此笔记已加密，请解锁后查看]" &&
        newState.noteContent !== "[此笔记已加密，请解锁后查看]" &&
        newState.isEncrypted &&
        newState.isUnlocked) {
      console.log('NoteEditor: 检测到内容从占位符变为解密内容')
      localNote.value.content = newState.noteContent;
    }
    
    // 如果当前本地内容是占位符，但传入的内容是解密后的内容，也要更新
    if (localNote.value.content === "[此笔记已加密，请解锁后查看]" &&
        newState.noteContent !== "[此笔记已加密，请解锁后查看]" &&
        !newState.noteContent.includes('"salt"') &&
        !newState.noteContent.includes('"encrypted_data"') &&
        newState.isUnlocked) {
      console.log('NoteEditor: 检测到本地内容为占位符，传入内容为解密内容，更新本地内容')
      localNote.value.content = newState.noteContent;
    }
  },
  { deep: true }
)

// 添加键盘快捷键处理函数
function handleKeyDown(event: KeyboardEvent) {
  // 检查是否按下了Ctrl键(Windows)或Command键(Mac)
  const isCtrlOrCmd = event.ctrlKey || event.metaKey

  // 撤销: Ctrl+Z
  if (isCtrlOrCmd && event.key === 'z' && !event.shiftKey) {
    event.preventDefault()
    undo()
    return
  }

  // 重做: Ctrl+Y 或 Ctrl+Shift+Z
  if ((isCtrlOrCmd && event.key === 'y') ||
    (isCtrlOrCmd && event.shiftKey && event.key === 'z')) {
    event.preventDefault()
    redo()
    return
  }

  // 粗体: Ctrl+B
  if (isCtrlOrCmd && event.key === 'b') {
    event.preventDefault()
    insertMarkdown('**', '**')
    return
  }

  // 斜体: Ctrl+I
  if (isCtrlOrCmd && event.key === 'i') {
    event.preventDefault()
    insertMarkdown('*', '*')
    return
  }

  // 链接: Ctrl+K
  if (isCtrlOrCmd && event.key === 'k') {
    event.preventDefault()
    insertMarkdown('[', '](https://)')
    return
  }

  // 代码块: Ctrl+Shift+C
  if (isCtrlOrCmd && event.shiftKey && event.key === 'c') {
    event.preventDefault()
    insertMarkdown('```\n', '\n```')
    return
  }

  // 任务列表: Ctrl+Shift+T
  if (isCtrlOrCmd && event.shiftKey && event.key === 't') {
    event.preventDefault()
    insertMarkdown('- [ ] ')
    return
  }

  // 保存: Ctrl+S
  if (isCtrlOrCmd && event.key === 's') {
    event.preventDefault()
    saveNoteToList()
    return
  }

  // 对于其他内容修改按键，添加到撤销堆栈
  // 避免在每次按键都保存，仅在内容实际变化时
  setTimeout(() => {
    const currentContent = localNote.value.content
    if (currentContent !== lastSavedContent.value) {
      // 计算差异并添加到撤销堆栈
      const diff = dmp.diff_main(lastSavedContent.value, currentContent, true);
      if (diff.length > 2) {
        dmp.diff_cleanupSemantic(diff);
      }
      const patch = dmp.patch_make(lastSavedContent.value, diff);
      
      undoStack.value.push(patch)
      // 清空重做堆栈
      redoStack.value = []
      // 更新最后保存的内容
      lastSavedContent.value = currentContent

      // 限制撤销堆栈大小以避免内存问题
      if (undoStack.value.length > 100) {
        undoStack.value = undoStack.value.slice(-100)
      }
    }
  }, 100)
}

// 撤销函数
function undo() {
  if (undoStack.value.length === 0) return

  const patch = undoStack.value.pop()

  // 应用补丁回到上一个状态
  const [previousContent, results] = dmp.patch_apply(patch, lastSavedContent.value)

  // 检查应用是否成功
  if (results.every((r: boolean) => r)) {
    // 将当前内容（撤销前）的逆向补丁保存到重做堆栈
    const redoDiff = dmp.diff_main(previousContent, lastSavedContent.value, true)
    const redoPatch = dmp.patch_make(previousContent, redoDiff)
    redoStack.value.push(redoPatch as any)

    // 更新编辑器内容
    localNote.value.content = previousContent
    lastSavedContent.value = previousContent
  } else {
    console.error("撤销失败: 补丁应用不成功", results)
    // 如果失败，将补丁放回栈中
    undoStack.value.push(patch)
    return
  }

  // 触发自动保存，但使用延迟，避免频繁保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }
  autoSaveTimeout.value = setTimeout(() => {
    emit('update', { ...localNote.value, _contentOnly: true })
  }, 1000) as unknown as number
}

// 重做函数
function redo() {
  if (redoStack.value.length === 0) return

  // 获取下一个状态的补丁
  const patch = redoStack.value.pop()

  // 应用补丁
  const [nextContent, results] = dmp.patch_apply(patch, lastSavedContent.value)
  
  if (results.every((r: boolean) => r)) {
    // 将当前内容（重做前）的逆向补丁保存到撤销堆栈
    const undoDiff = dmp.diff_main(nextContent, lastSavedContent.value, true)
    const undoPatch = dmp.patch_make(nextContent, undoDiff)
    undoStack.value.push(undoPatch as any)

    // 更新编辑器内容
    localNote.value.content = nextContent
    lastSavedContent.value = nextContent
  } else {
    console.error("重做失败: 补丁应用不成功", results)
    redoStack.value.push(patch)
    return
  }

  // 触发自动保存，但使用延迟，避免频繁保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }
  autoSaveTimeout.value = setTimeout(() => {
    emit('update', { ...localNote.value, _contentOnly: true })
  }, 1000) as unknown as number
}



function enhanceCodeBlocks() {
  // 查找所有还未处理的代码块
  const codeElements = document.querySelectorAll('.prose pre > code:not([data-enhanced])')
  const currentTheme = currentHighlightTheme.value || 'default'
  
  codeElements.forEach((codeElement, _index) => {
    const pre = codeElement.closest('pre')
    if (!pre) return
    
    // 标记已处理，避免重复处理
    codeElement.setAttribute('data-enhanced', 'true')
    
    // 避免重复处理
    if (pre.closest('.code-block-container')) {
      return
    }

    // 获取语言类型
    const classNames = codeElement.className.split(' ')
    const langClass = classNames.find(cls => cls.startsWith('language-'))
    const lang = langClass ? langClass.replace('language-', '') : 'plaintext'

    // 如果没有指定语言，为code元素添加language-plaintext类
    if (!langClass) {
      codeElement.classList.add('language-plaintext')
    }

    // 清理可能存在的旧样式类
    codeElement.classList.remove('prism-default', 'prism-okaidia', 'prism-twilight', 'prism-solarized-light', 'prism-tomorrow-night')
    pre.classList.remove('prism-default', 'prism-okaidia', 'prism-twilight', 'prism-solarized-light', 'prism-tomorrow-night')

    // 添加当前主题类
    const themeClass = `prism-${currentTheme}`
    codeElement.classList.add(themeClass)
    pre.classList.add(themeClass)
    
    // 创建容器
    const container = document.createElement('div')
    container.className = 'code-block-container'


    // 为pre元素添加行号支持
    pre.classList.add('line-numbers')

    // 将pre元素包装到容器中
    const parent = pre.parentNode
    if (parent) {
      parent.insertBefore(container, pre)
      container.appendChild(pre)
    }
  })
}

// HTML 转义函数
function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}



// 方法
function autoSave() {
  // 防抖自动保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }

  autoSaveTimeout.value = setTimeout(() => {
    // 更新本地状态，但暂不触发外部更新
    localNote.value.updated_at = Date.now()

    // 判断是否只更新标题
    if (localNote.value.content === lastSavedContent.value && localNote.value.title !== lastSavedContent.value) {
      // 只更新标题
      emit('update', { ...localNote.value, _titleOnly: true })
    } else {
      // 当内容变化时，仅更新内容但不触发列表重排序
      // 使用_contentOnly标记表示这是内容更新，不需要列表重排序
      emit('update', { ...localNote.value, _contentOnly: true })
    }
  }, 1000) as unknown as number // 延长防抖时间到1秒
}

// 当编辑器失去焦点时调用，将更新传递给父组件
function saveNoteToList() {
  // 清除任何未完成的自动保存计时器
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
    autoSaveTimeout.value = null
  }

  // 添加一个标记，表明这次更新是由失去焦点事件触发的
  // 允许父组件据此决定是否需要重新排序列表
  const noteToUpdate = { ...localNote.value, _fromBlur: true }

  // 如果笔记中包含图片，确保图片数据也被正确传递
  if (localNote.value.images && Object.keys(localNote.value.images).length > 0) {
    noteToUpdate.images = { ...localNote.value.images }
  }

  emit('update', noteToUpdate)
}

function insertMarkdown(prefix: string, suffix: string = '') {
  if (isPreviewMode.value) return

  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selectedText = localNote.value.content.substring(start, end)

  // 插入markdown标记
  const newText =
    localNote.value.content.substring(0, start) +
    prefix + selectedText + suffix +
    localNote.value.content.substring(end)

  localNote.value.content = newText

  // 更新后重新设置光标位置
  nextTick(() => {
    textarea.focus()
    if (selectedText.length > 0) {
      textarea.selectionStart = start + prefix.length
      textarea.selectionEnd = end + prefix.length
    } else {
      textarea.selectionStart = textarea.selectionEnd = start + prefix.length
    }
  })

  autoSave()
}

function handleContextMenu(event: MouseEvent) {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 防止默认菜单显示
  event.preventDefault()
  event.stopPropagation() // 阻止事件冒泡


  // 获取鼠标点击相对于编辑器的位置
  const editorRect = textarea.getBoundingClientRect()

  // 计算相对于编辑器内部的坐标
  contextMenuX.value = event.clientX - editorRect.left
  contextMenuY.value = event.clientY - editorRect.top

  // 显示右键菜单
  showContextMenu.value = true

  // 确保菜单不会超出编辑器边界
  nextTick(() => {
    const menu = document.querySelector('.context-menu') as HTMLElement
    if (!menu) return

    const menuRect = menu.getBoundingClientRect()

    // 如果菜单超出右边界，将其向左移动
    if (contextMenuX.value + menuRect.width > editorRect.width) {
      contextMenuX.value = Math.max(0, editorRect.width - menuRect.width - 5)
    }

    // 如果菜单超出下边界，将其向上移动
    if (contextMenuY.value + menuRect.height > editorRect.height) {
      contextMenuY.value = Math.max(0, editorRect.height - menuRect.height - 5)
    }

    // 点击其他区域关闭菜单
    const closeContextMenu = (e: MouseEvent) => {
      e.stopPropagation() // 阻止事件冒泡

      // 如果点击的不是菜单内部的元素，则关闭菜单
      const menu = document.querySelector('.context-menu')
      if (menu && !menu.contains(e.target as Node)) {
        showContextMenu.value = false
        document.removeEventListener('mousedown', closeContextMenu, true)
      }
    }

    // 使用mousedown事件而不是click事件，避免与菜单点击冲突
    // 使用捕获阶段监听点击事件，确保先于其他事件处理器
    document.addEventListener('mousedown', closeContextMenu, true)
  })
}

async function expandSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  
  // 显示TIP对话框让用户修改提示词，保存选择位置
  selectedTextForTip.value = selectedText
  originalTipPrompt.value = selectedText
  tipPrompt.value = originalTipPrompt.value
  
  // 保存选择位置用于后续处理
  ;(window as any)._tipSelectionStart = start
  ;(window as any)._tipSelectionEnd = end
  
  showTipDialog.value = true
  showContextMenu.value = false
}

// 添加解释功能函数
async function explainSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForExplanation.value = selectedText
  explanationContent.value = ''
  showExplanationBox.value = true
  showContextMenu.value = false

  // 使用AI解释选中的文本
  await processExplanation(selectedText)
}

// 根据笔记标题或选中内容使用AI扩充
async function expandWithAI(input?: string, start?: number, end?: number) {
  const textarea = editorTextarea.value
  let promptText = input || ''
  let startPos = start || 0
  let endPos = end || 0

  // 如果没有传入参数，检查是否有选中的文本
  if (!input) {
    if (textarea) {
      startPos = textarea.selectionStart
      endPos = textarea.selectionEnd

      if (startPos !== endPos) {
        // 使用选中的文本
        promptText = localNote.value.content.substring(startPos, endPos)
      } else {
        // 使用标题作为提示
        promptText = localNote.value.title
        if (!promptText) {
          await showAlert('请先输入笔记标题或选择要扩充的文本', { title: '提示' })
          return
        }

        // 追加到末尾
        startPos = localNote.value.content.length
        endPos = localNote.value.content.length
      }
    } else {
      // 使用标题作为提示
      promptText = localNote.value.title
      if (!promptText) {
        await showAlert('请先输入笔记标题或选择要扩充的文本', { title: '提示' })
        return
      }

      // 追加到末尾
      startPos = localNote.value.content.length
      endPos = localNote.value.content.length
    }
  }

  // 构建提示
  const expandPrompt = promptText

  // 处理AI请求
  await processWithAI(promptText, expandPrompt, false, startPos, endPos)
}

// 通用AI处理函数 - 处理不同类型的AI请求
async function processWithAI(originalText: string, prompt: string, appendResult = false, startPos?: number, endPos?: number) {
  try {
    isAIProcessing.value = true
    showContextMenu.value = false

    // 重置流式输出状态
    streamingContent.value = ''
    isStreaming.value = true

    // 先清理旧的事件监听器
    if (globalUnlisten) {
      try {
        globalUnlisten();
        globalUnlisten = null;
      } catch (e) {
        console.error('清理旧事件监听器失败:', e);
      }
    }

    // 生成唯一的流ID
    currentStreamingId.value = `stream_${Date.now()}`
    console.log(`生成流ID: ${currentStreamingId.value}`)

    // 使用全局默认AI提供商
    const providerId = defaultProviderId.value

    // 在发送API请求前设置事件监听器
    const { listen } = await import('@tauri-apps/api/event')
    globalUnlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }

      // 安全检查：确保我们仍在处理中且ID匹配
      if (!isStreaming.value || !currentStreamingId.value) {
        console.log(`流处理已取消或重置，忽略事件`)
        return
      }
      
      // 检查后端返回的错误
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        showAlert(`AI处理失败: ${payload.error}`, { title: '错误' })
        cleanupStream()
        return
      }

      // 确保只处理当前流ID的事件
      if (payload.id !== currentStreamingId.value) {
        console.log(`忽略不匹配的流ID: ${payload.id}, 当前ID: ${currentStreamingId.value}`)
        return
      }

      console.log(`收到流chunk: id=${payload.id}, 长度=${payload.chunk?.length || 0}, done=${payload.done}`)

      if (payload.chunk) {
        // 更新流式内容
        streamingContent.value += payload.chunk

        // 如果不需要实时更新编辑器内容（如解释模式），则不更新编辑器
        if (!appendResult) {
          updateEditorContent(startPos, endPos);
        }
      }

      // 如果完成了，清理并保存
      if (payload.done) {
        console.log('流式输出完成，清理资源')

        // 如果是解释模式，将结果追加到末尾
        if (appendResult) {
          const prefix = localNote.value.content.length > 0 ? '\n\n' : '';
          const explanation = `> 💡 ${originalText}\n\n${streamingContent.value}`;
          localNote.value.content += prefix + explanation;

          // 更新编辑器内容
          nextTick(() => {
            if (editorTextarea.value) {
              editorTextarea.value.value = localNote.value.content;
              editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }));

              // 滚动到底部
              editorTextarea.value.scrollTop = editorTextarea.value.scrollHeight;
            }
          });
        }

        cleanupStream()
        saveNoteToList()
      }
    })

    // 发送AI请求
    console.log(`使用模型${providerId}发送请求，提示文本长度: ${prompt.length}字符`)
    await invoke('send_ai_message_stream', {
      providerId: providerId,
      message: prompt,
      streamId: currentStreamingId.value,
      messages: undefined,
      customModelName: undefined
    })

    console.log('AI请求已发送，等待流式响应...')

  } catch (error) {
    console.error('AI处理失败:', error)
    await showAlert('AI处理失败: ' + error, { title: '错误' })
    cleanupStream()
  }
}

// 更新编辑器内容的函数 - 从processWithAI中提取出来
function updateEditorContent(startPos?: number, endPos?: number) {
  if (startPos === undefined || endPos === undefined) return;

  // 记录当前的滚动位置
  let scrollTop = 0
  if (editorTextarea.value) {
    scrollTop = editorTextarea.value.scrollTop
  }

  // 更新编辑器内容
  if (startPos !== endPos) {
    // 替换选中的文本
    localNote.value.content =
      localNote.value.content.substring(0, startPos) +
      streamingContent.value +
      localNote.value.content.substring(endPos)

    // 更新结束位置以反映新内容的长度
    endPos = startPos + streamingContent.value.length
  } else {
    // 追加到内容末尾
    const contentPrefix = localNote.value.content.length > 0 ? '\n\n' : ''
    const prefixLength = contentPrefix.length

    localNote.value.content =
      localNote.value.content.substring(0, startPos) +
      contentPrefix +
      streamingContent.value

    // 更新结束位置以反映新内容的长度
    endPos = startPos + prefixLength + streamingContent.value.length
  }

  // 强制更新编辑器内容
  nextTick(() => {
    if (editorTextarea.value) {
      if (editorTextarea.value.value !== localNote.value.content) {
        // 直接设置值
        editorTextarea.value.value = localNote.value.content

        // 模拟输入事件以触发其他可能的监听器
        editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }))

        // 恢复滚动位置
        editorTextarea.value.scrollTop = scrollTop

        // 设置光标位置到更新内容的末尾
        editorTextarea.value.setSelectionRange(endPos, endPos)

        // 如果处于编辑模式，保持焦点
        if (isEditOnly.value || isSplitMode.value || !isPreviewMode.value) {
          editorTextarea.value.focus()
        }
      }
    }
  })
}

function cleanupStream() {
  if (globalUnlisten) {
    try {
      globalUnlisten()
      globalUnlisten = null
      console.log('已清理事件监听器')
    } catch (e) {
      console.error('清理事件监听器失败:', e)
    }
  }

  isStreaming.value = false
  currentStreamingId.value = null
  isAIProcessing.value = false
}

async function cancelAIGeneration() {
  if (isStreaming.value && currentStreamingId.value) {
    try {
      console.log(`取消生成: ${currentStreamingId.value}`)
      await invoke('cancel_ai_generation', {
        streamId: currentStreamingId.value
      })

      // 更新编辑器内容以确保显示当前生成的内容
      nextTick(() => {
        if (editorTextarea.value) {
          editorTextarea.value.value = localNote.value.content
          editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }))
        }
      })

      // 保存当前已生成的内容
      saveNoteToList()

    } catch (error) {
      console.error('取消AI生成失败:', error)
    } finally {
      cleanupStream()
    }
  }
}

function setEditMode(mode: string) {
  if (mode === 'editOnly') {
    isEditOnly.value = true
    isPreviewMode.value = false
    isSplitMode.value = false
  } else if (mode === 'preview') {
    isEditOnly.value = false
    isPreviewMode.value = true
    isSplitMode.value = false
  } else if (mode === 'split') {
    isEditOnly.value = false
    isPreviewMode.value = false
    isSplitMode.value = true
  }
    // 模式切换后重新应用代码块主题
  nextTick(() => {
    forceRefreshCodeBlocks(currentHighlightTheme.value)
  })
}

// 添加copySelectedText函数
async function copySelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start !== end) {
    const selectedText = localNote.value.content.substring(start, end)
    try {
      await navigator.clipboard.writeText(selectedText)
      showContextMenu.value = false
    } catch (error) {
      console.error('复制到剪贴板失败:', error)
    }
  }

  showContextMenu.value = false
}

// 添加点击事件监听器以关闭右键菜单
function setupDocumentClickListener() {
  document.addEventListener('click', () => {
    showContextMenu.value = false
  })
}

// 修改handlePaste函数
async function handlePaste(event: ClipboardEvent) {
  // 检查是否包含图片
  const items = event.clipboardData?.items
  if (!items) return

  let hasImage = false

  // 检查剪贴板中的所有项
  for (let i = 0; i < items.length; i++) {
    const item = items[i]

    // 如果是图片类型
    if (item.type.indexOf('image') !== -1) {
      hasImage = true

      // 防止默认粘贴行为
      event.preventDefault()

      // 获取文件
      const file = item.getAsFile()
      if (!file) continue

      try {
        // 显示处理中状态
        isAIProcessing.value = true

        // 将图片转换为Base64
        const base64Image = await convertImageToBase64(file)
        console.log(`图片转换为Base64格式成功，长度: ${base64Image.length}`)

        // 生成唯一ID
        const imageId = `img_${Date.now()}_${Math.floor(Math.random() * 1000)}`
        console.log(`生成图片ID: ${imageId}`)

        // 确保笔记已保存（有ID）
        if (!localNote.value.id) {
          throw new Error('请先保存笔记再粘贴图片')
        }
        console.log(`笔记ID: ${localNote.value.id}，准备保存图片`)

        // 保存图片到数据库
        console.log(`调用save_tip_image API，参数: tip_id=${localNote.value.id}, image_id=${imageId}`)
        await invoke('save_tip_image', {
          imageData: {
            tip_id: localNote.value.id,
            image_id: imageId,
            image_data: base64Image
          }
        })
        console.log('图片已成功保存到数据库')

        // 确保images对象存在
        if (!localNote.value.images) {
          localNote.value.images = {}
        }

        // 保存图片到本地状态
        localNote.value.images[imageId] = base64Image

        // 在光标位置插入Markdown图片引用
        const textarea = editorTextarea.value
        if (textarea) {
          const start = textarea.selectionStart
          const end = textarea.selectionEnd

          // 构建Markdown图片引用，使用本地ID引用图片
          const imageMarkdown = `![图片](local://${imageId})`

          // 在光标位置插入
          localNote.value.content =
            localNote.value.content.substring(0, start) +
            imageMarkdown +
            localNote.value.content.substring(end)

          // 更新界面 - 确保编辑器内容更新
          nextTick(() => {
            if (textarea) {
              textarea.value = localNote.value.content
              textarea.dispatchEvent(new Event('input', { bubbles: true }))

              // 设置光标位置到图片引用后
              const newCursorPosition = start + imageMarkdown.length
              textarea.setSelectionRange(newCursorPosition, newCursorPosition)
              textarea.focus()
            }
          })

          // 立即更新编辑器状态以显示图片
          autoSave()

          // 确保笔记被保存到列表
          saveNoteToList()
        }

        // 显示成功提示
        console.log('图片已保存到数据库，ID:', imageId)
      } catch (error) {
        console.error('处理粘贴图片失败:', error)

        // 获取详细的错误信息
        let errorMessage = '处理图片失败';
        if (error instanceof Error) {
          errorMessage = `${errorMessage}: ${error.message}`;
          console.error('错误详情:', error.stack);
        } else {
          errorMessage = `${errorMessage}: ${String(error)}`;
        }

        await showAlert(errorMessage, { title: '错误' })
      } finally {
        isAIProcessing.value = false
      }

      // 只处理第一张图片
      break
    }
  }

  // 如果没有图片，则使用默认粘贴行为
  if (!hasImage) {
    return true
  }
}

// 将图片文件转换为Base64，重构为更可靠的实现
function convertImageToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = (event) => {
      if (event.target?.result) {
        const result = event.target.result as string
        // 确保结果是base64格式
        if (typeof result === 'string' && result.startsWith('data:')) {
          resolve(result)
        } else {
          reject(new Error('转换图片格式失败'))
        }
      } else {
        reject(new Error('读取图片失败'))
      }
    }

    reader.onerror = (error) => {
      reject(new Error('读取文件出错: ' + error))
    }

    // 确保以DataURL格式读取
    reader.readAsDataURL(file)
  })
}

// 添加这个新函数来处理解释
async function processExplanation(textToExplain: string) {
  try {
    isExplaining.value = true

    // 使用全局默认AI提供商
    const providerId = defaultProviderId.value

    // 创建唯一的流ID
    const streamId = `explain_${Date.now()}`

    // 构建包含笔记标题的提示信息
    const noteTitle = localNote.value.title || '无标题笔记'
    const prompt = `请解释以下内容，简明扼要。这段内容来自笔记"${noteTitle}"：\n\n${textToExplain}`

    // 设置事件监听器来接收流式响应
    const { listen } = await import('@tauri-apps/api/event')
    let rawExplanation = ''
    const unlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }

      // 确保ID匹配
      if (payload.id !== streamId) return

      // 检查后端返回的错误
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        explanationContent.value = `<p class="text-error">解释生成失败: ${payload.error}</p>`
        isExplaining.value = false
        unlisten()
        return
      }

      if (payload.chunk) {
        // 累积解释内容
        rawExplanation += payload.chunk
        // 不再使用 marked，直接设置为带有段落标签的HTML
        explanationContent.value = renderInlineMarkdown(rawExplanation)
      }

      // 如果完成了，清理监听器
      if (payload.done) {
        isExplaining.value = false
        unlisten()
      }
    })

    // 发送AI请求
    await invoke('send_ai_message_stream', {
      providerId: providerId,
      message: prompt,
      streamId: streamId,
      messages: undefined,
      customModelName: undefined
    })

  } catch (error) {
    console.error('AI解释生成失败:', error)
    explanationContent.value = `<p class=\"text-error\">解释生成失败: ${error}</p>`
    isExplaining.value = false
  }
}

// 复制解释内容
async function copyExplanation() {
  try {
    await navigator.clipboard.writeText(explanationContent.value)
    // 可以添加一个复制成功的提示
  } catch (error) {
    console.error('复制到剪贴板失败:', error)
  }
}

// 将解释内容插入到笔记中
function insertExplanationToContent() {
  // 在光标位置或文档末尾插入解释内容
  const textarea = editorTextarea.value
  if (!textarea) return

  const prefix = '\n\n> 💡 解释：\n\n'
  const content = prefix + explanationContent.value

  const end = textarea.selectionEnd

  // 插入内容
  localNote.value.content =
    localNote.value.content.substring(0, end) +
    content +
    localNote.value.content.substring(end)

  // 更新光标位置
  nextTick(() => {
    textarea.selectionStart = end + content.length
    textarea.selectionEnd = end + content.length
    textarea.focus()
  })

  // 保存笔记
  autoSave()

  // 关闭解释框
  showExplanationBox.value = false
}

// 分享笔记功能
async function shareNote() {
  // 实现分享功能
  await showAlert(`分享链接已复制: ${window.location.origin}/note/${localNote.value.id}`, { title: '分享成功' })
}

// 导出笔记功能
function exportNote() {
  // 实现导出功能
  const filename = `${localNote.value.title || 'note'}.md`
  const content = localNote.value.content

  const blob = new Blob([content], { type: 'text/markdown' })
  const url = URL.createObjectURL(blob)

  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()

  URL.revokeObjectURL(url)
}

// 格式化日期时间
function formatDateTime(dateString: number): string {
  if (!dateString) return ''

  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 在组件属性下添加以下状态变量
const currentHighlightTheme = ref(localStorage.getItem('mytips-highlight-theme') || getDefaultHighlightTheme())
const currentMarkdownTheme = ref(localStorage.getItem('mytips-markdown-theme') || 'github')

// 添加根据系统主题自动选择代码高亮主题的函数
function getDefaultHighlightTheme() {
  // 检查系统是否支持颜色模式查询
  if (window.matchMedia) {
    // 检查是否为暗色模式
    const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    return isDarkMode ? 'tomorrow-night' : 'default'
  }

  // 默认使用默认主题
  return 'okaidia'
}

// 设置代码复制功能
function setupCodeCopyFeature() {
  // 使用事件委托监听所有代码复制按钮的点击
  document.addEventListener('click', async (event) => {
    const target = event.target as HTMLElement
    const copyButton = target.closest('.copy-code-btn') as HTMLButtonElement

    if (copyButton) {
      try {
        const codeData = copyButton.getAttribute('data-code')
        if (!codeData) return

        const codeText = decodeURIComponent(codeData)
        await navigator.clipboard.writeText(codeText)

        // 显示复制成功状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        `
        copyButton.classList.add('text-success')
        copyButton.disabled = true

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-success')
          copyButton.disabled = false
        }, 2000)
      } catch (error) {
        console.error('复制代码失败:', error)

        // 显示复制失败状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        `
        copyButton.classList.add('text-error')

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-error')
        }, 2000)
      }
    }
  })
}

// 监听组件挂载，获取可用标签
onMounted(async () => {
  // 首先加载 Prism 语言组件
  await loadPrismLanguages()

  // 设置文档点击监听器
  setupDocumentClickListener()

  // 设置代码复制功能
  setupCodeCopyFeature()

  // 设置图片点击放大功能
  setupImageClickHandler()

  // 初始渲染
  renderMarkdown()

  // 加载保存的代码高亮主题
  const savedTheme = localStorage.getItem('mytips-highlight-theme')
  const theme = savedTheme || getDefaultHighlightTheme()
  currentHighlightTheme.value = theme
  console.log(`初始化代码高亮主题: ${theme}`)

  // 应用代码高亮主题
  setHighlightTheme(theme)

  // 设置图片懒加载
  if (isPreviewMode.value || isSplitMode.value) {
    setupImageLazyLoader()
  }

  // 加载保存的Markdown主题
  const savedMarkdownTheme = localStorage.getItem('mytips-markdown-theme')
  const markdownTheme = savedMarkdownTheme || 'github'
  currentMarkdownTheme.value = markdownTheme
  console.log(`初始化Markdown主题: ${markdownTheme}`)

  // 应用Markdown主题
  applyMarkdownTheme(markdownTheme)

  // 添加全局主题变更监听器
  const handleGlobalThemeChange = (event: CustomEvent) => {
    const { theme } = event.detail
    if (theme !== currentHighlightTheme.value) {
      console.log(`接收到全局主题变更事件: ${theme}`)
      currentHighlightTheme.value = theme
      
      // 重新应用主题
      nextTick(() => {
        // applyThemeStyles(theme) // REMOVED
        
        // 清理并重新处理代码块
        const codeBlocks = document.querySelectorAll('.prose pre code[data-enhanced]')
        codeBlocks.forEach(codeBlock => {
          codeBlock.removeAttribute('data-enhanced')
          codeBlock.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')
          
          const preElement = codeBlock.closest('pre')
          if (preElement) {
            preElement.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')
          }
        })
        
        setTimeout(() => {
          enhanceCodeBlocks()
          Prism.highlightAll()
        }, 50)
      })
    }
  }

  window.addEventListener('prism-theme-changed', handleGlobalThemeChange as EventListener)

  // 添加Markdown主题变更监听器
  const handleMarkdownThemeChange = (event: CustomEvent) => {
    const { theme } = event.detail
    if (theme !== currentMarkdownTheme.value) {
      console.log(`接收到全局Markdown主题变更事件: ${theme}`)
      currentMarkdownTheme.value = theme
      applyMarkdownTheme(theme)
    }
  }

  window.addEventListener('markdown-theme-changed', handleMarkdownThemeChange as EventListener)

  // 添加全屏状态监听器
  document.addEventListener('fullscreenchange', handleFullscreenChange)
  document.addEventListener('webkitfullscreenchange', handleFullscreenChange)
  document.addEventListener('msfullscreenchange', handleFullscreenChange)

  // 保存监听器引用以便后续清理
  ;(window as any)._prismThemeListener = handleGlobalThemeChange
  ;(window as any)._markdownThemeListener = handleMarkdownThemeChange

  // 监听系统主题变化
  if (window.matchMedia) {
    const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

    // 添加监听器以响应系统主题变化
    const themeChangeHandler = (event: MediaQueryListEvent) => {
      // 如果用户没有手动设置主题，则自动切换
      if (!localStorage.getItem('mytips-highlight-theme-manual')) {
        const newTheme = event.matches ? 'tomorrow-night' : 'default'
        setHighlightTheme(newTheme)
      }
    }

    // 根据浏览器兼容性使用不同的API
    if (darkModeMediaQuery.addEventListener) {
      darkModeMediaQuery.addEventListener('change', themeChangeHandler)
    } else if (darkModeMediaQuery.addListener) {
      // Safari和旧版浏览器支持
      darkModeMediaQuery.addListener(themeChangeHandler)
    }

    // 保存监听器引用以便后续清理
    ;(window as any)._systemThemeListener = themeChangeHandler
    ;(window as any)._darkModeMediaQuery = darkModeMediaQuery
  }

  // 在组件挂载时获取全局默认AI模型
  try {
    const defaultModel = await getDefaultAIModel('chat')
    if (defaultModel && defaultModel.provider) {
      defaultProviderId.value = defaultModel.provider
      console.log('NoteEditor: 获取全局默认AI provider:', defaultProviderId.value)
    }
  } catch (error) {
    console.error('NoteEditor: 获取默认AI模型失败:', error)
  }
})

// 强制刷新代码块主题的辅助函数
function forceRefreshCodeBlocks(theme: string) {
  console.log(`强制刷新代码块主题: ${theme}`)
  
  // 找到所有代码块
  const allCodeBlocks = document.querySelectorAll('.prose pre code')
  console.log(`找到 ${allCodeBlocks.length} 个代码块需要更新主题`)
  
  allCodeBlocks.forEach((codeBlock, index) => {
    // 移除所有主题类
    codeBlock.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')
    
    // 添加新主题类 - Prism会自动根据link加载的css文件来应用样式，我们无需手动加class
    // codeBlock.classList.add(`prism-${theme}`)
    
    // 更新父级pre元素
    const preElement = codeBlock.closest('pre')
    if (preElement) {
      preElement.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')
      // preElement.classList.add(`prism-${theme}`)
    }
    
    // console.log(`代码块 ${index + 1} 主题更新完成: ${theme}`)
  })
  
  // 重新应用 Prism 高亮
  Prism.highlightAll()
  console.log(`所有代码块主题刷新完成: ${theme}`)
}

// 修改setHighlightTheme函数，移除hljs相关代码，使用CSS变量方式
async function setHighlightTheme(theme: string) {
  console.log(`切换代码高亮主题: ${theme}`)
  currentHighlightTheme.value = theme
  localStorage.setItem('mytips-highlight-theme', theme)

  // 标记用户已手动选择主题
  localStorage.setItem('mytips-highlight-theme-manual', 'true')

  // 移除旧的动态加载的主题样式
  const existingThemeLink = document.querySelector('link[data-prism-theme]')
  if (existingThemeLink) {
    existingThemeLink.remove()
  }

  // 使用动态导入加载CSS
  try {
    switch (theme) {
      case 'default':
        // For default theme, we don't need to load any extra css.
        // The base styles should be sufficient.
        break;
      case 'okaidia':
        await import('prismjs/themes/prism-okaidia.css');
        break;
      case 'twilight':
        await import('prismjs/themes/prism-twilight.css');
        break;
      case 'solarized-light':
        await import('prismjs/themes/prism-solarizedlight.css');
        break;
      case 'tomorrow-night':
        await import('prismjs/themes/prism-tomorrow.css');
        break;
      default:
        // Default case, do nothing or load a base theme if necessary
        break;
    }
    console.log(`${theme} theme loaded successfully.`);
    Prism.highlightAll()
  } catch (error) {
    console.error(`Failed to load theme ${theme}:`, error);
  }


  // 发送全局事件，通知其他笔记编辑器实例更新主题
  window.dispatchEvent(new CustomEvent('prism-theme-changed', { 
    detail: { theme } 
  }))
}

// Markdown主题配置
const MARKDOWN_THEMES = {
  github: {
    name: 'GitHub',
    description: 'GitHub风格的Markdown主题',
    variables: {
      '--prose-body': '#24292f',
      '--prose-headings': '#1f2328',
      '--prose-lead': '#6e7781',
      '--prose-links': '#0969da',
      '--prose-bold': '#24292f',
      '--prose-counters': '#656d76',
      '--prose-bullets': '#d1d9e0',
      '--prose-hr': '#d1d9e0',
      '--prose-quotes': '#656d76',
      '--prose-quote-borders': '#d1d9e0',
      '--prose-captions': '#656d76',
      '--prose-code': '#cf222e',
      '--prose-pre-code': '#24292f',
      '--prose-pre-bg': '#f6f8fa',
      '--prose-th-borders': '#d1d9e0',
      '--prose-td-borders': '#d1d9e0',
      '--prose-invert-body': '#f0f6fc',
      '--prose-invert-headings': '#f0f6fc',
      '--prose-invert-lead': '#9198a1',
      '--prose-invert-links': '#58a6ff',
      '--prose-invert-bold': '#f0f6fc',
      '--prose-invert-counters': '#9198a1',
      '--prose-invert-bullets': '#6e7681',
      '--prose-invert-hr': '#21262d',
      '--prose-invert-quotes': '#9198a1',
      '--prose-invert-quote-borders': '#30363d',
      '--prose-invert-captions': '#9198a1',
      '--prose-invert-code': '#ff7b72',
      '--prose-invert-pre-code': '#f0f6fc',
      '--prose-invert-pre-bg': '#161b22',
      '--prose-invert-th-borders': '#21262d',
      '--prose-invert-td-borders': '#21262d'
    }
  },
  typora: {
    name: 'Typora',
    description: 'Typora经典主题风格',
    variables: {
      '--prose-body': '#333333',
      '--prose-headings': '#2c3e50',
      '--prose-lead': '#7f8c8d',
      '--prose-links': '#3498db',
      '--prose-bold': '#2c3e50',
      '--prose-counters': '#95a5a6',
      '--prose-bullets': '#bdc3c7',
      '--prose-hr': '#ecf0f1',
      '--prose-quotes': '#7f8c8d',
      '--prose-quote-borders': '#3498db',
      '--prose-captions': '#95a5a6',
      '--prose-code': '#e74c3c',
      '--prose-pre-code': '#2c3e50',
      '--prose-pre-bg': '#f8f9fa',
      '--prose-th-borders': '#bdc3c7',
      '--prose-td-borders': '#ecf0f1',
      '--prose-invert-body': '#ecf0f1',
      '--prose-invert-headings': '#ecf0f1',
      '--prose-invert-lead': '#95a5a6',
      '--prose-invert-links': '#74b9ff',
      '--prose-invert-bold': '#ecf0f1',
      '--prose-invert-counters': '#95a5a6',
      '--prose-invert-bullets': '#636e72',
      '--prose-invert-hr': '#2d3436',
      '--prose-invert-quotes': '#95a5a6',
      '--prose-invert-quote-borders': '#74b9ff',
      '--prose-invert-captions': '#95a5a6',
      '--prose-invert-code': '#fd79a8',
      '--prose-invert-pre-code': '#ecf0f1',
      '--prose-invert-pre-bg': '#2d3436',
      '--prose-invert-th-borders': '#636e72',
      '--prose-invert-td-borders': '#636e72'
    }
  },
  academic: {
    name: 'Academic',
    description: '学术论文风格主题',
    variables: {
      '--prose-body': '#1a202c',
      '--prose-headings': '#2d3748',
      '--prose-lead': '#4a5568',
      '--prose-links': '#3182ce',
      '--prose-bold': '#2d3748',
      '--prose-counters': '#718096',
      '--prose-bullets': '#cbd5e0',
      '--prose-hr': '#e2e8f0',
      '--prose-quotes': '#4a5568',
      '--prose-quote-borders': '#3182ce',
      '--prose-captions': '#718096',
      '--prose-code': '#d53f8c',
      '--prose-pre-code': '#2d3748',
      '--prose-pre-bg': '#edf2f7',
      '--prose-th-borders': '#cbd5e0',
      '--prose-td-borders': '#e2e8f0',
      '--prose-invert-body': '#f7fafc',
      '--prose-invert-headings': '#f7fafc',
      '--prose-invert-lead': '#a0aec0',
      '--prose-invert-links': '#63b3ed',
      '--prose-invert-bold': '#f7fafc',
      '--prose-invert-counters': '#a0aec0',
      '--prose-invert-bullets': '#4a5568',
      '--prose-invert-hr': '#2d3748',
      '--prose-invert-quotes': '#a0aec0',
      '--prose-invert-quote-borders': '#63b3ed',
      '--prose-invert-captions': '#a0aec0',
      '--prose-invert-code': '#f687b3',
      '--prose-invert-pre-code': '#f7fafc',
      '--prose-invert-pre-bg': '#1a202c',
      '--prose-invert-th-borders': '#4a5568',
      '--prose-invert-td-borders': '#4a5568'
    }
  },
  material: {
    name: 'Material Dark',
    description: 'Material Design暗色主题',
    variables: {
      '--prose-body': '#e0e0e0',
      '--prose-headings': '#ffffff',
      '--prose-lead': '#b0b0b0',
      '--prose-links': '#82b1ff',
      '--prose-bold': '#ffffff',
      '--prose-counters': '#9e9e9e',
      '--prose-bullets': '#616161',
      '--prose-hr': '#424242',
      '--prose-quotes': '#b0b0b0',
      '--prose-quote-borders': '#82b1ff',
      '--prose-captions': '#9e9e9e',
      '--prose-code': '#ff5722',
      '--prose-pre-code': '#e0e0e0',
      '--prose-pre-bg': '#1e1e1e',
      '--prose-th-borders': '#424242',
      '--prose-td-borders': '#424242',
      '--prose-invert-body': '#212121',
      '--prose-invert-headings': '#000000',
      '--prose-invert-lead': '#757575',
      '--prose-invert-links': '#1976d2',
      '--prose-invert-bold': '#000000',
      '--prose-invert-counters': '#616161',
      '--prose-invert-bullets': '#bdbdbd',
      '--prose-invert-hr': '#e0e0e0',
      '--prose-invert-quotes': '#757575',
      '--prose-invert-quote-borders': '#1976d2',
      '--prose-invert-captions': '#616161',
      '--prose-invert-code': '#d32f2f',
      '--prose-invert-pre-code': '#212121',
      '--prose-invert-pre-bg': '#f5f5f5',
      '--prose-invert-th-borders': '#e0e0e0',
      '--prose-invert-td-borders': '#e0e0e0'
    }
  },
  minimalist: {
    name: 'Minimalist',
    description: '极简主义风格主题',
    variables: {
      '--prose-body': '#374151',
      '--prose-headings': '#111827',
      '--prose-lead': '#6b7280',
      '--prose-links': '#1f2937',
      '--prose-bold': '#111827',
      '--prose-counters': '#9ca3af',
      '--prose-bullets': '#d1d5db',
      '--prose-hr': '#e5e7eb',
      '--prose-quotes': '#6b7280',
      '--prose-quote-borders': '#d1d5db',
      '--prose-captions': '#9ca3af',
      '--prose-code': '#111827',
      '--prose-pre-code': '#374151',
      '--prose-pre-bg': '#f9fafb',
      '--prose-th-borders': '#d1d5db',
      '--prose-td-borders': '#e5e7eb',
      '--prose-invert-body': '#d1d5db',
      '--prose-invert-headings': '#f9fafb',
      '--prose-invert-lead': '#9ca3af',
      '--prose-invert-links': '#f3f4f6',
      '--prose-invert-bold': '#f9fafb',
      '--prose-invert-counters': '#6b7280',
      '--prose-invert-bullets': '#4b5563',
      '--prose-invert-hr': '#374151',
      '--prose-invert-quotes': '#9ca3af',
      '--prose-invert-quote-borders': '#4b5563',
      '--prose-invert-captions': '#6b7280',
      '--prose-invert-code': '#f9fafb',
      '--prose-invert-pre-code': '#d1d5db',
      '--prose-invert-pre-bg': '#1f2937',
      '--prose-invert-th-borders': '#4b5563',
      '--prose-invert-td-borders': '#4b5563'
    }
  },
  elegant: {
    name: 'Elegant',
    description: '优雅经典主题',
    variables: {
      '--prose-body': '#8b5a2b',
      '--prose-headings': '#654321',
      '--prose-lead': '#a0522d',
      '--prose-links': '#cd853f',
      '--prose-bold': '#654321',
      '--prose-counters': '#daa520',
      '--prose-bullets': '#f4a460',
      '--prose-hr': '#deb887',
      '--prose-quotes': '#a0522d',
      '--prose-quote-borders': '#cd853f',
      '--prose-captions': '#daa520',
      '--prose-code': '#b22222',
      '--prose-pre-code': '#8b5a2b',
      '--prose-pre-bg': '#fdf5e6',
      '--prose-th-borders': '#deb887',
      '--prose-td-borders': '#f5deb3',
      '--prose-invert-body': '#f5deb3',
      '--prose-invert-headings': '#fff8dc',
      '--prose-invert-lead': '#daa520',
      '--prose-invert-links': '#ffd700',
      '--prose-invert-bold': '#fff8dc',
      '--prose-invert-counters': '#daa520',
      '--prose-invert-bullets': '#8b7355',
      '--prose-invert-hr': '#654321',
      '--prose-invert-quotes': '#daa520',
      '--prose-invert-quote-borders': '#ffd700',
      '--prose-invert-captions': '#daa520',
      '--prose-invert-code': '#ff6347',
      '--prose-invert-pre-code': '#f5deb3',
      '--prose-invert-pre-bg': '#2f1b14',
      '--prose-invert-th-borders': '#8b7355',
      '--prose-invert-td-borders': '#8b7355'
    }
  }
}

// 设置Markdown主题
function setMarkdownTheme(theme: string) {
  console.log(`切换Markdown主题: ${theme}`)
  currentMarkdownTheme.value = theme
  localStorage.setItem('mytips-markdown-theme', theme)

  // 应用主题样式
  applyMarkdownTheme(theme)

  // 发送全局事件，通知其他组件更新主题
  window.dispatchEvent(new CustomEvent('markdown-theme-changed', { 
    detail: { theme } 
  }))
}

// 应用Markdown主题样式
function applyMarkdownTheme(theme: string) {
  const themeConfig = MARKDOWN_THEMES[theme as keyof typeof MARKDOWN_THEMES] || MARKDOWN_THEMES.github;
  const container = fullscreenContainer.value;

  if (!container) {
    // 如果容器在初始渲染时还不可用，稍后重试
    setTimeout(() => applyMarkdownTheme(theme), 100);
    return;
  }
  
  // 将主题变量作为CSS自定义属性应用到容器元素上
  const variables = themeConfig.variables;
  Object.entries(variables).forEach(([key, value]) => {
    container.style.setProperty(key, value);
  });

  // 确保旧的动态样式表被移除（用于从旧版本平滑过渡）
  const styleElement = document.getElementById('markdown-theme-styles');
  if (styleElement) {
    styleElement.remove();
  }
  
  console.log(`已通过CSS变量应用Markdown主题: ${theme}`);
}

// 新增函数：应用主题样式
function applyThemeStyles(theme: string) {
  const themeConfig = PRISM_THEMES[theme as keyof typeof PRISM_THEMES] || PRISM_THEMES.default
  
  // 创建样式元素
  let styleElement = document.getElementById('prism-theme-styles') as HTMLStyleElement
  if (!styleElement) {
    styleElement = document.createElement('style')
    styleElement.id = 'prism-theme-styles'
    document.head.appendChild(styleElement)
  }

  // 根据主题生成CSS样式
  const cssContent = `
    /* 代码块主题样式 - ${theme} */
    .prose pre.prism-${theme} {
      background-color: ${themeConfig.background} !important;
      border: 1px solid rgba(0,0,0,0.1) !important;
      border-radius: 0.5rem !important;
      overflow: auto !important;
      margin: 1rem 0 !important;
      padding: 0 !important;
      box-shadow: 0 2px 4px rgba(0,0,0,0.05) !important;
    }

    .prose pre.prism-${theme} code {
      background: transparent !important;
      color: ${themeConfig.color} !important;
      padding: 1rem !important;
      border: none !important;
      border-radius: 0 !important;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
      font-size: 0.875rem !important;
      line-height: 1.5 !important;
      white-space: pre-wrap !important;
      word-wrap: break-word !important;
      overflow-wrap: break-word !important;
    }

    .prose .code-block-container {
      // margin: 1rem 0 !important;
      border-radius: 0.5rem !important;
      // overflow: hidden !important;
      // border: 1px solid rgba(0,0,0,0.1) !important;
      box-shadow: 0 2px 4px rgba(0,0,0,0.05) !important;
    }

    .prose .code-block-header {
      background: rgba(0,0,0,0.05) !important;
      padding: 0.5rem 1rem !important;
      display: flex !important;
      justify-content: space-between !important;
      align-items: center !important;
      border-bottom: 1px solid rgba(0,0,0,0.1) !important;
      font-size: 0.75rem !important;
    }

    .prose .code-language {
      color: rgba(0,0,0,0.6) !important;
      font-weight: 500 !important;
      text-transform: uppercase !important;
    }

    .prose .copy-code-btn {
      opacity: 0.6 !important;
      transition: opacity 0.2s ease !important;
    }

    .prose .copy-code-btn:hover {
      opacity: 1 !important;
    }

    /* 暗色主题适配 */
    ${theme === 'okaidia' || theme === 'twilight' || theme === 'tomorrow-night' ? `
      .prose .code-block-header {
        background: rgba(255,255,255,0.05) !important;
        border-bottom: 1px solid rgba(255,255,255,0.1) !important;
      }

      .prose .code-language {
        color: rgba(255,255,255,0.8) !important;
      }

      .prose pre.prism-${theme} {
        border: 1px solid rgba(255,255,255,0.1) !important;
      }

      .prose .code-block-container {
        // border: 1px solid rgba(255,255,255,0.1) !important;
      }
    ` : ''}

    /* 行号样式 */
    .prose pre.line-numbers {
      position: relative !important;
      padding-left: 3em !important;
      counter-reset: linenumber !important;
    }

    .prose pre.line-numbers > code {
      position: relative !important;
    }

    .prose pre.line-numbers .line-numbers-rows {
      position: absolute !important;
      pointer-events: none !important;
      top: 1rem !important;
      font-size: 100% !important;
      left: -3.8em !important;
      width: 3em !important;
      letter-spacing: -1px !important;
      border-right: 1px solid rgba(0,0,0,0.2) !important;
      user-select: none !important;
      ${theme === 'okaidia' || theme === 'twilight' || theme === 'tomorrow-night' ? 
        'border-right-color: rgba(255,255,255,0.2) !important;' : ''}
    }
  `

  styleElement.textContent = cssContent
  console.log(`已应用${theme}主题样式`)
}

// 添加插入表格的函数
function insertTable() {
  const tableTemplate = `| 表头1 | 表头2 | 表头3 |\n| --- | --- | --- |\n| 内容1 | 内容2 | 内容3 |\n| 内容4 | 内容5 | 内容6 |`;
  insertMarkdown(tableTemplate);
}

// 目录相关方法
function toggleToc() {
  showToc.value = !showToc.value
  if (showToc.value) {
    nextTick(() => {
      generateToc()
      updateActiveHeading()
    })
  }
}

function generateToc() {
  const preview = previewDiv.value;
  if (!preview) return;
  
  const headings = preview.querySelectorAll('h1, h2, h3, h4, h5, h6')
  tocItems.value = []
  
  headings.forEach((heading, index) => {
    const level = parseInt(heading.tagName.charAt(1))
    const text = heading.textContent || ''
    const id = `heading-${index}`
    
    heading.id = id
    
    tocItems.value.push({
      id,
      text: text.trim(),
      level
    })
  })
}

function scrollToHeading(headingId: string) {
  const preview = previewDiv.value;
  const heading = preview?.querySelector(`#${headingId}`)
  if (heading && preview) {
    const containerRect = preview.getBoundingClientRect()
    const headingRect = heading.getBoundingClientRect()
    
    const scrollTop = preview.scrollTop + headingRect.top - containerRect.top - 20
    
    preview.scrollTo({
      top: scrollTop,
      behavior: 'smooth'
    })
    
    activeHeadingId.value = headingId
  }
}

function updateActiveHeading() {
  const preview = previewDiv.value;
  if (!preview || tocItems.value.length === 0) return
  
  const containerRect = preview.getBoundingClientRect()
  const containerTop = containerRect.top + 50
  
  let activeId = ''
  
  for (const item of tocItems.value) {
    const heading = preview.querySelector(`#${item.id}`);
    if (heading) {
      const headingRect = heading.getBoundingClientRect()
      if (headingRect.top <= containerTop) {
        activeId = item.id
      } else {
        break
      }
    }
  }
  
  activeHeadingId.value = activeId
}

// 拖拽相关方法
function startDrag(event: MouseEvent | TouchEvent) {
  event.preventDefault()
  isDragging.value = true
  
  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX
  const clientY = 'touches' in event ? event.touches[0].clientY : event.clientY
  
  dragOffset.value = {
    x: clientX - tocPosition.value.x,
    y: clientY - tocPosition.value.y
  }
  
  document.addEventListener('mousemove', handleDrag)
  document.addEventListener('mouseup', stopDrag)
  document.addEventListener('touchmove', handleDrag)
  document.addEventListener('touchend', stopDrag)
}

function handleDrag(event: MouseEvent | TouchEvent) {
  if (!isDragging.value) return
  
  event.preventDefault()
  
  const clientY = 'touches' in event ? event.touches[0].clientY : event.clientY
  
  // 只更新Y坐标，保持X坐标固定
  tocPosition.value = {
    x: tocPosition.value.x, // 保持X坐标不变
    y: clientY - dragOffset.value.y
  }
  
  // 确保目录不会拖拽到屏幕外
  const maxY = window.innerHeight - 400
  
  tocPosition.value.y = Math.max(0, Math.min(tocPosition.value.y, maxY))
}

function stopDrag() {
  isDragging.value = false
  document.removeEventListener('mousemove', handleDrag)
  document.removeEventListener('mouseup', stopDrag)
  document.removeEventListener('touchmove', handleDrag)
  document.removeEventListener('touchend', stopDrag)
}

const isScrollingEditor = ref(false)
const isScrollingPreview = ref(false)

// 处理编辑器滚动事件
function handleEditorScroll(event: Event) {
  if (isScrollingPreview.value) return;

  // 标记正在从编辑器滚动
  isScrollingEditor.value = true;

  // 获取滚动元素
  const editor = event.target as HTMLTextAreaElement;
  if (!editor || !previewDiv.value || !isSplitMode.value) return;

  // 计算滚动比例
  const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight);

  // 设置预览区的滚动位置
  const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
  previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;

  // 重置标记，延迟执行防止抖动
  setTimeout(() => {
    isScrollingEditor.value = false;
  }, 100);
}

// 处理预览区滚动事件
function handlePreviewScroll(event: Event) {
  if (isScrollingEditor.value) return;

  // 标记正在从预览区滚动
  isScrollingPreview.value = true;

  // 获取滚动元素
  const preview = event.target as HTMLDivElement;
  if (!preview || !editorTextarea.value || !isSplitMode.value) return;

  // 计算滚动比例
  const previewScrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight);

  // 设置编辑器的滚动位置
  const editorScrollable = editorTextarea.value.scrollHeight - editorTextarea.value.clientHeight;
  editorTextarea.value.scrollTop = previewScrollRatio * editorScrollable;

  // 更新目录中的活跃标题
  if (showToc.value) {
    updateActiveHeading()
  }

  // 重置标记，延迟执行防止抖动
  setTimeout(() => {
    isScrollingPreview.value = false;
  }, 100);
}

// 监听内容变化时重新计算滚动同步
watch(() => localNote.value.content, (newValue, oldValue) => {
  if (newValue === oldValue) return;
  
  // 如果是切换笔记导致的内容变化，则跳过此监视器
  if (isSwitchingNote.value) {
    return;
  }

  if (renderTimeout.value) {
    clearTimeout(renderTimeout.value);
  }

  autoSave();
  
  renderTimeout.value = setTimeout(() => {
    renderMarkdown();

  // 内容变化后保持编辑器当前滚动位置的相对比例
  nextTick(() => {
    if (isSplitMode.value && editorTextarea.value && previewDiv.value) {
      const editor = editorTextarea.value;
      const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1);

      const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
      previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;
    }

    // 如果目录开启，重新生成目录
    if (showToc.value) {
      setTimeout(() => {
        generateToc()
        updateActiveHeading()
      }, 200)
    }
  });
  }, 500) as unknown as number; // 500ms 防抖
})

// 在切换模式时同步滚动位置
watch(() => isSplitMode.value, (newValue) => {
  if (newValue) {
    nextTick(() => {
      if (editorTextarea.value && previewDiv.value) {
        const editor = editorTextarea.value;
        const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1);

        const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
        previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;
      }
    });
  }
});

// 添加onActivated钩子
onActivated(() => {
  console.log('NoteEditor组件被激活')
  // 不重新加载数据，只确保编辑器状态与当前笔记同步
  if (editorTextarea.value && props.note) {
    // 仅在编辑器内容与笔记内容不一致时才更新
    const currentContent = editorTextarea.value.value
    if (currentContent !== props.note.content) {
      editorTextarea.value.value = props.note.content || ''
    }
  }
})

// 添加组件卸载时的清理逻辑
onBeforeUnmount(() => {
  console.log('NoteEditor组件即将卸载，清理资源')

  // 清理自动保存定时器
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
    autoSaveTimeout.value = null
  }

  // 清理AI流式输出相关资源
  if (globalUnlisten) {
    try {
      globalUnlisten()
      globalUnlisten = null
    } catch (e) {
      console.error('清理AI事件监听器失败:', e)
    }
  }

  // 清理图片加载相关资源
  imageLoadTimeouts.value.forEach((timeoutId) => {
    clearTimeout(timeoutId)
  })
  imageLoadTimeouts.value.clear()

  // 清理全局主题变更监听器
  const themeListener = (window as any)._prismThemeListener
  if (themeListener) {
    window.removeEventListener('prism-theme-changed', themeListener)
    delete (window as any)._prismThemeListener
  }

  // 清理Markdown主题变更监听器
  const markdownThemeListener = (window as any)._markdownThemeListener
  if (markdownThemeListener) {
    window.removeEventListener('markdown-theme-changed', markdownThemeListener)
    delete (window as any)._markdownThemeListener
  }

  // 清理系统主题变更监听器
  const systemThemeListener = (window as any)._systemThemeListener
  const darkModeMediaQuery = (window as any)._darkModeMediaQuery
  if (systemThemeListener && darkModeMediaQuery) {
    if (darkModeMediaQuery.removeEventListener) {
      darkModeMediaQuery.removeEventListener('change', systemThemeListener)
    } else if (darkModeMediaQuery.removeListener) {
      darkModeMediaQuery.removeListener(systemThemeListener)
    }
    delete (window as any)._systemThemeListener
    delete (window as any)._darkModeMediaQuery
  }

  // 清理动态创建的主题样式元素
  const themeStyleElement = document.getElementById('prism-theme-styles')
  if (themeStyleElement) {
    themeStyleElement.remove()
  }

  // 清理Markdown主题样式元素
  const markdownThemeStyleElement = document.getElementById('markdown-theme-styles')
  if (markdownThemeStyleElement) {
    markdownThemeStyleElement.remove()
  }

  // 清理可能残留的动态加载的主题CSS链接
  const existingThemeLinks = document.querySelectorAll('link[data-prism-theme]')
  existingThemeLinks.forEach(link => link.remove())

  // 清理全屏事件监听器
  document.removeEventListener('fullscreenchange', handleFullscreenChange)
  document.removeEventListener('webkitfullscreenchange', handleFullscreenChange)
  document.removeEventListener('msfullscreenchange', handleFullscreenChange)

  // 如果当前处于全屏状态，退出全屏
  if (isFullscreen.value) {
    exitFullscreen()
  }

  // 清理响应式工具栏资源
  if (resizeObserver.value) {
    resizeObserver.value.disconnect()
    resizeObserver.value = null
  }

  console.log('NoteEditor组件资源清理完成')

  // 可选：清理图片缓存（如果需要释放内存）
  // imageLoadCache.value.clear()
  window.removeEventListener('prism-theme-changed', handleThemeChange)
})

// 添加图片放大模态框的逻辑
const showImageModal = ref(false)
const modalImageSrc = ref('')
const modalImageAlt = ref('')
const modalImageLoading = ref(false)

function onModalImageLoad() {
  modalImageLoading.value = false
}

async function onModalImageError() {
  modalImageLoading.value = false
  await showAlert('图片加载失败', { title: '错误' })
}

function closeImageModal() {
  showImageModal.value = false
}

// 设置图片点击放大功能
function setupImageClickHandler() {
  // 使用事件委托监听所有图片的点击
  document.addEventListener('click', (event) => {
    const target = event.target as HTMLElement

    // 检查点击的是否是图片
    if (target.tagName === 'IMG' && target.closest('.prose')) {
      event.preventDefault()
      event.stopPropagation()

      const img = target as HTMLImageElement

      // 获取图片信息
      modalImageSrc.value = img.src
      modalImageAlt.value = img.alt || '图片'
      modalImageLoading.value = true

      // 显示模态框
      showImageModal.value = true
    }
  })

  // 添加键盘快捷键支持
  document.addEventListener('keydown', (event) => {
    // ESC键关闭图片模态框
    if (event.key === 'Escape' && showImageModal.value) {
      closeImageModal()
    }
  })
}

// 添加翻译功能
async function translateSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForTranslation.value = selectedText
  translationContent.value = ''
  showTranslationBox.value = true
  showContextMenu.value = false

  // 使用AI翻译选中的文本
  await processTranslation(selectedText)
}

// 处理翻译
async function processTranslation(text: string) {
  isTranslating.value = true
  try {
    const isEnglish = /^[a-zA-Z0-9\s.,?!;:'"()\[\]{}<>\/\\|@#$%^&*_+=-]+$/.test(text)
    const prompt = isEnglish
      ? `请将以下英文翻译成中文：\n\n${text}`
      : `请将以下中文翻译成英文：\n\n${text}`
    const streamId = `translate_${Date.now()}`
    translationContent.value = ''
    const providerId = defaultProviderId.value
    const { listen } = await import('@tauri-apps/api/event')
    let rawResult = ''
    const unlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }
      if (payload.id !== streamId) return

      // 检查后端返回的错误
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        translationContent.value = `<p class="text-error">翻译失败: ${payload.error}</p>`
        isTranslating.value = false
        unlisten()
        return
      }

      if (payload.chunk) {
        rawResult += payload.chunk
        translationContent.value = renderInlineMarkdown(rawResult)
      }
      if (payload.done) {
        isTranslating.value = false
        unlisten()
      }
    })
    await invoke('send_ai_message_stream', {
      providerId: providerId,
      message: prompt,
      streamId: streamId,
      messages: undefined,
      customModelName: undefined
    })
  } catch (error) {
    console.error('翻译失败:', error)
    translationContent.value = `<p class=\"text-error\">翻译失败: ${error}</p>`
    isTranslating.value = false
  }
}

// 复制翻译内容
async function copyTranslation() {
  // 使用临时元素提取HTML内容中的纯文本
  const tempElement = document.createElement('div')
  tempElement.innerHTML = translationContent.value
  const textContent = tempElement.textContent || ''

  // 复制到剪贴板
  try {
    await navigator.clipboard.writeText(textContent)
    // 显示成功消息
    await showAlert('翻译内容已复制到剪贴板', { title: '复制成功' })
  } catch (err) {
    console.error('复制失败:', err)
    await showAlert('复制失败，请手动选择并复制', { title: '复制失败' })
  }
}

// 将翻译结果插入到笔记内容
function insertTranslationToContent() {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 获取纯文本内容
  const tempElement = document.createElement('div')
  tempElement.innerHTML = translationContent.value
  const textContent = tempElement.textContent || ''

  // 获取当前光标位置
  const cursorPos = textarea.selectionEnd

  // 插入翻译内容
  const newContent =
    localNote.value.content.substring(0, cursorPos) +
    '\n\n' + textContent + '\n\n' +
    localNote.value.content.substring(cursorPos)

  // 更新内容
  localNote.value.content = newContent

  // 设置新的光标位置
  nextTick(() => {
    textarea.focus()
    textarea.selectionStart = textarea.selectionEnd = cursorPos + textContent.length + 4 // +4 for the newlines
  })

  // 关闭翻译框
  showTranslationBox.value = false

  // 触发自动保存
  autoSave()
}

// TIP对话框相关函数
function closeTipDialog() {
  showTipDialog.value = false
  tipPrompt.value = ''
  selectedTextForTip.value = ''
  originalTipPrompt.value = ''
  // 清理保存的位置
  delete (window as any)._tipSelectionStart
  delete (window as any)._tipSelectionEnd
}

function resetTipPrompt() {
  tipPrompt.value = originalTipPrompt.value
}

async function confirmTip(newPrompt?: string) {
  // 当对话框返回新的提示词时，更新本地状态
  if (typeof newPrompt === 'string') {
    tipPrompt.value = newPrompt;
  }

  if (!tipPrompt.value.trim()) {
    await showAlert('请输入提示词', { title: '提示' })
    return
  }

  // 获取之前保存的选择位置（用于后续插入，但TIP结果不自动插入）
  const start = (window as any)._tipSelectionStart || 0
  const end = (window as any)._tipSelectionEnd || 0

  // 关闭输入对话框，显示结果对话框
  showTipDialog.value = false

  try {
    // 调用新TIP处理函数，等待服务器响应
    await processTip(selectedTextForTip.value, tipPrompt.value)
  } catch (error) {
    console.error('TIP处理失败:', error)
    await showAlert('TIP处理失败: ' + error, { title: '错误' })
  } finally {
    // 清理状态
    tipPrompt.value = ''
    // 选中文本保留，以便插入时使用
    originalTipPrompt.value = ''
    // 清理保存的位置
    delete (window as any)._tipSelectionStart
    delete (window as any)._tipSelectionEnd
  }
}

function setTipTemplate(templateType: string) {
  const selectedText = selectedTextForTip.value
  
  const templates = {
    expand: `请基于以下内容进行详细扩充和完善，添加更多相关信息和细节：\n\n${selectedText}`,
    improve: `请改进以下内容，使其更加清晰、准确和易于理解：\n\n${selectedText}`,
    rewrite: `请重新组织和重写以下内容，保持原意但改善表达方式：\n\n${selectedText}`,
    summarize: `请总结以下内容的要点和核心信息：\n\n${selectedText}`,
    question: `请基于以下内容提出一些深入的思考问题：\n\n${selectedText}`,
    code: `请分析以下代码或技术内容，并提供详细的解释和改进建议：\n\n${selectedText}`
  }
  
  // 检查是否有用户自定义的模板
  const customTemplates = getCustomTipTemplates()
  const allTemplates = { ...templates, ...customTemplates }
  
  let selectedTemplate = allTemplates[templateType as keyof typeof allTemplates] || originalTipPrompt.value
  
  // 如果是自定义模板，替换占位符
  if (customTemplates[templateType] && selectedTemplate.includes('{{SELECTED_TEXT}}')) {
    selectedTemplate = selectedTemplate.replace('{{SELECTED_TEXT}}', selectedText)
  }
  
  tipPrompt.value = selectedTemplate
}

// 获取用户自定义的提示词模板
function getCustomTipTemplates() {
  const map: Record<string, string> = {};
  templateStore.templates.value.forEach((t: {name: string; content: string}) => {
    map[t.name] = t.content;
  });
  return map;
}

// 保存用户自定义的提示词模板
async function saveCustomTipTemplate(name: string, template: string) {
  try {
    await templateStore.addTemplate(name, template);
    console.log(`已保存自定义模板: ${name}`)
  } catch (error) {
    console.error('保存自定义模板失败:', error)
    await showAlert('保存模板失败', { title: '错误' })
  }
}

// 保存当前提示词为模板
async function saveCurrentAsTemplate() {
  if (!tipPrompt.value.trim()) {
    await showAlert('提示词不能为空', { title: '提示' })
    return
  }
  
  const templateName = prompt('请输入模板名称:')
  if (!templateName || !templateName.trim()) {
    return
  }
  
  // 将选中的文本替换为占位符，使模板可以复用
  const templateContent = tipPrompt.value.replace(selectedTextForTip.value, '{{SELECTED_TEXT}}')

  await saveCustomTipTemplate(templateName.trim(), templateContent)
  await showAlert(`模板 "${templateName.trim()}" 已保存成功！`, { title: '保存成功' })
}

// 添加粘贴功能
async function pasteFromClipboard() {
  const textarea = editorTextarea.value
  if (!textarea) return

  try {
    // 读取剪贴板内容
    const text = await navigator.clipboard.readText()

    // 获取当前光标位置
    const cursorPos = textarea.selectionStart

    // 插入剪贴板内容
    const newContent =
      localNote.value.content.substring(0, cursorPos) +
      text +
      localNote.value.content.substring(textarea.selectionEnd)

    // 更新内容
    localNote.value.content = newContent

    // 设置新的光标位置
    nextTick(() => {
      textarea.focus()
      textarea.selectionStart = textarea.selectionEnd = cursorPos + text.length
    })

    // 关闭右键菜单
    showContextMenu.value = false

    // 触发自动保存
    autoSave()
  } catch (error) {
    console.error('粘贴失败:', error)
    await showAlert('无法访问剪贴板，请使用键盘快捷键(Ctrl+V)粘贴', { title: '粘贴失败' })
  }
}

// 只更新内容
function onContentBlur() {
  emit('update', { id: localNote.value.id, content: localNote.value.content, updated_at: Date.now(), _contentOnly: true })
}

// 整个编辑器失焦时完整保存
function onEditorBlur(event: FocusEvent) {
  // 只有真正离开整个编辑器才触发
  if (!(event.currentTarget as HTMLElement).contains(event.relatedTarget as Node)) {
    saveNoteToList()
  }
}

// 全屏模式相关方法
function toggleFullscreen() {
  if (isFullscreen.value) {
    exitFullscreen()
  } else {
    enterFullscreen()
  }
}

async function enterFullscreen() {
  try {
    const container = fullscreenContainer.value
    if (!container) return

    // 使用浏览器原生全屏API
    if (container.requestFullscreen) {
      await container.requestFullscreen()
    } else if ((container as any).webkitRequestFullscreen) {
      await (container as any).webkitRequestFullscreen()
    } else if ((container as any).msRequestFullscreen) {
      await (container as any).msRequestFullscreen()
    }
    
    isFullscreen.value = true
    
    // 全屏后默认设置为分屏模式以获得最佳体验
    if (!isSplitMode.value && !isEditOnly.value && !isPreviewMode.value) {
      setEditMode('split')
    }
    
    console.log('已进入全屏模式')
  } catch (error) {
    console.error('进入全屏失败:', error)
    // 如果原生全屏失败，使用CSS全屏
    isFullscreen.value = true
  }
}

async function exitFullscreen() {
  try {
    if (document.fullscreenElement) {
      await document.exitFullscreen()
    } else if ((document as any).webkitFullscreenElement) {
      await (document as any).webkitExitFullscreen()
    } else if ((document as any).msFullscreenElement) {
      await (document as any).msExitFullscreen()
    }
    
    isFullscreen.value = false
    console.log('已退出全屏模式')
  } catch (error) {
    console.error('退出全屏失败:', error)
    // 如果原生退出全屏失败，直接设置状态
    isFullscreen.value = false
  }
}

// 监听全屏状态变化
function handleFullscreenChange() {
  const isCurrentlyFullscreen = !!(
    document.fullscreenElement ||
    (document as any).webkitFullscreenElement ||
    (document as any).msFullscreenElement
  )
  
  if (!isCurrentlyFullscreen && isFullscreen.value) {
    isFullscreen.value = false
    console.log('检测到退出全屏')
  }
}

// 全屏模式下的快捷键处理
function handleFullscreenKeyDown(event: KeyboardEvent) {
  if (!isFullscreen.value) return
  
  // F11 或 ESC 退出全屏
  if (event.key === 'F11' || event.key === 'Escape') {
    event.preventDefault()
    exitFullscreen()
    return
  }
  
  // F1 切换到编辑模式
  if (event.key === 'F1') {
    event.preventDefault()
    setEditMode('editOnly')
    return
  }
  
  // F2 切换到预览模式
  if (event.key === 'F2') {
    event.preventDefault()
    setEditMode('preview')
    return
  }
  
  // F3 切换到分屏模式
  if (event.key === 'F3') {
    event.preventDefault()
    setEditMode('split')
    return
  }
}

// 计算属性 - 检查笔记是否加密
const isNoteEncrypted = computed(() => {
  // 先检查store中是否有加密状态
  const storeResult = encryptionStore.isItemEncrypted(props.note.id)
  if (storeResult) {
    return true
  }
  
  // 如果内容是占位符，也认为是加密状态
  if (props.note.content === "[此笔记已加密，请解锁后查看]") {
    return true
  }
  
  // 检查内容是否为加密的JSON格式数据
  if (props.note.content && props.note.content.trim().startsWith('{') && props.note.content.includes('"salt"') && props.note.content.includes('"encrypted_data"')) {
    console.log('检测到加密的JSON数据格式')
    return true
  }
  
  return false
})

// 计算属性 - 检查笔记是否已解锁
const isNoteUnlocked = computed(() => {
  // 如果内容是占位符，说明未解锁
  if (props.note.content === "[此笔记已加密，请解锁后查看]") {
    return false
  }
  
  // 如果内容是加密的JSON数据，说明未解锁
  if (props.note.content && props.note.content.trim().startsWith('{') && props.note.content.includes('"salt"') && props.note.content.includes('"encrypted_data"')) {
    return false
  }
  
  // 否则检查store中的解锁状态
  return encryptionStore.isItemUnlocked(props.note.id)
})

// 处理解锁请求
function handleUnlockRequest() {
  emit('unlock-note', props.note.id)
}

// 处理解密请求
function handleDecryptRequest() {
  emit('decrypt-note', props.note.id)
}

// 监听主题变化，强制重新渲染代码块 - 简化监听器，避免重复处理
watch(() => currentHighlightTheme.value, (newTheme, oldTheme) => {
  if (newTheme !== oldTheme) {
    console.log(`主题变化监听器触发: ${oldTheme} -> ${newTheme}`)
    
    // 只有当主题变化不是由setHighlightTheme函数触发时才处理
    // setHighlightTheme函数已经处理了主题应用，这里只需要处理其他情况
    setTimeout(() => {
      // 刷新代码块
      forceRefreshCodeBlocks(newTheme)
    }, 50)
  }
}, { immediate: false })

function setupImageLazyLoader() {
  // 如果已存在观察器，先断开连接
  if (imageObserver.value) {
    imageObserver.value.disconnect()
  }

  // 确保预览容器存在
  if (!previewDiv.value) return

  const observerOptions = {
    root: previewDiv.value, // 以预览区为根
    rootMargin: '0px 0px 200px 0px', // 预加载视口下方200px的图片
    threshold: 0.01 // 元素可见度超过1%就触发
  }

  imageObserver.value = new IntersectionObserver((entries, observer) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const img = entry.target as HTMLImageElement
        const src = img.dataset.src
        if (src) {
          img.src = src
          img.classList.remove('lazy-load-image')
          img.classList.add('image-loaded')
          // 加载后停止观察
          observer.unobserve(img)
        }
      }
    })
  }, observerOptions)

  observeImages()
}

function observeImages() {
  if (!imageObserver.value || !previewDiv.value) return

  // 先断开旧的观察
  imageObserver.value.disconnect()

  // 寻找所有待加载的图片
  const imagesToLoad = previewDiv.value.querySelectorAll('img.lazy-load-image')
  imagesToLoad.forEach(img => {
    imageObserver.value!.observe(img)
  })
}

// 监听渲染内容的变化，以便在DOM更新后重新观察图片
watch(renderedContent, () => {
  if (isPreviewMode.value || isSplitMode.value) {
    nextTick(() => {
      observeImages()
    })
  }
})

// 处理来自工具栏的命令
function handleToolbarCommand(command: string, ...args: any[]) {
  switch (command) {
    case 'insert-markdown':
      insertMarkdown(args[0], args[1]);
      break;
    case 'insert-table':
      insertTable();
      break;
    case 'toggle-toc':
      toggleToc();
      break;
    case 'set-highlight-theme':
      setHighlightTheme(args[0]);
      break;
    case 'set-markdown-theme':
      setMarkdownTheme(args[0]);
      break;
    case 'set-edit-mode':
      setEditMode(args[0]);
      break;
  }
}

// 处理来自顶部栏的命令
function handleTopBarCommand(command: string) {
  switch (command) {
    case 'toggle-fullscreen':
      toggleFullscreen();
      break;
    case 'expand-with-ai':
      expandWithAI();
      break;
    case 'share-note':
      shareNote();
      break;
    case 'export-note':
      exportNote();
      break;
    case 'duplicate-note':
      emit('duplicate-note');
      break;
    case 'delete-note':
      emit('delete-note');
      break;
  }
}

// AI功能相关
async function explainWithAI() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForExplanation.value = selectedText
  explanationContent.value = ''
  showExplanationBox.value = true
  showContextMenu.value = false

  // 使用AI解释选中的文本
  await processExplanation(selectedText)
}

async function translateWithAI() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForTranslation.value = selectedText
  translationContent.value = ''
  showTranslationBox.value = true
  showContextMenu.value = false

  // 使用AI翻译选中的文本
  await processTranslation(selectedText)
}

async function tipWithAI() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  
  // 显示TIP对话框让用户修改提示词，保存选择位置
  selectedTextForTip.value = selectedText
  originalTipPrompt.value = selectedText
  tipPrompt.value = originalTipPrompt.value
  
  // 保存选择位置用于后续处理
  ;(window as any)._tipSelectionStart = start
  ;(window as any)._tipSelectionEnd = end
  
  showTipDialog.value = true
  showContextMenu.value = false
}

function handleThemeChange(event: Event) {
  const customEvent = event as CustomEvent;
  const newTheme = customEvent.detail.theme;
  if (newTheme && newTheme !== currentHighlightTheme.value) {
    console.log(`接收到全局主题变更事件，切换到: ${newTheme}`);
    setHighlightTheme(newTheme);
  }
}

// 全局默认AI提供商ID（在上方已定义并在顶层 onMounted 中赋值）
const defaultProviderId = ref<string>('gemini')

// 在组件挂载时获取全局默认AI模型
onMounted(async () => {
  try {
    const defaultModel = await getDefaultAIModel('chat')
    if (defaultModel && defaultModel.provider) {
      defaultProviderId.value = defaultModel.provider
      console.log('NoteEditor: 获取全局默认AI provider:', defaultProviderId.value)
    }
  } catch (error) {
    console.error('NoteEditor: 获取默认AI模型失败:', error)
  }
})

// TIP结果流监听器引用
let tipStreamUnlisten: (() => void) | null = null

// 处理TIP请求并生成结果
async function processTip(originalText: string, prompt: string) {
  try {
    isTipProcessing.value = true
    tipResultContent.value = ''
    showTipResultBox.value = true

    const providerId = defaultProviderId.value
    const streamId = `tip_${Date.now()}`

    // 用于累积流式内容
    let rawResult = ''

    // 监听流式返回
    const { listen } = await import('@tauri-apps/api/event')
    tipStreamUnlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }

      if (payload.id !== streamId) return

      // 错误处理
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        tipResultContent.value = `<p class="text-error">TIP生成失败: ${payload.error}</p>`
        isTipProcessing.value = false
        cleanupTipStream()
        return
      }

      if (payload.chunk) {
        rawResult += payload.chunk
        tipResultContent.value = renderInlineMarkdown(rawResult)
      }

      if (payload.done) {
        isTipProcessing.value = false
        cleanupTipStream()
      }
    })

    // 发送请求
    await invoke('send_ai_message_stream', {
      providerId,
      message: prompt,
      streamId,
      messages: undefined,
      customModelName: undefined
    })
  } catch (error) {
    console.error('TIP生成失败:', error)
    tipResultContent.value = `<p class=\"text-error\">TIP生成失败: ${error}</p>`
    isTipProcessing.value = false
    cleanupTipStream()
  }
}

// 复制TIP结果
async function copyTipResult() {
  try {
    const temp = document.createElement('div')
    temp.innerHTML = tipResultContent.value
    const textContent = temp.textContent || ''
    await navigator.clipboard.writeText(textContent)
  } catch (err) {
    console.error('复制TIP结果失败:', err)
    await showAlert('复制失败，请手动选择并复制', { title: '复制失败' })
  }
}

// 将TIP结果插入笔记
function insertTipResultToContent() {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 提取纯文本
  const temp = document.createElement('div')
  temp.innerHTML = tipResultContent.value
  const textContent = temp.textContent || ''

  const cursorPos = textarea.selectionEnd

  const newContent =
    localNote.value.content.substring(0, cursorPos) +
    '\n\n' + textContent + '\n\n' +
    localNote.value.content.substring(cursorPos)

  localNote.value.content = newContent

  nextTick(() => {
    textarea.focus()
    textarea.selectionStart = textarea.selectionEnd = cursorPos + textContent.length + 4
  })

  // 保存
  autoSave()

  // 关闭结果框
  showTipResultBox.value = false
}

// 清理TIP流监听器
function cleanupTipStream() {
  if (tipStreamUnlisten) {
    try {
      tipStreamUnlisten()
    } catch (e) {
      console.error('清理TIP事件监听器失败:', e)
    }
    tipStreamUnlisten = null
  }
}

// 关闭TIP结果弹窗
function closeTipResultBox() {
  cleanupTipStream()
  showTipResultBox.value = false
  isTipProcessing.value = false
}

// 将纯 Markdown 字符串转换为安全的 HTML，用于弹窗实时渲染
function renderInlineMarkdown(text: string): string {
  try {
    const md = new Marked()
    md.setOptions({ breaks: true, gfm: true, silent: true })

    const rawHtml = md.parse(text) as string
    return DOMPurify.sanitize(rawHtml, {
      ADD_ATTR: ['target', 'class', 'href'],
      ALLOW_DATA_ATTR: true
    })
  } catch (e) {
    console.error('Markdown 渲染失败:', e)
    // 失败时退化为纯文本
    return `<pre>${text}</pre>`
  }
}

const markdownWorker = ref<Worker | null>(null);

onMounted(() => {
  markdownWorker.value = new Worker(new URL('../workers/markdown.worker.ts', import.meta.url), { type: 'module' });

  markdownWorker.value.onmessage = (event: MessageEvent<{html?: string, error?: string}>) => {
    if (event.data.error) {
      console.error('Markdown rendering error:', event.data.error);
      renderedContent.value = `<div class="text-error">Markdown rendering error: ${event.data.error}</div>`;
      return;
    }
    if(event.data.html) {
        // 在主线程进行 HTML 清洗，避免在 Worker 中因缺少 `document` 报错
        const safeHtml = DOMPurify.sanitize(event.data.html, {
          ADD_ATTR: ['target', 'class', 'href'],
          ALLOW_DATA_ATTR: true
        });
        renderedContent.value = safeHtml;
        nextTick(() => {
            highlightCode();
            updateToc();
        });
    }
  };
});

onBeforeUnmount(() => {
  if (markdownWorker.value) {
    markdownWorker.value.terminate();
  }
});

// 渲染Markdown内容
const renderMarkdown = () => {
  if (markdownWorker.value && localNote.value) {
    // 向 Worker 传递的对象必须是可结构化克隆的数据，
    // 将可能带有 Vue Proxy 的 images 深拷贝为纯 JSON，避免 DataCloneError。
    markdownWorker.value.postMessage({
      markdown: localNote.value.content,
      images: localNote.value.images ? JSON.parse(JSON.stringify(localNote.value.images)) : undefined
    });
  }
};

const highlightCode = () => {
  const preview = document.querySelector('.markdown-preview');
  if (!preview) return;

  const blocks = preview.querySelectorAll('pre code:not([data-highlighted="true"])');
  
  const highlight = (deadline?: IdleDeadline) => {
    blocks.forEach((block, index) => {
      // If there's a deadline, check if we have time
      if (deadline && deadline.timeRemaining() <= 0 && index < blocks.length -1) {
          // Not enough time, schedule the rest for the next idle period
          requestIdleCallback(() => highlightRest(index));
          return;
      }
      Prism.highlightElement(block as HTMLElement);
      block.setAttribute('data-highlighted', 'true');
    });
  };

  const highlightRest = (startIndex: number) => {
      for (let i = startIndex; i < blocks.length; i++) {
          Prism.highlightElement(blocks[i] as HTMLElement);
          blocks[i].setAttribute('data-highlighted', 'true');
      }
  }

  if ('requestIdleCallback' in window) {
    requestIdleCallback(highlight);
  } else {
    // Fallback for browsers that don't support requestIdleCallback
    setTimeout(highlight, 0);
  }
};

const updateToc = () => {
  const container = document.querySelector('.markdown-preview');
  if (!container) return;
  const headings = container.querySelectorAll('h1, h2, h3, h4, h5, h6');
  const items: { id: string; level: number; text: string }[] = [];
  headings.forEach((heading, index) => {
    const level = parseInt(heading.tagName.substring(1), 10);
    let id = heading.id;
    if (!id) {
      id = `heading-${index}`;
      heading.id = id;
    }
    items.push({
      id,
      level,
      text: heading.textContent || '',
    });
  });
  tocItems.value = items;
};

onMounted(() => {
  markdownWorker.value = new Worker(new URL('../workers/markdown.worker.ts', import.meta.url), { type: 'module' });

  markdownWorker.value.onmessage = (event: MessageEvent<{html?: string, error?: string}>) => {
    if (event.data.error) {
      console.error('Markdown rendering error:', event.data.error);
      renderedContent.value = `<div class="text-error">Markdown rendering error: ${event.data.error}</div>`;
      return;
    }
    if(event.data.html) {
        // 在主线程进行 HTML 清洗，避免在 Worker 中因缺少 `document` 报错
        const safeHtml = DOMPurify.sanitize(event.data.html, {
          ADD_ATTR: ['target', 'class', 'href'],
          ALLOW_DATA_ATTR: true
        });
        renderedContent.value = safeHtml;
        nextTick(() => {
            highlightCode();
            updateToc();
        });
    }
  };
  
  // Other onMounted logic...
});

onBeforeUnmount(() => {
  if (markdownWorker.value) {
    markdownWorker.value.terminate();
  }
});

// Watch for content changes to trigger rendering
watch(() => localNote.value?.content, (newValue, oldValue) => {
  if (newValue !== oldValue) {
      renderMarkdown();
  }
}, { deep: true });

const templateStore = useTipTemplateStore();

</script>

<style scoped>


:deep(.line-numbers .line-numbers-rows) {
  position: absolute;
  pointer-events: none;
  top: 1rem;
  left: -3.8em;
  width: 3em;
  letter-spacing: -1px;
  border-right: 1px solid #a2a2a2;
  user-select: none;
}

:deep(.prose pre code) {
  background: transparent;
  padding: 0;
  border-radius: 0;
  font-size: var(--base-font-size-1);
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  display: block;
  width: 100%;
  line-height: 1.5;
  white-space: pre-wrap; 
  word-wrap: break-word; 
  overflow-wrap: break-word; 
  word-break: break-all;
}

/* NoteEditor特有的样式 */

/* 编辑器区域特殊样式 */
.editor-container {
  transition: all 0.3s ease;
}

.editor-toolbar {
  border-bottom: 1px solid var(--border-color);
  background: var(--background-secondary);
  padding: 8px 16px;
}

/* 代码编辑器的特殊样式 */
.code-editor {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  line-height: 1.6;
}

/* Markdown预览的特殊样式 */
.markdown-preview {
  padding: 20px;
  max-width: none;
}

/* 编辑器分割线 */
.editor-divider {
  width: 2px;
  background: var(--border-color);
  cursor: col-resize;
  transition: background-color 0.2s;
}

.editor-divider:hover {
  background: var(--primary);
}

/* 全屏模式样式 */
.fullscreen-editor {
  position: fixed !important;
  top: 0 !important;
  left: 0 !important;
  right: 0 !important;
  bottom: 0 !important;
  width: 100vw !important;
  height: 100vh !important;
  z-index: 9999 !important;
  background: var(--fallback-b1, oklch(var(--b1))) !important;
  margin: 0 !important;
  padding: 0 !important;
  border: none !important;
  border-radius: 0 !important;
  box-shadow: none !important;
}

/* 全屏模式下的容器样式 */
.fullscreen-editor .prose {
  max-width: none !important;
  padding: 2rem !important;
}

/* 全屏模式下的编辑器样式 */
.fullscreen-editor textarea {
  font-size: 1.1rem !important;
  line-height: 1.7 !important;
  padding: 2rem !important;
}

/* 全屏模式下的工具栏样式 */
.fullscreen-editor .border-b {
  border-color: var(--fallback-bc, oklch(var(--bc) / 0.2)) !important;
}

/* 全屏模式下的底部区域样式 */
.fullscreen-editor .border-t {
  border-color: var(--fallback-bc, oklch(var(--bc) / 0.2)) !important;
}

/* 全屏模式下隐藏滚动条但保持功能 */
.fullscreen-editor::-webkit-scrollbar {
  width: 8px;
}

.fullscreen-editor::-webkit-scrollbar-track {
  background: transparent;
}

.fullscreen-editor::-webkit-scrollbar-thumb {
  background: var(--fallback-bc, oklch(var(--bc) / 0.3));
  border-radius: 4px;
}

.fullscreen-editor::-webkit-scrollbar-thumb:hover {
  background: var(--fallback-bc, oklch(var(--bc) / 0.5));
}

/* 全屏模式下的响应式调整 */
@media (max-width: 768px) {
  .fullscreen-editor .prose {
    padding: 1rem !important;
  }
  
  .fullscreen-editor textarea {
    padding: 1rem !important;
    font-size: 1rem !important;
  }
  
  /* 在小屏幕上隐藏快捷键提示 */
  .fullscreen-editor .hidden.sm\\:block {
    display: none !important;
  }
}

/* 全屏模式动画 */
.fullscreen-editor {
  animation: fullscreenFadeIn 0.3s ease-out;
}

@keyframes fullscreenFadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* 工具栏按钮的特殊样式 */
.toolbar-btn {
  padding: 6px 12px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background: var(--background-hover);
  transform: translateY(-1px);
}

.toolbar-btn.active {
  background: var(--primary);
  color: var(--primary-content);
}


/* Markdown主题相关的基础样式 */
:deep(.prose) {
  /* 确保主题变量能够正确应用 */
  transition: color 0.3s ease, background-color 0.3s ease;
}

/* 标题样式增强 */
:deep(.prose h1),
:deep(.prose h2),
:deep(.prose h3),
:deep(.prose h4),
:deep(.prose h5),
:deep(.prose h6) {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  letter-spacing: -0.025em;
  scroll-margin-top: 2rem;
}

/* 段落样式增强 */
:deep(.prose p) {
  text-align: justify;
  hyphens: auto;
}

/* 链接样式增强 */
:deep(.prose a) {
  transition: color 0.2s ease;
  text-decoration-thickness: 1px;
  text-underline-offset: 2px;
}

:deep(.prose a:hover) {
  text-decoration-thickness: 2px;
}

/* 引用块样式增强 */
:deep(.prose blockquote) {
  border-radius: 0.375rem;
  background: rgba(var(--prose-quote-borders), 0.05);
  position: relative;
}

:deep(.prose blockquote::before) {
  content: '"';
  position: absolute;
  top: -0.5rem;
  left: 0.5rem;
  font-size: 3rem;
  color: var(--prose-quote-borders);
  opacity: 0.3;
  font-family: Georgia, serif;
}

/* 列表样式增强 */
:deep(.prose ul),
:deep(.prose ol) {
  padding-left: 1.5rem;
}

:deep(.prose li) {
  margin-top: 0.5rem;
  margin-bottom: 0.5rem;
}

/* 表格样式增强 */
:deep(.prose table) {
  border-collapse: collapse;
  border-spacing: 0;
  border-radius: 0.5rem;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

:deep(.prose thead th) {
  background: rgba(var(--prose-th-borders), 0.1);
  text-align: left;
}

:deep(.prose tbody tr:nth-child(even)) {
  background: rgba(var(--prose-td-borders), 0.05);
}

/* 行内代码样式 */
:deep(.prose code:not(pre code)) {
  padding: 0.125rem 0.375rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
  font-weight: 500;
  border: 1px solid rgba(var(--prose-code), 0.2);
  background: rgba(var(--prose-code), 0.1);
}

/* 确保所有图片都是响应式的 */
:deep(.prose img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  margin: 1rem auto;
  display: block;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  object-fit: contain;
}

/* 对于超大图片，设置最大高度以防止占据过多屏幕空间 */
:deep(.prose img),
:deep(.embedded-image) {
  max-height: 70vh;
  /* 最大高度为视口高度的70% */
  width: auto;
  object-fit: contain;
}

/* 在小屏幕上进一步限制图片大小 */
@media (max-width: 768px) {

  :deep(.prose img),
  :deep(.embedded-image) {
    max-height: 50vh;
    /* 在移动设备上限制为50%视口高度 */
    margin: 0.5rem auto;
  }
}

/* 图片容器样式，确保图片居中显示 */
:deep(.prose p:has(img)) {
  text-align: center;
  margin: 1rem 0;
}

/* 为图片添加加载状态和错误处理 */
:deep(.prose img) {
  transition: opacity 0.3s ease;
  cursor: zoom-in;
}

:deep(.prose img:hover) {
  opacity: 0.9;
}

/* 图片加载失败时的样式 */
:deep(.prose img[src=""]),
:deep(.prose img:not([src])) {
  display: none;
}

/* 响应式图片的额外样式 */
:deep(.responsive-image) {
  width: 100%;
  height: auto;
  max-width: 100%;
  object-fit: contain;
  border-radius: 0.5rem;
  transition: all 0.3s ease;
}

/* 图片容器的响应式布局 */
:deep(.prose) {
  overflow-wrap: break-word;
  word-wrap: break-word;
}

/* 确保图片不会破坏布局 */
:deep(.prose p) {
  overflow: hidden;
}

/* 图片加载时的占位效果 */
:deep(.prose img[loading="lazy"]) {
  background: linear-gradient(90deg, #f0f0f0 25%, transparent 37%, #f0f0f0 63%);
  background-size: 400% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
}

@keyframes shimmer {
  0% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

/* 暗色主题下的图片占位效果 */
[data-theme="dark"] :deep(.prose img[loading="lazy"]),
[data-theme="night"] :deep(.prose img[loading="lazy"]),
[data-theme="black"] :deep(.prose img[loading="lazy"]) {
  background: linear-gradient(90deg, #2a2a2a 25%, transparent 37%, #2a2a2a 63%);
  background-size: 400% 100%;
}

/* 图片模态框的响应式样式 */
.image-modal {
  backdrop-filter: blur(8px);
}

/* 确保模态框中的图片也是响应式的 */
.image-modal img {
  max-width: 95vw;
  max-height: 95vh;
  object-fit: contain;
}

/* 在极小屏幕上的特殊处理 */
@media (max-width: 480px) {

  :deep(.prose img),
  :deep(.embedded-image),
  :deep(.responsive-image) {
    max-height: 40vh;
    margin: 0.25rem auto;
  }

  .image-modal img {
    max-width: 98vw;
    max-height: 85vh;
  }
}

/* Prism 主题变量 */



/* 嵌入图片样式 */
:deep(.embedded-image) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  margin: 1rem 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  display: block;
  object-fit: contain;
}

/* 确保 Prism 高亮后的代码块也支持换行 */
:deep(.prose pre[class*="language-"]) {
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important; /* 强制在任意字符间换行 */
}

:deep(.prose pre[class*="language-"] code) {
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important; /* 强制在任意字符间换行 */
}

/* 行内代码样式 - 区别于代码块中的代码 */
:deep(.prose code:not(pre code)) {
  background-color: rgba(var(--bc), 0.1) !important;
  color: rgb(214, 51, 132) !important;
  padding: 0.125rem 0.375rem !important;
  border-radius: 0.25rem !important;
  font-size: 0.875em !important;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
  font-weight: 500 !important;
  border: 1px solid rgba(var(--bc), 0.2) !important;
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important;
  /* 修复重影问题 */
  text-shadow: none !important;
}

/* 暗色主题下的行内代码样式 */
[data-theme="dark"] :deep(.prose code:not(pre code)),
[data-theme="night"] :deep(.prose code:not(pre code)),
[data-theme="black"] :deep(.prose code:not(pre code)) {
  background-color: rgb(45, 45, 45);
  color: rgb(245, 245, 245);
  border: 1px solid rgb(75, 75, 75);
}

/* 确保行内代码在不同背景下都有良好的对比度 */
:deep(.prose p code:not(pre code)),
:deep(.prose li code:not(pre code)),
:deep(.prose td code:not(pre code)),
:deep(.prose th code:not(pre code)),
:deep(.prose blockquote code:not(pre code)) {
  background-color: rgba(175, 184, 193, 0.2);
  color: rgb(214, 51, 132);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
  font-weight: 600;
}

/* 暗色主题下的特定上下文行内代码 */
[data-theme="dark"] :deep(.prose p code:not(pre code)),
[data-theme="dark"] :deep(.prose li code:not(pre code)),
[data-theme="dark"] :deep(.prose td code:not(pre code)),
[data-theme="dark"] :deep(.prose th code:not(pre code)),
[data-theme="dark"] :deep(.prose blockquote code:not(pre code)),
[data-theme="night"] :deep(.prose p code:not(pre code)),
[data-theme="night"] :deep(.prose li code:not(pre code)),
[data-theme="night"] :deep(.prose td code:not(pre code)),
[data-theme="night"] :deep(.prose th code:not(pre code)),
[data-theme="night"] :deep(.prose blockquote code:not(pre code)),
[data-theme="black"] :deep(.prose p code:not(pre code)),
[data-theme="black"] :deep(.prose li code:not(pre code)),
[data-theme="black"] :deep(.prose td code:not(pre code)),
[data-theme="black"] :deep(.prose th code:not(pre code)),
[data-theme="black"] :deep(.prose blockquote code:not(pre code)) {
  background-color: rgba(100, 100, 100, 0.3);
  color: rgb(255, 182, 193);
}

/* TIP对话框样式 */
.tip-dialog-overlay {
  backdrop-filter: blur(4px);
  animation: fadeIn 0.2s ease-out;
}

.tip-dialog-content {
  animation: slideIn 0.3s ease-out;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideIn {
  from { 
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to { 
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

/* 快速模板按钮样式 */
.template-btn {
  transition: all 0.2s ease;
  border: 1px solid hsl(var(--bc) / 0.2);
}

.template-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  border-color: hsl(var(--primary));
}

/* 提示词编辑区样式 */
.tip-prompt-textarea {
  transition: border-color 0.2s ease;
  resize: vertical;
  min-height: 120px;
  max-height: 300px;
}

.tip-prompt-textarea:focus {
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
}

/* 选中文本显示区域样式 */
.selected-text-display {
  border-left: 4px solid hsl(var(--primary));
  background: linear-gradient(90deg, hsl(var(--primary) / 0.1), transparent);
}

/* 字符计数样式 */
.char-count {
  font-variant-numeric: tabular-nums;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

/* 链接中的行内代码样式 */
:deep(.prose a code:not(pre code)) {
  color: inherit;
  background-color: rgba(0, 0, 0, 0.1);
}

[data-theme="dark"] :deep(.prose a code:not(pre code)),
[data-theme="night"] :deep(.prose a code:not(pre code)),
[data-theme="black"] :deep(.prose a code:not(pre code)) {
  background-color: rgba(255, 255, 255, 0.2);
}

/* 目录相关样式 */
.toc-container {
  backdrop-filter: blur(8px);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  border: 1px solid hsl(var(--bc) / 0.1);
  user-select: none;
  transition: all 0.2s ease;
  right: 20px !important;
  left: auto !important;
  z-index: 1000;
  width: 200px;
  position: fixed !important;
  background-color: var(--fallback-b1, oklch(var(--b1))) !important;
  opacity: 0.95;
}

.toc-container:hover {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
}

.toc-header {
  cursor: grab;
}

.toc-header:active {
  cursor: grabbing;
}

.toc-item {
  transition: all 0.15s ease;
  border-radius: 0.25rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.toc-item:hover {
  background-color: hsl(var(--bc) / 0.05);
  transform: translateX(2px);
}

.toc-item.active {
  background-color: hsl(var(--primary) / 0.1);
  color: hsl(var(--primary));
  border-left: 2px solid hsl(var(--primary));
  padding-left: calc(0.5rem - 2px);
}

/* 目录层级缩进视觉效果 */
.toc-item[style*="padding-left: 12px"] {
  border-left: 1px solid hsl(var(--bc) / 0.1);
}

.toc-item[style*="padding-left: 24px"] {
  border-left: 1px solid hsl(var(--bc) / 0.1);
  position: relative;
}

.toc-item[style*="padding-left: 24px"]::before {
  content: '';
  position: absolute;
  left: 12px;
  top: 0;
  bottom: 0;
  width: 1px;
  background: hsl(var(--bc) / 0.1);
}

.toc-item[style*="padding-left: 36px"] {
  border-left: 1px solid hsl(var(--bc) / 0.1);
  position: relative;
}

.toc-item[style*="padding-left: 36px"]::before {
  content: '';
  position: absolute;
  left: 12px;
  top: 0;
  bottom: 0;
  width: 1px;
  background: hsl(var(--bc) / 0.1);
}

.toc-item[style*="padding-left: 36px"]::after {
  content: '';
  position: absolute;
  left: 24px;
  top: 0;
  bottom: 0;
  width: 1px;
  background: hsl(var(--bc) / 0.1);
}

/* 滚动条样式 */
.toc-container .overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.toc-container .overflow-y-auto::-webkit-scrollbar-track {
  background: transparent;
}

.toc-container .overflow-y-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--bc) / 0.2);
  border-radius: 2px;
}

.toc-container .overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--bc) / 0.3);
}

/* 拖拽时的样式 */
.toc-container.dragging {
  transform: rotate(2deg);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.4);
}

/* 修复代码块样式冲突 */
:deep(.prose pre) {
  background: transparent !important;
  padding: 0 !important;
  margin: 1rem 0 !important;
  border-radius: 0.5rem !important;
  overflow: hidden !important;
}

:deep(.prose pre code) {
  background: transparent !important;
  padding: 1rem !important;
  border: none !important;
  border-radius: 0 !important;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
  font-size: 0.875rem !important;
  line-height: 1.5 !important;
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important;
  display: block !important;
  width: 100% !important;
  /* 修复重影问题 */
  text-shadow: none !important;
  font-weight: normal !important;
}

/* 修复代码块容器样式 */
:deep(.prose .code-block-container) {
  /* margin: 1rem 0 !important; */
  border-radius: 0.5rem !important;
  /* overflow: hidden !important; */
  /* border: 1px solid rgba(var(--bc), 0.1) !important; */
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05) !important;
}

:deep(.prose .code-block-header) {
  background: rgba(var(--bc), 0.05) !important;
  padding: 0.5rem 1rem !important;
  display: flex !important;
  justify-content: space-between !important;
  align-items: center !important;
  border-bottom: 1px solid rgba(var(--bc), 0.1) !important;
  font-size: 0.75rem !important;
}

:deep(.prose .code-language) {
  color: rgba(var(--bc), 0.6) !important;
  font-weight: 500 !important;
  text-transform: uppercase !important;
}

:deep(.prose .copy-code-btn) {
  opacity: 0.6 !important;
  transition: opacity 0.2s ease !important;
}

:deep(.prose .copy-code-btn:hover) {
  opacity: 1 !important;
}

/* 暗色主题下的代码块头部适配 */
[data-theme="dark"] :deep(.prose .code-block-header),
[data-theme="night"] :deep(.prose .code-block-header),
[data-theme="black"] :deep(.prose .code-block-header) {
  background: rgba(255, 255, 255, 0.05) !important;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
}

[data-theme="dark"] :deep(.prose .code-language),
[data-theme="night"] :deep(.prose .code-language),
[data-theme="black"] :deep(.prose .code-language) {
  color: rgba(255, 255, 255, 0.8) !important;
}

</style>
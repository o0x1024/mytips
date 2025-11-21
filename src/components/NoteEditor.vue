<template>
  <div class="h-full flex flex-col" v-if="note">
    <!-- 加密内容视图 -->
    <div v-if="isNoteEncrypted && !isNoteUnlocked" class="h-full">
      <EncryptedContent 
        :title="t('noteEditor.encryptedNoteTitle', { title: note.title })"
        :description="t('noteEditor.encryptedNoteDescription')"
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
          <!-- Tiptap 富文本编辑器 -->
          <TiptapEditor
            :key="note.id"
            v-model="localNote.content"
            @update:modelValue="handleEditorModelUpdate"
            :note-id="note.id"
            :images="localNote.images"
            :placeholder="t('markdownEditor.placeholder')"
            mode="editOnly"
            ref="tiptapEditor"
            @contextmenu="handleContextMenu"
            @paste="handlePaste"
            @add-image="handleImageAdded"
            @keydown="handleKeyDown"
          />


          <!-- 右键菜单 -->
          <div v-if="showContextMenu"
            class="context-menu absolute bg-base-200 text-base-content rounded-md shadow-lg p-2 z-30"
            :style="{ top: contextMenuY + 'px', left: contextMenuX + 'px' }">
            <ul class="menu menu-sm p-0">
              <li>
                <a @click="copySelectedText" :class="{ 'disabled': !hasSelectedText }">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                  {{ t('common.copy') }}
                </a>
              </li>
              <li>
                <a @click="pasteFromClipboard">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                  </svg>
                  {{ t('common.paste') }}
                </a>
              </li>
              <li class="menu-title"><span></span></li>
              <li>
                <a @click="explainWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
                  </svg>
                  {{ t('noteEditor.aiExplain') }}
                </a>
              </li>
              <li>
                <a @click="translateWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
                  </svg>
                  {{ t('noteEditor.aiTranslate') }}
                </a>
              </li>
              <li>
                <a @click="tipWithAI" :class="{ 'disabled': !hasSelectedText || isAIProcessing }">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                  {{ t('noteEditor.tipWithAI') }}
                </a>
              </li>
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
                  {{ t('noteEditor.toc') }}
                </h3>
              <button @click="showToc = false" class="btn btn-xs btn-ghost btn-square" @mousedown.stop @touchstart.stop>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>
              <div class="overflow-y-auto overflow-x-hidden" style="max-height: 320px;">
                <ul class="space-y-1 w-full">
                <li v-for="(item, index) in tocItems" :key="item.id || index" :style="{ paddingLeft: (item.level - 1) * 12 + 'px' }" class="text-sm overflow-hidden">
                  <a @click.prevent="scrollToHeading(item.id)" @mousedown.stop @touchstart.stop class="toc-item block py-1 px-2 text-base-content/80 cursor-pointer" :class="{ 'active': item.id === activeHeadingId }" :title="item.text">
                      {{ item.text }}
                    </a>
                  </li>
                </ul>
            </div>
          </div>

          <!-- 右键AI操作全局Loading提示（等待服务端首个响应时显示） -->
          <div
            v-if="(isExplaining && !explanationContent) || (isTranslating && !translationContent) || (isTipProcessing && !tipResultContent)"
            class="absolute top-2 right-2 z-20 flex items-center gap-2 bg-base-200/90 rounded px-2 py-1 border border-base-300 shadow"
          >
            <span class="loading loading-spinner loading-sm"></span>
          </div>
        </div>

        <!-- 底部元数据区域 -->
        <EditorFooter
          class="editor-footer"
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

  <!-- 音频录制组件 -->
  <AudioRecorder
    :visible="showAudioRecorder"
    :note-id="localNote.id"
    @close="showAudioRecorder = false"
    @audio-inserted="handleAudioInserted"
  />

  <!-- 音频播放器组件 -->
  <AudioPlayer
    :show="showAudioPlayer"
    :tip-id="localNote.id"
    @close="showAudioPlayer = false"
    @insert-audio="handleAudioPlayerInsert"
  />

  <!-- Image Zoom Modal -->
  <div v-if="showImageModal" class="fixed inset-0 bg-black bg-opacity-70 flex items-center justify-center z-[99999] image-modal" @click="closeImageModal">
    <div class="relative w-full h-full overflow-hidden">
      <img 
        :src="modalImageSrc" 
        :alt="modalImageAlt" 
        class="absolute top-1/2 left-1/2 object-contain transition-transform duration-200 ease-out"
        @load="modalImageLoading = false"
        @wheel.prevent="handleZoom"
        @mousedown="startPan"
        @click.stop
        :class="{ 'cursor-grab': zoomLevel > 1, 'active:cursor-grabbing': isPanning }"
        :style="{ 
          transform: `translate(-50%, -50%) scale(${zoomLevel}) translate(${panX}px, ${panY}px)`,
          maxWidth: 'none',
          maxHeight: 'none',
        }"
      />
      <div v-if="modalImageLoading" class="absolute inset-0 flex items-center justify-center">
        <span class="loading loading-spinner loading-lg text-white"></span>
      </div>
    </div>
    
    <button @click.stop="closeImageModal" class="absolute top-4 right-4 text-white text-3xl btn btn-ghost btn-circle z-10">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
    </button>
    
    <div @click.stop class="absolute bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-2 bg-base-300/80 backdrop-blur-sm p-2 rounded-lg shadow-lg z-10">
      <button @click.stop="zoomOut" class="btn btn-sm btn-ghost">-</button>
      <input type="range" min="0.5" max="5" step="0.1" v-model.number="zoomLevel" @input="onZoomSliderChange" class="range range-xs w-40" />
      <button @click.stop="zoomIn" class="btn btn-sm btn-ghost">+</button>
      <div class="divider divider-horizontal mx-0"></div>
      <button @click.stop="resetZoom" class="btn btn-sm btn-ghost">{{ t('common.reset') }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, defineProps, defineEmits, nextTick, onMounted, onActivated, onBeforeUnmount, onDeactivated } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import EncryptedContent from './EncryptedContent.vue'
import EditorToolbar from './EditorToolbar.vue'
import EditorTopBar from './EditorTopBar.vue'
import EditorFooter from './EditorFooter.vue'
import TiptapEditor from './TiptapEditor.vue'
import AIExplanationDialog from './dialogs/AIExplanationDialog.vue'
import AITranslationDialog from './dialogs/AITranslationDialog.vue'
import TipInputDialog from './dialogs/TipInputDialog.vue'
import TipResultDialog from './dialogs/TipResultDialog.vue'
import AudioRecorder from './audio/AudioRecorder.vue'
import AudioPlayer from './audio/AudioPlayer.vue'
import { showAlert } from '../services/dialog'
import { useEncryptionStore } from '../stores/encryptionStore'
import { getDefaultAIModel } from '../services/aiService'
import { useAIStore } from '../stores/aiStore'
import Prism from 'prismjs'
// Import prism styles and plugins
import 'prismjs/plugins/line-numbers/prism-line-numbers.css'
import 'prismjs/plugins/line-numbers/prism-line-numbers'
import 'prismjs/plugins/toolbar/prism-toolbar.css'
import 'prismjs/plugins/toolbar/prism-toolbar'
// import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard' // 禁用复制按钮插件以避免不必要的插入
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
// import { diff_match_patch as DiffMatchPatch } from 'diff-match-patch'; // 不再使用，改用简单的快照式撤销/重做
import { LRUCache } from 'lru-cache'
import { useTipTemplateStore } from '../stores/tipTemplateStore'
import { useLocalStorageStore } from '../stores/localStorageStore'
import { useI18n } from 'vue-i18n'
import { renderMarkdown, renderInlineMarkdown } from '../services/markdownService'


const { t } = useI18n()
const localStorageStore = useLocalStorageStore()

// Image Zoom/Pan state
const zoomLevel = ref(0.7)
const panX = ref(0)
const panY = ref(0)
const isPanning = ref(false)
const panStart = ref({ x: 0, y: 0 })
const imageStart = ref({ x: 0, y: 0 })

const savedMode = localStorageStore.data.editorMode || 'split';


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
const isPreviewMode = ref(savedMode === 'preview')
const tiptapEditor = ref<any>(null);
// const noteImages = ref<Record<string, string>>({});
// Legacy editor references (deprecated, kept for compatibility)
const editorTextarea = computed(() => null as HTMLTextAreaElement | null);
const previewDiv = computed(() => null as HTMLDivElement | null);
const autoSaveTimeout = ref<number | null>(null)
const renderTimeout = ref<number | null>(null)
const renderedContent = ref('')
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const isAIProcessing = ref(false)
const isEditOnly = ref(savedMode === 'editOnly')
const isSplitMode = ref(savedMode === 'split')
// WYSIWYG mode removed
const isSwitchingNote = ref(false)
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
  if (!tiptapEditor.value) return false
  const selectedText = tiptapEditor.value.getSelectedText?.()
  return selectedText && selectedText.length > 0
})
const showExplanationBox = ref(false)
const explanationContent = ref('') // 渲染后的HTML内容
const explanationRawContent = ref('') // 原始文本内容
const selectedTextForExplanation = ref('')
const isExplaining = ref(false)
const showTranslationBox = ref(false)
const translationContent = ref('') // 渲染后的HTML内容
const translationRawContent = ref('') // 原始文本内容
const selectedTextForTranslation = ref('')
const isTranslating = ref(false)
// 添加TIP对话框相关状态
const showTipDialog = ref(false)
const tipPrompt = ref('')
const selectedTextForTip = ref('')
const originalTipPrompt = ref('')
// TIP结果弹窗相关状态
const showTipResultBox = ref(false)
const tipResultContent = ref('') // 渲染后的HTML内容
const tipResultRawContent = ref('') // 原始文本内容
const isTipProcessing = ref(false)

// 音频录制相关状态
const showAudioRecorder = ref(false)
const showAudioPlayer = ref(false)

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
const undoStack = ref<string[]>([])
const redoStack = ref<string[]>([])
const lastSavedContent = ref<string>('')

// 为自动保存机制引入独立的状态
const lastEmittedTitle = ref<string>('')
const lastEmittedContent = ref<string>('')

// 动态响应式工具栏相关函数
function initResponsiveToolbar() {
  if (typeof window === 'undefined' || !toolbarContainer.value || !toolbarLeft.value || !toolbarRight.value) return
  
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
  initResponsiveToolbar()
})

onBeforeUnmount(() => {
  // The worker has been removed, so no termination logic is needed.
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

    // 检查当前笔记是否還是目标笔记（避免切换过快导致的状态错乱）
    if (localNote.value.id === noteId && images && Object.keys(images).length > 0) {
      localNote.value.images = images
      console.log(`异步加载完成，笔记(${noteId})图片已更新到本地状态，触发重新渲染`)
      // 图片加载完成后，再次渲染以显示图片
      render()
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

    // 初始化/重置自动保存的状态
    lastEmittedTitle.value = localNote.value.title;
    lastEmittedContent.value = localNote.value.content;

    // 清除可能存在的延迟渲染
    if (renderTimeout.value) {
        clearTimeout(renderTimeout.value);
    }
    
    // 如果笔记有ID但没有images数据，先同步/异步加载图片（不阻塞界面太久）
    if (newNote.id && !newNote.images) {
      // 尝试快速同步加载缓存的图片，否则异步加载
      const cachedImages = await loadNoteImages(newNote.id, 500) // 500ms快速超时用于缓存
      if (cachedImages && Object.keys(cachedImages).length > 0) {
        // 缓存中有图片，立即使用
        localNote.value.images = cachedImages
        console.log(`使用缓存的笔记(${newNote.id})图片进行渲染`)
      } else {
        // 缓存中没有，设置为空对象，然后异步加载
        localNote.value.images = {}
        console.log(`缓存无图片或超时，笔记(${newNote.id})将异步加载图片`)
      }
    }
    
    // 现在用当前的图片数据进行渲染（可能是缓存的，也可能是空对象）
    render();

    // 如果图片仍然是空的，继续异步加载完整的图片
    if (newNote.id && (!localNote.value.images || Object.keys(localNote.value.images).length === 0)) {
      // 在后台继续加载更多/全部图片，完成后会触发第二次渲染
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
      const isCurrentPlaceholder = localNote.value.content === t('noteEditor.encryptedPlaceholder')
      const isNewContentDecrypted = newNote.content !== t('noteEditor.encryptedPlaceholder') && 
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
        oldState.noteContent === t('noteEditor.encryptedPlaceholder') &&
        newState.noteContent !== t('noteEditor.encryptedPlaceholder') &&
        newState.isEncrypted &&
        newState.isUnlocked) {
      console.log('NoteEditor: 检测到内容从占位符变为解密内容')
      localNote.value.content = newState.noteContent;
    }
    
    // 如果当前本地内容是占位符，但传入的内容是解密后的内容，也要更新
    if (localNote.value.content === t('noteEditor.encryptedPlaceholder') &&
        newState.noteContent !== t('noteEditor.encryptedPlaceholder') &&
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
    insertMarkdown('[', `](${t('noteEditor.linkUrlPlaceholder')})`)
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
      // 保存当前内容状态为一个快照，方便撤销恢复
      undoStack.value.push(lastSavedContent.value)
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
  if (undoStack.value.length === 0) {
    console.warn("没有可撤销的操作")
    return
  }

  // 保存当前内容到重做堆栈
  const currentContent = localNote.value.content
  redoStack.value.push(currentContent)

  // 恢复到上一个保存的状态
  const previousContent = undoStack.value.pop()
  if (previousContent !== undefined) {
    localNote.value.content = previousContent
    lastSavedContent.value = previousContent
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
  if (redoStack.value.length === 0) {
    console.warn("没有可重做的操作")
    return
  }

  // 保存当前内容到撤销堆栈
  const currentContent = localNote.value.content
  undoStack.value.push(currentContent)

  // 恢复到下一个状态
  const nextContent = redoStack.value.pop()
  if (nextContent !== undefined) {
    localNote.value.content = nextContent
    lastSavedContent.value = nextContent
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

    // 如果没有指定语言，为code元素添加language-plaintext类
    if (!langClass) {
      codeElement.classList.add('language-plaintext')
    }

    // 不再额外添加主题类，依靠全局 CSS 文件实现主题颜色
    // ---- 新增：为代码块添加容器及行号支持 ----
    const container = document.createElement('div')
    container.className = 'code-block-container'

    // Prism 的 line-numbers 插件行号
    pre.classList.add('line-numbers')

    // 避免重复包装
    const parent = pre.parentNode
    if (parent) {
      parent.insertBefore(container, pre)
      container.appendChild(pre)
    }
    // ---- 新增结束 ----
  })
}




// 方法
function autoSave() {
  // 防抖自动保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }

  autoSaveTimeout.value = setTimeout(() => {
    const titleChanged = localNote.value.title !== lastEmittedTitle.value;
    const contentChanged = localNote.value.content !== lastEmittedContent.value;

    if (!titleChanged && !contentChanged) {
      return; // 没有变化，无需保存
    }
    
    localNote.value.updated_at = Date.now()

    const payload: any = { ...localNote.value };
    if (titleChanged && !contentChanged) {
      payload._titleOnly = true;
    } else if (contentChanged && !titleChanged) {
      payload._contentOnly = true;
    }
    // 如果两者都已更改，则不发送标志，触发完整更新

    emit('update', payload)

    // 更新最后发出的状态
    lastEmittedTitle.value = localNote.value.title;
    lastEmittedContent.value = localNote.value.content;

  }, 1000) as unknown as number
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
  
  // 手动保存后，也更新自动保存的状态
  lastEmittedTitle.value = localNote.value.title
  lastEmittedContent.value = localNote.value.content
}

function insertMarkdown(_prefix: string, _suffix: string = '') {
  // No longer needed with Tiptap - use toolbar commands instead
  console.warn('insertMarkdown is deprecated, use Tiptap toolbar commands')
  return
}

function handleContextMenu(event: MouseEvent) {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 防止默认菜单显示 - 已注释，允许使用系统右键菜单
  // event.preventDefault()
  // event.stopPropagation() // 阻止事件冒泡


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
    const providerId = await getActualProviderId()
    console.log('AI扩充功能使用的providerId:', providerId, '原始defaultProviderId:', defaultProviderId.value)

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


function setEditMode(mode: string) {
  localStorageStore.setEditorMode(mode);
  isEditOnly.value = false;
  isPreviewMode.value = false;
  isSplitMode.value = false;

  if (mode === 'editOnly') {
    isEditOnly.value = true
  } else if (mode === 'preview') {
    isPreviewMode.value = true
  } else if (mode === 'split') {
    isSplitMode.value = true
  }

  // 模式切换后重新应用代码块主题
  nextTick(() => {
    forceRefreshCodeBlocks(currentHighlightTheme.value)
  })
}

// 复制选中的文本
async function copySelectedText() {
  if (!tiptapEditor.value) return

  const selectedText = tiptapEditor.value.getSelectedText?.()
  if (selectedText && selectedText.length > 0) {
    try {
      await navigator.clipboard.writeText(selectedText)
      showContextMenu.value = false
    } catch (error) {
      console.error('Copy to clipboard failed:', error)
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

// 处理图片添加事件
function handleImageAdded({ id, data }: { id: string, data: string }) {
  if (!localNote.value.images) {
    localNote.value.images = {}
  }
  localNote.value.images[id] = data
  
  // 触发自动保存以持久化图片关联（虽然图片本身已保存，但localNote状态需要更新）
  autoSave()
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
    // 重置内容
    explanationContent.value = ''
    explanationRawContent.value = ''

    // 使用全局默认AI提供商
    const providerId = await getActualProviderId()
    console.log('AI解释功能使用的providerId:', providerId, '原始defaultProviderId:', defaultProviderId.value)

    // 创建唯一的流ID
    const streamId = `explain_${Date.now()}`

    // 构建包含笔记标题的提示信息
    const noteTitle = localNote.value.title || '无标题笔记'
    const prompt = `请解释以下内容，简明扼要。这段内容来自笔记"${noteTitle}"：\n\n${textToExplain}`

    // 设置事件监听器来接收流式响应
    const { listen } = await import('@tauri-apps/api/event')
    const unlisten = await listen('ai-stream-chunk', async (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }

      // 确保ID匹配
      if (payload.id !== streamId) return

      // 检查后端返回的错误
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        explanationContent.value = `<p class="text-error">解释生成失败: ${payload.error}</p>`
        explanationRawContent.value = `解释生成失败: ${payload.error}`
        isExplaining.value = false
        unlisten()
        return
      }

      if (payload.chunk) {
        // 首个chunk到达即关闭loading
        isExplaining.value = false
        // 累积原始内容
        explanationRawContent.value += payload.chunk
        // 渲染HTML内容用于显示
        explanationContent.value = await renderInlineMarkdown(explanationRawContent.value)
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
    // 移除think标签内容
    const cleanText = explanationRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()
    await navigator.clipboard.writeText(cleanText)
    // 可以添加一个复制成功的提示
  } catch (error) {
    console.error('复制到剪贴板失败:', error)
  }
}

// 将解释内容插入到笔记中
function insertExplanationToContent() {
  if (!tiptapEditor.value) return

  // 使用原始内容并移除think标签
  const cleanText = explanationRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()
  
  const content = '\n\n> 💡 解释：\n\n' + cleanText

  // 插入内容到编辑器
  tiptapEditor.value.insertAtCursor?.(content)
  tiptapEditor.value.focus?.()

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

// 在组件属性下添加以下状态变量
const currentHighlightTheme = ref(localStorageStore.data.highlightTheme || getDefaultHighlightTheme())
const currentMarkdownTheme = ref(localStorageStore.data.markdownTheme || 'github')

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
            <path d="M5 13l4 4L19 7" />
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
  render()

  // 加载保存的代码高亮主题
  const savedTheme = localStorageStore.data.highlightTheme
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
  const savedMarkdownTheme = localStorageStore.data.markdownTheme
  const markdownTheme = savedMarkdownTheme || 'github'
  currentMarkdownTheme.value = markdownTheme
  console.log(`初始化Markdown主题: ${markdownTheme}`)

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




  // 添加全屏状态监听器
  document.addEventListener('fullscreenchange', handleFullscreenChange)
  document.addEventListener('webkitfullscreenchange', handleFullscreenChange)
  document.addEventListener('msfullscreenchange', handleFullscreenChange)

  // 保存监听器引用以便后续清理
  ;(window as any)._prismThemeListener = handleGlobalThemeChange

  // 监听系统主题变化
  if (window.matchMedia) {
    const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

    // 添加监听器以响应系统主题变化
    const themeChangeHandler = (event: MediaQueryListEvent) => {
      // 如果用户没有手动设置主题，则自动切换
      if (!localStorageStore.data.highlightThemeManual) {
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
      console.log('NoteEditor: 获取全局默认AI provider:', defaultModel.provider)
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
  
  allCodeBlocks.forEach((codeBlock) => {
    // 移除所有主题类
    codeBlock.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')

    // 更新父级pre元素
    const preElement = codeBlock.closest('pre')
    if (preElement) {
      preElement.classList.remove('prism-default', 'okaidia', 'twilight', 'solarized-light', 'tomorrow-night')
    }
    
  })
  
  // 重新应用 Prism 高亮
  Prism.highlightAll()
  console.log(`所有代码块主题刷新完成: ${theme}`)
}

// 修改setHighlightTheme函数，移除hljs相关代码，使用CSS变量方式
async function setHighlightTheme(theme: string) {
  console.log(`切换代码高亮主题: ${theme}`);

  currentHighlightTheme.value = theme;
  localStorageStore.setHighlightTheme(theme);
  // 标记用户已手动选择主题
  localStorageStore.setHighlightThemeManual(true);

  // 动态获取对应主题的 CSS 资源 URL（通过 ?url 让构建工具返回文件路径而不是自动插入 style）
  const themeUrlLoaders: Record<string, () => Promise<string>> = {
    default: async () => (await import('prismjs/themes/prism.css?url')).default,
    okaidia: async () => (await import('prismjs/themes/prism-okaidia.css?url')).default,
    twilight: async () => (await import('prismjs/themes/prism-twilight.css?url')).default,
    'solarized-light': async () => (await import('prismjs/themes/prism-solarizedlight.css?url')).default,
    'tomorrow-night': async () => (await import('prismjs/themes/prism-tomorrow.css?url')).default,
  };

  try {
    const getUrl = themeUrlLoaders[theme];
    if (!getUrl) {
      console.warn(`未找到主题 ${theme} 的 CSS。`);
      return;
    }

    const cssHref = await getUrl();

    // 处理 <link> 标签，按需创建或更新
    let linkEl = document.getElementById('prism-theme-link') as HTMLLinkElement | null;
    if (!linkEl) {
      linkEl = document.createElement('link');
      linkEl.id = 'prism-theme-link';
      linkEl.rel = 'stylesheet';
      document.head.appendChild(linkEl);
    }

    linkEl.href = cssHref;

    console.log(`已加载 Prism 主题 CSS: ${cssHref}`);

    // 重新高亮
    Prism.highlightAll();
  } catch (error) {
    console.error(`加载 Prism 主题 ${theme} 失败:`, error);
  }

  // 发送全局事件，通知其他笔记编辑器实例更新主题
  window.dispatchEvent(new CustomEvent('prism-theme-changed', { detail: { theme } }));
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
  // For TiptapEditor, get headings from the editor's DOM
  const editorElement = tiptapEditor.value?.$el?.querySelector('.ProseMirror') || 
                        tiptapEditor.value?.$el?.querySelector('.tiptap');
  
  if (!editorElement) {
    console.warn('Cannot find editor element for TOC generation');
    return;
  }
  
  const headings = editorElement.querySelectorAll('h1, h2, h3, h4, h5, h6')
  const newToc: { id: string; level: number; text: string }[] = []

  headings.forEach((heading: Element, index: number) => {
    const level = parseInt(heading.tagName.charAt(1))
    const text = (heading.textContent || '').trim()
    if (!text) return

    // Generate or use existing id
    let id = (heading as HTMLElement).id
    if (!id) {
      // Create a URL-friendly id from the text
      id = `heading-${text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')}-${index}`;
      (heading as HTMLElement).id = id
    }

    newToc.push({ id, level, text })
  })

  tocItems.value = newToc
}

// 编辑器内容更新的直接处理，确保分屏模式下即时预览
function handleEditorModelUpdate(val: string) {
  // 快速更新本地内容（v-model 已经做了，但这里作为显式入口保证时序一致）
  if (localNote.value.content !== val) {
    localNote.value.content = val
  }
  // 立即调度一次轻量渲染，避免重启后首次输入不触发预览
  if (renderTimeout.value) {
    clearTimeout(renderTimeout.value)
  }
  // 使用更短的延时确保顺滑
  renderTimeout.value = setTimeout(() => {
    render()
    // Update TOC if visible
    if (showToc.value) {
      nextTick(() => {
        generateToc()
      })
    }
  }, 50) as unknown as number
}


function scrollToHeading(headingId: string) {
  // For TiptapEditor, scroll within the editor container
  const editorElement = tiptapEditor.value?.$el?.querySelector('.ProseMirror') || 
                        tiptapEditor.value?.$el?.querySelector('.tiptap');
  const editorContainer = tiptapEditor.value?.$el?.querySelector('[class*="overflow"]');
  
  if (!editorElement) return;
  
  const heading = editorElement.querySelector(`#${CSS.escape(headingId)}`);
  if (heading && editorContainer) {
    // Calculate scroll position
    const containerRect = editorContainer.getBoundingClientRect();
    const headingRect = heading.getBoundingClientRect();
    const safeOffset = 12;
    const scrollTop = editorContainer.scrollTop + headingRect.top - containerRect.top - safeOffset;

    // Smooth scroll and highlight
    editorContainer.scrollTo({ top: Math.max(0, scrollTop), behavior: 'smooth' });
    activeHeadingId.value = headingId;

    // Ensure accurate positioning after render
    requestAnimationFrame(() => {
      const headingRect2 = heading.getBoundingClientRect();
      const newScrollTop = editorContainer.scrollTop + headingRect2.top - containerRect.top - safeOffset;
      if (Math.abs(newScrollTop - editorContainer.scrollTop) > 4) {
        editorContainer.scrollTo({ top: Math.max(0, newScrollTop), behavior: 'smooth' });
      }
    });
  }
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

// Legacy scroll sync (deprecated with Tiptap)
// const _isScrollingEditor = ref(false)
// const _isScrollingPreview = ref(false)

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
    render();

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
        updateActiveHeading()
      }, 200)
    }
  });
  }, 100) as unknown as number; // 减少防抖延迟从500ms到100ms，提供更即时的预览更新
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
  
  // 确保预览区也同步渲染最新内容
  nextTick(() => {
    render()
  })
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

onDeactivated(() => {
  // 清理响应式工具栏资源
  if (resizeObserver.value) {
    resizeObserver.value.disconnect()
    resizeObserver.value = null
  }
})

// 添加图片放大模态框的逻辑
const showImageModal = ref(false)
const modalImageSrc = ref('')
const modalImageAlt = ref('')
const modalImageLoading = ref(false)


function closeImageModal() {
  showImageModal.value = false
  resetZoom()
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

// 处理翻译
async function processTranslation(text: string) {
  isTranslating.value = true
  try {
    const isEnglish = /^[a-zA-Z0-9\s.,?!;:'"()\[\]{}<>\/\\|@#$%^&*_+=-]+$/.test(text)
    const prompt = isEnglish
      ? `请将以下英文翻译成中文：\n\n${text}`
      : `请将以下中文翻译成英文：\n\n${text}`
    const streamId = `translate_${Date.now()}`
    // 重置内容
    translationContent.value = ''
    translationRawContent.value = ''
    const providerId = await getActualProviderId()
    console.log('AI翻译功能使用的providerId:', providerId, '原始defaultProviderId:', defaultProviderId.value)
    const { listen } = await import('@tauri-apps/api/event')
    const unlisten = await listen('ai-stream-chunk', async (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }
      if (payload.id !== streamId) return

      // 检查后端返回的错误
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        translationContent.value = `<p class="text-error">翻译失败: ${payload.error}</p>`
        translationRawContent.value = `翻译失败: ${payload.error}`
        isTranslating.value = false
        unlisten()
        return
      }

      if (payload.chunk) {
        // 首个chunk到达即关闭loading
        isTranslating.value = false
        // 累积原始内容
        translationRawContent.value += payload.chunk
        // 渲染HTML内容用于显示
        translationContent.value = await renderInlineMarkdown(translationRawContent.value)
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
    translationRawContent.value = `翻译失败: ${error}`
    isTranslating.value = false
  }
}

// 复制翻译内容
async function copyTranslation() {
  // 使用原始内容并移除think标签
  const cleanText = translationRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()

  // 复制到剪贴板
  try {
    await navigator.clipboard.writeText(cleanText)
    // 显示成功消息
    await showAlert('翻译内容已复制到剪贴板', { title: '复制成功' })
  } catch (err) {
    console.error('复制失败:', err)
    await showAlert('复制失败，请手动选择并复制', { title: '复制失败' })
  }
}

// 将翻译结果插入到笔记内容
function insertTranslationToContent() {
  if (!tiptapEditor.value) return

  // 使用原始内容并移除think标签
  const cleanText = translationRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()

  // 插入翻译内容
  const content = '\n\n' + cleanText + '\n\n'
  tiptapEditor.value.insertAtCursor?.(content)

  nextTick(() => {
    tiptapEditor.value.focus?.()
  })

  // 保存
  autoSave()

  // 关闭翻译框
  showTranslationBox.value = false
}

// TIP 对话框相关函数
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



// 整个编辑器失焦时完整保存
function onEditorBlur(event: FocusEvent) {
  // 只有真正离开整个编辑器才触发
  if (!(event.currentTarget as HTMLElement).contains(event.relatedTarget as Node)) {
    saveNoteToList()
  }
}

// 全屏模式相关方法
function toggleFullscreen() {
  console.log('toggleFullscreen called, current state:', isFullscreen.value)
  if (isFullscreen.value) {
    exitFullscreen()
  } else {
    enterFullscreen()
  }
}

async function enterFullscreen() {
  try {
    const container = fullscreenContainer.value
    console.log('enterFullscreen: container ref:', container)
    
    // 禁用body滚动，防止全屏时出现滚动条
    document.body.style.overflow = 'hidden'
    
    if (container) {
      // 使用浏览器原生全屏API
      if (container.requestFullscreen) {
        await container.requestFullscreen()
      } else if ((container as any).webkitRequestFullscreen) {
        await (container as any).webkitRequestFullscreen()
      } else if ((container as any).msRequestFullscreen) {
        await (container as any).msRequestFullscreen()
      } else {
        console.warn('浏览器不支持全屏API')
        // 如果浏览器不支持全屏API，使用CSS全屏
        isFullscreen.value = true
        console.log('已进入CSS全屏模式')
      }
    } else {
      console.warn('fullscreenContainer ref 未初始化，使用CSS全屏')
      // 如果ref未初始化，直接使用CSS全屏
      isFullscreen.value = true
    }
    
    // 如果还没有设置，则设置为true
    if (!isFullscreen.value) {
      isFullscreen.value = true
    }
    
    // 全屏后默认设置为分屏模式以获得最佳体验
    if (!isSplitMode.value && !isEditOnly.value && !isPreviewMode.value) {
      setEditMode('split')
    }
    
    console.log('已进入全屏模式')
  } catch (error) {
    console.error('进入全屏失败:', error)
    // 如果原生全屏失败，使用CSS全屏
    isFullscreen.value = true
    console.log('使用CSS全屏作为备选方案')
  }
}

async function exitFullscreen() {
  try {
    console.log('exitFullscreen called')
    if (document.fullscreenElement) {
      await document.exitFullscreen()
    } else if ((document as any).webkitFullscreenElement) {
      await (document as any).webkitExitFullscreen()
    } else if ((document as any).msFullscreenElement) {
      await (document as any).msExitFullscreen()
    } else {
      console.log('未检测到浏览器全屏元素，直接退出CSS全屏')
    }
    
    // 恢复body滚动
    document.body.style.overflow = ''
    
    isFullscreen.value = false
    console.log('已退出全屏模式')
  } catch (error) {
    console.error('退出全屏失败:', error)
    // 恢复body滚动
    document.body.style.overflow = ''
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
    // 恢复body滚动
    document.body.style.overflow = ''
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
  if (props.note.content === t('noteEditor.encryptedPlaceholder')) {
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
  if (props.note.content === t('noteEditor.encryptedPlaceholder')) {
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
    case 'undo':
      tiptapEditor.value?.execToolbar('undo')
      break
    case 'redo':
      tiptapEditor.value?.execToolbar('redo')
      break
    case 'tiptap-bold':
      tiptapEditor.value?.execToolbar('bold')
      break
    case 'tiptap-italic':
      tiptapEditor.value?.execToolbar('italic')
      break
    case 'tiptap-strike':
      tiptapEditor.value?.execToolbar('strike')
      break
    case 'tiptap-underline':
      tiptapEditor.value?.execToolbar('underline')
      break
    case 'tiptap-code':
      tiptapEditor.value?.execToolbar('code')
      break
    case 'toggle-highlight':
      tiptapEditor.value?.execToolbar('toggleHighlight')
      break
    case 'toggle-link':
      toggleLink()
      break
    case 'tiptap-h1':
      tiptapEditor.value?.execToolbar('heading', { level: 1 })
      break
    case 'tiptap-h2':
      tiptapEditor.value?.execToolbar('heading', { level: 2 })
      break
    case 'tiptap-h3':
      tiptapEditor.value?.execToolbar('heading', { level: 3 })
      break
    case 'tiptap-paragraph':
      tiptapEditor.value?.execToolbar('paragraph')
      break
    case 'tiptap-blockquote':
      tiptapEditor.value?.execToolbar('blockquote')
      break
    case 'tiptap-codeblock':
      tiptapEditor.value?.execToolbar('codeblock')
      break
    case 'tiptap-bulletlist':
      tiptapEditor.value?.execToolbar('bulletList')
      break
    case 'tiptap-orderedlist':
      tiptapEditor.value?.execToolbar('orderedList')
      break
    case 'tiptap-tasklist':
      tiptapEditor.value?.execToolbar('taskList')
      break
    case 'tiptap-table':
      tiptapEditor.value?.execToolbar('table', { rows: 3, cols: 3 })
      break
    case 'toggle-toc':
      toggleToc()
      break
    case 'toggle-audio-recording':
      toggleAudioRecording()
      break
    case 'toggle-audio-player':
      toggleAudioPlayer()
      break
    case 'set-highlight-theme':
      setHighlightTheme(args[0])
      break
    case 'toggle-fullscreen':
      toggleFullscreen()
      break
    default:
      console.warn('Unknown toolbar command:', command)
  }
}

// 切换音频录制
function toggleAudioRecording() {
  // 确保笔记已保存（有ID）
  if (!localNote.value.id) {
    showAlert(t('noteEditor.saveNoteForAudio'), { title: t('common.tip') })
    return
  }
  
  showAudioRecorder.value = !showAudioRecorder.value
}

// 切换音频播放器
function toggleAudioPlayer() {
  // 确保笔记已保存（有ID）
  if (!localNote.value.id) {
    showAlert(t('noteEditor.saveNoteForPlayer'), { title: t('common.tip') })
    return
  }
  
  showAudioPlayer.value = !showAudioPlayer.value
}

// 切换链接
function toggleLink() {
  // 获取当前选中的文本
  const selectedText = tiptapEditor.value?.getSelectedText()
  
  if (!selectedText || selectedText.trim() === '') {
    showAlert(t('noteEditor.selectTextForLink') || 'Please select text to add a link', { title: t('common.tip') })
    return
  }
  
  // 提示用户输入链接
  const url = prompt(t('noteEditor.enterLinkUrl') || 'Enter the URL:', 'https://')
  
  if (url && url.trim()) {
    tiptapEditor.value?.execToolbar('link', { href: url.trim() })
  } else if (url === '') {
    // 如果用户清空了URL，移除链接
    tiptapEditor.value?.execToolbar('link', { href: null })
  }
}

// 处理音频插入完成
function handleAudioInserted(audioId: string, transcription?: string) {
  console.log('Audio inserted:', audioId, transcription)
  
  // 在光标位置插入音频引用
  const textarea = editorTextarea.value
  if (textarea) {
    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    
    let audioMarkdown = `\n\n🎵 **音频录制**\n`
    audioMarkdown += `<audio controls onerror="alert('音频加载失败: ' + this.error.message)">\n  <source src="audio://${audioId}" type="audio/webm">\n  您的浏览器不支持音频播放。\n</audio>\n`
    
    // 如果有转录文本，也插入
    if (transcription && transcription.trim()) {
      audioMarkdown += `\n**转录文本：**\n${transcription}\n`
    }
    
    audioMarkdown += `\n---\n`
    
    // 在光标位置插入
    localNote.value.content = 
      localNote.value.content.substring(0, start) +
      audioMarkdown +
      localNote.value.content.substring(end)
    
    // 更新界面
    nextTick(() => {
      if (textarea) {
        const newCursorPos = start + audioMarkdown.length
        textarea.selectionStart = newCursorPos
        textarea.selectionEnd = newCursorPos
        textarea.focus()
      }
      
      // 触发自动保存
      autoSave()
    })
  }
}

// 处理音频播放器插入
function handleAudioPlayerInsert(data: { text: string, type: 'link' | 'transcription' }) {
  console.log('Audio player insert:', data)
  
  const textarea = editorTextarea.value
  if (textarea) {
    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    
    let insertText = ''
    
    if (data.type === 'link') {
      insertText = `\n${data.text}\n`
    } else if (data.type === 'transcription') {
      insertText = `\n**${t('noteEditor.transcriptionText')}:**\n${data.text}\n`
    }
    
    // 在光标位置插入
    localNote.value.content = 
      localNote.value.content.substring(0, start) +
      insertText +
      localNote.value.content.substring(end)
    
    // 更新界面
    nextTick(() => {
      if (textarea) {
        const newCursorPos = start + insertText.length
        textarea.selectionStart = newCursorPos
        textarea.selectionEnd = newCursorPos
        textarea.focus()
      }
      
      // 触发自动保存
      autoSave()
    })
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
  if (!tiptapEditor.value) return

  const selectedText = tiptapEditor.value.getSelectedText?.()
  
  // 确保有选择的文本
  if (!selectedText || selectedText.length === 0) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  selectedTextForExplanation.value = selectedText
  explanationContent.value = ''
  showExplanationBox.value = true
  showContextMenu.value = false

  // 使用AI解释选中的文本
  await processExplanation(selectedText)
}

async function translateWithAI() {
  if (!tiptapEditor.value) return

  const selectedText = tiptapEditor.value.getSelectedText?.()
  
  // 确保有选择的文本
  if (!selectedText || selectedText.length === 0) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  selectedTextForTranslation.value = selectedText
  translationContent.value = ''
  showTranslationBox.value = true
  showContextMenu.value = false

  // 使用AI翻译选中的文本
  await processTranslation(selectedText)
}

async function tipWithAI() {
  if (!tiptapEditor.value) return

  const selectedText = tiptapEditor.value.getSelectedText?.()
  
  // 确保有选择的文本
  if (!selectedText || selectedText.length === 0) {
    await showAlert('请先选择一段文本', { title: '提示' })
    return
  }

  // 显示TIP对话框让用户修改提示词
  selectedTextForTip.value = selectedText
  originalTipPrompt.value = selectedText
  tipPrompt.value = originalTipPrompt.value
  
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

// 全局默认AI提供商ID（与全局store保持实时同步）
const aiStore = useAIStore()
const defaultProviderId = computed(() => aiStore.defaultChatProvider || 'deepseek')

// 自定义模型配置
const customModelConfigs = ref<Record<string, any>>({})

// 获取实际的provider ID（处理自定义模型的情况）
async function getActualProviderId(): Promise<string> {
  const providerId = defaultProviderId.value
  
  // 如果是自定义模型，需要获取其配置的真实适配器类型
  if (providerId.startsWith('custom_')) {
    try {
      // 加载自定义模型配置
      await loadCustomModels()
      const config = customModelConfigs.value[providerId]
      if (config && config.adapter_type) {
        console.log(`自定义模型 ${providerId} 的真实适配器类型:`, config.adapter_type)
        // 返回真实的适配器类型，比如 'ollama'
        return config.adapter_type
      }
    } catch (error) {
      console.error('获取自定义模型配置失败:', error)
    }
  }
  
  return providerId
}

// 加载自定义模型配置
async function loadCustomModels() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const customModels = await invoke('list_custom_model_configs') as Array<{
      id: string
      name: string
      endpoint: string
      model_name: string
      adapter_type: string
      api_key?: string
    }>
    
    customModelConfigs.value = {}
    customModels.forEach(model => {
      customModelConfigs.value[`custom_${model.id}`] = model
    })
    
    console.log('加载自定义模型配置:', customModelConfigs.value)
  } catch (error) {
    console.error('加载自定义模型配置失败:', error)
  }
}

// 在组件挂载时获取全局默认AI模型
onMounted(async () => {
  try {
    await aiStore.loadDefaultChatModel()
    await loadCustomModels() // 预加载自定义模型配置
    console.log('NoteEditor: 获取全局默认AI provider:', aiStore.defaultChatProvider)
    
    // 调试：显示实际将要使用的provider
    const actualProvider = await getActualProviderId()
    console.log('NoteEditor: 实际使用的AI provider:', actualProvider)
  } catch (error) {
    console.error('NoteEditor: 获取默认AI模型失败:', error)
  }
})

// 监听全局设置变更，更新默认AI模型
let settingsUnlisten: (() => void) | null = null
onMounted(async () => {
  try {
    const { listen } = await import('@tauri-apps/api/event')
    settingsUnlisten = await listen('global-settings-changed', async (event: { payload: any }) => {
      const key = event?.payload?.key
      if (key === 'defaultAIModel') {
        try {
          await aiStore.loadDefaultChatModel()
          console.log('NoteEditor: 默认AI provider已更新为:', aiStore.defaultChatProvider)
        } catch (e) {
          console.error('NoteEditor: 刷新默认AI模型失败:', e)
        }
      }
    })
  } catch (e) {
    console.error('NoteEditor: 监听全局设置变更失败:', e)
  }
})

onBeforeUnmount(() => {
  try { settingsUnlisten && settingsUnlisten(); } catch {}
  settingsUnlisten = null
})

// TIP结果流监听器引用
let tipStreamUnlisten: (() => void) | null = null

// 处理TIP请求并生成结果
async function processTip(_originalText: string, prompt: string) {
  try {
    isTipProcessing.value = true
    // 重置内容
    tipResultContent.value = ''
    tipResultRawContent.value = ''
    showTipResultBox.value = true

    const providerId = await getActualProviderId()
    console.log('TIP功能使用的providerId:', providerId, '原始defaultProviderId:', defaultProviderId.value)
    const streamId = `tip_${Date.now()}`

    // 监听流式返回
    const { listen } = await import('@tauri-apps/api/event')
    tipStreamUnlisten = await listen('ai-stream-chunk', async (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean, error?: string }

      if (payload.id !== streamId) return

      // 错误处理
      if (payload.error) {
        console.error('AI stream error from backend:', payload.error)
        tipResultContent.value = `<p class="text-error">TIP生成失败: ${payload.error}</p>`
        tipResultRawContent.value = `TIP生成失败: ${payload.error}`
        isTipProcessing.value = false
        cleanupTipStream()
        return
      }

      if (payload.chunk) {
        // 首个chunk到达即关闭loading
        isTipProcessing.value = false
        // 累积原始内容
        tipResultRawContent.value += payload.chunk
        // 渲染HTML内容用于显示
        tipResultContent.value = await renderInlineMarkdown(tipResultRawContent.value)
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
    // 使用原始内容并移除think标签
    const cleanText = tipResultRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()
    await navigator.clipboard.writeText(cleanText)
  } catch (err) {
    console.error('复制TIP结果失败:', err)
    await showAlert('复制失败，请手动选择并复制', { title: '复制失败' })
  }
}

// 将TIP结果插入笔记
function insertTipResultToContent() {
  if (!tiptapEditor.value) return

  // 使用原始内容并移除think标签
  const cleanText = tipResultRawContent.value.replace(/<think\b[\s\S]*?<\/think>/gi, '').replace(/<details\b[\s\S]*?<\/details>/gi, '').trim()
  
  const content = '\n\n' + cleanText + '\n\n'
  tiptapEditor.value.insertAtCursor?.(content)

  nextTick(() => {
    tiptapEditor.value.focus?.()
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


// 渲染Markdown内容
const render = async () => {
  if (localNote.value) {
    try {
      const { html, toc } = await renderMarkdown(
        localNote.value.content,
        localNote.value.images || {}
      )
      renderedContent.value = html
      tocItems.value = toc

      nextTick(() => {
        // Since rehype-prism-plus handles highlighting, 
        // we just need to ensure the line numbers are applied if needed.
        Prism.highlightAll()
    updateActiveHeading()
    // 渲染完成后，确保目录与标题 id 对齐
    generateToc()
      })
    } catch (error) {
      console.error('Markdown rendering error:', error)
      renderedContent.value = `<div class="text-error">${t('noteEditor.markdownRenderError', { error: String(error) })}</div>`
    }
  }
}
const updateActiveHeading = () => {
  const preview = previewDiv.value
  if (!preview || tocItems.value.length === 0) return

  const containerRect = preview.getBoundingClientRect()
  const containerTop = containerRect.top + 50

  let activeId = ''

  for (const item of tocItems.value) {
    const heading = preview.querySelector(`#${item.id}`)
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

const templateStore = useTipTemplateStore();

// 在其他 script 顶层常量之后添加音频缓存

// --- Start: Image Zoom & Pan Functions ---

function onZoomSliderChange(event: Event) {
  zoomLevel.value = parseFloat((event.target as HTMLInputElement).value);
  if (zoomLevel.value <= 1) {
    panX.value = 0;
    panY.value = 0;
  }
}

function handleZoom(event: WheelEvent) {
  event.preventDefault();
  const scaleAmount = 0.1;
  
  if (event.deltaY < 0) {
    zoomLevel.value = Math.min(5, zoomLevel.value + scaleAmount);
  } else {
    zoomLevel.value = Math.max(0.5, zoomLevel.value - scaleAmount);
  }

  if (zoomLevel.value <= 1) {
    panX.value = 0;
    panY.value = 0;
  }
}

function zoomIn() {
  zoomLevel.value = Math.min(5, zoomLevel.value + 0.5);
  if (zoomLevel.value <= 1) {
    panX.value = 0;
    panY.value = 0;
  }
}

function zoomOut() {
  zoomLevel.value = Math.max(0.5, zoomLevel.value - 0.5);
  if (zoomLevel.value <= 1) {
    panX.value = 0;
    panY.value = 0;
  }
}

function resetZoom() {
  zoomLevel.value = 0.7;
  panX.value = 0;
  panY.value = 0;
}

function startPan(event: MouseEvent) {
  if (event.button !== 0 || zoomLevel.value <= 1) return;
  event.preventDefault();
  
  isPanning.value = true;
  panStart.value = { x: event.clientX, y: event.clientY };
  imageStart.value = { x: panX.value, y: panY.value };
  
  document.addEventListener('mousemove', doPan);
  document.addEventListener('mouseup', endPan);
}

function doPan(event: MouseEvent) {
  if (!isPanning.value) return;
  event.preventDefault();
  
  const dx = event.clientX - panStart.value.x;
  const dy = event.clientY - panStart.value.y;
  
  panX.value = imageStart.value.x + dx / zoomLevel.value;
  panY.value = imageStart.value.y + dy / zoomLevel.value;
}

function endPan(event: MouseEvent) {
  if (!isPanning.value) return;
  event.preventDefault();
  isPanning.value = false;
  document.removeEventListener('mousemove', doPan);
  document.removeEventListener('mouseup', endPan);
}
// --- End: Image Zoom & Pan Functions ---
</script>

<style lang="scss" scoped>
.fullscreen-editor {
  position: fixed;
  inset: 0;
  width: 100vw !important;
  height: 100vh !important;
  z-index: 9999;
  background: var(--color-base-100, #ffffff);
  overflow: hidden !important;
  display: flex;
  flex-direction: column;
  margin: 0 !important;
  padding: 0 !important;
  border: none !important;
  border-radius: 0 !important;
}

// 防止全屏时编辑器被遮挡，确保之后的兄弟元素也不会显示
:deep(.fullscreen-editor ~ *) {
  display: none !important;
}

// 顶部栏和工具栏保持正常高度
:deep(.fullscreen-editor .editor-top-bar) {
  flex-shrink: 0;
  height: auto;
  overflow: visible;
}

:deep(.fullscreen-editor .editor-toolbar) {
  flex-shrink: 0;
  height: auto;
  overflow: visible;
}

// 主编辑区域填充剩余空间
:deep(.fullscreen-editor > .flex-1) {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: row;
  
  // 编辑器占据可用空间
  > * {
    flex: 1;
    overflow: auto;
  }
}

// 富文本编辑器在全屏时的样式
:deep(.fullscreen-editor .ProseMirror) {
  height: 100%;
  max-height: 100%;
}

// 目录面板在全屏时的样式
:deep(.fullscreen-editor .note-toc) {
  max-height: 100%;
  overflow-y: auto;
}

// 预览面板在全屏时的样式
:deep(.fullscreen-editor .markdown-preview) {
  max-height: 100%;
  overflow-y: auto;
}

// 禁用在全屏编辑器容器内的内部滚动条
:deep(.fullscreen-editor) {
  scrollbar-width: thin;
  
  &::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }
  
  &::-webkit-scrollbar-track {
    background: transparent;
  }
  
  &::-webkit-scrollbar-thumb {
    background: var(--color-base-300);
    border-radius: 4px;
  }
}
</style>

// Removed: WYSIWYG update handler
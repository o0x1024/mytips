<template>
  <!-- 拖拽覆盖层 -->
  <div 
    v-if="isDragOver" 
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center"
    @click="closeDragOverlay"
  >
    <div class="bg-base-100 rounded-lg p-8 shadow-2xl border-2 border-dashed border-primary border-opacity-50 relative" @click.stop>
      <!-- 关闭按钮 -->
      <button 
        class="absolute top-2 right-2 btn btn-sm btn-circle btn-ghost"
        @click="closeDragOverlay"
        :title="t('common.close')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
      
      <div class="text-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="text-xl font-bold mb-2">{{ t('markdownDropPreview.dropTitle') }}</h3>
        <p class="text-base-content/70">{{ t('markdownDropPreview.dropSubtitle') }}</p>
        <p class="text-xs text-base-content/50 mt-2">{{ t('markdownDropPreview.dropHint') }}</p>
      </div>
    </div>
  </div>

  <!-- 预览模态框 -->
  <div class="modal" :class="{'modal-open': showPreview}">
    <div class="modal-box max-w-5xl h-5/6">
      <div class="flex justify-between items-center mb-4">
        <h3 class="font-bold text-lg">{{ previewFile?.name }}</h3>
        <div class="flex gap-2">
          <button 
            class="btn btn-sm btn-primary" 
            @click="importAsNote"
            :disabled="!previewContent"
          >
            {{ t('markdownDropPreview.importAsNote') }}
          </button>
          <button class="btn btn-sm btn-ghost" @click="closePreview">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
      
      <div class="divider my-2"></div>
      
      <!-- 预览内容 -->
      <div class="h-full overflow-y-auto">
        <div 
          v-if="previewContent" 
          class="prose max-w-none"
          v-html="renderedContent"
        ></div>
        <div v-else-if="isLoading" class="flex items-center justify-center h-full">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
        <div v-else class="flex items-center justify-center h-full text-base-content/50">
          {{ t('markdownDropPreview.cannotReadFile') }}
        </div>
      </div>
    </div>
    <div class="modal-backdrop" @click="closePreview"></div>
  </div>

  <!-- 导入选项模态框 -->
  <div class="modal" :class="{'modal-open': showImportOptions}">
    <div class="modal-box">
      <h3 class="font-bold text-lg">{{ t('markdownDropPreview.importOptions') }}</h3>
      
      <div class="form-control w-full mt-4">
        <label class="label">
          <span class="label-text">{{ t('markdownDropPreview.noteTitle') }}</span>
        </label>
        <input 
          type="text" 
          class="input input-bordered w-full" 
          v-model="importTitle"
          :placeholder="previewFile?.name.replace(/\.[^/.]+$/, '') || ''"
        />
      </div>

      <div class="form-control w-full mt-4">
        <label class="label">
          <span class="label-text">{{ t('markdownDropPreview.selectNotebook') }}</span>
        </label>
        <select class="select select-bordered w-full" v-model="selectedNotebookId">
          <option value="">{{ t('markdownDropPreview.selectNotebookPlaceholder') }}</option>
          <option v-for="notebook in notebooks" :key="notebook.id" :value="notebook.id">
            {{ notebook.name }}
          </option>
        </select>
      </div>

      <div class="modal-action">
        <button class="btn" @click="cancelImport">{{ t('common.cancel') }}</button>
        <button 
          class="btn btn-primary" 
          @click="confirmImport"
          :disabled="!importTitle.trim() || !selectedNotebookId"
        >
          {{ t('markdownDropPreview.confirmImport') }}
        </button>
      </div>
    </div>
    <div class="modal-backdrop" @click="cancelImport"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Marked } from 'marked'
import { markedHighlight } from "marked-highlight";
import Prism from 'prismjs'
import 'prismjs/themes/prism-okaidia.css'
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-java'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-sql'
import DOMPurify from 'dompurify'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { readTextFile, exists } from '@tauri-apps/plugin-fs'
import { showAlert } from '../services/dialog'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// Props
interface Props {
  notebooks: Array<{id: string, name: string}>
}

const props = defineProps<Props>()

// Emits
const emit = defineEmits<{
  import: [data: {
    title: string
    content: string
    notebookId: string
    fileName: string
  }]
}>()

// 状态
const isDragOver = ref(false)
const showPreview = ref(false)
const showImportOptions = ref(false)
const isLoading = ref(false)
const previewFile = ref<{name: string, path: string} | null>(null)
const previewContent = ref('')
const importTitle = ref('')
const selectedNotebookId = ref('')
const dragPosition = ref({ x: 0, y: 0 })

// Tauri拖拽事件监听器
let unlistenDragDrop: (() => void) | null = null

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

// HTML 转义函数
function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

// 计算属性
const renderedContent = computed(() => {
  if (!previewContent.value) return ''
  
  try {
    // 创建 marked 实例并配置高亮
    const marked = new Marked();
    
    // 使用 marked-highlight 扩展
    marked.use(markedHighlight({
      langPrefix: 'language-',
      highlight(code: string, lang: string) {
        // 如果没有指定语言，使用 plaintext 作为默认语言
        const actualLang = lang || 'plaintext';
        
        // 使用安全检查函数
        if (actualLang && isPrismLanguageAvailable(actualLang)) {
          try {
            return Prism.highlight(code, Prism.languages[actualLang], actualLang);
          } catch (error) {
            console.warn(`Prism 高亮失败 (${actualLang}):`, error);
            return escapeHtml(code);
          }
        }
        
        // 如果 plaintext 也不可用，直接返回转义的代码
        return escapeHtml(code);
      }
    }));

    // 配置 marked 选项
    marked.setOptions({
      breaks: true,
      gfm: true,
      silent: true,
    });

    // 使用 marked 渲染 Markdown
    const htmlContent = marked.parse(previewContent.value) as string;
    
    return DOMPurify.sanitize(htmlContent, {
      ADD_TAGS: ['iframe', 'pre', 'code'],
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled']
    });
  } catch (error) {
    console.error('Markdown rendering error:', error)
    return `<p>${t('markdownDropPreview.renderError')}</p>`
  }
})

// Tauri拖拽事件处理
async function handleTauriDragDrop(event: any) {
  console.log('Tauri拖拽事件:', event.payload)
  
  try {
    if (event.payload.type === 'over') {
      // 文件悬停
  isDragOver.value = true
      dragPosition.value = { x: event.payload.position.x, y: event.payload.position.y }
      console.log('文件悬停在位置:', dragPosition.value)
    } else if (event.payload.type === 'drop') {
      // 文件放置
      isDragOver.value = false
      const paths = event.payload.paths
      console.log('文件放置，路径:', paths)
      
      if (paths && paths.length > 0) {
        // 查找markdown文件
        const markdownFiles = await filterMarkdownFiles(paths)
        console.log('找到的markdown文件:', markdownFiles)
        
        if (markdownFiles.length > 0) {
          const markdownFile = markdownFiles[0]
          console.log('Choosing markdown file to preview:', markdownFile)
          await previewMarkdownFileFromPath(markdownFile)
        } else {
          console.log('No markdown files found')
          const fileNames = paths.map((path: string) => path.split('/').pop() || path).join(', ')
          await showAlert(t('markdownDropPreview.fileTypeHintMessage', { files: fileNames }), { title: t('markdownDropPreview.fileTypeHintTitle') })
        }
      }
    } else if (event.payload.type === 'cancelled') {
      // 拖拽取消
      isDragOver.value = false
      console.log('文件拖拽取消')
    }
  } catch (error) {
    console.error('处理Tauri拖拽事件时出错:', error)
  isDragOver.value = false
  }
}

// 过滤markdown文件
async function filterMarkdownFiles(paths: string[]): Promise<string[]> {
  const markdownFiles: string[] = []
  
  for (const path of paths) {
    if (await isMarkdownFile(path)) {
      markdownFiles.push(path)
    }
  }
  
  return markdownFiles
}

// 文件类型检查 - Tauri版本（基于文件路径）
async function isMarkdownFile(path: string): Promise<boolean> {
  try {
    const fileName = path.split('/').pop()?.toLowerCase() || ''
    console.log('检查文件类型:', fileName)
    
    // 支持更多的markdown文件扩展名
    const markdownExtensions = [
      '.md', '.markdown', '.mdown', '.mkdn', '.mkd', 
      '.mdwn', '.mdtxt', '.mdtext', '.text','.txt'
    ]
    
    const isMarkdown = markdownExtensions.some(ext => fileName.endsWith(ext))
    console.log(`文件 ${fileName} 是否为markdown:`, isMarkdown)
    
    // 检查文件是否存在
    if (isMarkdown) {
      const fileExists = await exists(path)
      console.log(`文件 ${path} 是否存在:`, fileExists)
      return fileExists
    }
    
    return false
  } catch (error) {
    console.error('检查文件类型时出错:', error)
    return false
  }
}

// 预览文件（从文件路径）
async function previewMarkdownFileFromPath(filePath: string) {
  const fileName = filePath.split('/').pop() || filePath
  previewFile.value = { name: fileName, path: filePath }
  isLoading.value = true
  showPreview.value = true
  
  try {
    const content = await readTextFile(filePath)
    previewContent.value = content
    console.log(`Successfully read file ${fileName}, content length: ${content.length}`)
  } catch (error) {
    console.error('Failed to read file:', error)
    previewContent.value = ''
    await showAlert(t('markdownDropPreview.readFileError', { error }), { title: t('markdownDropPreview.readFileErrorTitle') })
  } finally {
    isLoading.value = false
  }
}

// 关闭预览
function closePreview() {
  showPreview.value = false
  previewFile.value = null
  previewContent.value = ''
}

// 导入为笔记
function importAsNote() {
  if (!previewFile.value || !previewContent.value) return
  
  importTitle.value = previewFile.value.name.replace(/\.[^/.]+$/, '')
  selectedNotebookId.value = props.notebooks[0]?.id || ''
  showImportOptions.value = true
}

// 确认导入
function confirmImport() {
  if (!previewFile.value || !previewContent.value || !importTitle.value.trim() || !selectedNotebookId.value) {
    return
  }
  
  emit('import', {
    title: importTitle.value.trim(),
    content: previewContent.value,
    notebookId: selectedNotebookId.value,
    fileName: previewFile.value.name
  })
  
  // 显示成功提示
  console.log(`Markdown file "${previewFile.value.name}" imported successfully!`)
  
  // 重置状态
  showImportOptions.value = false
  closePreview()
  importTitle.value = ''
  selectedNotebookId.value = ''
}

// 取消导入
function cancelImport() {
  showImportOptions.value = false
  importTitle.value = ''
  selectedNotebookId.value = ''
}

// 生命周期
onMounted(async () => {
  
  try {
    // 使用Tauri的原生拖拽事件API
    const webview = getCurrentWebview()
    unlistenDragDrop = await webview.onDragDropEvent(handleTauriDragDrop)
    
  
    // 添加测试函数到全局
  ;(window as any).testDragFunction = () => {
    console.log('手动测试拖拽功能')
    isDragOver.value = !isDragOver.value
    console.log('拖拽覆盖层状态:', isDragOver.value)
  }

    
    // 添加ESC键监听
    document.addEventListener('keydown', handleKeyDown)
    
  } catch (error) {
    console.error('设置Tauri拖拽事件监听器失败:', error)
    // 如果Tauri API不可用，可以考虑回退到HTML5拖拽API
    console.warn('将尝试使用HTML5拖拽API作为备用方案')
  }
})

onUnmounted(() => {
  console.log('MarkdownDropPreview 组件卸载，移除拖拽事件监听器')
  
  // 移除Tauri拖拽事件监听器
  if (unlistenDragDrop) {
    unlistenDragDrop()
    unlistenDragDrop = null
    console.log('✅ Tauri拖拽事件监听器已移除')
  }
  
  // 移除ESC键监听
  document.removeEventListener('keydown', handleKeyDown)
  
  // 清理全局测试函数
  if ((window as any).testDragFunction) {
    delete (window as any).testDragFunction
  }
})

// ESC键处理函数
function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Escape' && isDragOver.value) {
    closeDragOverlay()
  }
}

// 关闭拖拽覆盖层
function closeDragOverlay() {
  isDragOver.value = false
}
</script>

<style scoped>
.prose {
  color: inherit;
  font-size: var(--base-font-size); /* 继承全局字体大小 */
}

/* 确保prose内的所有元素都能正确继承字体大小 */
.prose * {
  font-size: inherit;
}

/* 为不同标题级别设置相对大小 */
.prose h1 {
  font-size: calc(var(--base-font-size) * 2.25); /* 相当于 text-4xl */
}

.prose h2 {
  font-size: calc(var(--base-font-size) * 1.875); /* 相当于 text-3xl */
}

.prose h3 {
  font-size: calc(var(--base-font-size) * 1.5); /* 相当于 text-2xl */
}

.prose h4 {
  font-size: calc(var(--base-font-size) * 1.25); /* 相当于 text-xl */
}

.prose h5 {
  font-size: calc(var(--base-font-size) * 1.125); /* 相当于 text-lg */
}

.prose h6 {
  font-size: var(--base-font-size); /* 相当于 text-base */
}

.prose h1,
.prose h2,
.prose h3,
.prose h4,
.prose h5,
.prose h6 {
  color: inherit;
}

.prose code {
  border-radius: 0.25rem;
  padding: 0.125rem 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  /* 使用CSS变量确保在所有主题下都有合适的颜色 */
  background-color: hsl(var(--b3));
  color: hsl(var(--bc));
  border: 1px solid hsl(var(--b3));
}

/* 暗色主题下的行内代码特殊优化 */
:deep(.prose code) {
  border-radius: 0.25rem;
  padding: 0.125rem 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  background-color: hsl(var(--b3));
  color: hsl(var(--bc));
  border: 1px solid hsl(var(--b3));
}

[data-theme="dark"] :deep(.prose code),
[data-theme="night"] :deep(.prose code),
[data-theme="black"] :deep(.prose code),
[data-theme="dracula"] :deep(.prose code),
[data-theme="halloween"] :deep(.prose code),
[data-theme="business"] :deep(.prose code),
[data-theme="luxury"] :deep(.prose code),
[data-theme="coffee"] :deep(.prose code),
[data-theme="forest"] :deep(.prose code),
[data-theme="synthwave"] :deep(.prose code) {
  background-color: rgba(255, 255, 255, 0.1) !important;
  color: rgb(251, 191, 36) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
}

/* 亮色主题下的行内代码优化 */
[data-theme="light"] :deep(.prose code),
[data-theme="cupcake"] :deep(.prose code),
[data-theme="bumblebee"] :deep(.prose code),
[data-theme="emerald"] :deep(.prose code),
[data-theme="corporate"] :deep(.prose code),
[data-theme="retro"] :deep(.prose code),
[data-theme="cyberpunk"] :deep(.prose code),
[data-theme="valentine"] :deep(.prose code),
[data-theme="garden"] :deep(.prose code),
[data-theme="aqua"] :deep(.prose code),
[data-theme="lofi"] :deep(.prose code),
[data-theme="pastel"] :deep(.prose code),
[data-theme="fantasy"] :deep(.prose code),
[data-theme="wireframe"] :deep(.prose code),
[data-theme="cmyk"] :deep(.prose code),
[data-theme="autumn"] :deep(.prose code),
[data-theme="acid"] :deep(.prose code),
[data-theme="lemonade"] :deep(.prose code),
[data-theme="winter"] :deep(.prose code) {
  background-color: rgba(0, 0, 0, 0.08) !important;
  color: rgb(124, 58, 237) !important;
  border: 1px solid rgba(0, 0, 0, 0.15) !important;
}

.prose pre {
  background-color: hsl(var(--b2));
  border-radius: 0.5rem;
  padding: 1rem;
  overflow-x: auto;
}

.prose pre code {
  background-color: transparent;
  padding: 0;
}

.prose blockquote {
  border-left: 4px solid hsl(var(--p));
  padding-left: 1rem;
  margin-left: 0;
  font-style: italic;
  color: hsl(var(--bc) / 0.8);
}

.prose table {
  border-collapse: collapse;
  width: 100%;
}

.prose th,
.prose td {
  border: 1px solid hsl(var(--b3));
  padding: 0.5rem;
  text-align: left;
}

.prose th {
  background-color: hsl(var(--b2));
  font-weight: 600;
}
</style> 
<template>
  <!-- 拖拽覆盖层 -->
  <div 
    v-if="isDragOver" 
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center"
    @drop="handleDrop"
    @dragover.prevent
    @dragleave="handleDragLeave"
  >
    <div class="bg-base-100 rounded-lg p-8 shadow-2xl border-2 border-dashed border-primary border-opacity-50">
      <div class="text-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="text-xl font-bold mb-2">拖拽Markdown文件到此处</h3>
        <p class="text-base-content/70">支持 .md 和 .markdown 文件</p>
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
            导入为笔记
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
          class="prose prose-sm max-w-none"
          v-html="renderedContent"
        ></div>
        <div v-else-if="isLoading" class="flex items-center justify-center h-full">
          <span class="loading loading-spinner loading-lg"></span>
        </div>
        <div v-else class="flex items-center justify-center h-full text-base-content/50">
          无法读取文件内容
        </div>
      </div>
    </div>
    <div class="modal-backdrop" @click="closePreview"></div>
  </div>

  <!-- 导入选项模态框 -->
  <div class="modal" :class="{'modal-open': showImportOptions}">
    <div class="modal-box">
      <h3 class="font-bold text-lg">导入选项</h3>
      
      <div class="form-control w-full mt-4">
        <label class="label">
          <span class="label-text">笔记标题</span>
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
          <span class="label-text">选择笔记本</span>
        </label>
        <select class="select select-bordered w-full" v-model="selectedNotebookId">
          <option value="">选择笔记本</option>
          <option v-for="notebook in notebooks" :key="notebook.id" :value="notebook.id">
            {{ notebook.name }}
          </option>
        </select>
      </div>

      <div class="modal-action">
        <button class="btn" @click="cancelImport">取消</button>
        <button 
          class="btn btn-primary" 
          @click="confirmImport"
          :disabled="!importTitle.trim() || !selectedNotebookId"
        >
          确认导入
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
import 'prismjs/themes/prism-tomorrow.css'
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-java'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-sql'
import DOMPurify from 'dompurify'

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
const previewFile = ref<File | null>(null)
const previewContent = ref('')
const importTitle = ref('')
const selectedNotebookId = ref('')

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
    console.error('Markdown渲染错误:', error)
    return '<p>渲染错误</p>'
  }
})

// 拖拽处理
function handleDragEnter(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  console.log('拖拽进入检测', e.type, e.dataTransfer?.types)
  
  // 简单检查 - 只要有拖拽事件就显示覆盖层
  isDragOver.value = true
  console.log('显示拖拽覆盖层')
}

function handleDragLeave(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  console.log('拖拽离开检测', e.type)
  
  // 延迟隐藏，避免在拖拽区域内部移动时误触发
  setTimeout(() => {
    // 检查鼠标位置
    const rect = document.documentElement.getBoundingClientRect()
    if (e.clientX < rect.left || e.clientX > rect.right || 
        e.clientY < rect.top || e.clientY > rect.bottom) {
      isDragOver.value = false
      console.log('隐藏拖拽覆盖层')
    }
  }, 50)
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'copy'
  }
}

async function handleDrop(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  isDragOver.value = false
  console.log('文件放置事件触发', e.dataTransfer?.files?.length)
  
  if (!e.dataTransfer?.files) {
    console.log('没有检测到文件')
    return
  }
  
  const files = Array.from(e.dataTransfer.files)
  console.log('检测到文件:', files.map(f => `${f.name} (${f.type})`))
  
  const markdownFile = files.find(file => isMarkdownFile(file))
  
  if (markdownFile) {
    console.log('找到markdown文件:', markdownFile.name)
    await previewMarkdownFile(markdownFile)
  } else {
    console.log('没有找到markdown文件，支持的格式:', ['.md', '.markdown', '.mdown', '.mkdn', '.mkd', '.mdwn', '.mdtxt', '.mdtext', '.text'])
    // 显示提示信息
    alert('请拖拽 Markdown 文件（.md, .markdown 等格式）')
  }
}

// 文件类型检查
function isMarkdownFile(file: File): boolean {
  const name = file.name.toLowerCase()
  return name.endsWith('.md') || name.endsWith('.markdown') || name.endsWith('.mdown') || name.endsWith('.mkd')
}

// 预览文件
async function previewMarkdownFile(file: File) {
  previewFile.value = file
  isLoading.value = true
  showPreview.value = true
  
  try {
    const content = await readFileContent(file)
    previewContent.value = content
  } catch (error) {
    console.error('读取文件失败:', error)
    previewContent.value = ''
  } finally {
    isLoading.value = false
  }
}

// 读取文件内容
function readFileContent(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      resolve(e.target?.result as string || '')
    }
    reader.onerror = () => {
      reject(new Error('文件读取失败'))
    }
    reader.readAsText(file, 'utf-8')
  })
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
  console.log(`Markdown文件 "${previewFile.value.name}" 导入成功！`)
  
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
onMounted(() => {
  console.log('MarkdownDropPreview 组件已挂载，正在添加拖拽事件监听器')
  
  // 添加简单的测试函数到全局
  ;(window as any).testDragFunction = () => {
    console.log('手动测试拖拽功能')
    isDragOver.value = !isDragOver.value
    console.log('拖拽覆盖层状态:', isDragOver.value)
  }
  
  // 使用window事件监听，确保能捕获到所有拖拽事件
  window.addEventListener('dragenter', handleDragEnter, true)
  window.addEventListener('dragover', handleDragOver, true)
  window.addEventListener('dragleave', handleDragLeave, true)
  window.addEventListener('drop', handleDrop, true)
  
  console.log('拖拽事件监听器已添加到window')
  console.log('可以在控制台运行 testDragFunction() 来测试覆盖层')
})

onUnmounted(() => {
  console.log('MarkdownDropPreview 组件卸载，移除拖拽事件监听器')
  // 移除事件监听
  window.removeEventListener('dragenter', handleDragEnter, true)
  window.removeEventListener('dragover', handleDragOver, true)
  window.removeEventListener('dragleave', handleDragLeave, true)
  window.removeEventListener('drop', handleDrop, true)
})
</script>

<style scoped>
.prose {
  color: inherit;
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
  background-color: hsl(var(--b2));
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-size: 0.875em;
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
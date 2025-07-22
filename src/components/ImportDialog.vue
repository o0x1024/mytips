<template>
  <div v-if="show">
    <!-- Normal Modal View -->
    <template v-if="!isMinimized">
      <div class="modal modal-open">
        <div class="modal-box max-w-4xl">
          <h3 class="font-bold text-lg mb-4">导入Markdown文档</h3>
          
          <!-- 步骤指示器 -->
          <div class="steps w-full mb-6">
            <div class="step" :class="{ 'step-primary': currentStep >= 1 }">选择来源</div>
            <div class="step" :class="{ 'step-primary': currentStep >= 2 }">预览</div>
            <div class="step" :class="{ 'step-primary': currentStep >= 3 }">导入</div>
          </div>

          <!-- Step 1, 2, 3 content... -->
          <!-- 步骤1: 选择来源 -->
          <div v-if="currentStep === 1" class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">导入类型</span>
              </label>
              <div class="flex gap-4">
                <label class="label cursor-pointer">
                  <input type="radio" name="import-type" class="radio radio-primary" v-model="importType" value="directory" />
                  <span class="label-text ml-2">从文件夹导入</span>
                </label>
                <label class="label cursor-pointer">
                  <input type="radio" name="import-type" class="radio radio-primary" v-model="importType" value="file" />
                  <span class="label-text ml-2">从单个文件导入</span>
                </label>
                <label class="label cursor-pointer">
                  <input type="radio" name="import-type" class="radio radio-primary" v-model="importType" value="github" />
                  <span class="label-text ml-2">从 GitHub 导入</span>
                </label>
              </div>
            </div>

            <div v-if="importType === 'directory' || importType === 'file'">
              <div class="form-control">
                <label class="label">
                  <span class="label-text">{{ importType === 'directory' ? '选择文件夹' : '选择文件' }}</span>
                </label>
                <div class="input-group">
                  <input 
                    type="text" 
                    placeholder="点击选择..." 
                    class="input input-bordered flex-1" 
                    :value="selectedPath"
                    readonly
                  />
                  <button class="btn btn-primary" @click="selectPath">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    </svg>
                    选择
                  </button>
                </div>
              </div>
            </div>

            <!-- GitHub Import Options -->
            <div v-if="importType === 'github'" class="space-y-4">
              <div class="form-control">
                <label class="label">
                  <span class="label-text">GitHub 仓库 URL</span>
                </label>
                <input type="text" placeholder="例如: https://github.com/user/repo" class="input input-bordered w-full" v-model="githubOptions.repo_url" />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">分支或 Tag (可选)</span>
                  </label>
                  <input type="text" placeholder="默认主分支" class="input input-bordered w-full" v-model="githubOptions.branch" />
                </div>
                <div class="form-control">
                  <label class="label">
                    <span class="label-text">子目录 (可选)</span>
                  </label>
                  <input type="text" placeholder="例如: docs" class="input input-bordered w-full" v-model="githubOptions.subdirectory" />
                </div>
              </div>
              <div class="form-control">
                <label class="label">
                  <span class="label-text">Personal Access Token (可选)</span>
                  <span class="label-text-alt">
                    <a href="https://github.com/settings/tokens" target="_blank" class="link link-primary text-xs">用于私有仓库</a>
                  </span>
                </label>
                <input type="password" placeholder="粘贴你的 Token" class="input input-bordered w-full" v-model="githubOptions.token" />
              </div>
            </div>


            <div v-if="importType === 'directory' || importType === 'github'" class="space-y-2">
              <div class="form-control">
                <label class="label">
                  <span class="label-text">导入选项</span>
                </label>
                <div class="space-y-2">
                  <label class="label cursor-pointer justify-start">
                    <input type="checkbox" class="checkbox checkbox-primary" v-model="options.include_subdirs" />
                    <span class="label-text ml-2">包含子文件夹</span>
                  </label>
                  <label class="label cursor-pointer justify-start">
                    <input type="checkbox" class="checkbox checkbox-primary" v-model="options.process_images" />
                    <span class="label-text ml-2">处理图片引用</span>
                  </label>

                  <!-- 图片压缩选项 -->
                  <div v-if="options.process_images" class="pl-8 space-y-2 mt-2 border-l-2 border-base-300">
                    <label class="label cursor-pointer justify-start">
                      <input type="checkbox" class="checkbox checkbox-sm" v-model="options.image_compression.enabled" />
                      <span class="label-text ml-2">压缩图片</span>
                    </label>

                    <div v-if="options.image_compression.enabled" class="space-y-3 p-3 bg-base-200/50 rounded-lg">
                      <div class="form-control">
                        <label class="label pb-0">
                          <span class="label-text text-sm">图片质量 (1-100)</span>
                          <span class="label-text-alt">{{ options.image_compression.quality }}%</span>
                        </label>
                        <input type="range" min="1" max="100" v-model.number="options.image_compression.quality" class="range range-primary range-xs" />
                      </div>
                      <div class="grid grid-cols-2 gap-4">
                        <div class="form-control">
                          <label class="label py-0">
                            <span class="label-text text-sm">最大宽度 (px)</span>
                          </label>
                          <input type="number" v-model.number="options.image_compression.max_width" class="input input-bordered input-sm" placeholder="例如: 1920" />
                        </div>
                        <div class="form-control">
                          <label class="label py-0">
                            <span class="label-text text-sm">最大高度 (px)</span>
                          </label>
                          <input type="number" v-model.number="options.image_compression.max_height" class="input input-bordered input-sm" placeholder="例如: 1080" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text">重名处理</span>
                </label>
                <select class="select select-bordered" v-model="options.conflict_resolution">
                  <option value="Rename">重命名</option>
                  <option value="Skip">跳过</option>
                  <option value="Merge">合并（仅笔记本）</option>
                </select>
              </div>

              <div class="form-control">
                <label class="label">
                  <span class="label-text">目标笔记本</span>
                </label>
                <select class="select select-bordered" v-model="targetNotebookId">
                  <option value="">创建新笔记本</option>
                  <option v-for="notebook in notebooks" :key="notebook.id" :value="notebook.id">
                    {{ notebook.name }}
                  </option>
                </select>
              </div>
            </div>

            <div v-if="importType === 'file'" class="form-control">
              <label class="label">
                <span class="label-text">目标笔记本</span>
              </label>
              <select class="select select-bordered" v-model="targetNotebookId" required>
                <option value="" disabled>选择笔记本</option>
                <option v-for="notebook in notebooks" :key="notebook.id" :value="notebook.id">
                  {{ notebook.name }}
                </option>
              </select>
            </div>
          </div>

          <!-- 步骤2: 预览 -->
          <div v-if="currentStep === 2" class="space-y-4">
            <div v-if="isLoadingPreview" class="flex items-center justify-center py-8">
              <span class="loading loading-spinner loading-lg"></span>
              <span class="ml-2">正在扫描文件...</span>
            </div>

            <div v-else-if="preview" class="space-y-4">
              <div class="stats shadow">
                <div class="stat">
                  <div class="stat-title">笔记本</div>
                  <div class="stat-value">{{ preview.notebooks.length }}</div>
                </div>
                <div class="stat">
                  <div class="stat-title">笔记</div>
                  <div class="stat-value">{{ preview.total_notes }}</div>
                </div>
                <div class="stat">
                  <div class="stat-title">图片</div>
                  <div class="stat-value">{{ preview.total_images }}</div>
                </div>
              </div>

              <div class="overflow-x-auto max-h-64">
                <div class="space-y-2">
                  <div v-for="notebook in preview.notebooks" :key="notebook.path" class="card card-compact bg-base-200">
                    <div class="card-body">
                      <h4 class="card-title text-sm">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        </svg>
                        {{ notebook.name }}
                      </h4>
                      <div v-if="notebook.notes.length > 0" class="ml-4 space-y-1">
                        <div v-for="note in notebook.notes" :key="note.path" class="text-sm">
                          <span class="text-base-content/70">📄</span>
                          {{ note.title }}
                          <span v-if="note.images.length > 0" class="badge badge-sm badge-outline">
                            {{ note.images.length }} 图片
                          </span>
                        </div>
                      </div>
                      <div v-if="notebook.children.length > 0" class="ml-4">
                        <!-- 这里可以递归显示子笔记本，为简化先显示数量 -->
                        <span class="text-sm text-base-content/70">+ {{ notebook.children.length }} 个子文件夹</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 步骤3: 导入进度 -->
          <div v-if="currentStep === 3" class="space-y-4">
            <div v-if="isImporting" class="space-y-4">
              <div class="flex items-center justify-center">
                <span class="loading loading-spinner loading-lg"></span>
                <span class="ml-2">{{ importStatusMessage }}</span>
              </div>
              <progress class="progress progress-primary w-full" :value="importProgress" :max="importTotalFiles"></progress>
              <div class="text-center text-sm text-base-content/70">
                {{ importProgress }} / {{ importTotalFiles }}
                <p v-if="currentImportFile" class="truncate ...">{{ currentImportFile }}</p>
              </div>
            </div>

            <div v-else-if="importResult" class="space-y-4">
              <div class="alert" :class="importResult.success ? 'alert-success' : 'alert-error'">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path v-if="importResult.success" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>
                  {{ importResult.success ? '导入完成！' : '导入失败' }}
                </span>
              </div>

              <div class="stats shadow">
                <div class="stat">
                  <div class="stat-title">创建笔记本</div>
                  <div class="stat-value">{{ importResult.notebooks_created }}</div>
                </div>
                <div class="stat">
                  <div class="stat-title">导入笔记</div>
                  <div class="stat-value">{{ importResult.notes_imported }}</div>
                </div>
                <div class="stat">
                  <div class="stat-title">处理图片</div>
                  <div class="stat-value">{{ importResult.images_processed }}</div>
                </div>
              </div>

              <div v-if="importResult.warnings.length > 0" class="space-y-2">
                <h4 class="font-semibold">警告信息：</h4>
                <div class="bg-yellow-50 border border-yellow-200 rounded p-3 max-h-32 overflow-y-auto">
                  <ul class="text-sm space-y-1">
                    <li v-for="warning in importResult.warnings" :key="warning" class="text-yellow-800">
                      • {{ warning }}
                    </li>
                  </ul>
                </div>
              </div>

              <div v-if="importResult.errors.length > 0" class="space-y-2">
                <h4 class="font-semibold">错误信息：</h4>
                <div class="bg-red-50 border border-red-200 rounded p-3 max-h-32 overflow-y-auto">
                  <ul class="text-sm space-y-1">
                    <li v-for="error in importResult.errors" :key="error" class="text-red-800">
                      • {{ error }}
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="modal-action">
            <button class="btn" @click="cancel">取消</button>
            <button 
              v-if="currentStep === 1" 
              class="btn btn-primary" 
              @click="nextStep" 
              :disabled="!canProceedFromStep1"
            >
              下一步
            </button>
            <button 
              v-if="currentStep === 2" 
              class="btn" 
              @click="prevStep"
            >
              上一步
            </button>
            <button 
              v-if="currentStep === 2" 
              class="btn btn-primary" 
              @click="startImport"
              :disabled="!preview"
            >
              开始导入
            </button>
            <button
              v-if="currentStep === 3 && isImporting"
              class="btn"
              @click="runInBackground"
            >
              后台运行
            </button>
            <button 
              v-if="currentStep === 3 && !isImporting" 
              class="btn btn-primary" 
              @click="finish"
            >
              完成
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="cancel"></div>
      </div>
    </template>

    <!-- Minimized Widget View -->
    <div
      v-if="isMinimized"
      class="fixed bottom-8 right-8 z-50 w-96 bg-base-100 rounded-lg shadow-2xl p-4 cursor-pointer pointer-events-auto"
      @click="restoreDialog"
    >
      <div class="flex justify-between items-center mb-2">
        <h4 class="font-bold text-sm">
          {{ isImporting ? '正在后台导入...' : '导入完成' }}
        </h4>
        <span v-if="isImporting" class="text-xs opacity-70">点击展开</span>
        <button v-else class="btn btn-xs btn-ghost" @click.stop="finish">完成</button>
      </div>
      <progress class="progress progress-primary w-full" :value="importProgress" :max="importTotalFiles"></progress>
      <div class="text-center text-xs text-base-content/70 truncate mt-1">
        <span>{{ importProgress }} / {{ importTotalFiles }}</span>
        <p v-if="isImporting && currentImportFile" class="truncate ...">{{ currentImportFile }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { showAlert } from '../services/dialog'
import { showToast } from '../services/notification'

// 组件属性
const props = defineProps<{
  show: boolean
  notebooks: Array<{ id: string; name: string }>
}>()

// 组件事件
const emit = defineEmits<{
  close: []
  success: [result: any]
}>()

// 状态
const currentStep = ref(1)
const importType = ref('directory')
const selectedPath = ref('')
const targetNotebookId = ref('')
const isLoadingPreview = ref(false)
const isImporting = ref(false)
const importProgress = ref(0)
const importTotalFiles = ref(0)
const currentImportFile = ref('')
const importStatusMessage = ref('正在准备导入...')
let unlisten: UnlistenFn | null = null
const isMinimized = ref(false)

// GitHub 导入选项
const githubOptions = ref({
  repo_url: '',
  branch: '',
  subdirectory: '',
  token: ''
})

// 导入选项
const options = ref({
  include_subdirs: true,
  process_images: true,
  conflict_resolution: 'Rename' as 'Rename' | 'Skip' | 'Merge',
  image_compression: {
    enabled: true,
    quality: 80,
    max_width: 1920,
    max_height: 1920,
  }
})

// 预览和结果
const preview = ref<any>(null)
const importResult = ref<any>(null)

// 计算属性
const canProceedFromStep1 = computed(() => {
  if (importType.value === 'directory') {
    return selectedPath.value !== ''
  } else if (importType.value === 'file') {
    return selectedPath.value !== '' && targetNotebookId.value !== ''
  } else if (importType.value === 'github') {
    // 简单校验 GitHub URL
    return githubOptions.value.repo_url.startsWith('https://github.com/')
  }
  return false
})

// 方法
async function selectPath() {
  try {
    const selected = await open({
      directory: importType.value === 'directory',
      multiple: false,
      filters: importType.value === 'file' ? [
        {
          name: 'Markdown',
          extensions: ['md', 'markdown']
        }
      ] : undefined
    })

    if (selected) {
      selectedPath.value = selected as string
    }
  } catch (error) {
    console.error('选择路径失败:', error)
  }
}

async function nextStep() {
  if (currentStep.value === 1) {
    if (importType.value === 'directory') {
      // 获取预览
      await loadPreview()
    } else if (importType.value === 'file') {
      // 对于单个文件，创建简单的预览数据
      const fileName = selectedPath.value.split(/[/\\]/).pop() || '未知文件'
      const fileTitle = fileName.replace(/\.(md|markdown)$/i, '')
      const targetNotebook = props.notebooks.find(n => n.id === targetNotebookId.value)
      
      preview.value = {
        notebooks: [{
          name: targetNotebook?.name || '选中的笔记本',
          path: selectedPath.value,
          notes: [{
            title: fileTitle,
            path: selectedPath.value,
            images: []
          }],
          children: []
        }],
        total_notes: 1,
        total_images: 0
      }
    } else if (importType.value === 'github') {
      // 对于 GitHub 导入，跳过预览，直接进入下一步
      // 后端会扫描文件，所以可以创建一个简单的占位符
       const repoName = githubOptions.value.repo_url.split('/').pop() || 'GitHub Repo';
      preview.value = {
        notebooks: [{
          name: repoName,
          path: githubOptions.value.repo_url,
          notes: [],
          children: []
        }],
        total_notes: '未知', // 显示未知，因为我们跳过了预览
        total_images: '未知'
      }
    }
    currentStep.value = 2
  }
}

function prevStep() {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

async function loadPreview() {
  if (importType.value !== 'directory' || !selectedPath.value) return

  isLoadingPreview.value = true
  try {
    preview.value = await invoke('get_import_preview', {
      directoryPath: selectedPath.value
    })
  } catch (error) {
    console.error('获取预览失败:', error)
    await showAlert('获取预览失败: ' + error, { title: '错误' })
  } finally {
    isLoadingPreview.value = false
  }
}

async function startImport() {
  isImporting.value = true
  importProgress.value = 0
  importTotalFiles.value = 0
  currentImportFile.value = ''
  importStatusMessage.value = '正在启动导入...'
  currentStep.value = 3

  try {
    if (importType.value === 'directory') {
      // 设置事件监听器来接收进度更新
      unlisten = await listen('import-progress', (event: any) => {
        const progress = event.payload
        
        // Always update UI state
        importStatusMessage.value = getStatusMessage(progress.status)
        importTotalFiles.value = progress.totalFiles
        importProgress.value = progress.processedFiles
        currentImportFile.value = progress.currentFile

        if (progress.status === 'completed' || progress.status === 'error') {
            importResult.value = progress.result
            isImporting.value = false

            // If minimized, also show a toast notification
            if (isMinimized.value) {
                const { success, notes_imported, notebooks_created, errors } = progress.result;
                const message = success
                    ? `后台导入完成: 导入 ${notes_imported} 篇笔记, 创建 ${notebooks_created} 个笔记本。`
                    : `后台导入失败: ${errors && errors.length > 0 ? errors[0] : '未知错误'}`;
                showToast(message, success ? 'success' : 'error', 10000);
            }

            if (unlisten) {
                unlisten()
                unlisten = null
            }
        }
      })

      // 异步调用，不会阻塞
      await invoke('import_from_directory', {
        directoryPath: selectedPath.value,
        targetNotebookId: targetNotebookId.value || null,
        options: options.value
      })

    } else if (importType.value === 'github') {
      // GitHub 导入也使用事件监听
      unlisten = await listen('import-progress', (event: any) => {
        const progress = event.payload
        
        importStatusMessage.value = getStatusMessage(progress.status)
        importTotalFiles.value = progress.totalFiles
        importProgress.value = progress.processedFiles
        currentImportFile.value = progress.currentFile

        if (progress.status === 'completed' || progress.status === 'error') {
            importResult.value = progress.result
            isImporting.value = false
            if (isMinimized.value) {
                const { success, notes_imported, notebooks_created, errors } = progress.result;
                const message = success
                    ? `后台导入完成: 导入 ${notes_imported} 篇笔记, 创建 ${notebooks_created} 个笔记本。`
                    : `后台导入失败: ${errors && errors.length > 0 ? errors[0] : '未知错误'}`;
                showToast(message, success ? 'success' : 'error', 10000);
            }
            if (unlisten) {
                unlisten()
                unlisten = null
            }
        }
      })

      // 异步调用 GitHub 导入
      await invoke('import_from_github', {
        githubOptions: githubOptions.value,
        targetNotebookId: targetNotebookId.value || null,
        options: options.value
      })

    } else {
      // 单文件导入保持同步逻辑
      const result = await invoke('import_markdown_file', {
        filePath: selectedPath.value,
        targetNotebookId: targetNotebookId.value
      })
      importResult.value = result
      importProgress.value = 1
      importTotalFiles.value = 1
      isImporting.value = false
    }
  } catch (error) {
    console.error('启动导入失败:', error)
    importResult.value = {
      success: false,
      notebooks_created: 0,
      notes_imported: 0,
      images_processed: 0,
      errors: [String(error)],
      warnings: []
    }
    isImporting.value = false
  }
}

function runInBackground() {
  console.log('runInBackground function was called')
  isMinimized.value = true
}

function restoreDialog() {
  isMinimized.value = false
}

function getStatusMessage(status: string): string {
  switch (status) {
    case 'starting':
      return '正在开始...'
    case 'cloning':
      return '正在克隆仓库...'
    case 'scanning':
      return '正在扫描文件...'
    case 'inProgress':
      return '正在导入文件...'
    case 'completed':
      return '导入完成！'
    case 'error':
      return '导入时发生错误'
    default:
      return '正在导入...'
  }
}

async function cancel() {
  if (isImporting.value) {
    try {
      await invoke('cancel_import')
      showToast('正在取消导入...', 'info')
    } catch (e) {
      console.error('Failed to cancel import', e)
      showToast('取消导入失败', 'error')
    }
  }
  if (unlisten) {
    unlisten()
    unlisten = null
  }
  resetDialog()
  emit('close')
}

function finish() {
  emit('success', importResult.value)
  resetDialog()
  emit('close')
}

function resetDialog() {
  currentStep.value = 1
  importType.value = 'directory'
  selectedPath.value = ''
  targetNotebookId.value = ''
  preview.value = null
  importResult.value = null
  isLoadingPreview.value = false
  isImporting.value = false
  importProgress.value = 0
  importTotalFiles.value = 0
  currentImportFile.value = ''
  isMinimized.value = false
  githubOptions.value = {
    repo_url: '',
    branch: '',
    subdirectory: '',
    token: ''
  }
  options.value = {
    include_subdirs: true,
    process_images: true,
    conflict_resolution: 'Rename',
    image_compression: {
      enabled: true,
      quality: 80,
      max_width: 1920,
      max_height: 1920,
    }
  }
}

// 监听显示状态变化
watch(() => props.show, (newShow) => {
  if (!newShow) {
    if (unlisten) {
      unlisten()
      unlisten = null
    }
    resetDialog()
  }
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})
</script> 
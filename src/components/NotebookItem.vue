<template>
  <div >
    <!-- 笔记本项目 -->
    <div class="flex items-center w-full">
      <a 
        :class="[
          'flex items-center w-full py-1 rounded-md min-w-0 cursor-pointer hover:bg-base-200 transition-colors', 
          {'active bg-primary/10 text-primary font-medium': selectedId === notebook.id}
        ]"
        @click="selectThis"
        @contextmenu.prevent="onContextMenu"
        :title="notebook.name"
        style="overflow: hidden;"
      >
        <!-- 展开/折叠按钮 (如果有子项) -->
        <button 
          v-if="notebook.children && notebook.children.length > 0 && !isCollapsed" 
          class="btn btn-xs btn-ghost btn-square mr-1"
          @click.stop="toggleExpanded"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  :d="isExpanded ? 'M19 9l-7 7-7-7' : 'M9 5l7 7-7 7'" />
          </svg>
        </button>
        
        <!-- 无子项时的占位图标 -->
        <div 
          v-if="(!notebook.children || notebook.children.length === 0) && !isCollapsed" 
          class="w-6 h-6 flex items-center justify-center mr-1"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-opacity="0.3">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
            
          </svg>
        </div>
        
        <!-- 笔记本图标 -->
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" :class="{'mr-1': !isCollapsed}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        
        <!-- 笔记本名称 -->
        <span v-if="!isCollapsed && !isEditing" class="notebook-name">
          <slot name="name" :name="notebook.name">
            {{ notebook.name }}
          </slot>
        </span>
        <input 
          v-if="!isCollapsed && isEditing"
          type="text"
          ref="editInput"
          v-model="newName"
          class="input input-xs w-full"
          @keyup.enter="finishEditing"
          @blur="finishEditing"
          @click.stop
        />
        
        <!-- 加密状态指示器 -->
        <span v-if="!isCollapsed && isNotebookEncrypted(notebook.id) && !isEditing" :title="$t('notebookItem.encrypted')" class="text-warning ml-1">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
        </span>
        
        <!-- 笔记数量指示器 -->
        <span v-if="!isCollapsed && !isEditing" class="badge badge-sm ml-auto">{{ notebook.count || 0 }}</span>
      </a>
      
      <!-- 操作菜单 -->
      <div v-if="!isCollapsed" class="dropdown dropdown-end">
        <button tabindex="0" class="btn btn-xs btn-ghost btn-square">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
          </svg>
        </button>
        <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-48">
          <li><a @click="encryptNotebook" class="text-warning">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
            {{ $t('notebookItem.encryptNotebook') }}
          </a></li>
          <li><a @click="decryptNotebook" class="text-info">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2 2v6a2 2 0 002 2z" />
            </svg>
            {{ $t('notebookItem.decryptNotebook') }}
          </a></li>
          <div class="divider my-0"></div>
          <li><a @click="editNotebook">{{ $t('notebookItem.rename') }}</a></li>
          <li><a @click="addChildNotebook">{{ $t('notebookItem.addChildNotebook') }}</a></li>
          <li>
            <details>
              <summary>{{ $t('notebookItem.export') }}</summary>
              <ul>
                <li><a @click="exportToFolder">{{ $t('notebookItem.exportToFolder') }}</a></li>
                <li><a @click="exportToPdf">{{ $t('notebookItem.exportToPdf') }}</a></li>
                <li><a @click="exportToWord">{{ $t('notebookItem.exportToWord') }}</a></li>
              </ul>
            </details>
          </li>
          <div class="divider my-0"></div>
          <li><a @click="deleteNotebook" class="text-error">{{ $t('notebookItem.delete') }}</a></li>
        </ul>
      </div>
    </div>
    
    <!-- 子笔记本递归渲染 - 展开模式 -->
    <div v-if="isExpanded && notebook.children && notebook.children.length > 0 && !isCollapsed" 
         class="pl-4 mt-1 border-l border-base-300 ml-3">
      <NotebookItem 
        v-for="child in notebook.children" 
        :key="child.id"
        :notebook="child"
        :is-collapsed="isCollapsed"
        :selected-id="selectedId"
        @select="(id: string) => $emit('select', id)"
        @edit="(data: any) => $emit('edit', data)"
        @add-child="(id: string) => $emit('add-child', id)"
        @delete="(id: string) => $emit('delete', id)"
        @encrypt="(id: string) => $emit('encrypt', id)"
        @decrypt="(id: string) => $emit('decrypt', id)"
      />
    </div>
    
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, defineEmits, watch, onMounted, nextTick, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { useEncryptionStore } from '../stores/encryptionStore'
import { invoke } from '@tauri-apps/api/core'
import { useNotificationStore } from '../stores/notificationStore'

const { t } = useI18n()

// 定义类型
interface NotebookType {
  id: string;
  name: string;
  count?: number;
  children?: NotebookType[];
}

// 组件属性
const props = defineProps({
  notebook: {
    type: Object as () => NotebookType,
    required: true
  },
  isCollapsed: {
    type: Boolean,
    default: false
  },
  selectedId: {
    type: String,
    default: null
  }
})

// 组件事件
const emit = defineEmits([
  'select', 
  'edit', 
  'delete', 
  'add-child',
  'edit-child',
  'add-grandchild',
  'delete-child',
  'encrypt',
  'decrypt',
  'export-to-folder',
  'export-to-pdf',
  'export-to-word'
])

// 加密存储
const encryptionStore = useEncryptionStore()
const notificationStore = useNotificationStore()

// 状态
const isExpanded = ref(false)
const showContextMenu = ref(false)
const menuX = ref(0)
const menuY = ref(0)
const isEditing = ref(false)
const newName = ref('')
const editInput = ref<HTMLInputElement | null>(null)

// 检查笔记本是否加密
function isNotebookEncrypted(notebookId: string): boolean {
  const result = encryptionStore.isItemEncrypted(notebookId)
  // 添加调试信息，帮助排查问题
  if (result) {
    console.log(`Notebook ${notebookId} is detected as encrypted.`)
  }
  return result
}

// 方法
function toggleExpanded(event: Event) {
  event.stopPropagation()
  isExpanded.value = !isExpanded.value
}

function selectThis() {
  emit('select', props.notebook.id)
}

function startEditing() {
  isEditing.value = true
  newName.value = props.notebook.name
  nextTick(() => {
    editInput.value?.focus()
    editInput.value?.select()
  })
}

function finishEditing() {
  if (isEditing.value && newName.value.trim() && newName.value.trim() !== props.notebook.name) {
    emit('edit', { id: props.notebook.id, name: newName.value.trim() })
  }
  isEditing.value = false
}

function editNotebook(event: Event) {
  event.stopPropagation()
  startEditing()
}

function addChildNotebook(event: Event) {
  event.stopPropagation()
  emit('add-child', props.notebook.id)
}

function deleteNotebook(event: Event) {
  event.stopPropagation()
  emit('delete', props.notebook.id)
}

function encryptNotebook(event: Event) {
  event.stopPropagation()
  emit('encrypt', props.notebook.id)
}

function decryptNotebook(event: Event) {
  event.stopPropagation()
  emit('decrypt', props.notebook.id)
}

async function exportToFolder(event: Event) {
  event.stopPropagation()
  try {
    await invoke('export_notebook_to_folder', { notebookId: props.notebook.id })
    notificationStore.addToast(t('notebookItem.exportSuccess'), 'success')
  } catch (error) {
    notificationStore.addToast(t('notebookItem.exportFailure', { error: error as string }), 'error')
  }
}

async function exportToPdf(event: Event) {
  event.stopPropagation()
  try {
    await invoke('export_notebook_to_pdf', { notebookId: props.notebook.id })
    notificationStore.addToast(t('notebookItem.exportSuccess'), 'success')
  } catch (error) {
    notificationStore.addToast(t('notebookItem.exportFailure', { error: error as string }), 'error')
  }
}

async function exportToWord(event: Event) {
  event.stopPropagation()
  try {
    await invoke('export_notebook_to_word', { notebookId: props.notebook.id })
    notificationStore.addToast(t('notebookItem.exportSuccess'), 'success')
  } catch (error) {
    notificationStore.addToast(t('notebookItem.exportFailure', { error: error as string }), 'error')
  }
}

function onContextMenu(e: MouseEvent) {
  e.preventDefault()
  window.dispatchEvent(new Event('close-all-notebook-context-menu'))
  nextTick(() => {
    showContextMenu.value = true
    menuX.value = e.clientX
    menuY.value = e.clientY
  })
}

function closeContextMenu() {
  showContextMenu.value = false
}

// 检查是否应该展开（当前节点被选中或子节点被选中）
function shouldExpand(): boolean {
  if (props.selectedId === props.notebook.id) {
    return true
  }
  
  if (props.notebook.children && props.notebook.children.length > 0) {
    return checkChildrenSelected(props.notebook.children, props.selectedId)
  }
  
  return false
}

// 递归检查子节点是否被选中
function checkChildrenSelected(children: NotebookType[], selectedId: string | null): boolean {
  if (!selectedId) return false
  
  for (const child of children) {
    if (child.id === selectedId) {
      return true
    }
    
    if (child.children && child.children.length > 0) {
      if (checkChildrenSelected(child.children, selectedId)) {
        return true
      }
    }
  }
  
  return false
}

// 监听选中状态变化
watch(() => props.selectedId, (_newVal) => {
  // 只在shouldExpand为true时展开，否则收起
  if (shouldExpand()) {
    isExpanded.value = true
  } else {
    isExpanded.value = false
  }
}, { immediate: true })

// 监听notebook变化，尤其是当children发生变化时
watch(() => props.notebook.children, () => {
  // 只在shouldExpand为true时展开，否则收起
  if (shouldExpand()) {
    isExpanded.value = true
  } else {
    isExpanded.value = false
  }
}, { deep: true })

// 组件挂载时检查是否需要展开
onMounted(() => {
  window.addEventListener('close-all-notebook-context-menu', closeContextMenu)
  window.addEventListener('click', closeContextMenu, true)
  // 只在shouldExpand为true时展开
  if (shouldExpand()) {
    isExpanded.value = true
  } else {
    isExpanded.value = false
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('close-all-notebook-context-menu', closeContextMenu)
  window.removeEventListener('click', closeContextMenu, true)
})
</script>

<style scoped>
.notebook-name {
  /* flex: 1 1 0%; */
  /* min-width: 0; */
  white-space: nowrap;
  /* overflow: hidden; */
  /* text-overflow: ellipsis; */
  word-break: normal;
  max-width: 100px;
}
</style> 
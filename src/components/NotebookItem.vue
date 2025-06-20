<template>
  <div >
    <!-- 笔记本项目 -->
    <div class="flex items-center w-full">
      <a 
        :class="[
          'flex items-center w-full py-1 rounded-md min-w-0', 
          {'active bg-primary/10': selectedId === notebook.id}
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
        <span v-if="!isCollapsed" class="notebook-name">
          <slot name="name" :name="notebook.name">
            {{ notebook.name }}
          </slot>
        </span>
        
        <!-- 笔记数量指示器 -->
        <span v-if="!isCollapsed" class="badge badge-sm ml-auto">{{ getTotalCount(notebook) }}</span>
      </a>
      
      <!-- 操作菜单 -->
      <div v-if="!isCollapsed" class="dropdown dropdown-end">
        <button tabindex="0" class="btn btn-xs btn-ghost btn-square">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
          </svg>
        </button>
        <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-36">
          <li><a @click="editNotebook">编辑</a></li>
          <li><a @click="addChildNotebook">添加子笔记本</a></li>
          <li><a @click="deleteNotebook" class="text-error">删除</a></li>
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
        @select="id => $emit('select', id)"
        @edit="id => $emit('edit', id)"
        @add-child="id => $emit('add-child', id)"
        @delete="id => $emit('delete', id)"
      />
    </div>
    
    <!-- 右键菜单 -->
    <ul v-if="showContextMenu" class="dropdown-content z-50 menu p-2 shadow bg-base-100 rounded-box w-36" :style="{ position: 'fixed', left: menuX + 'px', top: menuY + 'px' }">
      <li><a @click="handleEdit">编辑</a></li>
      <li><a @click="handleAddChild">添加子笔记本</a></li>
      <li><a @click="handleDelete" class="text-error">删除</a></li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, defineEmits, watch, onMounted, nextTick, onBeforeUnmount } from 'vue'

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
  'delete-child'
])

// 状态
const isExpanded = ref(false)
const showContextMenu = ref(false)
const menuX = ref(0)
const menuY = ref(0)

// 方法
function toggleExpanded(event: Event) {
  event.stopPropagation()
  isExpanded.value = !isExpanded.value
}

function selectThis() {
  emit('select', props.notebook.id)
}

function editNotebook(event: Event) {
  event.stopPropagation()
  emit('edit', props.notebook.id)
}

function addChildNotebook(event: Event) {
  event.stopPropagation()
  emit('add-child', props.notebook.id)
}

function deleteNotebook(event: Event) {
  event.stopPropagation()
  emit('delete', props.notebook.id)
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

// 点击菜单项后关闭菜单
function handleEdit() {
  closeContextMenu()
  emit('edit', props.notebook.id)
}
function handleAddChild() {
  closeContextMenu()
  emit('add-child', props.notebook.id)
}
function handleDelete() {
  closeContextMenu()
  emit('delete', props.notebook.id)
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

// 递归统计所有子孙节点的笔记总数
function getTotalCount(notebook: NotebookType): number {
  let total = notebook.count || 0
  if (notebook.children && notebook.children.length > 0) {
    for (const child of notebook.children) {
      total += getTotalCount(child)
    }
  }
  return total
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
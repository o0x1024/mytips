<template>
  <div class="h-full w-full flex flex-col bg-base-200 border-r border-base-300 transition-all duration-300 sidebar-width">
    <!-- 顶部标题和折叠按钮 -->
    <div class="p-3 flex items-center justify-between">
      <div :class="['transition-opacity duration-300 text-2xl font-bold text-primary relative flex items-center', isCollapsed ? 'opacity-0 w-0 absolute' : 'opacity-100']">
        MyTips
        <!-- 更新Badge -->
        <div v-if="updateStore.hasUpdate" class="badge badge-error badge-xs ml-2 animate-pulse">
          新版本
        </div>
      </div>
      <button 
        :class="['btn btn-sm btn-ghost relative z-10', isCollapsed ? 'btn btn-ghost btn-xs btn-square w-14 flex justify-center tooltip tooltip-righ' : 'ml-auto']" 
        @click.stop="toggleCollapse" 
        :title="isCollapsed ? '展开侧边栏' : '收起侧边栏'"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path v-if="isCollapsed" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
        <!-- 折叠状态下的更新指示器 -->
        <div v-if="isCollapsed && updateStore.hasUpdate" class="badge badge-error badge-xs absolute -top-1 -right-1">
          !
        </div>
      </button>
    </div>

    <!-- 搜索框 -->
    <div :class="[isCollapsed ? 'px-2' : 'px-4 mb-2']">
      <div class="relative transition-all duration-300" v-if="!isCollapsed">
        <input type="text" placeholder="搜索..." 
               class="input input-bordered input-sm w-full pl-8" 
               v-model="searchQuery" 
               @input="$emit('search', searchQuery)"
               @keyup.enter="$emit('search', searchQuery)" />
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 absolute left-2 top-1/2 transform -translate-y-1/2 text-base-content/80" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </div>
    </div>

    <!-- 导航区域 -->
    <div class="overflow-y-auto overflow-x-hidden flex-grow px-2">
      <!-- 笔记本区域 -->
      <div class="mb-1">
        <div v-if="!isCollapsed" class="flex justify-between items-center mt-2 px-2">
          <span class="text-base font-bold uppercase text-base-content/80 ">笔记本</span>
          <div class="flex gap-1">
            <button class="btn btn-xs btn-ghost" @click="$emit('import')" title="导入文档">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
              </svg>  
            </button>
            <button class="btn btn-xs btn-ghost " @click="$emit('add-notebook')" title="添加笔记本">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>  
            </button>
          </div>
        </div>
        
        <!-- 笔记本列表 - 展开模式 -->
        <div v-if="!isCollapsed" class="menu w-full" style="white-space: normal; word-break: break-all;">
          <template v-if="notebooks.length > 0">
            <template v-for="notebook in notebooks" :key="notebook.id">
              <div>
                <NotebookItem 
                  :notebook="notebook" 
                  :is-collapsed="isCollapsed"
                  :selected-id="selectedNotebookId"
                  @select="selectNotebook"
                  @edit="id => $emit('edit-notebook', id)"
                  @add-child="addChildNotebook"
                  @delete="deleteNotebook"
                  @encrypt="(id: string) => $emit('encrypt-notebook', id)"
                  @decrypt="(id: string) => $emit('decrypt-notebook', id)"
                >
                  <template #name="{ name }">
                    <span v-html="highlightKeyword(name, searchQuery)"></span>
                  </template>
                </NotebookItem>
              </div>
            </template>
          </template>
          <div v-else class="text-center text-base-content/80 py-4">
            <span v-if="searchQuery">未找到相关笔记本</span>
            <span v-else>暂无笔记本</span>
          </div>
        </div>
        
        <!-- 笔记本列表 - 折叠模式 -->
        <div v-if="isCollapsed" class="flex flex-col gap-1 my-2 items-center">
          <button 
            class="btn btn-ghost btn-sm btn-square w-14 flex  justify-center tooltip tooltip-right" 
            @click="toggleNotebooksPopup">
            
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <div class="badge badge-xs badge-primary absolute -top-1 -right-1" v-if="notebooks.length > 0">
              {{ notebooks.length > 99 ? '99+' : notebooks.length }}
            </div>
          </button>
          
          <!-- 笔记本弹出面板 -->
          <div 
            v-if="showNotebooksPopup" 
            class="fixed top-0 left-16 w-64 h-full bg-base-100 shadow-lg z-50 animate-slide-right border-r border-base-300"
            @click.stop>
            <div class="p-3 flex justify-between items-center border-b border-base-300">
              <span class="font-bold">笔记本</span>
              <button class="btn btn-sm btn-ghost" @click.stop="closeNotebooksPopup">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            <div class="max-h-[calc(100%-48px)] overflow-y-auto">
              <ul class="menu menu-compact">
                <template v-for="notebook in notebooks" :key="notebook.id">
                  <li :class="{'bordered': notebook.children && notebook.children.length > 0}">
                    <a 
                      @click="selectNotebookAndClose(notebook.id)"
                      :class="{'active': selectedNotebookId === notebook.id}">
                      <span class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                        </svg>
                        {{ notebook.name }}
                      </span>
                      <span v-if="notebook.count" class="badge badge-sm ml-auto">{{ notebook.count }}</span>
                    </a>
                    <!-- 子笔记本 -->
                    <ul v-if="notebook.children && notebook.children.length > 0">
                      <li v-for="child in notebook.children" :key="child.id">
                        <a 
                          @click="selectNotebookAndClose(child.id)"
                          :class="{'active': selectedNotebookId === child.id}">
                          {{ child.name }}
                          <span v-if="child.count" class="badge badge-sm ml-auto">{{ child.count }}</span>
                        </a>
                        <!-- 二级子笔记本 -->
                        <ul v-if="child.children && child.children.length > 0">
                          <li v-for="grandchild in child.children" :key="grandchild.id">
                            <a 
                              @click="selectNotebookAndClose(grandchild.id)"
                              :class="{'active': selectedNotebookId === grandchild.id}">
                              {{ grandchild.name }}
                              <span v-if="grandchild.count" class="badge badge-sm ml-auto">{{ grandchild.count }}</span>
                            </a>
                          </li>
                        </ul>
                      </li>
                    </ul>
                  </li>
                </template>
              </ul>
              <div v-if="notebooks.length === 0" class="px-4 py-2 text-sm text-base-content/80">
                暂无笔记本
              </div>
              <div class="p-3 flex justify-end">
                <button class="btn btn-sm btn-ghost" @click="addNotebookAndClose">添加笔记本</button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 标签区域 -->
      <div class="mb-4" v-if="!isCollapsed">
        <div class="flex justify-between items-center mb-2 px-2">
          <span class="text-base font-bold uppercase text-base-content/80">标签</span>
          <div class="flex items-center gap-1">
            <!-- 标签搜索按钮和输入框 -->
            <div class="relative">
              <button 
                class="btn btn-xs btn-ghost" 
                @click="toggleTagSearchInput"
                title="搜索标签">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </button>
              
              <!-- 搜索输入框（仅在showTagSearchInput为true时显示） -->
              <input 
                v-if="showTagSearchInput"
                type="text" 
                placeholder="搜索标签..." 
                class="absolute right-0 top-0 input input-xs input-bordered w-20 z-10" 
                v-model="tagSearchQuery"
                ref="tagSearchInputRef"
                @blur="hideTagSearchInputIfEmpty"
              />
            </div>
            
            <button class="btn btn-xs btn-ghost" @click="$emit('add-tag')" title="添加标签">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </button>
            <button class="btn btn-xs btn-ghost" @click="toggleTagsCollapsed" title="展开/折叠标签">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path v-if="tagsCollapsed" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
              </svg>
            </button>
          </div>
        </div>
        
        <!-- 标签列表 - 紧凑样式 -->
        <div v-if="!tagsCollapsed" class="px-2">
          <div v-if="tags.length > 0" class="flex flex-wrap gap-1 mb-1">
            <!-- 显示常用标签或选中的标签，仅当未展开全部标签时显示 -->
            <template v-if="!showAllTags">
              <div 
                v-for="tag in tagSearchQuery ? filteredTags.slice(0, maxVisibleTags) : visibleTags" 
                :key="tag.id" 
                class="badge badge-sm cursor-pointer group relative"
                :class="props.selectedTags.includes(tag.id) ? 'badge-primary' : 'badge-outline'"
                @click="toggleTag(tag.id)">
                {{ tag.name }}
                <button 
                  class="hidden group-hover:flex absolute -top-1.5 -right-1.5 bg-error text-white rounded-full w-3.5 h-3.5 items-center justify-center"
                  @click.stop="deleteTag(tag.id)"
                  title="删除标签">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-2.5 w-2.5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>

              <!-- 更多标签按钮 -->
              <div v-if="!tagSearchQuery && tags.length > maxVisibleTags" 
                  class="badge badge-sm badge-neutral cursor-pointer"
                  @click="showAllTags = true">
                更多 ({{ tags.length - maxVisibleTags }})
              </div>
            </template>
            
            <!-- 显示所有标签，当showAllTags为true时显示 -->
            <template v-else>
              <div 
                v-for="tag in tagSearchQuery ? filteredTags : tags" 
                :key="tag.id" 
                class="badge badge-sm cursor-pointer group relative"
                :class="props.selectedTags.includes(tag.id) ? 'badge-primary' : 'badge-outline'"
                @click="toggleTag(tag.id)">
                {{ tag.name }}
                <button 
                  class="hidden group-hover:flex absolute -top-1.5 -right-1.5 bg-error text-white rounded-full w-3.5 h-3.5 items-center justify-center"
                  @click.stop="deleteTag(tag.id)"
                  title="删除标签">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-2.5 w-2.5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
              
              <!-- 收起标签按钮 -->
              <div v-if="!tagSearchQuery && tags.length > maxVisibleTags" 
                  class="badge badge-sm badge-neutral cursor-pointer"
                  @click="showAllTags = false">
                收起
              </div>
            </template>
          </div>
          <div v-else class="text-xs text-base-content/80 py-2">
            暂无标签，请添加
          </div>
        </div>
      </div>
    </div>


    <!-- 底部操作按钮 -->
    <div class="mt-auto p-2 flex flex-col" :class="{'items-center': isCollapsed}">
      
      <button 
        :class="['btn mb-2', isCollapsed ? 'btn-ghost btn-sm btn-square w-14' : 'btn-outline w-full']"
        :data-tip="isCollapsed ? '临时笔记区' : ''"
        @click="$emit('clipboard')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
        <span v-if="!isCollapsed" class="ml-1">临时笔记区</span>
      </button>
      
      <button 
        :class="['btn mb-2', isCollapsed ? 'btn btn-ghost btn-sm btn-square w-14  flex justify-center tooltip tooltip-righ' : 'btn-outline w-full']" 
        :data-tip="isCollapsed ? 'AI助手' : ''"
        data-tip-class="tooltip-top-layer"
        @click="$emit('ai-assistant')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        <span v-if="!isCollapsed" class="ml-1">AI助手</span>
      </button>
      
      <button 
        :class="['btn', isCollapsed ? 'btn btn-ghost btn-sm btn-square w-14 flex justify-center tooltip tooltip-righ' : 'btn-outline w-full']" 
        :data-tip="isCollapsed ? '设置' : ''"
        data-tip-class="tooltip-top-layer"
        @click="$emit('settings')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
        <span v-if="!isCollapsed" class="ml-1">设置</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineEmits, defineProps, watch, onMounted, onBeforeUnmount, computed, nextTick, onActivated } from 'vue'
import NotebookItem from './NotebookItem.vue'
import { showConfirm } from '../services/dialog'
import { useUIStore } from '../stores/uiStore'
import { useUpdateStore } from '../stores/updateStore'
// const encryptionStore = useEncryptionStore()

// 获取UI存储
const uiStore = useUIStore()

// 获取更新状态存储
const updateStore = useUpdateStore()

// 定义类型
interface Notebook {
  id: string;
  name: string;
  count?: number;
  children: Notebook[];
}

interface Tag {
  id: string;
  name: string;
}

// 组件属性
const props = defineProps({
  notebooks: {
    type: Array as () => Notebook[],
    default: () => []
  },
  tags: {
    type: Array as () => Tag[],
    default: () => []
  },
  selectedNotebookId: {
    type: String,
    default: null
  },
  selectedTags: {
    type: Array as () => string[],
    default: () => []
  }
})

// 组件事件
const emit = defineEmits([
  'search', 
  'add-notebook', 
  'add-tag', 
  'select-notebook',
  'toggle-tag',
  'new-note',
  'ai-assistant',
  'settings',
  'toggle-collapse',
  'edit-notebook',
  'delete-notebook',
  'add-child-notebook',
  'delete-tag',
  'clipboard',
  'import',
  'encrypt-notebook',
  'decrypt-notebook'
])

// 状态
const isCollapsed = ref(false)
const searchQuery = ref('')
const showTagsPopup = ref(false)
const showNotebooksPopup = ref(false)

// 标签状态管理
const tagsCollapsed = ref(false)
const showAllTags = ref(false)
const tagSearchQuery = ref('')
const maxVisibleTags = 10

// 根据搜索过滤标签
const filteredTags = computed(() => {
  if (!tagSearchQuery.value) return props.tags
  const query = tagSearchQuery.value.toLowerCase().trim()
  return props.tags.filter(tag => 
    tag.name.toLowerCase().includes(query)
  )
})

// 获取可见标签 (常用标签或选中的标签)
const visibleTags = computed(() => {
  // 优先显示已选中的标签
  const selected = props.tags.filter(tag => props.selectedTags.includes(tag.id))
  
  // 如果已选标签数量少于最大显示数量，则补充常用标签
  if (selected.length < maxVisibleTags) {
    // 获取未选中的标签
    const unselected = props.tags.filter(tag => !props.selectedTags.includes(tag.id))
    
    // 按名称字母顺序排序（因为可能没有count属性）
    const sortedUnselected = [...unselected].sort((a, b) => a.name.localeCompare(b.name))
    
    // 补充到最大显示数量
    return [...selected, ...sortedUnselected.slice(0, maxVisibleTags - selected.length)]
  }
  
  // 如果选中标签超过最大显示数量，则只显示选中的标签
  return selected.slice(0, maxVisibleTags)
})

// 切换标签折叠状态
function toggleTagsCollapsed() {
  tagsCollapsed.value = !tagsCollapsed.value
  if (tagsCollapsed.value) {
    showAllTags.value = false
  }
}

// 关闭弹出面板的点击事件处理
const handleDocumentClick = (event: MouseEvent) => {
  // 检查点击事件目标是否在弹出窗口按钮内，如果是则不关闭弹出窗口
  const target = event.target as HTMLElement;
  
  // 查找最近的按钮祖先元素
  const closestButton = target.closest('button');
  
  // 如果点击的是触发弹出窗口的按钮或弹出窗口内的元素，不执行关闭操作
  if (closestButton && closestButton.closest('[data-tip]')) {
    // 允许通过工具提示按钮的点击事件
    return;
  }
  
  // 如果点击的是弹出窗口内部，不执行关闭操作
  if (target.closest('.fixed.top-0.left-16')) {
    return;
  }
  
  // 否则关闭所有弹出窗口
  if (showTagsPopup.value || showNotebooksPopup.value) {
    showTagsPopup.value = false;
    showNotebooksPopup.value = false;
  }
}

// 初始化
onMounted(() => {
  // 从UI存储中获取初始折叠状态
  isCollapsed.value = uiStore.settings.isSidebarCollapsed;
  
  // 确保HTML属性同步
  document.documentElement.setAttribute('data-sidebar-collapsed', isCollapsed.value ? 'true' : 'false');
  
  // 添加全局点击事件监听器来关闭弹出窗口
  document.addEventListener('click', handleDocumentClick);
})

// 组件被keep-alive缓存后重新激活时触发
onActivated(() => {
  // 只同步UI状态，不重新加载数据
  if (isCollapsed.value !== uiStore.settings.isSidebarCollapsed) {
    isCollapsed.value = uiStore.settings.isSidebarCollapsed
    document.documentElement.setAttribute('data-sidebar-collapsed', isCollapsed.value ? 'true' : 'false')
  }
})

// 监听UI存储折叠状态变化
watch(() => uiStore.settings.isSidebarCollapsed, (newValue) => {
  if (isCollapsed.value !== newValue) {
    console.log('SideNavBar从UI存储更新折叠状态:', newValue);
    isCollapsed.value = newValue;
  }
});

// 组件销毁前清理
onBeforeUnmount(() => {
  document.removeEventListener('click', handleDocumentClick)
})

// 方法
function toggleCollapse(event?: Event) {
  // 阻止事件冒泡，避免与document点击事件冲突
  if (event) {
    event.stopPropagation();
    event.preventDefault();
  }
  
  // 切换状态
  isCollapsed.value = !isCollapsed.value;
  console.log('侧边栏折叠状态已切换:', isCollapsed.value);
  
  // 更新UI存储
  uiStore.setSidebarCollapsed(isCollapsed.value);
  
  // 手动设置HTML属性，确保立即生效
  document.documentElement.setAttribute('data-sidebar-collapsed', isCollapsed.value ? 'true' : 'false');
  
  // 通知父组件
  emit('toggle-collapse', isCollapsed.value);
  
  // 关闭所有弹出窗口
  showTagsPopup.value = false;
  showNotebooksPopup.value = false;
}

function selectNotebook(id: string) {
  console.log('SideNavBar: 选择笔记本:', id)
  emit('select-notebook', id)
  closeNotebooksPopup()
}

function toggleTag(id: string) {
  // 只切换标签的状态，不再创建重复的标签
  emit('toggle-tag', id)
  
  // 关闭标签弹窗（如果正在显示）
  if (showTagsPopup.value) {
    closeTagsPopup()
  }
}



function toggleNotebooksPopup(event?: Event) {
  if (event) {
    event.stopPropagation()
    event.preventDefault()
  }
  showNotebooksPopup.value = !showNotebooksPopup.value
  showTagsPopup.value = false
}

function selectNotebookAndClose(id: string) {
  emit('select-notebook', id)
  showNotebooksPopup.value = false
}

function addNotebookAndClose() {
  emit('add-notebook')
  showNotebooksPopup.value = false
}

function closeNotebooksPopup() {
  showNotebooksPopup.value = false
}

function closeTagsPopup() {
  showTagsPopup.value = false
}


function addChildNotebook(parentId: string) {
  console.log('SideNavBar: 添加子笔记本，父ID:', parentId)
  emit('add-child-notebook', parentId)
}

function deleteNotebook(id: string) {
  emit('delete-notebook', id)
}

// 添加到script部分的状态定义中
const showTagSearchInput = ref(false)
const tagSearchInputRef = ref<HTMLInputElement | null>(null)

// 添加到script部分的方法定义中
function toggleTagSearchInput() {
  showTagSearchInput.value = !showTagSearchInput.value
  // 当显示搜索框时，自动聚焦
  if (showTagSearchInput.value) {
    nextTick(() => {
      if (tagSearchInputRef.value) {
        tagSearchInputRef.value.focus()
      }
    })
  } else {
    // 如果隐藏搜索框，清空搜索内容
    tagSearchQuery.value = ''
  }
}

function hideTagSearchInputIfEmpty() {
  // 如果搜索框为空，则在失去焦点时隐藏
  if (!tagSearchQuery.value.trim()) {
    showTagSearchInput.value = false
  }
}



// 修改监听搜索查询变化的逻辑
watch(tagSearchQuery, (newValue) => {
  // 当有搜索内容就显示结果
  if (newValue.trim()) {
    // 如果结果少于5个，直接显示全部
    if (filteredTags.value.length <= 5) {
      showAllTags.value = true
    } else {
      // 否则显示提示按钮
      showAllTags.value = false
    }
  } else {
    // 搜索框为空时重置显示
    showAllTags.value = false
  }
}, { immediate: true })

// 添加删除标签的函数
async function deleteTag(tagId: string) {
  // 确认对话框
  const confirmed = await showConfirm('确定要删除此标签吗？这将从所有笔记中移除该标签。', {
    title: '确认删除标签',
    confirmText: '删除',
    cancelText: '取消'
  })
  
  if (confirmed) {
    // 直接发送删除事件到父组件，无论标签是否被选中
    emit('delete-tag', tagId)
    
    // 如果该标签恰好在已选标签中，从选中列表中移除它
    // 这是一个UI状态更新，实际删除操作由父组件处理
    if (props.selectedTags.includes(tagId)) {
      toggleTag(tagId)
    }
  }
}

// 新增：高亮搜索关键字
const highlightKeyword = (text: string, keyword: string) => {
  if (!keyword) return text
  const reg = new RegExp(`(${keyword.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(reg, '<span class="text-primary font-bold bg-yellow-100">$1</span>')
}
</script>

<style scoped>
/* SideNavBar特有的样式 */

/* 侧边栏折叠动画 */
.sidebar-transition {
  transition: width 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
}

/* 导航项的特殊动画效果 */
.nav-item {
  transition: all 0.2s ease;
  position: relative;
}

.nav-item:hover {
  transform: translateX(2px);
}

.nav-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--primary);
  transform: scaleY(0);
  transition: transform 0.2s ease;
}

.nav-item.active::before {
  transform: scaleY(1);
}

/* 折叠状态下的tooltip样式 */
.tooltip-collapsed {
  position: absolute;
  left: 110%;
  top: 50%;
  transform: translateY(-50%);
  z-index: 1000;
  white-space: nowrap;
  padding: 6px 12px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  border-radius: 4px;
  font-size: 12px;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s ease;
}

.nav-item:hover .tooltip-collapsed {
  opacity: 1;
}

/* 徽章计数的特殊样式 */
.nav-badge {
  position: absolute;
  top: -2px;
  right: -2px;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  font-size: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
}

/* 折叠状态特殊样式 */
.collapsed .nav-text {
  opacity: 0;
  transform: translateX(-10px);
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.expanded .nav-text {
  opacity: 1;
  transform: translateX(0);
  transition: opacity 0.3s ease 0.1s, transform 0.3s ease 0.1s;
}
</style> 
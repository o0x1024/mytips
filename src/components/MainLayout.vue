<template>
  <div class="h-screen flex flex-col">
    <!-- 主体内容区 -->
    <main class="flex-1 flex overflow-hidden">
      <!-- 左侧导航栏 -->
      <SideNavBar 
        :notebooks="navFilteredNotebooks"
        :tags="tags"
        :search-query="navSearchQuery"
        :selected-notebook-id="selectedNotebookId"
        :selected-tags="selectedTags"
        :selected-note-id="selectedNoteId"
        :notes="navFilteredNotes"
        :is-collapsed="sidebarCollapsed"
        :is-focus-mode="isFocusMode"
        :focus-section="focusSection"
        @select-notebook="selectNotebook"
        @toggle-tag="toggleTag"
        @select-note="selectNote"
        @add-notebook="showAddNotebookModal = true"
        @import="openImportDialog"
        @add-child-notebook="addChildNotebook"
        @edit-notebook="editNotebook"
        @delete-notebook="deleteNotebook"
        @delete-tag="deleteTag"
        @search="(query) => navSearchQuery = query"
        @new-note="createNewNote"
        @clipboard="() => navigateTo('/clipboard')"
        @ai-assistant="() => navigateTo('/ai-assistant')"
        @settings="() => navigateTo('/settings')"
      />

      <!-- 中间笔记列表 -->
      <div 
        v-if="!isFocusMode || focusSection === 'list'" 
        class="h-full transition-all duration-300" 
        :class="{
          'list-width': !isFocusMode && !sidebarCollapsed,
          'list-width expanded-list': !isFocusMode && sidebarCollapsed,
          'w-full': isFocusMode && focusSection === 'list'
        }"
      >
          <NoteList 
            :notes="navFilteredNotes"
            :title="listTitle"
            :loading="isLoading"
            :selected-note-id="selectedNoteId"
            :notebooks="notebooks"
            :selected-notebook-id="selectedNotebookId"
            @select-note="selectNote"
            @search="handleListSearch"
            @new-note="createNewNote"
            @delete-note="deleteNote"
            @export-note="exportNote"
            @move-to-category="moveNoteToCategory"
            @refresh="refreshNotes"
            :key="selectedNotebookId"
          />
      </div>

      <!-- 右侧编辑区域 -->
      <div 
        v-if="!isFocusMode || focusSection === 'editor'" 
        class="h-full flex-1"
      >
        <div v-if="selectedNote" class="h-full">
          <NoteEditor 
            :note="selectedNote"
            @update="updateNote"
            @delete-note="deleteNote"
            @duplicate-note="duplicateNote"
            @add-tag="addTagToNote"
            @remove-tag="removeTagFromNote"
            @toggle-pin="toggleNotePin"
          />
        </div>
        <div v-else class="h-full flex items-center justify-center flex-col p-6 bg-base-200 text-base-content/80">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-24 w-24 mb-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
          <h2 class="text-2xl font-bold mb-2">选择或创建笔记</h2>
          <p class="mb-4 text-center max-w-md">
            在左侧选择一个笔记本，然后从列表中选择一个笔记进行编辑，或者创建一个新笔记开始记录你的想法。
          </p>
          <!-- <button class="btn btn-primary" @click="createNewNote">创建新笔记</button> -->
        </div>
      </div>
    </main>

    <!-- 添加笔记本模态框 -->
    <div class="modal" :class="{'modal-open': showAddNotebookModal}">
      <div class="modal-box">
        <h3 class="font-bold text-lg">创建新笔记本</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">笔记本名称</span>
          </label>
          <input type="text" placeholder="输入名称" class="input input-bordered w-full" v-model="newNotebookName" />
          
          <label class="label mt-2">
            <span class="label-text">父笔记本 (可选)</span>
          </label>
          <select class="select select-bordered w-full" v-model="newNotebookParentId">
            <option value="">无 (顶级笔记本)</option>
            <option v-for="notebook in flatNotebooks" :key="notebook.id" :value="notebook.id">
              {{ notebook.name }}
            </option>
          </select>
        </div>
        <div class="modal-action">
          <button class="btn" @click="showAddNotebookModal = false">取消</button>
          <button class="btn btn-primary" @click="addNotebook" :disabled="!newNotebookName.trim()">创建</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showAddNotebookModal = false"></div>
    </div>

    <!-- 添加标签模态框 -->
    <div class="modal" :class="{'modal-open': showAddTagModal}">
      <div class="modal-box">
        <h3 class="font-bold text-lg">创建新标签</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">标签名称</span>
          </label>
          <input type="text" placeholder="输入标签" class="input input-bordered w-full" v-model="newTagName" />
        </div>
        <div class="modal-action">
          <button class="btn" @click="showAddTagModal = false">取消</button>
          <button class="btn btn-primary" @click="addTag" :disabled="!newTagName.trim()">创建</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showAddTagModal = false"></div>
    </div>

    <!-- 删除笔记本确认 Modal -->
    <div class="modal" :class="{ 'modal-open': showDeleteModal }">
      <div class="modal-box">
        <h3 class="font-bold text-lg text-error">确定要删除笔记本？</h3>
        <p class="py-4">
          <span v-if="pendingDeleteNotebook">
            确定要删除笔记本 <b class="text-error">{{ pendingDeleteNotebook.name }}</b> 吗？<br>
            删除后不可恢复，且会删除其下所有子笔记本和笔记。
          </span>
        </p>
        <div class="modal-action">
          <button class="btn" @click="cancelDeleteNotebook">取消</button>
          <button class="btn btn-error" @click="confirmDeleteNotebook">确定删除</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="cancelDeleteNotebook"></div>
    </div>

    <!-- 编辑笔记本 Modal -->
    <div class="modal" :class="{ 'modal-open': showEditNotebookModal }">
      <div class="modal-box">
        <h3 class="font-bold text-lg">编辑笔记本</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">新名称</span>
          </label>
          <input type="text" class="input input-bordered w-full" v-model="editNotebookName" />
        </div>
        <div class="modal-action">
          <button class="btn" @click="cancelEditNotebook">取消</button>
          <button class="btn btn-primary" @click="confirmEditNotebook" :disabled="!editNotebookName.trim()">确定</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="cancelEditNotebook"></div>
    </div>

    <!-- Import Dialog -->
    <ImportDialog 
      :show="showImportDialog"
      :notebooks="flatNotebooks"
      @close="showImportDialog = false"
      @success="handleImportSuccess"
    />

    <!-- Markdown Drop Preview -->
    <MarkdownDropPreview 
      :notebooks="flatNotebooks"
      @import="handleMarkdownImport"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, onActivated, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import SideNavBar from './SideNavBar.vue'
import NoteList from './NoteList.vue'
import NoteEditor from './NoteEditor.vue'
import ImportDialog from './ImportDialog.vue'
import MarkdownDropPreview from './MarkdownDropPreview.vue'
import { useTipsStore, Tag, Tip, Category } from '../stores/tipsStore'
import { useUIStore } from '../stores/uiStore'
import { invoke } from '@tauri-apps/api/core'

// Store
const tipsStore = useTipsStore()
const uiStore = useUIStore()

// Router
const router = useRouter()

// 类型定义
interface Notebook {
  id: string;
  name: string;
  count?: number;
  children: Notebook[];
  parent_id?: string;
}

// 扩展Tip类型定义
interface Note extends Tip {
  isPinned?: boolean;
  isFavorite?: boolean;
  _fromBlur?: boolean;
  _contentOnly?: boolean;
  images?: Record<string, string>;
}

// 状态
const notebooks = ref<Notebook[]>([])
const tags = ref<Tag[]>([])
const notes = ref<Note[]>([])
const selectedNotebookId = ref<string | undefined>(undefined)
const selectedTags = ref<string[]>([])
const selectedNoteId = ref<string | undefined>(undefined)
const isLoading = ref(false)
const listSearchQuery = ref('')
const navSearchQuery = ref('')
const isFocusMode = ref(false)
const focusSection = ref<'nav' | 'list' | 'editor'>('editor')
const sidebarCollapsed = ref(false)

// 模态框状态
const showAddNotebookModal = ref(false)
const showAddTagModal = ref(false)
const showImportDialog = ref(false)
const newNotebookName = ref('')
const newNotebookParentId = ref('')
const newTagName = ref('')
const showDeleteModal = ref(false)
const pendingDeleteNotebook = ref<Notebook | null>(null)

// 编辑笔记本弹窗状态
const showEditNotebookModal = ref(false)
const editNotebookName = ref('')
const pendingEditNotebook = ref<Notebook | null>(null)


const selectedNote = computed(() => {
  if (!selectedNoteId.value) return null
  return notes.value.find(note => note.id === selectedNoteId.value) || null
})

const listTitle = computed(() => {
  if (selectedNotebookId.value) {
    const notebook = findNotebookById(notebooks.value, selectedNotebookId.value)
    return notebook ? notebook.name : '笔记'
  } else if (selectedTags.value.length > 0) {
    return `标签: ${selectedTags.value.map(id => {
      const tag = tags.value.find(t => t.id === id)
      return tag ? tag.name : ''
    }).filter(Boolean).join(', ')}`
  } else if (listSearchQuery.value) {
    return `搜索: ${listSearchQuery.value}`
  }
  return '所有笔记'
})

const flatNotebooks = computed(() => {
  // 将嵌套的笔记本结构扁平化以用于下拉选择
  const result: Notebook[] = []
  
  function flatten(items: Notebook[], prefix = '') {
    for (const item of items) {
      const flatItem = { ...item, name: prefix + item.name }
      result.push(flatItem)
      
      if (item.children && item.children.length > 0) {
        flatten(item.children, flatItem.name + ' > ')
      }
    }
  }
  
  flatten(notebooks.value)
  return result
})

// 新增：侧边栏搜索过滤结果
const navFilteredNotebooks = computed(() => {
  if (!navSearchQuery.value.trim()) return notebooks.value
  const query = navSearchQuery.value.trim().toLowerCase()
  // 递归过滤树
  function filterTree(items: Notebook[]): Notebook[] {
    return items
      .map(item => {
        const matched = item.name.toLowerCase().includes(query)
        const filteredChildren = item.children ? filterTree(item.children) : []
        if (matched || filteredChildren.length > 0) {
          return {
            ...item,
            children: filteredChildren
          }
        }
        return null
      })
      .filter(Boolean) as Notebook[]
  }
  return filterTree(notebooks.value)
})

const navFilteredNotes = computed(() => {
  let result = [...notes.value]
  
  // 首先处理导航搜索
  if (navSearchQuery.value.trim()) {
    const query = navSearchQuery.value.trim().toLowerCase()
    result = result.filter(note =>
      note.title.toLowerCase().includes(query) ||
      note.content.toLowerCase().includes(query)
    )
  }
  
  // 处理笔记本筛选
  if (selectedNotebookId.value) {
    const notebookIds = collectNotebookIds(notebooks.value, selectedNotebookId.value)
    result = result.filter(note => notebookIds.includes(String(note.category_id)))
  }
  
  // 处理标签筛选
  if (selectedTags.value.length > 0) {
    result = result.filter(note => {
      return (note.tags || []).some(tag => selectedTags.value.includes(tag.id))
    })
  }
  
  // 处理列表搜索
  if (listSearchQuery.value.trim()) {
    const query = listSearchQuery.value.trim().toLowerCase()
    result = result.filter(note =>
      note.title.toLowerCase().includes(query) ||
      note.content.toLowerCase().includes(query) ||
      (note.tags || []).some(tag => tag.name.toLowerCase().includes(query))
    )
  }
  
  return result
})

// 方法
function findNotebookById(notebooks: Notebook[], id: string): Notebook | null {
  for (const notebook of notebooks) {
    if (notebook.id === id) return notebook
    if (notebook.children && notebook.children.length > 0) {
      const found = findNotebookById(notebook.children, id)
      if (found) return found
    }
  }
  return null
}

function navigateTo(path: string) {
  router.push(path)
}

function handleListSearch(query: string) {
  listSearchQuery.value = query
  // 列表搜索时清除当前选中的笔记
  selectedNoteId.value = undefined
}

function selectNotebook(id: string) {
  selectedNotebookId.value = id
  // 清除其他筛选条件
  selectedTags.value = []
  listSearchQuery.value = ''
  
  // 获取当前笔记本下的所有笔记
  const notebookIds = collectNotebookIds(notebooks.value, id)
  const notesInNotebook = notes.value.filter(note => notebookIds.includes(String(note.category_id)))
  
  // 如果有笔记，自动选择第一个
  if (notesInNotebook.length > 0) {
    // 优先选择固定的笔记
    const pinnedNotes = notesInNotebook.filter(note => note.isPinned)
    if (pinnedNotes.length > 0) {
      selectedNoteId.value = pinnedNotes[0].id
    } else {
      // 否则按照更新时间排序，选择最近更新的
      const sortedNotes = [...notesInNotebook].sort((a, b) => (b.updated_at || 0) - (a.updated_at || 0))
      selectedNoteId.value = sortedNotes[0].id
    }
  } else {
    // 如果没有笔记，清除当前选择
    selectedNoteId.value = undefined
  }
  
  // 如果在专注模式下，切换到列表区域
  if (isFocusMode.value) {
    focusSection.value = 'list'
  }
}

function toggleTag(id: string) {
  const index = selectedTags.value.indexOf(id)
  if (index >= 0) {
    // 如果标签已经选中，则移除它
    selectedTags.value.splice(index, 1)
  } else {
    // 如果标签未选中，则添加它（不重复）
    selectedTags.value.push(id)
  }
  
  // 如果有标签选择，则清除笔记本选择和搜索条件
  if (selectedTags.value.length > 0) {
    selectedNotebookId.value = undefined
    listSearchQuery.value = ''
    
    // 获取包含所选标签的所有笔记
    const notesWithTags = notes.value.filter(note => {
      return (note.tags || []).some(tag => selectedTags.value.includes(tag.id))
    })
    
    // 如果有符合条件的笔记，自动选择第一个
    if (notesWithTags.length > 0) {
      // 优先选择固定的笔记
      const pinnedNotes = notesWithTags.filter(note => note.isPinned)
      if (pinnedNotes.length > 0) {
        selectedNoteId.value = pinnedNotes[0].id
      } else {
        // 否则按照更新时间排序，选择最近更新的
        const sortedNotes = [...notesWithTags].sort((a, b) => (b.updated_at || 0) - (a.updated_at || 0))
        selectedNoteId.value = sortedNotes[0].id
      }
    } else {
      // 如果没有符合条件的笔记，清除当前选择
      selectedNoteId.value = undefined
    }
  } else {
    // 如果没有选中任何标签，清除笔记选择
    selectedNoteId.value = undefined
  }
  
  // 如果在专注模式下，切换到列表区域
  if (isFocusMode.value) {
    focusSection.value = 'list'
  }
  
  // 重新加载标签列表以更新状态
  loadTags();
}

function selectNote(id: string) {
  selectedNoteId.value = id
  
  // 如果在专注模式下，切换到编辑器区域
  if (isFocusMode.value) {
    focusSection.value = 'editor'
  }
}

// 判断是否为未分类笔记本
function isUncategorizedNotebook(notebook: Notebook) {
  return notebook.name === '未分类'
}

async function createNewNote() {
  isLoading.value = true
  try {
    let categoryId: string | undefined;
    if (selectedNotebookId.value) {
      categoryId = selectedNotebookId.value;
    } else {
      // 查找未分类笔记本
      let uncategorized = notebooks.value.find(n => n.name === "未分类");
      if (!uncategorized) {
        // 不存在则自动创建
        const newCategory = await tipsStore.createCategory("未分类");
        if (newCategory) {
          uncategorized = {
            id: newCategory.id,
            name: newCategory.name,
            children: [],
            count: 0
          };
          notebooks.value.push(uncategorized);
        }
      }
      if (uncategorized) {
        categoryId = uncategorized.id;
      }
    }
    
    // 创建新笔记，放入确定的笔记本中
    const newNoteData = {
      title: '无标题笔记',
      content: '',
      tip_type: 'markdown',
      category_id: categoryId, // 使用确定的笔记本ID
      tags: []
    }
    
    const savedNote = await tipsStore.saveTip(newNoteData)
    if (savedNote) {
      // 重新拉取所有笔记，避免重复
      await tipsStore.fetchAllTips()
      notes.value = tipsStore.tips
      selectedNoteId.value = savedNote.id
      // 更新笔记本计数
      if (categoryId) {
        updateNotebookCount(categoryId, 1)
      }
    }
  } catch (error) {
    console.error('创建笔记失败:', error)
  } finally {
    isLoading.value = false
  }
  
  // 如果在专注模式下，切换到编辑器区域
  if (isFocusMode.value) {
    focusSection.value = 'editor'
  }
}

async function updateNote(updatedNote: Note & { _titleOnly?: boolean, _contentOnly?: boolean, _fromBlur?: boolean, _autoSave?: boolean, _pageSwitch?: boolean }) {
  // 只更新标题，不刷新/重排列表
  if (updatedNote._titleOnly) {
    const idx = notes.value.findIndex(n => n.id === updatedNote.id)
    if (idx !== -1) {
      notes.value[idx].title = updatedNote.title
      notes.value[idx].updated_at = Date.now()
    }
    return
  }

  isLoading.value = true
  try {
    // 提取标记
    const isContentOnlyUpdate = updatedNote._contentOnly === true
    const isAutoSave = updatedNote._autoSave === true
    const isPageSwitch = updatedNote._pageSwitch === true
    
    // 清除自定义标记，避免发送到后端
    const cleanedNote = { ...updatedNote }
    delete cleanedNote._contentOnly
    delete cleanedNote._fromBlur
    delete cleanedNote._titleOnly
    delete cleanedNote._autoSave
    delete cleanedNote._pageSwitch
    
    // 提取图片数据以单独处理
    const imageData = cleanedNote.images ? {...cleanedNote.images} : undefined
    const tipData = {
      id: cleanedNote.id,
      title: cleanedNote.title,
      content: cleanedNote.content,
      tip_type: cleanedNote.tip_type || 'markdown',
      language: cleanedNote.language,
      category_id: cleanedNote.category_id,
      tags: Array.isArray(cleanedNote.tags) ? cleanedNote.tags.map(tag => tag.id) : []
    }
    
    // 情况1：自动保存 - 仅内容更新，不触发列表重排序
    if (isAutoSave || isContentOnlyUpdate) {
      // 使用本地数据更新界面，而不是立即发送到后端
      const index = notes.value.findIndex(note => note.id === cleanedNote.id)
      if (index >= 0) {
        // 仅更新内容、更新时间和图片数据
        notes.value[index].content = cleanedNote.content
        notes.value[index].updated_at = cleanedNote.updated_at || Date.now()
        // 更新图片数据
        if (imageData) {
          if (!notes.value[index].images) {
            notes.value[index].images = {}
          }
          Object.assign(notes.value[index].images, imageData)
        }
      }
      isLoading.value = false
      return
    }
    
    // 情况2：页面切换时的完整保存 - 保存到后端并更新列表
    const savedNote = await tipsStore.saveTip(tipData)
    if (savedNote) {
      // 查找现有笔记
      const index = notes.value.findIndex(note => note.id === savedNote.id)
      
      if (index >= 0) {
        // 保存原始图片数据
        const originalImages = notes.value[index].images
        
        // 更新现有笔记，保留原来的引用
        Object.assign(notes.value[index], savedNote)
        
        // 保留图片数据，因为后端API可能不会返回图片数据
        if (originalImages || imageData) {
          if (!notes.value[index].images) {
            notes.value[index].images = {}
          }
          
          // 合并原有图片和新图片
          if (originalImages) {
            Object.assign(notes.value[index].images, originalImages)
          }
          
          if (imageData) {
            Object.assign(notes.value[index].images, imageData)
          }
        }
        
        // 页面切换时重新排序列表
        if (isPageSwitch) {
          sortNotes()
        }
      } else {
        // 如果找不到，可能是新笔记，添加到列表开头
        if (imageData) {
          savedNote.images = imageData 
        }
        notes.value.unshift(savedNote)
      }
      
      // 只在有重复时才去重并新建数组
      const seen: Record<string, boolean> = {}
      let hasDuplicate = false
      for (const note of notes.value) {
        if (seen[note.id]) {
          hasDuplicate = true
          break
        }
        seen[note.id] = true
      }
      if (hasDuplicate) {
        const uniqueNotes: Record<string, number> = {}
        notes.value = notes.value.filter((note, idx) => {
          if (uniqueNotes[note.id] !== undefined) {
            return false // 丢弃重复的
          }
          uniqueNotes[note.id] = idx
          return true
        })
      }
    }
  } catch (error) {
    console.error('更新笔记失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 添加一个排序笔记的函数
function sortNotes() {
  // 先将固定的笔记提取出来
  const pinnedNotes = notes.value.filter(note => note.isPinned)
  const unpinnedNotes = notes.value.filter(note => !note.isPinned)
  
  // 对未固定的笔记进行排序
  // 这里可以根据实际的排序规则进行调整
  unpinnedNotes.sort((a, b) => {
    // 默认按更新时间排序
    return (b.updated_at || 0) - (a.updated_at || 0)
  })
  
  // 合并固定和未固定的笔记
  notes.value = [...pinnedNotes, ...unpinnedNotes]
}

async function deleteNote() {
  if (!selectedNoteId.value) return

  isLoading.value = true
  try {
    const noteToDelete = notes.value.find(note => note.id === selectedNoteId.value)
    const success = await tipsStore.deleteTip(selectedNoteId.value)

    if (success) {
      // 找到当前被删除的索引
      const idx = notes.value.findIndex(note => note.id === selectedNoteId.value)
      // 只移除该项
      notes.value.splice(idx, 1)
      // 更新计数
      if (noteToDelete && noteToDelete.category_id) {
        updateNotebookCount(noteToDelete.category_id, -1)
      }
      // 选中下一个或上一个
      if (notes.value.length > 0) {
        if (idx < notes.value.length) {
          selectedNoteId.value = notes.value[idx].id
        } else {
          selectedNoteId.value = notes.value[notes.value.length - 1].id
        }
      } else {
        selectedNoteId.value = undefined
      }
    }
  } catch (error) {
    console.error('删除笔记失败:', error)
  } finally {
    isLoading.value = false
  }
}

async function duplicateNote() {
  if (!selectedNote.value) return
  
  isLoading.value = true
  try {
    const originalNote = selectedNote.value
    const duplicateData = {
      title: `${originalNote.title} (副本)`,
      content: originalNote.content,
      tip_type: originalNote.tip_type || 'markdown',
      language: originalNote.language,
      category_id: originalNote.category_id,
      tags: originalNote.tags.map(tag => tag.id)
    }
    
    const savedNote = await tipsStore.saveTip(duplicateData)
    if (savedNote) {
      notes.value.unshift(savedNote)
      selectedNoteId.value = savedNote.id
      
      // 更新笔记本计数
      if (savedNote.category_id) {
        updateNotebookCount(savedNote.category_id, 1)
      }
    }
  } catch (error) {
    console.error('复制笔记失败:', error)
  } finally {
    isLoading.value = false
  }
}

async function addNotebook() {
  if (!newNotebookName.value.trim()) return
  isLoading.value = true
  try {
    console.log('创建笔记本:222', newNotebookName.value, newNotebookParentId.value)
    // 创建新笔记本，传递parent_id
    const newCategory = await tipsStore.createCategory(
      newNotebookName.value.trim(),
      newNotebookParentId.value || undefined
    )
    console.log('创建笔记本:444', newCategory)
    if (newCategory) {
      // 刷新分类和笔记
      await tipsStore.fetchAllCategories()
      await tipsStore.fetchAllTips()
      notebooks.value = categoriesToNotebooks(tipsStore.categories)
      notes.value = tipsStore.tips
      // 关闭模态框、重置表单
      newNotebookName.value = ''
      newNotebookParentId.value = ''
      showAddNotebookModal.value = false
      // 选中新建的笔记本
      selectedNotebookId.value = newCategory.id
    }
  } catch (error) {
    console.error('创建笔记本失败:', error)
  } finally {
    isLoading.value = false
  }
}

async function addTag() {
  if (!newTagName.value.trim()) return
  
  isLoading.value = true
  try {
    const newTag = await tipsStore.createTag(newTagName.value.trim())
    
    if (newTag) {
      // 检查是否已经存在相同ID的标签，避免重复
      const existingTagIndex = tags.value.findIndex(t => t.id === newTag.id)
      if (existingTagIndex >= 0) {
        // 如果已存在，更新它
        tags.value[existingTagIndex] = newTag
      } else {
        // 否则添加新标签
        tags.value.push(newTag)
      }
      
      // 重置表单并关闭模态框
      newTagName.value = ''
      showAddTagModal.value = false
      
      // 选择新创建的标签
      selectedTags.value = [newTag.id]
      // 清除其他筛选条件
      selectedNotebookId.value = undefined
    }
  } catch (error) {
    console.error('创建标签失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 更新笔记本计数
function updateNotebookCount(categoryId: string, change: number) {
  const updateCount = (items: Notebook[]): boolean => {
    for (const item of items) {
      if (item.id === categoryId) {
        if (item.count !== undefined) {
          item.count += change
        } else {
          item.count = Math.max(0, change)
        }
        return true
      }
      
      if (item.children && item.children.length > 0) {
        if (updateCount(item.children)) return true
      }
    }
    return false
  }
  
  updateCount(notebooks.value)
}

// 将分类数据转换为笔记本结构
function categoriesToNotebooks(categories: Category[]): Notebook[] {
  if (!categories || categories.length === 0) {
    console.warn('没有找到任何分类数据')
    return []
  }

  // 第一步：建立分类到笔记本的映射
  const notebooksMap: Record<string, Notebook> = {}

  // 先创建所有笔记本对象
  categories.forEach(category => {
    notebooksMap[category.id] = {
      id: category.id,
      name: category.name,
      children: [],
      count: 0, // 稍后计算
      parent_id: category.parent_id // 直接用后端返回的parent_id
    }
  })

  // 第二步：构建树形结构
  const rootNotebooks: Notebook[] = []

  // 遍历所有笔记本，建立父子关系
  Object.values(notebooksMap).forEach(notebook => {
    if (notebook.parent_id && notebooksMap[notebook.parent_id]) {
      // 将当前笔记本添加为父笔记本的子项
      notebooksMap[notebook.parent_id].children.push(notebook)
    } else {
      // 没有父笔记本或父笔记本不存在，作为根笔记本
      rootNotebooks.push(notebook)
    }
  })

  return rootNotebooks
}

// 钩子
onMounted(async () => {
  // 初始化数据
  await loadData()
  
  // 加载上次编辑的笔记
  loadLastEditedNote()
})

// 组件被keep-alive缓存后重新激活时触发
onActivated(() => {
  console.log('MainLayout被重新激活')
  
  // 不重新加载数据，保持当前状态
  // 只有在需要定位到笔记时才执行相关操作
  if (selectedNoteId.value) {
    // 确保选中的笔记可见，但不触发数据重载
    nextTick(() => {
      const element = document.querySelector(`[data-note-id="${selectedNoteId.value}"]`)
      if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
      }
    })
  }
})

// 保存上次编辑的笔记ID
watch(selectedNoteId, (newId) => {
  if (newId) {
    localStorage.setItem('lastEditedNoteId', newId)
  }
})

// 加载上次编辑的笔记
async function loadLastEditedNote() {
  try {
    const lastNoteId = localStorage.getItem('lastEditedNoteId')
    if (lastNoteId) {
      // 检查笔记是否存在
      const noteExists = notes.value.find(note => note.id === lastNoteId)
      if (noteExists) {
        selectedNoteId.value = lastNoteId
        // 如果笔记属于某个笔记本，选中该笔记本
        if (noteExists.category_id) {
          selectedNotebookId.value = noteExists.category_id
        }
      }
    }
  } catch (error) {
    console.error('加载上次编辑的笔记失败:', error)
  }
}

// 导出笔记
async function exportNote(noteId: string, format: string) {
  try {
    isLoading.value = true
    
    // 获取要导出的笔记
    const note = notes.value.find(n => n.id === noteId)
    if (!note) {
      throw new Error('笔记未找到')
    }
    
    // 根据不同格式选择不同的导出方法
    let result;
    
    if (format === 'markdown') {
      // 导出为Markdown
      result = await invoke('export_as_markdown', { noteIds: [noteId] })
    } else if (format === 'html') {
      // 导出为HTML
      result = await invoke('export_as_html', { noteIds: [noteId] })
    } else if (format === 'pdf') {
      // 导出为PDF
      result = await invoke('export_as_pdf', { noteIds: [noteId] })
    } else {
      throw new Error(`不支持的导出格式: ${format}`)
    }
    
    console.log('导出结果:', result)
    
    // 显示成功提示
    if (typeof result === 'string') {
      // 使用 alert 弹窗提示用户导出成功
      alert(`导出成功: ${result}`)
    }
  } catch (error) {
    console.error('导出笔记失败:', error)
    alert(`导出失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

// 监听UI存储中侧边栏折叠状态的变化
watch(() => uiStore.settings.isSidebarCollapsed, (newValue) => {
  if (typeof newValue === 'boolean') {
    sidebarCollapsed.value = newValue
  }
})

// 监听组件内部侧边栏折叠状态的变化
watch(sidebarCollapsed, (newValue) => {
  console.log('MainLayout侧边栏折叠状态变化:', newValue)
  
  // 防止循环更新
  if (uiStore.settings.isSidebarCollapsed !== newValue) {
    uiStore.setSidebarCollapsed(newValue)
    
    // 直接设置HTML属性，确保立即生效
    document.documentElement.setAttribute('data-sidebar-collapsed', newValue ? 'true' : 'false')
  }
})

// 移动笔记到分类
async function moveNoteToCategory(noteId: string, newCategoryId: string) {
  isLoading.value = true
  try {
    // 查找笔记
    const note = notes.value.find(n => n.id === noteId)
    if (!note) {
      throw new Error('笔记未找到')
    }
    
    // 准备更新数据
    const updatedNote = { ...note }
    
    // 如果是从一个分类移动到另一个分类，更新计数
    if (updatedNote.category_id) {
      // 原分类减1
      updateNotebookCount(updatedNote.category_id, -1)
    }
    
    // 设置新分类
    updatedNote.category_id = newCategoryId
    
    // 新分类加1
    if (newCategoryId) {
      updateNotebookCount(newCategoryId, 1)
    }
    
    // 保存更新
    const tipData = {
      id: updatedNote.id,
      title: updatedNote.title,
      content: updatedNote.content,
      tip_type: updatedNote.tip_type || 'markdown',
      language: updatedNote.language,
      category_id: updatedNote.category_id,
      tags: updatedNote.tags.map(tag => tag.id)
    }
    
    const savedNote = await tipsStore.saveTip(tipData)
    if (savedNote) {
      const index = notes.value.findIndex(note => note.id === savedNote.id)
      if (index >= 0) {
        notes.value[index] = savedNote
      }
    }
  } catch (error) {
    console.error('移动笔记失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 修改editNotebook，禁止未分类笔记本编辑
async function editNotebook(notebookId: string) {
  const notebook = findNotebookById(notebooks.value, notebookId)
  if (!notebook) {
    console.error('找不到要编辑的笔记本:', notebookId)
    return
  }
  if (isUncategorizedNotebook(notebook)) {
    // 未分类笔记本禁止编辑
    return
  }
  pendingEditNotebook.value = notebook
  editNotebookName.value = notebook.name
  showEditNotebookModal.value = true
}

async function confirmEditNotebook() {
  if (!pendingEditNotebook.value) return
  const newName = editNotebookName.value.trim()
  if (!newName || newName === pendingEditNotebook.value.name) {
    showEditNotebookModal.value = false
    return
  }
  isLoading.value = true
  try {
    const updated = await tipsStore.updateCategory(pendingEditNotebook.value.id, newName)
    if (updated) {
      pendingEditNotebook.value.name = newName
    }
  } catch (error) {
    console.error('更新笔记本失败:', error)
  } finally {
    isLoading.value = false
    showEditNotebookModal.value = false
    pendingEditNotebook.value = null
  }
}

function cancelEditNotebook() {
  showEditNotebookModal.value = false
  pendingEditNotebook.value = null
}



// 递归删除笔记本及其下所有笔记，将笔记移动到未分类
async function deleteNotebook(notebookId: string) {
  const notebook = findNotebookById(notebooks.value, notebookId)
  if (!notebook) return
  pendingDeleteNotebook.value = notebook
  showDeleteModal.value = true
}

async function confirmDeleteNotebook() {
  if (pendingDeleteNotebook.value) {
    await doDeleteNotebook(pendingDeleteNotebook.value)
  }
  showDeleteModal.value = false
  pendingDeleteNotebook.value = null
}

function cancelDeleteNotebook() {
  showDeleteModal.value = false
  pendingDeleteNotebook.value = null
}

// 真正执行删除的逻辑，只有确认后才会调用
async function doDeleteNotebook(notebook: Notebook) {
  isLoading.value = true
  try {
    // 只调用后端递归删除接口
    await tipsStore.deleteCategory(notebook.id)
    // 删除后刷新分类和笔记数据
    await tipsStore.fetchAllCategories()
    await tipsStore.fetchAllTips()
    // 同步前端显示，需转为树结构
    notebooks.value = categoriesToNotebooks(tipsStore.categories)
    notes.value = tipsStore.tips
    recountAllNotebookCounts()
  } catch (error) {
    console.error('删除笔记本失败:', error)
  } finally {
    isLoading.value = false
  }
}

function addChildNotebook(parentNotebookId: string) {
  // 存储父笔记本ID并打开添加笔记本模态框
  newNotebookParentId.value = parentNotebookId
  showAddNotebookModal.value = true
}

// 打开导入对话框
function openImportDialog() {
  showImportDialog.value = true
}

// 处理导入成功
async function handleImportSuccess() {
  showImportDialog.value = false
  await loadData()
  console.log('导入成功，数据已重新加载')
}

// 处理markdown文件拖拽导入
async function handleMarkdownImport(data: {
  title: string
  content: string
  notebookId: string
  fileName: string
}) {
  isLoading.value = true
  try {
    // 创建新笔记
    const newTipData = {
      title: data.title,
      content: data.content,
      tip_type: 'markdown',
      category_id: data.notebookId,
      tags: []
    }
    
    const savedTip = await tipsStore.saveTip(newTipData)
    if (savedTip) {
      // 更新本地笔记列表
      notes.value.unshift(savedTip)
      recountAllNotebookCounts()
      
      // 选中新创建的笔记
      selectedNoteId.value = savedTip.id
      selectedNotebookId.value = data.notebookId
      
      console.log(`Markdown文件 "${data.fileName}" 已成功导入为笔记 "${data.title}"`)
    }
  } catch (error) {
    console.error('导入markdown文件失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 重新加载标签列表
async function loadTags() {
  try {
    isLoading.value = true
    // 从store获取最新标签
    await tipsStore.fetchAllTags()
    
    // 创建新数组并赋值给tags.value
    const newTags = [...tipsStore.tags]
    tags.value = newTags
    
    console.log('标签已重新加载:', tags.value)
  } catch (error) {
    console.error('加载标签失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 在适当位置添加删除标签的函数
async function deleteTag(tagId: string) {
  isLoading.value = true;
  try {
    // 调用后端API删除标签
    const success = await invoke('delete_tag', { id: tagId });
    if (success) {
      // 从标签列表中移除
      tags.value = tags.value.filter(tag => tag.id !== tagId);
      
      // 从已选择的标签列表中移除
      selectedTags.value = selectedTags.value.filter(id => id !== tagId);
      
      // 从所有笔记的标签中移除该标签
      notes.value.forEach(note => {
        if (note.tags) {
          note.tags = note.tags.filter(tag => tag.id !== tagId);
        }
      });

      // 更新store中的标签列表
      await tipsStore.fetchAllTags();
      
      console.log(`标签 ${tagId} 已成功删除`);
    }
  } catch (error) {
    console.error('删除标签失败:', error);
  } finally {
    isLoading.value = false;
  }
}

// 递归重置所有笔记本的count
function recountAllNotebookCounts() {
  // 先清零
  function resetCounts(items: Notebook[]) {
    for (const item of items) {
      item.count = 0
      if (item.children && item.children.length > 0) {
        resetCounts(item.children)
      }
    }
  }
  resetCounts(notebooks.value)

  // 统计每个分类下的笔记数量
  notes.value.forEach(note => {
    function addCount(items: Notebook[]): boolean {
      for (const item of items) {
        if (item.id === note.category_id) {
          item.count = (item.count || 0) + 1
          return true
        }
        if (item.children && item.children.length > 0) {
          if (addCount(item.children)) return true
        }
      }
      return false
    }
    addCount(notebooks.value)
  })
}

// 添加addTagToNote方法
async function addTagToNote(tag: any) {
  if (!selectedNote.value) return
  const updatedNote = { ...selectedNote.value }
  if (!updatedNote.tags.some((t: any) => t.id === tag.id)) {
    updatedNote.tags = [...updatedNote.tags, tag]
    await updateNote(updatedNote)
    await loadTags()
  }
}

// 添加removeTagFromNote方法
async function removeTagFromNote(tagId: string) {
  if (!selectedNote.value) return
  const updatedNote = { ...selectedNote.value }
  updatedNote.tags = updatedNote.tags.filter((t: any) => t.id !== tagId)
  await updateNote(updatedNote)
}

// 添加toggleNotePin方法
async function toggleNotePin(noteId: string, isPinned: boolean) {
  isLoading.value = true
  try {
    const note = notes.value.find(n => n.id === noteId)
    if (note) {
      note.isPinned = isPinned
      await updateNote(note)
      notes.value.sort((a, b) => {
        if (a.isPinned && !b.isPinned) return -1
        if (!a.isPinned && b.isPinned) return 1
        return 0
      })
    }
  } catch (error) {
    console.error('切换笔记固定状态失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 添加loadData方法
async function loadData() {
  isLoading.value = true
  try {
    await tipsStore.initializeData()
    tags.value = tipsStore.tags
    notebooks.value = categoriesToNotebooks(tipsStore.categories)
    notes.value = tipsStore.tips
    recountAllNotebookCounts()
  } catch (error) {
    console.error('加载数据失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 递归收集所有子节点id
function collectNotebookIds(notebooks: any[], targetId: string): string[] {
  for (const nb of notebooks) {
    if (nb.id === targetId) {
      return [nb.id, ...collectAllChildIds(nb)]
    }
    if (nb.children && nb.children.length > 0) {
      const found = collectNotebookIds(nb.children, targetId)
      if (found.length > 0) return found
    }
  }
  return []
}

function collectAllChildIds(notebook: any): string[] {
  let ids: string[] = []
  if (notebook.children && notebook.children.length > 0) {
    for (const child of notebook.children) {
      ids.push(child.id)
      ids = ids.concat(collectAllChildIds(child))
    }
  }
  return ids
}

// 添加 refreshNotes 方法
async function refreshNotes() {
  isLoading.value = true
  try {
    await tipsStore.fetchAllTips()
    notes.value = tipsStore.tips
    recountAllNotebookCounts()
  } catch (error) {
    console.error('刷新笔记失败:', error)
  } finally {
    isLoading.value = false
  }
}
</script>

<style scoped>
.fade-list-enter-active, .fade-list-leave-active {
  transition: opacity 0.3s;
}
.fade-list-enter-from, .fade-list-leave-to {
  opacity: 0;
}

/* MainLayout特有的动画和布局样式 */
.main-layout-container {
  transition: all 0.3s ease;
}

.sidebar-transition {
  transition: width 0.3s ease, margin 0.3s ease;
}

.content-transition {
  transition: margin-left 0.3s ease;
}

/* 特殊的模态框样式，如果有的话 */
.modal-overlay {
  backdrop-filter: blur(4px);
}
</style> 
<template>
  <div class="h-screen flex flex-col">
    <!-- 主体内容区 -->
    <main class="flex-1 flex overflow-hidden relative">
      <!-- 左侧导航栏 -->
      <div 
        class="h-full flex-shrink-0 relative sidebar-container"
        :style="{ width: sidebarCollapsed ? '62px' : `${sidebarWidth}px` }"
      >
        <SideNavBar 
          :notebooks="notebooks"
          :tags="storeTags"
          :search-query="navSearchQuery"
          :selected-notebook-id="selectedNotebookId || undefined"
          :selected-tags="selectedTags"
          :selected-note-id="selectedNoteId"
          :notes="storeTips"
          :is-collapsed="sidebarCollapsed"
          :is-focus-mode="isFocusMode"
          :focus-section="focusSection"
          @select-notebook="selectNotebook"
          @toggle-tag="toggleTag"
          @select-note="handleNoteSelection"
          @add-notebook="showAddNotebookModal = true"
          @add-tag="showAddTagModal = true"
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
          @encrypt-notebook="handleNotebookEncryption"
          @decrypt-notebook="handleNotebookDecryption"
        />
        
        <!-- 侧边栏拖拽手柄 -->
        <div 
          v-if="!sidebarCollapsed"
          class="absolute top-0 right-0 w-1 h-full cursor-col-resize bg-transparent hover:bg-primary/30 transition-colors duration-200 z-10"
          @mousedown="startResizeSidebar"
          title="拖拽调整侧边栏宽度"
        ></div>
      </div>

      <!-- 中间笔记列表 -->
      <div 
        v-if="(!isFocusMode || focusSection === 'list') && !noteListHidden" 
        class="h-full flex-shrink-0 relative transition-all duration-300 note-list-container" 
        :style="{ 
          width: isFocusMode && focusSection === 'list' ? '100%' : `${noteListWidth}px`
        }"
      >
        <!-- 笔记列表隐藏按钮 - 垂直居中 -->
        <div class="absolute top-1/2 right-1 transform -translate-y-1/2 z-10">
          <button 
            class="btn btn-xs btn-ghost btn-circle opacity-60 hover:opacity-100 shadow-sm"
            @click="toggleNoteList"
            title="隐藏笔记列表"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
        </div>
        
        <NoteList 
          :title="listTitle"
          :loading="storeIsLoading"
          :selected-note-id="selectedNoteId || undefined"
          :notebooks="notebooks"
          :selected-notebook-id="selectedNotebookId || undefined"
          @select-note="handleNoteSelection"
          @search="handleListSearch"
          @new-note="createNewNote"
          @delete-note="deleteNote"
          @export-note="exportNote"
          @move-to-category="moveNoteToCategory"
          @refresh="refreshNotes"
          @encrypt-note="handleNoteEncryption"
          @decrypt-note="handleNoteDecryption"
          :key="selectedNotebookId || undefined"
        />
        
        <!-- 笔记列表拖拽手柄 -->
        <div 
          class="absolute top-0 right-0 w-1 h-full cursor-col-resize bg-transparent hover:bg-primary/30 transition-colors duration-200 z-10"
          @mousedown="startResizeNoteList"
          title="拖拽调整笔记列表宽度"
        ></div>
      </div>
      
      <!-- 笔记列表隐藏时的显示按钮 -->
      <div 
        v-if="noteListHidden && !isFocusMode"
        class="w-8 h-full flex-shrink-0 bg-base-200 border-r border-base-300 flex items-center justify-center relative"
      >
        <button 
          class="btn btn-xs btn-ghost btn-circle rotate-180 opacity-60 hover:opacity-100 shadow-sm"
          @click="toggleNoteList"
          title="显示笔记列表"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
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
            @unlock-note="handleNoteUnlock"
            @decrypt-note="handleNoteDecryption"
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
    
    <!-- Update Manager -->
    <UpdateManager />

    <!-- Password Dialog -->
    <PasswordDialog 
      :show="showPasswordDialog"
      :mode="passwordDialogMode"
      :title="passwordDialogTitle"
      :message="passwordDialogMessage"
      :loading="encryptionStore.isLoading"
      @confirm="handlePasswordConfirm"
      @cancel="handlePasswordCancel"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useTipsStore, Tip, Category, TipSummary } from '../stores/tipsStore'
import { useUIStore } from '../stores/uiStore'
import { useEncryptionStore } from '../stores/encryptionStore'
import { storeToRefs } from 'pinia'
import { showConfirm, showAlert } from '../services/dialog'

// Components
import SideNavBar from './SideNavBar.vue'
import NoteList from './NoteList.vue'
import NoteEditor from './NoteEditor.vue'
import ImportDialog from './ImportDialog.vue'
import MarkdownDropPreview from './MarkdownDropPreview.vue'
import UpdateManager from './UpdateManager.vue'
import PasswordDialog from './PasswordDialog.vue'


// Store
const tipsStore = useTipsStore()
const { 
  tips: storeTips, 
  categories: storeCategories, 
  tags: storeTags, 
  isLoading: storeIsLoading,
} = storeToRefs(tipsStore)

const uiStore = useUIStore()
const encryptionStore = useEncryptionStore()

// Router
const router = useRouter()

// --- Refs for UI state ---
const sidebarCollapsed = ref(false)
const sidebarWidth = ref(240)
const noteListWidth = ref(300)
const noteListHidden = ref(false)
const isFocusMode = ref(false)
const focusSection = ref('editor')

const selectedNotebookId = ref<string | null>(null)
const selectedTags = ref<string[]>([])
const selectedNoteId = ref<string | null>(null)
const selectedNote = ref<Tip | null>(null)
const listSearchQuery = ref('')
const navSearchQuery = ref('')
const notebooks = ref<any[]>([])
const flatNotebooks = ref<any[]>([])

// Modals and Dialogs state
const showAddNotebookModal = ref(false)
const newNotebookName = ref('')
const newNotebookParentId = ref<string | null>('')
const showAddTagModal = ref(false)
const newTagName = ref('')
const showDeleteModal = ref(false)
const pendingDeleteNotebook = ref<any>(null)
const showEditNotebookModal = ref(false)
const editNotebookId = ref<string | null>(null)
const editNotebookName = ref('')
const showImportDialog = ref(false)
const showPasswordDialog = ref(false)
const passwordDialogMode = ref<'unlock' | 'encrypt' | 'decrypt'>('unlock')
const passwordDialogTitle = ref('')
const passwordDialogMessage = ref('')
const currentEncryptionTarget = ref<{ type: 'note' | 'notebook', id: string } | null>(null)

// --- Computed Properties ---

const listTitle = computed(() => {
  if (selectedNotebookId.value) {
    const notebook = notebooks.value.find(n => n.id === selectedNotebookId.value)
    return notebook ? notebook.name : '全部笔记'
  }
  if (selectedTags.value.length > 0) {
    return `标签: ${selectedTags.value.map(t => storeTags.value.find(tag => tag.id === t)?.name).join(', ')}`
  }
  return '全部笔记'
})


// Helper functions for notebook tree
function buildNotebookTree(
  categories: Category[],
  tips: TipSummary[],
  parentId: string | null = null
): any[] {
  const categoryCounts = tips.reduce((acc, tip) => {
    if (tip.category_id) {
      acc[tip.category_id] = (acc[tip.category_id] || 0) + 1;
    }
    return acc;
  }, {} as Record<string, number>);

  const getCategoryTotalCount = (categoryId: string): number => {
    let total = categoryCounts[categoryId] || 0;
    const children = categories.filter(c => c.parent_id === categoryId);
    for (const child of children) {
      total += getCategoryTotalCount(child.id);
    }
    return total;
  };

  return categories
    .filter(category => category.parent_id === parentId)
    .map(category => ({
      ...category,
      count: getCategoryTotalCount(category.id),
      children: buildNotebookTree(categories, tips, category.id)
    }));
}

function flattenNotebooks(notebooks: any[], level = 0, result: any[] = []) {
  for (const notebook of notebooks) {
    result.push({ ...notebook, level });
    if (notebook.children) {
      flattenNotebooks(notebook.children, level + 1, result);
    }
  }
  return result;
}

const fetchInitialData = async () => {
  await tipsStore.fetchAllCategories()
  await tipsStore.fetchAllTags()
  await tipsStore.fetchAllTipSummaries(); 
  notebooks.value = buildNotebookTree(storeCategories.value, storeTips.value, null)
  flatNotebooks.value = flattenNotebooks(notebooks.value)
}


// --- Event Handlers ---

function handleListSearch(query: string) {
  listSearchQuery.value = query
  if (query) {
    tipsStore.searchTips(query)
      } else {
    tipsStore.fetchTips(true)
  }
}

let searchDebounceTimer: number | undefined;
watch(navSearchQuery, (query) => {
  clearTimeout(searchDebounceTimer);
  searchDebounceTimer = window.setTimeout(() => {
    if (query.trim()) {
      tipsStore.searchTips(query.trim());
    } else {
      if (selectedNotebookId.value) {
        tipsStore.fetchTipsByCategory(selectedNotebookId.value);
      } else if (selectedTags.value.length > 0) {
        tipsStore.fetchTipsByTag(selectedTags.value.join(','));
      } else {
        tipsStore.fetchTips(true);
      }
    }
  }, 300);
});

// 接收来自NoteList的事件
async function handleNoteSelection(noteOrId: string | Tip) {
  let note: Tip | null = null;
  if (typeof noteOrId === 'string') {
    note = await tipsStore.fetchTip(noteOrId);
  } else {
    note = noteOrId;
  }

  if (note) {
    await nextTick(); // 等待DOM更新
    selectNote(note);
  } else {
    console.warn(`Note not found for selection:`, noteOrId);
  }
}

function selectNote(note: Tip) {
  if (note && note.id) {
    selectedNote.value = { ...note }; // 使用扩展运算符创建一个新的对象
    selectedNoteId.value = note.id;
  } else {
    console.warn("Invalid note selected:", note);
    selectedNote.value = null;
    selectedNoteId.value = null;
  }
}


async function refreshNotes() {
  await tipsStore.fetchTips(true)
}

// --- Lifecycle Hooks ---

onMounted(async () => {
  // 从 localStorage 恢复宽度
  const savedSidebarWidth = localStorage.getItem('sidebarWidth');
  if (savedSidebarWidth) sidebarWidth.value = parseFloat(savedSidebarWidth);
  
  const savedNoteListWidth = localStorage.getItem('noteListWidth');
  if (savedNoteListWidth) noteListWidth.value = parseFloat(savedNoteListWidth);

  // 从 localStorage 恢复笔记列表显示状态
  const savedNoteListHidden = localStorage.getItem('noteListHidden');
  if (savedNoteListHidden) noteListHidden.value = savedNoteListHidden === 'true';

  await fetchInitialData()
  tipsStore.fetchTips(true)

  // ... other onMounted logic
})

// watch for selection changes to clear search
watch(selectedNotebookId, () => {
  listSearchQuery.value = ''
  navSearchQuery.value = ''
})

watch(selectedTags, () => {
  listSearchQuery.value = ''
  navSearchQuery.value = ''
})



// Example of an updated method:
async function createNewNote() {
  const newNoteData = {
    title: '无标题笔记',
    content: '# 新笔记\n\n在这里开始你的创作...',
    tip_type: 'markdown', // 添加缺失的字段
    category_id: selectedNotebookId.value || undefined,
    tags: selectedTags.value // 直接使用 selectedTags
  };

  const savedNote = await tipsStore.saveTip(newNoteData);

  // 新增: 保存成功后立即更新计数
  if (savedNote && savedNote.category_id) {
    updateNotebookTreeCount(savedNote.category_id, 1);
  }

  if (savedNote) {
    // saveTip 应该返回完整的笔记对象，并已将其添加到store中
    await handleNoteSelection(savedNote);
  } else {
    console.error("Failed to create new note.");
    showAlert('创建新笔记失败，请稍后再试。', { title: '错误' });
  }
}

// 添加一个辅助函数来更新笔记本树的计数
function updateNotebookTreeCount(categoryId: string | undefined, delta: number) {
  if (!categoryId) return;
  
  // 递归更新笔记本及其所有父级的计数
  const updateCategoryCount = (notebookTree: any[], id: string): boolean => {
    for (const notebook of notebookTree) {
      if (notebook.id === id) {
        // 找到目标笔记本，更新计数
        notebook.count = (notebook.count || 0) + delta;
        return true;
      }
      
      // 递归检查子笔记本
      if (notebook.children && notebook.children.length > 0) {
        if (updateCategoryCount(notebook.children, id)) {
          // 如果在子笔记本中找到了目标，也要更新当前笔记本的计数
          notebook.count = (notebook.count || 0) + delta;
          return true;
        }
      }
    }
    
    return false;
  };
  
  // 更新笔记本树
  updateCategoryCount(notebooks.value, categoryId);
  
  // 更新扁平化的笔记本列表
  flatNotebooks.value = flattenNotebooks(notebooks.value);
}

async function deleteNote(id: string) {
  const confirmed = await showConfirm('确定要删除这篇笔记吗？', { title: '确认删除' });
  if (confirmed) {
    try {
      // 保存要删除笔记的分类ID，用于后续更新UI
      const noteToDelete = storeTips.value.find(note => note.id === id);
      const categoryId = noteToDelete?.category_id;
      
      // 调用后端API删除笔记
      await tipsStore.deleteTip(id);
      
      // 前端直接更新当前视图中的笔记列表
      storeTips.value = storeTips.value.filter(note => note.id !== id);
      
      // 如果笔记有分类，直接在前端更新笔记本树的计数
      if (categoryId) {
        updateNotebookTreeCount(categoryId, -1); // 减少计数
      }
      
      // 如果删除的是当前选中的笔记，则清除选中状态
      if (selectedNoteId.value === id) {
        selectedNoteId.value = null;
        selectedNote.value = null;
      }
    } catch (error) {
      console.error('删除笔记失败:', error);
    }
  }
}

async function updateNote(updatedFields: Partial<Tip> & { id: string }) {
  // 获取现有笔记数据（从选中笔记或全局列表中查找）
  const existingNote = selectedNote.value && selectedNote.value.id === updatedFields.id
    ? selectedNote.value
    : storeTips.value.find(n => n.id === updatedFields.id) as unknown as Tip | undefined

  if (!existingNote) {
    console.error(`Update failed: note ${updatedFields.id} not found.`)
    return
  }

  // 合并现有数据与更新字段，确保所有必需字段存在
  const merged: Tip = {
    ...existingNote,
    ...updatedFields,
    // 当部分更新未包含 tags 时使用原有 tags，保证后续 map 不报错
    tags: updatedFields.tags ?? existingNote.tags ?? [],
    tip_type: updatedFields.tip_type ?? existingNote.tip_type,
    language: updatedFields.language ?? existingNote.language,
    category_id: updatedFields.category_id ?? existingNote.category_id,
  }

  const savedNote = await tipsStore.saveTip({
    id: merged.id,
    title: merged.title,
    content: merged.content,
    tip_type: merged.tip_type,
    language: merged.language,
    category_id: merged.category_id,
    tags: (merged.tags ?? []).map(t => t.name)
  })

  if (!savedNote) return

  // 更新本地 TipSummary
  const tipSummary: TipSummary = {
    id: savedNote.id,
    title: savedNote.title,
    tip_type: savedNote.tip_type,
    language: savedNote.language,
    category_id: savedNote.category_id,
    created_at: savedNote.created_at,
    updated_at: savedNote.updated_at,
    tags: savedNote.tags,
    is_encrypted: savedNote.is_encrypted || false,
    content: savedNote.content,
  }

  const idx = storeTips.value.findIndex(t => t.id === savedNote.id)
  if (idx !== -1) {
    storeTips.value[idx] = tipSummary
  }

  // 处理分类变更计数
  if (existingNote.category_id !== savedNote.category_id) {
    if (existingNote.category_id) updateNotebookTreeCount(existingNote.category_id, -1)
    if (savedNote.category_id) updateNotebookTreeCount(savedNote.category_id, 1)

    // 如果当前列表是旧分类，移除条目
    if (selectedNotebookId.value && selectedNotebookId.value !== savedNote.category_id && existingNote.category_id === selectedNotebookId.value) {
      storeTips.value = storeTips.value.filter(t => t.id !== savedNote.id)
    }
  }

  // 更新当前选中笔记
  selectedNote.value = savedNote
}


// ... [other methods like navigation, modal handling, etc.]
function navigateTo(path: string) {
  router.push(path)
}

function addChildNotebook({ parentId, parentName }: { parentId: string, parentName: string }) {
  newNotebookParentId.value = parentId
  showAddNotebookModal.value = true
}

function editNotebook(notebook: any) {
  editNotebookId.value = notebook.id
  editNotebookName.value = notebook.name
  showEditNotebookModal.value = true
}

function deleteNotebook(notebookId: string) {
  const notebook = flatNotebooks.value.find(n => n.id === notebookId);
  if (notebook) {
    pendingDeleteNotebook.value = notebook;
    showDeleteModal.value = true;
  } else {
    console.warn(`Notebook with id ${notebookId} not found.`);
  }
}

function deleteTag(tagId: string) {
  // implementation
}

function openImportDialog() {
  showImportDialog.value = true
}

function handleNotebookEncryption({ id, name }: { id: string; name: string }) {
  console.log(`Encrypting notebook: ${name} (${id})`);
  // implementation
}

function handleNotebookDecryption({ id, name }: { id: string; name: string }) {
  console.log(`Decrypting notebook: ${name} (${id})`);
  // implementation
}

function createResizeHandler(
  containerSelector: string,
  widthRef: typeof sidebarWidth | typeof noteListWidth,
  storageKey: string,
  minWidth: number,
  maxWidth: number
) {
  return function (e: MouseEvent) {
    e.preventDefault();

    const container = document.querySelector(containerSelector) as HTMLElement;
    if (!container) return;

    const startX = e.clientX;
    const startWidth = widthRef.value;
    let animationFrameId: number | null = null;

    const onMouseMove = (moveEvent: MouseEvent) => {
      if (animationFrameId) cancelAnimationFrame(animationFrameId);

      animationFrameId = requestAnimationFrame(() => {
        const deltaX = moveEvent.clientX - startX;
        let newWidth = startWidth + deltaX;

        if (newWidth < minWidth) newWidth = minWidth;
        if (newWidth > maxWidth) newWidth = maxWidth;

        container.style.width = `${newWidth}px`;
      });
    };

    const onMouseUp = (upEvent: MouseEvent) => {
      if (animationFrameId) cancelAnimationFrame(animationFrameId);
      
      const finalWidth = parseFloat(container.style.width) || widthRef.value;
      widthRef.value = finalWidth;
      localStorage.setItem(storageKey, String(finalWidth));

      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };

    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
    document.body.style.cursor = 'col-resize';
    document.body.style.userSelect = 'none';
  };
}

const startResizeSidebar = createResizeHandler('.sidebar-container', sidebarWidth, 'sidebarWidth', 180, 500);
const startResizeNoteList = createResizeHandler('.note-list-container', noteListWidth, 'noteListWidth', 240, 600);

function toggleNoteList() {
  noteListHidden.value = !noteListHidden.value;
  localStorage.setItem('noteListHidden', String(noteListHidden.value));
}

async function selectNotebook(id: string) {
  selectedNotebookId.value = id
  selectedTags.value = [] // clear tag selection
  await tipsStore.fetchTipsByCategory(id)

  // 自动选中第一个笔记
  if (storeTips.value.length > 0) {
    const firstNoteId = storeTips.value[0].id
    const note = await tipsStore.fetchTip(firstNoteId)
    if (note) {
      selectNote(note)
    }
  } else {
    // 如果笔记本为空，则清除当前选中的笔记
    selectedNote.value = null
    selectedNoteId.value = null
  }
}
function toggleTag(id: string) {
  const index = selectedTags.value.indexOf(id);
  if (index > -1) {
    selectedTags.value.splice(index, 1);
  } else {
    selectedTags.value.push(id);
  }
  selectedNotebookId.value = null; // clear notebook selection
  if (selectedTags.value.length > 0) {
    tipsStore.fetchTipsByTag(selectedTags.value.join(',')); // Assuming API can handle multiple tags
  } else {
    tipsStore.fetchTips(true);
  }
}

function exportNote(id: string, format: string) {}
function moveNoteToCategory(noteId: string, categoryId: string) {
  // 查找要移动的笔记
  const noteToMove = storeTips.value.find(note => note.id === noteId);
  if (!noteToMove) {
    console.error(`笔记 ${noteId} 未找到，无法移动`);
    return;
  }
  
  // 保存原分类ID，用于后续更新UI
  const oldCategoryId = noteToMove.category_id;
  
  // 调用API移动笔记
  tipsStore.saveTip({
    id: noteId,
    title: noteToMove.title,
    content: noteToMove.content || '',
    tip_type: noteToMove.tip_type,
    language: noteToMove.language,
    category_id: categoryId,
    tags: noteToMove.tags.map(t => t.name)
  }).then(async (savedNote) => {
    if (savedNote) {
      // 更新前端状态
      const tipSummary: TipSummary = {
        id: savedNote.id,
        title: savedNote.title,
        tip_type: savedNote.tip_type,
        language: savedNote.language,
        category_id: savedNote.category_id,
        created_at: savedNote.created_at,
        updated_at: savedNote.updated_at,
        tags: savedNote.tags,
        is_encrypted: savedNote.is_encrypted || false,
        content: savedNote.content,
      };
      
      // 更新前端列表中的笔记
      const index = storeTips.value.findIndex(note => note.id === savedNote.id);
      if (index !== -1) {
        storeTips.value[index] = tipSummary;
      }
      
      // 更新笔记本树的计数
      if (oldCategoryId) {
        updateNotebookTreeCount(oldCategoryId, -1); // 从原分类减少计数
      }
      if (categoryId) {
        updateNotebookTreeCount(categoryId, 1); // 向新分类增加计数
      }
      
      // 如果当前选中的是被移动的笔记，更新选中的笔记
      if (selectedNoteId.value === noteId) {
        selectedNote.value = savedNote;
      }
      
      // 如果当前在特定笔记本下，可能需要从视图中移除该笔记
      if (selectedNotebookId.value && selectedNotebookId.value !== categoryId && noteToMove.category_id === selectedNotebookId.value) {
        // 从当前视图中移除笔记
        storeTips.value = storeTips.value.filter(note => note.id !== noteId);
      }
    }
  }).catch(err => {
    console.error(`移动笔记 ${noteId} 到分类 ${categoryId} 失败:`, err);
  });
}
function handleNoteEncryption(id: string) {}
function handleNoteDecryption(id: string) {}
function duplicateNote(id: string) {}
function addTagToNote(tag: string) {}
function removeTagFromNote(tagId: string) {}
function toggleNotePin(id: string) {}
function handleNoteUnlock(id: string) {}

async function addNotebook() {
  if(newNotebookName.value.trim()){
    await tipsStore.createCategory(newNotebookName.value.trim(), newNotebookParentId.value || undefined)
    await fetchInitialData()
    showAddNotebookModal.value = false
    newNotebookName.value = ''
    newNotebookParentId.value = ''
  }
}

async function addTag() {
  if (newTagName.value.trim()) {
    await tipsStore.createTag(newTagName.value.trim())
    await fetchInitialData()
    showAddTagModal.value = false
    newTagName.value = ''
  }
}

function cancelDeleteNotebook() {
  showDeleteModal.value = false
  pendingDeleteNotebook.value = null
}

async function confirmDeleteNotebook() {
  if(pendingDeleteNotebook.value) {
    await tipsStore.deleteCategory(pendingDeleteNotebook.value.id)
    await fetchInitialData()
    showDeleteModal.value = false
  }
}

function cancelEditNotebook() {
  showEditNotebookModal.value = false
  editNotebookId.value = null
  editNotebookName.value = ''
}

async function confirmEditNotebook() {
  if(editNotebookId.value && editNotebookName.value.trim()) {
    await tipsStore.updateCategory(editNotebookId.value, editNotebookName.value.trim())
    await fetchInitialData()
    showEditNotebookModal.value = false
  }
}

function handleImportSuccess(importResult: any) {
  // 导入成功后，后端已经创建了新的笔记本
  // 我们只需要重新获取所有分类，然后重建前端的笔记本树
  tipsStore.fetchAllCategories().then(() => {
    // 使用 store 中现有的 tips 数据来重建树
    // 因为导入的笔记可能还没有被加载到 storeTips 中，
    // 所以这里的计数可能不是最新的，但笔记本本身会出现
    notebooks.value = buildNotebookTree(storeCategories.value, storeTips.value, null)
    flatNotebooks.value = flattenNotebooks(notebooks.value)
    
    // 为了确保计数也正确，我们可以选择性地刷新所有笔记摘要
    // 这是一个权衡，可以在这里调用，也可以留给用户手动刷新
    tipsStore.fetchAllTipSummaries().then(allTips => {
       notebooks.value = buildNotebookTree(storeCategories.value, allTips, null);
       flatNotebooks.value = flattenNotebooks(notebooks.value);
    });
  })
}

function handleMarkdownImport() {
  refreshNotes()
}

function handlePasswordConfirm() {}

function handlePasswordCancel() {
  showPasswordDialog.value = false
  currentEncryptionTarget.value = null
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

/* 拖拽调整大小样式 */
.resize-handle {
  position: absolute;
  top: 0;
  right: 0;
  width: 4px;
  height: 100%;
  cursor: col-resize;
  background: transparent;
  transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 10;
  will-change: background-color;
}

.resize-handle:hover {
  background-color: rgba(var(--primary), 0.3);
  width: 6px;
}

.resize-handle:active {
  background-color: rgba(var(--primary), 0.5);
  width: 6px;
}

/* 拖拽时的全局样式 */
body.dragging {
  cursor: col-resize !important;
  user-select: none !important;
}

body.dragging * {
  cursor: col-resize !important;
  user-select: none !important;
}

/* 拖拽手柄在不同主题下的样式 */
[data-theme="dark"] .resize-handle:hover,
[data-theme="night"] .resize-handle:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

[data-theme="dark"] .resize-handle:active,
[data-theme="night"] .resize-handle:active {
  background-color: rgba(255, 255, 255, 0.3);
}

/* 笔记列表隐藏/显示动画 */
.note-list-enter-active,
.note-list-leave-active {
  transition: all 0.3s ease;
}

.note-list-enter-from {
  opacity: 0;
  transform: translateX(-100%);
}

.note-list-leave-to {
  opacity: 0;
  transform: translateX(-100%);
}

/* 隐藏按钮的悬停效果 */
.toggle-note-list-btn {
  transition: all 0.2s ease;
}

.toggle-note-list-btn:hover {
  background-color: rgba(var(--primary), 0.1);
  transform: scale(1.1);
}

/* 确保拖拽时元素不被选中 */
.no-select {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .resize-handle {
    width: 6px; /* 在移动设备上增加拖拽手柄宽度 */
  }
}

/* 拖拽手柄的视觉提示 */
.resize-handle::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 2px;
  height: 30px;
  background: repeating-linear-gradient(
    to bottom,
    transparent,
    transparent 2px,
    rgba(var(--bc), 0.3) 2px,
    rgba(var(--bc), 0.3) 4px
  );
  opacity: 0;
  transition: opacity 0.2s ease;
}

.resize-handle:hover::before {
  opacity: 1;
}

/* 容器优化样式 */
.sidebar-container,
.note-list-container {
  transition: width 0.1s cubic-bezier(0.4, 0, 0.2, 1);
  will-change: width;
}

/* 拖拽时的优化 */
.sidebar-container.resizing,
.note-list-container.resizing {
  transition: none;
  will-change: width;
}

/* 拖拽完成时的反馈动画 */
.sidebar-container.resize-complete,
.note-list-container.resize-complete {
  animation: resize-complete 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes resize-complete {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.002);
  }
  100% {
    transform: scale(1);
  }
}

/* 拖拽时的全局优化 */
body.dragging {
  cursor: col-resize !important;
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
  -ms-user-select: none !important;
}

body.dragging * {
  cursor: col-resize !important;
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
  -ms-user-select: none !important;
  pointer-events: none;
}

/* 拖拽手柄在拖拽时保持可交互 */
body.dragging .resize-handle {
  pointer-events: auto !important;
}

/* 高性能渲染优化 */
.sidebar-container,
.note-list-container {
  transform: translateZ(0);
  backface-visibility: hidden;
  perspective: 1000px;
}

/* 减少重绘和回流 */
.dragging .sidebar-container,
.dragging .note-list-container {
  contain: layout style paint;
}
</style> 
<template>
  <div class="h-screen flex flex-col bg-base-100">
    <!-- Mobile Header -->
    <div v-if="isMobile" class="flex-shrink-0 flex items-center justify-between p-2 bg-base-200 border-b border-base-300 h-14">
      <button v-if="!selectedNoteId" class="btn btn-ghost btn-square" @click="isSidebarOpenOnMobile = true">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
      <button v-else class="btn btn-ghost btn-square" @click="deselectNote">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </button>
      <div class="font-bold text-lg">
        <span v-if="selectedNote">{{ selectedNote.title }}</span>
        <span v-else>{{ listTitle }}</span>
      </div>
      <!-- Placeholder for potential actions -->
      <div class="w-10"></div>
    </div>

    <!-- Main Content -->
    <main class="flex-1 flex overflow-hidden relative">
      <!-- Sidebar for Mobile -->
      <div v-if="isMobile" 
           :class="['fixed inset-0 z-40 transition-transform duration-300', isSidebarOpenOnMobile ? 'translate-x-0' : '-translate-x-full']">
        <div class="absolute inset-0 bg-black/50" @click="isSidebarOpenOnMobile = false"></div>
        <div class="relative w-64 h-full bg-base-200 shadow-xl">
          <SideNavBar 
            :notebooks="notebooks"
            :tags="storeTags"
            :search-query="navSearchQuery"
            :selected-notebook-id="selectedNotebookId || undefined"
            :selected-tags="selectedTags"
            :selected-note-id="selectedNoteId"
            :notes="storeTips"
            :is-collapsed="false"
            :is-focus-mode="isFocusMode"
            :focus-section="focusSection"
            @select-notebook="selectNotebook($event); handleMobileNav()"
            @toggle-tag="toggleTag($event); handleMobileNav()"
            @select-note="handleNoteSelection($event); handleMobileNav()"
            @add-notebook="showAddNotebookModal = true"
            @add-tag="showAddTagModal = true"
            @import="openImportDialog"
            @add-child-notebook="addChildNotebook"
            @edit-notebook="editNotebook"
            @delete-notebook="deleteNotebook"
            @delete-tag="deleteTag"
            @search="(query: string) => navSearchQuery = query"
            @new-note="createNewNote(); handleMobileNav()"
            @clipboard="() => { navigateTo('/clipboard'); handleMobileNav(); }"
            @ai-assistant="() => { navigateTo('/ai-assistant'); handleMobileNav(); }"
            @settings="() => { navigateTo('/settings'); handleMobileNav(); }"
            @encrypt-notebook="handleNotebookEncryption"
            @decrypt-notebook="handleNotebookDecryption"
          />
        </div>
      </div>
      
      <!-- Sidebar for Desktop -->
      <div v-if="!isMobile"
        class="h-full flex-shrink-0 relative sidebar-container sidebar-width"
        :style="{ width: `${sidebarWidth}px`, display: sidebarCollapsed ? 'none' : 'flex' }"
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
          @search="(query: string) => navSearchQuery = query"
          @new-note="createNewNote"
          @clipboard="() => navigateTo('/clipboard')"
          @ai-assistant="() => navigateTo('/ai-assistant')"
          @settings="() => navigateTo('/settings')"
          @encrypt-notebook="handleNotebookEncryption"
          @decrypt-notebook="handleNotebookDecryption"
        />
        
        <div 
          v-if="!sidebarCollapsed"
          class="resize-handle sidebar-resize-handle"
          @mousedown="startResizeSidebar"
          :title="t('mainLayout.resizeSidebar')"
        ></div>
      </div>

      <!-- Content Area -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Desktop Layout -->
        <template v-if="!isMobile">
          <div 
            v-if="(!isFocusMode || focusSection === 'list') && !noteListHidden" 
            class="h-full flex-shrink-0 relative transition-all duration-300 note-list-container" 
            :style="{ 
              width: isFocusMode && focusSection === 'list' ? '100%' : `${noteListWidth}px`
            }"
          >
            <div class="absolute top-1/2 right-1 transform -translate-y-1/2 z-10">
              <button 
                class="btn btn-xs btn-ghost btn-circle opacity-60 hover:opacity-100 shadow-sm"
                @click="toggleNoteList"
                :title="t('mainLayout.hideNoteList')"
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
              :tips="displayTips"
              :total-count="displayTotalCount"
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
            
            <div 
              class="resize-handle notelist-resize-handle"
              @mousedown="startResizeNoteList"
              :title="t('mainLayout.resizeNoteList')"
            ></div>
          </div>
          
          <div 
            v-if="noteListHidden && !isFocusMode"
            class="w-8 h-full flex-shrink-0 bg-base-200 border-r border-base-300 flex items-center justify-center relative"
          >
            <button 
              class="btn btn-xs btn-ghost btn-circle rotate-180 opacity-60 hover:opacity-100 shadow-sm"
              @click="toggleNoteList"
              :title="t('mainLayout.showNoteList')"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
          </div>

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
              <h2 class="text-2xl font-bold mb-2">{{ t('mainLayout.selectOrCreateNote') }}</h2>
              <p class="mb-4 text-center max-w-md">
                {{ t('mainLayout.selectOrCreateNoteHint') }}
              </p>
            </div>
          </div>
        </template>
        
        <!-- Mobile Layout -->
        <template v-else>
          <!-- Note Editor is shown only if a note is selected -->
          <div v-if="selectedNoteId && selectedNote" class="h-full w-full">
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

          <!-- Note List is shown otherwise -->
          <div v-else class="h-full w-full">
            <NoteList 
              :title="listTitle"
              :loading="storeIsLoading"
              :selected-note-id="selectedNoteId || undefined"
              :notebooks="notebooks"
              :selected-notebook-id="selectedNotebookId || undefined"
              :tips="displayTips"
              :total-count="displayTotalCount"
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
          </div>
        </template>
      </div>
    </main>

    <!-- 添加笔记本模态框 -->
    <div class="modal" :class="{'modal-open': showAddNotebookModal}">
      <div class="modal-box">
        <h3 class="font-bold text-lg">{{ t('mainLayout.createNotebook') }}</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">{{ t('mainLayout.notebookName') }}</span>
          </label>
          <input type="text" :placeholder="t('mainLayout.enterName')" class="input input-bordered w-full" v-model="newNotebookName" />
          
          <label class="label mt-2">
            <span class="label-text">{{ t('mainLayout.parentNotebook') }}</span>
          </label>
          <select class="select select-bordered w-full" v-model="newNotebookParentId">
            <option value="">{{ t('mainLayout.noParent') }}</option>
            <option v-for="notebook in flatNotebooks" :key="notebook.id" :value="notebook.id">
              {{ notebook.name }}
            </option>
          </select>
        </div>
        <div class="modal-action">
          <button class="btn" @click="showAddNotebookModal = false">{{ t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="addNotebook" :disabled="!newNotebookName.trim()">{{ t('common.create') }}</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showAddNotebookModal = false"></div>
    </div>

    <!-- 添加标签模态框 -->
    <div class="modal" :class="{'modal-open': showAddTagModal}">
      <div class="modal-box">
        <h3 class="font-bold text-lg">{{ t('mainLayout.createTag') }}</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">{{ t('mainLayout.tagName') }}</span>
          </label>
          <input type="text" :placeholder="t('mainLayout.enterTag')" class="input input-bordered w-full" v-model="newTagName" />
        </div>
        <div class="modal-action">
          <button class="btn" @click="showAddTagModal = false">{{ t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="addTag" :disabled="!newTagName.trim()">{{ t('common.create') }}</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showAddTagModal = false"></div>
    </div>

    <!-- 删除笔记本确认 Modal -->
    <div class="modal" :class="{ 'modal-open': showDeleteModal }">
      <div class="modal-box">
        <h3 class="font-bold text-lg text-error">{{ t('mainLayout.confirmDeleteNotebookTitle') }}</h3>
        <p class="py-4">
          <span v-if="pendingDeleteNotebook">
            {{ t('mainLayout.confirmDeleteNotebookMessage', { name: pendingDeleteNotebook.name }) }}
          </span>
        </p>
        <div class="modal-action">
          <button class="btn" @click="cancelDeleteNotebook">{{ t('common.cancel') }}</button>
          <button class="btn btn-error" @click="confirmDeleteNotebook">{{ t('mainLayout.confirmDelete') }}</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="cancelDeleteNotebook"></div>
    </div>

    <!-- 编辑笔记本 Modal -->
    <div class="modal" :class="{ 'modal-open': showEditNotebookModal }">
      <div class="modal-box">
        <h3 class="font-bold text-lg">{{ t('mainLayout.editNotebook') }}</h3>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">{{ t('mainLayout.newName') }}</span>
          </label>
          <input type="text" class="input input-bordered w-full" v-model="editNotebookName" />
        </div>
        <div class="modal-action">
          <button class="btn" @click="cancelEditNotebook">{{ t('common.cancel') }}</button>
          <button class="btn btn-primary" @click="confirmEditNotebook" :disabled="!editNotebookName.trim()">{{ t('common.confirm') }}</button>
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
import { ref, computed, onMounted, watch, nextTick, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTipsStore, Tip, Category, TipSummary } from '../stores/tipsStore'
import { useEncryptionStore } from '../stores/encryptionStore'
import { storeToRefs } from 'pinia'
import { showConfirm, showAlert } from '../services/dialog'
import { useResponsive } from '../composables/useResponsive'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { useLocalStorageStore } from '../stores/localStorageStore'


// Components
import SideNavBar from './SideNavBar.vue'
import NoteList from './NoteList.vue'
import NoteEditor from './NoteEditor.vue'
import ImportDialog from './ImportDialog.vue'
import MarkdownDropPreview from './MarkdownDropPreview.vue'
import UpdateManager from './UpdateManager.vue'
import PasswordDialog from './PasswordDialog.vue'

const { t } = useI18n()

// Store
const tipsStore = useTipsStore()
const { 
  tips: storeTips, 
  categories: storeCategories, 
  tags: storeTags, 
  isLoading: storeIsLoading,
} = storeToRefs(tipsStore)

// 数据库状态管理
import { useDatabaseStore } from '../stores/databaseStore'
const databaseStore = useDatabaseStore()
const { databaseChangeCounter } = storeToRefs(databaseStore)

const encryptionStore = useEncryptionStore()
const localStorageStore = useLocalStorageStore()

// Router
const router = useRouter()

// --- Responsive state ---
const { isMobile } = useResponsive()
const isSidebarOpenOnMobile = ref(false)


watch(isMobile, (mobile) => {
  if (!mobile) {
    sidebarCollapsed.value = false;
    isSidebarOpenOnMobile.value = false
  }
});

const handleMobileNav = () => {
  if (isMobile.value) {
    isSidebarOpenOnMobile.value = false
  }
}

const deselectNote = () => {
  selectedNote.value = null;
  selectedNoteId.value = null;
}

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

function rebuildNotebookTree() {
  notebooks.value = buildNotebookTree(storeCategories.value, storeTips.value, null)
  flatNotebooks.value = flattenNotebooks(notebooks.value)
}

watch([storeCategories, storeTips], rebuildNotebookTree, { immediate: true })

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
    const notebook = flatNotebooks.value.find(n => n.id === selectedNotebookId.value)
    return notebook ? notebook.name : t('mainLayout.allNotes')
  }
  if (selectedTags.value.length > 0) {
    return `${t('mainLayout.tag')}: ${selectedTags.value.map(t => storeTags.value.find(tag => tag.id === t)?.name).join(', ')}`
  }
  return t('mainLayout.allNotes')
})

// 为NoteList提供统一的数据源
const displayTips = computed(() => {
  // 如果选中了笔记本且有分类浏览数据，使用分类浏览的数据
  if (selectedNotebookId.value && tipsStore.currentCategoryBrowse) {
    const categoryData = tipsStore.currentCategoryBrowse
    const result: TipSummary[] = []
    const seenIds = new Set<string>() // 用于去重
    
    // 添加特色笔记（第一条完整笔记）转换为TipSummary格式
    if (categoryData.featured_tip) {
      const featured = categoryData.featured_tip
      result.push({
        id: featured.id,
        title: featured.title,
        tip_type: featured.tip_type,
        language: featured.language,
        category_id: featured.category_id,
        created_at: featured.created_at,
        updated_at: featured.updated_at,
        tags: featured.tags,
        is_encrypted: featured.is_encrypted || false,
        content: featured.content // 保留完整内容
      })
      seenIds.add(featured.id) // 记录已添加的ID
    }
    
    // 添加其他笔记摘要，确保不重复
    for (const summary of categoryData.tip_summaries) {
      if (!seenIds.has(summary.id)) {
        result.push(summary)
        seenIds.add(summary.id)
      }
    }
    
    return result
  }
  
  // 否则使用常规的tips数据
  return storeTips.value
})

// 计算显示的笔记总数
const displayTotalCount = computed(() => {
  // 如果选中了笔记本且有分类浏览数据，使用分类浏览的总数
  if (selectedNotebookId.value && tipsStore.currentCategoryBrowse) {
    return tipsStore.currentCategoryBrowse.total_tips_count
  }
  
  // 否则使用当前tips的长度
  return storeTips.value.length
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

// 记录搜索前的状态，用于恢复
const lastSelectedState = ref({
  notebookId: null as string | null,
  tags: [] as string[]
})

let searchDebounceTimer: number | undefined;
watch(navSearchQuery, (query) => {
  clearTimeout(searchDebounceTimer);
  searchDebounceTimer = window.setTimeout(() => {
    if (query.trim()) {
      // 搜索前保存当前状态
      lastSelectedState.value = {
        notebookId: selectedNotebookId.value,
        tags: [...selectedTags.value]
      }
      
      // 执行搜索时清除选中状态，确保显示搜索结果
      selectedNotebookId.value = null;
      selectedTags.value = [];
      tipsStore.searchTips(query.trim());
    } else {
      // 搜索框清空时，恢复之前的状态
      if (lastSelectedState.value.notebookId) {
        selectedNotebookId.value = lastSelectedState.value.notebookId;
        selectedTags.value = [];
        // 使用新的分类浏览API
        tipsStore.browseCategory(lastSelectedState.value.notebookId);
      } else if (lastSelectedState.value.tags.length > 0) {
        selectedTags.value = [...lastSelectedState.value.tags];
        selectedNotebookId.value = null;
        tipsStore.fetchTipsByTag(lastSelectedState.value.tags.join(','));
      } else {
        // 如果之前没有选中任何状态，显示全部笔记
        selectedNotebookId.value = null;
        selectedTags.value = [];
        tipsStore.fetchTips(true);
      }
    }
  }, 200);
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
  // 从 localStorageStore 恢复宽度
  if (localStorageStore.data.sidebarWidth) sidebarWidth.value = localStorageStore.data.sidebarWidth;
  
  if (localStorageStore.data.noteListWidth) noteListWidth.value = localStorageStore.data.noteListWidth;

  // 从 localStorageStore 恢复笔记列表显示状态
  noteListHidden.value = localStorageStore.data.noteListHidden;

  await fetchInitialData()
  tipsStore.fetchTips(true)

  // ... other onMounted logic
})

// 监听数据库切换事件，自动刷新数据
watch(databaseChangeCounter, async (newCount, oldCount) => {
  if (newCount > oldCount && newCount > 0) {
    console.log('[MainLayout] 检测到数据库切换/同步事件，开始刷新数据...')
    try {
      // 清除当前选中状态
      selectedNote.value = null
      selectedNoteId.value = null
      selectedNotebookId.value = null
      selectedTags.value = []
      
      // 重新获取所有数据
      await fetchInitialData()
      
      // 刷新笔记列表
      await tipsStore.fetchAllTipSummaries()
      
      console.log('[MainLayout] 数据刷新完成')
    } catch (error) {
      console.error('[MainLayout] 数据刷新失败:', error)
    }
  }
})

onUnmounted(() => {
  // No longer need to remove resize listener here as it's handled globally
})

// watch for selection changes to clear search
watch(selectedNotebookId, () => {
  listSearchQuery.value = ''
  // 只有在不是恢复状态时才清空搜索框
  if (!navSearchQuery.value.trim()) {
    navSearchQuery.value = ''
  }
})

watch(selectedTags, () => {
  listSearchQuery.value = ''
  // 只有在不是恢复状态时才清空搜索框
  if (!navSearchQuery.value.trim()) {
    navSearchQuery.value = ''
  }
})

// Helper function to get or create the 'Uncategorized' notebook
async function getUncategorizedNotebookId(): Promise<string> {
  const uncategorizedName = t('mainLayout.uncategorized');
  let uncategorized = storeCategories.value.find(c => c.name === uncategorizedName);
  if (uncategorized) {
    return uncategorized.id;
  } else {
    // If it doesn't exist, create it
    const newCategory = await tipsStore.createCategory(uncategorizedName);
    if (newCategory) {
      // Manually add the new category to the store to avoid re-fetching
      storeCategories.value.push(newCategory);
      await fetchInitialData(); // Rebuild tree after adding category
      return newCategory.id;
    } else {
      showAlert(t('mainLayout.errorCreateUncategorized'), { title: t('common.error') });
      throw new Error("Failed to create 'Uncategorized' notebook.");
    }
  }
}


// Example of an updated method:
async function createNewNote() {
  let categoryId = selectedNotebookId.value;

  // If no notebook is selected, assign to 'Uncategorized'
  if (!categoryId) {
    try {
      categoryId = await getUncategorizedNotebookId();
    } catch (error) {
      console.error(error);
      return; // Stop if we can't get the category ID
    }
  }

  const newNoteData = {
    title: t('mainLayout.untitledNote'),
    content: `# ${t('mainLayout.newNote')}\n\n${t('mainLayout.newNoteHint')}`,
    tip_type: 'markdown',
    category_id: categoryId || undefined,
    tags: selectedTags.value
  };

  const savedNote = await tipsStore.saveTip(newNoteData);

  if (savedNote && savedNote.category_id) {
    updateNotebookTreeCount(savedNote.category_id, 1);
  }

  if (savedNote) {
    console.log('[MainLayout] 新笔记创建成功:', {
      noteId: savedNote.id,
      categoryId: savedNote.category_id,
      selectedNotebookId: selectedNotebookId.value,
      selectedTags: selectedTags.value
    })
    
    // 根据当前视图状态刷新数据
    if (selectedNotebookId.value) {
      // 选中了笔记本的情况
      if (savedNote.category_id === selectedNotebookId.value) {
        // 新笔记在当前选中的笔记本中，刷新分类浏览数据
        console.log('[MainLayout] 新笔记在当前笔记本中，刷新分类浏览数据')
        await tipsStore.browseCategory(selectedNotebookId.value)
      } else {
        // 新笔记不在当前选中的笔记本中，切换到新笔记所在的笔记本
        console.log('[MainLayout] 新笔记在其他笔记本中，切换到目标笔记本')
        if (savedNote.category_id) {
          selectedNotebookId.value = savedNote.category_id
          selectedTags.value = []
          await tipsStore.browseCategory(savedNote.category_id)
        }
      }
    } else if (selectedTags.value.length > 0) {
      // 选中了标签的情况，刷新标签视图
      console.log('[MainLayout] 当前选中标签，刷新标签视图')
      await tipsStore.fetchTipsByTag(selectedTags.value.join(','))
    } else {
      // 全部笔记视图，刷新全部笔记
      console.log('[MainLayout] 全部笔记视图，刷新笔记列表')
      await tipsStore.fetchAllTipSummaries()
    }
    
    await handleNoteSelection(savedNote);
  } else {
    console.error("Failed to create new note.");
    showAlert(t('mainLayout.errorCreateNote'), { title: t('common.error') });
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
  const confirmed = await showConfirm(t('mainLayout.confirmDeleteNote'), { title: t('mainLayout.confirmDeleteTitle') });
  if (confirmed) {
    try {
      // 保存要删除笔记的分类ID，用于后续更新UI
      const noteToDelete = storeTips.value.find(note => note.id === id);
      const categoryId = noteToDelete?.category_id;
      
      // 调用后端API删除笔记
      await tipsStore.deleteTip(id);
      

      
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

  // 如果当前在笔记本视图下，同时更新currentCategoryBrowse中的数据
  if (selectedNotebookId.value && tipsStore.currentCategoryBrowse) {
    const categoryBrowse = tipsStore.currentCategoryBrowse
    
    // 更新featured_tip（如果是特色笔记）
    if (categoryBrowse.featured_tip && categoryBrowse.featured_tip.id === savedNote.id) {
      // 使用Object.assign确保响应式更新
      Object.assign(categoryBrowse.featured_tip, {
        title: savedNote.title,
        content: savedNote.content,
        updated_at: savedNote.updated_at,
        tags: savedNote.tags
      })
    }
    
    // 更新tip_summaries中的对应项
    const summaryIdx = categoryBrowse.tip_summaries.findIndex(t => t.id === savedNote.id)
    if (summaryIdx !== -1) {
      // 直接替换数组元素以确保响应式更新
      categoryBrowse.tip_summaries.splice(summaryIdx, 1, tipSummary)
    }
    
    // 强制触发响应式更新（Windows兼容性）
    tipsStore.currentCategoryBrowse = { ...categoryBrowse }
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

function addChildNotebook({ parentId }: { parentId: string }) {
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

function deleteTag(_tagId: string) {
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
    e.stopPropagation();

    const container = document.querySelector(containerSelector) as HTMLElement;
    if (!container) {
      console.warn(`Container not found: ${containerSelector}`);
      return;
    }

    const startX = e.clientX;
    const startWidth = widthRef.value;
    let isDragging = true;

    // 创建一个全局覆盖层
    const overlay = document.createElement('div');
    overlay.style.position = 'fixed';
    overlay.style.top = '0';
    overlay.style.left = '0';
    overlay.style.width = '100vw';
    overlay.style.height = '100vh';
    overlay.style.cursor = 'col-resize';
    overlay.style.zIndex = '9999'; // 确保在最上层
    document.body.appendChild(overlay);

    document.body.classList.add('dragging');
    container.classList.add('resizing');

    const onMouseMove = (moveEvent: MouseEvent) => {
      if (!isDragging) return;

      moveEvent.preventDefault();
      moveEvent.stopPropagation();

      const deltaX = moveEvent.clientX - startX;
      let newWidth = startWidth + deltaX;
      newWidth = Math.max(minWidth, Math.min(newWidth, maxWidth));
      widthRef.value = newWidth;
    };

    const onMouseUp = (upEvent?: Event) => {
      if (!isDragging) return;
      isDragging = false;

      if (upEvent) {
        upEvent.preventDefault();
        upEvent.stopPropagation();
      }

      // 移除覆盖层和事件监听器
      document.body.removeChild(overlay);
      overlay.removeEventListener('mousemove', onMouseMove, { capture: true });
      overlay.removeEventListener('mouseup', onMouseUp as EventListener, { capture: true });
      overlay.removeEventListener('mouseleave', onMouseUp as EventListener, { capture: true });
      window.removeEventListener('blur', onMouseUp as EventListener);

      if (storageKey === 'sidebarWidth') {
        localStorageStore.setSidebarWidth(widthRef.value);
      } else if (storageKey === 'noteListWidth') {
        localStorageStore.setNoteListWidth(widthRef.value);
      }

      document.body.classList.remove('dragging');
      container.classList.remove('resizing');

      container.classList.add('resize-complete');
      setTimeout(() => {
        container.classList.remove('resize-complete');
      }, 200);
    };

    // 在覆盖层上监听事件
    overlay.addEventListener('mousemove', onMouseMove, { capture: true, passive: false });
    overlay.addEventListener('mouseup', onMouseUp as EventListener, { capture: true, passive: false });
    overlay.addEventListener('mouseleave', onMouseUp as EventListener, { capture: true, passive: false });
    window.addEventListener('blur', onMouseUp as EventListener, { once: true });
  };
}

const startResizeSidebar = createResizeHandler('.sidebar-container', sidebarWidth, 'sidebarWidth', 180, 500);
const startResizeNoteList = createResizeHandler('.note-list-container', noteListWidth, 'noteListWidth', 240, 600);

function toggleNoteList() {
  noteListHidden.value = !noteListHidden.value;
  localStorageStore.setNoteListHidden(noteListHidden.value);
}

async function selectNotebook(id: string) {
  selectedNotebookId.value = id
  selectedTags.value = [] // clear tag selection
  
  // 如果不在搜索状态，更新保存的状态
  if (!navSearchQuery.value.trim()) {
    lastSelectedState.value = {
      notebookId: id,
      tags: []
    }
  }
  
  // 使用新的分类浏览API获取优化的数据
  const categoryData = await tipsStore.browseCategory(id)

  if (isMobile.value) {
    // On mobile, just display the list of notes for the selected notebook.
    // So we clear the selected note.
    selectedNote.value = null
    selectedNoteId.value = null
  } else {
    // On desktop, auto-select the first note.
    // 优先使用featured_tip，如果没有则使用第一个摘要
    if (categoryData?.featured_tip) {
      selectNote(categoryData.featured_tip)
    } else if (categoryData?.tip_summaries && categoryData.tip_summaries.length > 0) {
      // 如果只有摘要，需要获取完整内容
      const firstTipId = categoryData.tip_summaries[0].id
      const note = await tipsStore.fetchTip(firstTipId)
      if (note) {
        selectNote(note)
      }
    } else {
      // If the notebook is empty, clear the current selection.
      selectedNote.value = null
      selectedNoteId.value = null
    }
  }
}
function toggleTag(id: string) {
  // If the clicked tag is already selected, unselect it.
  // Otherwise, select the clicked tag.
  if (selectedTags.value[0] === id) {
    selectedTags.value = [];
  } else {
    selectedTags.value = [id];
  }
  
  selectedNotebookId.value = null; // clear notebook selection
  
  // 如果不在搜索状态，更新保存的状态
  if (!navSearchQuery.value.trim()) {
    lastSelectedState.value = {
      notebookId: null,
      tags: [...selectedTags.value]
    }
  }
  
  if (selectedTags.value.length > 0) {
    tipsStore.fetchTipsByTag(selectedTags.value.join(','))
  } else {
    tipsStore.fetchTips(true);
  }
}

async function exportNote(noteId: string, format: string) {
  try {
    console.log(`Starting export for note: ${noteId}, format: ${format}`)
    
    // 根据格式调用不同的导出函数
    let result: string
    
    switch (format.toLowerCase()) {
      case 'markdown':
      case 'md':
        result = await invoke('export_as_markdown', { noteIds: [noteId] })
        break
      case 'html':
        result = await invoke('export_as_html', { noteIds: [noteId] })
        break
      case 'pdf':
        result = await invoke('export_as_pdf', { noteIds: [noteId] })
        break
      default:
        throw new Error(`Unsupported export format: ${format}`)
    }
    
    console.log('Export successful:', result)
    // 可以显示成功提示
    await showAlert(result, { title: t('mainLayout.exportSuccess') })
    
  } catch (error) {
    console.error('Failed to export note:', error)
    let errorMessage = t('mainLayout.exportFailed')
    
    if (typeof error === 'string') {
      errorMessage = error
    } else if (error && typeof error === 'object' && 'message' in error) {
      errorMessage = (error as any).message
    }
    
    await showAlert(errorMessage, { title: t('mainLayout.exportFailedTitle') })
  }
}
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
function handleNoteEncryption(_id: string) {}
function handleNoteDecryption(_id: string) {}
function duplicateNote(_id: string) {}
function addTagToNote(_tag: string) {}
function removeTagFromNote(_tagId: string) {}
function toggleNotePin(_id: string) {}
function handleNoteUnlock(_id: string) {}

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

function handleImportSuccess(_importResult: any) {
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
  /* Windows兼容性增强 */
  touch-action: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.resize-handle:hover {
  background-color: rgba(var(--primary), 0.3);
  width: 6px;
}

.resize-handle:active {
  background-color: rgba(var(--primary), 0.5);
  width: 6px;
}

/* Windows下增强拖拽手柄的可见性 */
.sidebar-resize-handle,
.notelist-resize-handle {
  /* 在Windows下提供更明显的视觉提示 */
  border-right: 1px solid transparent;
}

.sidebar-resize-handle:hover,
.notelist-resize-handle:hover {
  border-right-color: rgba(var(--primary), 0.4);
  background: linear-gradient(90deg, transparent, rgba(var(--primary), 0.1));
}

/* Windows高DPI显示器优化 */
@media (-webkit-min-device-pixel-ratio: 1.5), (min-resolution: 144dpi) {
  .resize-handle {
    width: 6px;
  }
  
  .resize-handle:hover {
    width: 8px;
  }
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
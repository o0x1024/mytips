<template>
  <div class="h-full flex flex-col">
    <!-- 顶部导航 -->
    <div class="p-2 border-b border-base-300 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <button class="btn btn-ghost btn-sm" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <h1 class="text-lg font-medium">{{ isNewTip ? '创建新笔记' : '编辑笔记' }}</h1>
      </div>
      <div class="flex gap-2">
        <button class="btn btn-error btn-sm" v-if="!isNewTip" @click="confirmDelete">删除</button>
        <button class="btn btn-primary btn-sm" @click="saveTip" :disabled="isLoading">
          <span v-if="isLoading" class="loading loading-spinner loading-xs"></span>
          保存
        </button>
      </div>
    </div>

    <!-- 主编辑器区域 -->
    <NoteEditor 
      v-if="tip" 
      :note="tip" 
      @update="handleTipUpdate" 
      @delete-note="confirmDelete"
    />

    <!-- 底部笔记本选择栏 -->
    <div class="mt-auto border-t border-base-200 bg-base-100 p-3 flex items-center gap-2">
      <span class="text-sm font-medium">笔记本：</span>
      <div class="flex-1">
        <select class="select select-bordered select-sm w-full max-w-xs" v-model="tip.category_id">
          <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.name }}</option>
        </select>
      </div>
      <button class="btn btn-sm btn-outline" @click="showNewCategoryModal = true">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        新建
      </button>
    </div>

    <!-- 新建分类对话框 -->
    <dialog id="new_category_modal" class="modal" :open="showNewCategoryModal">
      <div class="modal-box">
        <h3 class="font-bold text-lg">创建新笔记本</h3>
        <div class="py-4">
          <input 
            type="text" 
            placeholder="笔记本名称" 
            class="input input-bordered w-full" 
            v-model="newCategory"
            @keyup.enter="createCategory"
          />
        </div>
        <div class="modal-action">
          <button class="btn" @click="showNewCategoryModal = false">取消</button>
          <button class="btn btn-primary" @click="createCategory" :disabled="!newCategory.trim()">
            创建
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop" @click="showNewCategoryModal = false">
        <button>关闭</button>
      </form>
    </dialog>
    
    <!-- 删除确认对话框 -->
    <dialog id="delete_confirm_modal" class="modal" :open="showDeleteConfirm">
      <div class="modal-box">
        <h3 class="font-bold text-lg">确认删除</h3>
        <p class="py-4">确定要删除"{{ tip.title }}"吗？此操作不可撤销。</p>
        <div class="modal-action">
          <button class="btn" @click="showDeleteConfirm = false">取消</button>
          <button class="btn btn-error" @click="deleteTip">
            <span v-if="isDeleting" class="loading loading-spinner loading-xs"></span>
            删除
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop" @click="showDeleteConfirm = false">
        <button>关闭</button>
      </form>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useTipsStore, type TipData } from '../stores/tipsStore'
import NoteEditor from '../components/NoteEditor.vue'
import { showAlert } from '../services/dialog'

const route = useRoute()
const router = useRouter()
const tipsStore = useTipsStore()

// 判断是新建还是编辑
const tipId = computed(() => route.params.id as string | undefined)
const isNewTip = computed(() => !tipId.value)

// 对话框状态
const showNewCategoryModal = ref(false)
const showDeleteConfirm = ref(false)
const isDeleting = ref(false)

// 新建分类
const newCategory = ref('')

// 加载状态
const isLoading = computed(() => tipsStore.isLoading)

// 所有分类
const categories = computed(() => tipsStore.categories)

// 笔记数据 - 初始化为空笔记
const tip = ref({
  id: '',
  title: '',
  content: '',
  tip_type: 'markdown', // 默认都是markdown格式
  language: undefined as string | undefined, 
  category_id: undefined as string | undefined,
  created_at: Date.now(),
  updated_at: Date.now(),
  tags: [] as Array<{ id: string; name: string }>
})

// 加载数据
onMounted(async () => {
  // 加载分类和标签
  await Promise.all([
    tipsStore.fetchAllCategories(),
    tipsStore.fetchAllTags()
  ])
  
  // 如果是从临时笔记区创建的笔记，优先设置为"未分类"笔记本
  if (route.query.from === 'clipboard') {
    // 查找"未分类"笔记本
    const uncategorizedCategory = categories.value.find(cat => cat.name === '未分类')
    if (uncategorizedCategory) {
      tip.value.category_id = uncategorizedCategory.id
    } else {
      // 如果没有找到"未分类"笔记本，创建一个
      try {
        const newCategory = await tipsStore.createCategory('未分类')
        if (newCategory) {
          tip.value.category_id = newCategory.id
        }
      } catch (error) {
        console.error('创建未分类笔记本失败:', error)
        // 如果创建失败，使用第一个可用的分类
        if (categories.value.length > 0) {
          tip.value.category_id = categories.value[0].id
        }
      }
    }
  } else {
    // 设置默认分类（如果有）
    if (categories.value.length > 0 && !tip.value.category_id) {
      tip.value.category_id = categories.value[0].id
    }
  }
  
  // 如果是编辑模式，加载笔记数据
  if (tipId.value) {
    try {
      const loadedTip = await tipsStore.fetchTip(tipId.value)
      if (loadedTip) {
        tip.value = {
          id: loadedTip.id,
          title: loadedTip.title || '',
          content: loadedTip.content || '',
          tip_type: loadedTip.tip_type || 'markdown',
          language: loadedTip.language,
          category_id: loadedTip.category_id,
          created_at: loadedTip.created_at || Date.now(),
          updated_at: loadedTip.updated_at || Date.now(),
          tags: loadedTip.tags || []
        }
      }
    } catch (error) {
      console.error('加载笔记失败:', error)
    }
  }
  // 如果是新建笔记且没有ID，保持默认的空笔记状态
})

// 创建新分类
const createCategory = async () => {
  if (!newCategory.value.trim()) return
  
  const createdCategory = await tipsStore.createCategory(newCategory.value.trim())
  if (createdCategory) {
    tip.value.category_id = createdCategory.id
    showNewCategoryModal.value = false
    newCategory.value = ''
  }
}

// 处理笔记更新
const handleTipUpdate = (updatedNote: any) => {
  // 更新本地状态
  if (updatedNote._titleOnly) {
    // 仅更新标题
    tip.value.title = updatedNote.title
    tip.value.updated_at = Date.now()
  } else if (updatedNote._contentOnly) {
    // 仅更新内容
    tip.value.content = updatedNote.content
    tip.value.updated_at = Date.now()
  } else {
    // 完整更新
    Object.assign(tip.value, updatedNote)
  }
}

// 保存笔记
const saveTip = async () => {
  if (!tip.value.title.trim()) {
    await showAlert('请输入标题', { title: '提示' })
    return
  }
  
  // 明确指定每个字段的类型
  const tipData: TipData = {
    id: tip.value.id,
    title: tip.value.title,
    content: tip.value.content,
    tip_type: tip.value.tip_type,
    language: tip.value.language,
    category_id: tip.value.category_id,
    tags: tip.value.tags.map(tag => tag.id)
  }
  
  try {
    const savedTip = await tipsStore.saveTip(tipData)
    if (savedTip) {
      router.back()
    }
  } catch (error) {
    console.error('保存笔记失败:', error)
    await showAlert('保存失败: ' + error, { title: '错误' })
  }
}

// 确认删除
const confirmDelete = () => {
  showDeleteConfirm.value = true
}

// 删除笔记
const deleteTip = async () => {
  if (!tip.value.id) return
  
  isDeleting.value = true
  try {
    const success = await tipsStore.deleteTip(tip.value.id)
    isDeleting.value = false
    
    if (success) {
      showDeleteConfirm.value = false
      router.back()
    }
  } catch (error) {
    isDeleting.value = false
    console.error('删除笔记失败:', error)
    await showAlert('删除失败: ' + error, { title: '错误' })
  }
}

// 返回
const goBack = () => {
  router.back()
}
</script>

<style scoped>
/* 编辑器容器样式 */
.h-full {
  height: 100vh;
}
</style> 
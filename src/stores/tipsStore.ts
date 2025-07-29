import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 数据类型定义
export interface Tag {
  id: string
  name: string
}

export interface Category {
  id: string
  name: string
  parent_id?: string
}

export interface Tip {
  id: string
  title: string
  content: string
  tip_type: string
  language?: string
  category_id?: string
  created_at: number
  updated_at: number
  tags: Tag[]
  images?: Record<string, string>
  is_encrypted?: boolean
}

export interface TipSummary {
  id: string
  title: string
  tip_type: string
  language?: string
  category_id?: string
  created_at: number
  updated_at: number
  tags: Tag[]
  is_encrypted: boolean
  // 完整内容（可选）
  content?: string
}

export interface TipData {
  id?: string
  title: string
  content: string
  tip_type: string
  language?: string
  category_id?: string
  tags: string[]
}

// 新增：分类浏览响应数据结构
export interface CategoryBrowseResponse {
  current_category?: Category
  subcategories: Category[]
  featured_tip?: Tip
  tip_summaries: TipSummary[]
  total_tips_count: number
}

// 定义存储
export const useTipsStore = defineStore('tips', () => {
  // 状态
  const tips = ref<TipSummary[]>([])
  const categories = ref<Category[]>([])
  const tags = ref<Tag[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const currentPage = ref(1)
  const hasMore = ref(true)
  const pageSize = 20 // 每页加载的数量

  // 新增：分类浏览相关状态
  const currentCategoryBrowse = ref<CategoryBrowseResponse | null>(null)
  const isLoadingCategory = ref(false)
  const categoryError = ref<string | null>(null)
  const categoryTipsOffset = ref(0) // 分类笔记加载偏移量
  const categoryTipsLimit = ref(20) // 分类笔记每次加载数量

  // Getters
  const tipsByCategory = computed(() => (categoryId: string) => {
    return tips.value.filter(tip => tip.category_id === categoryId)
  })

  const tipsByTag = computed(() => (tagId: string) => {
    return tips.value.filter(tip => tip.tags.some(tag => tag.id === tagId))
  })

  const getTagById = computed(() => (tagId: string) => {
    return tags.value.find(tag => tag.id === tagId)
  })

  const getCategoryById = computed(() => (categoryId: string) => {
    return categories.value.find(category => category.id === categoryId)
  })

  // Actions
  // 分页加载笔记
  async function fetchTips(reset = false) {
    if (isLoading.value || !hasMore.value) return
    if (reset) {
      currentPage.value = 1
      tips.value = []
      hasMore.value = true
    }

    try {
      isLoading.value = true
      error.value = null
      const result = await invoke<TipSummary[]>('get_tips_paged', {
        page: currentPage.value,
        pageSize: pageSize,
      })
      if (result.length > 0) {
        tips.value.push(...result)
        currentPage.value++
      } else {
        hasMore.value = false
      }
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 获取笔记失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 新增：一次性加载所有笔记的摘要信息
  async function fetchAllTipSummaries(): Promise<TipSummary[]> {
    if (isLoading.value) return tips.value;
    try {
      isLoading.value = true;
      error.value = null;
      const result = await invoke<TipSummary[]>('get_all_tip_summaries');
      tips.value = result;
      hasMore.value = false; // 表示已加载全部
      return result;
    } catch (err) {
      error.value = (err as Error).message;
      console.error('[TipsStore] 获取所有笔记摘要失败:', err);
      return []; // 出错时返回空数组，避免使用旧数据
    } finally {
      isLoading.value = false;
    }
  }

  // 一次性加载全部笔记（含完整内容）
  async function fetchAllTips() {
    try {
      isLoading.value = true
      error.value = null

      const result = await invoke<Tip[]>('get_all_tips')

      // 将完整 Tip 转换为 TipSummary，并保留 content 字段
      tips.value = result.map(tip => ({
        id: tip.id,
        title: tip.title,
        tip_type: tip.tip_type,
        language: tip.language,
        category_id: tip.category_id,
        created_at: tip.created_at,
        updated_at: tip.updated_at,
        tags: tip.tags,
        is_encrypted: tip.is_encrypted || false,
        content: tip.content
      }))

      // 加载全部后不再分页
      hasMore.value = false
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 获取全部笔记失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 加载单个笔记的完整内容
  async function fetchTipContent(id: string): Promise<string | null> {
    try {
      const content = await invoke<string>('get_tip_content', { id })
      return content
    } catch (err) {
      console.error(`[TipsStore] 获取笔记内容失败 for id ${id}:`, err)
      return null
    }
  }

  // 加载单个笔记
  async function fetchTip(id: string) {
    try {
      isLoading.value = true
      error.value = null
      const tip = await invoke<Tip>('get_tip', { id })
      
      // 转换为 TipSummary
      const tipSummary: TipSummary = {
        id: tip.id,
        title: tip.title,
        tip_type: tip.tip_type,
        language: tip.language,
        category_id: tip.category_id,
        created_at: tip.created_at,
        updated_at: tip.updated_at,
        tags: tip.tags,
        is_encrypted: tip.is_encrypted || false,
        content: tip.content,
      };

      // 更新本地存储中的笔记
      const index = tips.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tips.value[index] = tipSummary;
      } else {
        tips.value.push(tipSummary);
      }
      
      return tip
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to fetch tip with id ${id}:`, err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 保存笔记
  async function saveTip(tipData: TipData) {
    try {
      isLoading.value = true
      error.value = null
      
      const savedTip = await invoke<Tip>('save_tip', { tipData })
      
      // 转换为 TipSummary
      const tipSummary: TipSummary = {
        id: savedTip.id,
        title: savedTip.title,
        tip_type: savedTip.tip_type,
        language: savedTip.language,
        category_id: savedTip.category_id,
        created_at: savedTip.created_at,
        updated_at: savedTip.updated_at,
        tags: savedTip.tags,
        is_encrypted: savedTip.is_encrypted || false,
        content: savedTip.content,
      };

      // 更新本地存储
      const index = tips.value.findIndex(tip => tip.id === savedTip.id)
      if (index !== -1) {
        tips.value[index] = tipSummary;
      } else {
        tips.value.unshift(tipSummary); // 新笔记添加到最前面
      }
      
      // await fetchAllTipSummaries(); // 刷新列表
      return savedTip
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to save tip:', err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 删除笔记
  async function deleteTip(tip_id: string) {
    try {
      isLoading.value = true
      error.value = null
      
      await invoke('delete_tip', { tip_id })
      
      // 从本地存储中移除
      tips.value = tips.value.filter(tip => tip.id !== tip_id)
      
      return true
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to delete tip with id ${tip_id}:`, err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 加载所有分类
  async function fetchAllCategories() {
    try {
      isLoading.value = true
      error.value = null
      
      const result = await invoke<Category[]>('get_all_categories')
      
      if (!result || !Array.isArray(result)) {
        console.error('获取到的分类数据无效:', result)
        return
      }
      
      categories.value = result
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to fetch categories:', err)
      // 添加错误处理时的兜底数据
      if (categories.value.length === 0) {
        categories.value = [
          { id: 'default', name: '默认笔记本' }
        ]
        console.log('使用默认分类数据:', categories.value)
      }
    } finally {
      isLoading.value = false
    }
  }

  // 创建分类
  async function createCategory(name: string, parent_id?: string) {
    console.log('创建分类:333', name, parent_id)
    try {
      isLoading.value = true
      error.value = null
      
      const newCategory = await invoke<Category>('create_category', { name, parent_id })
      categories.value.push(newCategory)
      
      return newCategory
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to create category:', err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 更新分类
  async function updateCategory(id: string, name: string) {
    try {
      isLoading.value = true
      error.value = null
      
      const updatedCategory = await invoke<Category>('update_category', { id, name })
      
      // 更新本地存储
      const index = categories.value.findIndex(cat => cat.id === id)
      if (index !== -1) {
        categories.value[index] = updatedCategory
      }
      
      return updatedCategory
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to update category:', err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 删除分类
  async function deleteCategory(id: string) {
    try {
      isLoading.value = true
      error.value = null
      
      await invoke('delete_category', { id })
      
      // 从本地存储中移除
      categories.value = categories.value.filter(cat => cat.id !== id)
      
      return true
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to delete category with id ${id}:`, err)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 加载所有标签
  async function fetchAllTags() {
    try {
      isLoading.value = true
      error.value = null
      
      tags.value = await invoke<Tag[]>('get_all_tags')
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to fetch tags:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 创建标签
  async function createTag(name: string) {
    try {
      isLoading.value = true
      error.value = null
      
      const newTag = await invoke<Tag>('create_tag', { name })
      tags.value.push(newTag)
      
      return newTag
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to create tag:', err)
      return null
    } finally {
      isLoading.value = false
    }
  }

  // 搜索笔记 - 优化版本
  async function searchTips(query: string) {
    try {
      isLoading.value = true
      error.value = null
      
      // 清除分类浏览状态，确保搜索结果不被覆盖
      currentCategoryBrowse.value = null
      
      // 使用优化的搜索摘要API，速度更快
      const result = await invoke<TipSummary[]>('search_tips_summary', { query })
      
      // 直接使用返回的TipSummary数组
      tips.value = result

      hasMore.value = false; // 搜索结果不分页
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 搜索笔记失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 新的分类浏览API - 优化性能
  async function browseCategory(categoryId: string = '') {
    try {
      isLoadingCategory.value = true
      categoryError.value = null
      
      const result = await invoke<CategoryBrowseResponse>('browse_category', { categoryId })
      currentCategoryBrowse.value = result
      
      // 重置分页偏移量
      categoryTipsOffset.value = result.tip_summaries.length + (result.featured_tip ? 1 : 0)
      
      return result
    } catch (err) {
      categoryError.value = (err as Error).message
      console.error(`[TipsStore] 浏览分类失败 for category ${categoryId}:`, err)
      return null
    } finally {
      isLoadingCategory.value = false
    }
  }

  // 加载分类下的更多笔记
  async function loadMoreCategoryTips(categoryId: string) {
    if (!currentCategoryBrowse.value) return []
    
    try {
      isLoadingCategory.value = true
      categoryError.value = null
      
      const result = await invoke<TipSummary[]>('load_more_tips_by_category', {
        categoryId,
        offset: categoryTipsOffset.value,
        limit: categoryTipsLimit.value
      })
      
      // 将新笔记添加到当前分类浏览数据中
      if (result.length > 0) {
        currentCategoryBrowse.value.tip_summaries.push(...result)
        categoryTipsOffset.value += result.length
      }
      
      return result
    } catch (err) {
      categoryError.value = (err as Error).message
      console.error(`[TipsStore] 加载更多分类笔记失败 for category ${categoryId}:`, err)
      return []
    } finally {
      isLoadingCategory.value = false
    }
  }

  // 按分类加载笔记（保留向后兼容，但建议使用browseCategory）
  async function fetchTipsByCategory(categoryId: string) {
    try {
      isLoading.value = true
      error.value = null
      // 调用递归接口，获取目标分类及其所有子分类下的笔记
      const result = await invoke<Tip[]>('get_tips_by_category_recursive', { categoryId })
      
      // 转换
      tips.value = result.map(tip => ({
        id: tip.id,
        title: tip.title,
        tip_type: tip.tip_type,
        language: tip.language,
        category_id: tip.category_id,
        created_at: tip.created_at,
        updated_at: tip.updated_at,
        tags: tip.tags,
        is_encrypted: tip.is_encrypted || false,
        content: tip.content
      }));

      hasMore.value = false; // 不分页
    } catch (err) {
      error.value = (err as Error).message
      console.error(`[TipsStore] 按分类获取笔记失败 for category ${categoryId}:`, err)
    } finally {
      isLoading.value = false
    }
  }

  // 按标签加载笔记
  async function fetchTipsByTag(tagId: string) {
    try {
      isLoading.value = true
      error.value = null
      const result = await invoke<Tip[]>('get_tips_by_tag', { tagId })

      // 转换
      tips.value = result.map(tip => ({
        id: tip.id,
        title: tip.title,
        tip_type: tip.tip_type,
        language: tip.language,
        category_id: tip.category_id,
        created_at: tip.created_at,
        updated_at: tip.updated_at,
        tags: tip.tags,
        is_encrypted: tip.is_encrypted || false,
        content: tip.content
      }));

      hasMore.value = false; // 不分页
    } catch (err) {
      error.value = (err as Error).message
      console.error(`[TipsStore] 按标签获取笔记失败 for tag ${tagId}:`, err)
    } finally {
      isLoading.value = false
    }
  }

  // 初始化加载数据
  async function initializeData() {
    try {
      isLoading.value = true
      error.value = null
      
      // 分步加载，确保每一步都成功
      await fetchAllCategories()
      await fetchAllTags()
      await fetchTips()
      
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 初始化数据失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 新增：在列表中更新单个笔记的摘要信息
  function updateTipInList(updatedTip: Partial<TipSummary>) {
    if (!updatedTip.id) return;

    const index = tips.value.findIndex(t => t.id === updatedTip.id)
    if (index !== -1) {
      // 合并更新，只更新传入的字段
      tips.value[index] = { ...tips.value[index], ...updatedTip };
      
      // 手动触发响应式更新
      tips.value = [...tips.value];
    }
  }

  return {
    // 状态
    tips,
    categories,
    tags,
    isLoading,
    error,
    currentPage,
    hasMore,
    pageSize,
    
    // 新增：分类浏览相关状态
    currentCategoryBrowse,
    isLoadingCategory,
    categoryError,
    categoryTipsOffset,
    categoryTipsLimit,
    
    // Getters
    tipsByCategory,
    tipsByTag,
    getTagById,
    getCategoryById,
    
    // Actions
    fetchTips,
    fetchTip,
    saveTip,
    deleteTip,
    fetchAllCategories,
    createCategory,
    updateCategory,
    deleteCategory,
    fetchAllTags,
    createTag,
    searchTips,
    fetchTipsByCategory,
    fetchTipsByTag,
    initializeData,
    fetchTipContent,
    fetchAllTips,
    fetchAllTipSummaries,
    
    // 新增：分类浏览相关方法
    browseCategory,
    loadMoreCategoryTips,

    // 新增：更新列表中的笔记
    updateTipInList
  }
}) 
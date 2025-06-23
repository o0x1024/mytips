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

// 定义存储
export const useTipsStore = defineStore('tips', () => {
  // 状态
  const tips = ref<Tip[]>([])
  const categories = ref<Category[]>([])
  const tags = ref<Tag[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

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
  // 加载所有笔记
  async function fetchAllTips() {
    try {
      isLoading.value = true
      error.value = null
      const result = await invoke<Tip[]>('get_all_tips')
      tips.value = result || []
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 获取笔记失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  // 加载单个笔记
  async function fetchTip(id: string) {
    try {
      isLoading.value = true
      error.value = null
      const tip = await invoke<Tip>('get_tip', { id })
      
      // 更新本地存储中的笔记
      const index = tips.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tips.value[index] = tip
      } else {
        tips.value.push(tip)
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
      
      // 更新本地存储
      const index = tips.value.findIndex(tip => tip.id === savedTip.id)
      if (index !== -1) {
        tips.value[index] = savedTip
      } else {
        tips.value.unshift(savedTip) // 新笔记添加到最前面
      }
      
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
  async function deleteTip(id: string) {
    try {
      isLoading.value = true
      error.value = null
      
      await invoke('delete_tip', { id })
      
      // 从本地存储中移除
      tips.value = tips.value.filter(tip => tip.id !== id)
      
      return true
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to delete tip with id ${id}:`, err)
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

  // 搜索笔记
  async function searchTips(query: string) {
    try {
      isLoading.value = true
      error.value = null
      
      return await invoke<Tip[]>('search_tips', { query })
    } catch (err) {
      error.value = (err as Error).message
      console.error('Failed to search tips:', err)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // 按分类获取笔记
  async function fetchTipsByCategory(categoryId: string) {
    try {
      isLoading.value = true
      error.value = null
      
      return await invoke<Tip[]>('get_tips_by_category', { categoryId })
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to fetch tips for category ${categoryId}:`, err)
      return []
    } finally {
      isLoading.value = false
    }
  }

  // 按标签获取笔记
  async function fetchTipsByTag(tagId: string) {
    try {
      isLoading.value = true
      error.value = null
      
      return await invoke<Tip[]>('get_tips_by_tag', { tagId })
    } catch (err) {
      error.value = (err as Error).message
      console.error(`Failed to fetch tips for tag ${tagId}:`, err)
      return []
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
      await fetchAllTips()
      
    } catch (err) {
      error.value = (err as Error).message
      console.error('[TipsStore] 初始化数据失败:', err)
    } finally {
      isLoading.value = false
    }
  }

  return {
    // 状态
    tips,
    categories,
    tags,
    isLoading,
    error,
    
    // Getters
    tipsByCategory,
    tipsByTag,
    getTagById,
    getCategoryById,
    
    // Actions
    fetchAllTips,
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
    initializeData
  }
}) 
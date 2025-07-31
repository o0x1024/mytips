import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

// 定义本地存储的数据结构
interface LocalStorageData {
  // 用户设置
  userLanguage: string
  
  // 标签使用统计
  tagUsage: Record<string, number>
  
  // AI助手相关
  needRefreshTips: boolean
  aiShowNotePanel: boolean
  aiNoteTitle: string
  aiNoteContent: string
  aiSelectedModel: string
  
  // 编辑器设置
  editorMode: string
  highlightTheme: string
  markdownTheme: string
  highlightThemeManual: boolean
  
  // 布局设置
  sidebarWidth: number
  noteListWidth: number
  noteListHidden: boolean
  
  // UI设置
  uiSettings: Record<string, any>
}

// 默认值
const defaultData: LocalStorageData = {
  userLanguage: 'zh',
  tagUsage: {},
  needRefreshTips: false,
  aiShowNotePanel: false,
  aiNoteTitle: '',
  aiNoteContent: '',
  aiSelectedModel: '',
  editorMode: 'split',
  highlightTheme: '',
  markdownTheme: 'github',
  highlightThemeManual: false,
  sidebarWidth: 280,
  noteListWidth: 320,
  noteListHidden: false,
  uiSettings: {}
}

export const useLocalStorageStore = defineStore('localStorage', () => {
  // 状态
  const data = ref<LocalStorageData>({ ...defaultData })
  
  // 初始化：从 localStorage 加载数据
  function loadFromLocalStorage() {
    try {
      // 用户语言
      const userLanguage = localStorage.getItem('user-language')
      if (userLanguage) data.value.userLanguage = userLanguage
      
      // 标签使用统计
      const tagUsageStr = localStorage.getItem('mytips-tag-usage')
      if (tagUsageStr) {
        try {
          data.value.tagUsage = JSON.parse(tagUsageStr)
        } catch (e) {
          console.warn('Failed to parse tag usage data:', e)
        }
      }
      
      // AI助手相关
      const needRefreshTips = localStorage.getItem('need-refresh-tips')
      if (needRefreshTips) data.value.needRefreshTips = needRefreshTips === 'true'
      
      const aiShowNotePanel = localStorage.getItem('ai-show-note-panel')
      if (aiShowNotePanel) data.value.aiShowNotePanel = aiShowNotePanel === 'true'
      
      const aiNoteTitle = localStorage.getItem('ai-note-title')
      if (aiNoteTitle) data.value.aiNoteTitle = aiNoteTitle
      
      const aiNoteContent = localStorage.getItem('ai-note-content')
      if (aiNoteContent) data.value.aiNoteContent = aiNoteContent
      
      const aiSelectedModel = localStorage.getItem('ai-selected-model')
      if (aiSelectedModel) data.value.aiSelectedModel = aiSelectedModel
      
      // 编辑器设置
      const editorMode = localStorage.getItem('mytips-editor-mode')
      if (editorMode) data.value.editorMode = editorMode
      
      const highlightTheme = localStorage.getItem('mytips-highlight-theme')
      if (highlightTheme) data.value.highlightTheme = highlightTheme
      
      const markdownTheme = localStorage.getItem('mytips-markdown-theme')
      if (markdownTheme) data.value.markdownTheme = markdownTheme
      
      const highlightThemeManual = localStorage.getItem('mytips-highlight-theme-manual')
      if (highlightThemeManual) data.value.highlightThemeManual = highlightThemeManual === 'true'
      
      // 布局设置
      const sidebarWidth = localStorage.getItem('sidebarWidth')
      if (sidebarWidth) data.value.sidebarWidth = parseInt(sidebarWidth, 10)
      
      const noteListWidth = localStorage.getItem('noteListWidth')
      if (noteListWidth) data.value.noteListWidth = parseInt(noteListWidth, 10)
      
      const noteListHidden = localStorage.getItem('noteListHidden')
      if (noteListHidden) data.value.noteListHidden = noteListHidden === 'true'
      
      // UI设置
      const uiSettings = localStorage.getItem('mytips-ui-settings')
      if (uiSettings) {
        try {
          data.value.uiSettings = JSON.parse(uiSettings)
        } catch (e) {
          console.warn('Failed to parse UI settings:', e)
        }
      }
    } catch (error) {
      console.error('Failed to load data from localStorage:', error)
    }
  }
  
  // 保存到 localStorage
  function saveToLocalStorage() {
    try {
      localStorage.setItem('user-language', data.value.userLanguage)
      localStorage.setItem('mytips-tag-usage', JSON.stringify(data.value.tagUsage))
      localStorage.setItem('need-refresh-tips', data.value.needRefreshTips.toString())
      localStorage.setItem('ai-show-note-panel', data.value.aiShowNotePanel.toString())
      localStorage.setItem('ai-note-title', data.value.aiNoteTitle)
      localStorage.setItem('ai-note-content', data.value.aiNoteContent)
      localStorage.setItem('ai-selected-model', data.value.aiSelectedModel)
      localStorage.setItem('mytips-editor-mode', data.value.editorMode)
      localStorage.setItem('mytips-highlight-theme', data.value.highlightTheme)
      localStorage.setItem('mytips-markdown-theme', data.value.markdownTheme)
      localStorage.setItem('mytips-highlight-theme-manual', data.value.highlightThemeManual.toString())
      localStorage.setItem('sidebarWidth', data.value.sidebarWidth.toString())
      localStorage.setItem('noteListWidth', data.value.noteListWidth.toString())
      localStorage.setItem('noteListHidden', data.value.noteListHidden.toString())
      localStorage.setItem('mytips-ui-settings', JSON.stringify(data.value.uiSettings))
    } catch (error) {
      console.error('Failed to save data to localStorage:', error)
    }
  }
  
  // 监听数据变化，自动保存到 localStorage
  watch(data, saveToLocalStorage, { deep: true })
  
  // 初始化
  loadFromLocalStorage()
  
  // Actions
  function setUserLanguage(language: string) {
    data.value.userLanguage = language
  }
  
  function updateTagUsage(tagUsage: Record<string, number>) {
    data.value.tagUsage = { ...tagUsage }
  }
  
  function incrementTagUsage(tagId: string) {
    data.value.tagUsage[tagId] = (data.value.tagUsage[tagId] || 0) + 1
  }
  
  function setNeedRefreshTips(value: boolean) {
    data.value.needRefreshTips = value
  }
  
  function setAiShowNotePanel(value: boolean) {
    data.value.aiShowNotePanel = value
  }
  
  function setAiNoteTitle(title: string) {
    data.value.aiNoteTitle = title
  }
  
  function setAiNoteContent(content: string) {
    data.value.aiNoteContent = content
  }
  
  function setAiSelectedModel(model: string) {
    data.value.aiSelectedModel = model
  }
  
  function setEditorMode(mode: string) {
    data.value.editorMode = mode
  }
  
  function setHighlightTheme(theme: string) {
    data.value.highlightTheme = theme
  }
  
  function setMarkdownTheme(theme: string) {
    data.value.markdownTheme = theme
  }
  
  function setHighlightThemeManual(value: boolean) {
    data.value.highlightThemeManual = value
  }
  
  function setSidebarWidth(width: number) {
    data.value.sidebarWidth = width
  }
  
  function setNoteListWidth(width: number) {
    data.value.noteListWidth = width
  }
  
  function setNoteListHidden(hidden: boolean) {
    data.value.noteListHidden = hidden
  }
  
  function setUiSettings(settings: Record<string, any>) {
    data.value.uiSettings = { ...settings }
  }
  
  return {
    // 状态
    data,
    
    // Actions
    setUserLanguage,
    updateTagUsage,
    incrementTagUsage,
    setNeedRefreshTips,
    setAiShowNotePanel,
    setAiNoteTitle,
    setAiNoteContent,
    setAiSelectedModel,
    setEditorMode,
    setHighlightTheme,
    setMarkdownTheme,
    setHighlightThemeManual,
    setSidebarWidth,
    setNoteListWidth,
    setNoteListHidden,
    setUiSettings,
    
    // 工具方法
    loadFromLocalStorage,
    saveToLocalStorage
  }
})
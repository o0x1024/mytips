import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

// 从本地存储获取主题设置并立即应用到文档上
// 这个函数应该在应用加载时尽早调用，防止主题闪烁
export function applyThemeEarly() {
  try {
    const savedSettings = localStorage.getItem('mytips-ui-settings')
    if (savedSettings) {
      const parsed = JSON.parse(savedSettings)
      if (parsed && parsed.theme) {
        document.documentElement.setAttribute('data-theme', parsed.theme)
      }
    }
  } catch (e) {
    console.error('无法提前应用主题:', e)
  }
}

export interface UISettings {
  fontSize: string;
  exactFontSize: number;
  componentSize: string;
  sidebarWidth: number;
  listWidth: number;
  exactSidebarWidth: number;
  exactListWidth: number;
  isSidebarCollapsed: boolean;
  theme: string;
  language: string; // 新增语言设置
  clipboard: {
    ignoreSensitiveContent: boolean;
    captureImages: boolean;
    captureSourceInfo: boolean;
    retentionDays: number;
    encryptStorage: boolean;
  };
}

// 默认UI设置
const defaultSettings: UISettings = {
  fontSize: 'medium',
  exactFontSize: 16,
  componentSize: 'medium',
  sidebarWidth: 300,
  listWidth: 300,
  exactSidebarWidth: 300,
  exactListWidth: 300,
  isSidebarCollapsed: false,
  theme: 'mytheme',
  language: 'zh', // 默认语言为中文
  clipboard: {
    ignoreSensitiveContent: true,
    captureImages: false,
    captureSourceInfo: true,
    retentionDays: 7,
    encryptStorage: false,
  }
}

export const useUIStore = defineStore('ui', () => {
  // 状态
  const settings = ref<UISettings>(loadSettings())
  
  // 是否已经加载过设置
  const settingsLoaded = ref(false)
  
  // 字体大小选项
  const fontSizeOptions = [
    { label: '小', value: 'small', class: 'text-sm', size: 14 },
    { label: '中', value: 'medium', class: 'text-base', size: 16 },
    { label: '大', value: 'large', class: 'text-lg', size: 18 },
    { label: '特大', value: 'x-large', class: 'text-xl', size: 20 }
  ]
  
  // 组件大小选项
  const componentSizeOptions = [
    { label: '紧凑', value: 'compact', navWidth: 200, listWidth: 250 },
    { label: '中等', value: 'medium', navWidth: 250, listWidth: 300 },
    { label: '宽松', value: 'comfortable', navWidth: 300, listWidth: 350 }
  ]
  
  // 剪贴板保留天数选项
  const clipboardRetentionOptions = [
    { label: '1天', value: 1 },
    { label: '3天', value: 3 },
    { label: '7天', value: 7 },
    { label: '14天', value: 14 },
    { label: '30天', value: 30 },
    { label: '永久保留', value: 0 }
  ]
  
  // DaisyUI主题选项
  const themeOptions = [
    { name: 'themes.mytheme', value: 'mytheme' },
    { name: 'themes.dark', value: 'dark' },
    { name: 'themes.light', value: 'light' },
    { name: 'themes.retro', value: 'retro' },
    { name: 'themes.cyberpunk', value: 'cyberpunk' },
    { name: 'themes.halloween', value: 'halloween' },
    { name: 'themes.garden', value: 'garden' },
    { name: 'themes.forest', value: 'forest' },
    { name: 'themes.aqua', value: 'aqua' },
    { name: 'themes.lofi', value: 'lofi' },
    { name: 'themes.pastel', value: 'pastel' },
    { name: 'themes.fantasy', value: 'fantasy' },
    { name: 'themes.black', value: 'black' },
    { name: 'themes.luxury', value: 'luxury' },
    { name: 'themes.dracula', value: 'dracula' },
    { name: 'themes.autumn', value: 'autumn' },
    { name: 'themes.business', value: 'business' },
    { name: 'themes.acid', value: 'acid' },
    { name: 'themes.lemonade', value: 'lemonade' },
    { name: 'themes.night', value: 'night' },
    { name: 'themes.coffee', value: 'coffee' },
    { name: 'themes.winter', value: 'winter' }
  ]
  
  // 从本地存储加载设置
  function loadSettings(): UISettings {
    const savedSettings = localStorage.getItem('mytips-ui-settings')
    if (savedSettings) {
      try {
        const parsed = JSON.parse(savedSettings)
        // 确保有所有必要的属性，包括嵌套属性
        return { 
          ...defaultSettings, 
          ...parsed,
          language: parsed.language || defaultSettings.language, // 确保 language 属性被加载
          clipboard: {
            ...defaultSettings.clipboard,
            ...(parsed.clipboard || {})
          }
        }
      } catch (e) {
        console.error('无法解析UI设置:', e)
      }
    }
    return { ...defaultSettings }
  }
  
  // 保存设置到本地存储
  function saveSettings() {
    localStorage.setItem('mytips-ui-settings', JSON.stringify(settings.value))
  }
  
  // 更新字体大小
  function setFontSize(size: string) {
    settings.value.fontSize = size
    
    // 同时更新精确字体大小
    const option = fontSizeOptions.find(opt => opt.value === size)
    if (option) {
      settings.value.exactFontSize = option.size
    }
    
    saveSettings()
    applySettings()
  }
  
  // 更新精确字体大小
  function setExactFontSize(size: number) {
    // 确保size是有效的数字
    if (isNaN(size) || size < 10 || size > 24) {
      console.error('无效的字体大小:', size);
      return;
    }
    
    // 设置字体大小
    settings.value.exactFontSize = size;
    
    // 尝试匹配预设大小
    const matchingOption = fontSizeOptions.find(opt => opt.size === size);
    if (matchingOption) {
      settings.value.fontSize = matchingOption.value;
    } else {
      // 如果没有匹配的预设，设置为自定义
      settings.value.fontSize = 'custom';
    }
    
    saveSettings();
    
    // 应用字体大小到文档元素
    applyFontSize(size);
  }
  
  // 应用字体大小到文档
  function applyFontSize(size: number) {
    
    // 设置CSS变量
    document.documentElement.style.setProperty('--base-font-size', `${size}px`);
    
    // 设置HTML元素字体大小
    document.documentElement.style.fontSize = `${size}px`;
    
    // 添加data属性用于调试
    document.documentElement.setAttribute('data-font-size', `${size}`);
  }
  
  // 更新组件大小
  function setComponentSize(size: string) {
    settings.value.componentSize = size
    
    // 更新相应的宽度值
    const option = componentSizeOptions.find(opt => opt.value === size)
    if (option) {
      settings.value.sidebarWidth = option.navWidth
      settings.value.listWidth = option.listWidth
    }
    
    saveSettings()
    applySettings()
  }

  // 设置侧边栏折叠状态
  function setSidebarCollapsed(collapsed: boolean) {
    console.log('UI Store设置侧边栏折叠状态:', collapsed)
    settings.value.isSidebarCollapsed = collapsed
    
    // 立即应用侧边栏折叠状态
    const isCollapsed = collapsed ? 'true' : 'false'
    document.documentElement.setAttribute('data-sidebar-collapsed', isCollapsed)
    
    saveSettings()
    applySettings()
  }
  
  // 设置主题
  function setTheme(theme: string) {
    settings.value.theme = theme
    document.documentElement.setAttribute('data-theme', theme)
    saveSettings()
  }
  
  // 设置语言
  function setLanguage(lang: string) {
    settings.value.language = lang
    saveSettings()
  }
  
  // 重置为默认设置
  function resetToDefaults() {
    settings.value = { ...defaultSettings }
    saveSettings()
    applySettings()
  }
  
  // 更新精确组件大小
  function setExactComponentSize(sidebarWidth: number, listWidth: number) {
    settings.value.exactSidebarWidth = sidebarWidth;
    settings.value.exactListWidth = listWidth;
    
    // 同时更新标准尺寸值
    settings.value.sidebarWidth = sidebarWidth;
    settings.value.listWidth = listWidth;
    
    // 尝试匹配预设大小
    const matchingOption = componentSizeOptions.find(
      opt => opt.navWidth === sidebarWidth && opt.listWidth === listWidth
    );
    
    if (matchingOption) {
      settings.value.componentSize = matchingOption.value;
    } else {
      // 如果没有匹配的预设，设置为自定义
      settings.value.componentSize = 'custom';
    }
    
    saveSettings();
    applySettings();
  }
  
  // 应用设置到DOM
  function applySettings() {
    // 应用字体大小 - 先清除现有的字体大小类
    document.documentElement.classList.forEach(cls => {
      if (cls.startsWith('text-')) {
        document.documentElement.classList.remove(cls);
      }
    });
    
    // 只有在有预设类的情况下才添加类
    const fontSizeClass = fontSizeOptions.find(opt => opt.value === settings.value.fontSize)?.class;
    if (fontSizeClass) {
      document.documentElement.classList.add(fontSizeClass);
    }
    
    // 应用精确字体大小
    applyFontSize(settings.value.exactFontSize);
    
    // 组件大小通过CSS变量应用
    document.documentElement.style.setProperty('--sidebar-width', `${settings.value.exactSidebarWidth || settings.value.sidebarWidth}px`);
    document.documentElement.style.setProperty('--list-width', `${settings.value.exactListWidth || settings.value.listWidth}px`);
    
    // 侧边栏折叠状态
    const isCollapsed = settings.value.isSidebarCollapsed ? 'true' : 'false';
    document.documentElement.setAttribute('data-sidebar-collapsed', isCollapsed);
    
    // 应用主题 - 确保主题设置正确应用
    const currentTheme = document.documentElement.getAttribute('data-theme');
    if (currentTheme !== settings.value.theme) {
      document.documentElement.setAttribute('data-theme', settings.value.theme);
    }
  }
  
  // 监听设置变化
  watch(settings, () => {
    saveSettings()
  }, { deep: true })
  
  // 初始化应用设置
  function initialize() {
    // 确保从localStorage读取主题设置并应用
    const theme = settings.value.theme
    if (theme) {
      document.documentElement.setAttribute('data-theme', theme)
    }
    
    // 强制侧边栏展开状态
    settings.value.isSidebarCollapsed = false
    
    // 应用其他设置
    applySettings()
  }
  
  // 更新剪贴板设置
  function setClipboardSettings(clipboardSettings: Partial<UISettings['clipboard']>) {
    settings.value.clipboard = {
      ...settings.value.clipboard,
      ...clipboardSettings
    }
    saveSettings()
  }
  
  // 初始化应用
  initialize()
  
  return {
    settings,
    settingsLoaded,
    fontSizeOptions,
    componentSizeOptions,
    themeOptions,
    clipboardRetentionOptions,
    setFontSize,
    setExactFontSize,
    setComponentSize,
    setExactComponentSize,
    setSidebarCollapsed,
    setTheme,
    resetToDefaults,
    setClipboardSettings,
    setLanguage,
    initialize
  }
}) 
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './assets/main.css'
import './assets/globals.scss'
import { useUIStore, applyThemeEarly } from './stores/uiStore'
import { initializeApp } from './services/appInitializer'
import i18n from './i18n'
import { listen } from '@tauri-apps/api/event'
import { showToast } from './services/notification'

// 在应用初始化前尽早应用主题设置
applyThemeEarly()

// 导入页面组件
import Home from './views/Home.vue'
import Editor from './views/Editor.vue'
import Settings from './views/Settings.vue'
import AIAssistant from './views/AIAssistant.vue'
import ClipboardHistory from './views/ClipboardHistory.vue'

// 路由配置
const routes = [
  { 
    path: '/', 
    component: Home,
    meta: { keepAlive: true }
  },
  { 
    path: '/editor/:id?', 
    component: Editor,
    meta: { keepAlive: true }
  },
  { 
    path: '/settings', 
    component: Settings,
    meta: { keepAlive: true }
  },
  { 
    path: '/clipboard', 
    component: ClipboardHistory,
    meta: { keepAlive: true }
  },
  { 
    path: '/ai-assistant', 
    component: AIAssistant,
    meta: { keepAlive: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 创建Pinia状态管理
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

// 创建应用实例
const app = createApp(App)

// 全局禁用浏览器默认右键菜单
// document.addEventListener('contextmenu', event => {
//   event.preventDefault()
//   return false
// }, { capture: true })

// 创建并挂载应用
app.use(pinia)
app.use(router)
app.use(i18n)

// 加载状态管理
function updateLoadingStatus(message: string) {
  const statusElement = document.getElementById('loading-status')
  if (statusElement) {
    statusElement.textContent = message
  }
}

function hideLoadingScreen() {
  const loadingScreen = document.getElementById('loading-screen')
  const appElement = document.getElementById('app')
  
  if (loadingScreen && appElement) {
    // 显示应用内容
    appElement.classList.add('loaded')
    
    // 隐藏加载屏幕
    loadingScreen.classList.add('hidden')
    
    // 延迟移除元素，等待动画完成
    setTimeout(() => {
      loadingScreen.remove()
      // 恢复body的正常样式
      document.body.style.overflow = ''
    }, 500)
  }
}

// 初始化应用
async function startApp() {
  try {
    updateLoadingStatus('正在加载配置...')
    
    // 初始化UI设置
    const uiStore = useUIStore()
    uiStore.initialize()
    
    // 初始化数据库和其他服务
    await initializeApp(updateLoadingStatus)
    
    updateLoadingStatus('正在启动应用...')
    
    // 挂载应用
    app.mount('#app')
    
    updateLoadingStatus('启动完成')
    
    // 等待下一个事件循环，确保 Vue 应用完全挂载和渲染
    await new Promise(resolve => {
      // 使用 requestAnimationFrame 确保在浏览器下一次重绘前执行
      requestAnimationFrame(() => {
        // 再等待一帧，确保应用内容已经渲染
        requestAnimationFrame(() => {
          resolve(void 0)
        })
      })
    })
    
    // 隐藏加载屏幕
    hideLoadingScreen()
    
    console.log('Application started successfully')

    // 注册 AI 流错误事件监听，显示右上角提示
    try {
      await listen<{ id?: string; error?: string }>('ai-stream-error', (event) => {
        const payload = event.payload || {}
        const idPart = payload.id ? ` [${payload.id}]` : ''
        const message = payload.error ? `AI stream error${idPart}: ${payload.error}` : `AI stream error${idPart}`
        showToast(message, 'error', 6000)
        console.error('[AI Stream] Error event:', payload)
      })
    } catch (e) {
      console.warn('Failed to register ai-stream-error listener:', e)
    }
  } catch (error) {
    console.error('Failed to start application:', error)
    updateLoadingStatus('启动失败，正在重试...')
    
    // 即使初始化失败也要挂载应用，让用户能够看到错误
    app.mount('#app')
    
    // 等待应用挂载完成后再隐藏加载屏幕
    await new Promise(resolve => {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          resolve(void 0)
        })
      })
    })
    
    // 延迟隐藏加载屏幕，给用户时间看到错误信息
    setTimeout(() => {
      hideLoadingScreen()
    }, 1000)
  }
}

// 启动应用
startApp()

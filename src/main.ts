import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './assets/main.css'
import { useUIStore, applyThemeEarly } from './stores/uiStore'

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

// 创建并挂载应用
const app = createApp(App)
app.use(pinia)
app.use(router)

// 初始化UI设置
const uiStore = useUIStore()
uiStore.initialize()

app.mount('#app')

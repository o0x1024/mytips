<template>
  <div class="flex flex-col h-screen bg-base-100">
    <!-- 顶部导航 -->
    <div class="navbar bg-base-200 px-4">
      <div class="flex-1">
        <button class="btn btn-ghost" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <h1 v-if="!isMobile" class="text-xl font-bold">{{ $t('settings.title') }} - {{ getCurrentPageTitle() }}</h1>
        <h1 v-else class="text-xl font-bold">{{ $t('settings.title') }}</h1>
      </div>

      <!-- Mobile Page Selector -->
      <div v-if="isMobile" class="flex-none">
        <select class="select select-bordered" v-model="currentPage">
          <option v-for="page in settingsPages" :key="page.id" :value="page.id">
            {{ $t(page.title) }}
          </option>
        </select>
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- 左侧导航栏 (仅桌面) -->
      <nav v-if="!isMobile" class="w-64 min-w-[240px] max-w-[280px] bg-base-100 flex flex-col py-6 px-3 gap-2 shadow-lg border-r border-base-200">
        <div>
          <h2 class="text-lg font-semibold text-base-content px-3 mb-4">{{ $t('settings.optionsTitle') }}</h2>
        </div>
        
        <button 
          v-for="page in settingsPages" 
          :key="page.id"
          class="nav-button group flex items-center gap-3 px-4 py-3 rounded-lg text-left transition-all duration-200 ease-in-out hover:bg-base-200 hover:shadow-sm"
          :class="{ 
            'bg-primary text-primary-content shadow-md border-primary': currentPage === page.id,
            'text-base-content hover:text-primary border-transparent': currentPage !== page.id 
          }"
          @click="setCurrentPage(page.id)"
        >
          <!-- 图标 -->
          <div class="flex-shrink-0 w-5 h-5" :class="{ 'text-primary-content': currentPage === page.id, 'text-base-content/70 group-hover:text-primary': currentPage !== page.id }">
            <!-- 外观图标 -->
            <svg v-if="page.id === 'appearance'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z" />
            </svg>
            
            <!-- 临时笔记图标 -->
            <svg v-else-if="page.id === 'clipboard'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
            </svg>
            
            <!-- 网络图标 -->
            <svg v-else-if="page.id === 'network'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
            </svg>
            
            <!-- 数据管理图标 -->
            <svg v-else-if="page.id === 'data'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
            </svg>
            
            <!-- 应用设置图标 -->
            <svg v-else-if="page.id === 'app'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            
            <!-- AI助手图标 -->
            <svg v-else-if="page.id === 'ai'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
            </svg>
            
            <!-- 更新图标 -->
            <svg v-else-if="page.id === 'update'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            
            <!-- 提示词模板图标 -->
            <svg v-else-if="page.id === 'templates'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" />
            </svg>
            
            <!-- 关于图标 -->
            <svg v-else-if="page.id === 'about'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          
          <!-- 标题 -->
          <span class="font-medium text-sm flex-1">{{ $t(page.title) }}</span>
          
          <!-- 活跃状态指示器 -->
          <div v-if="currentPage === page.id" class="w-2 h-2 bg-primary-content rounded-full opacity-80"></div>
          
          <!-- 箭头指示器（非活跃状态） -->
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 opacity-0 group-hover:opacity-60 transition-opacity duration-200" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
        
        <!-- 底部装饰 -->
        <div class="mt-auto pt-4 border-t border-base-200">
          <div class="text-xs text-base-content/60 px-3 text-center">
            {{ $t('settings.footer') }}
          </div>
        </div>
      </nav>

      <!-- 右侧内容区域 -->
      <div class="flex-1 overflow-auto p-4 md:p-6 bg-base-100">
        <div class="max-w-3xl mx-auto">
          <!-- 动态渲染设置组件 -->
          <component :is="currentSettingComponent" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'

// 导入所有设置组件
import AppearanceSettings from '../components/settings/AppearanceSettings.vue'
import ClipboardSettings from '../components/settings/ClipboardSettings.vue'
import NetworkSettings from '../components/settings/NetworkSettings.vue'
import DataSettings from '../components/settings/DataSettings.vue'
import AppSettings from '../components/settings/AppSettings.vue'
import AISettings from '../components/settings/AISettings.vue'
import UpdateSettings from '../components/settings/UpdateSettings.vue'
import TemplateSettings from '../components/settings/TemplateSettings.vue'
import AboutSettings from '../components/settings/AboutSettings.vue'

const router = useRouter()
const { t } = useI18n()

// 响应式状态
const windowWidth = ref(window.innerWidth)
const isMobile = computed(() => windowWidth.value < 768)

// 页面管理
const currentPage = ref('appearance')
const settingsPages = [
  { id: 'appearance', title: 'settings.pages.appearance' },
  { id: 'clipboard', title: 'settings.pages.clipboard' },
  { id: 'network', title: 'settings.pages.network' },
  { id: 'data', title: 'settings.pages.data' },
  { id: 'app', title: 'settings.pages.app' },
  { id: 'ai', title: 'settings.pages.ai' },
  { id: 'templates', title: 'settings.pages.templates' },
  { id: 'update', title: 'settings.pages.update' },
  { id: 'about', title: 'settings.pages.about' }
]

// 组件映射
const settingComponents = {
  appearance: AppearanceSettings,
  clipboard: ClipboardSettings,
  network: NetworkSettings,
  data: DataSettings,
  app: AppSettings,
  ai: AISettings,
  update: UpdateSettings,
  templates: TemplateSettings,
  about: AboutSettings
}

// 当前设置组件
const currentSettingComponent = computed(() => {
  return settingComponents[currentPage.value as keyof typeof settingComponents] || AppearanceSettings
})

// 设置当前页面
function setCurrentPage(pageId: string) {
  currentPage.value = pageId
}

// 获取当前页面标题
function getCurrentPageTitle() {
  const page = settingsPages.find(p => p.id === currentPage.value)
  return page ? t(page.title) : t('settings.fallbackTitle')
}

// 返回上一页
function goBack() {
  window.history.back()
}

// 处理从路由参数导航
function handlePageNavigationFromRoute() {
  const route = router.currentRoute.value
  if (route.query.page && typeof route.query.page === 'string') {
    const targetPage = route.query.page
    if (settingsPages.some(page => page.id === targetPage)) {
      currentPage.value = targetPage
    }
  }
}

// 响应式处理
const onResize = () => {
  windowWidth.value = window.innerWidth
}

// 生命周期
onMounted(() => {
  handlePageNavigationFromRoute()
  window.addEventListener('resize', onResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', onResize)
})
</script>

<style scoped>
/* 响应式调整 */
@media (max-width: 768px) {
  .navbar {
    padding-left: 1rem;
    padding-right: 1rem;
  }
}

/* 导航按钮优化样式 */
.nav-button {
  position: relative;
  overflow: hidden;
  border: 1px solid transparent;
  backdrop-filter: blur(10px);
}

.nav-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left 0.5s ease;
}

.nav-button:hover::before {
  left: 100%;
}

/* 活跃状态的按钮样式 */
.nav-button.bg-primary {
  background: hsl(var(--p)) !important;
  color: hsl(var(--pc)) !important;
  border-color: hsl(var(--p)) !important;
  box-shadow: 0 4px 12px hsl(var(--p) / 0.3), 0 0 0 1px hsl(var(--pc) / 0.1) inset;
}

/* 确保选中状态的图标和文字颜色正确 */
.nav-button.bg-primary .flex-shrink-0,
.nav-button.bg-primary span {
  color: hsl(var(--pc)) !important;
}

/* 悬停状态优化 */
.nav-button:hover {
  transform: translateX(2px);
  border-color: hsl(var(--p) / 0.3);
}

.nav-button:not(.bg-primary):hover {
  background: hsl(var(--b2));
  color: hsl(var(--p));
}

/* 图标动画 */
.nav-button .flex-shrink-0 {
  transition: transform 0.2s ease, color 0.2s ease;
}

.nav-button:hover .flex-shrink-0 {
  transform: scale(1.1);
}

/* 导航栏整体样式 */
nav {
  background: hsl(var(--b1));
  backdrop-filter: blur(10px);
}

/* 设置选项标题样式优化 */
nav h2 {
  color: hsl(var(--bc)) !important;
  font-weight: 600;
  opacity: 0.9;
}

/* 底部装饰文字样式优化 */
nav .mt-auto .text-xs {
  color: hsl(var(--bc) / 0.6) !important;
}
</style>
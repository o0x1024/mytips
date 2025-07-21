<template>
  <div class="border-b border-base-300 p-2 bg-base-200" ref="toolbarContainer">
    <!-- 主工具栏 - 动态响应式布局 -->
    <div class="flex items-center justify-between gap-2">
      <!-- 左侧工具组 -->
      <div class="flex items-center gap-1 flex-shrink-0" ref="toolbarLeft">
        <!-- 目录按钮 -->
        
        <!-- 标题下拉菜单 - 始终显示 -->
        <div class="dropdown dropdown-bottom toolbar-item" data-priority="1">
          <button tabindex="0" class="btn btn-sm btn-ghost" title="插入标题">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2">
              <path d="M6 12h12M6 20V4M18 20V4M14 4v16M10 4v16"></path>
            </svg>
            <span class="ml-1 text-xs toolbar-text">标题</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 ml-1" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-32">
            <li><a @click="emit('command', 'insert-markdown', '# ')" class="text-2xl font-bold">H1</a></li>
            <li><a @click="emit('command', 'insert-markdown', '## ')" class="text-xl font-bold">H2</a></li>
            <li><a @click="emit('command', 'insert-markdown', '### ')" class="text-lg font-bold">H3</a></li>
            <li><a @click="emit('command', 'insert-markdown', '#### ')" class="text-base font-bold">H4</a></li>
            <li><a @click="emit('command', 'insert-markdown', '##### ')" class="text-sm font-bold">H5</a></li>
            <li><a @click="emit('command', 'insert-markdown', '###### ')" class="text-xs font-bold">H6</a></li>
          </ul>
        </div>

        <!-- 文本格式工具组 -->
        <div class="btn-group toolbar-item" data-priority="2">
          <button class="btn btn-sm btn-ghost" title="粗体" @click="emit('command', 'insert-markdown', '**', '**')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
              stroke-width="2">
              <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
              <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
            </svg>
          </button>
          <button class="btn btn-sm btn-ghost" title="斜体" @click="emit('command', 'insert-markdown', '*', '*')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
              stroke-width="2">
              <line x1="19" y1="4" x2="10" y2="4"></line>
              <line x1="14" y1="20" x2="5" y2="20"></line>
              <line x1="15" y1="4" x2="9" y2="20"></line>
            </svg>
          </button>
        </div>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="3" title="删除线"
          @click="emit('command', 'insert-markdown', '~~', '~~')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M17 9V5H7v4"></path>
            <path d="M7 13v6h10v-6"></path>
            <line x1="4" y1="12" x2="20" y2="12"></line>
          </svg>
        </button>

        <!-- 列表工具组 -->
        <div class="btn-group toolbar-item" data-priority="4">
          <button class="btn btn-sm btn-ghost" title="无序列表" @click="emit('command', 'insert-markdown', '- ')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
              stroke-width="2">
              <line x1="8" y1="6" x2="21" y2="6"></line>
              <line x1="8" y1="12" x2="21" y2="12"></line>
              <line x1="8" y1="18" x2="21" y2="18"></line>
              <line x1="3" y1="6" x2="3.01" y2="6"></line>
              <line x1="3" y1="12" x2="3.01" y2="12"></line>
              <line x1="3" y1="18" x2="3.01" y2="18"></line>
            </svg>
          </button>
          <button class="btn btn-sm btn-ghost" title="有序列表" @click="emit('command', 'insert-markdown', '1. ')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
              stroke-width="2">
              <line x1="10" y1="6" x2="21" y2="6"></line>
              <line x1="10" y1="12" x2="21" y2="12"></line>
              <line x1="10" y1="18" x2="21" y2="18"></line>
              <path d="M4 6h1v4"></path>
              <path d="M4 10h2"></path>
              <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>
            </svg>
          </button>
        </div>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="5" title="插入链接"
          @click="emit('command', 'insert-markdown', '[', '](https://)')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
          </svg>
        </button>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="6" title="引用块"
          @click="emit('command', 'insert-markdown', '> ')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
            <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
          </svg>
        </button>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="7" title="代码块"
          @click="emit('command', 'insert-markdown', '```\\n', '\\n```')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <polyline points="16 18 22 12 16 6"></polyline>
            <polyline points="8 6 2 12 8 18"></polyline>
          </svg>
        </button>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="8" title="插入图片"
          @click="emit('command', 'insert-markdown', '![', '](图片URL)')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <circle cx="8.5" cy="8.5" r="1.5"></circle>
            <polyline points="21 15 16 10 5 21"></polyline>
          </svg>
        </button>

        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="9" title="插入表格"
          @click="emit('command', 'insert-table')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="3" y1="9" x2="21" y2="9"></line>
            <line x1="3" y1="15" x2="21" y2="15"></line>
            <line x1="9" y1="3" x2="9" y2="21"></line>
            <line x1="15" y1="3" x2="15" y2="21"></line>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost toolbar-item" :class="{ 'btn-active': showToc }" data-priority="10"
          title="显示/隐藏目录" @click="emit('command', 'toggle-toc')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
          </svg>
        </button>

        <!-- 音频录制按钮 -->
        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="11"
          title="录制音频" @click="emit('command', 'toggle-audio-recording')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
        </button>

        <!-- 音频播放器按钮 -->
        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="12"
          title="音频播放器" @click="emit('command', 'toggle-audio-player')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z" />
          </svg>
        </button>

        <!-- 代码高亮主题 -->
        <div class="dropdown dropdown-bottom dropdown-end toolbar-item" data-priority="13">
          <button tabindex="0" class="btn btn-sm btn-ghost" title="代码高亮主题">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2">
              <path d="M12 2v20M4 12h16"></path>
              <rect x="6" y="6" width="12" height="12" rx="2" ry="2"></rect>
            </svg>
          </button>
          <ul tabindex="0"
            class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 max-h-96 overflow-y-auto">
            <li>
              <a href="#" @click.prevent="emit('command', 'set-highlight-theme', 'default')"
                :class="{ 'bg-primary text-primary-content': currentHighlightTheme === 'default' }">
                Default
                <span v-if="currentHighlightTheme === 'default'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-highlight-theme', 'okaidia')"
                :class="{ 'bg-primary text-primary-content': currentHighlightTheme === 'okaidia' }">
                Okaidia
                <span v-if="currentHighlightTheme === 'okaidia'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-highlight-theme', 'twilight')"
                :class="{ 'bg-primary text-primary-content': currentHighlightTheme === 'twilight' }">
                Twilight
                <span v-if="currentHighlightTheme === 'twilight'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-highlight-theme', 'solarized-light')"
                :class="{ 'bg-primary text-primary-content': currentHighlightTheme === 'solarized-light' }">
                Solarized Light
                <span v-if="currentHighlightTheme === 'solarized-light'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-highlight-theme', 'tomorrow-night')"
                :class="{ 'bg-primary text-primary-content': currentHighlightTheme === 'tomorrow-night' }">
                Tomorrow Night
                <span v-if="currentHighlightTheme === 'tomorrow-night'" class="ml-auto">✓</span>
              </a>
            </li>
          </ul>
        </div>
        <!-- Markdown 主题 -->
        <div class="dropdown dropdown-bottom toolbar-item" data-priority="12">
          <button tabindex="0" class="btn btn-sm btn-ghost" title="Markdown主题">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2">
              <path d="M12 2v20M4 12h16"></path>
              <path d="M6 6l6-4 6 4v12l-6 4-6-4V6z"></path>
            </svg>
          </button>
          <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'github')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'github' }">
                GitHub
                <span v-if="currentMarkdownTheme === 'github'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'typora')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'typora' }">
                Typora
                <span v-if="currentMarkdownTheme === 'typora'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'academic')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'academic' }">
                Academic
                <span v-if="currentMarkdownTheme === 'academic'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'material')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'material' }">
                Material Dark
                <span v-if="currentMarkdownTheme === 'material'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'minimalist')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'minimalist' }">
                Minimalist
                <span v-if="currentMarkdownTheme === 'minimalist'" class="ml-auto">✓</span>
              </a>
            </li>
            <li>
              <a href="#" @click.prevent="emit('command', 'set-markdown-theme', 'elegant')"
                :class="{ 'bg-primary text-primary-content': currentMarkdownTheme === 'elegant' }">
                Elegant
                <span v-if="currentMarkdownTheme === 'elegant'" class="ml-auto">✓</span>
              </a>
            </li>
          </ul>
        </div>
      </div>

      <!-- 右侧工具组 -->
      <div class="flex items-center gap-1 flex-shrink-0 toolbar-right-group" ref="toolbarRight">
        
        <div v-if="isFullscreen" class="text-xs text-base-content/60 mr-2 toolbar-item" data-priority="13">
          <span class="badge badge-primary badge-xs mr-1">全屏</span>
          <span class="toolbar-text">F1:编辑 F2:预览 F3:分屏 ESC:退出</span>
        </div>

        <!-- 视图模式 -->
        <div class="btn-group toolbar-item" data-priority="13">
          <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isEditOnly }" title="仅编辑"
            @click="emit('command', 'set-edit-mode', 'editOnly')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
            </svg>
          </button>
          <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isPreviewMode }"
            @click="emit('command', 'set-edit-mode', 'preview')" :title="isFullscreen ? '预览 (F2)' : '预览'">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2">
              <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
              <circle cx="12" cy="12" r="3"></circle>
            </svg>
          </button>
          <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isSplitMode }"
            @click="emit('command', 'set-edit-mode', 'split')" :title="isFullscreen ? '分屏模式 (F3)' : '分屏模式'">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
              <line x1="12" y1="3" x2="12" y2="21"></line>
            </svg>
          </button>
        </div>

        <!-- 全屏切换按钮 -->
        <button class="btn btn-sm btn-ghost toolbar-item" data-priority="14" :class="{ 'btn-active': isFullscreen }"
          title="切换全屏" @click="emit('command', 'toggle-fullscreen')">
          <!-- 进入全屏图标 -->
          <svg v-if="!isFullscreen" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M4 8V4h4M20 8V4h-4M4 16v4h4M20 16v4h-4" />
          </svg>
          <!-- 退出全屏图标 -->
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8 4H4v4M16 4h4v4M8 20H4v-4M20 16v4h-4" />
          </svg>
        </button>

        <!-- 更多工具下拉菜单 -->
        <div class="dropdown dropdown-bottom dropdown-end" ref="moreToolsDropdown" v-if="hiddenItems.length > 0">
          <button tabindex="0" class="btn btn-sm btn-ghost" title="更多工具">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01" />
            </svg>
          </button>
          <div tabindex="0" class="dropdown-content z-[1] mt-2 p-2 shadow-lg bg-base-200 rounded-box w-auto">
            <div class="flex flex-wrap items-center gap-1">
              <button v-for="item in hiddenItems" :key="item.priority" @click="item.action()" v-html="item.content" class="btn btn-sm btn-ghost flex items-center gap-1"></button>
              <div class="divider divider-horizontal mx-1" v-if="hiddenItems.length > 0"></div>
              <button @click="emit('command', 'insert-markdown', '- [ ] ')" class="btn btn-sm btn-ghost flex items-center gap-1">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none"
                  stroke="currentColor" stroke-width="2">
                  <rect x="3" y="5" width="6" height="6" rx="1"></rect>
                  <path d="m9 11-6-6"></path>
                  <line x1="13" y1="8" x2="21" y2="8"></line>
                  <rect x="3" y="17" width="6" height="6" rx="1"></rect>
                  <line x1="13" y1="20" x2="21" y2="20"></line>
                </svg>
                任务列表
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'

defineProps({
  isFullscreen: Boolean,
  isEditOnly: Boolean,
  isPreviewMode: Boolean,
  isSplitMode: Boolean,
  showToc: Boolean,
  currentHighlightTheme: String,
  currentMarkdownTheme: String,
});

const emit = defineEmits(['command']);

// 动态响应式工具栏相关状态
const toolbarContainer = ref<HTMLElement | null>(null)
const toolbarLeft = ref<HTMLElement | null>(null)
const toolbarRight = ref<HTMLElement | null>(null)
const moreToolsDropdown = ref<HTMLElement | null>(null)
const hiddenItems = ref<any[]>([])
const resizeObserver = ref<ResizeObserver | null>(null)

function updateToolbar() {
  if (!toolbarContainer.value || !toolbarLeft.value || !toolbarRight.value || !moreToolsDropdown.value) return

  const availableWidth = toolbarContainer.value.clientWidth
  const rightWidth = toolbarRight.value.clientWidth
  const leftWidth = toolbarLeft.value.clientWidth
  const moreMenuWidth = moreToolsDropdown.value.clientWidth
  const totalWidth = leftWidth + rightWidth

  const allItems = Array.from(toolbarLeft.value.querySelectorAll('.toolbar-item'))
    .map(el => ({
      element: el as HTMLElement,
      priority: parseInt((el as HTMLElement).dataset.priority || '999')
    }))
    .sort((a, b) => b.priority - a.priority)

  hiddenItems.value = []

  // 先将所有项目设为可见
  allItems.forEach(({ element }) => {
    element.style.display = ''
  });

  let currentWidth = totalWidth
  if (currentWidth > availableWidth) {
    moreToolsDropdown.value.style.display = ''
    currentWidth += moreMenuWidth
  } else {
    moreToolsDropdown.value.style.display = 'none'
  }

  for (const item of allItems) {
    if (currentWidth <= availableWidth) break

    const itemWidth = item.element.offsetWidth
    item.element.style.display = 'none'
    currentWidth -= itemWidth

    hiddenItems.value.push({
      priority: item.priority,
      content: getItemContent(item.element),
      action: () => {
        const command = getItemAction(item.element);
        if (command) {
          command();
        }
      }
    })
  }

  // 按优先级排序隐藏项目
  hiddenItems.value.sort((a, b) => a.priority - b.priority)

  // 隐藏文本标签如果空间不足
  updateTextVisibility(availableWidth, currentWidth)
}

function updateTextVisibility(availableWidth: number, currentWidth: number) {
  const textElements = toolbarLeft.value?.querySelectorAll('.toolbar-text') as NodeListOf<HTMLElement>

  if (!textElements) return

  // 如果空间紧张，隐藏文本标签
  const shouldHideText = currentWidth > availableWidth * 0.8

  textElements.forEach(element => {
    element.style.display = shouldHideText ? 'none' : ''
  })
}

function getItemAction(item: HTMLElement): (() => void) | null {
  const priority = parseInt(item.dataset.priority || '999')

  // 根据优先级返回对应的操作函数
  switch (priority) {
    case 3: return () => emit('command', 'insert-markdown', '~~', '~~')
    case 4: return () => emit('command', 'insert-markdown', '- ')
    case 5: return () => emit('command', 'insert-markdown', '[', '](https://)')
    case 6: return () => emit('command', 'insert-markdown', '> ')
    case 7: return () => emit('command', 'insert-markdown', '```\\n', '\\n```')
    case 8: return () => emit('command', 'insert-markdown', '![', '](图片URL)')
    case 9: return () => emit('command', 'insert-table')
    case 10: return () => { } // 显示/隐藏目录，由父组件处理
    case 11: return () => { } // 代码高亮主题 - 在模板中处理
    case 12: return () => { } // Markdown主题 - 在模板中处理
    case 13: return () => { } // 全屏提示 - 在模板中处理
    default: return null
  }
}

function getItemContent(item: HTMLElement): string {
  const priority = parseInt(item.dataset.priority || '999')

  // 根据优先级返回对应的HTML内容
  switch (priority) {
    case 3: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M17 9V5H7v4"></path>
        <path d="M7 13v6h10v-6"></path>
        <line x1="4" y1="12" x2="20" y2="12"></line>
      </svg>
      删除线`
    case 4: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="8" y1="6" x2="21" y2="6"></line>
        <line x1="8" y1="12" x2="21" y2="12"></line>
        <line x1="8" y1="18" x2="21" y2="18"></line>
        <line x1="3" y1="6" x2="3.01" y2="6"></line>
        <line x1="3" y1="12" x2="3.01" y2="12"></line>
        <line x1="3" y1="18" x2="3.01" y2="18"></line>
      </svg>
      无序列表`
    case 5: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
        <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
      </svg>
      插入链接`
    case 6: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
        <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
      </svg>
      引用块`
    case 7: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="16 18 22 12 16 6"></polyline>
        <polyline points="8 6 2 12 8 18"></polyline>
      </svg>
      代码块`
    case 8: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <circle cx="8.5" cy="8.5" r="1.5"></circle>
        <polyline points="21 15 16 10 5 21"></polyline>
      </svg>
      插入图片`
    case 9: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <line x1="3" y1="9" x2="21" y2="9"></line>
        <line x1="3" y1="15" x2="21" y2="15"></line>
        <line x1="9" y1="3" x2="9" y2="21"></line>
        <line x1="15" y1="3" x2="15" y2="21"></line>
      </svg>
      插入表格`
    case 10: return `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
      </svg>
      目录`
    default: return '未知工具'
  }
}


onMounted(() => {
  if (typeof window !== 'undefined') {
    resizeObserver.value = new ResizeObserver(() => {
      updateToolbar()
    })
    if (toolbarContainer.value) {
      resizeObserver.value.observe(toolbarContainer.value)
    }
    nextTick(updateToolbar);
  }
})

onBeforeUnmount(() => {
  if (resizeObserver.value) {
    resizeObserver.value.disconnect()
  }
})
</script>

<style scoped>
/* 可以在这里添加一些工具栏的特定样式 */
.toolbar-item {
  flex-shrink: 0;
}
</style> 
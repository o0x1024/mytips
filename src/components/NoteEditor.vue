<template>
  <div class="h-full flex flex-col" tabindex="0" @focusout="onEditorBlur">
    <!-- 顶部工具栏 -->
    <div class="p-2 border-b border-base-300 flex items-center justify-between">
      <!-- 标题和状态区 -->
      <div class="flex-1">
        <input type="text" placeholder="无标题笔记..."
          class="input input-lg w-full text-xl font-bold p-0 border-0 focus:outline-none bg-transparent"
          v-model="localNote.title" @input="autoSave" @blur="onTitleBlur" />
      </div>

      <!-- 操作按钮区 -->
      <div class="flex items-center gap-2">
        <!-- AI扩充按钮 -->
        <button class="btn btn-sm btn-ghost btn-square" title="AI扩充内容" @click="expandWithAI()">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
          </svg>
        </button>

        <div class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-sm btn-ghost btn-square" title="更多操作">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
            </svg>
          </button>
          <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-base-100 rounded-box w-52">
            <li><a @click="shareNote">分享链接</a></li>
            <li><a @click="exportNote">导出</a></li>
            <li><a @click="$emit('duplicate-note')">创建副本</a></li>
            <li><a @click="$emit('delete-note')" class="text-error">删除</a></li>
          </ul>
        </div>
      </div>
    </div>

    <!-- 编辑器工具栏 -->
    <div class="border-b border-base-300 p-2 flex flex-wrap items-center gap-2 bg-base-200">
      <div class="btn-group">
        <button class="btn btn-sm btn-ghost" title="标题1" @click="insertMarkdown('# ')">H1</button>
        <button class="btn btn-sm btn-ghost" title="标题2" @click="insertMarkdown('## ')">H2</button>
        <button class="btn btn-sm btn-ghost" title="标题3" @click="insertMarkdown('### ')">H3</button>
      </div>

      <div class="btn-group">
        <button class="btn btn-sm btn-ghost" title="粗体" @click="insertMarkdown('**', '**')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
            <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="斜体" @click="insertMarkdown('*', '*')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <line x1="19" y1="4" x2="10" y2="4"></line>
            <line x1="14" y1="20" x2="5" y2="20"></line>
            <line x1="15" y1="4" x2="9" y2="20"></line>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="删除线" @click="insertMarkdown('~~', '~~')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M17 9V5H7v4"></path>
            <path d="M7 13v6h10v-6"></path>
            <line x1="4" y1="12" x2="20" y2="12"></line>
          </svg>
        </button>
      </div>

      <div class="btn-group">
        <button class="btn btn-sm btn-ghost" title="无序列表" @click="insertMarkdown('- ')">
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
        <button class="btn btn-sm btn-ghost" title="有序列表" @click="insertMarkdown('1. ')">
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
        <button class="btn btn-sm btn-ghost" title="任务列表" @click="insertMarkdown('- [ ] ')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <rect x="3" y="5" width="6" height="6" rx="1"></rect>
            <path d="m9 11-6-6"></path>
            <line x1="13" y1="8" x2="21" y2="8"></line>
            <rect x="3" y="17" width="6" height="6" rx="1"></rect>
            <line x1="13" y1="20" x2="21" y2="20"></line>
          </svg>
        </button>
      </div>

      <button class="btn btn-sm btn-ghost" title="引用块" @click="insertMarkdown('> ')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
          stroke-width="2">
          <path
            d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z">
          </path>
          <path
            d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z">
          </path>
        </svg>
      </button>

      <button class="btn btn-sm btn-ghost" title="插入链接" @click="insertMarkdown('[', '](https://)')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
          stroke-width="2">
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
        </svg>
      </button>

      <button class="btn btn-sm btn-ghost" title="插入图片" @click="insertMarkdown('![', '](图片URL)')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
          stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
      </button>

      <button class="btn btn-sm btn-ghost" title="代码块" @click="insertMarkdown('```\n', '\n```')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
          stroke-width="2">
          <polyline points="16 18 22 12 16 6"></polyline>
          <polyline points="8 6 2 12 8 18"></polyline>
        </svg>
      </button>

      <button class="btn btn-sm btn-ghost" title="插入表格" @click="insertTable">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
          stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="3" y1="9" x2="21" y2="9"></line>
          <line x1="3" y1="15" x2="21" y2="15"></line>
          <line x1="9" y1="3" x2="9" y2="21"></line>
          <line x1="15" y1="3" x2="15" y2="21"></line>
        </svg>
      </button>

      <div class="dropdown dropdown-end ml-1">
        <button tabindex="0" class="btn btn-sm btn-ghost" title="代码高亮主题">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M12 2v20M4 12h16"></path>
            <rect x="6" y="6" width="12" height="12" rx="2" ry="2"></rect>
          </svg>
        </button>
        <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
          <li><a href="#" @click.prevent="setHighlightTheme('default')">Default</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('dark')">Dark</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('tomorrow')">Tomorrow</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('twilight')">Twilight</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('okaidia')">Okaidia</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('solarizedlight')">Solarized Light</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('funky')">Funky</a></li>
        </ul>
      </div>

      <div class="ml-auto flex gap-1">
        <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isEditOnly }" @click="setEditMode('editOnly')"
          title="仅编辑">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isPreviewMode }" @click="setEditMode('preview')"
          title="预览">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
            <circle cx="12" cy="12" r="3"></circle>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" :class="{ 'btn-active': isSplitMode }" @click="setEditMode('split')"
          title="分屏模式">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="12" y1="3" x2="12" y2="21"></line>
          </svg>
        </button>
      </div>
    </div>

    <!-- 主要编辑区域 -->
    <div class="flex-1 flex overflow-hidden relative">
      <!-- Markdown编辑器 -->
      <textarea v-if="!isPreviewMode || isEditOnly || isSplitMode"
        class="flex-1 p-4 h-full resize-none focus:outline-none font-mono text-base overflow-auto"
        :class="{ 'w-1/2': isSplitMode }" placeholder="开始输入内容..." v-model="localNote.content" @input="autoSave"
        @contextmenu.prevent="handleContextMenu" @paste="handlePaste" @keydown="handleKeyDown"
        @scroll="handleEditorScroll" ref="editorTextarea" @blur="onContentBlur"></textarea>

      <!-- 右键菜单 -->
      <div v-if="showContextMenu"
        class="context-menu absolute bg-base-200 text-base-content rounded-md shadow-lg p-2 z-30"
        :style="{ top: contextMenuY + 'px', left: contextMenuX + 'px' }">
        <ul class="menu menu-sm p-1">
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="expandSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
              TIP一下
            </button>
          </li>
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="explainSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              解释一下
            </button>
          </li>
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="translateSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
              </svg>
              翻译一下
            </button>
          </li>
          <li>
            <button class="flex items-center gap-1" @click="copySelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
              </svg>
              复制
            </button>
          </li>
          <li>
            <button class="flex items-center gap-1" @click="pasteFromClipboard">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
              粘贴
            </button>
          </li>
        </ul>
      </div>

      <!-- AI处理中遮罩 -->
      <div v-if="isAIProcessing" class="absolute inset-0 bg-base-300/50 flex items-center justify-center z-20">
        <div class="card bg-base-100 p-4 shadow-lg max-w-md w-full">
          <div class="flex items-center gap-4 mb-4">
            <span class="loading loading-spinner loading-md"></span>
            <p>AI正在处理中...</p>
          </div>

          <div v-if="isStreaming && streamingContent" class="mb-4 p-3 bg-base-200 rounded-lg max-h-60 overflow-y-auto">
            <div class="prose prose-sm">{{ streamingContent }}</div>
          </div>

          <div class="flex justify-end">
            <button v-if="isStreaming" class="btn btn-sm btn-error" @click="cancelAIGeneration">
              取消生成
            </button>
          </div>
        </div>
      </div>

      <!-- Markdown预览区 -->
      <div v-if="isPreviewMode || isSplitMode"
        class="flex-1 p-4 overflow-auto prose prose-sm md:prose-base lg:prose-lg dark:prose-invert max-w-none"
        :class="{ 'w-1/2': isSplitMode }" @scroll="handlePreviewScroll" ref="previewDiv">
        <div v-html="renderedContent" class="prose max-w-none"></div>
      </div>
    </div>

    <!-- 底部元数据区域 -->
    <div class="border-t border-base-100 p-4 bg-base-200">
      <!-- 将标签选择器和统计信息放在同一行 -->
      <div class="flex flex-wrap w-full gap-4 items-center justify-between">
        <!-- 标签选择器组件 -->
        <div class="flex-1">
          <TagSelector v-model="localNote.tags" :contentText="localNote.content" :titleText="localNote.title"
            @saveNote="saveNoteToList" />
        </div>

        <!-- 统计信息和状态指示器 -->
        <div class="text-xs text-base-content/80 flex items-center gap-4 shrink-0">
          <!-- 图片加载状态指示器 -->
          <div v-if="isLoadingImages" class="flex items-center gap-1 text-info" title="图片加载中...">
            <span class="loading loading-spinner loading-xs"></span>
            <span>加载图片</span>
          </div>
          <span title="字数">{{ wordCount }} 字</span>
          <span title="创建时间">创建: {{ formatDateTime(localNote.created_at) }}</span>
          <span title="修改时间">修改: {{ formatDateTime(localNote.updated_at) }}</span>
        </div>
      </div>
    </div>

    <!-- 在模板中添加AI解释结果的浮动框 -->
    <div v-if="showExplanationBox" class="fixed inset-0 flex items-center justify-center z-50 bg-base-300/50">
      <div class="card bg-base-100 shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col">
        <div class="card-body p-4">
          <div class="flex justify-between items-start mb-4">
            <h2 class="card-title">解释说明</h2>
            <button class="btn btn-sm btn-ghost" @click="showExplanationBox = false">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div v-if="isExplaining" class="flex items-center gap-2 mb-4">
            <span class="loading loading-spinner loading-md"></span>
            <p>AI 正在生成解释...</p>
          </div>

          <div v-else class="overflow-y-auto max-h-[60vh] prose prose-sm md:prose-base dark:prose-invert">
            <blockquote class="bg-base-200 p-3 rounded-lg mb-4">
              {{ selectedTextForExplanation }}
            </blockquote>
            <div v-html="explanationContent"></div>
          </div>

          <div class="card-actions justify-end mt-4">
            <button class="btn btn-sm btn-primary" @click="copyExplanation" v-if="!isExplaining && explanationContent">
              复制解释
            </button>
            <button class="btn btn-sm btn-primary" @click="insertExplanationToContent"
              v-if="!isExplaining && explanationContent">
              插入到笔记
            </button>
            <button class="btn btn-sm btn-ghost" @click="showExplanationBox = false">
              关闭
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 在模板中添加翻译结果的浮动框 -->
    <div v-if="showTranslationBox" class="fixed inset-0 flex items-center justify-center z-50 bg-base-300/50">
      <div class="card bg-base-100 shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col">
        <div class="card-body p-4">
          <div class="flex justify-between items-start mb-4">
            <h2 class="card-title">翻译结果</h2>
            <button class="btn btn-sm btn-ghost" @click="showTranslationBox = false">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <div v-if="isTranslating" class="flex items-center gap-2 mb-4">
            <span class="loading loading-spinner loading-md"></span>
            <p>AI 正在翻译...</p>
          </div>

          <div v-else class="overflow-y-auto max-h-[60vh] prose prose-sm md:prose-base dark:prose-invert">
            <blockquote class="bg-base-200 p-3 rounded-lg mb-4">
              {{ selectedTextForTranslation }}
            </blockquote>
            <div v-html="translationContent"></div>
          </div>

          <div class="card-actions justify-end mt-4">
            <button class="btn btn-sm btn-primary" @click="copyTranslation" v-if="!isTranslating && translationContent">
              复制翻译
            </button>
            <button class="btn btn-sm btn-primary" @click="insertTranslationToContent"
              v-if="!isTranslating && translationContent">
              插入到笔记
            </button>
            <button class="btn btn-sm btn-ghost" @click="showTranslationBox = false">
              关闭
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 图片放大模态框 -->
    <div v-if="showImageModal" class="fixed inset-0 flex items-center justify-center z-50 bg-black/80"
      @click="closeImageModal">
      <div class="relative max-w-[95vw] max-h-[95vh] flex items-center justify-center" @click.stop>
        <img :src="modalImageSrc" :alt="modalImageAlt"
          class="max-w-full max-h-full object-contain rounded-lg shadow-2xl" @load="onModalImageLoad"
          @error="onModalImageError" />

        <!-- 关闭按钮 -->
        <button class="absolute top-4 right-4 btn btn-sm btn-circle btn-ghost bg-black/50 text-white hover:bg-black/70"
          @click="closeImageModal">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>

        <!-- 图片信息 -->
        <div v-if="modalImageAlt"
          class="absolute bottom-4 left-4 right-4 bg-black/50 text-white p-2 rounded text-center text-sm">
          {{ modalImageAlt }}
        </div>

        <!-- 加载指示器 -->
        <div v-if="modalImageLoading" class="absolute inset-0 flex items-center justify-center bg-black/50 rounded-lg">
          <span class="loading loading-spinner loading-lg text-white"></span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, defineProps, defineEmits, nextTick, onMounted, onActivated, onBeforeUnmount } from 'vue'
import DOMPurify from 'dompurify'
import { invoke } from '@tauri-apps/api/core'
import TagSelector from './TagSelector.vue'
// 使用 marked 和 Prism 替代 highlight.js
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import Prism from 'prismjs'
// 导入 Prism 的核心样式和主题
import 'prismjs/themes/prism-tomorrow.css'
import 'prismjs/themes/prism-dark.css'
// 导入行号插件
import 'prismjs/plugins/line-numbers/prism-line-numbers.css'
import 'prismjs/plugins/line-numbers/prism-line-numbers'
// 导入工具栏插件（用于复制按钮）
import 'prismjs/plugins/toolbar/prism-toolbar.css'
import 'prismjs/plugins/toolbar/prism-toolbar'
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard'
// 静态导入所需的 Prism 语言组件

import 'prismjs/components/prism-markup-templating'
import 'prismjs/components/prism-markup'
import 'prismjs/components/prism-css'

import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'

import 'prismjs/components/prism-python'
import 'prismjs/components/prism-java'
import 'prismjs/components/prism-go'

import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-sql'
import 'prismjs/components/prism-yaml'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-php'
import 'prismjs/components/prism-csharp'

// 导入 plaintext 支持（通常 Prism 将 plaintext 作为基础语言支持）
// 注意：plaintext 通常不需要特殊的语言组件，因为它就是纯文本

// 安全检查 Prism 语言是否可用
function isPrismLanguageAvailable(lang: string): boolean {
  try {
    // plaintext 总是可用的，因为它不需要特殊的语法高亮
    if (lang === 'plaintext' || lang === 'text' || lang === 'plain') {
      return true;
    }
    
    return !!(
      typeof Prism !== 'undefined' && 
      Prism.languages && 
      typeof Prism.languages === 'object' &&
      Prism.languages[lang] &&
      typeof Prism.highlight === 'function'
    );
  } catch (error) {
    console.warn(`检查 Prism 语言 ${lang} 时出错:`, error);
    return false;
  }
}

// 简化的语言组件初始化函数
async function loadPrismLanguages() {
  try {
    // 检查已加载的语言
    const loadedLanguages = typeof Prism !== 'undefined' ? Object.keys(Prism.languages || {}) : [];
    console.log('Prism 语言组件已静态加载:', loadedLanguages);
    console.log('Prism 语言组件加载完成');
  } catch (error) {
    console.error('检查 Prism 语言组件失败:', error);
  }
}

interface Tag {
  id: string;
  name: string;
}

interface Note {
  id: string;
  title: string;
  content: string;
  created_at: number;
  updated_at: number;
  tags: Tag[];
  isPinned?: boolean;
  category_id?: string;
  images?: { [key: string]: string }; // 添加图片存储
}

// 组件属性
const props = defineProps({
  note: {
    type: Object as () => Note,
    required: true
  }
})

// 组件事件
const emit = defineEmits([
  'update',
  'delete-note',
  'duplicate-note',
  'add-tag',
  'remove-tag',
  'toggle-pin'
])

// 状态
const localNote = ref<Note>({ ...props.note })
const isPreviewMode = ref(false)
const editorTextarea = ref<HTMLTextAreaElement | null>(null)
const autoSaveTimeout = ref<number | null>(null)
const showContextMenu = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
const isAIProcessing = ref(false)
const isEditOnly = ref(false)
const isSplitMode = ref(true)
const streamingContent = ref('')  // 用于存储流式输出的内容
const isStreaming = ref(false)    // 是否正在流式输出
const currentStreamingId = ref<string | null>(null)  // 当前流式输出的ID

// 添加图片加载相关状态
const isLoadingImages = ref(false)
const imageLoadCache = ref<Map<string, Record<string, string>>>(new Map())
const imageLoadTimeouts = ref<Map<string, number>>(new Map())

const hasSelectedText = computed(() => {
  const textarea = editorTextarea.value
  if (!textarea) return false

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  return start !== end
})
const showExplanationBox = ref(false)
const explanationContent = ref('')
const selectedTextForExplanation = ref('')
const isExplaining = ref(false)
const showTranslationBox = ref(false)
const translationContent = ref('')
const selectedTextForTranslation = ref('')
const isTranslating = ref(false)
let globalUnlisten: (() => void) | null = null; // 全局事件监听器引用

// 添加撤销/重做堆栈
const undoStack = ref<string[]>([])
const redoStack = ref<string[]>([])
const lastSavedContent = ref<string>('')

// 初始化时保存初始内容到撤销栈
onMounted(() => {
  undoStack.value = [localNote.value.content]
  lastSavedContent.value = localNote.value.content
})

// 优化的图片加载函数
async function loadNoteImages(noteId: string, timeout: number = 5000): Promise<Record<string, string> | null> {
  // 检查缓存
  if (imageLoadCache.value.has(noteId)) {
    console.log(`从缓存加载笔记(${noteId})的图片`)
    return imageLoadCache.value.get(noteId) || null
  }

  // 检查是否已有相同请求在进行
  if (imageLoadTimeouts.value.has(noteId)) {
    console.log(`笔记(${noteId})的图片正在加载中，跳过重复请求`)
    return null
  }

  try {
    isLoadingImages.value = true
    console.log(`开始异步加载笔记(${noteId})的图片...`)

    // 创建超时Promise
    const timeoutPromise = new Promise<never>((_, reject) => {
      const timeoutId = setTimeout(() => {
        reject(new Error(`图片加载超时(${timeout}ms)`))
      }, timeout) as unknown as number

      imageLoadTimeouts.value.set(noteId, timeoutId)
    })

    // 先获取图片总数
    const countPromise = invoke('get_tip_images_count', { tip_id: noteId }) as Promise<number>
    const totalCount = await Promise.race([countPromise, timeoutPromise])

    if (totalCount === 0) {
      console.log(`笔记(${noteId})没有图片`)
      imageLoadCache.value.set(noteId, {})
      return {}
    }

    // 分批加载图片，每次最多加载5张
    const batchSize = 5
    const allImages: Record<string, string> = {}

    for (let offset = 0; offset < totalCount; offset += batchSize) {
      const batchPromise = invoke('get_tip_images', {
        tip_id: noteId,
        limit: batchSize,
        offset: offset
      }) as Promise<Record<string, string>>

      const batchImages = await Promise.race([batchPromise, timeoutPromise])
      Object.assign(allImages, batchImages)

      console.log(`加载笔记(${noteId})图片批次 ${Math.floor(offset / batchSize) + 1}/${Math.ceil(totalCount / batchSize)}`)

      // 如果是第一批，立即更新界面显示
      if (offset === 0 && Object.keys(batchImages).length > 0) {
        // 检查当前笔记是否还是目标笔记
        if (localNote.value.id === noteId) {
          localNote.value.images = { ...batchImages }
          console.log(`首批图片已显示，笔记(${noteId})`)
        }
      }
    }

    console.log(`获取到笔记(${noteId})的图片总数: ${Object.keys(allImages).length}`)

    // 缓存完整结果
    imageLoadCache.value.set(noteId, allImages)

    return allImages
  } catch (error) {
    console.error(`加载笔记(${noteId})图片失败:`, error)

    // 如果是超时错误，缓存空结果避免重复请求
    if (error instanceof Error && error.message.includes('超时')) {
      imageLoadCache.value.set(noteId, {})
      console.warn(`笔记(${noteId})图片加载超时，已缓存空结果`)
    }

    return null
  } finally {
    // 清理超时定时器
    const timeoutId = imageLoadTimeouts.value.get(noteId)
    if (timeoutId) {
      clearTimeout(timeoutId)
      imageLoadTimeouts.value.delete(noteId)
    }

    isLoadingImages.value = false
  }
}

// 异步加载图片，不阻塞界面
function loadImagesAsync(noteId: string) {
  // 使用nextTick确保在下一个事件循环中执行
  nextTick(async () => {
    const images = await loadNoteImages(noteId, 3000) // 3秒超时

    // 检查当前笔记是否还是目标笔记（避免切换过快导致的状态错乱）
    if (localNote.value.id === noteId && images && Object.keys(images).length > 0) {
      localNote.value.images = images
      console.log(`异步加载完成，笔记(${noteId})图片已更新到本地状态`)
    }
  })
}



// 监听外部note变化 - 优化版本
watch(() => props.note, async (newNote, oldNote) => {
  // 如果是初始化（oldNote为undefined）或者笔记ID发生变化（切换到不同的笔记），才完全重新设置localNote
  if (!oldNote || oldNote.id !== newNote.id) {
    // 深拷贝对象，保留图片数据
    const images = newNote.images ? { ...newNote.images } : undefined;
    localNote.value = { ...newNote, images };

    // 如果笔记有ID但没有images数据，异步加载图片（不阻塞界面）
    if (newNote.id && !newNote.images) {
      // 立即显示笔记内容，图片稍后异步加载
      console.log(`笔记(${newNote.id})将异步加载图片，不阻塞界面显示`)
      loadImagesAsync(newNote.id)
    }
  }
  // 如果是同一个笔记的更新，只更新非编辑相关的字段（如category_id等）
  else {
    // 只更新非内容相关的字段，避免覆盖用户正在编辑的内容
    if (newNote.category_id !== localNote.value.category_id) {
      localNote.value.category_id = newNote.category_id;
    }
    if (newNote.tags && JSON.stringify(newNote.tags) !== JSON.stringify(localNote.value.tags)) {
      localNote.value.tags = newNote.tags;
    }
  }
}, { immediate: true, deep: true })

// 添加键盘快捷键处理函数
function handleKeyDown(event: KeyboardEvent) {
  // 检查是否按下了Ctrl键(Windows)或Command键(Mac)
  const isCtrlOrCmd = event.ctrlKey || event.metaKey

  // 撤销: Ctrl+Z
  if (isCtrlOrCmd && event.key === 'z' && !event.shiftKey) {
    event.preventDefault()
    undo()
    return
  }

  // 重做: Ctrl+Y 或 Ctrl+Shift+Z
  if ((isCtrlOrCmd && event.key === 'y') ||
    (isCtrlOrCmd && event.shiftKey && event.key === 'z')) {
    event.preventDefault()
    redo()
    return
  }

  // 粗体: Ctrl+B
  if (isCtrlOrCmd && event.key === 'b') {
    event.preventDefault()
    insertMarkdown('**', '**')
    return
  }

  // 斜体: Ctrl+I
  if (isCtrlOrCmd && event.key === 'i') {
    event.preventDefault()
    insertMarkdown('*', '*')
    return
  }

  // 链接: Ctrl+K
  if (isCtrlOrCmd && event.key === 'k') {
    event.preventDefault()
    insertMarkdown('[', '](https://)')
    return
  }

  // 代码块: Ctrl+Shift+C
  if (isCtrlOrCmd && event.shiftKey && event.key === 'c') {
    event.preventDefault()
    insertMarkdown('```\n', '\n```')
    return
  }

  // 任务列表: Ctrl+Shift+T
  if (isCtrlOrCmd && event.shiftKey && event.key === 't') {
    event.preventDefault()
    insertMarkdown('- [ ] ')
    return
  }

  // 保存: Ctrl+S
  if (isCtrlOrCmd && event.key === 's') {
    event.preventDefault()
    saveNoteToList()
    return
  }

  // 对于其他内容修改按键，添加到撤销堆栈
  // 避免在每次按键都保存，仅在内容实际变化时
  setTimeout(() => {
    const currentContent = localNote.value.content
    if (currentContent !== lastSavedContent.value) {
      // 添加到撤销堆栈
      undoStack.value.push(currentContent)
      // 清空重做堆栈
      redoStack.value = []
      // 更新最后保存的内容
      lastSavedContent.value = currentContent

      // 限制撤销堆栈大小以避免内存问题
      if (undoStack.value.length > 100) {
        undoStack.value = undoStack.value.slice(-100)
      }
    }
  }, 100)
}

// 撤销函数
function undo() {
  if (undoStack.value.length <= 1) return // 至少保留一个初始状态

  // 将当前内容保存到重做堆栈
  redoStack.value.push(localNote.value.content)

  // 移除当前状态，获取上一个状态
  undoStack.value.pop()
  const previousContent = undoStack.value[undoStack.value.length - 1]

  // 更新编辑器内容
  localNote.value.content = previousContent
  lastSavedContent.value = previousContent

  // 触发自动保存，但使用延迟，避免频繁保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }
  autoSaveTimeout.value = setTimeout(() => {
    emit('update', { ...localNote.value, _contentOnly: true })
  }, 1000) as unknown as number
}

// 重做函数
function redo() {
  if (redoStack.value.length === 0) return

  // 获取下一个状态
  const nextContent = redoStack.value.pop() as string

  // 将当前内容保存到撤销堆栈
  undoStack.value.push(nextContent)

  // 更新编辑器内容
  localNote.value.content = nextContent
  lastSavedContent.value = nextContent

  // 触发自动保存，但使用延迟，避免频繁保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }
  autoSaveTimeout.value = setTimeout(() => {
    emit('update', { ...localNote.value, _contentOnly: true })
  }, 1000) as unknown as number
}

// 计算属性
const renderedContent = computed(() => {
  if (!localNote.value.content) return ''

  try {
    // 首先替换本地图片引用
    let processedContent = localNote.value.content

    // 如果笔记中有图片，处理本地图片引用
    if (localNote.value.images) {
      // 匹配 ![xxx](local://img_id) 格式的图片引用
      const localImageRegex = /!\[([^\]]*)\]\(local:\/\/([^)]+)\)/g

      processedContent = processedContent.replace(localImageRegex, (match, alt, imageId) => {
        // 检查图片ID是否存在于images对象中
        if (localNote.value.images && localNote.value.images[imageId]) {
          // 返回HTML图片标签，使用base64数据，添加响应式类名
          return `<img src="${localNote.value.images[imageId]}" alt="${alt || '图片'}" class="embedded-image responsive-image" loading="lazy" />`
        }
        // 如果找不到图片，保持原样
        return match
      })
    }

    // 创建 marked 实例并配置高亮
    const marked = new Marked();
    
    // 使用 marked-highlight 扩展
    marked.use(markedHighlight({
      langPrefix: 'language-',
      highlight(code: string, lang: string) {
        // 如果没有指定语言，使用 plaintext 作为默认语言
        const actualLang = lang || 'plaintext';
        
        // 使用安全检查函数
        if (actualLang && isPrismLanguageAvailable(actualLang)) {
          try {
            return Prism.highlight(code, Prism.languages[actualLang], actualLang);
          } catch (error) {
            console.warn(`Prism 高亮失败 (${actualLang}):`, error);
            return escapeHtml(code);
          }
        }
        
        // 如果 plaintext 也不可用，直接返回转义的代码
        return escapeHtml(code);
      }
    }));

    // 配置 marked 选项
    marked.setOptions({
      breaks: true,
      gfm: true,
      silent: true,
    })

    // 使用 marked 渲染 Markdown
    const htmlContent = marked.parse(processedContent) as string

    // 使用DOMPurify清理HTML，防止XSS，但允许安全的HTML标签和图片
    const cleanHtml = DOMPurify.sanitize(htmlContent, {
      ADD_TAGS: ['iframe', 'pre', 'code'],
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled', 'data-code', 'data-language']
    })

    // 在下一个 tick 中处理代码块UI增强
    nextTick(() => {
      enhanceCodeBlocks()
    })

    return cleanHtml
  } catch (err) {
    console.error('Markdown渲染错误:', err)
    const errorMessage = err instanceof Error ? err.message : String(err)
    return `<div class="text-error">Markdown渲染错误: ${errorMessage}</div>
            <pre>${DOMPurify.sanitize(localNote.value.content)}</pre>`
  }
})

// HTML 转义函数
function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

const wordCount = computed(() => {
  if (!localNote.value.content) return 0
  // 简单的字数统计
  return localNote.value.content
    .replace(/\s+/g, ' ')
    .replace(/[\r\n]/g, '')
    .trim()
    .length
})

// 方法
function autoSave() {
  // 防抖自动保存
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
  }

  autoSaveTimeout.value = setTimeout(() => {
    // 更新本地状态，但暂不触发外部更新
    localNote.value.updated_at = Date.now()

    // 判断是否只更新标题
    if (localNote.value.content === lastSavedContent.value && localNote.value.title !== lastSavedContent.value) {
      // 只更新标题
      emit('update', { ...localNote.value, _titleOnly: true })
    } else {
      // 当内容变化时，仅更新内容但不触发列表重排序
      // 使用_contentOnly标记表示这是内容更新，不需要列表重排序
      emit('update', { ...localNote.value, _contentOnly: true })
    }
  }, 1000) as unknown as number // 延长防抖时间到1秒
}

// 当编辑器失去焦点时调用，将更新传递给父组件
function saveNoteToList() {
  // 清除任何未完成的自动保存计时器
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
    autoSaveTimeout.value = null
  }

  // 添加一个标记，表明这次更新是由失去焦点事件触发的
  // 允许父组件据此决定是否需要重新排序列表
  const noteToUpdate = { ...localNote.value, _fromBlur: true }

  // 如果笔记中包含图片，确保图片数据也被正确传递
  if (localNote.value.images && Object.keys(localNote.value.images).length > 0) {
    noteToUpdate.images = { ...localNote.value.images }
  }

  emit('update', noteToUpdate)
}

function insertMarkdown(prefix: string, suffix: string = '') {
  if (isPreviewMode.value) return

  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selectedText = localNote.value.content.substring(start, end)

  // 插入markdown标记
  const newText =
    localNote.value.content.substring(0, start) +
    prefix + selectedText + suffix +
    localNote.value.content.substring(end)

  localNote.value.content = newText

  // 更新后重新设置光标位置
  nextTick(() => {
    textarea.focus()
    if (selectedText.length > 0) {
      textarea.selectionStart = start + prefix.length
      textarea.selectionEnd = end + prefix.length
    } else {
      textarea.selectionStart = textarea.selectionEnd = start + prefix.length
    }
  })

  autoSave()
}

function handleContextMenu(event: MouseEvent) {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 防止默认菜单显示
  event.preventDefault()
  event.stopPropagation() // 阻止事件冒泡


  // 获取鼠标点击相对于编辑器的位置
  const editorRect = textarea.getBoundingClientRect()

  // 计算相对于编辑器内部的坐标
  contextMenuX.value = event.clientX - editorRect.left
  contextMenuY.value = event.clientY - editorRect.top

  // 显示右键菜单
  showContextMenu.value = true

  // 确保菜单不会超出编辑器边界
  nextTick(() => {
    const menu = document.querySelector('.context-menu') as HTMLElement
    if (!menu) return

    const menuRect = menu.getBoundingClientRect()

    // 如果菜单超出右边界，将其向左移动
    if (contextMenuX.value + menuRect.width > editorRect.width) {
      contextMenuX.value = Math.max(0, editorRect.width - menuRect.width - 5)
    }

    // 如果菜单超出下边界，将其向上移动
    if (contextMenuY.value + menuRect.height > editorRect.height) {
      contextMenuY.value = Math.max(0, editorRect.height - menuRect.height - 5)
    }

    // 点击其他区域关闭菜单
    const closeContextMenu = (e: MouseEvent) => {
      e.stopPropagation() // 阻止事件冒泡

      // 如果点击的不是菜单内部的元素，则关闭菜单
      const menu = document.querySelector('.context-menu')
      if (menu && !menu.contains(e.target as Node)) {
        showContextMenu.value = false
        document.removeEventListener('mousedown', closeContextMenu, true)
      }
    }

    // 使用mousedown事件而不是click事件，避免与菜单点击冲突
    // 使用捕获阶段监听点击事件，确保先于其他事件处理器
    document.addEventListener('mousedown', closeContextMenu, true)
  })
}

async function expandSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    alert('请先选择一段文本')
    return
  }

  const selectedText = localNote.value.content.substring(start, end)

  // 使用AI扩充选中的文本
  await expandWithAI(selectedText, start, end)
}

// 添加解释功能函数
async function explainSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    alert('请先选择一段文本')
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForExplanation.value = selectedText
  explanationContent.value = ''
  showExplanationBox.value = true
  showContextMenu.value = false

  // 使用AI解释选中的文本
  await processExplanation(selectedText)
}

// 根据笔记标题或选中内容使用AI扩充
async function expandWithAI(input?: string, start?: number, end?: number) {
  const textarea = editorTextarea.value
  let promptText = input || ''
  let startPos = start || 0
  let endPos = end || 0

  // 如果没有传入参数，检查是否有选中的文本
  if (!input) {
    if (textarea) {
      startPos = textarea.selectionStart
      endPos = textarea.selectionEnd

      if (startPos !== endPos) {
        // 使用选中的文本
        promptText = localNote.value.content.substring(startPos, endPos)
      } else {
        // 使用标题作为提示
        promptText = localNote.value.title
        if (!promptText) {
          alert('请先输入笔记标题或选择要扩充的文本')
          return
        }

        // 追加到末尾
        startPos = localNote.value.content.length
        endPos = localNote.value.content.length
      }
    } else {
      // 使用标题作为提示
      promptText = localNote.value.title
      if (!promptText) {
        alert('请先输入笔记标题或选择要扩充的文本')
        return
      }

      // 追加到末尾
      startPos = localNote.value.content.length
      endPos = localNote.value.content.length
    }
  }

  // 构建提示
  const expandPrompt = `请基于以下内容进行扩充和完善：\n\n${promptText}`

  // 处理AI请求
  await processWithAI(promptText, expandPrompt, false, startPos, endPos)
}

// 通用AI处理函数 - 处理不同类型的AI请求
async function processWithAI(originalText: string, prompt: string, appendResult = false, startPos?: number, endPos?: number) {
  try {
    isAIProcessing.value = true
    showContextMenu.value = false

    // 重置流式输出状态
    streamingContent.value = ''
    isStreaming.value = true

    // 先清理旧的事件监听器
    if (globalUnlisten) {
      try {
        globalUnlisten();
        globalUnlisten = null;
      } catch (e) {
        console.error('清理旧事件监听器失败:', e);
      }
    }

    // 生成唯一的流ID
    currentStreamingId.value = `stream_${Date.now()}`
    console.log(`生成流ID: ${currentStreamingId.value}`)

    // 获取选择的AI模型
    const selectedModel = localStorage.getItem('mytips-default-ai-model') || 'gemini'

    // 检查API密钥
    try {
      const hasApiKey = await invoke('has_api_key', { modelId: selectedModel })
      if (!hasApiKey) {
        throw new Error(`未配置${selectedModel}模型的API密钥，请前往AI助手页面进行设置`)
      }
    } catch (e) {
      console.error('API密钥检查失败:', e)
      throw new Error('无法验证API密钥，请前往AI助手页面进行设置')
    }

    // 在发送API请求前设置事件监听器
    const { listen } = await import('@tauri-apps/api/event')
    globalUnlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean }

      // 安全检查：确保我们仍在处理中且ID匹配
      if (!isStreaming.value || !currentStreamingId.value) {
        console.log(`流处理已取消或重置，忽略事件`)
        return
      }

      // 确保只处理当前流ID的事件
      if (payload.id !== currentStreamingId.value) {
        console.log(`忽略不匹配的流ID: ${payload.id}, 当前ID: ${currentStreamingId.value}`)
        return
      }

      console.log(`收到流chunk: id=${payload.id}, 长度=${payload.chunk?.length || 0}, done=${payload.done}`)

      if (payload.chunk) {
        // 更新流式内容
        streamingContent.value += payload.chunk

        // 如果不需要实时更新编辑器内容（如解释模式），则不更新编辑器
        if (!appendResult) {
          updateEditorContent(startPos, endPos);
        }
      }

      // 如果完成了，清理并保存
      if (payload.done) {
        console.log('流式输出完成，清理资源')

        // 如果是解释模式，将结果追加到末尾
        if (appendResult) {
          const prefix = localNote.value.content.length > 0 ? '\n\n' : '';
          const explanation = `> 💡 ${originalText}\n\n${streamingContent.value}`;
          localNote.value.content += prefix + explanation;

          // 更新编辑器内容
          nextTick(() => {
            if (editorTextarea.value) {
              editorTextarea.value.value = localNote.value.content;
              editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }));

              // 滚动到底部
              editorTextarea.value.scrollTop = editorTextarea.value.scrollHeight;
            }
          });
        }

        cleanupStream()
        saveNoteToList()
      }
    })

    // 发送AI请求
    console.log(`使用模型${selectedModel}发送请求，提示文本长度: ${prompt.length}字符`)
    await invoke('send_ai_message_stream', {
      modelId: selectedModel,
      message: prompt,
      streamId: currentStreamingId.value,
      messages: undefined,
      customModelName: undefined
    })

    console.log('AI请求已发送，等待流式响应...')

  } catch (error) {
    console.error('AI处理失败:', error)
    alert('AI处理失败: ' + error)
    cleanupStream()
  }
}

// 更新编辑器内容的函数 - 从processWithAI中提取出来
function updateEditorContent(startPos?: number, endPos?: number) {
  if (startPos === undefined || endPos === undefined) return;

  // 记录当前的滚动位置
  let scrollTop = 0
  if (editorTextarea.value) {
    scrollTop = editorTextarea.value.scrollTop
  }

  // 更新编辑器内容
  if (startPos !== endPos) {
    // 替换选中的文本
    localNote.value.content =
      localNote.value.content.substring(0, startPos) +
      streamingContent.value +
      localNote.value.content.substring(endPos)

    // 更新结束位置以反映新内容的长度
    endPos = startPos + streamingContent.value.length
  } else {
    // 追加到内容末尾
    const contentPrefix = localNote.value.content.length > 0 ? '\n\n' : ''
    const prefixLength = contentPrefix.length

    localNote.value.content =
      localNote.value.content.substring(0, startPos) +
      contentPrefix +
      streamingContent.value

    // 更新结束位置以反映新内容的长度
    endPos = startPos + prefixLength + streamingContent.value.length
  }

  // 强制更新编辑器内容
  nextTick(() => {
    if (editorTextarea.value) {
      if (editorTextarea.value.value !== localNote.value.content) {
        // 直接设置值
        editorTextarea.value.value = localNote.value.content

        // 模拟输入事件以触发其他可能的监听器
        editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }))

        // 恢复滚动位置
        editorTextarea.value.scrollTop = scrollTop

        // 设置光标位置到更新内容的末尾
        editorTextarea.value.setSelectionRange(endPos, endPos)

        // 如果处于编辑模式，保持焦点
        if (isEditOnly.value || isSplitMode.value || !isPreviewMode.value) {
          editorTextarea.value.focus()
        }
      }
    }
  })
}

function cleanupStream() {
  if (globalUnlisten) {
    try {
      globalUnlisten()
      globalUnlisten = null
      console.log('已清理事件监听器')
    } catch (e) {
      console.error('清理事件监听器失败:', e)
    }
  }

  isStreaming.value = false
  currentStreamingId.value = null
  isAIProcessing.value = false
}

async function cancelAIGeneration() {
  if (isStreaming.value && currentStreamingId.value) {
    try {
      console.log(`取消生成: ${currentStreamingId.value}`)
      await invoke('cancel_ai_generation', {
        streamId: currentStreamingId.value
      })

      // 更新编辑器内容以确保显示当前生成的内容
      nextTick(() => {
        if (editorTextarea.value) {
          editorTextarea.value.value = localNote.value.content
          editorTextarea.value.dispatchEvent(new Event('input', { bubbles: true }))
        }
      })

      // 保存当前已生成的内容
      saveNoteToList()

    } catch (error) {
      console.error('取消AI生成失败:', error)
    } finally {
      cleanupStream()
    }
  }
}

function setEditMode(mode: string) {
  if (mode === 'editOnly') {
    isEditOnly.value = true
    isPreviewMode.value = false
    isSplitMode.value = false
  } else if (mode === 'preview') {
    isEditOnly.value = false
    isPreviewMode.value = true
    isSplitMode.value = false
  } else if (mode === 'split') {
    isEditOnly.value = false
    isPreviewMode.value = false
    isSplitMode.value = true
  }
}

// 添加copySelectedText函数
async function copySelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start !== end) {
    const selectedText = localNote.value.content.substring(start, end)
    try {
      await navigator.clipboard.writeText(selectedText)
      showContextMenu.value = false
    } catch (error) {
      console.error('复制到剪贴板失败:', error)
    }
  }

  showContextMenu.value = false
}

// 添加点击事件监听器以关闭右键菜单
function setupDocumentClickListener() {
  document.addEventListener('click', () => {
    showContextMenu.value = false
  })
}

// 修改handlePaste函数
async function handlePaste(event: ClipboardEvent) {
  // 检查是否包含图片
  const items = event.clipboardData?.items
  if (!items) return

  let hasImage = false

  // 检查剪贴板中的所有项
  for (let i = 0; i < items.length; i++) {
    const item = items[i]

    // 如果是图片类型
    if (item.type.indexOf('image') !== -1) {
      hasImage = true

      // 防止默认粘贴行为
      event.preventDefault()

      // 获取文件
      const file = item.getAsFile()
      if (!file) continue

      try {
        // 显示处理中状态
        isAIProcessing.value = true

        // 将图片转换为Base64
        const base64Image = await convertImageToBase64(file)
        console.log(`图片转换为Base64格式成功，长度: ${base64Image.length}`)

        // 生成唯一ID
        const imageId = `img_${Date.now()}_${Math.floor(Math.random() * 1000)}`
        console.log(`生成图片ID: ${imageId}`)

        // 确保笔记已保存（有ID）
        if (!localNote.value.id) {
          throw new Error('请先保存笔记再粘贴图片')
        }
        console.log(`笔记ID: ${localNote.value.id}，准备保存图片`)

        // 保存图片到数据库
        console.log(`调用save_tip_image API，参数: tip_id=${localNote.value.id}, image_id=${imageId}`)
        await invoke('save_tip_image', {
          imageData: {
            tip_id: localNote.value.id,
            image_id: imageId,
            image_data: base64Image
          }
        })
        console.log('图片已成功保存到数据库')

        // 确保images对象存在
        if (!localNote.value.images) {
          localNote.value.images = {}
        }

        // 保存图片到本地状态
        localNote.value.images[imageId] = base64Image

        // 在光标位置插入Markdown图片引用
        const textarea = editorTextarea.value
        if (textarea) {
          const start = textarea.selectionStart
          const end = textarea.selectionEnd

          // 构建Markdown图片引用，使用本地ID引用图片
          const imageMarkdown = `![图片](local://${imageId})`

          // 在光标位置插入
          localNote.value.content =
            localNote.value.content.substring(0, start) +
            imageMarkdown +
            localNote.value.content.substring(end)

          // 更新界面 - 确保编辑器内容更新
          nextTick(() => {
            if (textarea) {
              textarea.value = localNote.value.content
              textarea.dispatchEvent(new Event('input', { bubbles: true }))

              // 设置光标位置到图片引用后
              const newCursorPosition = start + imageMarkdown.length
              textarea.setSelectionRange(newCursorPosition, newCursorPosition)
              textarea.focus()
            }
          })

          // 立即更新编辑器状态以显示图片
          autoSave()

          // 确保笔记被保存到列表
          saveNoteToList()
        }

        // 显示成功提示
        console.log('图片已保存到数据库，ID:', imageId)
      } catch (error) {
        console.error('处理粘贴图片失败:', error)

        // 获取详细的错误信息
        let errorMessage = '处理图片失败';
        if (error instanceof Error) {
          errorMessage = `${errorMessage}: ${error.message}`;
          console.error('错误详情:', error.stack);
        } else {
          errorMessage = `${errorMessage}: ${String(error)}`;
        }

        alert(errorMessage)
      } finally {
        isAIProcessing.value = false
      }

      // 只处理第一张图片
      break
    }
  }

  // 如果没有图片，则使用默认粘贴行为
  if (!hasImage) {
    return true
  }
}

// 将图片文件转换为Base64，重构为更可靠的实现
function convertImageToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = (event) => {
      if (event.target?.result) {
        const result = event.target.result as string
        // 确保结果是base64格式
        if (typeof result === 'string' && result.startsWith('data:')) {
          resolve(result)
        } else {
          reject(new Error('转换图片格式失败'))
        }
      } else {
        reject(new Error('读取图片失败'))
      }
    }

    reader.onerror = (error) => {
      reject(new Error('读取文件出错: ' + error))
    }

    // 确保以DataURL格式读取
    reader.readAsDataURL(file)
  })
}

// 添加这个新函数来处理解释
async function processExplanation(textToExplain: string) {
  try {
    isExplaining.value = true

    // 获取选择的AI模型
    const selectedModel = localStorage.getItem('mytips-default-ai-model') || 'gemini'

    // 检查API密钥
    try {
      const hasApiKey = await invoke('has_api_key', { modelId: selectedModel })
      if (!hasApiKey) {
        throw new Error(`未配置${selectedModel}模型的API密钥，请前往AI助手页面进行设置`)
      }
    } catch (e) {
      console.error('API密钥检查失败:', e)
      throw new Error('无法验证API密钥，请前往AI助手页面进行设置')
    }

    // 创建唯一的流ID
    const streamId = `explain_${Date.now()}`

    // 构建包含笔记标题的提示信息
    const noteTitle = localNote.value.title || '无标题笔记'
    const prompt = `请解释以下内容，简明扼要。这段内容来自笔记"${noteTitle}"：\n\n${textToExplain}`

    // 设置事件监听器来接收流式响应
    const { listen } = await import('@tauri-apps/api/event')
    let rawExplanation = ''
    const unlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean }

      // 确保ID匹配
      if (payload.id !== streamId) return

      if (payload.chunk) {
        // 累积解释内容
        rawExplanation += payload.chunk
        // 不再使用 marked，直接设置为带有段落标签的HTML
        explanationContent.value = `<p>${rawExplanation.replace(/\n\n/g, '</p><p>')}</p>`;
      }

      // 如果完成了，清理监听器
      if (payload.done) {
        isExplaining.value = false
        unlisten()
      }
    })

    // 发送AI请求
    await invoke('send_ai_message_stream', {
      modelId: selectedModel,
      message: prompt,
      streamId: streamId,
      messages: undefined,
      customModelName: undefined
    })

  } catch (error) {
    console.error('AI解释生成失败:', error)
    explanationContent.value = `<p class=\"text-error\">解释生成失败: ${error}</p>`
    isExplaining.value = false
  }
}

// 复制解释内容
async function copyExplanation() {
  try {
    await navigator.clipboard.writeText(explanationContent.value)
    // 可以添加一个复制成功的提示
  } catch (error) {
    console.error('复制到剪贴板失败:', error)
  }
}

// 将解释内容插入到笔记中
function insertExplanationToContent() {
  // 在光标位置或文档末尾插入解释内容
  const textarea = editorTextarea.value
  if (!textarea) return

  const prefix = '\n\n> 💡 解释：\n\n'
  const content = prefix + explanationContent.value

  const end = textarea.selectionEnd

  // 插入内容
  localNote.value.content =
    localNote.value.content.substring(0, end) +
    content +
    localNote.value.content.substring(end)

  // 更新光标位置
  nextTick(() => {
    textarea.selectionStart = end + content.length
    textarea.selectionEnd = end + content.length
    textarea.focus()
  })

  // 保存笔记
  autoSave()

  // 关闭解释框
  showExplanationBox.value = false
}

// 分享笔记功能
function shareNote() {
  // 实现分享功能
  alert(`分享链接已复制: ${window.location.origin}/note/${localNote.value.id}`)
}

// 导出笔记功能
function exportNote() {
  // 实现导出功能
  const filename = `${localNote.value.title || 'note'}.md`
  const content = localNote.value.content

  const blob = new Blob([content], { type: 'text/markdown' })
  const url = URL.createObjectURL(blob)

  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()

  URL.revokeObjectURL(url)
}

// 格式化日期时间
function formatDateTime(dateString: number): string {
  if (!dateString) return ''

  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 在组件属性下添加以下状态变量
const currentHighlightTheme = ref(localStorage.getItem('mytips-highlight-theme') || getDefaultHighlightTheme())

// 添加根据系统主题自动选择代码高亮主题的函数
function getDefaultHighlightTheme() {
  // 检查系统是否支持颜色模式查询
  if (window.matchMedia) {
    // 检查是否为暗色模式
    const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    return isDarkMode ? 'dark' : 'default'
  }

  // 默认使用暗色主题
  return 'dark'
}

// 设置代码复制功能
function setupCodeCopyFeature() {
  // 使用事件委托监听所有代码复制按钮的点击
  document.addEventListener('click', async (event) => {
    const target = event.target as HTMLElement
    const copyButton = target.closest('.copy-code-btn') as HTMLButtonElement

    if (copyButton) {
      try {
        const codeData = copyButton.getAttribute('data-code')
        if (!codeData) return

        const codeText = decodeURIComponent(codeData)
        await navigator.clipboard.writeText(codeText)

        // 显示复制成功状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        `
        copyButton.classList.add('text-success')
        copyButton.disabled = true

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-success')
          copyButton.disabled = false
        }, 2000)
      } catch (error) {
        console.error('复制代码失败:', error)

        // 显示复制失败状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        `
        copyButton.classList.add('text-error')

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-error')
        }, 2000)
      }
    }
  })
}

// 监听组件挂载，获取可用标签
onMounted(async () => {
  // 检查 Prism 初始状态
  console.log('Prism 初始状态:', {
    available: typeof Prism !== 'undefined',
    languages: typeof Prism !== 'undefined' ? Object.keys(Prism.languages || {}) : [],
    highlight: typeof Prism !== 'undefined' && typeof Prism.highlight === 'function'
  });

  // 首先加载 Prism 语言组件
  await loadPrismLanguages()

  // 加载完成后再次检查状态
  console.log('Prism 加载后状态:', {
    available: typeof Prism !== 'undefined',
    languages: typeof Prism !== 'undefined' ? Object.keys(Prism.languages || {}) : [],
    highlight: typeof Prism !== 'undefined' && typeof Prism.highlight === 'function'
  });

  // 设置文档点击监听器
  setupDocumentClickListener()

  // 设置代码复制功能
  setupCodeCopyFeature()

  // 设置图片点击放大功能
  setupImageClickHandler()

  // 加载默认主题
  const theme = currentHighlightTheme.value
  console.log(`初始化代码高亮主题: ${theme}`)

  // 动态加载主题样式表
  loadPrismTheme(theme)

  // 应用代码高亮主题
  setHighlightTheme(currentHighlightTheme.value)

  // 监听系统主题变化
  if (window.matchMedia) {
    const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

    // 添加监听器以响应系统主题变化
    const themeChangeHandler = (event: MediaQueryListEvent) => {
      // 如果用户没有手动设置主题，则自动切换
      if (!localStorage.getItem('mytips-highlight-theme-manual')) {
        const newTheme = event.matches ? 'dark' : 'default'
        setHighlightTheme(newTheme)
      }
    }

    // 根据浏览器兼容性使用不同的API
    if (darkModeMediaQuery.addEventListener) {
      darkModeMediaQuery.addEventListener('change', themeChangeHandler)
    } else if (darkModeMediaQuery.addListener) {
      // Safari和旧版浏览器支持
      darkModeMediaQuery.addListener(themeChangeHandler)
    }
  }
})

// 修改setHighlightTheme函数，移除hljs相关代码
function setHighlightTheme(theme: string) {
  currentHighlightTheme.value = theme
  localStorage.setItem('mytips-highlight-theme', theme)

  // 标记用户已手动选择主题
  localStorage.setItem('mytips-highlight-theme-manual', 'true')

  // 动态加载对应的Prism主题
  loadPrismTheme(theme)

  // 更新所有代码块的样式类
  nextTick(() => {
    const codeBlocks = document.querySelectorAll('.prose pre code')
    console.log(`应用主题: ${theme}, 找到 ${codeBlocks.length} 个代码块`)

    codeBlocks.forEach(codeBlock => {
      // 移除所有主题类
      codeBlock.classList.remove('prism-default', 'prism-dark', 'prism-tomorrow', 'prism-twilight', 'prism-okaidia', 'prism-solarizedlight', 'prism-funky')
      // 添加当前主题类
      codeBlock.classList.add(`prism-${theme}`)

      // 更新父级pre元素的主题类
      const preElement = codeBlock.closest('pre')
      if (preElement) {
        preElement.classList.remove('prism-default', 'prism-dark', 'prism-tomorrow', 'prism-twilight', 'prism-okaidia', 'prism-solarizedlight', 'prism-funky')
        preElement.classList.add(`prism-${theme}`)

        // 确保复制按钮背景色与当前主题匹配
        const codeBlockContainer = preElement.closest('.code-block-container')
        if (codeBlockContainer) {
          const copyButton = codeBlockContainer.querySelector('.copy-code-btn')
          if (copyButton) {
            // 调整复制按钮颜色以适应当前主题
            const isDarkTheme = ['dark', 'tomorrow', 'twilight', 'okaidia', 'funky'].includes(theme)
            if (isDarkTheme) {
              copyButton.classList.add('bg-base-100')
              copyButton.classList.remove('bg-base-300')
            } else {
              copyButton.classList.add('bg-base-300')
              copyButton.classList.remove('bg-base-100')
            }
          }
        }
      }
    })

    // 重新应用 Prism 高亮
    setTimeout(() => {
      Prism.highlightAll()
    }, 10)
  })
}

// 加载 Prism 主题的函数
function loadPrismTheme(theme: string) {
  // 移除之前可能加载的主题样式
  const existingThemeLinks = document.querySelectorAll('link[data-prism-theme]')
  existingThemeLinks.forEach(link => link.remove())

  // 根据主题名称选择对应的 Prism 主题文件
  let themeFile = 'prism-dark.css' // 默认主题
  switch (theme) {
    case 'dark':
      themeFile = 'prism-dark.css'
      break
    case 'tomorrow':
      themeFile = 'prism-tomorrow.css'
      break
    case 'twilight':
      themeFile = 'prism-twilight.css'
      break
    case 'okaidia':
      themeFile = 'prism-okaidia.css'
      break
    case 'solarizedlight':
      themeFile = 'prism-solarizedlight.css'
      break
    case 'funky':
      themeFile = 'prism-funky.css'
      break
    case 'default':
    default:
      themeFile = 'prism-dark.css'
  }

  // 创建新的样式链接
  const linkElement = document.createElement('link')
  linkElement.rel = 'stylesheet'
  linkElement.href = `https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/${themeFile}`
  linkElement.setAttribute('data-prism-theme', theme)

  // 添加到文档头部
  document.head.appendChild(linkElement)

  console.log(`已加载 Prism 代码高亮主题: ${theme}`)
}

// 添加插入表格的函数
function insertTable() {
  const tableTemplate = `| 表头1 | 表头2 | 表头3 |\n| --- | --- | --- |\n| 内容1 | 内容2 | 内容3 |\n| 内容4 | 内容5 | 内容6 |`;
  insertMarkdown(tableTemplate);
}

const previewDiv = ref<HTMLDivElement | null>(null)
const isScrollingEditor = ref(false)
const isScrollingPreview = ref(false)

// 处理编辑器滚动事件
function handleEditorScroll(event: Event) {
  if (isScrollingPreview.value) return;

  // 标记正在从编辑器滚动
  isScrollingEditor.value = true;

  // 获取滚动元素
  const editor = event.target as HTMLTextAreaElement;
  if (!editor || !previewDiv.value || !isSplitMode.value) return;

  // 计算滚动比例
  const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight);

  // 设置预览区的滚动位置
  const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
  previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;

  // 重置标记，延迟执行防止抖动
  setTimeout(() => {
    isScrollingEditor.value = false;
  }, 100);
}

// 处理预览区滚动事件
function handlePreviewScroll(event: Event) {
  if (isScrollingEditor.value) return;

  // 标记正在从预览区滚动
  isScrollingPreview.value = true;

  // 获取滚动元素
  const preview = event.target as HTMLDivElement;
  if (!preview || !editorTextarea.value || !isSplitMode.value) return;

  // 计算滚动比例
  const previewScrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight);

  // 设置编辑器的滚动位置
  const editorScrollable = editorTextarea.value.scrollHeight - editorTextarea.value.clientHeight;
  editorTextarea.value.scrollTop = previewScrollRatio * editorScrollable;

  // 重置标记，延迟执行防止抖动
  setTimeout(() => {
    isScrollingPreview.value = false;
  }, 100);
}

// 监听内容变化时重新计算滚动同步
watch(() => localNote.value.content, () => {
  // 内容变化后保持编辑器当前滚动位置的相对比例
  nextTick(() => {
    if (isSplitMode.value && editorTextarea.value && previewDiv.value) {
      const editor = editorTextarea.value;
      const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1);

      const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
      previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;
    }
  });
});

// 在切换模式时同步滚动位置
watch(() => isSplitMode.value, (newValue) => {
  if (newValue) {
    nextTick(() => {
      if (editorTextarea.value && previewDiv.value) {
        const editor = editorTextarea.value;
        const editorScrollRatio = editor.scrollTop / (editor.scrollHeight - editor.clientHeight || 1);

        const previewScrollable = previewDiv.value.scrollHeight - previewDiv.value.clientHeight;
        previewDiv.value.scrollTop = editorScrollRatio * previewScrollable;
      }
    });
  }
});

// 添加onActivated钩子
onActivated(() => {
  console.log('NoteEditor组件被激活')
  // 不重新加载数据，只确保编辑器状态与当前笔记同步
  if (editorTextarea.value && props.note) {
    // 仅在编辑器内容与笔记内容不一致时才更新
    const currentContent = editorTextarea.value.value
    if (currentContent !== props.note.content) {
      editorTextarea.value.value = props.note.content || ''
    }
  }
})

// 添加组件卸载时的清理逻辑
onBeforeUnmount(() => {
  console.log('NoteEditor组件即将卸载，清理资源')

  // 清理自动保存定时器
  if (autoSaveTimeout.value) {
    clearTimeout(autoSaveTimeout.value)
    autoSaveTimeout.value = null
  }

  // 清理AI流式输出相关资源
  if (globalUnlisten) {
    try {
      globalUnlisten()
      globalUnlisten = null
    } catch (e) {
      console.error('清理AI事件监听器失败:', e)
    }
  }

  // 清理图片加载相关资源
  imageLoadTimeouts.value.forEach((timeoutId) => {
    clearTimeout(timeoutId)
  })
  imageLoadTimeouts.value.clear()

  // 可选：清理图片缓存（如果需要释放内存）
  // imageLoadCache.value.clear()
})

// 添加图片放大模态框的逻辑
const showImageModal = ref(false)
const modalImageSrc = ref('')
const modalImageAlt = ref('')
const modalImageLoading = ref(false)

function onModalImageLoad() {
  modalImageLoading.value = false
}

function onModalImageError() {
  modalImageLoading.value = false
  alert('图片加载失败')
}

function closeImageModal() {
  showImageModal.value = false
}

// 设置图片点击放大功能
function setupImageClickHandler() {
  // 使用事件委托监听所有图片的点击
  document.addEventListener('click', (event) => {
    const target = event.target as HTMLElement

    // 检查点击的是否是图片
    if (target.tagName === 'IMG' && target.closest('.prose')) {
      event.preventDefault()
      event.stopPropagation()

      const img = target as HTMLImageElement

      // 获取图片信息
      modalImageSrc.value = img.src
      modalImageAlt.value = img.alt || '图片'
      modalImageLoading.value = true

      // 显示模态框
      showImageModal.value = true
    }
  })

  // 添加键盘快捷键支持
  document.addEventListener('keydown', (event) => {
    // ESC键关闭图片模态框
    if (event.key === 'Escape' && showImageModal.value) {
      closeImageModal()
    }
  })
}

// 添加翻译功能
async function translateSelectedText() {
  const textarea = editorTextarea.value
  if (!textarea) return

  const start = textarea.selectionStart
  const end = textarea.selectionEnd

  // 确保有选择的文本
  if (start === end) {
    alert('请先选择一段文本')
    return
  }

  const selectedText = localNote.value.content.substring(start, end)
  selectedTextForTranslation.value = selectedText
  translationContent.value = ''
  showTranslationBox.value = true
  showContextMenu.value = false

  // 使用AI翻译选中的文本
  await processTranslation(selectedText)
}

// 处理翻译
async function processTranslation(text: string) {
  isTranslating.value = true
  try {
    const isEnglish = /^[a-zA-Z0-9\s.,?!;:'"()\[\]{}<>\/\\|@#$%^&*_+=-]+$/.test(text)
    const prompt = isEnglish
      ? `请将以下英文翻译成中文：\n\n${text}`
      : `请将以下中文翻译成英文：\n\n${text}`
    const streamId = `translate_${Date.now()}`
    translationContent.value = ''
    const selectedModel = localStorage.getItem('mytips-default-ai-model') || 'gemini'
    try {
      const hasApiKey = await invoke('has_api_key', { modelId: selectedModel })
      if (!hasApiKey) {
        throw new Error(`未配置${selectedModel}模型的API密钥，请前往AI助手页面进行设置`)
      }
    } catch (e) {
      console.error('API密钥检查失败:', e)
      throw new Error('无法验证API密钥，请前往AI助手页面进行设置')
    }
    const { listen } = await import('@tauri-apps/api/event')
    let rawResult = ''
    const unlisten = await listen('ai-stream-chunk', (event: { payload: any }) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean }
      if (payload.id !== streamId) return
      if (payload.chunk) {
        rawResult += payload.chunk
        translationContent.value = `<p>${rawResult}</p>`
      }
      if (payload.done) {
        isTranslating.value = false
        unlisten()
      }
    })
    await invoke('send_ai_message_stream', {
      modelId: selectedModel,
      message: prompt,
      streamId: streamId,
      messages: undefined,
      customModelName: undefined
    })
  } catch (error) {
    console.error('翻译失败:', error)
    translationContent.value = `<p class=\"text-error\">翻译失败: ${error}</p>`
    isTranslating.value = false
  }
}

// 复制翻译内容
function copyTranslation() {
  // 使用临时元素提取HTML内容中的纯文本
  const tempElement = document.createElement('div')
  tempElement.innerHTML = translationContent.value
  const textContent = tempElement.textContent || ''

  // 复制到剪贴板
  navigator.clipboard.writeText(textContent)
    .then(() => {
      // 显示成功消息
      alert('翻译内容已复制到剪贴板')
    })
    .catch(err => {
      console.error('复制失败:', err)
      alert('复制失败，请手动选择并复制')
    })
}

// 将翻译结果插入到笔记内容
function insertTranslationToContent() {
  const textarea = editorTextarea.value
  if (!textarea) return

  // 获取纯文本内容
  const tempElement = document.createElement('div')
  tempElement.innerHTML = translationContent.value
  const textContent = tempElement.textContent || ''

  // 获取当前光标位置
  const cursorPos = textarea.selectionEnd

  // 插入翻译内容
  const newContent =
    localNote.value.content.substring(0, cursorPos) +
    '\n\n' + textContent + '\n\n' +
    localNote.value.content.substring(cursorPos)

  // 更新内容
  localNote.value.content = newContent

  // 设置新的光标位置
  nextTick(() => {
    textarea.focus()
    textarea.selectionStart = textarea.selectionEnd = cursorPos + textContent.length + 4 // +4 for the newlines
  })

  // 关闭翻译框
  showTranslationBox.value = false

  // 触发自动保存
  autoSave()
}

// 添加粘贴功能
async function pasteFromClipboard() {
  const textarea = editorTextarea.value
  if (!textarea) return

  try {
    // 读取剪贴板内容
    const text = await navigator.clipboard.readText()

    // 获取当前光标位置
    const cursorPos = textarea.selectionStart

    // 插入剪贴板内容
    const newContent =
      localNote.value.content.substring(0, cursorPos) +
      text +
      localNote.value.content.substring(textarea.selectionEnd)

    // 更新内容
    localNote.value.content = newContent

    // 设置新的光标位置
    nextTick(() => {
      textarea.focus()
      textarea.selectionStart = textarea.selectionEnd = cursorPos + text.length
    })

    // 关闭右键菜单
    showContextMenu.value = false

    // 触发自动保存
    autoSave()
  } catch (error) {
    console.error('粘贴失败:', error)
    alert('无法访问剪贴板，请使用键盘快捷键(Ctrl+V)粘贴')
  }
}

// 只更新标题
function onTitleBlur() {
  emit('update', { id: localNote.value.id, title: localNote.value.title, _titleOnly: true })
}

// 只更新内容
function onContentBlur() {
  emit('update', { id: localNote.value.id, content: localNote.value.content, updated_at: Date.now(), _contentOnly: true })
}

// 整个编辑器失焦时完整保存
function onEditorBlur(event: FocusEvent) {
  // 只有真正离开整个编辑器才触发
  if (!(event.currentTarget as HTMLElement).contains(event.relatedTarget as Node)) {
    emit('update', { ...localNote.value })
  }
}

// 增强代码块UI的函数
function enhanceCodeBlocks() {
  // 查找所有包含language-类的code元素，以及没有language-类的pre>code元素
  const codeElements = document.querySelectorAll('.prose code[class*="language-"], .prose pre > code:not([class*="language-"])')
  
  codeElements.forEach((codeElement) => {
    const pre = codeElement.closest('pre')
    if (!pre) return
    
    // 避免重复处理
    if (pre.closest('.code-block-container')) {
      return
    }

    // 获取语言类型
    const classNames = codeElement.className.split(' ')
    const langClass = classNames.find(cls => cls.startsWith('language-'))
    const lang = langClass ? langClass.replace('language-', '') : 'plaintext'

    // 如果没有指定语言，为code元素添加language-plaintext类
    if (!langClass) {
      codeElement.classList.add('language-plaintext')
    }

    // 获取原始代码内容
    const codeText = codeElement.textContent || ''

    // 创建容器
    const container = document.createElement('div')
    container.className = 'code-block-container'

    // 创建头部
    const header = document.createElement('div')
    header.className = 'code-block-header'
    header.innerHTML = `
      <span class="code-language">${lang}</span>
      <button class="copy-code-btn" data-code="${encodeURIComponent(codeText)}" title="复制代码">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
        </svg>
      </button>
    `

    // 将pre元素包装到容器中
    const parent = pre.parentNode
    if (parent) {
      parent.insertBefore(container, pre)
      container.appendChild(header)
      container.appendChild(pre)
    }
  })
}

</script>

<style scoped>
/* 编辑器基础样式 */
/* 移除冗余的 markdown-body 样式 */

/* 使 prose 样式生效于深层元素 */
:deep(.prose) {
  max-width: none;
  color: inherit;
  width: 100%;
}

/* 代码块相关样式 */
:deep(.code-block-container) {
  position: relative;
  margin: 1rem 0;
  border-radius: 0.375rem;
  overflow: hidden;
  background: var(--prism-background, #272727);
  border: 1px solid var(--prism-border, #333333);
}

:deep(.code-block-header) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  background: var(--prism-header-background, #333333);
  font-size: 0.875rem;
}

:deep(.code-language) {
  font-weight: 600;
  color: var(--prism-header-text, #ffffff);
  text-transform: uppercase;
  font-size: 0.75rem;
}

:deep(.copy-code-btn) {
  padding: 0.25rem;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

:deep(.copy-code-btn:hover) {
  background: var(--prism-button-hover, rgba(255, 255, 255, 0.1));
}

:deep(.copy-code-btn svg) {
  width: 1rem;
  height: 1rem;
  color: var(--prism-header-text, #ffffff);
}

/* Prism 行号样式 */
:deep(.line-numbers) {
  position: relative;
  padding-left: 3.8em;
  counter-reset: linenumber;
}

:deep(.line-numbers .line-numbers-rows) {
  position: absolute;
  pointer-events: none;
  top: 0;
  font-size: 100%;
  left: -3.8em;
  width: 3em;
  letter-spacing: -1px;
  border-right: 1px solid var(--prism-line-border, #999);
  user-select: none;
}

:deep(.line-numbers-rows > span) {
  display: block;
  counter-increment: linenumber;
}

:deep(.line-numbers-rows > span:before) {
  content: counter(linenumber);
  color: var(--prism-line-number, #999);
  display: block;
  padding-right: 0.8em;
  text-align: right;
}

/* 代码块基础样式 */
:deep(.prose pre) {
  margin: 0;
  padding: 1rem;
  overflow-x: auto;
  background: transparent;
  border: none;
  border-radius: 0;
  white-space: pre-wrap; /* 支持自动换行 */
  word-wrap: break-word; /* 长单词换行 */
  overflow-wrap: break-word; /* 现代浏览器的换行属性 */
  word-break: break-all; /* 强制在任意字符间换行 */
}

:deep(.prose pre code) {
  background: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.9em;
  color: inherit;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  display: block;
  width: 100%;
  line-height: 1.5;
  white-space: pre-wrap; /* 支持自动换行 */
  word-wrap: break-word; /* 长单词换行 */
  overflow-wrap: break-word; /* 现代浏览器的换行属性 */
  word-break: break-all; /* 强制在任意字符间换行 */
}

:deep(.prose code) {
  border-radius: 0.25rem;
  padding: 0.2em 0.4em;
  font-size: 0.9em;
  background-color: hsl(var(--b2, 215 28% 17%));
  color: hsl(var(--p, 217 91% 60%));
  font-weight: 500;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

/* 确保编辑器文本区域在暗色主题下清晰可见 */
[data-theme="dark"] textarea,
[data-theme="night"] textarea,
[data-theme="black"] textarea,
[data-theme="dracula"] textarea,
[data-theme="halloween"] textarea,
[data-theme="business"] textarea,
[data-theme="luxury"] textarea,
[data-theme="coffee"] textarea {
  color: rgb(226, 232, 240) !important;
}

/* NoteEditor特有的样式 */

/* 编辑器区域特殊样式 */
.editor-container {
  transition: all 0.3s ease;
}

.editor-toolbar {
  border-bottom: 1px solid var(--border-color);
  background: var(--background-secondary);
  padding: 8px 16px;
}

/* 代码编辑器的特殊样式 */
.code-editor {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  line-height: 1.6;
}

/* Markdown预览的特殊样式 */
.markdown-preview {
  padding: 20px;
  max-width: none;
}

/* 编辑器分割线 */
.editor-divider {
  width: 2px;
  background: var(--border-color);
  cursor: col-resize;
  transition: background 0.2s ease;
}

.editor-divider:hover {
  background: var(--primary);
}

/* 全屏模式样式 */
.fullscreen-editor {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 100;
  background: var(--background);
}

/* 工具栏按钮的特殊样式 */
.toolbar-btn {
  padding: 6px 12px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background: var(--background-hover);
  transform: translateY(-1px);
}

.toolbar-btn.active {
  background: var(--primary);
  color: var(--primary-content);
}

/* 编辑器内容区域的暗色主题特殊优化 - 这些是编辑器特有的 */
[data-theme="dark"] :deep(.prose code),
[data-theme="night"] :deep(.prose code),
[data-theme="black"] :deep(.prose code) {
  background: rgba(255, 255, 255, 0.1) !important;
  color: rgb(251, 191, 36) !important;
  padding: 2px 4px;
  border-radius: 3px;
}

[data-theme="dark"] :deep(.prose pre),
[data-theme="night"] :deep(.prose pre),
[data-theme="black"] :deep(.prose pre) {
  background: rgba(255, 255, 255, 0.05) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
  color: rgb(229, 231, 235) !important;
}

[data-theme="dark"] :deep(.prose blockquote),
[data-theme="night"] :deep(.prose blockquote),
[data-theme="black"] :deep(.prose blockquote) {
  border-left: 4px solid rgba(255, 255, 255, 0.3) !important;
  background: rgba(255, 255, 255, 0.03) !important;
  color: rgb(209, 213, 219) !important;
}

/* 编辑器textarea的特殊暗色主题优化 */
[data-theme="dark"] textarea.editor-textarea,
[data-theme="night"] textarea.editor-textarea,
[data-theme="black"] textarea.editor-textarea {
  background: rgba(255, 255, 255, 0.03) !important;
  color: rgb(243, 244, 246) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
}

[data-theme="dark"] textarea.editor-textarea:focus,
[data-theme="night"] textarea.editor-textarea:focus,
[data-theme="black"] textarea.editor-textarea:focus {
  border-color: rgba(255, 255, 255, 0.3) !important;
  background: rgba(255, 255, 255, 0.05) !important;
}

/* 确保所有图片都是响应式的 */
:deep(.prose img) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  margin: 1rem auto;
  display: block;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  object-fit: contain;
}

/* 对于超大图片，设置最大高度以防止占据过多屏幕空间 */
:deep(.prose img),
:deep(.embedded-image) {
  max-height: 70vh;
  /* 最大高度为视口高度的70% */
  width: auto;
  object-fit: contain;
}

/* 在小屏幕上进一步限制图片大小 */
@media (max-width: 768px) {

  :deep(.prose img),
  :deep(.embedded-image) {
    max-height: 50vh;
    /* 在移动设备上限制为50%视口高度 */
    margin: 0.5rem auto;
  }
}

/* 图片容器样式，确保图片居中显示 */
:deep(.prose p:has(img)) {
  text-align: center;
  margin: 1rem 0;
}

/* 为图片添加加载状态和错误处理 */
:deep(.prose img) {
  transition: opacity 0.3s ease;
  cursor: zoom-in;
}

:deep(.prose img:hover) {
  opacity: 0.9;
}

/* 图片加载失败时的样式 */
:deep(.prose img[src=""]),
:deep(.prose img:not([src])) {
  display: none;
}

/* 响应式图片的额外样式 */
:deep(.responsive-image) {
  width: 100%;
  height: auto;
  max-width: 100%;
  object-fit: contain;
  border-radius: 0.5rem;
  transition: all 0.3s ease;
}

/* 图片容器的响应式布局 */
:deep(.prose) {
  overflow-wrap: break-word;
  word-wrap: break-word;
}

/* 确保图片不会破坏布局 */
:deep(.prose p) {
  overflow: hidden;
}

/* 图片加载时的占位效果 */
:deep(.prose img[loading="lazy"]) {
  background: linear-gradient(90deg, #f0f0f0 25%, transparent 37%, #f0f0f0 63%);
  background-size: 400% 100%;
  animation: shimmer 1.5s ease-in-out infinite;
}

@keyframes shimmer {
  0% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

/* 暗色主题下的图片占位效果 */
[data-theme="dark"] :deep(.prose img[loading="lazy"]),
[data-theme="night"] :deep(.prose img[loading="lazy"]),
[data-theme="black"] :deep(.prose img[loading="lazy"]) {
  background: linear-gradient(90deg, #2a2a2a 25%, transparent 37%, #2a2a2a 63%);
  background-size: 400% 100%;
}

/* 图片模态框的响应式样式 */
.image-modal {
  backdrop-filter: blur(8px);
}

/* 确保模态框中的图片也是响应式的 */
.image-modal img {
  max-width: 95vw;
  max-height: 95vh;
  object-fit: contain;
}

/* 在极小屏幕上的特殊处理 */
@media (max-width: 480px) {

  :deep(.prose img),
  :deep(.embedded-image),
  :deep(.responsive-image) {
    max-height: 40vh;
    margin: 0.25rem auto;
  }

  .image-modal img {
    max-width: 98vw;
    max-height: 85vh;
  }
}

/* Prism 主题变量 */
:root {
  --prism-background: #f5f5f5;
  --prism-border: #e0e0e0;
  --prism-header-background: #f0f0f0;
  --prism-header-text: #666;
  --prism-button-hover: rgba(0, 0, 0, 0.1);
  --prism-line-border: #999;
  --prism-line-number: #999;
}

/* 暗色主题的 Prism 变量 */
[data-theme="dark"] {
  --prism-background: #2d3748;
  --prism-border: #4a5568;
  --prism-header-background: #1a202c;
  --prism-header-text: #a0aec0;
  --prism-button-hover: rgba(255, 255, 255, 0.1);
  --prism-line-border: #4a5568;
  --prism-line-number: #718096;
}

/* Prism 语法高亮颜色 */
:deep(.token.comment),
:deep(.token.prolog),
:deep(.token.doctype),
:deep(.token.cdata) {
  color: #708090;
}

:deep(.token.punctuation) {
  color: #999;
}

:deep(.token.property),
:deep(.token.tag),
:deep(.token.boolean),
:deep(.token.number),
:deep(.token.constant),
:deep(.token.symbol),
:deep(.token.deleted) {
  color: #905;
}

:deep(.token.selector),
:deep(.token.attr-name),
:deep(.token.string),
:deep(.token.char),
:deep(.token.builtin),
:deep(.token.inserted) {
  color: #690;
}

:deep(.token.operator),
:deep(.token.entity),
:deep(.token.url),
:deep(.language-css .token.string),
:deep(.style .token.string) {
  color: #9a6e3a;
}

:deep(.token.atrule),
:deep(.token.attr-value),
:deep(.token.keyword) {
  color: #07a;
}

:deep(.token.function),
:deep(.token.class-name) {
  color: #dd4a68;
}

:deep(.token.regex),
:deep(.token.important),
:deep(.token.variable) {
  color: #e90;
}

/* 嵌入图片样式 */
:deep(.embedded-image) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5rem;
  margin: 1rem 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  display: block;
  object-fit: contain;
}

/* 确保 Prism 高亮后的代码块也支持换行 */
:deep(.prose pre[class*="language-"]) {
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important; /* 强制在任意字符间换行 */
}

:deep(.prose pre[class*="language-"] code) {
  white-space: pre-wrap !important;
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important; /* 强制在任意字符间换行 */
}

/* 行号模式下的换行支持 */
:deep(.line-numbers > code) {
  position: relative;
  white-space: pre-wrap !important; /* 强制支持换行 */
  word-wrap: break-word !important;
  overflow-wrap: break-word !important;
  word-break: break-all !important; /* 强制在任意字符间换行 */
}
</style>
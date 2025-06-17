<template>
  <div class="h-full flex flex-col" tabindex="0" @focusout="onEditorBlur">
    <!-- 顶部工具栏 -->
    <div class="p-2 border-b border-base-300 flex items-center justify-between">
      <!-- 标题和状态区 -->
      <div class="flex-1">
        <input 
          type="text" 
          placeholder="无标题笔记..." 
          class="input input-lg w-full text-xl font-bold p-0 border-0 focus:outline-none bg-transparent" 
          v-model="localNote.title"
          @input="autoSave"
          @blur="onTitleBlur"
        />
      </div>
      
      <!-- 操作按钮区 -->
      <div class="flex items-center gap-2">
        <!-- AI扩充按钮 -->
        <button class="btn btn-sm btn-ghost btn-square" title="AI扩充内容" @click="expandWithAI()">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
          </svg>
        </button>
        
        <div class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-sm btn-ghost btn-square" title="更多操作">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
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
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
            <path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="斜体" @click="insertMarkdown('*', '*')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="19" y1="4" x2="10" y2="4"></line>
            <line x1="14" y1="20" x2="5" y2="20"></line>
            <line x1="15" y1="4" x2="9" y2="20"></line>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="删除线" @click="insertMarkdown('~~', '~~')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 9V5H7v4"></path>
            <path d="M7 13v6h10v-6"></path>
            <line x1="4" y1="12" x2="20" y2="12"></line>
          </svg>
        </button>
      </div>
      
      <div class="btn-group">
        <button class="btn btn-sm btn-ghost" title="无序列表" @click="insertMarkdown('- ')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="8" y1="6" x2="21" y2="6"></line>
            <line x1="8" y1="12" x2="21" y2="12"></line>
            <line x1="8" y1="18" x2="21" y2="18"></line>
            <line x1="3" y1="6" x2="3.01" y2="6"></line>
            <line x1="3" y1="12" x2="3.01" y2="12"></line>
            <line x1="3" y1="18" x2="3.01" y2="18"></line>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="有序列表" @click="insertMarkdown('1. ')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="10" y1="6" x2="21" y2="6"></line>
            <line x1="10" y1="12" x2="21" y2="12"></line>
            <line x1="10" y1="18" x2="21" y2="18"></line>
            <path d="M4 6h1v4"></path>
            <path d="M4 10h2"></path>
            <path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path>
          </svg>
        </button>
        <button class="btn btn-sm btn-ghost" title="任务列表" @click="insertMarkdown('- [ ] ')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="5" width="6" height="6" rx="1"></rect>
            <path d="m9 11-6-6"></path>
            <line x1="13" y1="8" x2="21" y2="8"></line>
            <rect x="3" y="17" width="6" height="6" rx="1"></rect>
            <line x1="13" y1="20" x2="21" y2="20"></line>
          </svg>
        </button>
      </div>
      
      <button class="btn btn-sm btn-ghost" title="引用块" @click="insertMarkdown('> ')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path>
          <path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path>
        </svg>
      </button>
      
      <button class="btn btn-sm btn-ghost" title="插入链接" @click="insertMarkdown('[', '](https://)')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
          <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
        </svg>
      </button>
      
      <button class="btn btn-sm btn-ghost" title="插入图片" @click="insertMarkdown('![', '](图片URL)')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
      </button>
      
      <button class="btn btn-sm btn-ghost" title="代码块" @click="insertMarkdown('```\n', '\n```')">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="16 18 22 12 16 6"></polyline>
          <polyline points="8 6 2 12 8 18"></polyline>
        </svg>
      </button>
      
      <button class="btn btn-sm btn-ghost" title="插入表格" @click="insertTable">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <line x1="3" y1="9" x2="21" y2="9"></line>
          <line x1="3" y1="15" x2="21" y2="15"></line>
          <line x1="9" y1="3" x2="9" y2="21"></line>
          <line x1="15" y1="3" x2="15" y2="21"></line>
        </svg>
      </button>
      
      <div class="dropdown dropdown-end ml-1">
        <button tabindex="0" class="btn btn-sm btn-ghost" title="代码高亮主题">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v20M4 12h16"></path>
            <rect x="6" y="6" width="12" height="12" rx="2" ry="2"></rect>
          </svg>
        </button>
        <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
          <li><a href="#" @click.prevent="setHighlightTheme('github-dark')">GitHub Dark</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('atom-one-dark')">Atom One Dark</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('atom-one-light')">Atom One Light</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('github')">GitHub Light</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('monokai')">Monokai</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('vs')">Visual Studio</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('dracula')">Dracula</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('tomorrow-night')">Tomorrow Night</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('solarized-dark')">Solarized Dark</a></li>
          <li><a href="#" @click.prevent="setHighlightTheme('solarized-light')">Solarized Light</a></li>
        </ul>
      </div>
      
      <div class="ml-auto flex gap-1">
        <button 
          class="btn btn-sm btn-ghost" 
          :class="{'btn-active': isEditOnly}" 
          @click="setEditMode('editOnly')" 
          title="仅编辑">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
          </svg>
        </button>
        <button 
          class="btn btn-sm btn-ghost" 
          :class="{'btn-active': isPreviewMode}" 
          @click="setEditMode('preview')" 
          title="预览">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
            <circle cx="12" cy="12" r="3"></circle>
          </svg>
        </button>
        <button 
          class="btn btn-sm btn-ghost" 
          :class="{'btn-active': isSplitMode}" 
          @click="setEditMode('split')" 
          title="分屏模式">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="12" y1="3" x2="12" y2="21"></line>
          </svg>
        </button>
      </div>
    </div>

    <!-- 主要编辑区域 -->
    <div class="flex-1 flex overflow-hidden relative">
      <!-- Markdown编辑器 -->
      <textarea 
        v-if="!isPreviewMode || isEditOnly || isSplitMode"
        class="flex-1 p-4 h-full resize-none focus:outline-none font-mono text-base overflow-auto"
        :class="{ 'w-1/2': isSplitMode }"
        placeholder="开始输入内容..."
        v-model="localNote.content"
        @input="autoSave"
        @contextmenu.prevent="handleContextMenu"
        @paste="handlePaste"
        @keydown="handleKeyDown"
        @scroll="handleEditorScroll"
        ref="editorTextarea"
        @blur="onContentBlur"
      ></textarea>
      
      <!-- 右键菜单 -->
      <div v-if="showContextMenu" class="context-menu absolute bg-base-200 text-base-content rounded-md shadow-lg p-2 z-30"
        :style="{top: contextMenuY + 'px', left: contextMenuX + 'px'}">
        <ul class="menu menu-sm p-1">
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="expandSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
              TIP一下
            </button>
          </li>
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="explainSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              解释一下
            </button>
          </li>
          <li v-if="hasSelectedText">
            <button class="flex items-center gap-1" @click="translateSelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
              </svg>
              翻译一下
            </button>
          </li>
          <li>
            <button class="flex items-center gap-1" @click="copySelectedText">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
              </svg>
              复制
            </button>
          </li>
          <li>
            <button class="flex items-center gap-1" @click="pasteFromClipboard">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
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
            <button 
              v-if="isStreaming" 
              class="btn btn-sm btn-error" 
              @click="cancelAIGeneration">
              取消生成
            </button>
          </div>
        </div>
      </div>
      
      <!-- Markdown预览区 -->
      <div 
        v-if="isPreviewMode || isSplitMode"
        class="flex-1 p-4 overflow-auto prose prose-sm md:prose-base lg:prose-lg dark:prose-invert max-w-none"
        :class="{ 'w-1/2': isSplitMode }"
        @scroll="handlePreviewScroll"
        ref="previewDiv"
      >
        <div v-html="renderedContent" class="prose max-w-none"></div>
      </div>
    </div>

    <!-- 底部元数据区域 -->
    <div class="border-t border-base-100 p-4 bg-base-200">
      <!-- 将标签选择器和统计信息放在同一行 -->
      <div class="flex flex-wrap w-full gap-4 items-center justify-between">
        <!-- 标签选择器组件 -->
        <div class="flex-1">
          <TagSelector 
            v-model="localNote.tags" 
            :contentText="localNote.content" 
            :titleText="localNote.title"
            @saveNote="saveNoteToList"
          />
        </div>
        
        <!-- 统计信息 -->
        <div class="text-xs text-base-content/80 flex items-center gap-4 shrink-0">
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
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
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
            <button class="btn btn-sm btn-primary" @click="insertExplanationToContent" v-if="!isExplaining && explanationContent">
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
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
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
            <button class="btn btn-sm btn-primary" @click="insertTranslationToContent" v-if="!isTranslating && translationContent">
              插入到笔记
            </button>
            <button class="btn btn-sm btn-ghost" @click="showTranslationBox = false">
              关闭
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, defineProps, defineEmits, nextTick, onMounted, onActivated } from 'vue'
import DOMPurify from 'dompurify'
import { invoke } from '@tauri-apps/api/core'
import TagSelector from './TagSelector.vue'
// 引入 highlight.js 及其样式

import hljs from 'highlight.js'

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
const localNote = ref<Note>({...props.note})
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
    emit('update', {...localNote.value, _contentOnly: true})
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
    emit('update', {...localNote.value, _contentOnly: true})
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
          // 返回HTML图片标签，使用base64数据
          return `<img src="${localNote.value.images[imageId]}" alt="${alt || '图片'}" class="embedded-image" />`
        }
        // 如果找不到图片，保持原样
        return match
      })
    }
    
    // 以下是简单的 Markdown 转 HTML 处理，仅处理常见的几种格式
    // 代码块 - 修复异常回车问题
    processedContent = processedContent.replace(/```([a-z]*)\n([\s\S]*?)\n```/g, (_match, lang, code) => {
      try {
        // 标准化换行符，避免Windows、Mac、Linux系统之间的差异
        let normalizedCode = code.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
        
        // 处理代码高亮
        let highlightedCode = normalizedCode;
        if (lang && hljs.getLanguage(lang)) {
          highlightedCode = hljs.highlight(normalizedCode, { language: lang }).value;
        } else if (normalizedCode.trim().length > 0) { // 只在有内容时自动检测语言
          highlightedCode = hljs.highlightAuto(normalizedCode).value;
        }
        
        // 确保代码中的换行符正确转换为HTML
        highlightedCode = highlightedCode.replace(/\n/g, '<br>');
        
        // 为每个代码块添加唯一ID以便后续样式更新
        const blockId = `code-block-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
        const theme = currentHighlightTheme.value;
        
        // 重要：应用当前选择的高亮主题
        return `<div class="code-block-wrapper relative">
          <pre class="pre-${theme}"><code id="${blockId}" class="hljs language-${lang || ''} ${theme}-theme">${highlightedCode}</code></pre>
          <button class="copy-code-btn absolute top-2 right-2 btn btn-xs btn-circle btn-ghost opacity-70 hover:opacity-100 bg-base-300" data-code="${encodeURIComponent(normalizedCode)}" title="复制代码">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
            </svg>
          </button>
        </div>`;
      } catch (e) {
        console.error('代码高亮处理错误:', e);
        // 作为纯文本返回，确保安全
        return `<pre><code>${DOMPurify.sanitize(code)}</code></pre>`;
      }
    });
    
    // 行内代码 - 确保能处理包含特殊字符的代码
    processedContent = processedContent.replace(/`([^`]+)`/g, (_match, codeContent) => {
      // 使用DOMPurify清理内容，防止XSS
      const sanitizedCode = DOMPurify.sanitize(codeContent);
      return `<code>${sanitizedCode}</code>`;
    });
    
    // 标题 (h1, h2, h3)
    processedContent = processedContent.replace(/^# (.+)$/gm, '<h1>$1</h1>');
    processedContent = processedContent.replace(/^## (.+)$/gm, '<h2>$1</h2>');
    processedContent = processedContent.replace(/^### (.+)$/gm, '<h3>$1</h3>');
    processedContent = processedContent.replace(/^#### (.+)$/gm, '<h4>$1</h5>');
    // 粗体和斜体
    processedContent = processedContent.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
    processedContent = processedContent.replace(/\*([^*]+)\*/g, '<em>$1</em>');
    
    // 删除线
    processedContent = processedContent.replace(/~~([^~]+)~~/g, '<del>$1</del>');
    
    // 链接
    processedContent = processedContent.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>');
    
    // 图片 (除了已处理的本地图片)
    processedContent = processedContent.replace(/!\[([^\]]*)\]\((?!local:\/\/)([^)]+)\)/g, '<img src="$2" alt="$1">');
    
    // 无序列表 - 改进处理，支持多级列表
    processedContent = processedContent.replace(/^(\s*)[-*+]\s+(.+)$/gm, (_match, indent, content) => {
      const indentLevel = Math.floor(indent.length / 2);
      return `<li data-level="${indentLevel}" class="ml-${indentLevel ? indentLevel * 4 : 0}">${content}</li>`;
    });
    
    // 将无序列表项组织成嵌套的HTML结构
    const listRegex = /(<li data-level="\d+"[^>]*>.*<\/li>\n?)+/g;
    processedContent = processedContent.replace(listRegex, (match) => {
      let items = match.match(/<li data-level="(\d+)"[^>]*>(.*?)<\/li>/g) || [];
      if (items.length === 0) return match;
      
      // 简单处理：将所有列表项放入一个ul标签
      return `<ul>${items.join('')}</ul>`;
    });
    
    // 任务列表
    processedContent = processedContent.replace(/^\s*-\s+\[\s\]\s+(.+)$/gm, '<li class="flex items-start"><input type="checkbox" class="mt-1 mr-2" disabled /><span>$1</span></li>');
    processedContent = processedContent.replace(/^\s*-\s+\[x\]\s+(.+)$/gm, '<li class="flex items-start"><input type="checkbox" class="mt-1 mr-2" checked disabled /><span>$1</span></li>');
    
    // 有序列表
    processedContent = processedContent.replace(/^(\s*)(\d+)\.\s+(.+)$/gm, (_match, indent, _num, content) => {
      const indentLevel = Math.floor(indent.length / 2);
      return `<li data-level="${indentLevel}" class="ml-${indentLevel ? indentLevel * 4 : 0}" data-ordered="true">${content}</li>`;
    });
    
    // 将有序列表项组织成嵌套的HTML结构
    const orderedListRegex = /(<li data-level="\d+"[^>]*data-ordered="true"[^>]*>.*<\/li>\n?)+/g;
    processedContent = processedContent.replace(orderedListRegex, (match) => {
      let items = match.match(/<li data-level="(\d+)"[^>]*data-ordered="true"[^>]*>(.*?)<\/li>/g) || [];
      if (items.length === 0) return match;
      
      // 简单处理：将所有列表项放入一个ol标签
      return `<ol>${items.join('')}</ol>`;
    });
    
    // 处理表格
    const tableRegex = /^\|(.+)\|\s*\n\|([-:\s|]+)\|\s*\n(\|.+\|\s*\n)+/gm;
    processedContent = processedContent.replace(tableRegex, (match) => {
      const lines = match.trim().split('\n');
      
      // 第一行是表头
      const headerCells = lines[0].split('|').filter(cell => cell.trim() !== '').map(cell => cell.trim());
      
      // 第二行是对齐方式
      const alignments = lines[1].split('|').filter(cell => cell.trim() !== '').map(cell => {
        const trimmed = cell.trim();
        if (trimmed.startsWith(':') && trimmed.endsWith(':')) return 'center';
        if (trimmed.endsWith(':')) return 'right';
        return 'left';
      });
      
      // 创建表头
      let tableHTML = '<table class="w-full border-collapse"><thead><tr>';
      headerCells.forEach((cell, index) => {
        const align = index < alignments.length ? alignments[index] : 'left';
        tableHTML += `<th class="border border-base-300 px-4 py-2 text-${align}">${cell}</th>`;
      });
      tableHTML += '</tr></thead><tbody>';
      
      // 创建表格内容
      for (let i = 2; i < lines.length; i++) {
        const row = lines[i];
        if (!row.trim()) continue;
        
        const cells = row.split('|').filter(cell => cell.trim() !== '').map(cell => cell.trim());
        
        tableHTML += '<tr>';
        cells.forEach((cell, index) => {
          const align = index < alignments.length ? alignments[index] : 'left';
          tableHTML += `<td class="border border-base-300 px-4 py-2 text-${align}">${cell}</td>`;
        });
        tableHTML += '</tr>';
      }
      
      tableHTML += '</tbody></table>';
      return tableHTML;
    });
    
    // 引用块
    processedContent = processedContent.replace(/^\s*>\s+(.+)$/gm, (_match, content) => {
      // 检测不同类型的引用
      if (content.startsWith('💡') || content.includes('提示') || content.includes('tip')) {
        return `<blockquote class="bg-info/10 border-l-4 border-info p-4 rounded-r-lg"><p class="text-info flex items-center gap-2"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg> ${content}</p></blockquote>`;
      } else if (content.startsWith('⚠️') || content.includes('警告') || content.includes('warning')) {
        return `<blockquote class="bg-warning/10 border-l-4 border-warning p-4 rounded-r-lg"><p class="text-warning flex items-center gap-2"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg> ${content}</p></blockquote>`;
      } else if (content.startsWith('🚫') || content.includes('错误') || content.includes('error')) {
        return `<blockquote class="bg-error/10 border-l-4 border-error p-4 rounded-r-lg"><p class="text-error flex items-center gap-2"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg> ${content}</p></blockquote>`;
      } else if (content.startsWith('✅') || content.includes('成功') || content.includes('success')) {
        return `<blockquote class="bg-success/10 border-l-4 border-success p-4 rounded-r-lg"><p class="text-success flex items-center gap-2"><svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg> ${content}</p></blockquote>`;
      } else {
        return `<blockquote class="border-l-4 border-base-300 p-4 rounded-r-lg"><p>${content}</p></blockquote>`;
      }
    });
    
    // 段落 (确保所有文本都在段落标签内)
    processedContent = processedContent.replace(/^(?!<[a-z][^>]*>)(.+)$/gm, '<p>$1</p>');
    
    // 确保换行正确处理
    processedContent = processedContent.replace(/\n\n/g, '</p><p>');
    
    // 使用DOMPurify清理HTML，防止XSS，但允许安全的HTML标签和图片
    return DOMPurify.sanitize(processedContent, {
      ADD_TAGS: ['iframe'],  // 允许iframe标签
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled']  // 允许这些属性
    })
  } catch (err) {
    console.error('Markdown渲染错误:', err)
    // 返回错误提示，或回退到纯文本
    const errorMessage = err instanceof Error ? err.message : String(err)
    return `<div class="text-error">Markdown渲染错误: ${errorMessage}</div>
            <pre>${DOMPurify.sanitize(localNote.value.content)}</pre>`
  }
})

const wordCount = computed(() => {
  if (!localNote.value.content) return 0
  // 简单的字数统计
  return localNote.value.content
    .replace(/\s+/g, ' ')
    .replace(/[\r\n]/g, '')
    .trim()
    .length
})

// 监听外部note变化
watch(() => props.note, async (newNote, oldNote) => {
  // 如果是初始化（oldNote为undefined）或者笔记ID发生变化（切换到不同的笔记），才完全重新设置localNote
  if (!oldNote || oldNote.id !== newNote.id) {
    // 深拷贝对象，保留图片数据
    const images = newNote.images ? {...newNote.images} : undefined;
    localNote.value = {...newNote, images};
    
    // 如果笔记有ID但没有images数据，尝试从后端加载
    if (newNote.id && !newNote.images) {
      try {
        console.log(`尝试加载笔记(${newNote.id})的图片...`)
        const images = await invoke('get_tip_images', { tip_id: newNote.id }) as Record<string, string>
        console.log(`获取到的图片数: ${Object.keys(images).length}`)
        
        if (Object.keys(images).length > 0) {
          localNote.value.images = images
          console.log('图片已加载到本地状态')
        }
      } catch (error) {
        console.error('加载笔记图片失败:', error)
        
        // 获取详细的错误信息
        let errorMessage = '加载笔记图片失败';
        if (error instanceof Error) {
          errorMessage = `${errorMessage}: ${error.message}`;
          console.error('错误详情:', error.stack);
        } else {
          errorMessage = `${errorMessage}: ${String(error)}`;
        }
        
        console.error(errorMessage)
      }
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
      emit('update', {...localNote.value, _titleOnly: true})
    } else {
      // 当内容变化时，仅更新内容但不触发列表重排序
      // 使用_contentOnly标记表示这是内容更新，不需要列表重排序
      emit('update', {...localNote.value, _contentOnly: true})
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
  const noteToUpdate = {...localNote.value, _fromBlur: true}
  
  // 如果笔记中包含图片，确保图片数据也被正确传递
  if (localNote.value.images && Object.keys(localNote.value.images).length > 0) {
    noteToUpdate.images = {...localNote.value.images}
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

// 监听组件挂载，获取可用标签
onMounted(() => {
  // 设置文档点击监听器
  setupDocumentClickListener()
  
  // 设置代码复制功能
  setupCodeCopyFeature()
  
  // 加载默认主题
  const theme = currentHighlightTheme.value
  console.log(`初始化代码高亮主题: ${theme}`)
  
  // 动态加载主题样式表
  loadHighlightTheme(theme)
  
  // 应用代码高亮主题
  setHighlightTheme(currentHighlightTheme.value)
  
  // 监听系统主题变化
  if (window.matchMedia) {
    const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    
    // 添加监听器以响应系统主题变化
    const themeChangeHandler = (event: MediaQueryListEvent) => {
      // 如果用户没有手动设置主题，则自动切换
      if (!localStorage.getItem('mytips-highlight-theme-manual')) {
        const newTheme = event.matches ? 'github-dark' : 'github'
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
        alert('复制失败，请手动选择并复制')
      }
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
        // 不再使用marked，直接赋值
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

// 在组件属性下添加以下状态变量
const currentHighlightTheme = ref(localStorage.getItem('mytips-highlight-theme') || getDefaultHighlightTheme())

// 添加根据系统主题自动选择代码高亮主题的函数
function getDefaultHighlightTheme() {
  // 检查系统是否支持颜色模式查询
  if (window.matchMedia) {
    // 检查是否为暗色模式
    const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    return isDarkMode ? 'github-dark' : 'github'
  }
  
  // 默认使用暗色主题
  return 'github-dark'
}

// 添加动态加载highlight.js主题的函数
function loadHighlightTheme(theme: string) {
  // 移除之前可能加载的主题样式
  const existingThemeLinks = document.querySelectorAll('link[data-highlight-theme]')
  existingThemeLinks.forEach(link => link.remove())
  
  // 创建新的样式链接
  const linkElement = document.createElement('link')
  linkElement.rel = 'stylesheet'
  linkElement.href = `https://cdn.jsdelivr.net/npm/highlight.js@11.7.0/styles/${theme}.min.css`
  linkElement.setAttribute('data-highlight-theme', theme)
  
  // 添加到文档头部
  document.head.appendChild(linkElement)
  
  console.log(`已加载代码高亮主题: ${theme}`)
}

// 修改setHighlightTheme函数，添加动态加载主题
function setHighlightTheme(theme: string) {
  currentHighlightTheme.value = theme
  localStorage.setItem('mytips-highlight-theme', theme)
  
  // 标记用户已手动选择主题
  localStorage.setItem('mytips-highlight-theme-manual', 'true')
  
  // 动态加载对应的highlight.js主题
  loadHighlightTheme(theme)
  
  // 更新所有代码块的样式类
  nextTick(() => {
    const codeBlocks = document.querySelectorAll('.prose pre code')
    console.log(`应用主题: ${theme}, 找到 ${codeBlocks.length} 个代码块`)
    
    codeBlocks.forEach(codeBlock => {
      // 移除所有主题类
      codeBlock.classList.remove('github-dark-theme', 'atom-one-dark-theme', 'atom-one-light-theme', 'github-theme', 'monokai-theme', 'vs-theme', 'dracula-theme')
      // 添加当前主题类
      codeBlock.classList.add(`${theme}-theme`)
      
      // 更新父级pre元素的主题类，为不支持:has选择器的浏览器提供备选方案
      const preElement = codeBlock.closest('pre')
      if (preElement) {
        preElement.classList.remove('pre-github-dark', 'pre-atom-one-dark', 'pre-atom-one-light', 'pre-github', 'pre-monokai', 'pre-vs', 'pre-dracula')
        preElement.classList.add(`pre-${theme}`)
        
        // 确保复制按钮背景色与当前主题匹配
        const codeBlockWrapper = preElement.closest('.code-block-wrapper')
        if (codeBlockWrapper) {
          const copyButton = codeBlockWrapper.querySelector('.copy-code-btn')
          if (copyButton) {
            // 调整复制按钮颜色以适应当前主题
            const isDarkTheme = ['github-dark', 'atom-one-dark', 'monokai', 'dracula', 'tomorrow-night', 'solarized-dark'].includes(theme)
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
      
      // 重新应用代码高亮
      const code = codeBlock.textContent || ''
      const language = Array.from(codeBlock.classList)
        .find(cls => cls.startsWith('language-'))
        ?.replace('language-', '') || ''
      
      if (code) {
        try {
          // 清除之前的高亮
          codeBlock.innerHTML = DOMPurify.sanitize(code)
          
          // 重新应用高亮
          if (language && hljs.getLanguage(language)) {
            codeBlock.innerHTML = hljs.highlight(code, { language }).value
          } else {
            codeBlock.innerHTML = hljs.highlightAuto(code).value
          }
        } catch (e) {
          console.error('重新应用代码高亮失败:', e)
        }
      }
    })
    
    // 强制刷新视图以应用新样式
    setTimeout(() => {
      const scrollPos = editorTextarea.value ? editorTextarea.value.scrollTop : 0
      const currentContent = localNote.value.content
      localNote.value.content = " " + currentContent
      nextTick(() => {
        localNote.value.content = currentContent
        nextTick(() => {
          if (editorTextarea.value) {
            editorTextarea.value.scrollTop = scrollPos
          }
        })
      })
    }, 10)
  })
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
:deep(.code-block-wrapper) {
  position: relative;
  margin: 1rem 0;
}

:deep(.copy-code-btn) {
  transition: opacity 0.2s ease-in-out;
  z-index: 10;
}

:deep(.code-block-wrapper:hover .copy-code-btn) {
  opacity: 1;
}

/* 代码高亮相关样式 */
:deep(.prose pre) {
  margin: 1rem 0;
  padding: 1rem;
  border-radius: 0.375rem;
  overflow-x: auto;
  /* 移除固定背景色，使其能响应主题变化 */
  background-color: transparent;
}

/* 主题特定的pre样式 */
:deep(.pre-github-dark) {
  background-color: #0d1117 !important;
}

:deep(.pre-atom-one-dark) {
  background-color: #282c34 !important;
}

:deep(.pre-atom-one-light) {
  background-color: #fafafa !important;
}

:deep(.pre-github) {
  background-color: #ffffff !important;
}

:deep(.pre-monokai) {
  background-color: #272822 !important;
}

:deep(.pre-vs) {
  background-color: #ffffff !important;
}

:deep(.prose pre code) {
  background-color: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.9em;
  color: inherit;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  display: block;
  width: 100%;
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

/* 嵌入图片样式 */
:deep(.embedded-image) {
  max-width: 100%;
  border-radius: 0.5rem;
  margin: 1rem 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 表格样式 */
:deep(.prose table) {
  width: 100%;
  border-collapse: collapse;
  margin: 1.5rem 0;
  font-size: 0.9em;
}

:deep(.prose table th) {
  background-color: hsl(var(--b2, 215 28% 17%));
  font-weight: 600;
  text-align: left;
  padding: 0.75rem;
  border: 1px solid hsl(var(--b3, 215 28% 17%));
}

:deep(.prose table td) {
  padding: 0.75rem;
  border: 1px solid hsl(var(--b3, 215 28% 17%));
  vertical-align: top;
}

:deep(.prose table tr:nth-child(even)) {
  background-color: hsl(var(--b1, 215 28% 12%/0.05));
}

/* 列表样式增强 */
:deep(.prose ul) {
  margin: 1rem 0;
  padding-left: 1.5rem;
  list-style-type: disc;
}

:deep(.prose ol) {
  margin: 1rem 0;
  padding-left: 1.5rem;
  list-style-type: decimal;
}

:deep(.prose li) {
  margin: 0.5rem 0;
}

:deep(.prose li > ul),
:deep(.prose li > ol) {
  margin: 0.5rem 0;
}

/* 引用块样式 */
:deep(.prose blockquote) {
  border-left: 4px solid hsl(var(--p, 217 91% 60%));
  padding: 0.5rem 0 0.5rem 1rem;
  margin: 1.5rem 0;
  color: hsl(var(--bc, 215 28% 65%));
  font-style: italic;
}

:deep(.prose blockquote p) {
  margin: 0;
}

/* 链接样式 */
:deep(.prose a) {
  color: hsl(var(--p, 217 91% 60%));
  text-decoration: underline;
  text-decoration-thickness: 1px;
  text-underline-offset: 2px;
  transition: color 0.2s;
}

:deep(.prose a:hover) {
  color: hsl(var(--pf, 217 91% 70%));
  text-decoration-thickness: 2px;
}

/* 标题样式增强 */
:deep(.prose h1) {
  font-size: 2em;
  font-weight: 700;
  margin: 1.5em 0 0.5em;
  color: hsl(var(--bc, 215 28% 65%));
}

:deep(.prose h2) {
  font-size: 1.5em;
  font-weight: 600;
  margin: 1.4em 0 0.5em;
  color: hsl(var(--bc, 215 28% 65%));
  border-bottom: 1px solid hsl(var(--b3, 215 28% 17%));
  padding-bottom: 0.3em;
}

:deep(.prose h3) {
  font-size: 1.25em;
  font-weight: 600;
  margin: 1.3em 0 0.5em;
  color: hsl(var(--bc, 215 28% 65%));
}

:deep(.prose h4) {
  font-size: 1.1em;
  font-weight: 600;
  margin: 1.2em 0 0.5em;
  color: hsl(var(--bc, 215 28% 65%));
}

/* 任务列表样式 */
:deep(.prose li input[type="checkbox"]) {
  margin-right: 0.5rem;
}

:deep(.prose li.flex) {
  display: flex;
  align-items: flex-start;
  margin: 0.25rem 0;
}

/* 确保highlight.js主题样式正确加载 */
:deep(.hljs) {
  display: block;
  overflow-x: auto;
  padding: 0.5em;
  background: transparent;
}

/* 修复代码块内换行显示问题 */
:deep(.hljs br) {
  display: block;
  content: '';
  margin-top: 0.25em;
}

/* 暗黑主题下增强字体可见性 */
[data-theme="dark"] .text-xs,
[data-theme="night"] .text-xs,
[data-theme="black"] .text-xs,
[data-theme="dracula"] .text-xs,
[data-theme="halloween"] .text-xs,
[data-theme="business"] .text-xs,
[data-theme="luxury"] .text-xs,
[data-theme="coffee"] .text-xs {
  color: rgb(203, 213, 225) !important;
}

[data-theme="dark"] .text-sm,
[data-theme="night"] .text-sm,
[data-theme="black"] .text-sm,
[data-theme="dracula"] .text-sm,
[data-theme="halloween"] .text-sm,
[data-theme="business"] .text-sm,
[data-theme="luxury"] .text-sm,
[data-theme="coffee"] .text-sm {
  color: rgb(203, 213, 225) !important;
}

/* 对于特别暗的主题，使用更亮的文字 */
[data-theme="black"] .text-xs,
[data-theme="night"] .text-xs {
  color: rgb(229, 231, 235) !important;
}

[data-theme="black"] .text-sm,
[data-theme="night"] .text-sm {
  color: rgb(229, 231, 235) !important;
}

/* 增强编辑器内文本的可读性 */
[data-theme="dark"] :deep(.prose),
[data-theme="night"] :deep(.prose),
[data-theme="black"] :deep(.prose),
[data-theme="dracula"] :deep(.prose),
[data-theme="halloween"] :deep(.prose),
[data-theme="business"] :deep(.prose),
[data-theme="luxury"] :deep(.prose),
[data-theme="coffee"] :deep(.prose) {
  color: rgb(226, 232, 240) !important;
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
</style>
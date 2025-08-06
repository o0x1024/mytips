<template>
  <div class="p-2 border-b border-base-300 bg-base-100 flex items-center justify-between text-base-content text-sm" ref="toolbarContainer">
    <div class="flex items-center gap-2 flex-wrap" ref="toolbarLeft">
      <!-- 模式切换 -->
      <div class="flex items-center gap-1 rounded-md p-0.5">
        <button
          @click="emitCommand('set-edit-mode', 'editOnly')"
          class="btn btn-xs btn-ghost"
          :class="{ 'btn-active': isEditOnly }"
          :title="t('noteEditor.editModeTooltip')"
          data-priority="1"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path></svg>
        </button>
        <button
          @click="emitCommand('set-edit-mode', 'split')"
          class="btn btn-xs btn-ghost"
          :class="{ 'btn-active': isSplitMode }"
          :title="t('noteEditor.splitModeTooltip')"
          data-priority="1"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"></rect><line x1="12" y1="3" x2="12" y2="21"></line><path d="M7 8h2M7 12h2M16 8h1M16 12h1M16 16h1M7 16h2"></path></svg>
        </button>
        <button
          @click="emitCommand('set-edit-mode', 'preview')"
          class="btn btn-xs btn-ghost"
          :class="{ 'btn-active': isPreviewMode }"
          :title="t('noteEditor.previewModeTooltip')"
          data-priority="1"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
        </button>
        <!-- WYSIWYG模式已注释 -->
        <!-- <button
          @click="emitCommand('set-edit-mode', 'wysiwyg')"
          class="btn btn-xs btn-ghost"
          :class="{ 'btn-active': isWysiwygMode }"
          title="WYSIWYG Mode"
          data-priority="1"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19l7-7 3 3-7 7-3-3z"></path><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path><path d="M2 2l7.586 7.586"></path><circle cx="11" cy="11" r="2"></circle></svg>
        </button> -->
      </div>

      <div class="divider divider-horizontal mx-1"></div>
      
      <!-- Markdown编辑工具 -->
      <!-- 标题 -->
      <div class="dropdown dropdown-bottom">
        <label tabindex="0" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.headingTooltip')" data-priority="1">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 12h12"></path><path d="M6 20V4"></path><path d="M18 20V4"></path></svg>
        </label>
        <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-32 z-20">
          <li><a @click="emitCommand('insert-markdown', '# ', '')">H1</a></li>
          <li><a @click="emitCommand('insert-markdown', '## ', '')">H2</a></li>
          <li><a @click="emitCommand('insert-markdown', '### ', '')">H3</a></li>
          <li><a @click="emitCommand('insert-markdown', '#### ', '')">H4</a></li>
          <li><a @click="emitCommand('insert-markdown', '##### ', '')">H5</a></li>
          <li><a @click="emitCommand('insert-markdown', '###### ', '')">H6</a></li>
        </ul>
      </div>
      <!-- 加粗 -->
      <button @click="emitCommand('insert-markdown', '**', '**')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.boldTooltip')" data-priority="2">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path></svg>
      </button>
      <!-- 斜体 -->
      <button @click="emitCommand('insert-markdown', '*', '*')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.italicTooltip')" data-priority="2">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="19" y1="4" x2="10" y2="4"></line><line x1="14" y1="20" x2="5" y2="20"></line><line x1="15" y1="4" x2="9" y2="20"></line></svg>
      </button>
      <!-- 删除线 -->
      <button @click="emitCommand('insert-markdown', '~~', '~~')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.strikethroughTooltip')" data-priority="3">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 4H9a3 3 0 0 0-2.83 4"></path><path d="M14 12a4 4 0 0 1 0 8H6"></path><line x1="4" y1="12" x2="20" y2="12"></line></svg>
      </button>
      <!-- 任务列表 -->
      <button @click="emitCommand('insert-markdown', '- [ ] ')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.taskListTooltip')" data-priority="4">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 11H3v2h6v-2z"></path><path d="M9 7H3v2h6V7z"></path><path d="M9 15H3v2h6v-2z"></path><path d="M13 7h8v2h-8V7z"></path><path d="M13 11h8v2h-8v-2z"></path><path d="M13 15h8v2h-8v-2z"></path></svg>
      </button>
      <!-- 链接 -->
      <button @click="emitCommand('insert-markdown', '[', `](${t('noteEditor.linkUrlPlaceholder')})`)" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.linkTooltip')" data-priority="5">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path></svg>
      </button>
      <!-- 引用 -->
      <button @click="emitCommand('insert-markdown', '> ')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.quoteTooltip')" data-priority="6">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 21c3 0 7-1 7-8V5c0-1.25-.756-2.017-2-2H4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2 1 0 1 0 1 1v1c0 1-1 2-2 2s-1 .008-1 1.031V20c0 1 0 1 1 1z"></path><path d="M15 21c3 0 7-1 7-8V5c0-1.25-.757-2.017-2-2h-4c-1.25 0-2 .75-2 1.972V11c0 1.25.75 2 2 2h.75c0 2.25.25 4-2.75 4v3c0 1 0 1 1 1z"></path></svg>
      </button>
      <!-- 代码块 -->
      <button @click="emitCommand('insert-markdown', '```\\n', '\\n```')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.codeBlockTooltip')" data-priority="7">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"></polyline><polyline points="8 6 2 12 8 18"></polyline></svg>
      </button>
      <!-- 表格 -->
      <button @click="emitCommand('insert-table')" class="btn btn-xs btn-ghost toolbar-item" :title="t('noteEditor.tableTooltip')" data-priority="9">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="3" y1="9" x2="21" y2="9"></line><line x1="3" y1="15" x2="21" y2="15"></line><line x1="9" y1="3" x2="9" y2="21"></line><line x1="15" y1="3" x2="15" y2="21"></line></svg>
      </button>

      <!-- 所见即所得模式下的工具 - 已注释 -->
      <!-- <template v-if="isWysiwygMode">
        <button @click="emitCommand('prosemirror-command', 'toggleBold')" class="btn btn-xs btn-ghost" title="Bold">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path></svg>
        </button>
        <button @click="emitCommand('prosemirror-command', 'toggleItalic')" class="btn btn-xs btn-ghost" title="Italic">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="19" y1="4" x2="10" y2="4"></line><line x1="14" y1="20" x2="5" y2="20"></line><line x1="15" y1="4" x2="9" y2="20"></line></svg>
        </button>
        <button @click="emitCommand('prosemirror-command', 'toggleBulletList')" class="btn btn-xs btn-ghost" title="Bullet List">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line></svg>
        </button>
        <button @click="emitCommand('prosemirror-command', 'toggleOrderedList')" class="btn btn-xs btn-ghost" title="Ordered List">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="10" y1="6" x2="21" y2="6"></line><line x1="10" y1="12" x2="21" y2="12"></line><line x1="10" y1="18" x2="21" y2="18"></line><path d="M4 6h1v4"></path><path d="M4 10h2"></path><path d="M6 18H4c0-1 2-2 2-3s-1-1.5-2-1"></path></svg>
        </button>
      </template> -->
    </div>

    <div class="flex items-center gap-2" ref="toolbarRight">

      <!-- 目录 -->
      <button 
        @click="emitCommand('toggle-toc')" 
        class="btn btn-xs btn-ghost" 
        :class="{ 'btn-active': showToc, 'btn-disabled': isEditOnly || isSplitMode }" 
        :disabled="isEditOnly || isSplitMode"
        :title="isEditOnly || isSplitMode ? t('noteEditor.tocDisabledTooltip') : t('noteEditor.toggleTocTooltip')" 
        data-priority="10"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 6h16M4 10h16M4 14h16M4 18h16" /></svg>
      </button>

      <!-- 音频录制 -->
      <button @click="emitCommand('toggle-audio-recording')" class="btn btn-xs btn-ghost" :title="t('noteEditor.audioRecordingTooltip')" data-priority="11">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path><path d="M19 10v2a7 7 0 0 1-14 0v-2"></path><line x1="12" y1="19" x2="12" y2="23"></line></svg>
      </button>

      <!-- 音频播放列表 -->
      <button @click="emitCommand('toggle-audio-player')" class="btn btn-xs btn-ghost" :title="t('noteEditor.audioPlayerTooltip')" data-priority="12">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 18V5l12-2v13"></path><circle cx="6" cy="18" r="3"></circle><circle cx="18" cy="16" r="3"></circle></svg>
      </button>

      <div class="divider divider-horizontal mx-1"></div>

      <!-- 主题选择 -->
      <div class="dropdown dropdown-end">
        <label tabindex="0" class="btn btn-xs btn-ghost" :title="t('noteEditor.themeTooltip')">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z"></path></svg>
        </label>
        <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-52 z-20">
          <li class="menu-title"><span>{{ t('noteEditor.codeTheme') }}</span></li>
          <li><a @click="emitCommand('set-highlight-theme', 'default')" :class="{'active': currentHighlightTheme === 'default'}">Default</a></li>
          <li><a @click="emitCommand('set-highlight-theme', 'okaidia')" :class="{'active': currentHighlightTheme === 'okaidia'}">Okaidia</a></li>
          <li><a @click="emitCommand('set-highlight-theme', 'twilight')" :class="{'active': currentHighlightTheme === 'twilight'}">Twilight</a></li>
          <li><a @click="emitCommand('set-highlight-theme', 'solarized-light')" :class="{'active': currentHighlightTheme === 'solarized-light'}">Solarized Light</a></li>
          <li><a @click="emitCommand('set-highlight-theme', 'tomorrow-night')" :class="{'active': currentHighlightTheme === 'tomorrow-night'}">Tomorrow Night</a></li>
        </ul>
      </div>

      <!-- 全屏切换 -->
      <button @click="emitCommand('toggle-fullscreen')" class="btn btn-xs btn-ghost" :title="isFullscreen ? t('noteEditor.exitFullscreenTooltip') : t('noteEditor.enterFullscreenTooltip')" data-priority="12">
        <svg v-if="!isFullscreen" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 0-2-2h-3M3 16h3a2 2 0 0 0 2 2v3"></path></svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps({
  isFullscreen: Boolean,
  isEditOnly: Boolean,
  isPreviewMode: Boolean,
  isSplitMode: Boolean,
  // isWysiwygMode: Boolean, // WYSIWYG模式已注释
  showToc: Boolean,

  currentHighlightTheme: String,
  currentMarkdownTheme: String,
});

const emit = defineEmits(['command']);

const emitCommand = (command: string, ...args: any[]) => {
  emit('command', command, ...args);
};
</script>

<style scoped>
/* 可以在这里添加一些工具栏的特定样式 */
.toolbar-item {
  flex-shrink: 0;
}
</style>
<template>
  <div class="editor-toolbar bg-base-100 border-b border-base-300" ref="toolbarContainer" @click.self="closeDropdown">
    <div class="toolbar-content">
      <!-- 左侧编辑工具组 -->
      <div class="toolbar-group">
        <!-- 撤销/重做 -->
        <div class="toolbar-section">
          <button class="toolbar-btn" @click="emitCommand('undo')" :title="t('noteEditor.undoTooltip')"
            aria-label="Undo">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="15" height="15" fill="currentColor">
              <path
                d="M5.82843 6.99955L8.36396 9.53509L6.94975 10.9493L2 5.99955L6.94975 1.0498L8.36396 2.46402L5.82843 4.99955H13C17.4183 4.99955 21 8.58127 21 12.9996C21 17.4178 17.4183 20.9996 13 20.9996H4V18.9996H13C16.3137 18.9996 19 16.3133 19 12.9996C19 9.68584 16.3137 6.99955 13 6.99955H5.82843Z">
              </path>
            </svg> </button>
          <button class="toolbar-btn" @click="emitCommand('redo')" :title="t('noteEditor.redoTooltip')"
            aria-label="Redo">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="15" height="15" fill="currentColor">
              <path
                d="M18.1716 6.99955H11C7.68629 6.99955 5 9.68584 5 12.9996C5 16.3133 7.68629 18.9996 11 18.9996H20V20.9996H11C6.58172 20.9996 3 17.4178 3 12.9996C3 8.58127 6.58172 4.99955 11 4.99955H18.1716L15.636 2.46402L17.0503 1.0498L22 5.99955L17.0503 10.9493L15.636 9.53509L18.1716 6.99955Z">
              </path>
            </svg> </button>
        </div>
        <div class="tiptap-separator" data-orientation="vertical" role="none"></div>



        <!-- 列表和代码 -->
        <div class="toolbar-section">
          <!-- 标题和文本 -->
          <div class="toolbar-section">
            <div class="dropdown" :class="{ 'dropdown-open': openDropdown === 'heading' }">
              <button class="toolbar-btn dropdown-toggle" @click="toggleDropdown('heading', $event)"
                :title="t('noteEditor.headingTooltip')" aria-label="Heading" ref="headingButton">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                  <path d="M17 11V4H19V21H17V13H7V21H5V4H7V11H17Z"></path>
                </svg>
                <i class="ri-arrow-down-s-line text-xs"></i>

              </button>
            </div>
          </div>
          <!-- 列表 -->
          <div class="dropdown" :class="{ 'dropdown-open': openDropdown === 'list' }">
            <button class="toolbar-btn dropdown-toggle" @click="toggleDropdown('list', $event)"
              :title="t('noteEditor.listTooltip') || 'Lists'" aria-label="List" ref="listButton">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 4H21V6H8V4ZM4.5 6.5C3.67157 6.5 3 5.82843 3 5C3 4.17157 3.67157 3.5 4.5 3.5C5.32843 3.5 6 4.17157 6 5C6 5.82843 5.32843 6.5 4.5 6.5ZM4.5 13.5C3.67157 13.5 3 12.8284 3 12C3 11.1716 3.67157 10.5 4.5 10.5C5.32843 10.5 6 11.1716 6 12C6 12.8284 5.32843 13.5 4.5 13.5ZM4.5 20.4C3.67157 20.4 3 19.7284 3 18.9C3 18.0716 3.67157 17.4 4.5 17.4C5.32843 17.4 6 18.0716 6 18.9C6 19.7284 5.32843 20.4 4.5 20.4ZM8 11H21V13H8V11ZM8 18H21V20H8V18Z"></path></svg>
               <i class="ri-arrow-down-s-line text-xs"></i>

            </button>
          </div>
          <!-- 引用 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-blockquote')" :title="t('noteEditor.quoteTooltip')"
            aria-label="Blockquote">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19.4167 6.67891C20.4469 7.77257 21.0001 9 21.0001 10.9897C21.0001 14.4891 18.5436 17.6263 14.9695 19.1768L14.0768 17.7992C17.4121 15.9946 18.0639 13.6539 18.3245 12.178C17.7875 12.4557 17.0845 12.5533 16.3954 12.4895C14.591 12.3222 13.1689 10.8409 13.1689 9C13.1689 7.067 14.7359 5.5 16.6689 5.5C17.742 5.5 18.7681 5.99045 19.4167 6.67891ZM9.41669 6.67891C10.4469 7.77257 11.0001 9 11.0001 10.9897C11.0001 14.4891 8.54359 17.6263 4.96951 19.1768L4.07682 17.7992C7.41206 15.9946 8.06392 13.6539 8.32447 12.178C7.78747 12.4557 7.08452 12.5533 6.39539 12.4895C4.59102 12.3222 3.16895 10.8409 3.16895 9C3.16895 7.067 4.73595 5.5 6.66895 5.5C7.742 5.5 8.76814 5.99045 9.41669 6.67891Z"></path></svg>
          </button>
          <!-- 代码块 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-codeblock')" :title="t('noteEditor.codeBlockTooltip')"
            aria-label="Code Block">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3.41436 5.99995L5.70726 3.70706L4.29304 2.29285L0.585938 5.99995L4.29304 9.70706L5.70726 8.29285L3.41436 5.99995ZM9.58594 5.99995L7.29304 3.70706L8.70726 2.29285L12.4144 5.99995L8.70726 9.70706L7.29304 8.29285L9.58594 5.99995ZM14.0002 2.99995H21.0002C21.5524 2.99995 22.0002 3.44767 22.0002 3.99995V20C22.0002 20.5522 21.5524 21 21.0002 21H3.00015C2.44787 21 2.00015 20.5522 2.00015 20V12H4.00015V19H20.0002V4.99995H14.0002V2.99995Z"></path></svg>          </button>
        </div>

        <div class="tiptap-separator" data-orientation="vertical" role="none"></div>

        <!-- 文本样式 -->
        <div class="toolbar-section">
          <!-- 粗体 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-bold')" :title="t('noteEditor.boldTooltip')"
            aria-label="Bold">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path
                d="M8 11H12.5C13.8807 11 15 9.88071 15 8.5C15 7.11929 13.8807 6 12.5 6H8V11ZM18 15.5C18 17.9853 15.9853 20 13.5 20H6V4H12.5C14.9853 4 17 6.01472 17 8.5C17 9.70431 16.5269 10.7981 15.7564 11.6058C17.0979 12.3847 18 13.837 18 15.5ZM8 13V18H13.5C14.8807 18 16 16.8807 16 15.5C16 14.1193 14.8807 13 13.5 13H8Z">
              </path>
            </svg> </button>
          <!-- 斜体 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-italic')" :title="t('noteEditor.italicTooltip')"
            aria-label="Italic">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path d="M15 20H7V18H9.92661L12.0425 6H9V4H17V6H14.0734L11.9575 18H15V20Z"></path>
            </svg>
          </button>
          <!-- 删除线 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-strike')"
            :title="t('noteEditor.strikethroughTooltip')" aria-label="Strikethrough">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path
                d="M17.1538 14C17.3846 14.5161 17.5 15.0893 17.5 15.7196C17.5 17.0625 16.9762 18.1116 15.9286 18.867C14.8809 19.6223 13.4335 20 11.5862 20C9.94674 20 8.32335 19.6185 6.71592 18.8555V16.6009C8.23538 17.4783 9.7908 17.917 11.3822 17.917C13.9333 17.917 15.2128 17.1846 15.2208 15.7196C15.2208 15.0939 15.0049 14.5598 14.5731 14.1173C14.5339 14.0772 14.4939 14.0381 14.4531 14H3V12H21V14H17.1538ZM13.076 11H7.62908C7.4566 10.8433 7.29616 10.6692 7.14776 10.4778C6.71592 9.92084 6.5 9.24559 6.5 8.45207C6.5 7.21602 6.96583 6.165 7.89749 5.299C8.82916 4.43299 10.2706 4 12.2219 4C13.6934 4 15.1009 4.32808 16.4444 4.98426V7.13591C15.2448 6.44921 13.9293 6.10587 12.4978 6.10587C10.0187 6.10587 8.77917 6.88793 8.77917 8.45207C8.77917 8.87172 8.99709 9.23796 9.43293 9.55079C9.86878 9.86362 10.4066 10.1135 11.0463 10.3004C11.6665 10.4816 12.3431 10.7148 13.076 11H13.076Z">
              </path>
            </svg> </button>
          <!-- 代码 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-code')" :title="t('noteEditor.codeTooltip')"
            aria-label="Code">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path
                d="M24 12L18.3431 17.6569L16.9289 16.2426L21.1716 12L16.9289 7.75736L18.3431 6.34315L24 12ZM2.82843 12L7.07107 16.2426L5.65685 17.6569L0 12L5.65685 6.34315L7.07107 7.75736L2.82843 12ZM9.78845 21H7.66009L14.2116 3H16.3399L9.78845 21Z">
              </path>
            </svg>
          </button>
          <!-- 下划线 -->
          <button class="toolbar-btn" @click="emitCommand('tiptap-underline')" :title="t('noteEditor.underlineTooltip')"
            aria-label="Underline">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 3V12C8 14.2091 9.79086 16 12 16C14.2091 16 16 14.2091 16 12V3H18V12C18 15.3137 15.3137 18 12 18C8.68629 18 6 15.3137 6 12V3H8ZM4 20H20V22H4V20Z"></path></svg>
          </button>
        </div>

        <!-- 高亮和颜色 -->
        <div class="toolbar-section">
          <button class="toolbar-btn" @click="emitCommand('toggle-highlight')" :title="t('noteEditor.highlightTooltip')"
            aria-label="Highlight">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path
                d="M5 18.89H6.41421L15.7279 9.57627L14.3137 8.16206L5 17.4758V18.89ZM21 20.89H3V16.6473L16.435 3.21231C16.8256 2.82179 17.4587 2.82179 17.8492 3.21231L20.6777 6.04074C21.0682 6.43126 21.0682 7.06443 20.6777 7.45495L9.24264 18.89H21V20.89ZM15.7279 6.74785L17.1421 8.16206L18.5563 6.74785L17.1421 5.33363L15.7279 6.74785Z">
              </path>
            </svg></button>
        </div>

        <!-- 链接 -->
        <div class="toolbar-section">
          <button class="toolbar-btn" @click="emitCommand('toggle-link')" :title="t('noteEditor.linkTooltip')"
            aria-label="link">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
              <path
                d="M13.0607 8.11097L14.4749 9.52518C17.2086 12.2589 17.2086 16.691 14.4749 19.4247L14.1214 19.7782C11.3877 22.5119 6.95555 22.5119 4.22188 19.7782C1.48821 17.0446 1.48821 12.6124 4.22188 9.87874L5.6361 11.293C3.68348 13.2456 3.68348 16.4114 5.6361 18.364C7.58872 20.3166 10.7545 20.3166 12.7072 18.364L13.0607 18.0105C15.0133 16.0578 15.0133 12.892 13.0607 10.9394L11.6465 9.52518L13.0607 8.11097ZM19.7782 14.1214L18.364 12.7072C20.3166 10.7545 20.3166 7.58872 18.364 5.6361C16.4114 3.68348 13.2456 3.68348 11.293 5.6361L10.9394 5.98965C8.98678 7.94227 8.98678 11.1081 10.9394 13.0607L12.3536 14.4749L10.9394 15.8891L9.52518 14.4749C6.79151 11.7413 6.79151 7.30911 9.52518 4.57544L9.87874 4.22188C12.6124 1.48821 17.0446 1.48821 19.7782 4.22188C22.5119 6.95555 22.5119 11.3877 19.7782 14.1214Z">
              </path>
            </svg>
          </button>
        </div>

        <div class="tiptap-separator" data-orientation="vertical" role="none"></div>


      </div>

      <!-- 右侧工具组 -->
      <div class="toolbar-group ml-auto">
        <!-- 音频 -->
        <div class="toolbar-section">
          <button class="toolbar-btn" @click="emitCommand('toggle-audio-recording')"
            :title="t('noteEditor.audioRecordingTooltip')" aria-label="Audio Recording">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M11.9998 3C10.3429 3 8.99976 4.34315 8.99976 6V10C8.99976 11.6569 10.3429 13 11.9998 13C13.6566 13 14.9998 11.6569 14.9998 10V6C14.9998 4.34315 13.6566 3 11.9998 3ZM11.9998 1C14.7612 1 16.9998 3.23858 16.9998 6V10C16.9998 12.7614 14.7612 15 11.9998 15C9.23833 15 6.99976 12.7614 6.99976 10V6C6.99976 3.23858 9.23833 1 11.9998 1ZM3.05469 11H5.07065C5.55588 14.3923 8.47329 17 11.9998 17C15.5262 17 18.4436 14.3923 18.9289 11H20.9448C20.4837 15.1716 17.1714 18.4839 12.9998 18.9451V23H10.9998V18.9451C6.82814 18.4839 3.51584 15.1716 3.05469 11Z"></path></svg>
          </button>
          <button class="toolbar-btn" @click="emitCommand('toggle-audio-player')"
            :title="t('noteEditor.audioPlayerTooltip')" aria-label="Audio Player">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 3V17C20 19.2091 18.2091 21 16 21C13.7909 21 12 19.2091 12 17C12 14.7909 13.7909 13 16 13C16.7286 13 17.4117 13.1948 18 13.5351V5H9V17C9 19.2091 7.20914 21 5 21C2.79086 21 1 19.2091 1 17C1 14.7909 2.79086 13 5 13C5.72857 13 6.41165 13.1948 7 13.5351V3H20ZM5 19C6.10457 19 7 18.1046 7 17C7 15.8954 6.10457 15 5 15C3.89543 15 3 15.8954 3 17C3 18.1046 3.89543 19 5 19ZM16 19C17.1046 19 18 18.1046 18 17C18 15.8954 17.1046 15 16 15C14.8954 15 14 15.8954 14 17C14 18.1046 14.8954 19 16 19Z"></path></svg>
          </button>
        </div>

        <!-- 目录 -->
        <div class="toolbar-section">
          <button class="toolbar-btn" :class="{ active: showToc }" @click="emitCommand('toggle-toc')"
            :title="t('noteEditor.toggleTocTooltip')" aria-label="Table of Contents">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M20 22H4C3.44772 22 3 21.5523 3 21V3C3 2.44772 3.44772 2 4 2H20C20.5523 2 21 2.44772 21 3V21C21 21.5523 20.5523 22 20 22ZM19 20V4H5V20H19ZM8 7H16V9H8V7ZM8 11H16V13H8V11ZM8 15H16V17H8V15Z"></path></svg>
          </button>
        </div>

        <!-- 主题和显示 -->
        <div class="toolbar-section">
          <!-- <div class="dropdown" :class="{ 'dropdown-open': openDropdown === 'theme' }">
            <button class="toolbar-btn dropdown-toggle" @click="toggleDropdown('theme', $event)"
              :title="t('noteEditor.themeTooltip')" aria-label="Theme" ref="themeButton">
              <i class="ri-palette-line"></i>
              <i class="ri-arrow-down-s-line text-xs"></i>
            </button>
          </div> -->

          <!-- 全屏 -->
          <button class="toolbar-btn" @click="emitCommand('toggle-fullscreen')"
            :title="isFullscreen ? t('noteEditor.exitFullscreenTooltip') : t('noteEditor.enterFullscreenTooltip')"
            aria-label="Fullscreen">
            <div v-if="!isFullscreen">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 3V5H4V9H2V3H8ZM2 21V15H4V19H8V21H2ZM22 21H16V19H20V15H22V21ZM22 9H20V5H16V3H22V9Z"></path></svg>
            </div>
            <div v-else>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M18 7H22V9H16V3H18V7ZM8 9H2V7H6V3H8V9ZM18 17V21H16V15H22V17H18ZM8 15V21H6V17H2V15H8Z"></path></svg>
            </div>
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- 使用 Teleport 将下拉菜单挪到 body，避免被父容器遮挡 -->
  <Teleport to="body">
    <div v-if="openDropdown === 'heading' && headingButtonRect" class="dropdown-menu dropdown-menu-teleport" :style="{
      position: 'fixed',
      top: (headingButtonRect.bottom + 4) + 'px',
      left: headingButtonRect.left + 'px',
      zIndex: 99999
    }" @click.stop>
      <button @click="emitCommand('tiptap-h1')" class="dropdown-item">
        <i class="ri-h-1"></i> H1
      </button>
      <button @click="emitCommand('tiptap-h2')" class="dropdown-item">
        <i class="ri-h-2"></i> H2
      </button>
      <button @click="emitCommand('tiptap-h3')" class="dropdown-item">
        <i class="ri-h-3"></i> H3
      </button>
      <button @click="emitCommand('tiptap-paragraph')" class="dropdown-item">
        <i class="ri-text"></i> Paragraph
      </button>
    </div>
  </Teleport>

  <!-- 列表下拉菜单 Teleport -->
  <Teleport to="body">
    <div v-if="openDropdown === 'list' && listButtonRect" class="dropdown-menu dropdown-menu-teleport" :style="{
      position: 'fixed',
      top: (listButtonRect.bottom + 4) + 'px',
      left: listButtonRect.left + 'px',
      zIndex: 99999
    }" @click.stop>
      <button @click="emitCommand('tiptap-bulletlist')" class="dropdown-item">
        <i class="ri-list-unordered"></i> Bullet List
      </button>
      <button @click="emitCommand('tiptap-orderedlist')" class="dropdown-item">
        <i class="ri-list-ordered"></i> Ordered List
      </button>
      <button @click="emitCommand('tiptap-tasklist')" class="dropdown-item">
        <i class="ri-checkbox-line"></i> Task List
      </button>
    </div>
  </Teleport>

  <!-- 主题下拉菜单 Teleport -->
  <Teleport to="body">
    <div v-if="openDropdown === 'theme' && themeButtonRect" class="dropdown-menu dropdown-menu-teleport" :style="{
      position: 'fixed',
      top: (themeButtonRect.bottom + 4) + 'px',
      left: themeButtonRect.left + 'px',
      zIndex: 99999
    }" @click.stop>
      <div class="dropdown-label">{{ t('noteEditor.codeTheme') }}</div>
      <button @click="emitCommand('set-highlight-theme', 'default')" class="dropdown-item"
        :class="{ 'active': currentHighlightTheme === 'default' }">
        <i class="ri-check-line" v-if="currentHighlightTheme === 'default'"></i>
        Default
      </button>
      <button @click="emitCommand('set-highlight-theme', 'okaidia')" class="dropdown-item"
        :class="{ 'active': currentHighlightTheme === 'okaidia' }">
        <i class="ri-check-line" v-if="currentHighlightTheme === 'okaidia'"></i>
        Okaidia
      </button>
      <button @click="emitCommand('set-highlight-theme', 'twilight')" class="dropdown-item"
        :class="{ 'active': currentHighlightTheme === 'twilight' }">
        <i class="ri-check-line" v-if="currentHighlightTheme === 'twilight'"></i>
        Twilight
      </button>
      <button @click="emitCommand('set-highlight-theme', 'solarized-light')" class="dropdown-item"
        :class="{ 'active': currentHighlightTheme === 'solarized-light' }">
        <i class="ri-check-line" v-if="currentHighlightTheme === 'solarized-light'"></i>
        Solarized Light
      </button>
      <button @click="emitCommand('set-highlight-theme', 'tomorrow-night')" class="dropdown-item"
        :class="{ 'active': currentHighlightTheme === 'tomorrow-night' }">
        <i class="ri-check-line" v-if="currentHighlightTheme === 'tomorrow-night'"></i>
        Tomorrow Night
      </button>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, ref, onMounted, onBeforeUnmount } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps({
  isFullscreen: Boolean,
  showToc: Boolean,
  currentHighlightTheme: String,
  currentMarkdownTheme: String,
  isEditOnly: Boolean,
  isPreviewMode: Boolean,
  isSplitMode: Boolean,
});

const emit = defineEmits(['command']);

// 下拉菜单状态
const openDropdown = ref<string | null>(null);
const headingButton = ref<HTMLElement | null>(null);
const listButton = ref<HTMLElement | null>(null);
const themeButton = ref<HTMLElement | null>(null);
const headingButtonRect = ref<DOMRect | null>(null);
const listButtonRect = ref<DOMRect | null>(null);
const themeButtonRect = ref<DOMRect | null>(null);

const emitCommand = (command: string, ...args: any[]) => {
  emit('command', command, ...args);
  openDropdown.value = null; // 选择后关闭下拉菜单
};

const toggleDropdown = (dropdownName: string, event?: Event) => {
  event?.stopPropagation();

  if (dropdownName === 'heading' && headingButton.value) {
    headingButtonRect.value = headingButton.value.getBoundingClientRect();
  } else if (dropdownName === 'list' && listButton.value) {
    listButtonRect.value = listButton.value.getBoundingClientRect();
  } else if (dropdownName === 'theme' && themeButton.value) {
    themeButtonRect.value = themeButton.value.getBoundingClientRect();
  }

  openDropdown.value = openDropdown.value === dropdownName ? null : dropdownName;
};

const closeDropdown = () => {
  openDropdown.value = null;
};

// 点击外部关闭下拉菜单
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  if (!target.closest('.dropdown') && !target.closest('.dropdown-menu-teleport')) {
    closeDropdown();
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<style lang="scss" scoped>
.editor-toolbar {
  display: flex;
  align-items: center;
  padding: 0.5rem 1rem;
  background-color: var(--color-base-100);
  border-bottom: 1px solid var(--color-base-300);
  min-height: 48px;
  overflow-x: auto;
  overflow-y: visible;
  position: relative;
  z-index: 9999;

  &::-webkit-scrollbar {
    height: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: var(--color-base-300);
    border-radius: 2px;

    &:hover {
      background: var(--color-base-400);
    }
  }
}

.toolbar-content {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
  flex-wrap: wrap;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.toolbar-section {
  display: flex;
  align-items: center;
  padding: 0 0.25rem;
  border-right: 1px solid var(--color-base-300);
  position: static;
  overflow: visible;

  &:last-child {
    border-right: none;
  }
}

.toolbar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 32px;
  height: 32px;
  padding: 0 0.5rem;
  border: none;
  background: transparent;
  color: var(--color-base-content);
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s ease;
  position: relative;

  i {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  &:hover:not(:disabled) {
    background-color: var(--color-base-200);
    color: var(--color-primary);
  }

  &:active:not(:disabled) {
    background-color: var(--color-base-300);
    transform: scale(0.95);
  }

  &.active {
    background-color: var(--color-primary);
    color: var(--color-primary-content);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &.dropdown-toggle {
    display: flex;
    align-items: center;
    gap: 0.25rem;

    .ri-arrow-down-s-line {
      transition: transform 0.2s ease;
    }
  }

  &:focus {
    outline: none;
  }

  &:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }
}

.tiptap-button-dropdown-small {
  width: .625rem;
  height: .625rem;

}

// 下拉菜单
.dropdown {
  position: relative;
  display: inline-block;
  z-index: 999999;

  .dropdown-toggle::after {
    content: '';
  }
}

.dropdown-menu {
  position: fixed !important;
  min-width: 200px;
  background: var(--color-base-100, #ffffff) !important;
  border: 1px solid var(--color-base-300, #e5e7eb);
  border-radius: 6px;
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  z-index: 99999 !important;
  margin-top: 4px;
  padding: 0.5rem;
  display: block !important;
  opacity: 1 !important;
  visibility: visible !important;
  animation: dropdown-fade-in 0.15s ease-out;
}

@keyframes dropdown-fade-in {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dropdown-left {
  .dropdown-menu {
    left: auto;
    right: 0;
  }
}

.dropdown-label {
  display: block;
  padding: 0.5rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-base-content);
  opacity: 0.6;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.5rem 0.75rem;
  border: none;
  background: transparent;
  color: var(--color-base-content);
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
  text-align: left;

  i {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    font-size: 0.875rem;
  }

  &:hover {
    background-color: var(--color-base-200);
    color: var(--color-primary);
  }

  &.active {
    background-color: var(--color-primary);
    color: var(--color-primary-content);

    i {
      color: currentColor;
    }
  }

  &:first-of-type {
    margin-top: 0;
  }
}

// 响应式设计
@media (max-width: 768px) {
  .editor-toolbar {
    padding: 0.5rem;
  }

  .toolbar-section {
    padding: 0 0.125rem;
    border-right-width: 0;
  }

  .toolbar-btn {
    min-width: 28px;
    height: 28px;
    padding: 0 0.25rem;
    font-size: 0.875rem;
  }

  .dropdown-menu {
    min-width: 160px;
  }
}

@media (max-width: 480px) {
  .toolbar-section {
    padding: 0;
  }

  .toolbar-btn {
    min-width: 24px;
    height: 24px;
    font-size: 0.75rem;
  }
}

.tiptap-separator {
  height: 1.5rem;
  width: 1px;
  flex-shrink: 0;
  background-color: var(--tt-link-border-color);
  --tt-link-border-color: var(--tt-gray-light-a-200);
}
</style>
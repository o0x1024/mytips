<template>
  <div class="p-2 border-b border-base-300 flex items-center justify-between">
    <!-- 标题和状态区 -->
    <div class="flex-1">
      <input 
        type="text" 
        placeholder="无标题笔记..."
        class="input input-lg w-full text-xl font-bold p-0 border-0 focus:outline-none bg-transparent"
        :value="title"
        @input="emit('update:title', ($event.target as HTMLInputElement).value); emit('input')"
        @blur="emit('title-blur')"
      />
    </div>

    <!-- 操作按钮区 -->
    <div class="flex items-center gap-2">
      <!-- 全屏按钮 -->
      <button 
        class="btn btn-sm btn-ghost btn-square" 
        :class="{ 'btn-active': isFullscreen }"
        :title="isFullscreen ? '退出全屏 (ESC)' : '全屏编辑 (F11)'" 
        @click="emit('command', 'toggle-fullscreen')">
        <svg v-if="!isFullscreen" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5v-4m0 4h-4m4 0l-5-5" />
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <!-- AI扩充按钮 -->
      <button class="btn btn-sm btn-ghost btn-square" title="AI扩充内容" @click="emit('command', 'expand-with-ai')">
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
          <li><a @click="emit('command', 'share-note')">分享链接</a></li>
          <li><a @click="emit('command', 'export-note')">导出</a></li>
          <li><a @click="emit('command', 'duplicate-note')">创建副本</a></li>
          <li><a @click="emit('command', 'delete-note')" class="text-error">删除</a></li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  title: {
    type: String,
    required: true,
  },
  isFullscreen: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:title', 'command', 'title-blur', 'input']);
</script> 
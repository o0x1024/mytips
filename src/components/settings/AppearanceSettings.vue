<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">外观设置</h2>

      <!-- 主题选择 -->
      <div class="form-control mb-6">
        <label class="label">
          <span class="label-text">主题</span>
        </label>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
          <template v-for="theme in uiStore.themeOptions" :key="theme.value">
            <label :class="[
              'cursor-pointer rounded-lg border-2 px-3 py-2 flex items-center gap-2',
              uiStore.settings.theme === theme.value
                ? 'border-primary'
                : 'border-base-300'
            ]" @click="changeTheme(theme.value)">
              <input type="radio" :value="theme.value" v-model="selectedTheme"
                class="radio radio-primary radio-sm" />
              <span>{{ theme.name }}</span>
            </label>
          </template>
        </div>

        <!-- 主题预览区域 -->
        <div class="mt-6 p-4 rounded-lg border border-base-300">
          <h3 class="text-lg font-medium mb-4">主题预览</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <!-- 按钮预览 -->
            <div class="space-y-2">
              <button class="btn btn-primary w-full">主要按钮</button>
              <button class="btn btn-secondary w-full">次要按钮</button>
              <button class="btn btn-accent w-full">强调按钮</button>
              <button class="btn btn-neutral w-full">中性按钮</button>
            </div>
            <!-- 颜色预览 -->
            <div class="grid grid-cols-2 gap-2">
              <div class="p-3 rounded-lg bg-primary text-primary-content text-center">主色</div>
              <div class="p-3 rounded-lg bg-secondary text-secondary-content text-center">次色</div>
              <div class="p-3 rounded-lg bg-accent text-accent-content text-center">强调色</div>
              <div class="p-3 rounded-lg bg-neutral text-neutral-content text-center">中性色</div>
              <div class="p-3 rounded-lg bg-base-100 text-base-content border border-base-300 text-center">背景色</div>
              <div class="p-3 rounded-lg bg-base-200 text-base-content border border-base-300 text-center">背景色 2</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 字体大小设置 -->
      <div class="form-control mb-6">
        <label class="label">
          <span class="label-text">字体大小</span>
        </label>

        <!-- 精确字体大小调整 -->
        <div class="mt-2">
          <label class="label">
            <span class="label-text">字体大小 ({{ exactFontSize }}px)</span>
          </label>
          <div class="flex items-center gap-2">
            <button class="btn btn-sm btn-circle" @click="decreaseExactFontSize"
              :disabled="exactFontSize <= 10">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
              </svg>
            </button>
            <input type="range" min="10" max="24" step="1" v-model.number="exactFontSize"
              class="range range-primary range-sm flex-1" @input="updateExactFontSize" />
            <button class="btn btn-sm btn-circle" @click="increaseExactFontSize"
              :disabled="exactFontSize >= 24">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </button>
          </div>
        </div>

        <!-- 字体大小预览 -->
        <div class="mt-4 p-4 rounded-lg border border-base-300">
          <h3 class="text-lg font-medium mb-2">字体大小预览</h3>
          <div class="space-y-3">
            <div :style="`font-size: ${exactFontSize}px;`" class="p-2 border border-base-200 rounded">
              <span>当前选择的字体大小 ({{ exactFontSize }}px)</span>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
              <div :style="`font-size: 12px;`" class="p-2 border border-base-200 rounded">
                <span>12px 大小示例</span>
              </div>
              <div :style="`font-size: 16px;`" class="p-2 border border-base-200 rounded">
                <span>16px 大小示例</span>
              </div>
              <div :style="`font-size: 20px;`" class="p-2 border border-base-200 rounded">
                <span>20px 大小示例</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 重置按钮 -->
      <div class="form-control mt-6">
        <button class="btn btn-outline" @click="resetUISettings">
          重置为默认设置
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useUIStore } from '../../stores/uiStore'

const uiStore = useUIStore()

// 主题设置
const selectedTheme = ref(uiStore.settings.theme)
const exactFontSize = ref<number>(Number(uiStore.settings.exactFontSize))

// 精确字体大小相关方法
function updateExactFontSize() {
  const size = Number(exactFontSize.value)
  if (!isNaN(size) && size >= 10 && size <= 24) {
    uiStore.setExactFontSize(size)
  }
}

function increaseExactFontSize() {
  if (exactFontSize.value < 24) {
    exactFontSize.value++
    updateExactFontSize()
  }
}

function decreaseExactFontSize() {
  if (exactFontSize.value > 10) {
    exactFontSize.value--
    updateExactFontSize()
  }
}

// 更改主题
function changeTheme(theme: string) {
  selectedTheme.value = theme
  uiStore.setTheme(theme)
}

// 重置UI设置
function resetUISettings() {
  uiStore.resetToDefaults()
  selectedTheme.value = uiStore.settings.theme
  exactFontSize.value = uiStore.settings.exactFontSize
}
</script> 
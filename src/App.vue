<script setup lang="ts">
// App.vue 无需额外初始化，因为已经在main.ts中完成了UI初始化
import FloatingAIButton from './components/FloatingAIButton.vue'
import ToastContainer from './components/ToastContainer.vue'
</script>

<template>
  <div class="app">
    <router-view v-slot="{ Component, route }">
      <template v-if="route.meta.keepAlive">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </template>
      <component 
        v-else
        :is="Component" 
      />
    </router-view>
    
    <!-- 全局 Toast 通知 -->
    <ToastContainer />

    <!-- 全局浮动AI按钮 -->
    <FloatingAIButton />
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.app {
  width: 100%;
  height: 100vh;
  overflow: hidden;
  font-size: var(--base-font-size);
}

* {
  font-size: inherit;
}

/* DaisyUI 覆盖样式 */
.btn {
  text-transform: none;
}

.prose {
  max-width: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>
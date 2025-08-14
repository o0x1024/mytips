<template>
  <div class="toast toast-end toast-top z-[10000]">
    <TransitionGroup name="toast" tag="div">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        :class="[
          'alert',
          alertClass(toast.type),
          'shadow-lg',
          'mb-2',
          'items-start',
          'gap-2',
          'min-w-0',
          'w-auto',
          'max-w-[90vw]',
          'sm:max-w-md',
          'md:max-w-lg',
          'lg:max-w-xl',
          'overflow-y-auto',
          'max-h-[50vh]'
        ]"
      >
        <span class="flex-1 break-words break-all whitespace-pre-wrap">{{ toast.message }}</span>
        <button class="btn btn-xs btn-ghost" @click="close(toast.id)">✕</button>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useNotificationStore } from '../stores/notificationStore'

const store = useNotificationStore()
const toasts = computed(() => store.toasts)

function close(id: string) {
  store.removeToast(id)
}

function alertClass(type: string) {
  switch (type) {
    case 'success':
      return 'alert-success'
    case 'error':
      return 'alert-error'
    case 'warning':
      return 'alert-warning'
    default:
      return 'alert-info'
  }
}
</script>

<style scoped>
/* 进入/离开动画 */
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease;
}
</style> 
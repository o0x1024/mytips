<template>
  <dialog ref="dialogRef" class="modal">
    <div class="modal-box">
      <!-- 图标和标题 -->
      <div class="flex items-center gap-3 mb-4">
        <!-- 确认对话框图标 -->
        <div v-if="type === 'confirm'" class="flex-shrink-0">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        
        <!-- 提示对话框图标 -->
        <div v-else class="flex-shrink-0">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-info" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        
        <h3 class="font-bold text-lg">{{ title }}</h3>
      </div>
      
      <!-- 消息内容 -->
      <div class="py-4">
        <p class="text-base-content" v-html="formattedMessage"></p>
      </div>
      
      <!-- 按钮区域 -->
      <div class="modal-action">
        <!-- 确认对话框的按钮 -->
        <template v-if="type === 'confirm'">
          <button class="btn btn-ghost" @click="handleCancel">
            {{ cancelText }}
          </button>
          <button class="btn btn-primary" @click="handleConfirm">
            {{ confirmText }}
          </button>
        </template>
        
        <!-- 提示对话框的按钮 -->
        <template v-else>
          <button class="btn btn-primary" @click="handleConfirm">
            {{ confirmText }}
          </button>
        </template>
      </div>
    </div>
    
    <!-- 点击背景关闭 -->
    <div class="modal-backdrop" @click="handleCancel"></div>
  </dialog>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'

interface Props {
  type?: 'confirm' | 'alert'
  title?: string
  message: string
  confirmText?: string
  cancelText?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'alert',
  title: '',
  confirmText: '确定',
  cancelText: '取消'
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const dialogRef = ref<HTMLDialogElement>()

// 格式化消息，支持换行
const formattedMessage = computed(() => {
  return props.message.replace(/\n/g, '<br>')
})

// 计算默认标题
const title = computed(() => {
  if (props.title) return props.title
  return props.type === 'confirm' ? '确认操作' : '提示'
})

// 显示对话框
const show = async () => {
  await nextTick()
  dialogRef.value?.showModal()
}

// 隐藏对话框
const hide = () => {
  dialogRef.value?.close()
}

// 处理确认
const handleConfirm = () => {
  emit('confirm')
  hide()
}

// 处理取消
const handleCancel = () => {
  emit('cancel')
  hide()
}

// 暴露方法给父组件
defineExpose({
  show,
  hide
})
</script>

<style scoped>
/* 确保对话框在最顶层 */
.modal {
  z-index: 9999;
}

/* 消息内容样式 */
.modal-box p {
  line-height: 1.6;
  word-wrap: break-word;
}

/* 按钮间距 */
.modal-action {
  gap: 0.5rem;
}
</style> 
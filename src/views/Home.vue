<template>
  <MainLayout />
</template>

<script setup lang="ts">
import MainLayout from '../components/MainLayout.vue'
import { onActivated, onDeactivated } from 'vue'

onActivated(() => {
  // 不执行任何数据加载操作，保持当前状态
  // 子组件将通过各自的onActivated钩子处理激活逻辑
})

onDeactivated(() => {
  console.log('Home组件被停用')
})
</script> 
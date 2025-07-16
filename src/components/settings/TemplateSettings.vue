<template>
  <div class="card bg-base-100 shadow-md">
    <div class="card-body">
      <h2 class="card-title text-primary mb-4">提示词模板管理</h2>

      <!-- 新增/编辑模板表单 -->
      <div class="form-control mb-3">
        <label class="label">
          <span class="label-text">模板名称</span>
        </label>
        <input type="text" v-model="templateName" placeholder="输入模板名称" class="input input-bordered w-full" />
      </div>
      <div class="form-control mb-3">
        <label class="label">
          <span class="label-text">模板内容</span>
        </label>
        <textarea v-model="templateContent" class="textarea textarea-bordered w-full h-32" placeholder="输入模板内容"></textarea>
      </div>
      <div class="flex gap-2 mb-4">
        <button class="btn btn-primary" @click="saveTemplate" :disabled="!templateName.trim() || !templateContent.trim()">
          {{ isEditingTemplate ? '更新模板' : '添加模板' }}
        </button>
        <button v-if="isEditingTemplate" class="btn btn-ghost" @click="cancelEdit">取消编辑</button>
      </div>

      <div class="divider"></div>

      <!-- 模板列表 -->
      <div v-if="templateStore.isLoading.value" class="flex justify-center py-6">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <div v-else>
        <div v-if="templateStore.templates.value.length === 0" class="text-center text-base-content/60">暂无模板</div>
        <div v-else class="space-y-2">
          <div v-for="tpl in templateStore.templates.value" :key="tpl.name" class="flex items-start justify-between p-3 bg-base-200 rounded">
            <div class="flex-1">
              <div class="font-medium">{{ tpl.name }}</div>
              <div class="text-sm text-base-content/70 whitespace-pre-wrap">{{ tpl.content }}</div>
            </div>
            <div class="flex gap-2 ml-3">
              <button class="btn btn-xs btn-outline" @click="editTemplate(tpl)">编辑</button>
              <button class="btn btn-xs btn-error btn-outline" @click="deleteTemplate(tpl.name)">删除</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTipTemplateStore } from '../../stores/tipTemplateStore'

const templateStore = useTipTemplateStore()
const templateName = ref('')
const templateContent = ref('')
const isEditingTemplate = ref(false)
let originalTemplateName = ''

function resetTemplateForm() {
  templateName.value = ''
  templateContent.value = ''
  isEditingTemplate.value = false
  originalTemplateName = ''
}

async function saveTemplate() {
  if (!templateName.value.trim() || !templateContent.value.trim()) return
  await templateStore.addTemplate(templateName.value.trim(), templateContent.value.trim())
  if (isEditingTemplate.value && originalTemplateName && originalTemplateName !== templateName.value.trim()) {
    await templateStore.deleteTemplate(originalTemplateName)
  }
  resetTemplateForm()
}

function editTemplate(tpl: { name: string; content: string }) {
  templateName.value = tpl.name
  templateContent.value = tpl.content
  isEditingTemplate.value = true
  originalTemplateName = tpl.name
}

async function deleteTemplate(name: string) {
  await templateStore.deleteTemplate(name)
  if (isEditingTemplate.value && name === originalTemplateName) {
    resetTemplateForm()
  }
}

function cancelEdit() {
  resetTemplateForm()
}

onMounted(() => {
  templateStore.loadTemplates()
})
</script> 
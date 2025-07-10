<template>
  <div class="tag-selector w-full">
    <!-- 标签区域 -->
    <div class="flex flex-wrap items-center gap-2">
      <span class="text-xs text-base-content/80">标签:</span>
      
      <!-- 已选择的标签 -->
      <div class="flex flex-wrap gap-1 items-center">
        <div 
          v-for="tag in modelValue" 
          :key="tag.id" 
          class="badge badge-primary cursor-pointer gap-1 badge-md"
        >
          {{ tag.name }}
          <button class="hover:bg-primary-focus rounded-full" @click="removeTag(tag.id)">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
        
        <!-- 添加标签输入框 -->
        <div class="tag-input-container relative">
          <input 
            type="text" 
            placeholder="添加标签..." 
            class="input input-sm input-bordered w-36 pr-7" 
            v-model="newTag"
            @input="filterAvailableTags"
            @focus="handleTagInputFocus"
            @keyup.enter="addTag"
            @keydown.escape="showTagDropdown = false"
            @keydown.down="navigateTagList('down')"
            @keydown.up="navigateTagList('up')"
            @keydown.tab.prevent="selectHighlightedTag"
          />
          <button 
            class="absolute right-1 top-1/2 -translate-y-1/2 btn btn-ghost btn-xs btn-circle"
            v-if="newTag.trim()"
            @click="addTag"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
          </button>
          <button 
            class="absolute right-1 top-1/2 -translate-y-1/2 btn btn-ghost btn-xs btn-circle"
            v-else
            @click="toggleTagDropdown"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
          
          <!-- 标签下拉列表 -->
          <div 
            v-if="showTagDropdown" 
            class="absolute left-0 bottom-full mb-1 z-10 bg-base-100 shadow-lg rounded-lg overflow-hidden w-72 border border-base-300"
            ref="tagDropdownRef"
          >
            <!-- 标签状态信息 -->
            <div class="p-2 border-b border-base-300 text-xs text-base-content/80 flex justify-between items-center">
              <div>
                <span v-if="newTag.trim() && filteredAvailableTags.length > 0">
                  找到 {{ filteredAvailableTags.length }} 个匹配标签
                </span>
                <span v-else-if="newTag.trim() && filteredAvailableTags.length === 0">
                  未找到匹配标签
                </span>
                <span v-else>
                  共 {{ availableTags.length }} 个标签
                </span>
              </div>
              <button 
                class="text-primary hover:underline"
                @click="refreshTagsList"
              >
                刷新
              </button>
            </div>
            
            <!-- 标签快速选择区域 -->
            <div class="p-2" v-if="!newTag.trim()">
              <div class="text-xs font-medium mb-1 flex justify-between items-center">
                <span>常用标签</span>
                <button 
                  class="text-xs text-primary hover:underline"
                  @click="showTagSection = showTagSection === 'frequent' ? 'all' : 'frequent'"
                >
                  {{ showTagSection === 'frequent' ? '显示全部' : '显示常用' }}
                </button>
              </div>
              
              <!-- 频繁使用的标签 -->
              <div v-if="showTagSection === 'frequent'" class="flex flex-wrap gap-1 max-h-24 overflow-y-auto">
                <button 
                  v-for="tag in frequentTags" 
                  :key="tag.id"
                  class="badge badge-outline hover:badge-primary cursor-pointer"
                  :class="{'bg-base-200': highlightedTagId === tag.id}"
                  @click="selectExistingTag(tag)"
                  :data-tag-id="tag.id"
                >
                  {{ tag.name }}
                </button>
                <div v-if="frequentTags.length === 0" class="text-xs text-base-content/80 py-1">
                  暂无常用标签
                </div>
              </div>
              
              <!-- 所有标签 -->
              <div v-else class="flex flex-wrap gap-1 max-h-48 overflow-y-auto pb-1">
                <button 
                  v-for="tag in filteredAvailableTags" 
                  :key="tag.id"
                  class="badge badge-outline hover:badge-primary cursor-pointer"
                  :class="{'bg-base-200': highlightedTagId === tag.id}"
                  @click="selectExistingTag(tag)"
                  :data-tag-id="tag.id"
                >
                  {{ tag.name }}
                </button>
                <div v-if="filteredAvailableTags.length === 0" class="text-xs text-base-content/80 py-1">
                  所有标签已添加
                </div>
              </div>
            </div>
            
            <!-- 搜索结果 -->
            <div v-else-if="filteredAvailableTags.length > 0" class="max-h-48 overflow-y-auto">
              <ul class="menu menu-compact">
                <li v-for="tag in filteredAvailableTags" :key="tag.id">
                  <button 
                    class="py-2"
                    :class="{'bg-base-200': highlightedTagId === tag.id}"
                    @click="selectExistingTag(tag)"
                    :data-tag-id="tag.id"
                  >
                    {{ tag.name }}
                  </button>
                </li>
              </ul>
            </div>
            
            <!-- 创建新标签选项 -->
            <div 
              v-if="newTag.trim() && !tagExists(newTag)" 
              class="p-2 border-t border-base-300"
            >
              <button 
                class="btn btn-primary btn-sm w-full"
                @click="addTag"
              >
                创建标签 "{{ newTag.trim() }}"
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 标签建议 -->
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTipsStore } from '../stores/tipsStore'

// 类型定义
interface Tag {
  id: string;
  name: string;
}

// 组件属性和事件
const props = defineProps<{
  modelValue: Tag[];
  contentText?: string;
  titleText?: string;
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: Tag[]): void;
  (e: 'saveNote'): void;
}>()

// Store
const tipsStore = useTipsStore()

// 标签相关状态
const newTag = ref('')
const showTagDropdown = ref(false)
const availableTags = ref<Tag[]>([])
const filteredAvailableTags = ref<Tag[]>([])
const frequentTags = ref<Tag[]>([])
const tagSuggestions = ref<Tag[]>([])
const highlightedTagId = ref<string | null>(null)
const showTagSection = ref<'frequent' | 'all'>('frequent')
const tagDropdownRef = ref<HTMLElement | null>(null)

// 监听外部note变化
onMounted(() => {
  // 获取所有标签
  fetchAvailableTags()
  
  // 设置标签的全局点击处理
  document.addEventListener('click', (e) => {
    const target = e.target as HTMLElement
    if (tagDropdownRef.value && !tagDropdownRef.value.contains(target) && 
        !target.closest('.tag-input-container')) {
      showTagDropdown.value = false
    }
  })
})

// 获取所有可用标签
async function fetchAvailableTags() {
  try {
    // 直接从后端获取所有标签
    availableTags.value = await invoke('get_all_tags') as Tag[]
    
    // 更新过滤的标签
    filterAvailableTags()
    // 加载常用标签
    loadFrequentTags()
    // 生成标签建议
    generateTagSuggestions()
  } catch (error) {
    console.error('获取可用标签失败:', error)
  }
}

// 刷新标签列表
async function refreshTagsList() {
  await fetchAvailableTags()
}

// 过滤可用标签
function filterAvailableTags() {
  const excludedIds = props.modelValue.map(tag => tag.id)
  
  if (!newTag.value.trim()) {
    // 当没有搜索词时，显示所有未添加的标签
    filteredAvailableTags.value = availableTags.value
      .filter(tag => !excludedIds.includes(tag.id))
  } else {
    const query = newTag.value.toLowerCase().trim()
    filteredAvailableTags.value = availableTags.value
      .filter(tag => 
        tag.name.toLowerCase().includes(query) && 
        !excludedIds.includes(tag.id)
      )
  }
  
  // 如果有高亮的标签ID，但它不在过滤后的列表中，重置高亮
  if (highlightedTagId.value && 
      !filteredAvailableTags.value.some(tag => tag.id === highlightedTagId.value)) {
    highlightedTagId.value = null
  }
  
  // 如果有过滤结果且没有高亮标签，设置第一个为高亮
  if (filteredAvailableTags.value.length > 0 && !highlightedTagId.value) {
    highlightedTagId.value = filteredAvailableTags.value[0].id
  }
}

// 加载常用标签
function loadFrequentTags() {
  try {
    // 从本地存储获取标签使用频率
    const tagUsageStr = localStorage.getItem('mytips-tag-usage') || '{}'
    const tagUsage = JSON.parse(tagUsageStr) as Record<string, number>
    
    // 获取常用标签（排除已添加的标签）
    const excludedIds = props.modelValue.map(tag => tag.id)
    
    // 按使用频率排序并限制数量
    frequentTags.value = availableTags.value
      .filter(tag => !excludedIds.includes(tag.id) && tagUsage[tag.id])
      .sort((a, b) => (tagUsage[b.id] || 0) - (tagUsage[a.id] || 0))
      .slice(0, 10)
  } catch (error) {
    console.error('加载常用标签失败:', error)
    frequentTags.value = []
  }
}

// 根据笔记内容生成标签建议
function generateTagSuggestions() {
  // 如果笔记内容为空，不生成建议
  if (!props.contentText?.trim()) {
    tagSuggestions.value = []
    return
  }
  
  // 获取笔记内容中的关键词
  const content = props.contentText.toLowerCase()
  const title = props.titleText?.toLowerCase() || ''
  
  // 排除已添加的标签
  const excludedIds = props.modelValue.map(tag => tag.id)
  
  // 找出内容中包含的标签名
  tagSuggestions.value = availableTags.value
    .filter(tag => {
      const tagName = tag.name.toLowerCase()
      return !excludedIds.includes(tag.id) &&
        (content.includes(tagName) || title.includes(tagName))
    })
    .slice(0, 5) // 限制建议数量
}

// 显示/隐藏标签下拉框
function toggleTagDropdown() {
  showTagDropdown.value = !showTagDropdown.value
  if (showTagDropdown.value) {
    filterAvailableTags()
    loadFrequentTags()
  } else {
    highlightedTagId.value = null
  }
}

// 处理标签输入框获取焦点
function handleTagInputFocus() {
  showTagDropdown.value = true
  filterAvailableTags()
  loadFrequentTags()
}

// 添加标签
async function addTag() {
  if (!newTag.value.trim()) return
  
  try {
    const tagName = newTag.value.trim()
    
    // 检查是否已存在相同的标签
    const existingTag = availableTags.value.find(tag => 
      tag.name.toLowerCase() === tagName.toLowerCase()
    )
    
    if (existingTag) {
      // 检查标签是否已添加到笔记
      if (!props.modelValue.some(t => t.id === existingTag.id)) {
        // 添加到笔记
        addTagToNote(existingTag)
      }
    } else {
      // 通过 Pinia store 创建新标签并自动同步全局状态
      const createdTag = await tipsStore.createTag(tagName)
      if (createdTag) {
        // 更新本地可用标签列表
        availableTags.value.push(createdTag)
        // 添加到笔记
        addTagToNote(createdTag)
      }
    }
    
    // 清空输入框
    newTag.value = ''
    // 刷新过滤的标签
    filterAvailableTags()
  } catch (e) {
    console.error('添加标签失败:', e)
  }
}

// 添加标签到笔记
function addTagToNote(tag: Tag) {
  // 检查标签是否已添加
  if (props.modelValue.some(t => t.id === tag.id)) {
    // 如果标签已存在，则不重复添加，直接关闭下拉框
    showTagDropdown.value = false;
    return;
  }
  
  console.log('添加标签到笔记:', tag)
  
  // 添加标签 - 通过事件通知父组件
  emit('update:modelValue', [...props.modelValue, tag])
  
  // 更新标签使用频率
  updateTagUsage(tag.id)
  
  // 重新生成标签建议
  generateTagSuggestions()
  
  // 过滤可用标签
  filterAvailableTags()
  
  // 加载常用标签
  loadFrequentTags()
  
  // 关闭下拉框
  showTagDropdown.value = false;
  
  // 通知父组件保存笔记
  emit('saveNote')
}

// 移除标签
function removeTag(tagId: string) {
  try {
    // 从笔记中移除标签 - 通过事件通知父组件
    emit('update:modelValue', props.modelValue.filter(tag => tag.id !== tagId))
    
    // 重新生成标签建议
    generateTagSuggestions()
    
    // 刷新过滤的标签
    filterAvailableTags()
    
    // 加载常用标签
    loadFrequentTags()
    
    // 通知父组件保存笔记
    emit('saveNote')
  } catch (e) {
    console.error('移除标签失败:', e)
  }
}

// 选择现有标签
function selectExistingTag(tag: Tag) {
  // 添加标签到笔记
  addTagToNote(tag)
  
  // 继续保持下拉框打开状态
  highlightedTagId.value = null
}

// 更新标签使用频率
function updateTagUsage(tagId: string) {
  try {
    // 从本地存储获取标签使用频率
    const tagUsageStr = localStorage.getItem('mytips-tag-usage') || '{}'
    const tagUsage = JSON.parse(tagUsageStr) as Record<string, number>
    
    // 更新使用次数
    tagUsage[tagId] = (tagUsage[tagId] || 0) + 1
    
    // 保存回本地存储
    localStorage.setItem('mytips-tag-usage', JSON.stringify(tagUsage))
  } catch (error) {
    console.error('更新标签使用频率失败:', error)
  }
}

// 检查标签是否已存在
function tagExists(tagName: string): boolean {
  const normalizedName = tagName.trim().toLowerCase()
  return availableTags.value.some(tag => 
    tag.name.toLowerCase() === normalizedName
  )
}

// 键盘导航标签列表
function navigateTagList(direction: 'up' | 'down') {
  if (!showTagDropdown.value || filteredAvailableTags.value.length === 0) return
  
  const currentIndex = highlightedTagId.value 
    ? filteredAvailableTags.value.findIndex(tag => tag.id === highlightedTagId.value)
    : -1
  
  let newIndex: number
  
  if (direction === 'down') {
    newIndex = currentIndex === -1 || currentIndex === filteredAvailableTags.value.length - 1
      ? 0 : currentIndex + 1
  } else {
    newIndex = currentIndex <= 0 
      ? filteredAvailableTags.value.length - 1 : currentIndex - 1
  }
  
  highlightedTagId.value = filteredAvailableTags.value[newIndex].id
  
  // 确保滚动到可见区域
  nextTick(() => {
    const highlightedElement = document.querySelector(`[data-tag-id="${highlightedTagId.value}"]`)
    highlightedElement?.scrollIntoView({ block: 'nearest' })
  })
}

// 选择当前高亮的标签
function selectHighlightedTag() {
  if (!showTagDropdown.value || !highlightedTagId.value) return
  
  const tag = filteredAvailableTags.value.find(tag => tag.id === highlightedTagId.value)
  if (tag) {
    selectExistingTag(tag)
  }
}

// 监听属性变化
watch(() => props.contentText, () => {
  generateTagSuggestions()
}, { immediate: true })

watch(() => props.titleText, () => {
  generateTagSuggestions()
}, { immediate: true })
</script>

<style scoped>
/* TagSelector特有的样式 */

/* 标签输入框的特殊样式 */
.tag-input {
  transition: all 0.2s ease;
}

.tag-input:focus {
  transform: scale(1.02);
}

/* 标签项的特殊动画 */
.tag-item {
  transition: all 0.2s ease;
  position: relative;
}

.tag-item:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 标签删除按钮的特殊样式 */
.tag-remove-btn {
  opacity: 0.7;
  transition: opacity 0.2s ease;
}

.tag-item:hover .tag-remove-btn {
  opacity: 1;
}

/* 标签建议下拉框的特殊样式 */
.tag-suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  z-index: 100;
  max-height: 200px;
  overflow-y: auto;
  backdrop-filter: blur(8px);
}

.tag-suggestion-item {
  padding: 8px 12px;
  cursor: pointer;
  transition: background 0.2s ease;
}

.tag-suggestion-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

/* 新标签创建的特殊样式 */
.new-tag-indicator {
  font-style: italic;
  opacity: 0.8;
}
</style> 
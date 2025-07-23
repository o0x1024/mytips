<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- 对话框背景 -->
    <div class="fixed inset-0 bg-black bg-opacity-50" @click="closeDialog" />
    
    <!-- 对话框内容 -->
    <div class="relative w-full max-w-4xl mx-4 max-h-[90vh] overflow-hidden bg-base-100 rounded-xl shadow-xl">
      <!-- 对话框头部 -->
      <div class="flex items-center justify-between p-6 border-b border-base-300">
        <h2 class="text-xl font-bold text-error flex items-center gap-3">
          <i class="fas fa-exclamation-triangle"></i>
          {{ $t('conflictResolutionDialog.title') }}
        </h2>
        <button class="btn btn-ghost btn-sm btn-circle" @click="closeDialog">
          <i class="fas fa-times text-lg"></i>
        </button>
      </div>
      
      <!-- 对话框主体 -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-8rem)]">
        <!-- 加载状态 -->
        <div v-if="loading" class="flex flex-col items-center justify-center py-12">
          <span class="loading loading-spinner loading-lg text-primary"></span>
          <p class="mt-4 text-base-content/70">{{ $t('conflictResolutionDialog.loading') }}</p>
        </div>
        
        <!-- 空状态 -->
        <div v-else-if="conflicts.length === 0" class="text-center py-12">
          <i class="fas fa-check-circle text-6xl text-success mb-4"></i>
          <h3 class="text-xl font-bold text-base-content mb-2">{{ $t('conflictResolutionDialog.noConflictsTitle') }}</h3>
          <p class="text-base-content/70">{{ $t('conflictResolutionDialog.noConflictsMessage') }}</p>
        </div>
        
        <!-- 冲突列表 -->
        <div v-else class="space-y-6">
          <!-- 冲突总览 -->
          <div class="alert alert-warning">
            <i class="fas fa-exclamation-triangle"></i>
            <div>
              <h3 class="font-bold">{{ $t('conflictResolutionDialog.conflictOverview', { count: conflicts.length }) }}</h3>
              <div class="text-sm">{{ $t('conflictResolutionDialog.conflictInstruction') }}</div>
            </div>
          </div>
          
          <!-- 冲突项目 -->
          <div v-for="conflict in conflicts" :key="conflict.id" class="card bg-base-200 shadow-sm border border-base-300">
            <div class="card-body p-6">
              <!-- 冲突标题 -->
              <div class="flex items-center gap-3 mb-4">
                <i :class="getTableIcon(conflict.table_name)" class="text-primary text-lg"></i>
                <span class="font-semibold text-base-content">{{ getTableDisplayName(conflict.table_name) }}</span>
                <div class="badge" :class="getOperationBadgeClass(conflict.operation)">
                  {{ getOperationDisplayName(conflict.operation) }}
                </div>
              </div>
              
              <!-- 冲突详情 -->
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                <div class="flex justify-between">
                  <span class="text-base-content/70 text-sm">{{ $t('conflictResolutionDialog.recordId') }}</span>
                  <span class="font-mono text-sm font-medium">{{ conflict.record_id }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-base-content/70 text-sm">{{ $t('conflictResolutionDialog.createdAt') }}</span>
                  <span class="text-sm">{{ formatDate(conflict.created_at) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-base-content/70 text-sm">{{ $t('conflictResolutionDialog.retryCount') }}</span>
                  <span class="text-sm font-medium">{{ conflict.retry_count }}</span>
                </div>
              </div>
              
              <!-- 冲突数据展开 -->
              <div v-if="conflict.conflict_data" class="mb-4">
                <button 
                  class="btn btn-ghost btn-sm gap-2"
                  @click="toggleConflictData(conflict.id)"
                >
                  <i class="fas" :class="expandedConflicts.has(conflict.id) ? 'fa-chevron-up' : 'fa-chevron-down'"></i>
                  {{ expandedConflicts.has(conflict.id) ? $t('conflictResolutionDialog.hideDetails') : $t('conflictResolutionDialog.showDetails') }} {{ $t('conflictResolutionDialog.conflictDetails') }}
                </button>
                
                <div v-if="expandedConflicts.has(conflict.id)" class="mt-3">
                  <div class="mockup-code text-xs">
                    <pre class="text-base-content"><code>{{ formatConflictData(conflict.conflict_data) }}</code></pre>
                  </div>
                </div>
              </div>
              
              <!-- 解决策略 -->
              <div class="divider text-sm">{{ $t('conflictResolutionDialog.resolutionStrategy') }}</div>
              
              <div v-if="resolvingConflicts.has(conflict.id)" class="flex items-center gap-2 justify-center py-4">
                <span class="loading loading-spinner loading-sm"></span>
                <span class="text-sm">{{ $t('conflictResolutionDialog.resolving') }}</span>
              </div>
              
              <div v-else class="flex flex-wrap gap-3">
                <button 
                  class="btn btn-outline btn-info gap-2 flex-1 min-w-[120px]"
                  @click="resolveConflict(conflict, 'LOCAL_WINS')"
                >
                  <i class="fas fa-laptop"></i>
                  {{ $t('conflictResolutionDialog.localWins') }}
                </button>
                
                <button 
                  class="btn btn-outline btn-warning gap-2 flex-1 min-w-[120px]"
                  @click="resolveConflict(conflict, 'REMOTE_WINS')"
                >
                  <i class="fas fa-cloud"></i>
                  {{ $t('conflictResolutionDialog.remoteWins') }}
                </button>
                
                <button 
                  class="btn btn-outline btn-success gap-2 flex-1 min-w-[120px]"
                  @click="resolveConflict(conflict, 'MERGE')"
                >
                  <i class="fas fa-code-merge"></i>
                  {{ $t('conflictResolutionDialog.tryMerge') }}
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 批量操作 -->
        <div v-if="conflicts.length > 0" class="mt-8">
          <div class="divider">{{ $t('conflictResolutionDialog.batchActions') }}</div>
          
          <div class="flex flex-wrap gap-3 justify-center">
            <button 
              class="btn btn-info gap-2"
              @click="resolveBatch('LOCAL_WINS')"
              :disabled="isBatchResolving"
            >
              <i class="fas fa-laptop"></i>
              {{ $t('conflictResolutionDialog.batchLocalWins') }}
            </button>
            
            <button 
              class="btn btn-warning gap-2"
              @click="resolveBatch('REMOTE_WINS')"
              :disabled="isBatchResolving"
            >
              <i class="fas fa-cloud"></i>
              {{ $t('conflictResolutionDialog.batchRemoteWins') }}
            </button>
          </div>
        </div>
      </div>
      
      <!-- 对话框底部 -->
      <div class="flex justify-between items-center p-6 border-t border-base-300 bg-base-50">
        <button class="btn btn-ghost gap-2" @click="refreshConflicts">
          <i class="fas fa-refresh"></i>
          {{ $t('conflictResolutionDialog.refreshList') }}
        </button>
        <button class="btn btn-primary" @click="closeDialog">{{ $t('conflictResolutionDialog.close') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'

const { t } = useI18n()

// Emits
const emit = defineEmits(['close'])

// 响应式数据
const conflicts = ref([])
const loading = ref(false)
const expandedConflicts = reactive(new Set())
const resolvingConflicts = reactive(new Set())
const isBatchResolving = ref(false)

// 方法
const loadConflicts = async () => {
  loading.value = true
  try {
    const conflictsList = await invoke('get_sync_conflicts')
    conflicts.value = conflictsList
  } catch (error) {
    console.error('Failed to load conflicts:', error)
  } finally {
    loading.value = false
  }
}

const resolveConflict = async (conflict, strategy) => {
  resolvingConflicts.add(conflict.id)
  
  try {
    await invoke('resolve_conflict', {
      request: {
        record_id: conflict.record_id,
        table_name: conflict.table_name,
        strategy: strategy
      }
    })
    
    // 从列表中移除已解决的冲突
    const index = conflicts.value.findIndex(c => c.id === conflict.id)
    if (index !== -1) {
      conflicts.value.splice(index, 1)
    }
    
    console.log(`Conflict resolved with strategy: ${strategy}`)
  } catch (error) {
    console.error('Failed to resolve conflict:', error)
  } finally {
    resolvingConflicts.delete(conflict.id)
  }
}

const resolveBatch = async (strategy) => {
  isBatchResolving.value = true
  
  try {
    const pendingConflicts = [...conflicts.value]
    
    for (const conflict of pendingConflicts) {
      await resolveConflict(conflict, strategy)
    }
    
    console.log(`Batch resolution completed with strategy: ${strategy}`)
  } catch (error) {
    console.error('Batch resolution failed:', error)
  } finally {
    isBatchResolving.value = false
  }
}

const toggleConflictData = (conflictId) => {
  if (expandedConflicts.has(conflictId)) {
    expandedConflicts.delete(conflictId)
  } else {
    expandedConflicts.add(conflictId)
  }
}

const refreshConflicts = async () => {
  await loadConflicts()
}

const closeDialog = () => {
  emit('close')
}

// 辅助方法
const getTableIcon = (tableName) => {
  const icons = {
    'tips': 'fas fa-sticky-note',
    'categories': 'fas fa-folder',
    'tags': 'fas fa-tags',
    'ai_conversations': 'fas fa-comments',
    'ai_messages': 'fas fa-comment'
  }
  return icons[tableName] || 'fas fa-database'
}

const getTableDisplayName = (tableName) => {
  return t(`conflictResolutionDialog.tables.${tableName}`, tableName)
}

const getOperationBadgeClass = (operation) => {
  const classes = {
    'INSERT': 'badge-success',
    'UPDATE': 'badge-warning',
    'DELETE': 'badge-error'
  }
  return classes[operation] || 'badge-neutral'
}

const getOperationDisplayName = (operation) => {
  return t(`conflictResolutionDialog.operations.${operation}`, operation)
}

const formatDate = (timestamp) => {
  return new Date(timestamp).toLocaleString()
}

const formatConflictData = (conflictData) => {
  if (!conflictData) return t('conflictResolutionDialog.noConflictData')
  
  try {
    const data = JSON.parse(conflictData)
    return JSON.stringify(data, null, 2)
  } catch (error) {
    return conflictData
  }
}

// 生命周期
onMounted(async () => {
  await loadConflicts()
})
</script>

<style scoped>
.conflict-resolution-dialog {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1001;
  display: flex;
  align-items: center;
  justify-content: center;
}

.dialog-backdrop {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
}

.dialog-content {
  position: relative;
  background: var(--bg-primary);
  border-radius: 12px;
  width: 90%;
  max-width: 900px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1);
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--error-color);
  display: flex;
  align-items: center;
  gap: 8px;
}

.close-button {
  background: none;
  border: none;
  font-size: 18px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.dialog-body {
  padding: 20px 24px;
}

.dialog-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
}

/* 加载和空状态 */
.loading-state,
.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-secondary);
}

.loading-state i,
.empty-state i {
  font-size: 48px;
  margin-bottom: 16px;
  display: block;
}

.empty-state i {
  color: var(--success-color);
}

.empty-state h3 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0;
}

/* 冲突列表 */
.conflicts-header {
  margin-bottom: 24px;
}

.conflicts-header h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.conflicts-header p {
  margin: 0;
  color: var(--text-secondary);
}

.conflict-item {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 16px;
  border: 1px solid var(--border-color);
}

.conflict-info {
  margin-bottom: 20px;
}

.conflict-title {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.table-name {
  font-weight: 500;
  color: var(--text-primary);
}

.operation-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.operation-insert {
  background: var(--success-bg);
  color: var(--success-color);
}

.operation-update {
  background: var(--warning-bg);
  color: var(--warning-color);
}

.operation-delete {
  background: var(--error-bg);
  color: var(--error-color);
}

.conflict-details {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
  margin-bottom: 12px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.detail-item .label {
  color: var(--text-secondary);
  font-size: 13px;
}

.detail-item .value {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 13px;
}

.conflict-data-content {
  margin-top: 12px;
  background: var(--bg-primary);
  border-radius: 6px;
  padding: 12px;
  border: 1px solid var(--border-color);
}

.conflict-data-content pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  line-height: 1.4;
  color: var(--text-primary);
  white-space: pre-wrap;
  overflow-x: auto;
}

/* 冲突操作 */
.conflict-actions h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.resolution-options {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.resolution-btn {
  flex: 1;
  min-width: 120px;
}

.resolving-indicator {
  margin-top: 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--primary-color);
  font-size: 14px;
}

/* 批量操作 */
.batch-actions {
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid var(--border-color);
}

.batch-actions h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.batch-buttons {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

/* 按钮样式 */
.btn {
  padding: 10px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  text-decoration: none;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
}

.btn-outline {
  background: transparent;
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-outline:hover:not(:disabled) {
  background: var(--bg-secondary);
  border-color: var(--primary-color);
}

.btn-link {
  background: none;
  color: var(--primary-color);
  border: none;
  padding: 4px 0;
  font-size: 13px;
}

.btn-link:hover:not(:disabled) {
  text-decoration: underline;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .dialog-content {
    width: 95%;
    margin: 10px;
  }
  
  .conflict-details {
    grid-template-columns: 1fr;
  }
  
  .resolution-options {
    flex-direction: column;
  }
  
  .resolution-btn {
    min-width: auto;
  }
  
  .batch-buttons {
    flex-direction: column;
  }
}
</style> 
<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- 对话框背景 -->
    <div class="fixed inset-0 bg-black bg-opacity-50" @click="closeDialog" />
    
    <!-- 对话框内容 -->
    <div class="relative w-full max-w-4xl mx-4 max-h-[90vh] overflow-hidden bg-base-100 rounded-xl shadow-xl">
      <!-- 对话框头部 -->
      <div class="flex items-center justify-between p-6 border-b border-base-300">
        <h2 class="text-xl font-bold text-base-content flex items-center gap-3">
          <i class="fas fa-history text-primary"></i>
          同步历史记录
        </h2>
        <button class="btn btn-ghost btn-sm btn-circle" @click="closeDialog">
          <i class="fas fa-times text-lg"></i>
        </button>
      </div>
            
      <!-- 对话框主体 -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-8rem)]">
        <!-- 过滤器 -->
        <div class="card bg-base-200 mb-6">
          <div class="card-body p-4">
            <div class="flex flex-col sm:flex-row justify-between items-center gap-4">
              <div class="form-control">
                <label class="label">
                  <span class="label-text font-medium">显示记录数</span>
                </label>
                <select class="select select-bordered select-sm" v-model="historyLimit" @change="loadHistory">
                  <option value="20">最近20条</option>
                  <option value="50">最近50条</option>
                  <option value="100">最近100条</option>
                </select>
              </div>
              
              <button class="btn btn-outline btn-sm gap-2" @click="loadHistory">
                <i class="fas fa-refresh"></i>
                刷新
              </button>
            </div>
          </div>
        </div>
        
        <!-- 加载状态 -->
        <div v-if="loading" class="flex flex-col items-center justify-center py-12">
          <span class="loading loading-spinner loading-lg text-primary"></span>
          <p class="mt-4 text-base-content/70">正在加载历史记录...</p>
        </div>
        
        <!-- 空状态 -->
        <div v-else-if="history.length === 0" class="text-center py-12">
          <i class="fas fa-clock text-6xl text-base-content/30 mb-4"></i>
          <h3 class="text-xl font-bold text-base-content mb-2">暂无历史记录</h3>
          <p class="text-base-content/70">还没有任何同步操作或冲突解决记录</p>
        </div>
                
        <!-- 历史记录列表 -->
        <div v-else class="space-y-4">
          <!-- 历史记录项 -->
          <div v-for="item in history" :key="item.id" class="card bg-base-200 hover:bg-base-300 transition-colors duration-200 border border-base-300">
            <div class="card-body p-4">
              <!-- 历史记录头部 -->
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-3">
                <div class="flex items-center gap-3">
                  <i :class="getTableIcon(item.table_name)" class="text-primary text-lg"></i>
                  <span class="font-semibold text-base-content">{{ getTableDisplayName(item.table_name) }}</span>
                  <div class="badge" :class="getStrategyBadgeClass(item.resolution_strategy)">
                    {{ getStrategyDisplayName(item.resolution_strategy) }}
                  </div>
                </div>
                
                <div class="text-sm text-base-content/70">
                  <i class="fas fa-clock mr-1"></i>
                  {{ formatDate(item.resolved_at) }}
                </div>
              </div>
              
              <!-- 详细信息 -->
              <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div class="flex justify-between items-center">
                  <span class="text-base-content/70 text-sm">记录ID:</span>
                  <span class="font-mono text-sm font-medium bg-base-300 px-2 py-1 rounded">{{ item.record_id }}</span>
                </div>
                
                <div class="flex justify-between items-center">
                  <span class="text-base-content/70 text-sm">解决方式:</span>
                  <span class="text-sm font-medium">{{ getResolvedByDisplayName(item.resolved_by) }}</span>
                </div>
              </div>
            </div>
                    </div>
          
          <!-- 统计信息 -->
          <div class="mt-8">
            <div class="divider">
              <span class="text-base-content font-medium">统计信息</span>
            </div>
            
            <div class="stats stats-vertical lg:stats-horizontal shadow bg-base-200">
              <div class="stat">
                <div class="stat-figure text-primary">
                  <i class="fas fa-list text-2xl"></i>
                </div>
                <div class="stat-title">总记录数</div>
                <div class="stat-value text-primary">{{ history.length }}</div>
              </div>
              
              <div class="stat">
                <div class="stat-figure text-info">
                  <i class="fas fa-laptop text-2xl"></i>
                </div>
                <div class="stat-title">本地优先</div>
                <div class="stat-value text-info">{{ getStrategyCount('LOCAL_WINS') }}</div>
              </div>
              
              <div class="stat">
                <div class="stat-figure text-warning">
                  <i class="fas fa-cloud text-2xl"></i>
                </div>
                <div class="stat-title">远程优先</div>
                <div class="stat-value text-warning">{{ getStrategyCount('REMOTE_WINS') }}</div>
              </div>
              
              <div class="stat">
                <div class="stat-figure text-success">
                  <i class="fas fa-code-merge text-2xl"></i>
                </div>
                <div class="stat-title">自动合并</div>
                <div class="stat-value text-success">{{ getStrategyCount('MERGE') }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 对话框底部 -->
      <div class="flex justify-end items-center p-6 border-t border-base-300 bg-base-50">
        <button class="btn btn-primary" @click="closeDialog">关闭</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Emits
const emit = defineEmits(['close'])

// 响应式数据
const history = ref([])
const loading = ref(false)
const historyLimit = ref(50)

// 方法
const loadHistory = async () => {
  loading.value = true
  try {
    const historyList = await invoke('get_sync_history', {
      limit: historyLimit.value
    })
    history.value = historyList
  } catch (error) {
    console.error('Failed to load sync history:', error)
  } finally {
    loading.value = false
  }
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
  const names = {
    'tips': '笔记',
    'categories': '分类',
    'tags': '标签',
    'ai_conversations': 'AI对话',
    'ai_messages': 'AI消息'
  }
  return names[tableName] || tableName
}

const getStrategyBadgeClass = (strategy) => {
  const classes = {
    'LOCAL_WINS': 'badge-info',
    'REMOTE_WINS': 'badge-warning',
    'MERGE': 'badge-success',
    'USER_CHOICE': 'badge-accent'
  }
  return classes[strategy] || 'badge-neutral'
}

const getStrategyDisplayName = (strategy) => {
  const names = {
    'LOCAL_WINS': '本地优先',
    'REMOTE_WINS': '远程优先',
    'MERGE': '自动合并',
    'USER_CHOICE': '用户选择'
  }
  return names[strategy] || strategy
}

const getResolvedByDisplayName = (resolvedBy) => {
  const names = {
    'AUTO': '自动解决',
    'USER': '用户手动',
    'SYSTEM': '系统处理'
  }
  return names[resolvedBy] || resolvedBy
}

const formatDate = (timestamp) => {
  return new Date(timestamp).toLocaleString('zh-CN')
}

const getStrategyCount = (strategy) => {
  return history.value.filter(item => item.resolution_strategy === strategy).length
}

// 生命周期
onMounted(async () => {
  await loadHistory()
})
</script>

<style scoped>
.sync-history-dialog {
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
  max-width: 800px;
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
  color: var(--text-primary);
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
  justify-content: flex-end;
}

/* 过滤器 */
.history-filters {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.filter-group label {
  font-weight: 500;
  color: var(--text-primary);
}

.filter-group select {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 14px;
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

.empty-state h3 {
  margin: 0 0 8px 0;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0;
}

/* 历史记录列表 */
.history-list {
  margin-bottom: 32px;
}

.history-item {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 12px;
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.history-item:hover {
  border-color: var(--primary-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.history-title {
  display: flex;
  align-items: center;
  gap: 12px;
}

.table-name {
  font-weight: 500;
  color: var(--text-primary);
}

.strategy-badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.strategy-local {
  background: var(--info-bg);
  color: var(--info-color);
}

.strategy-remote {
  background: var(--primary-bg);
  color: var(--primary-color);
}

.strategy-merge {
  background: var(--success-bg);
  color: var(--success-color);
}

.strategy-user {
  background: var(--warning-bg);
  color: var(--warning-color);
}

.history-time {
  font-size: 13px;
  color: var(--text-secondary);
}

.history-details {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.detail-row .label {
  color: var(--text-secondary);
  font-size: 13px;
}

.detail-row .value {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 13px;
}

/* 统计信息 */
.history-stats {
  border-top: 1px solid var(--border-color);
  padding-top: 24px;
}

.history-stats h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.stat-item {
  text-align: center;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.stat-value {
  display: block;
  font-size: 24px;
  font-weight: 700;
  color: var(--primary-color);
  margin-bottom: 4px;
}

.stat-label {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
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

/* 响应式设计 */
@media (max-width: 640px) {
  .dialog-content {
    width: 95%;
    margin: 10px;
  }
  
  .history-filters {
    flex-direction: column;
    gap: 16px;
    align-items: stretch;
  }
  
  .filter-group {
    justify-content: space-between;
  }
  
  .history-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .history-details {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style> 
<template>
  <div class="space-y-6">
    <!-- 初始化加载状态 -->
    <div v-if="isInitializing" class="flex justify-center items-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
      <span class="ml-3 text-base-content/70">正在加载设置...</span>
    </div>
    
    <template v-else>

              <!-- 数据库配置卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
            </svg>
            数据库配置
          </h2>

          <!-- 数据库类型选择 -->
          <div class="form-control mb-6">
            <label class="label">
              <span class="label-text font-medium">数据库类型</span>
            </label>
            <div class="flex gap-4">
              <label class="label cursor-pointer gap-3">
                <input 
                  type="radio" 
                  name="database-type" 
                  class="radio radio-primary" 
                  value="local"
                  v-model="databaseType"
                />
                <span class="label-text">本地数据库（默认）</span>
              </label>
              <label class="label cursor-pointer gap-3">
                <input 
                  type="radio" 
                  name="database-type" 
                  class="radio radio-primary" 
                  value="remote"
                  v-model="databaseType"
                />
                <span class="label-text">远程数据库</span>
              </label>
            </div>
          </div>

          <!-- 本地数据库信息 -->
          <div v-if="databaseType === 'local'" class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">当前数据库路径</span>
              </label>
              <div class="join">
                <input 
                  type="text" 
                  :value="currentDatabasePath" 
                  readonly 
                  class="input input-bordered join-item flex-1 bg-base-200"
                  :title="currentDatabasePath"
                />
                <button 
                  class="btn btn-outline join-item" 
                  @click="copyDatabasePath"
                  title="复制路径"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </button>
              </div>
            </div>

            <div class="flex gap-2 flex-wrap">
              <button 
                class="btn btn-sm btn-outline" 
                @click="selectDatabaseFile"
                :disabled="isOperationInProgress"
              >
                选择数据库文件
              </button>
              <button 
                class="btn btn-sm btn-outline btn-secondary" 
                @click="createNewDatabase"
                :disabled="isOperationInProgress"
              >
                创建新数据库
              </button>
              <button 
                class="btn btn-sm btn-outline btn-info" 
                @click="resetToDefaultDatabase"
                :disabled="isOperationInProgress"
              >
                重置为默认位置
              </button>
            </div>

            <!-- 数据库信息显示 -->
            <div v-if="databaseInfo" class="stats shadow bg-base-200">
              <div class="stat">
                <div class="stat-title">文件大小</div>
                <div class="stat-value text-sm">{{ databaseInfo.size }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">笔记数量</div>
                <div class="stat-value text-sm">{{ databaseInfo.noteCount }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">分类数量</div>
                <div class="stat-value text-sm">{{ databaseInfo.categoryCount }}</div>
              </div>
            </div>
          </div>

          <!-- 远程数据库配置 -->
          <div v-else class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">远程数据库URL</span>
              </label>
              <input 
                type="text" 
                v-model="syncConfig.remote_url" 
                placeholder="libsql://your-database.turso.io"
                class="input input-bordered"
                :disabled="isOperationInProgress"
                v-if="syncConfig"
              />
              <label class="label">
                <span class="label-text-alt">远程Turso数据库的连接URL</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">认证令牌（可选）</span>
              </label>
              <div class="join">
                <input 
                  :type="showSyncToken ? 'text' : 'password'"
                  v-model="syncConfig.auth_token" 
                  placeholder="输入认证令牌（可留空）"
                  class="input input-bordered join-item flex-1"
                  :disabled="isOperationInProgress"
                  v-if="syncConfig"
                />
                <button 
                  type="button" 
                  class="btn btn-outline join-item"
                  @click="showSyncToken = !showSyncToken"
                >
                  <svg v-if="showSyncToken" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464M9.878 9.878a3 3 0 104.243 4.243M4.929 19.071L19.071 4.929" />
                  </svg>
                  <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </button>
              </div>
              <label class="label">
                <span class="label-text-alt">认证令牌可选，部分数据库配置可能不需要</span>
              </label>
            </div>

            <div class="flex gap-2">
              <button 
                class="btn btn-primary btn-sm" 
                @click="testSyncConnection"
                :disabled="!canTestSync || isTestingSync"
              >
                <span v-if="isTestingSync" class="loading loading-spinner loading-sm mr-2"></span>
                {{ isTestingSync ? '测试中...' : '测试连接' }}
              </button>
              <button 
                class="btn btn-success btn-sm" 
                @click="setupRemoteDatabase"
                :disabled="!canTestSync || isOperationInProgress"
              >
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ isOperationInProgress ? '设置中...' : '设置远程数据库' }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 数据同步模式卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            数据同步模式
          </h2>

          <!-- 同步状态显示 -->
          <div v-if="databaseType === 'remote'" class="alert mb-6" :class="{
            'alert-success': syncStatus.is_enabled && syncStatus.is_online,
            'alert-error': syncStatus.is_enabled && !syncStatus.is_online,
            'alert-info': !syncStatus.is_enabled
          }">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div>
              <h3 class="font-bold">
                {{ syncStatus.is_enabled ? (syncStatus.is_online ? '远程数据库已连接' : '远程数据库连接失败') : '远程数据库未配置' }}
              </h3>
              <div v-if="syncStatus.stats" class="text-sm mt-1">
                总记录数: {{ syncStatus.stats.total_records }} | 已同步: {{ syncStatus.stats.synced_records }} | 上次同步: {{ formatSyncTime(syncStatus.last_sync_time) }}
              </div>
            </div>
          </div>

          <!-- 同步模式卡片 -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <!-- 纯离线模式 -->
            <div class="card bg-base-200 border-2 transition-all cursor-pointer hover:shadow-md" 
                 :class="{ 'border-primary bg-primary/10': syncConfig?.sync_mode === 'OFFLINE' }"
                 @click="selectSyncMode('OFFLINE')">
              <div class="card-body p-4">
                <div class="flex items-center gap-3 mb-2">
                  <div class="w-3 h-3 rounded-full bg-gray-500"></div>
                  <h3 class="card-title text-sm">纯离线模式</h3>
                </div>
                <p class="text-xs text-base-content/70 mb-3">
                  完全使用本地数据库，不进行任何网络同步操作，适合注重隐私或网络受限的环境。
                </p>
                <div class="text-xs text-base-content/60">
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    数据完全本地化
                  </div>
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    无网络依赖
                  </div>
                  <div class="flex items-center gap-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    隐私性最高
                  </div>
                </div>
              </div>
            </div>

            <!-- 手动同步模式 -->
            <div class="card bg-base-200 border-2 transition-all cursor-pointer hover:shadow-md"
                 :class="{ 'border-warning bg-warning/10': syncConfig?.sync_mode === 'MANUAL' }"
                 @click="selectSyncMode('MANUAL')">
              <div class="card-body p-4">
                <div class="flex items-center gap-3 mb-2">
                  <div class="w-3 h-3 rounded-full bg-warning"></div>
                  <h3 class="card-title text-sm">手动同步模式</h3>
                </div>
                <p class="text-xs text-base-content/70 mb-3">
                  本地保存远程数据副本，用户主动触发同步操作，适合对同步时机有特殊要求的场景。
                </p>
                <div class="text-xs text-base-content/60">
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    手动控制同步时机
                  </div>
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    本地副本可离线使用
                  </div>
                  <div class="flex items-center gap-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    节省网络流量
                  </div>
                </div>
              </div>
            </div>

            <!-- 自动同步模式 -->
            <div class="card bg-base-200 border-2 transition-all cursor-pointer hover:shadow-md"
                 :class="{ 'border-success bg-success/10': syncConfig?.sync_mode === 'AUTO' }"
                 @click="selectSyncMode('AUTO')">
              <div class="card-body p-4">
                <div class="flex items-center gap-3 mb-2">
                  <div class="w-3 h-3 rounded-full bg-success"></div>
                  <h3 class="card-title text-sm">自动同步模式</h3>
                </div>
                <p class="text-xs text-base-content/70 mb-3">
                  定期自动同步本地和远程数据，确保数据实时性，适合多设备协作或实时备份需求。
                </p>
                <div class="text-xs text-base-content/60">
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    数据实时同步
                  </div>
                  <div class="flex items-center gap-1 mb-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    多设备数据一致
                  </div>
                  <div class="flex items-center gap-1">
                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>
                    自动备份保护
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 自动同步配置 -->
          <div v-if="syncConfig?.sync_mode === 'AUTO'" class="mt-6 p-4 bg-base-200 rounded-lg">
            <h4 class="font-medium mb-3">自动同步配置</h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label">
                  <span class="label-text">同步间隔 (秒)</span>
                </label>
                <input 
                  type="number" 
                  v-model.number="syncConfig.sync_interval"
                  v-if="syncConfig" 
                  min="30"
                  max="3600"
                  class="input input-bordered input-sm"
                  :disabled="isOperationInProgress"
                />
                <label class="label">
                  <span class="label-text-alt">建议300秒以上</span>
                </label>
              </div>
              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">启用自动同步</span>
                  <input 
                    type="checkbox" 
                    class="toggle toggle-success toggle-sm" 
                    v-model="syncConfig.auto_sync_enabled"
                    v-if="syncConfig"
                    :disabled="isOperationInProgress"
                  />
                </label>
              </div>
            </div>
          </div>

          <!-- 应用同步设置按钮 -->
          <div v-if="databaseType === 'remote'" class="mt-6">
            <button 
              class="btn btn-primary" 
              @click="applySyncSettings"
              :disabled="!canApplySyncSettings || isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              {{ isOperationInProgress ? '应用中...' : '应用同步设置' }}
            </button>
          </div>
        </div>
      </div>



      <!-- 数据操作卡片 -->
      <div v-if="databaseType === 'remote' && syncStatus.is_enabled" class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            同步操作
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
            <button 
              class="btn btn-info" 
              @click="manualSync"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              {{ isOperationInProgress ? '同步中...' : '立即同步' }}
            </button>
            
            <button 
              class="btn btn-secondary" 
              @click="showConflictDialog"
              :disabled="isOperationInProgress"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
              处理冲突
            </button>
            
            <button 
              class="btn btn-accent" 
              @click="showSyncHistoryDialog"
              :disabled="isOperationInProgress"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              同步历史
            </button>
          </div>

          <!-- 高级操作 -->
          <div class="collapse collapse-arrow bg-base-200 mt-4">
            <input type="checkbox" />
            <div class="collapse-title text-sm font-medium">
              高级同步操作
            </div>
            <div class="collapse-content">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                <button 
                  class="btn btn-error btn-outline btn-sm" 
                  @click="resetSyncStatus"
                  :disabled="isOperationInProgress"
                >
                  重置同步状态
                </button>
                <button 
                  class="btn btn-warning btn-outline btn-sm" 
                  @click="reinitializeRemoteDatabase"
                  :disabled="isOperationInProgress"
                >
                  重新初始化远程库
                </button>
                <button 
                  class="btn btn-warning btn-sm" 
                  @click="disableSync"
                  :disabled="isOperationInProgress"
                >
                  禁用远程同步
                </button>
              </div>
              <div class="text-xs text-base-content/70 mt-2">
                注意：这些操作可能导致数据丢失，请谨慎使用
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 数据备份与恢复卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
            </svg>
            数据备份与恢复
          </h2>

          <!-- 同步提醒 -->
          <div v-if="syncStatus.is_enabled" class="alert alert-warning mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <div class="text-sm">
              <div class="font-medium">同步已启用提醒</div>
              <div>进行备份/恢复操作时，建议先禁用同步以避免数据冲突</div>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
            <button 
              class="btn btn-outline" 
              @click="backupDatabase"
              :disabled="isOperationInProgress"
              :class="{ 'btn-warning': syncStatus.is_enabled }"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
              </svg>
              {{ isOperationInProgress ? '备份中...' : '本地备份' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="restoreDatabase"
              :disabled="isOperationInProgress"
              :class="{ 'btn-warning': syncStatus.is_enabled }"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 17l4 4 4-4m-4-5v9m-8-6h16" />
              </svg>
              {{ isOperationInProgress ? '恢复中...' : '本地恢复' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="exportAsMarkdown"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
              </svg>
              {{ isOperationInProgress ? '导出中...' : '导出Markdown' }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="migrateConfigToDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
              </svg>
              {{ isOperationInProgress ? '迁移中...' : '迁移配置' }}
            </button>
          </div>
        </div>
      </div>
    </template>

    <!-- 冲突解决对话框 -->
    <ConflictResolutionDialog v-if="showConflictResolutionDialog" @close="showConflictResolutionDialog = false" />
    
    <!-- 同步历史对话框 -->
    <SyncHistoryDialog v-if="showSyncHistoryDialogVisible" @close="showSyncHistoryDialogVisible = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { showMessage, showConfirm } from '../../services/dialog'
import ConflictResolutionDialog from '../dialogs/ConflictResolutionDialog.vue'
import SyncHistoryDialog from '../dialogs/SyncHistoryDialog.vue'

// 数据库类型选择
const databaseType = ref('local')

// 统一的操作状态管理
const isOperationInProgress = ref(false)
const isInitializing = ref(true)

// 数据库路径管理相关变量
const currentDatabasePath = ref('')
const databaseInfo = ref<{
  size: string
  noteCount: number
  categoryCount: number
  lastModified: string
} | null>(null)

// 同步相关状态
const syncStatus = ref({
  is_enabled: false,
  is_online: false,
  sync_mode: 'OFFLINE',
  last_sync_time: 0,
  stats: null as any
})

const syncConfig = ref({
  remote_url: 'http://127.0.0.1:8888',
  auth_token: '',
  sync_mode: 'OFFLINE',
  sync_interval: 300,
  auto_sync_enabled: true
})

const showSyncToken = ref(false)
const isTestingSync = ref(false)
const showConflictResolutionDialog = ref(false)
const showSyncHistoryDialogVisible = ref(false)

// 计算属性
const canTestSync = computed(() => {
  return syncConfig.value?.remote_url?.trim() !== ''
})

const canApplySyncSettings = computed(() => {
  return databaseType.value === 'remote' && 
         syncConfig.value?.remote_url?.trim() !== '' && 
         syncConfig.value?.sync_mode !== 'OFFLINE'
})

// 监听数据库类型变化
watch(databaseType, (newType) => {
  if (!syncConfig.value) return
  
  if (newType === 'local') {
    syncConfig.value.sync_mode = 'OFFLINE'
  } else if (newType === 'remote' && syncConfig.value.sync_mode === 'OFFLINE') {
    syncConfig.value.sync_mode = 'MANUAL'
  }
})

// 格式化同步时间
function formatSyncTime(timestamp: number): string {
  if (!timestamp) return '从未同步'
  return new Date(timestamp).toLocaleString('zh-CN')
}

// 选择同步模式
function selectSyncMode(mode: string) {
  if (!syncConfig.value) return
  
  if (databaseType.value === 'local' && mode !== 'OFFLINE') {
    showMessage('本地数据库只支持离线模式', { title: '提示' })
    return
  }
  syncConfig.value.sync_mode = mode
}

// 数据库路径管理相关方法
async function loadCurrentDatabasePath(): Promise<void> {
  try {
    const path = await invoke('get_current_database_path') as string
    currentDatabasePath.value = path
    await loadDatabaseInfo()
  } catch (error) {
    console.error('Failed to get current database path:', error)
  }
}

async function loadDatabaseInfo(): Promise<void> {
  try {
    const info = await invoke('get_database_info') as {
      size: string
      note_count: number
      category_count: number
      last_modified: string
    }
    
    databaseInfo.value = {
      size: info.size,
      noteCount: info.note_count,
      categoryCount: info.category_count,
      lastModified: new Date(info.last_modified).toLocaleString('zh-CN')
    }
  } catch (error) {
    console.error('Failed to get database info:', error)
    databaseInfo.value = null
  }
}

async function copyDatabasePath(): Promise<void> {
  try {
    await navigator.clipboard.writeText(currentDatabasePath.value)
    showMessage('数据库路径已复制到剪贴板', { title: '成功' })
  } catch (error) {
    console.error('Failed to copy path:', error)
    showMessage('复制路径失败', { title: '错误' })
  }
}

// 统一的数据库操作处理
async function handleDatabaseOperation(operation: () => Promise<any>, confirmMessage?: string): Promise<void> {
  if (isOperationInProgress.value) return
  
  // 如果启用了同步，先提醒用户
  if (syncStatus.value.is_enabled && confirmMessage) {
    const confirmed = await showConfirm(
      confirmMessage + '\n\n注意：此操作将自动禁用同步功能，操作后需要重新配置同步设置。',
      { title: '确认操作', confirmText: '继续', cancelText: '取消' }
    )
    if (!confirmed) return
  }
  
  isOperationInProgress.value = true
  try {
    const result = await operation()
    
    // 如果操作涉及数据库文件更改，重新加载相关信息
    if (result?.path) {
      currentDatabasePath.value = result.path
      await loadDatabaseInfo()
      // 自动禁用同步
      if (syncStatus.value.is_enabled) {
        try {
          // 临时跳过禁用同步操作，等待完整实现
          await loadSyncStatus()
          showMessage('数据库已更改，同步功能已自动禁用', { title: '提醒' })
        } catch (error) {
          console.error('Failed to disable sync:', error)
        }
      }
    }
    
    if (result?.restart_required) {
      showMessage('操作完成，请重启应用以使更改生效', { title: '需要重启' })
    } else if (typeof result === 'string') {
      showMessage(result, { title: '操作成功' })
    } else {
      showMessage('操作完成', { title: '成功' })
    }
  } catch (error) {
    console.error('Database operation failed:', error)
    showMessage('操作失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function selectDatabaseFile(): Promise<void> {
  await handleDatabaseOperation(
    () => invoke('select_database_file'),
    '确定要选择新的数据库文件吗？'
  )
}

async function createNewDatabase(): Promise<void> {
  await handleDatabaseOperation(
    () => invoke('create_new_database'),
    '确定要创建新的数据库吗？'
  )
}

async function resetToDefaultDatabase(): Promise<void> {
  await handleDatabaseOperation(
    () => invoke('reset_to_default_database'),
    '确定要重置到默认数据库位置吗？'
  )
}

// 远程数据库设置
async function setupRemoteDatabase() {
  if (!canTestSync.value || !syncConfig.value) return
  
  isOperationInProgress.value = true
  try {
    // 先测试连接
    const testResult = await invoke('test_remote_connection', {
      remoteUrl: syncConfig.value.remote_url,
      authToken: syncConfig.value.auth_token
    })
    
    if (!testResult) {
      showMessage('远程数据库连接失败，请检查URL和令牌！', { title: '连接失败' })
      return
    }
    
    // 创建远程数据库表结构并同步本地数据
    await invoke('configure_remote_database', {
      config: {
        remote_url: syncConfig.value.remote_url,
        auth_token: syncConfig.value.auth_token,
        sync_mode: 'MANUAL', // 设置时默认为手动模式
        sync_interval: 300,
        auto_sync_enabled: false
      }
    })
    
    showMessage('远程数据库设置成功！本地数据已同步到远程。', { title: '设置成功' })
    databaseType.value = 'remote'
    syncConfig.value.sync_mode = 'MANUAL'
    await loadSyncStatus()
  } catch (error) {
    console.error('Setup remote database failed:', error)
    showMessage('设置远程数据库失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

// 同步相关方法
async function testSyncConnection() {
  if (!canTestSync.value || !syncConfig.value) return
  
  isTestingSync.value = true
  try {
    const result = await invoke('test_remote_connection', {
      remoteUrl: syncConfig.value.remote_url,
      authToken: syncConfig.value.auth_token
    })
    
    if (result) {
      showMessage('远程数据库连接测试成功！', { title: '测试成功' })
    } else {
      showMessage('远程数据库连接测试失败，请检查URL和令牌！', { title: '测试失败' })
    }
  } catch (error) {
    console.error('Test connection failed:', error)
    showMessage('测试连接失败: ' + error, { title: '错误' })
  } finally {
    isTestingSync.value = false
  }
}

async function applySyncSettings() {
  if (!canApplySyncSettings.value || !syncConfig.value) return
  
  isOperationInProgress.value = true
  try {
    await invoke('save_sync_config', { config: syncConfig.value })
    await invoke('set_sync_mode', { mode: syncConfig.value.sync_mode })
    
    showMessage('同步设置已应用！', { title: '设置成功' })
    await loadSyncStatus()
  } catch (error) {
    console.error('Apply sync settings failed:', error)
    showMessage('应用同步设置失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function disableSync() {
  const confirmed = await showConfirm('确定要禁用远程同步吗？', {
    title: '确认禁用',
    confirmText: '禁用',
    cancelText: '取消'
  })
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    await invoke('set_sync_mode', { mode: 'OFFLINE' })
    if (syncConfig.value) {
      syncConfig.value.sync_mode = 'OFFLINE'
      await invoke('save_sync_config', { config: syncConfig.value })
    }
    
    showMessage('远程同步已禁用！', { title: '禁用成功' })
    databaseType.value = 'local'
    await loadSyncStatus()
  } catch (error) {
    console.error('Disable sync failed:', error)
    showMessage('禁用同步失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function manualSync() {
  isOperationInProgress.value = true
  try {
    await invoke('manual_sync') as any
    showMessage('手动同步完成！', { title: '同步成功' })
    await loadSyncStatus()
  } catch (error) {
    console.error('Manual sync failed:', error)
    showMessage('手动同步失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

function showConflictDialog() {
  showConflictResolutionDialog.value = true
}

function showSyncHistoryDialog() {
  showSyncHistoryDialogVisible.value = true
}

async function resetSyncStatus() {
  const confirmed = await showConfirm('确定要重置同步状态吗？这将清除所有同步记录。', {
    title: '确认重置',
    confirmText: '重置',
    cancelText: '取消'
  })
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    // 重置同步配置到默认状态
    syncConfig.value = {
      remote_url: 'http://127.0.0.1:8888',
      auth_token: '',
      sync_mode: 'OFFLINE',
      sync_interval: 300,
      auto_sync_enabled: true
    }
    await invoke('save_sync_config', { config: syncConfig.value })
    await invoke('set_sync_mode', { mode: 'OFFLINE' })
    
    showMessage('同步状态已重置！', { title: '重置成功' })
    databaseType.value = 'local'
    await loadSyncStatus()
  } catch (error) {
    console.error('Reset sync status failed:', error)
    showMessage('重置同步状态失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function reinitializeRemoteDatabase() {
  const confirmed = await showConfirm('确定要重新初始化远程数据库吗？这将清空远程数据库并重新同步所有本地数据。', {
    title: '确认重新初始化',
    confirmText: '重新初始化',
    cancelText: '取消'
  })
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    // 重新配置远程数据库
    await invoke('configure_remote_database', {
      config: {
        remote_url: syncConfig.value.remote_url,
        auth_token: syncConfig.value.auth_token,
        sync_mode: syncConfig.value.sync_mode,
        sync_interval: syncConfig.value.sync_interval,
        auto_sync_enabled: syncConfig.value.auto_sync_enabled
      }
    })
    
    showMessage('远程数据库已重新初始化！', { title: '重新初始化成功' })
    await loadSyncStatus()
  } catch (error) {
    console.error('Reinitialize remote database failed:', error)
    showMessage('重新初始化远程数据库失败: ' + error, { title: '错误' })
  } finally {
    isOperationInProgress.value = false
  }
}

async function loadSyncStatus() {
  try {
    const status = await invoke('get_sync_status') as any
    syncStatus.value = status
    
    // 根据同步状态设置数据库类型
    if (syncStatus.value.is_enabled) {
      databaseType.value = 'remote'
    } else {
      databaseType.value = 'local'
    }
  } catch (error) {
    console.error('Load sync status failed:', error)
    // 使用默认状态作为回退
    syncStatus.value = {
      is_enabled: false,
      is_online: false,
      sync_mode: 'OFFLINE',
      last_sync_time: 0,
      stats: null
    }
  }
}

async function loadSyncConfig() {
  try {
    const config = await invoke('get_sync_config') as any
    if (config) {
      syncConfig.value = config
    } else {
      // 使用默认配置
      syncConfig.value = {
        remote_url: '',
        auth_token: '',
        sync_mode: 'OFFLINE',
        sync_interval: 300,
        auto_sync_enabled: true
      }
    }
  } catch (error) {
    console.error('Load sync config failed:', error)
    // 使用默认配置作为回退
    syncConfig.value = {
      remote_url: 'http://127.0.0.1:8888',
      auth_token: '',
      sync_mode: 'OFFLINE',
      sync_interval: 300,
      auto_sync_enabled: true
    }
  }
}

// 数据操作方法
async function backupDatabase() {
  // 如果启用了同步，提醒用户
  if (syncStatus.value.is_enabled) {
    const confirmed = await showConfirm(
      '检测到同步功能已启用。建议先禁用同步后再进行备份，以确保数据一致性。\n\n是否继续备份？',
      { title: '同步提醒', confirmText: '继续备份', cancelText: '取消' }
    )
    if (!confirmed) return
  }
  
  await handleDatabaseOperation(() => invoke('backup_database'))
}

async function restoreDatabase() {
  // 强制确认恢复操作
  const confirmed = await showConfirm(
    '恢复数据库将覆盖当前所有数据，此操作不可撤销！\n\n' +
    (syncStatus.value.is_enabled ? '同步功能将被自动禁用。\n\n' : '') +
    '确定要继续吗？',
    { title: '确认恢复', confirmText: '确认恢复', cancelText: '取消' }
  )
  
  if (!confirmed) return
  
  await handleDatabaseOperation(() => invoke('restore_database'))
}

async function exportAsMarkdown() {
  await handleDatabaseOperation(() => invoke('export_as_markdown'))
}

async function migrateConfigToDatabase() {
  await handleDatabaseOperation(() => invoke('migrate_config_to_database'))
}

// 组件挂载时加载设置
onMounted(async () => {
  try {
    await loadCurrentDatabasePath()
    await loadSyncStatus()
    await loadSyncConfig()
  } finally {
    isInitializing.value = false
  }
})
</script>

<style scoped>
/* 卡片悬停效果 */
.card:hover {
  transform: translateY(-2px);
  transition: transform 0.2s ease;
}

/* 模式选择卡片样式 */
.card.bg-base-200 {
  transition: all 0.3s ease;
}

.card.bg-base-200:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

/* 选中状态的卡片样式 */
.card.border-primary {
  box-shadow: 0 4px 12px rgba(var(--p) / 0.3);
}

.card.border-warning {
  box-shadow: 0 4px 12px rgba(var(--wa) / 0.3);
}

.card.border-success {
  box-shadow: 0 4px 12px rgba(var(--su) / 0.3);
}

/* 统计信息样式 */
.stats .stat {
  padding: 1rem;
}

.stats .stat-value {
  font-size: 1rem;
}
</style> 
<template>
  <div class="space-y-6">
    <!-- 初始化加载状态 -->
    <div v-if="isInitializing" class="flex justify-center items-center py-12">
      <span class="loading loading-spinner loading-lg"></span>
      <span class="ml-3 text-base-content/70">{{ $t('dataSettings.loading') }}</span>
    </div>
    
    <template v-else>
      <!-- 数据库配置卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
            </svg>
            {{ $t('dataSettings.databaseConfig') }}
          </h2>

          <!-- 数据库模式选择 -->
          <div class="form-control mb-6">
            <label class="label">
              <span class="label-text font-medium">{{ $t('dataSettings.databaseMode') }}</span>
              <span v-if="isLoadingStatus" class="loading loading-spinner loading-sm"></span>
            </label>
            
            <!-- 当前模式状态 -->
            <div v-if="databaseStatus" class="alert mb-4">
              <div class="flex items-center gap-2">
                <span class="text-lg">📊</span>
                <div class="flex-1">
                  <div class="font-semibold">
                    {{ $t('dataSettings.currentMode') }}: {{ $t(`dataSettings.modes.${currentDatabaseMode}.label`) }}
                  </div>
                  <div class="text-sm opacity-70">
                    {{ $t(`dataSettings.modes.${currentDatabaseMode}.description`) }}
                  </div>
                </div>
              </div>
              <div class="flex gap-2 ml-auto">
                <div class="badge" :class="databaseStatus.is_connected ? 'badge-success' : 'badge-error'">
                  {{ databaseStatus.is_connected ? $t('dataSettings.connected') : $t('dataSettings.disconnected') }}
                </div>
                <div v-if="databaseStore.supportsSync" 
                     class="badge" 
                     :class="databaseStatus?.sync_status?.is_online ? 'badge-primary' : 'badge-warning'">
                  {{ databaseStatus?.sync_status?.is_online ? $t('dataSettings.syncAvailable') : $t('dataSettings.syncOffline') }}
                </div>
              </div>
            </div>

            <!-- 同步状态指示器 -->
            <div v-if="databaseStore.supportsSync" class="mb-4">
              <SyncStatusIndicator />
            </div>

            <!-- 模式选择卡片 -->
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
              <div v-for="mode in availableModes.filter((m: any) => m.supported)" 
                   :key="mode.value"
                   class="card bg-base-200 hover:bg-base-300 cursor-pointer transition-all"
                   :class="{ 'border-2 border-primary': currentDatabaseMode === mode.value }"
                   @click="() => currentDatabaseMode === mode.value ? null : switchDatabaseMode(mode.value)">
                <div class="card-body p-4">
                  <div class="flex items-center gap-3">
                    <div class="flex-1">
                      <h3 class="font-semibold">{{ $t(`dataSettings.modes.${mode.value}.label`) }}</h3>
                      <p class="text-sm opacity-70">{{ $t(`dataSettings.modes.${mode.value}.description`) }}</p>
                      <div v-if="mode.value === 'embedded_replica'" class="mt-2">
                        <div class="badge badge-success badge-sm">{{ $t('dataSettings.modes.embedded_replica.recommended') }}</div>
                      </div>
                    </div>
                    <div v-if="currentDatabaseMode === mode.value" class="badge badge-primary">{{ $t('dataSettings.currentMode') }}</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 快速操作按钮 -->
            <div class="flex gap-2 flex-wrap">
              <button 
                v-if="currentDatabaseMode !== 'embedded_replica' && hasRemoteConfig"
                class="btn btn-primary btn-sm"
                @click="switchToEmbeddedReplicaMode"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ $t('dataSettings.actions.switchToEmbedded') }}
              </button>
              
              <button 
                v-if="databaseStore.supportsSync"
                class="btn btn-outline btn-sm"
                @click="performDatabaseSync"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ $t('dataSettings.actions.syncNow') }}
              </button>
              
              <button 
                class="btn btn-outline btn-sm"
                @click="testCurrentDatabaseConnection"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ $t('dataSettings.actions.testConnection') }}
              </button>
              
              <button 
                v-if="currentDatabaseMode === 'local'"
                class="btn btn-outline btn-info btn-sm"
                @click="optimizeDatabaseWAL"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ $t('dataSettings.actions.optimizeWAL') }}
              </button>
              
              <button 
                v-if="currentDatabaseMode === 'local'"
                class="btn btn-outline btn-warning btn-sm"
                @click="cleanupLocalDatabaseFiles"
                :disabled="isOperationInProgress">
                <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
                {{ $t('dataSettings.actions.cleanupFiles') }}
              </button>
            </div>
          </div>

          <!-- 远程数据库配置 -->
          <div class="space-y-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('dataSettings.remoteConfig.url') }}</span>
              </label>
              <input 
                type="text" 
                v-model="syncConfig.remote_url" 
                :placeholder="$t('dataSettings.remoteConfig.urlPlaceholder')"
                class="input input-bordered"
                :disabled="isOperationInProgress"
                v-if="syncConfig"
              />
              <label class="label">
                <span class="label-text-alt">{{ $t('dataSettings.remoteConfig.urlHelp') }}</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('dataSettings.remoteConfig.token') }}</span>
                <span v-if="isLocalDevUrl" class="label-text-alt text-info">
                  {{ $t('dataSettings.remoteConfig.tokenOptional') }}
                </span>
              </label>
              <div class="join">
                <input 
                  :type="showSyncToken ? 'text' : 'password'"
                  v-model="syncConfig.auth_token" 
                  :placeholder="isLocalDevUrl ? $t('dataSettings.remoteConfig.tokenPlaceholderLocal') : $t('dataSettings.remoteConfig.tokenPlaceholder')"
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
            </div>

            <div class="flex gap-2">
              <button 
                class="btn btn-primary btn-sm" 
                @click="testAndSaveRemoteConfig"
                :disabled="!canTestSync || isTestingSync"
              >
                <span v-if="isTestingSync" class="loading loading-spinner loading-sm mr-2"></span>
                {{ isTestingSync ? $t('dataSettings.remoteConfig.testing') : $t('dataSettings.remoteConfig.testAndSave') }}
              </button>
            </div>
          </div>

          <!-- 数据库信息显示 -->
          <div v-if="databaseInfo" class="mt-4 space-y-4">
            <!-- 基本信息统计 -->
            <div class="stats shadow bg-base-200">
              <div class="stat">
                <div class="stat-title">{{ $t('dataSettings.databaseInfo.size') }}</div>
                <div class="stat-value text-sm">{{ databaseInfo.size }}</div>
                <div v-if="databaseInfo.mode_description" class="stat-desc">{{ databaseInfo.mode_description }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">{{ $t('dataSettings.databaseInfo.noteCount') }}</div>
                <div class="stat-value text-sm">{{ databaseInfo.noteCount }}</div>
              </div>
              <div class="stat">
                <div class="stat-title">{{ $t('dataSettings.databaseInfo.categoryCount') }}</div>
                <div class="stat-value text-sm">{{ databaseInfo.categoryCount }}</div>
              </div>
              <div v-if="databaseInfo.isRemote" class="stat">
                <div class="stat-title">{{ $t('dataSettings.databaseInfo.onlineStatus') }}</div>
                <div class="stat-value text-sm">
                  <div class="badge" :class="databaseInfo.isOnline ? 'badge-success' : 'badge-error'">
                    {{ databaseInfo.isOnline ? $t('dataSettings.databaseInfo.online') : $t('dataSettings.databaseInfo.offline') }}
                  </div>
                </div>
              </div>
            </div>

            <!-- 数据库路径信息 -->
            <div v-if="databaseInfo.database_path" class="alert alert-info">
              <div class="flex-1 min-w-0">
                <div class="font-semibold text-sm mb-2 flex items-center gap-2">
                  <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
                  </svg>
                  {{ $t('dataSettings.databaseInfo.path') }}
                </div>
                <div class="bg-base-100 rounded-lg border p-3 space-y-2">
                  <div class="text-xs font-mono break-all leading-relaxed select-all 
                              bg-base-200 p-2 rounded border-l-4 border-primary
                              hover:bg-base-300 transition-colors cursor-text
                              min-h-[2.5rem] flex items-center"
                       :title="`${t('dataSettings.databaseInfo.copyPath')} • ${databaseInfo.database_path}`">
                    {{ databaseInfo.database_path }}
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center text-xs opacity-70 gap-1">
                    <span>{{ $t('dataSettings.databaseInfo.lastModified') }}: {{ databaseInfo.lastModified }}</span>
                    <span class="text-primary font-medium">{{ databaseInfo.size }}</span>
                  </div>
                </div>
              </div>
              <div class="flex-none ml-3">
                <button 
                  class="btn btn-ghost btn-xs tooltip tooltip-left"
                  @click="copyDatabasePath"
                  :data-tip="$t('dataSettings.databaseInfo.copyPath')"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 媒体文件管理卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            {{ $t('dataSettings.mediaManagement.title') }}
          </h2>

          <!-- 媒体文件统计信息 -->
          <div v-if="mediaStatistics" class="stats shadow bg-base-200 mb-4">
            <div class="stat">
              <div class="stat-title">{{ $t('dataSettings.mediaManagement.totalImages') }}</div>
              <div class="stat-value text-sm">{{ mediaStatistics.total_images }}</div>
              <div class="stat-desc">{{ $t('dataSettings.mediaManagement.orphanedImages') }}: {{ mediaStatistics.orphaned_images }}</div>
            </div>
            <div class="stat">
              <div class="stat-title">{{ $t('dataSettings.mediaManagement.totalAudio') }}</div>
              <div class="stat-value text-sm">{{ mediaStatistics.total_audio_files }}</div>
              <div class="stat-desc">{{ $t('dataSettings.mediaManagement.orphanedAudio') }}: {{ mediaStatistics.orphaned_audio_files }}</div>
            </div>
            <div class="stat">
              <div class="stat-title">{{ $t('dataSettings.mediaManagement.totalOrphaned') }}</div>
              <div class="stat-value text-sm text-warning">{{ mediaStatistics.orphaned_images + mediaStatistics.orphaned_audio_files }}</div>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            <button 
              class="btn btn-outline btn-info" 
              @click="getMediaStatistics"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
              {{ $t('dataSettings.mediaManagement.getStatistics') }}
            </button>
            
            <button 
              class="btn btn-outline btn-warning" 
              @click="cleanupOrphanedMedia"
              :disabled="isOperationInProgress || !mediaStatistics || (mediaStatistics.orphaned_images + mediaStatistics.orphaned_audio_files) === 0"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              {{ $t('dataSettings.mediaManagement.cleanupOrphaned') }}
            </button>
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
            {{ $t('dataSettings.backupAndRestore.title') }}
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
            <button 
              class="btn btn-outline" 
              @click="backupDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
              </svg>
              {{ isOperationInProgress ? $t('dataSettings.backupAndRestore.backingUp') : $t('dataSettings.backupAndRestore.localBackup') }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="restoreDatabase"
              :disabled="isOperationInProgress"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 17l4 4 4-4m-4-5v9m-8-6h16" />
              </svg>
              {{ isOperationInProgress ? $t('dataSettings.backupAndRestore.restoring') : $t('dataSettings.backupAndRestore.localRestore') }}
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
              {{ isOperationInProgress ? $t('dataSettings.backupAndRestore.exporting') : $t('dataSettings.backupAndRestore.exportMarkdown') }}
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
              {{ isOperationInProgress ? $t('dataSettings.backupAndRestore.migrating') : $t('dataSettings.backupAndRestore.migrateConfig') }}
            </button>
          </div>
        </div>
      </div>

      <!-- 证书生成管理卡片 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
            </svg>
            {{ $t('dataSettings.certificateManagement.title') }}
          </h2>

          <div class="mb-4">
            <p class="text-sm text-base-content/70 mb-4">
              {{ $t('dataSettings.certificateManagement.description') }}
            </p>
            
            <!-- 证书保存目录选择 -->
            <div class="form-control mb-4">
              <label class="label">
                <span class="label-text">{{ $t('dataSettings.certificateManagement.outputDirectory') }}</span>
              </label>
              <div class="join">
                <input 
                  type="text" 
                  v-model="certificateOutputDir" 
                  :placeholder="$t('dataSettings.certificateManagement.outputDirectoryPlaceholder')"
                  class="input input-bordered join-item flex-1"
                  :disabled="isOperationInProgress"
                />
                <button 
                  type="button" 
                  class="btn btn-outline join-item"
                  @click="selectCertificateDirectory"
                  :disabled="isOperationInProgress"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
                  </svg>
                  {{ $t('dataSettings.certificateManagement.selectDirectory') }}
                </button>
              </div>
            </div>
          </div>

          <!-- 证书生成结果显示 -->
          <div v-if="certificateResult" class="mb-4">
            <div class="alert" :class="certificateResult.success ? 'alert-success' : 'alert-error'">
              <div class="flex-1">
                <div class="font-semibold mb-2">{{ certificateResult.message }}</div>
                <div v-if="certificateResult.success" class="space-y-2">
                  <div class="text-sm">
                    <strong>CA证书:</strong> {{ certificateResult.ca_cert.cert_path }}
                  </div>
                  <div class="text-sm">
                    <strong>服务器证书:</strong> {{ certificateResult.server_cert.cert_path }}
                  </div>
                  <div class="text-sm">
                    <strong>客户端证书:</strong> {{ certificateResult.client_cert.cert_path }}
                  </div>
                  <div class="text-sm text-warning">
                    <strong>过期时间:</strong> {{ certificateResult.ca_cert.expires_at }}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="flex gap-3">
            <button 
              class="btn btn-primary" 
              @click="generateCertificates"
              :disabled="isOperationInProgress || !certificateOutputDir.trim()"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
              {{ isOperationInProgress ? $t('dataSettings.certificateManagement.generating') : $t('dataSettings.certificateManagement.generateCertificates') }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="openCertificateDirectory"
              :disabled="!certificateResult?.success || !certificateOutputDir.trim()"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
              </svg>
              {{ $t('dataSettings.certificateManagement.openDirectory') }}
            </button>
          </div>
        </div>
      </div>

      <!-- JWT密钥生成管理 -->
      <div class="card bg-base-100 shadow-lg">
        <div class="card-body">
          <h2 class="card-title text-primary mb-4">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-3a1 1 0 011-1h2.586l6.243-6.243A6 6 0 0121 9z" />
            </svg>
            {{ $t('dataSettings.jwtManagement.title') }}
          </h2>

          <div class="mb-4">
            <p class="text-sm text-base-content/70 mb-4">
              {{ $t('dataSettings.jwtManagement.description') }}
            </p>
            
            <!-- JWT保存目录选择 -->
            <div class="form-control mb-4">
              <label class="label">
                <span class="label-text">{{ $t('dataSettings.jwtManagement.outputDirectory') }}</span>
              </label>
              <div class="join">
                <input 
                  type="text" 
                  v-model="jwtOutputDir" 
                  :placeholder="$t('dataSettings.jwtManagement.outputDirectoryPlaceholder')"
                  class="input input-bordered join-item flex-1"
                  :disabled="isOperationInProgress"
                />
                <button 
                  type="button" 
                  class="btn btn-outline join-item"
                  @click="selectJwtDirectory"
                  :disabled="isOperationInProgress"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
                  </svg>
                  {{ $t('dataSettings.jwtManagement.selectDirectory') }}
                </button>
              </div>
            </div>
          </div>

          <!-- JWT生成结果显示 -->
          <div v-if="jwtResult" class="mb-4">
            <div class="alert" :class="jwtResult.success ? 'alert-success' : 'alert-error'">
              <div class="flex-1">
                <div class="font-semibold mb-2">{{ jwtResult.message }}</div>
                <div v-if="jwtResult.success" class="space-y-2">
                  <div class="text-sm">
                    <strong>{{ $t('dataSettings.jwtManagement.publicKeyPem') }}:</strong> {{ jwtResult.public_key_pem_path }}
                  </div>
                  <div class="text-sm">
                    <strong>{{ $t('dataSettings.jwtManagement.publicKeyBase64') }}:</strong> {{ jwtResult.public_key_base64_path }}
                  </div>
                  <div class="text-sm text-info">
                    <strong>{{ $t('dataSettings.jwtManagement.fullAccessToken') }}:</strong>
                    <div class="font-mono text-xs break-all bg-base-200 p-2 rounded mt-1">{{ jwtResult.full_access_token }}</div>
                  </div>
                  <div class="text-sm text-warning">
                    <strong>{{ $t('dataSettings.jwtManagement.readOnlyToken') }}:</strong>
                    <div class="font-mono text-xs break-all bg-base-200 p-2 rounded mt-1">{{ jwtResult.read_only_token }}</div>
                  </div>
                  <div class="text-sm text-warning">
                    <strong>{{ $t('dataSettings.jwtManagement.expiresAt') }}:</strong> {{ jwtResult.expires_at }}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="flex gap-3">
            <button 
              class="btn btn-primary" 
              @click="generateJwtKeys"
              :disabled="isOperationInProgress || !jwtOutputDir.trim()"
            >
              <span v-if="isOperationInProgress" class="loading loading-spinner loading-sm mr-2"></span>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-3a1 1 0 011-1h2.586l6.243-6.243A6 6 0 0121 9z" />
              </svg>
              {{ isOperationInProgress ? $t('dataSettings.jwtManagement.generating') : $t('dataSettings.jwtManagement.generateKeys') }}
            </button>
            
            <button 
              class="btn btn-outline" 
              @click="openJwtDirectory"
              :disabled="!jwtResult?.success || !jwtOutputDir.trim()"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2 2z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0H8v0z" />
              </svg>
              {{ $t('dataSettings.jwtManagement.openDirectory') }}
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
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { showMessage, showConfirm } from '../../services/dialog'
import ConflictResolutionDialog from '../dialogs/ConflictResolutionDialog.vue'
import SyncHistoryDialog from '../dialogs/SyncHistoryDialog.vue'
import SyncStatusIndicator from '../SyncStatusIndicator.vue'
import { useDatabaseStore } from '../../stores/databaseStore'
import { DatabaseService } from '../../services/databaseService'
import type { LegacySyncConfig } from '../../types/database'

const { t } = useI18n()

// 使用数据库store
const databaseStore = useDatabaseStore()

// 数据库模式管理（从store获取）
const currentDatabaseMode = computed(() => databaseStore.currentMode)
const availableModes = computed(() => databaseStore.availableModes)
const databaseStatus = computed(() => databaseStore.databaseStatus)

// 操作状态管理
const isOperationInProgress = ref(false)
const isInitializing = ref(true)
const isLoadingStatus = ref(false)
const isTestingSync = ref(false)
const showSyncToken = ref(false)
const showConflictResolutionDialog = ref(false)
const showSyncHistoryDialogVisible = ref(false)

// 数据库信息
const databaseInfo = ref<{
  size: string
  noteCount: number
  categoryCount: number
  lastModified: string
  database_path?: string
  mode_description?: string
  isRemote?: boolean
  isOnline?: boolean
} | null>(null)

// 媒体文件统计信息
const mediaStatistics = ref<{
  total_images: number
  orphaned_images: number
  total_audio_files: number
  orphaned_audio_files: number
} | null>(null)

// 同步配置
const syncConfig = ref<LegacySyncConfig>({
  remote_url: '',
  auth_token: '',
  sync_mode: 'OFFLINE' as const,
  sync_interval: 300,
  auto_sync_enabled: true,
  is_online: false
})

// 证书生成相关
const certificateOutputDir = ref('')
const certificateResult = ref<{
  ca_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
  server_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
  client_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
  success: boolean
  message: string
} | null>(null)

// JWT生成相关
const jwtOutputDir = ref('')
const jwtResult = ref<{
  public_key_pem_path: string
  public_key_base64_path: string
  private_key_pem_path: string
  full_access_token: string
  read_only_token: string
  expires_at: string
  success: boolean
  message: string
} | null>(null)

// 计算属性
const canTestSync = computed(() => {
  return syncConfig.value?.remote_url?.trim() !== ''
})

const isLocalDevUrl = computed(() => {
  const url = syncConfig.value?.remote_url?.trim() || ''
  return url.startsWith('http://127.0.0.1') || 
         url.startsWith('http://localhost') ||
         url.startsWith('https://127.0.0.1') ||
         url.startsWith('https://localhost')
})

const hasRemoteConfig = computed(() => {
  const hasUrl = syncConfig.value?.remote_url?.trim() !== ''
  if (!hasUrl) return false
  
  const hasToken = syncConfig.value?.auth_token?.trim() !== ''
  
  // 本地开发环境允许空token
  if (isLocalDevUrl.value) {
    return true
  }
  
  // 生产环境需要token
  return hasToken
})

// === 数据库状态管理 ===

/**
 * 加载数据库状态
 */
async function loadDatabaseStatus() {
  isLoadingStatus.value = true
  try {
    const status = await databaseStore.loadStatus(true)
    
    // 更新数据库信息
    if (status?.database_info) {
      databaseInfo.value = {
        size: status.database_info.size,
        noteCount: status.database_info.note_count,
        categoryCount: status.database_info.category_count,
        lastModified: status.database_info.last_modified,
        database_path: status.database_info.database_path,
        mode_description: status.database_info.mode_description,
        isRemote: databaseStore.supportsSync,
        isOnline: status.is_connected
      }
    }
    
    console.log('Database status loaded:', status)
  } catch (error) {
    console.error('Failed to load database status:', error)
    showMessage(`${t('dataSettings.prompts.switchFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isLoadingStatus.value = false
  }
}

/**
 * 切换数据库模式
 */
async function switchDatabaseMode(mode: string, params?: any) {
  if (isOperationInProgress.value) return
  
  const modeOption = availableModes.value.find(m => m.value === mode)
  if (!modeOption) {
    showMessage(t('dataSettings.prompts.unsupportedMode'), { title: t('common.error') })
    return
  }
  
  const confirmed = await showConfirm(
    t('dataSettings.prompts.switchToModeDesc', { description: t(`dataSettings.modes.${mode}.description`) }),
    {
      title: t('dataSettings.prompts.switchToMode', { modeLabel: t(`dataSettings.modes.${mode}.label`) }),
      confirmText: t('dataSettings.prompts.switchTo'),
      cancelText: t('dataSettings.prompts.cancel')
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await databaseStore.switchMode(mode, params || syncConfig.value)
    showMessage(t('dataSettings.prompts.switchSuccess', { result }), { title: t('common.success') })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Failed to switch database mode:', error)
    showMessage(`${t('dataSettings.prompts.switchFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 切换到嵌入式副本模式（推荐）
 */
async function switchToEmbeddedReplicaMode() {
  
  // 检查是否有远程配置信息
  if (!hasRemoteConfig.value) {
    showMessage(t('dataSettings.prompts.needRemoteConfigDesc'), { title: t('dataSettings.prompts.needRemoteConfigTitle') })
    return
  }
  
  // 构建完整的嵌入式副本模式参数
  const embeddedReplicaParams = {
    remote_url: syncConfig.value.remote_url,
    auth_token: syncConfig.value.auth_token,
    sync_interval: syncConfig.value.sync_interval || 300,
    sync_interval_seconds: syncConfig.value.sync_interval || 300,
    local_path: undefined, // 使用默认本地路径
    read_your_writes: true, // 启用读写一致性
    auto_sync_enabled: true
  }
  
  try {
    await switchDatabaseMode('embedded_replica', embeddedReplicaParams)
  } catch (error) {
    // 如果错误提到WAL索引，提供清理选项
    const errorMessage = String(error)
    if (errorMessage.includes('wal_index') || errorMessage.includes('WAL') || errorMessage.includes('schema verification failure')) {
      const shouldCleanup = await showConfirm(
        t('dataSettings.prompts.switchFailedWAL'),
        {
          title: t('dataSettings.prompts.walCleanupFailedTitle'),
          confirmText: t('dataSettings.prompts.walCleanupRetry'),
          cancelText: t('dataSettings.prompts.cancel')
        }
      )
      
      if (shouldCleanup) {
        isOperationInProgress.value = true
        try {
          // 执行清理
          const cleanupResult = await DatabaseService.cleanupLocalDatabaseFiles()
          console.log('Cleanup result:', cleanupResult)
          
          // 短暂延时后重试
          await new Promise(resolve => setTimeout(resolve, 500))
          
          // 清理后重试
          await switchDatabaseMode('embedded_replica', embeddedReplicaParams)
        } catch (retryError) {
          console.error('Retry after cleanup failed:', retryError)
          showMessage(`${t('dataSettings.prompts.walCleanupFailedMessage')}: ${retryError}`, { title: t('common.error') })
        } finally {
          isOperationInProgress.value = false
        }
      }
    } else {
      showMessage(`${t('dataSettings.prompts.switchFailed')}: ${error}`, { title: t('common.error') })
    }
  }
}

/**
 * 执行数据库同步
 */
async function performDatabaseSync() {
  if (isOperationInProgress.value) return
  
  if (!databaseStore.supportsSync) {
    showMessage(t('dataSettings.prompts.syncNotSupported'), { title: t('common.tip') })
    return
  }
  
  isOperationInProgress.value = true
  try {
    const result = await databaseStore.sync()
    showMessage(t('dataSettings.prompts.syncSuccess', { result }), { title: t('common.success') })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Database sync failed:', error)
    showMessage(`${t('dataSettings.prompts.syncFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 测试数据库连接
 */
async function testCurrentDatabaseConnection() {
  if (isOperationInProgress.value) return
  
  isOperationInProgress.value = true
  try {
    const connected = await databaseStore.testConnection()
    if (connected) {
      showMessage(t('dataSettings.prompts.connectionTestSuccess'), { title: t('dataSettings.actions.testConnection') })
    } else {
      showMessage(t('dataSettings.prompts.connectionTestFailed'), { title: t('dataSettings.actions.testConnection') })
    }
  } catch (error) {
    console.error('Connection test failed:', error)
    showMessage(`${t('dataSettings.actions.testConnection')} ${t('common.failed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 清理本地数据库文件
 */
async function cleanupLocalDatabaseFiles() {
  if (isOperationInProgress.value) return
  
  const confirmed = await showConfirm(
    t('dataSettings.prompts.cleanupConfirmMessage'),
    {
      title: t('dataSettings.prompts.cleanupConfirmTitle'),
      confirmText: t('dataSettings.prompts.cleanup'),
      cancelText: t('dataSettings.prompts.cancel')
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await DatabaseService.cleanupLocalDatabaseFiles()
    showMessage(result, { title: t('dataSettings.prompts.cleanupComplete') })
  } catch (error) {
    console.error('Cleanup failed:', error)
    showMessage(`${t('dataSettings.prompts.cleanupFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 优化WAL文件
 */
async function optimizeDatabaseWAL() {
  if (isOperationInProgress.value) return
  
  const confirmed = await showConfirm(
    t('dataSettings.prompts.optimizeWALConfirmMessage'),
    {
      title: t('dataSettings.prompts.optimizeWALConfirmTitle'),
      confirmText: t('dataSettings.prompts.optimize'),
      cancelText: t('dataSettings.prompts.cancel')
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
         const result = await DatabaseService.optimizeDatabaseWAL()
    showMessage(result, { title: t('dataSettings.prompts.optimizeComplete') })
  } catch (error) {
    console.error('WAL optimization failed:', error)
    showMessage(`${t('dataSettings.prompts.optimizeFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

// === 远程配置管理 ===

/**
 * 测试并保存远程配置
 */
async function testAndSaveRemoteConfig() {
  if (!canTestSync.value || !syncConfig.value) return
  
  isTestingSync.value = true
  try {
    const result = await invoke('test_remote_connection', {
      remote_url: syncConfig.value.remote_url,
      auth_token: syncConfig.value.auth_token
    })
    
    if (result) {
      // 保存配置
      await invoke('save_sync_config', { config: syncConfig.value })
      showMessage(t('dataSettings.prompts.testSuccess'), { title: t('common.success') })
      
      // 刷新状态
      await loadDatabaseStatus()
    } else {
      showMessage(t('dataSettings.prompts.testFailed'), { title: t('common.failed') })
    }
  } catch (error) {
    console.error('Test connection failed:', error)
    showMessage(`${t('dataSettings.actions.testConnection')} ${t('common.failed')}: ${error}`, { title: t('common.error') })
  } finally {
    isTestingSync.value = false
  }
}

/**
 * 加载同步配置
 */
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
        auto_sync_enabled: true,
        is_online: false
      }
    }
    
  } catch (error) {
    console.error('Load sync config failed:', error)
    // 使用默认配置作为回退
    syncConfig.value = {
      remote_url: '',
      auth_token: '',
      sync_mode: 'OFFLINE',
      sync_interval: 300,
      auto_sync_enabled: true,
      is_online: false
    }
  }
}

// === 媒体文件管理函数 ===

/**
 * 获取媒体文件统计信息
 */
async function getMediaStatistics() {
  if (isOperationInProgress.value) return
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('get_media_statistics') as {
      total_images: number
      orphaned_images: number
      total_audio_files: number
      orphaned_audio_files: number
    }
    mediaStatistics.value = result
    showMessage(t('dataSettings.mediaManagement.statisticsLoaded'), { title: t('common.success') })
  } catch (error) {
    console.error('Failed to get media statistics:', error)
    showMessage(`${t('dataSettings.mediaManagement.statisticsFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 清理孤儿媒体文件
 */
async function cleanupOrphanedMedia() {
  if (isOperationInProgress.value) return
  
  if (!mediaStatistics.value || (mediaStatistics.value.orphaned_images + mediaStatistics.value.orphaned_audio_files) === 0) {
    showMessage(t('dataSettings.mediaManagement.noOrphanedFiles'), { title: t('common.tip') })
    return
  }
  
  const confirmed = await showConfirm(
    t('dataSettings.mediaManagement.cleanupConfirmMessage', {
      images: mediaStatistics.value.orphaned_images,
      audio: mediaStatistics.value.orphaned_audio_files
    }),
    {
      title: t('dataSettings.mediaManagement.cleanupConfirmTitle'),
      confirmText: t('dataSettings.mediaManagement.confirmCleanup'),
      cancelText: t('dataSettings.prompts.cancel')
    }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('cleanup_orphaned_media') as {
      deleted_images: number
      deleted_audio_files: number
    }
    
    showMessage(
      t('dataSettings.mediaManagement.cleanupSuccess', {
        images: result.deleted_images,
        audio: result.deleted_audio_files
      }),
      { title: t('common.success') }
    )
    
    // 重新获取统计信息
    await getMediaStatistics()
  } catch (error) {
    console.error('Failed to cleanup orphaned media:', error)
    showMessage(`${t('dataSettings.mediaManagement.cleanupFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

// === 辅助函数 ===

/**
 * 复制数据库路径到剪贴板
 */
async function copyDatabasePath() {
  if (!databaseInfo.value?.database_path) {
    showMessage(t('common.tip.emptyPath'), { title: t('common.tip.title') })
    return
  }
  try {
    await invoke('copy_to_clipboard', { text: databaseInfo.value.database_path })
    showMessage(t('dataSettings.prompts.copySuccess'), { title: t('common.success') })
  } catch (error) {
    console.error('Failed to copy database path:', error)
    showMessage(`${t('dataSettings.prompts.copyFailed')}: ${error}`, { title: t('common.error') })
  }
}



// === 数据操作方法 ===

async function backupDatabase() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('backup_database') as string
    showMessage(result, { title: t('dataSettings.prompts.backupSuccess') })
  } catch (error) {
    console.error('Backup failed:', error)
    showMessage(`${t('dataSettings.prompts.backupFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

async function restoreDatabase() {
  const confirmed = await showConfirm(
    t('dataSettings.prompts.restoreConfirmMessage'),
    { title: t('dataSettings.prompts.restoreConfirmTitle'), confirmText: t('dataSettings.prompts.confirmRestore'), cancelText: t('dataSettings.prompts.cancel') }
  )
  
  if (!confirmed) return
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('restore_database') as string
    showMessage(result, { title: t('dataSettings.prompts.restoreSuccess') })
    await loadDatabaseStatus()
  } catch (error) {
    console.error('Restore failed:', error)
    showMessage(`${t('dataSettings.prompts.restoreFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

async function exportAsMarkdown() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('export_as_markdown') as string
    showMessage(result, { title: t('dataSettings.prompts.exportSuccess') })
  } catch (error) {
    console.error('Export failed:', error)
    showMessage(`${t('dataSettings.prompts.exportFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

async function migrateConfigToDatabase() {
  isOperationInProgress.value = true
  try {
    const result = await invoke('migrate_config_to_database') as string
    showMessage(result, { title: t('dataSettings.prompts.migrateSuccess') })
  } catch (error) {
    console.error('Migration failed:', error)
    showMessage(`${t('dataSettings.prompts.migrateFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

// === 证书生成方法 ===

/**
 * 选择证书保存目录
 */
async function selectCertificateDirectory() {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: t('dataSettings.certificateManagement.selectDirectoryTitle') as string
    })
    
    if (typeof result === 'string' && result) {
      certificateOutputDir.value = result
    }
  } catch (error) {
    console.error('Failed to select directory:', error)
    showMessage(`${t('dataSettings.certificateManagement.selectDirectoryFailed')}: ${error}`, { title: t('common.error') })
  }
}

/**
 * 生成开发证书
 */
async function generateCertificates() {
  if (!certificateOutputDir.value.trim()) {
    showMessage(t('dataSettings.certificateManagement.selectDirectoryFirst'), { title: t('common.tip.title') })
    return
  }
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('generate_dev_certificates', {
      outputDir: certificateOutputDir.value
    }) as {
      ca_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
      server_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
      client_cert: { name: string; cert_path: string; key_path: string; expires_at: string }
      success: boolean
      message: string
    }
    
    certificateResult.value = result
    
    if (result.success) {
      // 显示成功消息
      const files = [
        'ca_cert.pem, ca_key.pem',
        'server_cert.pem, server_key.pem', 
        'client_cert.pem, client_key.pem'
      ].join('\n')
      
      showMessage(
        t('dataSettings.certificateManagement.generateSuccess', {
          files,
          expiry: result.ca_cert.expires_at
        }),
        { title: t('common.success') }
      )
    } else {
      showMessage(result.message, { title: t('common.error') })
    }
  } catch (error) {
    console.error('Failed to generate certificates:', error)
    showMessage(`${t('dataSettings.certificateManagement.generateFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 打开证书保存目录
 */
async function openCertificateDirectory() {
  if (!certificateOutputDir.value.trim()) {
    showMessage(t('dataSettings.certificateManagement.selectDirectoryFirst'), { title: t('common.tip.title') })
    return
  }
  
  try {
    await invoke('open_url', { url: certificateOutputDir.value })
  } catch (error) {
    console.error('Failed to open directory:', error)
    showMessage(`${t('dataSettings.certificateManagement.openDirectoryFailed')}: ${error}`, { title: t('common.error') })
  }
}

/**
 * 初始化证书输出目录
 */
async function initializeCertificateDirectory() {
  try {
    const defaultDir = await invoke('get_default_cert_directory') as string
    certificateOutputDir.value = defaultDir
  } catch (error) {
    console.error('Failed to get default certificate directory:', error)
  }
}

/**
 * 选择JWT保存目录
 */
async function selectJwtDirectory() {
  try {
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: t('dataSettings.jwtManagement.selectDirectoryTitle') as string
    })
    
    if (typeof result === 'string' && result) {
      jwtOutputDir.value = result
    }
  } catch (error) {
    console.error('Failed to select JWT directory:', error)
    showMessage(`${t('dataSettings.jwtManagement.selectDirectoryFailed')}: ${error}`, { title: t('common.error') })
  }
}

/**
 * 生成JWT密钥和令牌
 */
async function generateJwtKeys() {
  if (!jwtOutputDir.value.trim()) {
    showMessage(t('dataSettings.jwtManagement.selectDirectoryFirst'), { title: t('common.tip.title') })
    return
  }
  
  isOperationInProgress.value = true
  try {
    const result = await invoke('generate_jwt_keys', {
      outputDir: jwtOutputDir.value
    }) as {
      jwt_info: {
        public_key_pem_path: string
        public_key_base64_path: string
        full_access_token: string
        read_only_token: string
        expires_at: string
      } | null
      success: boolean
      message: string
    }
    
    // 构造前端期望的结构
    if (result.success && result.jwt_info) {
      jwtResult.value = {
        public_key_pem_path: result.jwt_info.public_key_pem_path,
        public_key_base64_path: result.jwt_info.public_key_base64_path,
        private_key_pem_path: '', // 后端没有返回私钥路径，暂时为空
        full_access_token: result.jwt_info.full_access_token,
        read_only_token: result.jwt_info.read_only_token,
        expires_at: result.jwt_info.expires_at,
        success: result.success,
        message: result.message
      }
      
      // 显示成功消息
      const files = [
        'jwt_key.pem (公钥PEM格式)',
        'jwt_key.base64 (公钥Base64格式)'
      ].join('\n')
      
      showMessage(
        t('dataSettings.jwtManagement.generateSuccess', {
          files,
          expiry: result.jwt_info.expires_at
        }),
        { title: t('common.success') }
      )
    } else {
      jwtResult.value = {
        public_key_pem_path: '',
        public_key_base64_path: '',
        private_key_pem_path: '',
        full_access_token: '',
        read_only_token: '',
        expires_at: '',
        success: result.success,
        message: result.message
      }
      showMessage(result.message, { title: t('common.error') })
    }
  } catch (error) {
    console.error('Failed to generate JWT keys:', error)
    showMessage(`${t('dataSettings.jwtManagement.generateFailed')}: ${error}`, { title: t('common.error') })
  } finally {
    isOperationInProgress.value = false
  }
}

/**
 * 打开JWT保存目录
 */
async function openJwtDirectory() {
  if (!jwtOutputDir.value.trim()) {
    showMessage(t('dataSettings.jwtManagement.selectDirectoryFirst'), { title: t('common.tip.title') })
    return
  }
  
  try {
    await invoke('open_url', { url: jwtOutputDir.value })
  } catch (error) {
    console.error('Failed to open JWT directory:', error)
    showMessage(`${t('dataSettings.jwtManagement.openDirectoryFailed')}: ${error}`, { title: t('common.error') })
  }
}

/**
 * 初始化JWT输出目录
 */
async function initializeJwtDirectory() {
  try {
    const defaultDir = await invoke('get_default_jwt_directory') as string
    jwtOutputDir.value = defaultDir
  } catch (error) {
    console.error('Failed to get default JWT directory:', error)
  }
}

// 组件挂载时加载设置
onMounted(async () => {
  try {
    // 加载数据库状态和配置
    await Promise.all([
      loadDatabaseStatus(),
      loadSyncConfig(),
      initializeCertificateDirectory(),
      initializeJwtDirectory()
    ])
  
    
  } catch (error) {
    console.error('[DataSettings] Initialization failed:', error)
  } finally {
    isInitializing.value = false
  }
})
</script>

<style scoped lang="postcss">
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

/* 数据库路径显示优化 */
.alert.alert-info {
  @apply border-info/20 bg-info/5;
}

/* 路径文本框样式 */
.select-all {
  user-select: all;
  -webkit-user-select: all;
  -moz-user-select: all;
  -ms-user-select: all;
}

/* 工具提示样式 */
.tooltip:before {
  @apply text-xs;
}

/* 响应式文本 */
@media (max-width: 640px) {
  .font-mono {
    word-break: break-all;
    overflow-wrap: break-word;
  }
  
  .text-xs {
    line-height: 1.4;
  }
}

/* 复制按钮动画 */
.btn:active {
  transform: scale(0.95);
}

/* 路径容器悬停效果 */
.cursor-text:hover {
  @apply shadow-sm;
}
</style>
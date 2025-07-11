<template>
  <div class="flex flex-col h-screen">
    <!-- 顶部导航 -->
    <div class="navbar bg-base-200 px-4">
      <div class="flex-1">
        <button class="btn btn-ghost" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <h1 class="text-xl font-bold">设置 - {{ getCurrentPageTitle() }}</h1>
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- 左侧导航栏 -->
      <nav class="w-64 min-w-[240px] max-w-[280px] bg-base-100 flex flex-col py-6 px-3 gap-2 shadow-lg border-r border-base-200">
        <div>
          <h2 class="text-lg font-semibold text-base-content px-3 mb-4">设置选项</h2>
        </div>
        
        <button 
          v-for="page in settingsPages" 
          :key="page.id"
          class="nav-button group flex items-center gap-3 px-4 py-3 rounded-lg text-left transition-all duration-200 ease-in-out hover:bg-base-200 hover:shadow-sm"
          :class="{ 
            'bg-primary text-primary-content shadow-md border-primary': currentPage === page.id,
            'text-base-content hover:text-primary border-transparent': currentPage !== page.id 
          }"
          @click="setCurrentPage(page.id)"
        >
          <!-- 图标 -->
          <div class="flex-shrink-0 w-5 h-5" :class="{ 'text-primary-content': currentPage === page.id, 'text-base-content/70 group-hover:text-primary': currentPage !== page.id }">
            <!-- 外观图标 -->
            <svg v-if="page.id === 'appearance'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z" />
            </svg>
            
            <!-- 临时笔记图标 -->
            <svg v-else-if="page.id === 'clipboard'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
            </svg>
            
            <!-- 网络图标 -->
            <svg v-else-if="page.id === 'network'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
            </svg>
            
            <!-- 数据管理图标 -->
            <svg v-else-if="page.id === 'data'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
            </svg>
            
            <!-- 应用设置图标 -->
            <svg v-else-if="page.id === 'app'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            
            <!-- AI助手图标 -->
            <svg v-else-if="page.id === 'ai'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
            </svg>
            
            <!-- 更新图标 -->
            <svg v-else-if="page.id === 'update'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            
            <!-- 提示词模板图标 -->
            <svg v-else-if="page.id === 'templates'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" />
            </svg>
            
            <!-- 关于图标 -->
            <svg v-else-if="page.id === 'about'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          
          <!-- 标题 -->
          <span class="font-medium text-sm flex-1">{{ page.title }}</span>
          
          <!-- 活跃状态指示器 -->
          <div v-if="currentPage === page.id" class="w-2 h-2 bg-primary-content rounded-full opacity-80"></div>
          
          <!-- 箭头指示器（非活跃状态） -->
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 opacity-0 group-hover:opacity-60 transition-opacity duration-200" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
        
        <!-- 底部装饰 -->
        <div class="mt-auto pt-4 border-t border-base-200">
          <div class="text-xs text-base-content/60 px-3 text-center">
            MyTips 设置中心
          </div>
        </div>
      </nav>

      <!-- 右侧内容区域 -->
      <div class="flex-1 overflow-auto p-6">
        <div class="max-w-3xl mx-auto">
          <!-- 外观设置 -->
          <div v-if="currentPage === 'appearance'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">外观设置</h2>

              <!-- 主题选择 -->
              <div class="form-control mb-6">
                <label class="label">
                  <span class="label-text">主题</span>
                </label>
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                  <template v-for="theme in uiStore.themeOptions" :key="theme.value">
                    <label :class="[
                      'cursor-pointer rounded-lg border-2 px-3 py-2 flex items-center gap-2',
                      uiStore.settings.theme === theme.value
                        ? 'border-primary'
                        : 'border-base-300'
                    ]" @click="changeTheme(theme.value)">
                      <input type="radio" :value="theme.value" v-model="selectedTheme"
                        class="radio radio-primary radio-sm" />
                      <span>{{ theme.name }}</span>
                    </label>
                  </template>
                </div>

                <!-- 主题预览区域 -->
                <div class="mt-6 p-4 rounded-lg border border-base-300">
                  <h3 class="text-lg font-medium mb-4">主题预览</h3>
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <!-- 按钮预览 -->
                    <div class="space-y-2">
                      <button class="btn btn-primary w-full">主要按钮</button>
                      <button class="btn btn-secondary w-full">次要按钮</button>
                      <button class="btn btn-accent w-full">强调按钮</button>
                      <button class="btn btn-neutral w-full">中性按钮</button>
                    </div>
                    <!-- 颜色预览 -->
                    <div class="grid grid-cols-2 gap-2">
                      <div class="p-3 rounded-lg bg-primary text-primary-content text-center">主色</div>
                      <div class="p-3 rounded-lg bg-secondary text-secondary-content text-center">次色</div>
                      <div class="p-3 rounded-lg bg-accent text-accent-content text-center">强调色</div>
                      <div class="p-3 rounded-lg bg-neutral text-neutral-content text-center">中性色</div>
                      <div class="p-3 rounded-lg bg-base-100 text-base-content border border-base-300 text-center">背景色</div>
                      <div class="p-3 rounded-lg bg-base-200 text-base-content border border-base-300 text-center">背景色 2</div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 字体大小设置 -->
              <div class="form-control mb-6">
                <label class="label">
                  <span class="label-text">字体大小</span>
                </label>

                <!-- 精确字体大小调整 -->
                <div class="mt-2">
                  <label class="label">
                    <span class="label-text">字体大小 ({{ exactFontSize }}px)</span>
                  </label>
                  <div class="flex items-center gap-2">
                    <button class="btn btn-sm btn-circle" @click="decreaseExactFontSize"
                      :disabled="exactFontSize <= 10">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                      </svg>
                    </button>
                    <input type="range" min="10" max="24" step="1" v-model.number="exactFontSize"
                      class="range range-primary range-sm flex-1" @input="updateExactFontSize" />
                    <button class="btn btn-sm btn-circle" @click="increaseExactFontSize"
                      :disabled="exactFontSize >= 24">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                    </button>
                  </div>
                </div>

                <!-- 字体大小预览 -->
                <div class="mt-4 p-4 rounded-lg border border-base-300">
                  <h3 class="text-lg font-medium mb-2">字体大小预览</h3>
                  <div class="space-y-3">
                    <div :style="`font-size: ${exactFontSize}px;`" class="p-2 border border-base-200 rounded">
                      <span>当前选择的字体大小 ({{ exactFontSize }}px)</span>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
                      <div :style="`font-size: 12px;`" class="p-2 border border-base-200 rounded">
                        <span>12px 大小示例</span>
                      </div>
                      <div :style="`font-size: 16px;`" class="p-2 border border-base-200 rounded">
                        <span>16px 大小示例</span>
                      </div>
                      <div :style="`font-size: 20px;`" class="p-2 border border-base-200 rounded">
                        <span>20px 大小示例</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 重置按钮 -->
              <div class="form-control mt-6">
                <button class="btn btn-outline" @click="resetUISettings">
                  重置为默认设置
                </button>
              </div>
            </div>
          </div>

          <!-- 临时笔记设置 -->
          <div v-else-if="currentPage === 'clipboard'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">临时笔记设置</h2>

              <div class="divider">内容捕获设置</div>

              <!-- 启用剪贴板监听 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">启用剪贴板监听</span>
                  <input type="checkbox" class="toggle toggle-primary"
                    v-model="clipboardSettings.enableMonitoring" @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，系统将自动监听并捕获剪贴板变化。关闭后，只能通过快捷键手动添加内容
                </p>
              </div>

              <!-- 应用白名单设置 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">启用应用白名单</span>
                  <input type="checkbox" class="toggle toggle-primary"
                    v-model="clipboardSettings.enableAppWhitelist" @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，来自白名单应用的剪贴板内容将不会被添加到临时笔记区
                </p>
              </div>

              <!-- 忽略敏感内容设置 -->
              <!-- <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">忽略敏感内容</span>
                  <input type="checkbox" class="toggle toggle-primary"
                    v-model="clipboardSettings.ignoreSensitiveContent" @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，将自动忽略从密码框等敏感区域复制的内容
                </p>
              </div> -->

              <!-- 捕获图片设置 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">捕获图片内容</span>
                  <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureImages"
                    @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，临时笔记区将同时捕获剪贴板中的图片内容
                </p>
              </div>

              <!-- 捕获来源信息设置 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">记录来源信息</span>
                  <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.captureSourceInfo"
                    @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，将记录剪贴板内容的来源应用或窗口名称
                </p>
              </div>

              <div class="divider">应用白名单管理</div>

              <!-- 白名单应用管理 -->
              <div v-if="clipboardSettings.enableAppWhitelist" class="form-control mb-4">
                <label class="label">
                  <span class="label-text">白名单应用管理</span>
                </label>
                
                <!-- 添加新应用 -->
                <div class="flex gap-2 mb-3">
                  <input 
                    type="text" 
                    v-model="newWhitelistApp" 
                    placeholder="输入应用名称（如：chrome.exe, notepad.exe）" 
                    class="input input-bordered flex-1"
                    @keydown.enter="addWhitelistApp"
                  />
                  <button 
                    class="btn btn-primary btn-sm" 
                    @click="addWhitelistApp"
                    :disabled="!newWhitelistApp.trim()"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    添加
                  </button>
                </div>

                <!-- 预设应用快速添加 -->
                <div class="mb-3">
                  <p class="text-sm text-base-content/80 mb-2">常用应用：</p>
                  <div class="flex flex-wrap gap-2">
                    <button 
                      v-for="preset in presetWhitelistApps" 
                      :key="preset.name"
                      class="btn btn-xs btn-outline"
                      @click="addPresetApp(preset.name)"
                      :disabled="clipboardSettings.whitelistApps.includes(preset.name)"
                    >
                      {{ preset.label }}
                    </button>
                  </div>
                </div>

                <!-- 白名单应用列表 -->
                <div class="bg-base-200 rounded-lg p-3 max-h-48 overflow-y-auto">
                  <div v-if="clipboardSettings.whitelistApps.length === 0" class="text-center py-4 text-base-content/60">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mx-auto mb-2 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <p class="text-sm">暂无白名单应用</p>
                  </div>
                  
                  <div v-else class="space-y-2">
                    <div 
                      v-for="(app, index) in clipboardSettings.whitelistApps" 
                      :key="index"
                      class="flex items-center justify-between p-2 bg-base-100 rounded border border-base-300"
                    >
                      <div class="flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-base-content/70" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        <span class="text-sm font-mono">{{ app }}</span>
                      </div>
                      <button 
                        class="btn btn-xs btn-error btn-outline"
                        @click="removeWhitelistApp(index)"
                      >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>

                <div class="mt-2 text-xs text-base-content/60">
                  <p>💡 提示：</p>
                  <ul class="list-disc list-inside ml-2 space-y-1">
                    <li>Windows: 使用进程名称（如 chrome.exe, notepad.exe）</li>
                    <li>macOS: 使用应用名称（如 Google Chrome, TextEdit）</li>
                    <li>Linux: 使用进程名称（如 chrome, gedit）</li>
                  </ul>
                </div>
              </div>

              <!-- 白名单功能未启用时的提示 -->
              <div v-else class="alert alert-info">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <span>请先启用应用白名单功能，然后管理白名单应用列表</span>
              </div>

              <div class="divider">数据保留设置</div>

              <!-- 保留时间设置 -->
              <div class="form-control mb-4">
                <label class="label">
                  <span class="label-text">临时笔记保留时间</span>
                </label>
                <select v-model="clipboardSettings.retentionDays" class="select select-bordered w-full"
                  @change="updateClipboardSettings">
                  <option v-for="option in uiStore.clipboardRetentionOptions" :key="option.value" :value="option.value">
                    {{ option.label }}
                  </option>
                </select>
                <p class="text-xs text-base-content/80 mt-1">
                  设置临时笔记区自动清理数据的时间，选择"永久保留"则不会自动清理
                </p>

                <button class="btn btn-outline btn-sm mt-2" @click="cleanExpiredEntries" :disabled="isCleaningEntries">
                  <span v-if="isCleaningEntries">
                    <span class="loading loading-spinner loading-xs mr-2"></span>
                    清理中...
                  </span>
                  <span v-else>
                    <div class="flex items-center">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                      立即清理过期内容
                    </div>
                  </span>
                </button>
              </div>

              <!-- 加密存储设置 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">加密存储临时笔记</span>
                  <input type="checkbox" class="toggle toggle-primary" v-model="clipboardSettings.encryptStorage"
                    @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，临时笔记将使用加密方式存储在数据库中
                </p>
              </div>

              <div class="divider">快捷键设置</div>

              <!-- 全局快捷键设置 -->
              <div class="form-control mb-4">
                <label class="label">
                  <span class="label-text">选中文本添加到临时笔记区</span>
                </label>
                <div class="flex items-center gap-2">
                  <div class="join">
                    <button class="btn join-item" 
                      :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('meta')}"
                      @click="toggleShortcutModifier('meta')">
                      {{ isMac ? '⌘ Command' : 'Ctrl' }}
                    </button>
                    <button class="btn join-item" 
                      :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('shift')}"
                      @click="toggleShortcutModifier('shift')">
                      Shift
                    </button>
                    <button class="btn join-item" 
                      :class="{'btn-primary': clipboardSettings.shortcut.modifiers.includes('alt')}"
                      @click="toggleShortcutModifier('alt')">
                      {{ isMac ? 'Option' : 'Alt' }}
                    </button>
                  </div>
                  <span class="mx-2">+</span>
                  <select v-model="clipboardSettings.shortcut.key" class="select select-bordered" @change="updateClipboardSettings">
                    <option v-for="key in shortcutKeys" :key="key.value" :value="key.value">{{ key.label }}</option>
                  </select>
                </div>
                <p class="text-xs text-base-content/80 mt-2">
                  选择修饰键和按键组合，用于将选中文本快速添加到临时笔记区
                </p>
                <div class="alert alert-warning mt-3" v-if="isShortcutInvalid">
                  <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                  <span>快捷键必须至少包含一个修饰键和一个字符键</span>
                </div>
                <button class="btn btn-sm btn-outline mt-3" @click="applyShortcutChanges" :disabled="isShortcutInvalid || isApplyingShortcut">
                  <span v-if="isApplyingShortcut">
                    <span class="loading loading-spinner loading-xs mr-2"></span>
                    应用中...
                  </span>
                  <span v-else>应用快捷键更改</span>
                </button>
                <div class="alert alert-info mt-3">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                  <span>修改快捷键后需要点击"应用快捷键更改"并重启应用才能生效</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 网络设置 -->
          <div v-else-if="currentPage === 'network'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">网络设置</h2>

              <!-- 代理设置 -->
              <div class="form-control">
                <div class="flex items-center mb-4">
                  <label class="label cursor-pointer">
                    <span class="label-text text-base mr-4">启用代理</span>
                    <input type="checkbox" v-model="proxySettings.enabled" class="toggle toggle-primary" />
                  </label>
                </div>

                <div v-if="proxySettings.enabled" class="grid gap-4">
                  <!-- 代理类型 -->
                  <div class="form-control">
                    <label class="label">
                      <span class="label-text">代理类型</span>
                    </label>
                    <select v-model="proxySettings.type" class="select select-bordered w-full">
                      <option value="http">HTTP</option>
                      <option value="socks5">SOCKS5</option>
                    </select>
                  </div>

                  <!-- 代理地址和端口 -->
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div class="form-control">
                      <label class="label">
                        <span class="label-text">主机</span>
                      </label>
                      <input type="text" v-model="proxySettings.host" placeholder="例如：127.0.0.1"
                        class="input input-bordered" />
                    </div>
                    <div class="form-control">
                      <label class="label">
                        <span class="label-text">端口</span>
                      </label>
                      <input type="number" v-model="proxySettings.port" placeholder="例如：7890"
                        class="input input-bordered" />
                    </div>
                  </div>

                  <!-- 代理认证 -->
                  <div class="form-control">
                    <div class="flex items-center">
                      <label class="label cursor-pointer">
                        <span class="label-text mr-4">需要认证</span>
                        <input type="checkbox" v-model="proxySettings.auth" class="checkbox checkbox-primary" />
                      </label>
                    </div>
                  </div>

                  <!-- 认证信息 -->
                  <div v-if="proxySettings.auth" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div class="form-control">
                      <label class="label">
                        <span class="label-text">用户名</span>
                      </label>
                      <input type="text" v-model="proxySettings.username" class="input input-bordered" />
                    </div>
                    <div class="form-control">
                      <label class="label">
                        <span class="label-text">密码</span>
                      </label>
                      <input type="password" v-model="proxySettings.password" class="input input-bordered" />
                    </div>
                  </div>

                  <!-- 测试按钮 -->
                  <div class="form-control mt-2">
                    <button class="btn btn-outline btn-primary" @click="testProxyConnection" :disabled="isTestingProxy">
                      <span v-if="isTestingProxy">
                        <span class="loading loading-spinner loading-xs mr-2"></span>
                        测试中...
                      </span>
                      <span v-else>测试连接</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 数据管理 -->
          <div v-else-if="currentPage === 'data'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">数据管理</h2>

              <!-- 数据库路径设置 -->
              <div class="mb-6">
                <h3 class="text-lg font-medium mb-4">数据库文件设置</h3>
                
                <!-- 当前数据库路径显示 -->
                <div class="form-control mb-4">
                  <label class="label">
                    <span class="label-text">当前数据库路径</span>
                  </label>
                  <div class="flex items-center gap-2">
                    <input 
                      type="text" 
                      :value="currentDatabasePath" 
                      readonly 
                      class="input input-bordered flex-1 bg-base-200"
                      :title="currentDatabasePath"
                    />
                    <button 
                      class="btn btn-outline btn-sm" 
                      @click="copyDatabasePath"
                      title="复制路径"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                      </svg>
                    </button>
                  </div>
                  <label class="label">
                    <span class="label-text-alt">这是当前正在使用的数据库文件路径</span>
                  </label>
                </div>

                <!-- 数据库路径选择 -->
                <div class="form-control mb-4">
                  <label class="label">
                    <span class="label-text">选择数据库文件</span>
                  </label>
                  <div class="flex gap-2">
                    <button 
                      class="btn btn-outline" 
                      @click="selectDatabaseFile"
                      :disabled="isChangingDatabase"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v0M8 5a2 2 0 012-2h2a2 2 0 012 2v0" />
                      </svg>
                      选择现有数据库文件
                    </button>
                    
                    <button 
                      class="btn btn-outline btn-secondary" 
                      @click="createNewDatabase"
                      :disabled="isChangingDatabase"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                      创建新数据库
                    </button>

                    <button 
                      class="btn btn-outline btn-info" 
                      @click="resetToDefaultDatabase"
                      :disabled="isChangingDatabase"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                      </svg>
                      重置为默认位置
                    </button>
                  </div>
                  <label class="label">
                    <span class="label-text-alt">
                      选择或创建新的数据库文件。更改数据库后需要重启应用。
                    </span>
                  </label>
                </div>

                <!-- 数据库信息显示 -->
                <div v-if="databaseInfo" class="alert alert-info">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                  <div>
                    <div class="font-medium">数据库信息</div>
                    <div class="text-sm">
                      <p>文件大小: {{ databaseInfo.size }}</p>
                      <p>笔记数量: {{ databaseInfo.noteCount }}</p>
                      <p>分类数量: {{ databaseInfo.categoryCount }}</p>
                      <p>最后修改: {{ databaseInfo.lastModified }}</p>
                    </div>
                  </div>
                </div>
              </div>

              <div class="divider">数据操作</div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <button class="btn btn-outline" @click="backupDatabase">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                  </svg>
                  备份数据库
                </button>
                <button class="btn btn-outline" @click="restoreDatabase">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
                  </svg>
                  恢复数据库
                </button>
                <button class="btn btn-outline" @click="exportAsMarkdown">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                  </svg>
                  导出为Markdown
                </button>
                <button class="btn btn-outline" @click="migrateConfigToDatabase">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
                  </svg>
                  迁移配置到数据库
                </button>
              </div>
            </div>
          </div>

          <!-- 应用设置 -->
          <div v-else-if="currentPage === 'app'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">应用设置</h2>

              <div class="form-control">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">开机自动启动</span>
                  <input type="checkbox" class="toggle toggle-primary" v-model="autoStartEnabled"
                    @change="toggleAutoStart" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，系统启动时将自动运行 MyTips
                </p>
              </div>
            </div>
          </div>

          <!-- AI助手 -->
          <div v-else-if="currentPage === 'ai'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">AI助手</h2>
              
              <div class="form-control mb-4">
                <label class="label">
                  <span class="label-text">全局AI默认模型</span>
                </label>
                <select v-model="defaultAIModel" class="select select-bordered w-full" @change="saveDefaultAIModel">
                  <option value="openai">OpenAI ChatGPT</option>
                  <option value="gemini">Gemini</option>
                  <option value="anthropic">Anthropic Claude</option>
                  <option value="deepseek">DeepSeek</option>
                  <option value="ali">通义千问</option>
                  <option value="doubao">字节豆包</option>
                  <option value="grok">xAI Grok</option>
                  <option value="custom">自定义模型</option>
                </select>
              </div>
              
              <div class="tabs tabs-boxed mb-4">
                <a 
                  v-for="(provider, id) in aiProviders" 
                  :key="id"
                  class="tab" 
                  :class="{ 'tab-active': selectedConfigModel === id }"
                  @click="selectedConfigModel = id"
                >
                  {{ provider.name }}
                </a>
              </div>
              
              <div v-if="selectedConfigModel" class="border rounded-lg p-4 mb-4">
                <h3 class="font-bold mb-2">{{ aiProviders[selectedConfigModel]?.name }} 配置</h3>
                
                <div class="form-control mb-2">
                  <label class="label">
                    <span class="label-text">API密钥</span>
                  </label>
                  <input 
                    type="password" 
                    v-model="aiProviders[selectedConfigModel].api_key" 
                    placeholder="输入API密钥" 
                    class="input input-bordered w-full"
                  />
                </div>
                
                <div class="form-control mb-2" v-if="selectedConfigModel === 'openai' || selectedConfigModel === 'custom'">
                  <label class="label">
                    <span class="label-text">API端点</span>
                  </label>
                  <input 
                    type="text" 
                    v-model="aiProviders[selectedConfigModel].api_base" 
                    placeholder="API端点URL" 
                    class="input input-bordered w-full"
                  />
                </div>
                
                <div class="form-control mb-2" v-if="selectedConfigModel === 'openai'">
                  <label class="label">
                    <span class="label-text">组织ID (可选)</span>
                  </label>
                  <input 
                    type="text" 
                    v-model="aiProviders[selectedConfigModel].organization" 
                    placeholder="组织ID" 
                    class="input input-bordered w-full"
                  />
                </div>
                
                <div class="form-control mb-2" v-if="selectedConfigModel === 'custom'">
                  <label class="label">
                    <span class="label-text">模型名称</span>
                  </label>
                  <input 
                    type="text" 
                    v-model="aiProviders[selectedConfigModel].default_model" 
                    placeholder="模型名称" 
                    class="input input-bordered w-full"
                  />
                </div>
                
                <!-- 输入框 + datalist，同时支持手动输入和下拉选择 -->
                <div class="form-control mb-4" v-else>
                  <label class="label">
                    <span class="label-text">默认模型</span>
                  </label>
                  <input
                    type="text"
                    v-model="aiProviders[selectedConfigModel].default_model"
                    :list="`models-${selectedConfigModel}`"
                    placeholder="输入模型名称或选择"
                    class="input input-bordered w-full"
                  />
                  <datalist :id="`models-${selectedConfigModel}`">
                    <option
                      v-for="model in aiProviders[selectedConfigModel].models"
                      :key="model.name"
                      :value="model.name"
                    />
                  </datalist>
                </div>
                
                <div class="flex justify-between">
                  <button 
                    class="btn btn-primary" 
                    :disabled="isTestingApi" 
                    @click="() => testApiConnection(selectedConfigModel)"
                  >
                    <span v-if="isTestingApi">测试中...</span>
                    <span v-else>测试连接</span>
                  </button>
                  
                  <button 
                    class="btn btn-accent" 
                    :disabled="isSavingApiConfig" 
                    @click="saveAIProviderConfig"
                  >
                    <span v-if="isSavingApiConfig">保存中...</span>
                    <span v-else>保存配置</span>
                  </button>
                </div>
              </div>
              
              <div class="divider">使用统计</div>
              
              <div class="stats stats-vertical lg:stats-horizontal shadow w-full">
                <div class="stat">
                  <div class="stat-title">对话总数</div>
                  <div class="stat-value">{{ aiStats.conversations }}</div>
                </div>
                
                <div class="stat">
                  <div class="stat-title">消息总数</div>
                  <div class="stat-value">{{ aiStats.messages }}</div>
                </div>
                
                <div class="stat">
                  <div class="stat-title">Token总数</div>
                  <div class="stat-value">{{ aiStats.tokens.total }}</div>
                  <div class="stat-desc">输入: {{ aiStats.tokens.input }} / 输出: {{ aiStats.tokens.output }}</div>
                </div>
              </div>
              
              <button class="btn btn-sm btn-ghost mt-2" @click="refreshAIStats">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
                刷新统计
              </button>
            </div>
          </div>

          <!-- 更新设置 -->
          <div v-else-if="currentPage === 'update'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">更新设置</h2>
              
              <!-- 当前版本信息 -->
              <div class="mb-6">
                <h3 class="text-lg font-medium mb-2">版本信息</h3>
                <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
                  <div>
                    <p class="font-medium">当前版本: {{ updateStore.currentVersion || '获取中...' }}</p>
                    <p class="text-sm text-base-content/80 mt-1">
                      上次检查: {{ formatLastCheckTime() }}
                    </p>
                  </div>
                  <div v-if="updateStore.hasUpdate" class="badge badge-error">
                    有新版本
                  </div>
                  <div v-else class="badge badge-success">
                    已是最新
                  </div>
                </div>
              </div>

              <!-- 新版本信息 -->
              <div v-if="updateStore.hasUpdate && updateStore.updateInfo" class="mb-6">
                <h3 class="text-lg font-medium mb-2">新版本可用</h3>
                <div class="p-4 bg-accent/10 border border-accent/20 rounded-lg">
                  <p class="font-medium text-accent mb-2">
                    版本 {{ updateStore.updateInfo.version }}
                  </p>
                  <p class="text-sm text-base-content/80 mb-2">
                    发布时间: {{ formatDate(updateStore.updateInfo.pub_date) }}
                  </p>
                  <div v-if="updateStore.updateInfo.body" class="text-sm bg-base-200 p-3 rounded max-h-32 overflow-y-auto">
                    <div v-html="formatReleaseNotes(updateStore.updateInfo.body)"></div>
                  </div>
                  <button 
                    class="btn btn-accent btn-sm mt-3" 
                    @click="showUpdateDialog = true"
                  >
                    立即更新
                  </button>
                </div>
              </div>

              <!-- 更新设置 -->
              <div class="mb-6">
                <h3 class="text-lg font-medium mb-4">更新设置</h3>
                
                <!-- 自动检查更新 -->
                <div class="form-control mb-4">
                  <label class="label cursor-pointer">
                    <span class="label-text">自动检查更新</span>
                    <input 
                      type="checkbox" 
                      class="checkbox checkbox-primary" 
                      v-model="updateStore.autoCheck"
                      @change="updateAutoCheckSetting"
                    />
                  </label>
                  <div class="label">
                    <span class="label-text-alt">启用后，应用会定期检查新版本</span>
                  </div>
                </div>

                <!-- 检查间隔 -->
                <div v-if="updateStore.autoCheck" class="form-control mb-4">
                  <label class="label">
                    <span class="label-text">检查间隔</span>
                  </label>
                  <select 
                    class="select select-bordered w-full max-w-xs" 
                    v-model="checkIntervalHours"
                    @change="updateCheckInterval"
                  >
                    <option value="1">每小时</option>
                    <option value="6">每6小时</option>
                    <option value="12">每12小时</option>
                    <option value="24">每天</option>
                    <option value="168">每周</option>
                  </select>
                </div>
              </div>

              <!-- 检查更新按钮 -->
              <div class="flex flex-wrap gap-3">
                <button 
                  class="btn btn-primary" 
                  @click="checkForUpdates"
                  :disabled="updateStore.isChecking"
                >
                  <span v-if="updateStore.isChecking" class="loading loading-spinner loading-sm mr-2"></span>
                  {{ updateStore.isChecking ? '检查中...' : '检查更新' }}
                </button>
                
                <button 
                  class="btn btn-outline btn-primary" 
                  @click="checkForUpdatesNoSignature"
                  :disabled="updateStore.isChecking"
                  title="跳过签名验证检查更新（用于解决签名问题）"
                >
                  <span v-if="updateStore.isChecking" class="loading loading-spinner loading-sm mr-2"></span>
                  {{ updateStore.isChecking ? '检查中...' : '检查更新(无签名)' }}
                </button>
                
                <button 
                  v-if="updateStore.hasUpdate" 
                  class="btn btn-accent" 
                  @click="showUpdateDialog = true"
                >
                  立即更新
                </button>

                <button 
                  class="btn btn-outline btn-info" 
                  @click="testWindowsUpdate"
                  :disabled="isTestingWindowsUpdate"
                >
                  <span v-if="isTestingWindowsUpdate" class="loading loading-spinner loading-sm mr-2"></span>
                  {{ isTestingWindowsUpdate ? '测试中...' : 'Windows更新测试' }}
                </button>

                <button 
                  class="btn btn-outline btn-secondary" 
                  @click="showPlatformInfo"
                >
                  平台信息
                </button>

                <button 
                  class="btn btn-outline btn-warning" 
                  @click="testWindowsUpdateNoSignature"
                  :disabled="isTestingWindowsUpdateNoSig"
                  title="测试无签名验证的Windows更新（用于网络问题诊断）"
                >
                  <span v-if="isTestingWindowsUpdateNoSig" class="loading loading-spinner loading-sm mr-2"></span>
                  {{ isTestingWindowsUpdateNoSig ? '测试中...' : '网络诊断测试' }}
                </button>
              </div>
            </div>
          </div>

          <!-- 关于 -->
          <div v-else-if="currentPage === 'about'" class="card bg-base-100 shadow-md">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">关于</h2>
              <p>MyTips v{{ updateStore.currentVersion || '0.2.0' }}</p>
              <p class="text-sm text-base-content/80 mt-2">
                一个简单的笔记管理应用，帮助你收集和整理有用的代码片段、文档和提示。
              </p>
            </div>
          </div>

          <!-- 提示词模板管理 -->
          <div v-else-if="currentPage === 'templates'" class="card bg-base-100 shadow-md">
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
        </div>
      </div>
    </div>

    <!-- 更新对话框 -->
    <UpdateDialog v-model="showUpdateDialog" />

    <!-- 模型配置管理对话框 -->
    <dialog ref="modelConfigModal" class="modal">
      <div class="modal-box w-11/12 max-w-3xl">
        <h3 class="font-bold text-lg mb-4">
          配置 {{ getModelNameById(editingModelType) }} 的模型名称建议
        </h3>

        <!-- 添加新模型 -->
        <div class="bg-base-200 rounded-lg p-4 mb-4">
          <h4 class="font-medium mb-3">添加新模型</h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div class="form-control">
              <input type="text" v-model="newModelName" placeholder="模型名称" class="input input-bordered input-sm"
                @keydown.enter="addNewModel" />
            </div>
            <div class="form-control">
              <input type="text" v-model="newModelDescription" placeholder="模型描述" class="input input-bordered input-sm"
                @keydown.enter="addNewModel" />
            </div>
          </div>
          <div class="mt-3">
            <button class="btn btn-sm btn-primary" @click="addNewModel"
              :disabled="!newModelName.trim() || !newModelDescription.trim()">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              添加模型
            </button>
          </div>
        </div>

        <!-- 现有模型列表 -->
        <div class="max-h-96 overflow-y-auto">
          <div class="space-y-2">
            <div v-for="(model, index) in editingModelList" :key="index"
              class="flex items-center justify-between p-3 bg-base-100 rounded-lg border border-base-300">
              <div class="flex-1">
                <div class="font-medium">{{ model.name }}</div>
                <div class="text-sm text-base-content/70">{{ model.description }}</div>
              </div>
              <button class="btn btn-xs btn-error ml-3" @click="removeModel(index)">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>

          <div v-if="editingModelList.length === 0" class="text-center py-8 text-base-content/70">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto mb-2 opacity-50" fill="none"
              viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2m13-4V6a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M9 9V6a1 1 0 011-1h4a1 1 0 011 1v3" />
            </svg>
            <p>暂无模型配置</p>
          </div>
        </div>

        <div class="modal-action">
          <button class="btn btn-ghost" @click="resetToDefault">重置为默认</button>
          <button class="btn" @click="closeModelConfigModal">取消</button>
          <button class="btn btn-primary" @click="saveModelConfig">保存配置</button>
        </div>
      </div>
    </dialog>

    <!-- 自定义模型配置模态框 -->
    <dialog ref="customModelModal" class="modal">
      <div class="modal-box w-11/12 max-w-2xl">
        <h3 class="font-bold text-lg mb-4">{{ editingCustomModelId ? '编辑自定义模型' : '添加自定义模型' }}</h3>
        
        <div class="space-y-4">
          <!-- 模型名称 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">模型名称 *</span>
            </label>
            <input type="text" v-model="customModelForm.name" placeholder="例如：我的GPT模型" 
                   class="input input-bordered w-full" />
          </div>

          <!-- API端点 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">API端点 *</span>
            </label>
            <input type="text" v-model="customModelForm.endpoint" 
                   placeholder="例如：https://api.openai.com/v1/chat/completions" 
                   class="input input-bordered w-full" />
            <label class="label">
              <span class="label-text-alt">完整的API端点URL，包括协议和路径</span>
            </label>
          </div>

          <!-- 模型名称（API） -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">模型标识符 *</span>
            </label>
            <input type="text" v-model="customModelForm.model_name" 
                   placeholder="例如：gpt-3.5-turbo" 
                   class="input input-bordered w-full" />
            <label class="label">
              <span class="label-text-alt">API调用时使用的模型名称</span>
            </label>
          </div>

          <!-- API密钥 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">API密钥</span>
            </label>
            <div class="input-group">
              <input 
                :type="showCustomModelApiKey ? 'text' : 'password'" 
                v-model="customModelForm.api_key" 
                placeholder="可选，某些本地模型不需要API密钥" 
                class="input input-bordered flex-1" />
              <button 
                type="button" 
                class="btn btn-square" 
                @click="showCustomModelApiKey = !showCustomModelApiKey">
                <svg v-if="showCustomModelApiKey" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464M9.878 9.878a3 3 0 104.243 4.243M4.929 19.071L19.071 4.929" />
                </svg>
                <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
              </button>
            </div>
            <label class="label">
              <span class="label-text-alt">留空表示不需要认证（如本地模型）</span>
            </label>
          </div>

          <!-- 适配器类型 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">适配器类型</span>
            </label>
            <select v-model="customModelForm.adapter_type" class="select select-bordered w-full">
              <option value="openai">OpenAI 兼容</option>
              <option value="anthropic">Anthropic (Claude)</option>
              <option value="gemini">Google Gemini</option>
              <option value="deepseek">DeepSeek</option>
            </select>
            <label class="label">
              <span class="label-text-alt">选择API格式兼容的类型</span>
            </label>
          </div>

          <!-- 自定义头部 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">自定义HTTP头部</span>
              <button type="button" class="btn btn-xs btn-primary" @click="addCustomHeader">
                添加头部
              </button>
            </label>
            <div v-if="customModelForm.custom_headers.length > 0" class="space-y-2">
              <div v-for="(header, index) in customModelForm.custom_headers" :key="index" 
                   class="flex gap-2 items-center">
                <input 
                  v-model="header.key" 
                  placeholder="头部名称" 
                  class="input input-bordered input-sm flex-1" />
                <input 
                  v-model="header.value" 
                  placeholder="头部值" 
                  class="input input-bordered input-sm flex-1" />
                <button type="button" class="btn btn-xs btn-error" @click="removeCustomHeader(index)">
                  删除
                </button>
              </div>
            </div>
            <label class="label">
              <span class="label-text-alt">可选，用于添加额外的HTTP头部</span>
            </label>
          </div>
        </div>

        <!-- 模态框操作按钮 -->
        <div class="modal-action">
          <button class="btn" @click="closeCustomModelModal">取消</button>
          <button 
            class="btn btn-secondary" 
            @click="testCustomModelConnection"
            :disabled="!canTestCustomModel || isTestingCustomModel">
            <span v-if="isTestingCustomModel" class="loading loading-spinner loading-xs"></span>
            {{ isTestingCustomModel ? '测试中...' : '测试连接' }}
          </button>
          <button 
            class="btn btn-primary" 
            @click="saveCustomModel"
            :disabled="!canSaveCustomModel || isSavingCustomModel">
            <span v-if="isSavingCustomModel" class="loading loading-spinner loading-xs"></span>
            {{ isSavingCustomModel ? '保存中...' : '保存配置' }}
          </button>
        </div>
      </div>
    </dialog>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed, onActivated } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { message } from '@tauri-apps/plugin-dialog'
import { useUIStore } from '../stores/uiStore'
import { useUpdateStore } from '../stores/updateStore'
import UpdateDialog from '../components/UpdateDialog.vue'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { useRouter } from 'vue-router'
import { showConfirm } from '../services/dialog'
import {
  AIProvider, AIModel, TestConnectionRequest,
  getChatModels, getDefaultAIModel,
  testAIConnection, saveAIConfig, getAIConfig, getAIUsageStats, reloadAIServices,
  getAIServiceStatus, defaultProviders
} from '../services/aiService'
import { useTipTemplateStore } from '../stores/tipTemplateStore'

const uiStore = useUIStore()
const updateStore = useUpdateStore()
const router = useRouter()

// 页面管理
const currentPage = ref('appearance')
const settingsPages = [
  { id: 'appearance', title: '外观' },
  { id: 'clipboard', title: '临时笔记' },
  { id: 'network', title: '网络' },
  { id: 'data', title: '数据管理' },
  { id: 'app', title: '应用设置' },
  { id: 'ai', title: 'AI助手' },
  { id: 'templates', title: '提示词模板' },
  { id: 'update', title: '更新' },
  { id: 'about', title: '关于' }
]

// 设置当前页面
function setCurrentPage(pageId: string) {
  currentPage.value = pageId
}

// 获取当前页面标题
function getCurrentPageTitle() {
  const page = settingsPages.find(p => p.id === currentPage.value)
  return page ? page.title : '设置'
}

// 更新相关状态
const showUpdateDialog = ref(false)
const checkIntervalHours = ref(1)
const isTestingWindowsUpdate = ref(false)
const isTestingWindowsUpdateNoSig = ref(false)

// 主题设置
const selectedTheme = ref(uiStore.settings.theme)

// UI设置
// const selectedFontSize = ref(uiStore.settings.fontSize)
// const selectedComponentSize = ref(uiStore.settings.componentSize)
const exactFontSize = ref<number>(Number(uiStore.settings.exactFontSize))
const exactSidebarWidth = ref<number>(Number(uiStore.settings.exactSidebarWidth || uiStore.settings.sidebarWidth))
const exactListWidth = ref<number>(Number(uiStore.settings.exactListWidth || uiStore.settings.listWidth))

// 剪贴板设置
const clipboardSettings = ref({
  enableMonitoring: true, // 默认启用剪贴板监听
  enableAppWhitelist: false, // 默认关闭应用白名单
  whitelistApps: [] as string[], // 白名单应用列表
  ignoreSensitiveContent: false,
  captureImages: false,
  captureSourceInfo: false,
  retentionDays: 30,
  encryptStorage: false,
  shortcut: {
    modifiers: ['meta', 'shift'], // 默认组合键：Command/Ctrl + Shift
    key: 'c' // 默认按键：C
  }
})

// 初始化时合并用户设置
if (uiStore.settings.clipboard) {
  Object.assign(clipboardSettings.value, uiStore.settings.clipboard)
}

// 白名单相关变量
const newWhitelistApp = ref('')

// 预设的常用应用
const presetWhitelistApps = [
  // Windows
  { name: 'chrome.exe', label: 'Chrome' },
  { name: 'firefox.exe', label: 'Firefox' },
  { name: 'edge.exe', label: 'Edge' },
  { name: 'notepad.exe', label: '记事本' },
  { name: 'winword.exe', label: 'Word' },
  { name: 'excel.exe', label: 'Excel' },
  { name: 'powerpnt.exe', label: 'PowerPoint' },
  { name: 'code.exe', label: 'VS Code' },
  { name: 'devenv.exe', label: 'Visual Studio' },
  { name: 'slack.exe', label: 'Slack' },
  { name: 'discord.exe', label: 'Discord' },
  { name: 'teams.exe', label: 'Teams' },
  // macOS
  { name: 'Google Chrome', label: 'Chrome (Mac)' },
  { name: 'Firefox', label: 'Firefox (Mac)' },
  { name: 'Safari', label: 'Safari' },
  { name: 'TextEdit', label: 'TextEdit' },
  { name: 'Microsoft Word', label: 'Word (Mac)' },
  { name: 'Microsoft Excel', label: 'Excel (Mac)' },
  { name: 'Visual Studio Code', label: 'VS Code (Mac)' },
  { name: 'Slack', label: 'Slack (Mac)' },
  { name: 'Discord', label: 'Discord (Mac)' },
  // Linux
  { name: 'chrome', label: 'Chrome (Linux)' },
  { name: 'firefox', label: 'Firefox (Linux)' },
  { name: 'gedit', label: 'Gedit' },
  { name: 'code', label: 'VS Code (Linux)' },
  { name: 'slack', label: 'Slack (Linux)' }
]

// 快捷键相关
const shortcutKeys = [
  { value: 'a', label: 'A' },
  { value: 'b', label: 'B' },
  { value: 'c', label: 'C' },
  { value: 'd', label: 'D' },
  { value: 'e', label: 'E' },
  { value: 'f', label: 'F' },
  { value: 'g', label: 'G' },
  { value: 'h', label: 'H' },
  { value: 'i', label: 'I' },
  { value: 'j', label: 'J' },
  { value: 'k', label: 'K' },
  { value: 'l', label: 'L' },
  { value: 'm', label: 'M' },
  { value: 'n', label: 'N' },
  { value: 'o', label: 'O' },
  { value: 'p', label: 'P' },
  { value: 'q', label: 'Q' },
  { value: 'r', label: 'R' },
  { value: 's', label: 'S' },
  { value: 't', label: 'T' },
  { value: 'u', label: 'U' },
  { value: 'v', label: 'V' },
  { value: 'w', label: 'W' },
  { value: 'x', label: 'X' },
  { value: 'y', label: 'Y' },
  { value: 'z', label: 'Z' },
  { value: '1', label: '1' },
  { value: '2', label: '2' },
  { value: '3', label: '3' },
  { value: '4', label: '4' },
  { value: '5', label: '5' },
  { value: '6', label: '6' },
  { value: '7', label: '7' },
  { value: '8', label: '8' },
  { value: '9', label: '9' },
  { value: '0', label: '0' }
]

// 判断当前快捷键是否有效
const isShortcutInvalid = computed(() => {
  return clipboardSettings.value.shortcut.modifiers.length === 0 || 
         !clipboardSettings.value.shortcut.key
})

// 快捷键修改状态
const isApplyingShortcut = ref(false)

// 切换修饰键
function toggleShortcutModifier(modifier: string) {
  const modifiers = clipboardSettings.value.shortcut.modifiers
  const index = modifiers.indexOf(modifier)
  
  if (index === -1) {
    // 添加修饰键
    modifiers.push(modifier)
  } else {
    // 移除修饰键，但确保至少有一个修饰键
    if (modifiers.length > 1) {
      modifiers.splice(index, 1)
    }
  }
}

// 应用快捷键更改
async function applyShortcutChanges() {
  if (isShortcutInvalid.value) return
  
  isApplyingShortcut.value = true
  try {
    // 转换为后端格式
    const shortcutConfig = {
      modifiers: clipboardSettings.value.shortcut.modifiers,
      key: clipboardSettings.value.shortcut.key
    }
    
    // 保存到后端
    await invoke('update_global_shortcut', { config: shortcutConfig })
    message('快捷键已更新，重启应用后生效', { title: '成功' })
  } catch (error) {
    console.error('更新快捷键失败:', error)
    message('更新快捷键失败: ' + error, { title: '错误' })
  } finally {
    isApplyingShortcut.value = false
  }
}

// 判断是否为Mac系统
const isMac = computed(() => {
  return navigator.platform.toUpperCase().indexOf('MAC') >= 0
})

// 同步剪贴板设置到后端
async function syncClipboardSettingsToBackend() {
  try {
    // 创建后端格式的设置对象
    const backendSettings = {
      ignore_sensitive_content: clipboardSettings.value.ignoreSensitiveContent,
      capture_images: clipboardSettings.value.captureImages,
      capture_source_info: clipboardSettings.value.captureSourceInfo,
      retention_days: clipboardSettings.value.retentionDays,
      encrypt_storage: clipboardSettings.value.encryptStorage,
      enable_monitoring: clipboardSettings.value.enableMonitoring,
      enable_app_whitelist: clipboardSettings.value.enableAppWhitelist,
      whitelist_apps: clipboardSettings.value.whitelistApps
    }

    // 保存到后端
    await invoke('save_clipboard_settings', { settings: backendSettings })
    console.log('剪贴板设置已同步到后端')
    
    // 根据监听设置，启用或禁用剪贴板监听
    if (clipboardSettings.value.enableMonitoring) {
      await invoke('start_clipboard_monitoring')
    } else {
      await invoke('stop_clipboard_monitoring')
    }
    
  } catch (error) {
    console.error('同步剪贴板设置失败:', error)
  }
}

// 从后端加载剪贴板设置
async function loadClipboardSettingsFromBackend() {
  try {
    const backendSettings = await invoke<any>('get_clipboard_settings')
    if (backendSettings) {
      // 转换为前端格式并更新
      clipboardSettings.value = {
        ignoreSensitiveContent: backendSettings.ignore_sensitive_content,
        captureImages: backendSettings.capture_images,
        captureSourceInfo: backendSettings.capture_source_info,
        retentionDays: backendSettings.retention_days,
        encryptStorage: backendSettings.encrypt_storage,
        enableMonitoring: backendSettings.enable_monitoring !== undefined 
                        ? backendSettings.enable_monitoring 
                        : true,
        enableAppWhitelist: backendSettings.enable_app_whitelist || false,
        whitelistApps: backendSettings.whitelist_apps || [],
        shortcut: clipboardSettings.value.shortcut // 保留当前快捷键设置
      }

      // 同步到UI存储
      updateClipboardSettings()
    }
    
    // 加载当前快捷键设置
    const shortcutConfig = await invoke<any>('get_global_shortcut_config')
    if (shortcutConfig) {
      clipboardSettings.value.shortcut = {
        modifiers: shortcutConfig.modifiers || ['meta', 'shift'],
        key: shortcutConfig.key || 'c'
      }
    }
  } catch (error) {
    console.error('加载设置失败:', error)
  }
}

// 更新剪贴板设置
function updateClipboardSettings() {
  uiStore.setClipboardSettings(clipboardSettings.value)
  syncClipboardSettingsToBackend()
}

// 自动启动设置
const autoStartEnabled = ref(false)

// 精确字体大小相关方法
function updateExactFontSize() {
  // 确保值是数值类型
  const size = Number(exactFontSize.value)
  if (!isNaN(size) && size >= 10 && size <= 24) {
    uiStore.setExactFontSize(size)
  }
}

function increaseExactFontSize() {
  if (exactFontSize.value < 24) {
    exactFontSize.value++
    updateExactFontSize()
  }
}

function decreaseExactFontSize() {
  if (exactFontSize.value > 10) {
    exactFontSize.value--
    updateExactFontSize()
  }
}

// 更改主题
function changeTheme(theme: string) {
  selectedTheme.value = theme
  uiStore.setTheme(theme)
}

// 重置UI设置
function resetUISettings() {
  uiStore.resetToDefaults()
  selectedTheme.value = uiStore.settings.theme
  // selectedFontSize.value = uiStore.settings.fontSize
  // selectedComponentSize.value = uiStore.settings.componentSize
  exactFontSize.value = uiStore.settings.exactFontSize
  exactSidebarWidth.value = uiStore.settings.sidebarWidth
  exactListWidth.value = uiStore.settings.listWidth
}

// 返回上一页
function goBack() {
  window.history.back()
}

// 代理设置
interface ProxySettings {
  enabled: boolean
  type: string
  host: string
  port: number
  auth: boolean
  username: string
  password: string
}

const proxySettings = ref<ProxySettings>({
  enabled: false,
  type: 'http',
  host: '127.0.0.1',
  port: 10809,
  auth: false,
  username: '',
  password: ''
})

const isTestingProxy = ref(false)

// 更新onActivated支持
onActivated(async () => {
  // 检查URL参数，自动切换到指定页面
  const route = router.currentRoute.value
  if (route.query.page && typeof route.query.page === 'string') {
    const targetPage = route.query.page
    if (settingsPages.some(page => page.id === targetPage)) {
      currentPage.value = targetPage
    }
  }

  if (!uiStore.settingsLoaded) {
    try {
      // 获取代理设置
      const proxyData = await invoke<ProxySettings>('get_proxy_settings')
      if (proxyData) {
        proxySettings.value = proxyData
      }

      // 获取自动启动状态
      autoStartEnabled.value = await isEnabled()

      // 加载剪贴板设置
      await loadClipboardSettingsFromBackend()
      
      // 加载自定义模型列表
      await loadCustomModels()
      
      // 加载当前数据库路径
      await loadCurrentDatabasePath()
      
      // 标记设置已加载
      uiStore.settingsLoaded = true
    } catch (error) {
      console.error('获取设置失败:', error)
    }
  }
})

// 初始加载
onMounted(async () => {
  // 检查URL参数，自动切换到指定页面
  const route = router.currentRoute.value
  if (route.query.page && typeof route.query.page === 'string') {
    const targetPage = route.query.page
    if (settingsPages.some(page => page.id === targetPage)) {
      currentPage.value = targetPage
    }
  }

  // 如果设置尚未加载，则加载设置
  if (!uiStore.settingsLoaded) {
    try {
      // 获取代理设置
      const proxyData = await invoke<ProxySettings>('get_proxy_settings')
      if (proxyData) {
        proxySettings.value = proxyData
      }

      // 获取自动启动状态
      autoStartEnabled.value = await isEnabled()

      // 加载剪贴板设置
      await loadClipboardSettingsFromBackend()
      
      // 标记设置已加载
      uiStore.settingsLoaded = true
    } catch (error) {
      console.error('获取设置失败:', error)
    }
  }
  
  // 初始化更新设置
  checkIntervalHours.value = Math.floor(updateStore.checkInterval / (1000 * 60 * 60))
  
  // 获取当前版本
  try {
    const version = await invoke('get_current_version') as string
    updateStore.setCurrentVersion(version)
  } catch (error) {
    console.error('获取当前版本失败:', error)
  }
  
  // 加载自定义模型列表
  await loadCustomModels()
  
  // 加载默认AI模型
  await loadDefaultAIModel()
  
  // 加载当前数据库路径
  await loadCurrentDatabasePath()
})

// 保存代理设置
async function saveProxySettings() {
  try {
    await invoke('save_proxy_settings', { proxySettings: proxySettings.value })
    // message('代理设置已保存')
  } catch (error) {
    console.error('保存代理设置失败:', error)
    message('保存代理设置失败: ' + error, { title: '错误' })
  }
}

// 测试代理连接
async function testProxyConnection() {
  isTestingProxy.value = true

  try {
    // 先保存设置
    await saveProxySettings()

    // 测试连接
    const result = await invoke<string>('test_proxy_connection', {
      proxySettings: proxySettings.value
    })

    message(result, { title: '连接测试' })
  } catch (error) {
    console.error('代理测试失败:', error)
    message('代理测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingProxy.value = false
  }
}

// 备份数据库
async function backupDatabase() {
  try {
    const result = await invoke<string>('backup_database')
    message(result, { title: '备份成功' })
  } catch (error) {
    console.error('备份数据库失败:', error)
    message('备份数据库失败: ' + error, { title: '错误' })
  }
}

// 恢复数据库
async function restoreDatabase() {
  try {
    const result = await invoke<string>('restore_database')
    message(result, { title: '恢复成功' })
  } catch (error) {
    console.error('恢复数据库失败:', error)
    message('恢复数据库失败: ' + error, { title: '错误' })
  }
}

// 导出为Markdown
async function exportAsMarkdown() {
  try {
    const result = await invoke<string>('export_as_markdown')
    message(result, { title: '导出成功' })
  } catch (error) {
    console.error('导出失败:', error)
    message('导出失败: ' + error, { title: '错误' })
  }
}

// 切换自动启动
async function toggleAutoStart() {
  if (autoStartEnabled.value) {
    autoStartEnabled.value = true
    await enable();
  } else {
    autoStartEnabled.value = false
    await disable();
  }
}

// 监听代理设置变化
watch(proxySettings, async () => {
  await saveProxySettings()
}, { deep: true })

const defaultAIModel = ref('chatgpt')
async function saveDefaultAIModel() {
  try {
    const providerId = defaultAIModel.value;
    const providerConfig = aiProviders.value[providerId];
    
    if (!providerConfig) {
      console.error(`Provider config not found for: ${providerId}`);
      message(`未找到提供商配置: ${providerId}`, { title: '错误' });
      return;
    }

    const providerForBackend = providerMapping[providerId];
    const modelName = providerConfig.default_model;
    
    if (!providerForBackend || !modelName) {
        console.error('Provider or model name is missing for the selected default AI model.');
        message('提供商或模型名称丢失', { title: '错误' });
        return;
    }
    
    // 调用后端将全局默认模型存入数据库
    await invoke('set_default_ai_model', {
      modelType: 'chat',
      provider: providerForBackend,
      modelName: modelName
    });
    
    // 同时保存到localStorage，用于快速加载
    localStorage.setItem('defaultAIModel', providerId);
    
    message('默认AI模型已保存', { title: '成功' });

  } catch (error) {
    console.error('保存默认AI模型失败:', error);
    message('保存默认AI模型失败: ' + error, { title: '错误' });
  }
}

// 加载默认AI模型
async function loadDefaultAIModel() {
  try {
    // 先尝试从新API获取默认模型
    try {
      const defaultModel = await invoke('get_default_ai_model', { modelType: 'chat' }) as any
      if (defaultModel && defaultModel.provider && defaultModel.name) {
        // 找到对应的本地模型ID
        for (const [id, provider] of Object.entries(providerMapping)) {
          if (provider === defaultModel.provider) {
            defaultAIModel.value = id
            break
          }
        }
        console.log('从API加载了默认AI模型:', defaultAIModel.value)
        return
      }
    } catch (error) {
      console.warn('从API获取默认AI模型失败，尝试使用localStorage:', error)
    }

    // 回退到localStorage
    const model = localStorage.getItem('defaultAIModel')
    if (model) {
      defaultAIModel.value = model
    }
  } catch (error) {
    console.error('获取默认AI模型失败:', error)
  }
}

// 新增手动清理过期剪贴板条目的方法
const isCleaningEntries = ref(false)

async function cleanExpiredEntries() {
  isCleaningEntries.value = true
  try {
    await invoke('clean_expired_clipboard_entries')
    message('过期剪贴板条目已清理', { title: '清理成功' })
  } catch (error) {
    console.error('清理过期剪贴板条目失败:', error)
    message('清理过期剪贴板条目失败: ' + error, { title: '错误' })
  } finally {
    isCleaningEntries.value = false
  }
}

// 新增迁移配置到数据库的方法
async function migrateConfigToDatabase() {
  try {
    const result = await invoke<string>('migrate_config_to_database')
    message(result, { title: '迁移成功' })
  } catch (error) {
    console.error('迁移配置到数据库失败:', error)
    message('迁移配置到数据库失败: ' + error, { title: '错误' })
  }
}

// 新增模型配置管理功能
const selectedConfigModel = ref('openai')


const editingModelList = ref<Array<{ name: string, description: string }>>([])
const editingModelType = ref('')
const newModelName = ref('')
const newModelDescription = ref('')
const isSavingApiConfig = ref(false)
const isTestingApi = ref(false)
const modelConfigModal = ref<HTMLDialogElement | null>(null)

// 可用的AI模型
const availableModels = [
  { id: 'chatgpt', name: 'OpenAI ChatGPT' },
  { id: 'gemini', name: 'Google Gemini' },
  { id: 'deepseek', name: 'DeepSeek' },
  { id: 'qwen', name: '阿里通义千问' },
  { id: 'claude', name: 'Anthropic Claude' },
  { id: 'doubao', name: '字节豆包' },
  { id: 'grok', name: 'xAI Grok' },
]

// 新增AI模型和提供商映射关系
const providerMapping: Record<string, string> = {
  'chatgpt': 'openai',
  'gemini': 'gemini',
  'deepseek': 'deepseek',
  'qwen': 'ali',
  'claude': 'anthropic',
  'doubao': 'doubao',
  'grok': 'xai',
  'custom': 'custom'
}

// 新增AI配置状态
const aiProviders = ref<Record<string, AIProvider>>(structuredClone(defaultProviders))
const aiModels = ref<AIModel[]>([])
const isLoadingModels = ref(false)
const aiStats = ref({
  conversations: 0,
  messages: 0,
  tokens: {
    input: 0,
    output: 0,
    total: 0
  },
  providers: {}
})

// 默认的各AI模型的常用模型名称数据
const defaultModelSuggestions = {
  chatgpt: [
    { name: 'gpt-4o', description: 'GPT-4o - 最新版本，支持文本和图像' },
    { name: 'gpt-4o-mini', description: 'GPT-4o Mini - 轻量级版本，成本更低' },
    { name: 'gpt-4-turbo', description: 'GPT-4 Turbo - 高性能版本' },
    { name: 'gpt-4', description: 'GPT-4 - 标准版本' },
    { name: 'gpt-3.5-turbo', description: 'GPT-3.5 Turbo - 经典版本，性价比高' }
  ],
  gemini: [
    { name: 'gemini-2.0-flash', description: 'Gemini 2.0 Flash - 最新版本' },
    { name: 'gemini-1.5-flash', description: 'Gemini 1.5 Flash - 快速响应版本' },
    { name: 'gemini-1.5-pro', description: 'Gemini 1.5 Pro - 专业版本' },
    { name: 'gemini-pro', description: 'Gemini Pro - 标准专业版' }
  ],
  deepseek: [
    { name: 'deepseek-chat', description: 'DeepSeek Chat - 对话模型' },
    { name: 'deepseek-reasoner', description: 'DeepSeek reasoner - 深度思考模型' },
  ],
  claude: [
  { name: 'claude-sonnet-4-20250514', description: 'claude-sonnet-4-20250514 - 最新版本' },
  { name: 'claude-opus-4-20250514t', description: 'claude-opus-4-20250514 - 最新版本' },
  { name: 'claude-3-7-sonnet-20250219', description: 'claude-3-7-sonnet-20250219' },
  { name: 'claude-3-5-sonnet-20241022', description: 'claude-3-5-sonnet-20241022' },
  ],
  qwen: [
    { name: 'qwen-max', description: '通义千问Max - 最强版本' },
    { name: 'qwen-plus', description: '通义千问Plus - 增强版本' },
    { name: 'qwen-turbo', description: '通义千问Turbo - 快速版本' },
    { name: 'qwen-long', description: '通义千问Long - 长文本版本' }
  ],
  doubao: [
    { name: 'doubao-seed-1.6', description: '全新多模态深度思考模型，同时支持 thinking、non-thinking、auto三种思考模式。其中 non-thinking 模型对比 doubao-1.5-pro-32k-250115 模型大幅提升。' },
    { name: 'doubao-seed-1.6-flash', description: '有极致推理速度的多模态深度思考模型；同时支持文本和视觉理解。文本理解能力超过上一代 Lite 系列模型，视觉理解比肩友商 Pro 系列模型。' },
    { name: 'doubao-seed-1.6-thinking', description: '在思考能力上进行了大幅强化， 对比 doubao 1.5 代深度理解模型，在编程、数学、逻辑推理等基础能力上进一步提升， 支持视觉理解。' }
  ],
  grok: [
    { name: 'grok-3', description: 'Grok 3 - 最新版本' },
    { name: 'grok-3-mini', description: 'Grok 3 Mini - 轻量级版本' },
    { name: 'grok-1.5', description: 'Grok 1.5 - 增强版本' }
  ],
  custom: [
    { name: 'gpt-3.5-turbo', description: 'OpenAI兼容 - GPT-3.5 Turbo' },
    { name: 'gpt-4', description: 'OpenAI兼容 - GPT-4' },
    { name: 'claude-3-sonnet', description: 'Claude兼容 - Sonnet' }
  ]
}

// 可配置的模型建议数据
const modelSuggestions = ref<typeof defaultModelSuggestions>({ ...defaultModelSuggestions })

function getModelNameById(modelId: string): string {
  const model = availableModels.find(m => m.id === modelId)
  return model ? model.name : '未知模型'
}





async function testApiConnection(providerId: string): Promise<void> {
  if (!aiProviders.value[providerId]) return
  
  const provider = aiProviders.value[providerId]
  
  // 对于非自定义模型，至少需要API Key
  if (providerId !== 'custom' && !provider.api_key) {
    message('请输入API密钥后再测试', { title: '提示' })
    return
  }
  
  // 对于自定义模型，至少需要Endpoint和Model Name
  if (providerId === 'custom' && (!provider.api_base || !provider.default_model)) {
    message('请输入API端点和模型标识符后再测试', { title: '提示' })
    return
  }
  
  const request: TestConnectionRequest = {
    provider: provider.provider,
    api_key: provider.api_key,
    api_base: provider.api_base,
    model: provider.default_model
  }
  
  isTestingApi.value = true
  try {
    const response = await testAIConnection(request)
    
    if (response.success) {
      message(response.message, { title: '连接成功' })
      
      // 如果返回了模型列表，更新提供商的模型列表
      if (response.models && response.models.length > 0) {
        provider.models = response.models.map(name => ({
          name,
          provider: providerId
        }))
      }
    } else {
      message(response.message, { title: '连接失败' })
    }
  } catch (error) {
    console.error('API连接测试失败:', error)
    message('API连接测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingApi.value = false
  }
}



function closeModelConfigModal(): void {
  editingModelType.value = ''
  editingModelList.value = []
  newModelName.value = ''
  newModelDescription.value = ''
  modelConfigModal.value?.close()
}

function resetToDefault(): void {
  if (editingModelType.value && defaultModelSuggestions[editingModelType.value as keyof typeof defaultModelSuggestions]) {
    editingModelList.value = [...defaultModelSuggestions[editingModelType.value as keyof typeof defaultModelSuggestions]]
  }
}

async function saveModelConfig(): Promise<void> {
  if (editingModelType.value) {
    modelSuggestions.value[editingModelType.value as keyof typeof modelSuggestions.value] = [...editingModelList.value]
    try {
      localStorage.setItem('ai-model-suggestions-config', JSON.stringify(modelSuggestions.value))
      message('模型配置保存成功', { title: '成功' })
      closeModelConfigModal()
    } catch (error) {
      console.error('保存模型配置失败:', error)
      message('保存模型配置失败: ' + error, { title: '错误' })
    }
  }
}


function addNewModel(): void {
  if (!newModelName.value.trim() || !newModelDescription.value.trim()) return

  editingModelList.value.push({
    name: newModelName.value.trim(),
    description: newModelDescription.value.trim()
  })

  newModelName.value = ''
  newModelDescription.value = ''
}

function removeModel(index: number): void {
  editingModelList.value.splice(index, 1)
}

// 更新相关函数
function formatLastCheckTime(): string {
  if (!updateStore.lastCheckTime) return '从未检查'
  const now = new Date()
  const diff = now.getTime() - updateStore.lastCheckTime.getTime()
  const minutes = Math.floor(diff / (1000 * 60))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  
  const days = Math.floor(hours / 24)
  return `${days}天前`
}

function formatDate(dateString?: string): string {
  if (!dateString) return '未知'
  
  try {
    // 尝试解析日期字符串
    const date = new Date(dateString)
    
    // 检查日期是否有效
    if (isNaN(date.getTime())) {
      console.warn('无效的日期格式:', dateString)
      return dateString // 如果无法解析，返回原始字符串
    }
    
    // 返回本地化的日期格式
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit'
    })
  } catch (error) {
    console.error('日期格式化错误:', error, '原始日期:', dateString)
    return dateString // 出错时返回原始字符串
  }
}

function formatReleaseNotes(notes: string): string {
  // 简单的 Markdown 格式化
  return notes
    .replace(/\n/g, '<br>')
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
    .replace(/#{1,6}\s*(.*)/g, '<strong>$1</strong>')
}

async function checkForUpdates(): Promise<void> {
  if (updateStore.isChecking) return
  
  updateStore.setChecking(true)
  
  try {
    const updateResult = await invoke('check_for_updates_with_config', {
      timeoutSeconds: 30,
      proxy: null
    }) as any
    
    if (updateResult.available) {
      updateStore.setUpdateInfo({
        version: updateResult.version,
        pub_date: updateResult.pub_date || '',
        body: updateResult.body || '',
        available: true
      })
      message(`发现新版本 ${updateResult.version}！`, { title: '更新检查' })
    } else {
      updateStore.setUpdateInfo(null)
      message('当前已是最新版本！', { title: '更新检查' })
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setUpdateInfo(null)
    message('检查更新失败: ' + error, { title: '错误' })
  } finally {
    updateStore.setChecking(false)
  }
}

// 无签名验证的更新检查
async function checkForUpdatesNoSignature(): Promise<void> {
  if (updateStore.isChecking) return
  
  updateStore.setChecking(true)
  
  try {
    const updateResult = await invoke('check_for_updates_no_signature', {
      timeoutSeconds: 30,
      proxy: null
    }) as any
    
    if (updateResult.available) {
      updateStore.setUpdateInfo({
        version: updateResult.version,
        pub_date: updateResult.pub_date || '',
        body: updateResult.body || '',
        available: true
      })
      message(`发现新版本 ${updateResult.version}！（无签名验证模式）`, { title: '更新检查' })
    } else {
      updateStore.setUpdateInfo(null)
      message('当前已是最新版本！（无签名验证模式）', { title: '更新检查' })
    }
  } catch (error) {
    console.error('检查更新失败:', error)
    updateStore.setUpdateInfo(null)
    message('检查更新失败: ' + error, { title: '错误' })
  } finally {
    updateStore.setChecking(false)
  }
}

function updateAutoCheckSetting(): void {
  // 这里可以添加保存到本地存储的逻辑
  console.log('自动检查设置更新:', updateStore.autoCheck)
}

function updateCheckInterval(): void {
  const hours = parseInt(checkIntervalHours.value.toString())
  const milliseconds = hours * 60 * 60 * 1000
  updateStore.setAutoCheck(updateStore.autoCheck, milliseconds)
  console.log('检查间隔更新:', hours, '小时')
}

// Windows 更新测试函数
async function testWindowsUpdate(): Promise<void> {
  if (isTestingWindowsUpdate.value) return
  
  isTestingWindowsUpdate.value = true
  
  try {
    const result = await invoke('test_windows_update_with_proxy') as string
    message(result, { title: 'Windows 更新测试' })
  } catch (error) {
    console.error('Windows 更新测试失败:', error)
    message('Windows 更新测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingWindowsUpdate.value = false
  }
}

// Windows 更新网络诊断测试函数
async function testWindowsUpdateNoSignature(): Promise<void> {
  if (isTestingWindowsUpdateNoSig.value) return
  
  isTestingWindowsUpdateNoSig.value = true
  
  try {
    const result = await invoke('test_windows_update_no_signature') as string
    message(result, { title: '网络诊断测试' })
  } catch (error) {
    console.error('网络诊断测试失败:', error)
    message('网络诊断测试失败: ' + error, { title: '错误' })
  } finally {
    isTestingWindowsUpdateNoSig.value = false
  }
}

// 显示平台信息函数
async function showPlatformInfo(): Promise<void> {
  try {
    const platformInfo = await invoke('get_platform_info') as string
    message(platformInfo, { title: '平台信息' })
  } catch (error) {
    console.error('获取平台信息失败:', error)
    message('获取平台信息失败: ' + error, { title: '错误' })
  }
}

// 白名单应用管理方法
function addWhitelistApp() {
  const appName = newWhitelistApp.value.trim()
  if (!appName) return
  
  // 检查是否已存在
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    message(`应用 "${appName}" 已在白名单中`, { title: '提示' })
    return
  }
  
  // 添加到白名单
  clipboardSettings.value.whitelistApps.push(appName)
  newWhitelistApp.value = ''
  updateClipboardSettings()
  
  message(`已添加 "${appName}" 到白名单`, { title: '成功' })
}

function removeWhitelistApp(index: number) {
  const appName = clipboardSettings.value.whitelistApps[index]
  clipboardSettings.value.whitelistApps.splice(index, 1)
  updateClipboardSettings()
  
  message(`已从白名单中移除 "${appName}"`, { title: '成功' })
}

function addPresetApp(appName: string) {
  if (clipboardSettings.value.whitelistApps.includes(appName)) {
    return
  }
  
  clipboardSettings.value.whitelistApps.push(appName)
  updateClipboardSettings()
  
  message(`已添加 "${appName}" 到白名单`, { title: '成功' })
}

// 新增自定义模型配置管理功能
const customModels = ref<Array<{
  id: string
  name: string
  endpoint: string
  model_name: string
  adapter_type: string
  api_key?: string
}>>([])

const customModelModal = ref<HTMLDialogElement | null>(null)
const editingCustomModelId = ref('')
const showCustomModelApiKey = ref(false)
const isSavingCustomModel = ref(false)
const isTestingCustomModel = ref(false)

// 数据库路径管理相关变量
const currentDatabasePath = ref('')
const isChangingDatabase = ref(false)
const databaseInfo = ref<{
  size: string
  noteCount: number
  categoryCount: number
  lastModified: string
} | null>(null)

const customModelForm = ref({
  name: '',
  endpoint: '',
  model_name: '',
  api_key: '',
  adapter_type: 'openai',
  custom_headers: [] as Array<{ key: string, value: string }>
})

// 计算属性
const canSaveCustomModel = computed(() => {
  return customModelForm.value.name.trim() &&
         customModelForm.value.endpoint.trim() &&
         customModelForm.value.model_name.trim()
         // 移除API密钥的必填验证，允许为空
})

const canTestCustomModel = computed(() => {
  return customModelForm.value.endpoint.trim() &&
         customModelForm.value.model_name.trim()
         // 测试连接时也不要求API密钥
})

// 加载自定义模型列表
async function loadCustomModels(): Promise<void> {
  try {
    const models = await invoke('list_custom_model_configs')
    customModels.value = models as Array<{
      id: string
      name: string
      endpoint: string
      model_name: string
      adapter_type: string
    }>
  } catch (error) {
    console.error('加载自定义模型失败:', error)
    customModels.value = []
  }
}



// 关闭自定义模型配置模态框
function closeCustomModelModal(): void {
  editingCustomModelId.value = ''
  customModelForm.value = {
    name: '',
    endpoint: '',
    model_name: '',
    api_key: '',
    adapter_type: 'openai',
    custom_headers: []
  }
  customModelModal.value?.close()
}

// 保存自定义模型配置
async function saveCustomModel(): Promise<void> {
  if (!canSaveCustomModel.value) return
  
  isSavingCustomModel.value = true
  try {
    const configId = editingCustomModelId.value || `custom_${Date.now()}`
    
    // 转换自定义头部为对象格式
    const customHeaders = customModelForm.value.custom_headers
      .filter(h => h.key.trim() && h.value.trim())
      .reduce((acc, h) => {
        acc[h.key.trim()] = h.value.trim()
        return acc
      }, {} as Record<string, string>)

    await invoke('save_custom_model_config', {
      configId,
      name: customModelForm.value.name.trim(),
      endpoint: customModelForm.value.endpoint.trim(),
      modelName: customModelForm.value.model_name.trim(),
      apiKey: customModelForm.value.api_key.trim(),
      adapterType: customModelForm.value.adapter_type,
      customHeaders: Object.keys(customHeaders).length > 0 ? customHeaders : null
    })

    message(editingCustomModelId.value ? '自定义模型配置更新成功' : '自定义模型配置添加成功', { title: '成功' })
    closeCustomModelModal()
    await loadCustomModels()
  } catch (error) {
    console.error('保存自定义模型配置失败:', error)
    message('保存自定义模型配置失败: ' + error, { title: '错误' })
  } finally {
    isSavingCustomModel.value = false
  }
}



// 测试自定义模型连接
async function testCustomModelConnection(): Promise<void> {
  if (!canTestCustomModel.value) return
  
  isTestingCustomModel.value = true
  try {
    // 转换自定义头部为对象格式
    const customHeaders = customModelForm.value.custom_headers
      .filter(h => h.key.trim() && h.value.trim())
      .reduce((acc, h) => {
        acc[h.key.trim()] = h.value.trim()
        return acc
      }, {} as Record<string, string>)

    // 调用后端测试API
    const result = await invoke('test_custom_model_connection', {
      endpoint: customModelForm.value.endpoint.trim(),
      modelName: customModelForm.value.model_name.trim(),
      apiKey: customModelForm.value.api_key.trim(),
      adapterType: customModelForm.value.adapter_type,
      customHeaders: Object.keys(customHeaders).length > 0 ? customHeaders : null
    })

    message(result as string, { title: '测试结果' })
    
  } catch (error) {
    console.error('测试自定义模型连接失败:', error)
    message('测试连接失败: ' + error, { title: '错误' })
  } finally {
    isTestingCustomModel.value = false
  }
}

// 添加自定义头部
function addCustomHeader(): void {
  customModelForm.value.custom_headers.push({ key: '', value: '' })
}

// 移除自定义头部
function removeCustomHeader(index: number): void {
  customModelForm.value.custom_headers.splice(index, 1)
}

// 数据库路径管理相关方法
async function loadCurrentDatabasePath(): Promise<void> {
  try {
    const path = await invoke('get_current_database_path') as string
    currentDatabasePath.value = path
    await loadDatabaseInfo()
  } catch (error) {
    console.error('获取当前数据库路径失败:', error)
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
    console.error('获取数据库信息失败:', error)
    databaseInfo.value = null
  }
}

async function copyDatabasePath(): Promise<void> {
  try {
    await navigator.clipboard.writeText(currentDatabasePath.value)
    message('数据库路径已复制到剪贴板', { title: '成功' })
  } catch (error) {
    console.error('复制路径失败:', error)
    message('复制路径失败', { title: '错误' })
  }
}

async function selectDatabaseFile(): Promise<void> {
  if (isChangingDatabase.value) return
  
  isChangingDatabase.value = true
  try {
    const result = await invoke('select_database_file') as { path: string | null, restart_required: boolean }
    
    if (result.path) {
      currentDatabasePath.value = result.path
      await loadDatabaseInfo()
      
      if (result.restart_required) {
        message('数据库文件已更改，请重启应用以使更改生效', { title: '需要重启' })
      } else {
        message('数据库文件已更改', { title: '成功' })
      }
    }
  } catch (error) {
    console.error('选择数据库文件失败:', error)
    message('选择数据库文件失败: ' + error, { title: '错误' })
  } finally {
    isChangingDatabase.value = false
  }
}

async function createNewDatabase(): Promise<void> {
  if (isChangingDatabase.value) return
  
  isChangingDatabase.value = true
  try {
    const result = await invoke('create_new_database') as { path: string | null, restart_required: boolean }
    
    if (result.path) {
      currentDatabasePath.value = result.path
      await loadDatabaseInfo()
      
      if (result.restart_required) {
        message('新数据库已创建，请重启应用以使更改生效', { title: '需要重启' })
      } else {
        message('新数据库已创建', { title: '成功' })
      }
    }
  } catch (error) {
    console.error('创建新数据库失败:', error)
    message('创建新数据库失败: ' + error, { title: '错误' })
  } finally {
    isChangingDatabase.value = false
  }
}

async function resetToDefaultDatabase(): Promise<void> {
  if (isChangingDatabase.value) return
  
  const confirmed = await showConfirm('确定要重置到默认数据库位置吗？这将需要重启应用。', {
    title: '确认重置',
    confirmText: '重置',
    cancelText: '取消'
  })
  
  if (!confirmed) {
    return
  }
  
  isChangingDatabase.value = true
  try {
    const result = await invoke('reset_to_default_database') as { path: string, restart_required: boolean }
    
    currentDatabasePath.value = result.path
    await loadDatabaseInfo()
    
    if (result.restart_required) {
      message('已重置到默认数据库位置，请重启应用以使更改生效', { title: '需要重启' })
    } else {
      message('已重置到默认数据库位置', { title: '成功' })
    }
  } catch (error) {
    console.error('重置数据库位置失败:', error)
    message('重置数据库位置失败: ' + error, { title: '错误' })
  } finally {
    isChangingDatabase.value = false
  }
}

// 加载AI提供商配置
async function loadAIProvidersConfig() {
  isLoadingModels.value = true
  try {
    // 一次性获取所有AI提供商的配置
    const config = await getAIConfig();
    if (config && config.providers) {
      // 合并数据库中的配置和前端的默认配置
      for (const providerId in defaultProviders) {
        if (config.providers[providerId]) {
          aiProviders.value[providerId] = {
            ...defaultProviders[providerId],
            ...config.providers[providerId],
          };
        }
      }
    }

    // 加载聊天模型列表
    const models = await getChatModels()
    aiModels.value = models
    
    // 加载默认AI模型
    const defaultModel = await getDefaultAIModel('chat')
    if (defaultModel) {
      // 从返回的 provider 'openai' 找到对应的本地id 'chatgpt'
      for (const key in providerMapping) {
        if (providerMapping[key] === defaultModel.provider) {
          defaultAIModel.value = key;
          break;
        }
      }
    }
    
    // 获取服务状态 (可以考虑也合并到get_ai_config中)
    const statuses = await getAIServiceStatus()
    for (const status of statuses) {
      if (aiProviders.value[status.provider]) {
        aiProviders.value[status.provider].enabled = status.is_available
      }
    }
  } catch (error) {
    console.error('加载AI配置失败:', error)
    message('加载AI配置失败: ' + error, { title: '错误' })
  } finally {
    isLoadingModels.value = false
  }
}

// 刷新AI使用统计
async function refreshAIStats() {
  try {
    const stats = await getAIUsageStats()
    aiStats.value = stats
  } catch (error) {
    console.error('获取AI使用统计失败:', error)
    message('获取AI使用统计失败: ' + error, { title: '错误' })
  }
}

// 初始化时加载AI配置
onMounted(async () => {
  // 获取代理设置
  try {
    const settings = await invoke('get_proxy_settings')
    if (settings) {
      proxySettings.value = settings as ProxySettings
    }
  } catch (error) {
    console.error('获取代理设置失败:', error)
  }

  // 加载默认AI模型
  await loadDefaultAIModel()

  // 加载AI提供商配置
  await loadAIProvidersConfig()
  
  // 加载AI使用统计
  await refreshAIStats()
  
  // 加载自定义模型列表
  await loadCustomModels()

  // 检查是否启用了自动启动
  try {
    const enabled = await isEnabled()
    autoStartEnabled.value = enabled
  } catch (error) {
    console.error('检查自动启动状态失败:', error)
  }
})

// 保存AI配置
async function saveAIProviderConfig() {
  isSavingApiConfig.value = true
  try {
    // 保存所有提供商配置
    await saveAIConfig(aiProviders.value)
    
    message('AI配置已保存', { title: '成功' })
    
    // 重新加载AI服务
    await reloadAIServices()
  } catch (error) {
    console.error('保存AI配置失败:', error)
    message('保存AI配置失败: ' + error, { title: '错误' })
  } finally {
    isSavingApiConfig.value = false
  }
}

// ---------------- 提示词模板管理 ----------------
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

<style scoped>
/* Settings页面特有的样式 - 大部分暗色主题样式已移至全局 */

/* 导航按钮优化样式 */
.nav-button {
  position: relative;
  overflow: hidden;
  border: 1px solid transparent;
  backdrop-filter: blur(10px);
}

.nav-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left 0.5s ease;
}

.nav-button:hover::before {
  left: 100%;
}

/* 活跃状态的按钮样式 - 确保跟随主题色 */
.nav-button.bg-primary {
  background: hsl(var(--p)) !important;
  color: hsl(var(--pc)) !important;
  border-color: hsl(var(--p)) !important;
  box-shadow: 0 4px 12px hsl(var(--p) / 0.3), 0 0 0 1px hsl(var(--pc) / 0.1) inset;
}

/* 确保选中状态的图标和文字颜色正确 */
.nav-button.bg-primary .flex-shrink-0,
.nav-button.bg-primary span {
  color: hsl(var(--pc)) !important;
}

/* 悬停状态优化 */
.nav-button:hover {
  transform: translateX(2px);
  border-color: hsl(var(--p) / 0.3);
}

.nav-button:not(.bg-primary):hover {
  background: hsl(var(--b2));
  color: hsl(var(--p));
}

/* 图标动画 */
.nav-button .flex-shrink-0 {
  transition: transform 0.2s ease, color 0.2s ease;
}

.nav-button:hover .flex-shrink-0 {
  transform: scale(1.1);
}

/* 导航栏整体样式 */
nav {
  background: hsl(var(--b1));
  backdrop-filter: blur(10px);
}

/* 设置选项标题样式优化 */
nav h2 {
  color: hsl(var(--bc)) !important;
  font-weight: 600;
  opacity: 0.9;
}

/* 底部装饰文字样式优化 */
nav .mt-auto .text-xs {
  color: hsl(var(--bc) / 0.6) !important;
}

/* 特殊的设置页面布局优化 */
.settings-nav-button {
  transition: all 0.2s ease;
}

.settings-nav-button:hover {
  transform: translateX(2px);
}

/* 设置预览区域的特殊样式 */
.theme-preview-grid {
  transition: all 0.3s ease;
}

/* 字体预览区域的特殊样式 */
.font-preview-sample {
  transition: font-size 0.3s ease, color 0.3s ease;
}

/* 布局预览的特殊间距 */
.layout-preview-container {
  min-height: 120px;
}

/* Range滑块的特殊样式 - 只在设置页面需要 */
[data-theme="dark"] .range-primary::-webkit-slider-thumb,
[data-theme="night"] .range-primary::-webkit-slider-thumb,
[data-theme="black"] .range-primary::-webkit-slider-thumb {
  border: 2px solid rgba(255, 255, 255, 0.4);
}

[data-theme="dark"] .range-primary::-moz-range-thumb,
[data-theme="night"] .range-primary::-moz-range-thumb,
[data-theme="black"] .range-primary::-moz-range-thumb {
  border: 2px solid rgba(255, 255, 255, 0.4);
}

/* 设置卡片的特殊阴影效果 */
[data-theme="dark"] .settings-card:hover,
[data-theme="night"] .settings-card:hover,
[data-theme="black"] .settings-card:hover {
  box-shadow: 0 8px 25px rgba(255, 255, 255, 0.05);
}

/* 快捷键显示的特殊样式 */
.shortcut-display {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  letter-spacing: 0.5px;
}

/* 主题选择标签的特殊激活状态 */
.theme-selector-active {
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
}

[data-theme="dark"] .theme-selector-active,
[data-theme="night"] .theme-selector-active,
[data-theme="black"] .theme-selector-active {
  box-shadow: 0 0 0 2px rgba(147, 197, 253, 0.5);
}
</style>
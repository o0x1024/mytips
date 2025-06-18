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
        <h1 class="text-lg font-medium ml-2">设置</h1>
      </div>
    </div>

    <!-- 左侧完整侧边导航栏 -->
    <div class="flex-1 overflow-auto p-0 flex">
      <nav
        class="w-56 min-w-[180px] max-w-[220px] h-screen bg-base-100 flex flex-col py-8 px-2 gap-1 shadow-sm sticky top-0 left-0 z-20">
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('appearance')">外观</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('clipboard')">临时笔记</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('network')">网络</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('data')">数据管理</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('app')">应用设置</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3 mb-1"
          @click="scrollToSection('ai')">AI助手</button>
        <button class="btn btn-ghost justify-start text-base rounded-none px-4 py-3"
          @click="scrollToSection('about')">关于</button>
      </nav>
      <div class="flex-1 max-w-2xl mx-auto p-6">
        <!-- 设置内容 -->
        <div class="max-w-2xl mx-auto">
          <!-- 外观设置 -->
          <div id="appearance" class="card bg-base-100 shadow-md mb-6">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">外观</h2>

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
                      <div class="p-3 rounded-lg bg-base-100 text-base-content border border-base-300 text-center">背景色
                      </div>
                      <div class="p-3 rounded-lg bg-base-200 text-base-content border border-base-300 text-center">背景色 2
                      </div>
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

              <!-- 组件大小设置 -->
              <div class="form-control mb-6">
                <label class="label">
                  <span class="label-text">界面布局</span>
                </label>

                <!-- 侧边栏宽度调整 -->
                <div class="mt-2">
                  <label class="label">
                    <span class="label-text">侧边栏宽度 ({{ exactSidebarWidth }}px)</span>
                  </label>
                  <div class="flex items-center gap-2">
                    <button class="btn btn-sm btn-circle" @click="decreaseSidebarWidth"
                      :disabled="exactSidebarWidth <= 150">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                      </svg>
                    </button>
                    <input type="range" min="150" max="400" step="5" v-model="exactSidebarWidth"
                      class="range range-primary range-sm flex-1" @input="updateExactComponentSize" />
                    <button class="btn btn-sm btn-circle" @click="increaseSidebarWidth"
                      :disabled="exactSidebarWidth >= 400">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                    </button>
                  </div>
                </div>

                <!-- 列表宽度调整 -->
                <div class="mt-4">
                  <label class="label">
                    <span class="label-text">列表宽度 ({{ exactListWidth }}px)</span>
                  </label>
                  <div class="flex items-center gap-2">
                    <button class="btn btn-sm btn-circle" @click="decreaseListWidth" :disabled="exactListWidth <= 200">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                      </svg>
                    </button>
                    <input type="range" min="200" max="500" step="5" v-model="exactListWidth"
                      class="range range-primary range-sm flex-1" @input="updateExactComponentSize" />
                    <button class="btn btn-sm btn-circle" @click="increaseListWidth" :disabled="exactListWidth >= 500">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                    </button>
                  </div>
                </div>

                <!-- 预设布局按钮 -->
                <div class="mt-4 flex flex-wrap gap-2">
                  <button class="btn btn-sm" :class="{ 'btn-primary': uiStore.settings.componentSize === 'compact' }"
                    @click="applyPresetLayout('compact')">
                    紧凑
                  </button>
                  <button class="btn btn-sm" :class="{ 'btn-primary': uiStore.settings.componentSize === 'medium' }"
                    @click="applyPresetLayout('medium')">
                    中等
                  </button>
                  <button class="btn btn-sm" :class="{ 'btn-primary': uiStore.settings.componentSize === 'comfortable' }"
                    @click="applyPresetLayout('comfortable')">
                    宽松
                  </button>
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
          <div id="clipboard" class="card bg-base-100 shadow-md mb-6">
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

              <!-- 忽略敏感内容设置 -->
              <div class="form-control mb-4">
                <label class="label cursor-pointer justify-start gap-4">
                  <span class="label-text">忽略敏感内容</span>
                  <input type="checkbox" class="toggle toggle-primary"
                    v-model="clipboardSettings.ignoreSensitiveContent" @change="updateClipboardSettings" />
                </label>
                <p class="text-xs text-base-content/80 mt-1">
                  启用后，将自动忽略从密码框等敏感区域复制的内容
                </p>
              </div>

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
          <div id="network" class="card bg-base-100 shadow-md mb-6">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">网络</h2>

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
          <div id="data" class="card bg-base-100 shadow-md mb-6">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">数据管理</h2>

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
          <div id="app" class="card bg-base-100 shadow-md mb-6">
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

          <!-- 在外观设置下方新增AI默认模型设置 -->
          <div id="ai" class="card bg-base-100 shadow-md mb-6">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">AI助手</h2>
              <div class="form-control mb-4">
                <label class="label">
                  <span class="label-text">全局AI默认模型</span>
                </label>
                <select v-model="defaultAIModel" class="select select-bordered w-full" @change="saveDefaultAIModel">
                  <option value="gemini">Gemini</option>
                  <option value="chatgpt">OpenAI ChatGPT</option>
                  <option value="deepseek">DeepSeek</option>
                  <option value="qwen">通义千问</option>
                  <option value="claude">Anthropic Claude</option>
                  <option value="doubao">字节豆包</option>
                  <option value="grok">xAI Grok</option>
                  <option value="custom">自定义API</option>
                </select>
                <p class="text-xs text-base-content/80 mt-1">设置后，AI相关功能将默认使用该模型</p>
              </div>
            </div>
          </div>

          <!-- 关于 -->
          <div id="about" class="card bg-base-100 shadow-md mb-6">
            <div class="card-body">
              <h2 class="card-title text-primary mb-4">关于</h2>
              <p>MyTips v1.0.0</p>
              <p class="text-sm text-base-content/80 mt-2">
                一个简单的笔记管理应用，帮助你收集和整理有用的代码片段、文档和提示。
              </p>
            </div>
          </div>

        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed, onActivated } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { message } from '@tauri-apps/plugin-dialog'
import { useUIStore } from '../stores/uiStore'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';

const uiStore = useUIStore()

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
  ...uiStore.settings.clipboard,
  enableMonitoring: true, // 默认启用剪贴板监听
  shortcut: {
    modifiers: ['meta', 'shift'], // 默认组合键：Command/Ctrl + Shift
    key: 'c' // 默认按键：C
  }
})

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
      enable_monitoring: clipboardSettings.value.enableMonitoring
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

// 精确组件大小相关方法
function updateExactComponentSize() {
  uiStore.setExactComponentSize(exactSidebarWidth.value, exactListWidth.value)
}

// 更改主题
function changeTheme(theme: string) {
  selectedTheme.value = theme
  uiStore.setTheme(theme)
}

function increaseSidebarWidth() {
  if (exactSidebarWidth.value < 400) {
    exactSidebarWidth.value += 5
    updateExactComponentSize()
  }
}

function decreaseSidebarWidth() {
  if (exactSidebarWidth.value > 150) {
    exactSidebarWidth.value -= 5
    updateExactComponentSize()
  }
}

function increaseListWidth() {
  if (exactListWidth.value < 500) {
    exactListWidth.value += 5
    updateExactComponentSize()
  }
}

function decreaseListWidth() {
  if (exactListWidth.value > 200) {
    exactListWidth.value -= 5
    updateExactComponentSize()
  }
}

// 应用预设布局
function applyPresetLayout(preset: string) {
  const option = uiStore.componentSizeOptions.find(opt => opt.value === preset)
  if (option) {
    exactSidebarWidth.value = option.navWidth
    exactListWidth.value = option.listWidth
    updateExactComponentSize()
  }
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
  port: 7890,
  auth: false,
  username: '',
  password: ''
})

const isTestingProxy = ref(false)

// 更新onActivated支持
onActivated(async () => {
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
})

// 初始加载
onMounted(async () => {
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

const defaultAIModel = ref(localStorage.getItem('mytips-default-ai-model') || 'gemini')
function saveDefaultAIModel() {
  localStorage.setItem('mytips-default-ai-model', defaultAIModel.value)
}

// 平滑滚动到指定锚点
function scrollToSection(id: string) {
  const el = document.getElementById(id)
  if (el) {
    el.scrollIntoView({ behavior: 'smooth', block: 'start' })
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
</script>

<style scoped>
/* Settings页面特有的样式 - 大部分暗色主题样式已移至全局 */

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
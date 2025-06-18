<template>
  <div class="ai-assistant-page flex flex-col h-screen" @click="hideContextMenu">
    <!-- 顶部区域 -->
    <div class="page-header p-4 bg-base-200 flex items-center justify-between">
      <div class="flex items-center">
        <button class="btn btn-ghost mr-2" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <div>
          <h1 class="text-2xl font-bold">AI 助手</h1>
          <p class="text-base-content/70">在MyTips内与您喜爱的AI模型对话</p>
        </div>
      </div>
      <div class="flex gap-2">
        <button class="btn" @click="openRoleManager">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          角色管理{{ roles.length > 0 ? ` (${roles.length})` : '' }}
        </button>
        <button class="btn" @click="openConversationsList">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          对话列表 ({{ orderedConversations.length }})
        </button>
        <button class="btn btn-primary" @click="createNewConversation">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          新建对话
        </button>
      </div>
    </div>

    <!-- 主体内容区域 - 分为左侧聊天和右侧笔记 -->
    <div class="flex flex-1 overflow-hidden relative">
      <!-- 对话列表抽屉 -->
      <transition name="fade-slide">
        <div v-if="showConversationsDrawer"
          class="conversations-drawer absolute top-0 left-0 bottom-0 z-[99999] w-full h-full flex">
          <div
            class="w-80 h-full bg-white shadow-xl overflow-y-auto transform transition-transform duration-300 ease-in-out translate-x-0"
            @click.stop>
            <div class="p-4 flex flex-col h-full">
              <div class="flex-1 overflow-y-auto">
                <!-- 这里恢复原有对话列表内容 -->
                <div v-if="isLoadingConversations" class="flex items-center justify-center h-40 text-base-content/70">
                  <span class="loading loading-spinner loading-lg mr-2"></span> 正在加载对话...
                </div>
                <template v-else>
                  <div v-if="orderedConversations.length === 0" class="text-center py-8 text-base-content/70">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto mb-2 opacity-50" fill="none"
                      viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                    <p class="font-medium">暂无对话记录</p>
                    <p class="text-sm mb-3">您可以创建新的对话开始与AI交流</p>
                    <button class="btn btn-sm btn-primary" @click="createNewConversation">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                      创建新对话
                    </button>
                  </div>
                  <div v-else class="space-y-3">
                    <div class="flex justify-between items-center text-sm text-base-content/70 mb-2 px-1">
                      <span>{{ orderedConversations.length }}个对话</span>
                      <span>按更新时间排序</span>
                    </div>
                    <div v-for="conversation in orderedConversations" :key="conversation.id"
                      class="p-3 rounded-lg cursor-pointer hover:bg-base-200 transition-colors border border-base-300 flex flex-col"
                      :class="{ 'bg-base-200 border-primary': conversation.id === activeConversationId }"
                      @click="switchConversation(conversation.id)">
                      <div class="flex justify-between items-start">
                        <div class="flex-1 min-w-0">
                          <div class="font-medium truncate">{{ conversation.title || '无标题对话' }}</div>
                          <div class="text-xs text-base-content/70 flex items-center flex-wrap">
                            <span class="truncate">{{ getModelNameById(conversation.model) }}</span>
                            <span class="mx-1">•</span>
                            <span>{{ formatDate(conversation.updated_at || conversation.updatedAt) }}</span>
                          </div>
                        </div>
                        <div class="flex">
                          <button class="btn btn-ghost btn-xs" @click.stop="editConversationTitle(conversation.id)">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                              stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                          </button>
                          <button class="btn btn-ghost btn-xs text-error"
                            @click.stop="confirmDeleteConversation(conversation.id)">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                              stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                          </button>
                        </div>
                      </div>
                      <div v-if="conversation.messages && conversation.messages.length > 0"
                        class="text-xs text-base-content/50 mt-1 line-clamp-2 bg-base-200/50 p-1 rounded">
                        {{ conversation.messages[0].content.substring(0, 100) }}{{
                          conversation.messages[0].content.length > 100 ? '...' : '' }}
                      </div>
                    </div>
                  </div>
                </template>
              </div>
            </div>
          </div>
          <div class="flex-1 h-full bg-black bg-opacity-20 cursor-pointer" @click="showConversationsDrawer = false">
          </div>
        </div>
      </transition>

      <!-- 左侧AI聊天区域 -->
      <div class="flex-1 flex flex-col p-4 border-r border-base-300 overflow-hidden">
        <!-- 聊天界面 -->
        <div class="chat-container bg-base-200 rounded-lg p-4 flex-1 flex flex-col overflow-hidden">
          <!-- 消息显示区域 -->
          <div class="flex-grow overflow-y-auto mb-4 space-y-4" ref="messagesContainer" @mouseup="handleTextSelection"
            @contextmenu.prevent="handleContextMenu">
            <div v-if="messages.length === 0"
              class="flex flex-col items-center justify-center h-full text-base-content/60">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-16 h-16 mb-4">
                <path stroke-linecap="round" stroke-linejoin="round"
                  d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
              </svg>
              <p class="text-lg">选择AI模型开始对话</p>
              <p class="text-sm">您的对话仅保存在本地，不会上传到服务器</p>
            </div>

            <!-- 消息数量提示 -->
            <div v-if="messages.length > MAX_VISIBLE_MESSAGES" class="message-count-notice bg-warning/10 border border-warning/30 rounded-lg p-3 mb-4 text-center">
              <div class="flex items-center justify-center gap-2 text-warning-content">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span class="text-sm font-medium">
                  为了提升性能，仅显示最新的 {{ MAX_VISIBLE_MESSAGES }} 条消息 (共 {{ messages.length }} 条)
                </span>
              </div>
              <p class="text-xs text-warning-content/70 mt-1">
                较早的消息已隐藏，但仍保存在对话历史中
              </p>
            </div>

            <template v-for="(message, _messageIndex) in visibleMessages" :key="_messageIndex">
              <div :class="['chat', message.role === 'user' ? 'chat-end' : 'chat-start']">
                <div class="chat-image avatar">
                  <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                    <img v-if="message.role === 'user'" src="/img/user-avatar.svg" alt="User" />
                    <img v-else :src="`/img/${selectedModel}-avatar.svg`" :alt="getSelectedModelName()" />
                  </div>
                </div>
                <div class="chat-header">
                  {{ message.role === 'user' ? '您' : (message.role_name || getSelectedModelName()) }}
                  <time class="text-xs opacity-50 ml-1">{{ formatTime(message.timestamp) }}</time>
                </div>
                <div :class="['chat-bubble', message.failed ? 'border border-red-500' : '']">
                  <!-- 显示附件 -->
                  <div v-if="message.attachments && message.attachments.length > 0" class="mb-2">
                    <div v-for="attachment in message.attachments" :key="attachment.id" class="attachment-item mb-2">
                      <!-- 图片附件 -->
                      <div v-if="attachment.type.startsWith('image/')" class="image-attachment">
                        <img :src="attachment.url" :alt="attachment.name"
                          class="max-w-xs max-h-48 rounded-lg cursor-pointer" @click="previewImage(attachment.url)" />
                        <p class="text-xs text-base-content/70 mt-1">{{ attachment.name }}</p>
                      </div>
                      <!-- 文档附件 -->
                      <div v-else class="document-attachment flex items-center p-2 bg-base-300 rounded-lg">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2 text-primary" fill="none"
                          viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        <div class="flex-1">
                          <p class="text-sm font-medium">{{ attachment.name }}</p>
                          <p class="text-xs text-base-content/70">{{ formatFileSize(attachment.size) }}</p>
                        </div>
                        <button class="btn btn-xs btn-ghost" @click="downloadFile(attachment)">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                          </svg>
                        </button>
                      </div>
                    </div>
                  </div>

                  <!-- 显示引用的笔记 -->
                  <div v-if="message.referencedNotes && message.referencedNotes.length > 0" class="mb-2">
                    <div v-for="note in message.referencedNotes" :key="note.id" class="referenced-note-item mb-2">
                      <div
                        class="flex p-2 bg-info/10 rounded-lg border border-info/20 cursor-pointer hover:bg-info/20 transition-colors"
                        @click="showNoteDetail(note)">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-info flex-shrink-0" fill="none"
                          viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        <div class="flex-1 min-w-0">
                          <p class="text-sm font-medium text-info ">{{ note.title }}</p>
                          <p class="text-xs text-base-content/70">{{ note.tip_type }} • 点击查看详情</p>
                        </div>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-info flex-shrink-0 ml-1" fill="none"
                          viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                      </div>
                    </div>
                  </div>

                  <!-- 消息内容 -->
                  <div v-html="formatMessage(message.content)"></div>
                </div>
                <div class="chat-footer opacity-50 flex gap-1" v-if="message.role === 'assistant'">
                  <button class="btn btn-xs btn-ghost" @click="copyToClipboard(message.content)">复制</button>
                  <button class="btn btn-xs btn-ghost" @click="addToNote(message.content)">添加到笔记</button>
                </div>
                <div class="chat-footer opacity-80 flex gap-1" v-if="message.failed && message.role === 'user'">
                  <span class="text-red-500">发送失败</span>
                  <button class="btn btn-xs btn-error" @click="sendMessage(message)">重新发送</button>
                </div>
              </div>
            </template>

            <!-- 流式输出响应 -->
            <div v-if="isStreaming" class="chat chat-start">
              <div class="chat-image avatar">
                <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                  <img :src="`/img/${selectedModel}-avatar.svg`" :alt="getSelectedModelName()" />
                </div>
              </div>
              <div class="chat-header">
                {{ selectedRole ? selectedRole.name : getSelectedModelName() }}
                <time class="text-xs opacity-50 ml-1">{{ formatTime(Date.now()) }}</time>
              </div>
              <div class="chat-bubble" v-html="formatMessage(streamingContent)"></div>
            </div>

            <!-- 加载中指示器 -->
            <div v-if="isLoading && !isStreaming" class="chat chat-start">
              <div class="chat-image avatar">
                <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                  <img :src="`/img/${selectedModel}-avatar.svg`" :alt="getSelectedModelName()" />
                </div>
              </div>
              <div class="chat-bubble">
                <span class="loading loading-dots loading-md"></span>
              </div>
            </div>
          </div>

          <!-- 输入区域 -->
          <div class="flex flex-col gap-3">
            <!-- 已上传文件预览区域 -->
            <div v-if="uploadedFiles.length > 0" class="uploaded-files-preview bg-base-100 rounded-lg p-3">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">已选择文件 ({{ uploadedFiles.length }})</span>
                <button class="btn btn-xs btn-ghost" @click="clearAllFiles">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                  清空
                </button>
              </div>
              <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
                <div v-for="(file, index) in uploadedFiles" :key="index" class="file-preview-item relative">
                  <!-- 图片预览 -->
                  <div v-if="file.type.startsWith('image/')" class="relative">
                    <img :src="file.preview" :alt="file.name" class="w-full h-20 object-cover rounded-lg" />
                    <button class="absolute -top-1 -right-1 btn btn-xs btn-circle btn-error" @click="removeFile(index)">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <!-- 文档预览 -->
                  <div v-else
                    class="relative bg-base-200 rounded-lg p-2 h-20 flex flex-col items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-primary mb-1" fill="none"
                      viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <span class="text-xs text-center truncate w-full">{{ file.name }}</span>
                    <button class="absolute -top-1 -right-1 btn btn-xs btn-circle btn-error" @click="removeFile(index)">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <p class="text-xs text-center mt-1 truncate">{{ file.name }}</p>
                </div>
              </div>
            </div>



            <!-- 消息输入框区域 -->
            <div class="message-input-container bg-base-100 rounded-lg border border-base-300">
              <!-- 顶部工具栏 -->
              <div class="input-toolbar flex items-center justify-between p-2 border-b border-base-300">
                <div class="flex items-center gap-2">
                  <!-- AI模型选择 -->
                  <select v-model="selectedModel" class="select select-sm select-bordered">
                    <option disabled value="">选择AI模型</option>
                    <option v-for="model in availableModels" :key="model.id" :value="model.id">
                      {{ model.name }}
                    </option>
                  </select>

                  <button @click="openApiSettings" class="btn btn-sm btn-ghost tooltip tooltip-top" data-tip="API设置">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                      stroke="currentColor" class="w-4 h-4">
                      <path stroke-linecap="round" stroke-linejoin="round"
                        d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" />
                      <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                  </button>

                  <!-- 角色选择显示 -->
                  <div class="flex items-center gap-2">
                    <div v-if="selectedRole" class="role-badge badge badge-primary badge-sm flex items-center gap-1">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                      {{ selectedRole.name }}
                      <button @click="clearSelectedRole" class="ml-1 hover:bg-primary-focus rounded-full">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                          stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M6 18L18 6M6 6l12 12" />
                        </svg>
                      </button>
                    </div>
                    <button class="btn btn-xs btn-outline tooltip tooltip-bottom flex items-center" data-tip="选择角色"
                      @click="openRoleManager">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                      <span>{{ selectedRole ? '更换角色' : '选择角色' }}</span>
                    </button>
                  </div>

                  <!-- 文件上传按钮 -->
                  <div class="file-upload-container">
                    <input ref="fileInput" type="file" multiple
                      accept="image/jpeg,image/jpg,image/png,image/gif,image/webp,image/bmp,image/tiff,image/x-icon,.jpg,.jpeg,.png,.apng,.gif,.webp,.bmp,.dib,.tiff,.tif,.ico,.icns,.sgi,.j2c,.j2k,.jp2,.jpc,.jpf,.jpx,.pdf,.doc,.docx,.txt,.md,.json,.csv,.xlsx,.xls"
                      @change="handleFileUpload" class="hidden" />
                    <button class="btn btn-sm btn-ghost tooltip tooltip-top" data-tip="上传文件" @click="triggerFileUpload"
                      :disabled="isLoading || isStreaming">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                      </svg>
                    </button>
                  </div>
                </div>

                <!-- 右侧工具 -->
                <div class="flex items-center gap-1">
                  <!-- 工具按钮 -->
                  <button class="btn btn-sm btn-outline" @click="createNewConversation" title="新建对话">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    新建对话
                  </button>
                  <button class="btn btn-sm btn-outline" @click="clearMessages" title="清空对话历史">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    清空对话
                  </button>
                  <button class="btn btn-sm btn-outline" @click="exportMessages" title="导出对话内容">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                    导出对话
                  </button>
                  

                </div>
              </div>

              <!-- API密钥警告 -->
              <div v-if="!hasApiKey" class="alert alert-warning m-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                  viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <span>您尚未配置{{ getSelectedModelName() }}的API密钥。请点击设置图标进行配置。</span>
              </div>

              <!-- 文本输入区域 -->
              <div class="relative p-3">
                <!-- 角色提示信息 -->
                <div v-if="selectedRole"
                  class="role-prompt-container mb-2 p-2 bg-primary/10 rounded-lg border border-primary/20">
                  <div class="flex items-center gap-2 mb-1">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-primary" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                    <span class="text-sm font-medium text-primary">当前角色：{{ selectedRole.name }}</span>
                  </div>
                  <p class="text-xs text-base-content/70 line-clamp-2">{{ selectedRole.description }}</p>
                </div>

                <textarea ref="messageTextarea" v-model="userInput" @keydown="handleInputKeydown"
                  @keydown.enter.prevent="handleEnterKey" :placeholder="getInputPlaceholder()"
                  class="textarea w-full resize-none border-0 focus:outline-none bg-transparent"
                  :disabled="!selectedModel || !hasApiKey || isLoading || isStreaming" rows="3"></textarea>

                <!-- 笔记选择器 -->
                <div v-if="showNoteSelector"
                  class="note-selector absolute z-50 w-full bg-base-100 border border-base-300 rounded-lg shadow-xl max-h-60 overflow-y-auto"
                  style="bottom: 100%; margin-bottom: 8px;" @click.stop>
                  <!-- 搜索框 -->
                  <div class="p-2 border-b border-base-300">
                    <input v-model="noteSearchQuery" type="text" placeholder="搜索笔记...（回车选择第一个，ESC关闭）"
                      class="input input-xs input-bordered w-full" @keydown.enter.prevent="selectFirstNote"
                      @keydown.escape.prevent="hideNoteSelector" @keydown.tab.prevent="hideNoteSelector"
                      ref="noteSearchInput" />
                    <div class="text-xs text-base-content/50 mt-1 flex justify-between">
                      <span>显示 {{ filteredNotes.length }} 篇笔记{{ noteSearchQuery.trim() ? ' (搜索结果)' : ' (最新)' }}</span>
                      <span>Enter选择 • ESC关闭</span>
                    </div>
                  </div>

                  <!-- 笔记列表 -->
                  <div class="max-h-48 overflow-y-auto">
                    <div v-for="note in filteredNotes" :key="note.id"
                      class="note-selector-item px-3 py-2 hover:bg-base-200 cursor-pointer border-b border-base-200 last:border-b-0"
                      @click="selectNote(note)">
                      <div class="flex items-start gap-2">
                        <div class="flex-shrink-0 mt-1">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-base-content/50" fill="none"
                            viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                          </svg>
                        </div>
                        <div class="flex-1 min-w-0">
                          <div class="font-medium text-sm truncate">{{ note.title }}</div>
                          <div class="text-xs text-base-content/70 line-clamp-2 mt-1">
                            {{ note.content.substring(0, 100) }}{{ note.content.length > 100 ? '...' : '' }}
                          </div>
                          <div class="flex items-center gap-2 mt-1">
                            <span class="text-xs text-base-content/50">{{ note.tip_type }}</span>
                            <span class="text-xs text-base-content/50">{{ formatDate(note.updated_at) }}</span>
                          </div>
                        </div>
                      </div>
                    </div>

                    <!-- 空状态 -->
                    <div v-if="filteredNotes.length === 0" class="p-4 text-center text-base-content/70">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mx-auto mb-2 opacity-50" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                      <p class="text-sm">{{ tipsStore.tips.length === 0 ? '暂无笔记，请先创建一些笔记' : '没有找到匹配的笔记' }}</p>
                    </div>
                  </div>
                </div>

                <!-- 发送按钮 -->
                <button @click="isStreaming ? cancelGeneration() : sendMessage()" class="btn absolute right-5 bottom-5"
                  :class="isStreaming ? 'btn-error' : 'btn-primary'"
                  :disabled="(!userInput.trim() && uploadedFiles.length === 0 && !isStreaming) || !selectedModel || !hasApiKey"
                  :title="isStreaming ? '取消生成' : '发送消息'">
                  <!-- 发送图标（右箭头） -->
                  <svg v-if="!isStreaming" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M14 5l7 7m0 0l-7 7m7-7H3" />
                  </svg>
                  <!-- 取消图标（停止方块） -->
                  <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <rect x="6" y="6" width="12" height="12" stroke-width="2" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧笔记区域 -->
      <transition name="fade-slide">
        <div class="w-1/3 min-w-[350px] flex flex-col p-4 overflow-hidden" v-show="showNotePanel">
          <div class="flex justify-between items-center mb-4">
            <h2 class="text-xl font-bold">笔记</h2>
            <div class="flex gap-2">
              <button class="btn btn-sm" @click="showNotePanel = false">关闭</button>
              <button class="btn btn-sm btn-primary" @click="saveNoteAsTip"
                :disabled="!noteTitle.trim() || isNoteSaving">
                <span v-if="isNoteSaving" class="loading loading-spinner loading-xs"></span>
                保存笔记
              </button>
            </div>
          </div>

          <div class="form-control w-full mb-4">
            <label class="label">
              <span class="label-text">笔记标题</span>
            </label>
            <input type="text" placeholder="输入笔记标题..." class="input input-bordered w-full" v-model="noteTitle" />
          </div>

          <div class="flex-1 overflow-hidden flex flex-col">
            <label class="label">
              <span class="label-text">笔记内容 (支持Markdown格式)</span>
            </label>
            <textarea v-model="noteContent" placeholder="可以从左侧AI对话添加内容到这里..."
              class="textarea textarea-bordered w-full flex-1 font-mono resize-none"></textarea>
          </div>
        </div>
      </transition>
    </div>

    <!-- 笔记保存成功提示 -->
    <div class="toast toast-end" v-if="showSaveSuccess">
      <div class="alert alert-success">
        <span>笔记保存成功!</span>
      </div>
    </div>

    <!-- 设置对话框 -->
    <dialog ref="apiSettingsModal" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">API 设置</h3>

        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">选择AI模型</span>
          </label>
          <select v-model="editingModel" class="select select-bordered w-full">
            <option disabled value="">选择要配置的AI模型</option>
            <option v-for="model in availableModels" :key="model.id" :value="model.id">
              {{ model.name }}
            </option>
          </select>
        </div>

        <div class="form-control w-full mt-4">
          <label class="label">
            <span class="label-text">API 密钥</span>
          </label>
          <div class="relative">
            <input :type="showApiKey ? 'text' : 'password'" v-model="apiKey" placeholder="输入您的API密钥"
              class="input input-bordered w-full pr-10" />
            <button @click="showApiKey = !showApiKey" type="button"
              class="absolute inset-y-0 right-0 flex items-center px-3">
              <svg v-if="showApiKey" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
            </button>
          </div>
          <label class="label">
            <span class="label-text-alt">API密钥仅存储在本地，不会上传到任何服务器</span>
          </label>
        </div>

        <div class="form-control w-full mt-4">
          <label class="label">
            <span class="label-text">自定义模型名称 <span class="text-sm opacity-70">(可选)</span></span>
            <button class="btn btn-xs btn-outline" @click="openModelConfigModal(editingModel)"
              :disabled="!editingModel">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4" />
              </svg>
              配置模型
            </button>
          </label>

          <!-- 可搜索的模型名称选择器 -->
          <div class="relative">
            <input type="text" v-model="customModelName" :placeholder="getDefaultModelName(editingModel)"
              class="input input-bordered w-full" @focus="showModelSuggestions = true" @input="filterModelSuggestions"
              @blur="hideModelSuggestions" autocomplete="off" />

            <!-- 模型建议下拉框 -->
            <div v-if="showModelSuggestions && filteredModelSuggestions.length > 0"
              class="absolute z-10 w-full bg-base-100 border border-base-300 rounded-lg shadow-lg max-h-60 overflow-y-auto mt-1 model-suggestions-dropdown">
              <div v-for="suggestion in filteredModelSuggestions" :key="suggestion.name"
                class="px-3 py-2 hover:bg-base-200 cursor-pointer border-b border-base-200 last:border-b-0 model-suggestion-item"
                @mousedown.prevent="selectModelSuggestion(suggestion.name)">
                <div class="font-medium">{{ suggestion.name }}</div>
                <div class="text-sm text-base-content/70">{{ suggestion.description }}</div>
              </div>
            </div>
          </div>

          <label class="label">
            <span class="label-text-alt">不同的API提供商可能使用不同的模型名称，您可以在此自定义或从建议中选择</span>
          </label>
        </div>

        <!-- Max Tokens 配置 -->
        <div class="form-control w-full mt-4">
          <label class="label">
            <span class="label-text">最大输出长度 (Max Tokens)</span>
            <span class="label-text-alt">{{ maxTokens || getDefaultMaxTokens(editingModel) }}</span>
          </label>
          <div class="flex gap-3 items-center">
            <input type="range" v-model="maxTokens" :min="getMinMaxTokens(editingModel)"
              :max="getMaxMaxTokens(editingModel)" :step="100" class="range range-primary flex-1" />
            <input type="number" v-model="maxTokens" :min="getMinMaxTokens(editingModel)"
              :max="getMaxMaxTokens(editingModel)" :step="100" class="input input-bordered w-20 text-center" />
          </div>
          <div class="flex justify-between text-xs text-base-content/70 mt-1">
            <span>{{ getMinMaxTokens(editingModel) }}</span>
            <span>默认: {{ getDefaultMaxTokens(editingModel) }}</span>
            <span>{{ getMaxMaxTokens(editingModel) }}</span>
          </div>
          <label class="label">
            <span class="label-text-alt">控制AI响应的最大长度，越高生成内容越长，但响应时间和成本也会增加</span>
          </label>
        </div>

        <div class="form-control w-full mt-4" v-if="editingModel === 'custom'">
          <label class="label">
            <span class="label-text">API 端点</span>
          </label>
          <input type="text" v-model="apiEndpoint" placeholder="例如: https://api.example.com/v1/chat/completions"
            class="input input-bordered w-full" />
        </div>

        <div class="modal-action">
          <button class="btn" @click="closeApiSettings">取消</button>
          <button class="btn btn-primary" @click="saveApiSettings">保存</button>
        </div>
      </div>
    </dialog>

    <!-- 编辑对话标题对话框 -->
    <dialog ref="editTitleModal" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">编辑对话标题</h3>

        <div class="form-control w-full">
          <input type="text" v-model="editingTitle" placeholder="输入对话标题" class="input input-bordered w-full" />
        </div>

        <div class="modal-action">
          <button class="btn" @click="closeEditTitleModal">取消</button>
          <button class="btn btn-primary" @click="saveEditedTitle">保存</button>
        </div>
      </div>
    </dialog>

    <!-- 删除确认对话框 -->
    <dialog ref="deleteConfirmModal" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">确认删除</h3>
        <p>确定要删除这个对话吗？此操作无法撤销。</p>
        <div class="modal-action">
          <button class="btn" @click="closeDeleteConfirmModal">取消</button>
          <button class="btn btn-error" @click="confirmDelete">删除</button>
        </div>
      </div>
    </dialog>

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
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
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

    <!-- 自定义右键菜单 -->
    <div v-if="showContextMenu"
      class="custom-context-menu absolute bg-base-100 shadow-lg rounded-lg overflow-hidden z-50"
      :style="{ top: contextMenuPos.y + 'px', left: contextMenuPos.x + 'px' }">
      <ul class="menu menu-sm p-1">
        <li v-if="selectedText">
          <button @click="addSelectedTextToNote" class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
            添加到笔记
          </button>
        </li>
        <li>
          <button @click="copySelectedText" class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
            </svg>
            复制选中内容
          </button>
        </li>
      </ul>
    </div>

    <!-- 角色管理对话框 -->
    <dialog ref="roleManagerModal" class="modal">
      <div class="modal-box w-11/12 max-w-4xl">
        <h3 class="font-bold text-lg mb-4">角色管理</h3>

        <!-- 角色列表 -->
        <div v-if="!showRoleForm" class="space-y-4">
          <div class="flex justify-between items-center">
            <span class="text-sm text-base-content/70">
              {{ roles.length }} 个角色
            </span>
            <button class="btn btn-sm btn-primary" @click="openRoleForm()">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              新建角色
            </button>
          </div>

          <!-- 加载状态 -->
          <div v-if="isLoadingRoles" class="flex items-center justify-center py-8">
            <span class="loading loading-spinner loading-lg mr-2"></span>
            正在加载角色...
          </div>

          <!-- 角色列表 -->
          <div v-else-if="roles.length > 0" class="grid grid-cols-1 md:grid-cols-2 gap-4 max-h-96 overflow-y-auto">
            <div v-for="role in roles" :key="role.id"
              class="role-card card bg-base-200 shadow-sm border border-base-300">
              <div class="card-body p-4">
                <div class="flex justify-between items-start mb-2">
                  <h4 class="card-title text-base">{{ role.name }}</h4>
                  <div class="flex gap-1">
                    <button class="btn btn-xs btn-ghost" @click="openRoleForm(role)">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                      </svg>
                    </button>
                    <button class="btn btn-xs btn-error" @click="confirmDeleteRole(role.id)">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </div>
                <p class="text-sm text-base-content/70 line-clamp-3">{{ role.description }}</p>
                <div class="card-actions justify-end mt-3">
                  <button class="btn btn-xs btn-outline" @click="selectRole(role); closeRoleManager()">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                    </svg>
                    选择此角色
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- 空状态 -->
          <div v-else class="text-center py-12">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 text-base-content/30" fill="none"
              viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            <h3 class="text-lg font-medium mb-2">暂无角色</h3>
            <p class="text-base-content/70 mb-4">创建您的第一个AI角色来开始角色扮演对话</p>
            <button class="btn btn-primary" @click="openRoleForm()">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              创建角色
            </button>
          </div>
        </div>

        <!-- 角色表单 -->
        <div v-if="showRoleForm" class="space-y-4">
          <div class="flex items-center gap-2 mb-4">
            <button class="btn btn-sm btn-ghost" @click="closeRoleForm">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
            </button>
            <h4 class="text-lg font-medium">
              {{ editingRole ? '编辑角色' : '创建新角色' }}
            </h4>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text">角色名称</span>
            </label>
            <input type="text" v-model="newRoleName" placeholder="例如：编程导师、创意写手、数据分析师..."
              class="input input-bordered w-full" maxlength="50" />
            <label class="label">
              <span class="label-text-alt">{{ newRoleName.length }}/50</span>
            </label>
          </div>

          <div class="form-control">
            <label class="label">
              <span class="label-text">角色描述</span>
            </label>
            <textarea v-model="newRoleDescription"
              placeholder="详细描述这个角色的特点、专长、说话风格等。例如：你是一位经验丰富的编程导师，擅长用简单易懂的方式解释复杂的编程概念..."
              class="textarea textarea-bordered w-full h-32 resize-none" maxlength="100000"></textarea>
            <label class="label">
              <span class="label-text-alt">{{ newRoleDescription.length }}/100000</span>
            </label>
          </div>

          <div class="flex gap-2 pt-4">
            <button class="btn btn-ghost" @click="closeRoleForm">取消</button>
            <button class="btn btn-primary" @click="saveRole"
              :disabled="!newRoleName.trim() || !newRoleDescription.trim()">
              {{ editingRole ? '保存更改' : '创建角色' }}
            </button>
          </div>
        </div>

        <div class="modal-action" v-if="!showRoleForm">
          <button class="btn" @click="closeRoleManager">关闭</button>
        </div>
      </div>
    </dialog>

    <!-- 笔记详情显示对话框 -->
    <dialog ref="noteDetailModal" class="modal">
      <div class="modal-box max-w-4xl">
        <h3 class="font-bold text-lg mb-4">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 inline mr-2" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          {{ currentNoteDetail?.title }}
        </h3>

        <div v-if="currentNoteDetail" class="space-y-4">
          <div class="flex items-center gap-4 text-sm text-base-content/70">
            <span class="badge badge-outline">{{ currentNoteDetail.tip_type }}</span>
            <span>{{ formatDate(currentNoteDetail.updated_at || Date.now()) }}</span>
          </div>

          <div class="divider"></div>

          <div class="prose max-w-none">
            <div v-html="formatMessage(currentNoteDetail.content)"></div>
          </div>
        </div>

        <div class="modal-action">
          <button class="btn" @click="closeNoteDetail">关闭</button>
          <button class="btn btn-outline" @click="copyNoteContent">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
              stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
          </svg>
            复制内容
          </button>
        </div>
      </div>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, computed, watch, onBeforeUnmount, onActivated, onDeactivated } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import DOMPurify from 'dompurify'
import { useTipsStore } from '../stores/tipsStore'
import { useRouter } from 'vue-router'
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import Prism from 'prismjs'
import 'prismjs/themes/prism-tomorrow.css'
import 'prismjs/components/prism-javascript'
import 'prismjs/components/prism-typescript'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-java'
import 'prismjs/components/prism-json'
import 'prismjs/components/prism-bash'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-sql'
import 'prismjs/components/prism-go'
import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-php'
import 'prismjs/components/prism-csharp'

// 引入tips存储
const tipsStore = useTipsStore()

// 后端API调用封装
async function listAIConversations() {
  return await invoke('list_ai_conversations')
}
async function listAIMessages(conversationId: string) {
  return await invoke('list_ai_messages', { conversation_id: conversationId })
}
async function createAIConversation(model: string, title?: string) {
  return await invoke('create_ai_conversation', { model, title })
}
async function deleteAIConversation(conversationId: string) {
  return await invoke('delete_ai_conversation', { conversation_id: conversationId })
}
async function updateAIConversationTitle(conversationId: string, newTitle: string) {
  return await invoke('update_ai_conversation_title', { conversation_id: conversationId, new_title: newTitle })
}
async function addAIMessage(conversationId: string, role: string, content: string) {
  return await invoke('add_ai_message', { conversation_id: conversationId, role, content })
}

// 角色相关API调用
async function listAIRoles() {
  return await invoke('list_ai_roles')
}
async function createAIRole(name: string, description: string) {
  return await invoke('create_ai_role', { name, description })
}
async function updateAIRole(roleId: string, name: string, description: string) {
  return await invoke('update_ai_role', { role_id: roleId, name, description })
}
async function deleteAIRole(roleId: string) {
  return await invoke('delete_ai_role', { role_id: roleId })
}
// @ts-ignore - 保留此函数供未来使用
async function getAIRole(roleId: string) {
  return await invoke('get_ai_role', { role_id: roleId })
}

// 引入路由
const router = useRouter()

// 返回主页
const goBack = () => {
  router.push('/')
}

// 安全检查 Prism 语言是否可用
function isPrismLanguageAvailable(lang: string): boolean {
  try {
    // plaintext 总是可用的，因为它不需要特殊的语法高亮
    if (lang === 'plaintext' || lang === 'text' || lang === 'plain') {
      return true;
    }
    
    return !!(
      typeof Prism !== 'undefined' && 
      Prism.languages && 
      typeof Prism.languages === 'object' &&
      Prism.languages[lang] &&
      typeof Prism.highlight === 'function'
    );
  } catch (error) {
    console.warn(`检查 Prism 语言 ${lang} 时出错:`, error);
    return false;
  }
}

// HTML 转义函数
function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

// 对话和消息状态
const conversations = ref<any[]>([])
const activeConversationId = ref('')
const messages = ref<any[]>([])
const isLoadingConversations = ref(false)
const isLoadingMessages = ref(false)

// 持久化状态
const selectedModel = ref(localStorage.getItem('ai-selected-model') || 'gemini')
const hasApiKey = ref(true)
const userInput = ref('')
const isLoading = ref(false)
const messagesContainer = ref<HTMLElement | null>(null)

// 流式输出相关
const isStreaming = ref(false)
const streamingContent = ref('')
const currentStreamingId = ref<string | null>(null)

// 性能优化相关
const messageRenderCache = ref(new Map<string, string>()) // 消息渲染缓存
const formatMessageDebounced = ref(new Map<string, NodeJS.Timeout>()) // 格式化防抖
const MAX_VISIBLE_MESSAGES = ref(100) // 最大可见消息数量
const visibleMessages = computed(() => {
  // 只显示最近的消息，避免渲染过多内容
  if (messages.value.length <= MAX_VISIBLE_MESSAGES.value) {
    return messages.value
  }
  return messages.value.slice(-MAX_VISIBLE_MESSAGES.value)
})

// 是否显示对话列表抽屉
const showConversationsDrawer = ref(false)

// API 设置相关
const apiSettingsModal = ref<HTMLDialogElement | null>(null)
const editingModel = ref('')
const apiKey = ref('')
const apiEndpoint = ref('')
const customModelName = ref('')
const showApiKey = ref(false)
const maxTokens = ref<number>(3000) // 默认值3000

// 模型建议相关
const showModelSuggestions = ref(false)
const filteredModelSuggestions = ref<Array<{ name: string, description: string }>>([])

// 默认的各AI模型的常用模型名称数据
const defaultModelSuggestions = {
  chatgpt: [
    { name: 'gpt-4o', description: 'GPT-4o - 最新版本，支持文本和图像' },
    { name: 'gpt-4o-mini', description: 'GPT-4o Mini - 轻量级版本，成本更低' },
    { name: 'gpt-4-turbo', description: 'GPT-4 Turbo - 高性能版本' },
    { name: 'gpt-4', description: 'GPT-4 - 标准版本' },
    { name: 'gpt-3.5-turbo', description: 'GPT-3.5 Turbo - 经典版本，性价比高' },
    { name: 'gpt-3.5-turbo-16k', description: 'GPT-3.5 Turbo - 16K上下文版本' }
  ],
  gemini: [
    { name: 'gemini-2.0-flash', description: 'Gemini 2.0 Flash - 最新版本' },
    { name: 'gemini-1.5-flash', description: 'Gemini 1.5 Flash - 快速响应版本' },
    { name: 'gemini-1.5-pro', description: 'Gemini 1.5 Pro - 专业版本' },
    { name: 'gemini-pro', description: 'Gemini Pro - 标准专业版' },
    { name: 'gemini-pro-vision', description: 'Gemini Pro Vision - 支持图像理解' }
  ],
  deepseek: [
    { name: 'deepseek-chat', description: 'DeepSeek Chat - 对话模型' },
    { name: 'deepseek-coder', description: 'DeepSeek Coder - 代码生成专用' },
    { name: 'deepseek-v2', description: 'DeepSeek V2 - 第二代模型' },
    { name: 'deepseek-v2.5', description: 'DeepSeek V2.5 - 增强版本' }
  ],
  claude: [
    { name: 'claude-3.5-sonnet', description: 'Claude 3.5 Sonnet - 最新版本' },
    { name: 'claude-3-sonnet-20240229', description: 'Claude 3 Sonnet - 平衡版本' },
    { name: 'claude-3-opus-20240229', description: 'Claude 3 Opus - 最强版本' },
    { name: 'claude-3-haiku-20240307', description: 'Claude 3 Haiku - 快速版本' }
  ],
  qwen: [
    { name: 'qwen-max', description: '通义千问Max - 最强版本' },
    { name: 'qwen-plus', description: '通义千问Plus - 增强版本' },
    { name: 'qwen-turbo', description: '通义千问Turbo - 快速版本' },
    { name: 'qwen-long', description: '通义千问Long - 长文本版本' },
    { name: 'qwen2-72b-instruct', description: '通义千问2.0 72B指令版' }
  ],
  doubao: [
    { name: 'doubao-1.5-pro-32k', description: '豆包 1.5 Pro 32K - 长上下文版本' },
    { name: 'doubao-1.5-pro-4k', description: '豆包 1.5 Pro 4K - 标准版本' },
    { name: 'doubao-lite-32k', description: '豆包 Lite 32K - 轻量级长上下文' },
    { name: 'doubao-lite-4k', description: '豆包 Lite 4K - 轻量级标准版' }
  ],
  grok: [
    { name: 'grok-3', description: 'Grok 3 - 最新版本' },
    { name: 'grok-3-mini', description: 'Grok 3 Mini - 轻量级版本' },
    { name: 'grok-3-fast', description: 'Grok 3 Fast - 快速响应版本' },
    { name: 'grok-1.5', description: 'Grok 1.5 - 增强版本' }
  ],
  custom: [
    { name: 'gpt-3.5-turbo', description: 'OpenAI兼容 - GPT-3.5 Turbo' },
    { name: 'gpt-4', description: 'OpenAI兼容 - GPT-4' },
    { name: 'claude-3-sonnet', description: 'Claude兼容 - Sonnet' },
    { name: 'llama-2-70b-chat', description: 'Llama 2 70B Chat' },
    { name: 'mixtral-8x7b-instruct', description: 'Mixtral 8x7B Instruct' }
  ]
}

// 可配置的模型建议数据
const modelSuggestions = ref<typeof defaultModelSuggestions>({ ...defaultModelSuggestions })

// 模型配置管理相关
const showModelConfigModal = ref(false)
const modelConfigModal = ref<HTMLDialogElement | null>(null)
const editingModelType = ref('')
const editingModelList = ref<Array<{ name: string, description: string }>>([])
const newModelName = ref('')
const newModelDescription = ref('')

// 加载模型配置
const loadModelSuggestions = async () => {
  try {
    const config = localStorage.getItem('ai-model-suggestions-config')
    if (config) {
      const savedConfig = JSON.parse(config)
      modelSuggestions.value = { ...defaultModelSuggestions, ...savedConfig }
    }
  } catch (error) {
    console.warn('加载模型配置失败，使用默认配置:', error)
    modelSuggestions.value = { ...defaultModelSuggestions }
  }
}

// 保存模型配置
const saveModelSuggestions = async () => {
  try {
    localStorage.setItem('ai-model-suggestions-config', JSON.stringify(modelSuggestions.value))
  } catch (error) {
    console.error('保存模型配置失败:', error)
  }
}

// 打开模型配置管理对话框
const openModelConfigModal = (modelType: string) => {
  editingModelType.value = modelType
  editingModelList.value = [...(modelSuggestions.value[modelType as keyof typeof modelSuggestions.value] || [])]
  showModelConfigModal.value = true
  modelConfigModal.value?.showModal()
}

// 关闭模型配置管理对话框
const closeModelConfigModal = () => {
  showModelConfigModal.value = false
  editingModelType.value = ''
  editingModelList.value = []
  newModelName.value = ''
  newModelDescription.value = ''
  modelConfigModal.value?.close()
}

// 添加新模型
const addNewModel = () => {
  if (!newModelName.value.trim() || !newModelDescription.value.trim()) return

  editingModelList.value.push({
    name: newModelName.value.trim(),
    description: newModelDescription.value.trim()
  })

  newModelName.value = ''
  newModelDescription.value = ''
}

// 删除模型
const removeModel = (index: number) => {
  editingModelList.value.splice(index, 1)
}

// 重置为默认配置
const resetToDefault = () => {
  if (editingModelType.value && defaultModelSuggestions[editingModelType.value as keyof typeof defaultModelSuggestions]) {
    editingModelList.value = [...defaultModelSuggestions[editingModelType.value as keyof typeof defaultModelSuggestions]]
  }
}

// 保存模型配置更改
const saveModelConfig = async () => {
  if (editingModelType.value) {
    modelSuggestions.value[editingModelType.value as keyof typeof modelSuggestions.value] = [...editingModelList.value]
    await saveModelSuggestions()
    closeModelConfigModal()
  }
}

// 筛选模型建议
const filterModelSuggestions = () => {
  const modelType = editingModel.value as keyof typeof defaultModelSuggestions
  const allSuggestions = modelSuggestions.value[modelType] || []

  if (!customModelName.value.trim()) {
    filteredModelSuggestions.value = allSuggestions
  } else {
    const searchTerm = customModelName.value.toLowerCase()
    filteredModelSuggestions.value = allSuggestions.filter(suggestion =>
      suggestion.name.toLowerCase().includes(searchTerm) ||
      suggestion.description.toLowerCase().includes(searchTerm)
    )
  }
}

// 选择模型建议
const selectModelSuggestion = (modelName: string) => {
  customModelName.value = modelName
  showModelSuggestions.value = false
}

// 延迟隐藏建议框（避免点击建议时立即隐藏）
const hideModelSuggestions = () => {
  setTimeout(() => {
    showModelSuggestions.value = false
  }, 150)
}

// 监听编辑模型变化，自动更新模型建议
watch(editingModel, () => {
  if (editingModel.value) {
    filterModelSuggestions()
    // 当模型改变时，如果maxTokens为0或者小于最小值，则设置为默认值
    if (!maxTokens.value || maxTokens.value < getMinMaxTokens(editingModel.value)) {
      maxTokens.value = getDefaultMaxTokens(editingModel.value)
    }
    // 如果当前值超过最大值，则设置为最大值
    if (maxTokens.value > getMaxMaxTokens(editingModel.value)) {
      maxTokens.value = getMaxMaxTokens(editingModel.value)
    }
  }
})

// 笔记面板相关
const showNotePanel = ref(Boolean(localStorage.getItem('ai-show-note-panel') === 'true'))
const noteTitle = ref(localStorage.getItem('ai-note-title') || '')
const noteContent = ref(localStorage.getItem('ai-note-content') || '')
const isNoteSaving = ref(false)
const showSaveSuccess = ref(false)

// 编辑对话标题相关
const editTitleModal = ref<HTMLDialogElement | null>(null)
const editingTitle = ref('')
const editingConversationId = ref('')

// 删除确认相关
const deleteConfirmModal = ref<HTMLDialogElement | null>(null)
const deletingConversationId = ref('')

// 文件上传相关
const fileInput = ref<HTMLInputElement | null>(null)
const uploadedFiles = ref<Array<{
  id: string
  name: string
  type: string
  size: number
  preview?: string
  file: File
}>>([])

// 可用的AI模型
const availableModels = [
  { id: 'chatgpt', name: 'OpenAI ChatGPT' },
  { id: 'gemini', name: 'Google Gemini' },
  { id: 'deepseek', name: 'DeepSeek' },
  { id: 'qwen', name: '阿里通义千问' },
  { id: 'claude', name: 'Anthropic Claude' },
  { id: 'doubao', name: '字节豆包' },
  { id: 'grok', name: 'xAI Grok' },
  { id: 'custom', name: '自定义API' }
]

// Max Tokens 配置相关的辅助函数
const getDefaultMaxTokens = (modelId: string): number => {
  const defaults: Record<string, number> = {
    chatgpt: 4000,
    gemini: 4000,
    deepseek: 3000,
    qwen: 3000,
    claude: 4000,
    doubao: 2500,
    grok: 3000,
    custom: 3000
  }
  return defaults[modelId] || 3000
}

const getMinMaxTokens = (modelId: string): number => {
  const mins: Record<string, number> = {
    chatgpt: 500,
    gemini: 500,
    deepseek: 500,
    qwen: 500,
    claude: 500,
    doubao: 500,
    grok: 500,
    custom: 500
  }
  return mins[modelId] || 500
}

const getMaxMaxTokens = (modelId: string): number => {
  const maxs: Record<string, number> = {
    chatgpt: 8000,
    gemini: 8000,
    deepseek: 6000,
    qwen: 6000,
    claude: 8000,
    doubao: 4000,
    grok: 130000,
    custom: 8000
  }
  return maxs[modelId] || 8000
}

// 加载对话列表
async function loadConversations() {
  isLoadingConversations.value = true
  try {
    console.log('开始加载对话列表...')
    const result = await listAIConversations()

    conversations.value = Array.isArray(result) ? result : []
    console.log(`加载到 ${conversations.value.length} 个对话`)

    if (conversations.value.length > 0) {
      // 如果当前选中的对话不存在，选择第一个
      if (!activeConversationId.value || !conversations.value.some(c => c.id === activeConversationId.value)) {
        activeConversationId.value = conversations.value[0].id
        console.log(`选择对话: ${activeConversationId.value}`)
      }
      // 加载当前选中对话的消息
      await loadMessages(activeConversationId.value)
    } else {
      messages.value = []
      console.log('没有找到对话')
    }
  } catch (error) {
    console.error('加载对话列表失败:', error)
    conversations.value = []
    messages.value = []
  } finally {
    isLoadingConversations.value = false
  }
}

// 加载消息列表
async function loadMessages(conversationId: string) {
  if (!conversationId) {
    console.warn('尝试加载消息但没有有效的对话ID')
    messages.value = []
    return
  }

  isLoadingMessages.value = true
  try {
    console.log(`加载对话 ${conversationId} 的消息...`)
    const result = await listAIMessages(conversationId)
    const rawMessages = Array.isArray(result) ? result : []

    // 解析消息中的附件信息
    messages.value = rawMessages.map((msg: any) => {
      const processedMsg = { ...msg }

      // 检查消息内容中是否包含附件信息
      if (msg.content && msg.content.includes('__ATTACHMENTS__:')) {
        const parts = msg.content.split('\n\n__ATTACHMENTS__:')
        if (parts.length === 2) {
          try {
            // 解析附件JSON
            const attachments = JSON.parse(parts[1])
            processedMsg.content = parts[0] // 纯文本内容
            processedMsg.attachments = attachments // 附件信息
          } catch (error) {
            console.error('解析附件信息失败:', error)
            // 如果解析失败，保持原始内容
          }
        }
      }

      // 检查消息内容中是否包含笔记引用信息
      if (msg.content && msg.content.includes('__REFERENCED_NOTES__:')) {
        const parts = msg.content.split('\n\n__REFERENCED_NOTES__:')
        if (parts.length === 2) {
          try {
            // 解析笔记引用JSON
            const referencedNotes = JSON.parse(parts[1])
            processedMsg.content = parts[0] // 纯文本内容
            processedMsg.referencedNotes = referencedNotes // 笔记引用信息
          } catch (error) {
            console.error('解析笔记引用信息失败:', error)
            // 如果解析失败，保持原始内容
          }
        }
      }

      // 检查消息内容中是否包含角色名称信息
      if (msg.content && msg.content.includes('__ROLE_NAME__:')) {
        const parts = msg.content.split('\n\n__ROLE_NAME__:')
        if (parts.length === 2) {
          processedMsg.content = parts[0] // 纯文本内容
          processedMsg.role_name = parts[1] // 角色名称
        }
      }

      return processedMsg
    })

    console.log(`加载到 ${messages.value.length} 条消息`)
  } catch (error) {
    console.error(`加载消息失败:`, error)
    messages.value = []
  } finally {
    isLoadingMessages.value = false
  }
}

// 新建对话
async function createNewConversation() {
  const model = selectedModel.value
  const newId = await createAIConversation(model) as string
  await loadConversations()
  activeConversationId.value = newId
  await loadMessages(String(newId))
  showConversationsDrawer.value = false
}

// 切换对话
async function switchConversation(conversationId: string) {
  activeConversationId.value = conversationId
  await loadMessages(conversationId)
  showConversationsDrawer.value = false
}

// 删除对话
async function deleteConversation(conversationId: string) {
  await deleteAIConversation(conversationId)
  await loadConversations()
}


// 监听流式输出事件
const setupStreamListeners = async () => {
  let unlisten: any = null;
  try {
    unlisten = await listen('ai-stream-chunk', async (event: any) => {
      const payload = event.payload as { id: string, chunk: string, done: boolean };

      // 忽略不匹配当前会话ID的事件
      if (payload.id !== currentStreamingId.value) return;

      if (payload.done) {
        console.log(`收到完成事件: id=${payload.id}`);

        // 流式输出完成，添加完整消息到数据库，包含角色信息
        const messageContent = streamingContent.value
        let finalContent = messageContent

        // 如果有选中的角色，在消息内容中添加角色信息标记
        if (selectedRole.value) {
          finalContent += `\n\n__ROLE_NAME__:${selectedRole.value.name}`
        }

        await addAIMessage(activeConversationId.value, 'assistant', finalContent)

        // 重新加载消息列表
        await loadMessages(activeConversationId.value)

        // 重置状态
        isStreaming.value = false;
        streamingContent.value = '';
        currentStreamingId.value = null;
        isLoading.value = false;

        // 滚动到底部
        scrollToBottom();
      } else {
        // 收到第一个chunk时，关闭加载状态
        if (isLoading.value) {
          isLoading.value = false;
        }

        // 累加内容
        streamingContent.value += payload.chunk;

        // 滚动到底部
        scrollToBottom();
      }
    });
  } catch (error) {
    console.error('设置流式输出监听失败:', error);
  }

  // 在组件卸载时取消监听
  onBeforeUnmount(() => {
    if (unlisten && typeof unlisten === 'function') {
      unlisten();
    }
  });

  return unlisten;
};

// 发送消息到AI - 使用流式输出
async function sendMessage(resendMessage?: any) {
  let messageToSend = ''
  let resendIndex = -1
  let attachments: any[] = []
  let imageFileData: [string, number[]][] = [] // 预先准备图片文件数据

  if (resendMessage) {
    // 重新发送时，取消息内容和附件
    messageToSend = resendMessage.content
    attachments = resendMessage.attachments || []
    resendIndex = messages.value.findIndex(m => m === resendMessage)

    // 如果有图片附件，需要重新处理图片数据
    if (attachments.length > 0) {
      const imageAttachments = attachments.filter((att: any) => att.type.startsWith('image/'))
      if (imageAttachments.length > 0) {
        try {
          // 从URL重新获取图片数据
          imageFileData = await Promise.all(
            imageAttachments.map(async (att: any) => {
              const response = await fetch(att.url)
              const arrayBuffer = await response.arrayBuffer()
              const uint8Array = new Uint8Array(arrayBuffer)
              return [att.name, Array.from(uint8Array)] as [string, number[]]
            })
          )
        } catch (error) {
          console.error('重新处理图片文件失败:', error)
          alert('重新处理图片文件失败，请重试')
          return
        }
      }
    }

    // 先移除旧的失败消息
    if (resendIndex !== -1) messages.value.splice(resendIndex, 1)
  } else {
    // 正常发送新消息
    if ((!userInput.value.trim() && uploadedFiles.value.length === 0) || !selectedModel.value || isLoading.value || isStreaming.value) {
      return
    }
    messageToSend = userInput.value

    // 解析输入框中的笔记引用（#标题格式）并添加笔记内容到消息中
    let finalMessage = messageToSend
    let referencedNotes: any[] = []
    const noteReferences = messageToSend.match(/#([^#\s]+)/g) // 匹配 #标题 格式
    if (noteReferences && noteReferences.length > 0) {
      let notesContent = ''

      for (const ref of noteReferences) {
        const noteTitle = ref.substring(1) // 移除#号
        const note = tipsStore.tips.find(tip => tip.title === noteTitle)
        if (note) {
          notesContent += `\n\n--- 笔记：${note.title} ---\n${note.content}\n--- 笔记内容结束 ---\n`
          referencedNotes.push({
            id: note.id,
            title: note.title,
            content: note.content,
            tip_type: note.tip_type
          })
        }
      }

      if (notesContent) {
        // 将笔记内容添加到消息前面（用于AI处理）
        finalMessage = notesContent + '\n\n' + messageToSend
      }
    }

    messageToSend = finalMessage

    // 处理上传的文件
    if (uploadedFiles.value.length > 0) {
      attachments = uploadedFiles.value.map(file => ({
        id: file.id,
        name: file.name,
        type: file.type,
        size: file.size,
        url: file.preview || URL.createObjectURL(file.file)
      }))

      // 预先处理图片文件数据，避免后续清空文件列表后无法访问
      const imageFiles = uploadedFiles.value.filter(file => file.type.startsWith('image/'))
      if (imageFiles.length > 0) {
        try {
          imageFileData = await Promise.all(
            imageFiles.map(async (file) => {
              // 将File转换为ArrayBuffer，然后转换为Uint8Array
              const arrayBuffer = await file.file.arrayBuffer()
              const uint8Array = new Uint8Array(arrayBuffer)
              return [file.name, Array.from(uint8Array)] as [string, number[]]
            })
          )
        } catch (error) {
          console.error('处理图片文件失败:', error)
          alert('处理图片文件失败，请重试')
          return
        }
      }
    }

    // 保存用户消息到数据库（包含附件信息和笔记引用信息）
    // 将附件信息和笔记引用信息序列化为JSON字符串，附加到消息内容中
    let messageContent = userInput.value // 保存原始用户输入，不包含笔记内容
    if (attachments.length > 0) {
      const attachmentsJson = JSON.stringify(attachments)
      messageContent += `\n\n__ATTACHMENTS__:${attachmentsJson}`
    }
    if (referencedNotes.length > 0) {
      const notesJson = JSON.stringify(referencedNotes)
      messageContent += `\n\n__REFERENCED_NOTES__:${notesJson}`
    }

    await addAIMessage(activeConversationId.value, 'user', messageContent)

    userInput.value = ''
    clearAllFiles() // 清空已上传的文件
    clearAllSelectedNotes() // 清空选中的笔记（清空内部记录，不影响输入框）
    await loadMessages(activeConversationId.value)

    // 立即设置加载状态，让用户看到机器人正在思考
    isLoading.value = true

    await nextTick()
    scrollToBottom()
  }

  // 设置加载状态
  isLoading.value = true

  try {
    // 开始流式输出
    isStreaming.value = true
    isLoading.value = true
    streamingContent.value = ''
    currentStreamingId.value = generateUniqueId()

    // 获取当前模型的max_tokens配置
    let currentMaxTokens = maxTokens.value
    if (!currentMaxTokens) {
      try {
        currentMaxTokens = await invoke('get_max_tokens_config', { modelId: selectedModel.value }) as number
        if (!currentMaxTokens) {
          currentMaxTokens = getDefaultMaxTokens(selectedModel.value)
        }
      } catch (error) {
        console.warn('获取max_tokens配置失败，使用默认值:', error)
        currentMaxTokens = getDefaultMaxTokens(selectedModel.value)
      }
    }

    // 获取自定义模型名称
    let customModelName = ''
    try {
      customModelName = await invoke('get_model_name_config', { modelId: selectedModel.value }) as string
    } catch (error) {
      console.warn('获取自定义模型名称失败:', error)
    }

    // 检查是否有图片文件
    if (imageFileData.length > 0) {
      // 有图片文件，使用支持图片的API
      console.log(`发送包含${imageFileData.length}张图片的消息`)

      await invoke('send_ai_message_with_images_stream', {
        modelId: selectedModel.value,
        message: messageToSend,
        imageFiles: imageFileData,
        streamId: currentStreamingId.value,
        maxTokens: currentMaxTokens
      })
    } else {
      // 没有图片文件，使用普通API
      const recentMessages = messages.value
        .slice(-10)
        .map((msg: any) => ({ role: msg.role, content: msg.content }))

      // 如果有非图片附件，添加提示信息
      let finalMessage = messageToSend
      if (attachments.length > 0) {
        finalMessage += `\n\n[用户上传了${attachments.length}个文件: ${attachments.map(a => a.name).join(', ')}，但当前模型不支持文件处理]`
      }

      await invoke('send_ai_message_stream', {
        modelId: selectedModel.value,
        message: finalMessage,
        streamId: currentStreamingId.value,
        messages: recentMessages,
        customModelName: customModelName,
        maxTokens: currentMaxTokens,
        roleId: selectedRole.value?.id || null
      })
    }
  } catch (error) {
    console.error('AI响应失败:', error)
    // 添加失败消息到本地（不入库）
    messages.value.push({
      role: 'user',
      content: messageToSend,
      timestamp: Date.now(),
      failed: true,
      attachments: attachments
    })
    await nextTick()
    scrollToBottom()
    isStreaming.value = false
    streamingContent.value = ''
    currentStreamingId.value = null
    isLoading.value = false
    return
  }
}

// 检查是否有已配置的密钥
const checkApiKey = async () => {
  if (!selectedModel.value) {
    hasApiKey.value = false
    return
  }

  try {
    hasApiKey.value = await invoke('has_api_key', { modelId: selectedModel.value })
  } catch (error) {
    console.error('检查API密钥失败:', error)
    hasApiKey.value = false
  }
}

// 监听模型选择变化
watch(selectedModel, async (newModel) => {
  await checkApiKey()
  localStorage.setItem('ai-selected-model', newModel)
})

// 格式化消息内容（支持Markdown）
const formatMessage = (content: string): string => {
  // 检查缓存
  const cacheKey = content.substring(0, 1000) + content.length; // 使用内容前1000字符+长度作为缓存键
  if (messageRenderCache.value.has(cacheKey)) {
    return messageRenderCache.value.get(cacheKey)!;
  }

  try {
    // 创建 marked 实例并配置高亮
    const marked = new Marked();
    
    // 使用 marked-highlight 扩展
    marked.use(markedHighlight({
      langPrefix: 'language-',
      highlight(code: string, lang: string) {
        // 如果没有指定语言，使用 plaintext 作为默认语言
        const actualLang = lang || 'plaintext';
        
        // 限制代码块大小，超过50KB的代码块不进行语法高亮
        const MAX_CODE_SIZE = 50 * 1024; // 50KB
        const MAX_LINES = 1000; // 最大行数限制
        
        const codeLines = code.split('\n');
        const isLargeCode = code.length > MAX_CODE_SIZE || codeLines.length > MAX_LINES;
        
        if (isLargeCode) {
          // 对于大代码块，显示警告并提供折叠功能
          const truncatedCode = codeLines.length > MAX_LINES 
            ? codeLines.slice(0, MAX_LINES).join('\n') + `\n\n... (剩余 ${codeLines.length - MAX_LINES} 行已省略)`
            : code;
          
          return `<div class="large-code-block">
            <div class="code-warning bg-warning/20 text-warning-content p-2 text-sm rounded-t border border-warning/40">
              ⚠️ 大代码块 (${codeLines.length} 行, ${(code.length / 1024).toFixed(1)}KB) - 已禁用语法高亮以提升性能
              <button class="btn btn-xs btn-outline ml-2" onclick="this.parentElement.nextElementSibling.style.display = this.parentElement.nextElementSibling.style.display === 'none' ? 'block' : 'none'; this.textContent = this.textContent.includes('显示') ? '隐藏代码' : '显示代码'">
                ${codeLines.length > MAX_LINES ? '显示完整代码' : '隐藏代码'}
              </button>
            </div>
            <pre class="code-content"><code class="language-${actualLang}">${DOMPurify.sanitize(truncatedCode)}</code></pre>
            ${codeLines.length > MAX_LINES ? `<pre class="code-content" style="display: none;"><code class="language-${actualLang}">${DOMPurify.sanitize(code)}</code></pre>` : ''}
          </div>`;
        }
        
        // 使用安全检查函数
        if (actualLang && isPrismLanguageAvailable(actualLang)) {
          try {
            return Prism.highlight(code, Prism.languages[actualLang], actualLang);
          } catch (error) {
            console.warn(`Prism 高亮失败 (${actualLang}):`, error);
            return escapeHtml(code);
          }
        }
        
        // 如果 plaintext 也不可用，直接返回转义的代码
        return escapeHtml(code);
      }
    }));

    // 配置 marked 选项
    marked.setOptions({
      breaks: true,
      gfm: true,
      silent: true,
    });

    // 使用 marked 渲染 Markdown
    const htmlContent = marked.parse(content) as string;

    // 使用DOMPurify清理HTML，防止XSS
    const result = DOMPurify.sanitize(htmlContent, {
      ADD_TAGS: ['iframe', 'pre', 'code', 'div', 'button'],
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled', 'onclick', 'data-code', 'data-language']
    });

    // 缓存结果，限制缓存大小
    if (messageRenderCache.value.size > 200) {
      // 清理最旧的缓存项
      const firstKey = messageRenderCache.value.keys().next().value;
      if (firstKey) {
        messageRenderCache.value.delete(firstKey);
      }
    }
    messageRenderCache.value.set(cacheKey, result);
    
    // 在下一个 tick 中处理代码块UI增强
    nextTick(() => {
      enhanceCodeBlocks()
    })
    
    return result;
  } catch (err) {
    console.error('Markdown渲染错误:', err);
    const errorMessage = err instanceof Error ? err.message : String(err);
    return `<div class="text-error">Markdown渲染错误: ${errorMessage}</div>
            <pre>${DOMPurify.sanitize(content)}</pre>`;
  }
}

// 增强代码块UI的函数
function enhanceCodeBlocks() {
  // 查找所有包含language-类的code元素，以及没有language-类的pre>code元素
  const codeElements = document.querySelectorAll('.chat-bubble code[class*="language-"], .chat-bubble pre > code:not([class*="language-"])')
  
  codeElements.forEach((codeElement) => {
    const pre = codeElement.closest('pre')
    if (!pre) return
    
    // 避免重复处理
    if (pre.closest('.code-block-container')) {
      return
    }

    // 获取语言类型
    const classNames = codeElement.className.split(' ')
    const langClass = classNames.find(cls => cls.startsWith('language-'))
    const lang = langClass ? langClass.replace('language-', '') : 'plaintext'

    // 如果没有指定语言，为code元素添加language-plaintext类
    if (!langClass) {
      codeElement.classList.add('language-plaintext')
    }

    // 获取原始代码内容
    const codeText = codeElement.textContent || ''

    // 创建容器
    const container = document.createElement('div')
    container.className = 'code-block-container'

    // 创建头部
    const header = document.createElement('div')
    header.className = 'code-block-header'
    header.innerHTML = `
      <span class="code-language">${lang}</span>
      <button class="copy-code-btn" data-code="${encodeURIComponent(codeText)}" title="复制代码">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
        </svg>
      </button>
    `

    // 将pre元素包装到容器中
    const parent = pre.parentNode
    if (parent) {
      parent.insertBefore(container, pre)
      container.appendChild(header)
      container.appendChild(pre)
    }
  })
}

// 设置代码复制功能
function setupCodeCopyFeature() {
  // 使用事件委托监听所有代码复制按钮的点击
  document.addEventListener('click', async (event) => {
    const target = event.target as HTMLElement
    const copyButton = target.closest('.copy-code-btn') as HTMLButtonElement

    if (copyButton) {
      try {
        const codeData = copyButton.getAttribute('data-code')
        if (!codeData) return

        const codeText = decodeURIComponent(codeData)
        await navigator.clipboard.writeText(codeText)

        // 显示复制成功状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        `
        copyButton.classList.add('text-success')
        copyButton.disabled = true

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-success')
          copyButton.disabled = false
        }, 2000)
      } catch (error) {
        console.error('复制代码失败:', error)

        // 显示复制失败状态
        const originalHTML = copyButton.innerHTML
        copyButton.innerHTML = `
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        `
        copyButton.classList.add('text-error')

        // 2秒后恢复原始状态
        setTimeout(() => {
          copyButton.innerHTML = originalHTML
          copyButton.classList.remove('text-error')
        }, 2000)
      }
    }
  })
}

// 确保消息区域滚动到底部
const scrollToBottom = () => {
  if (messagesContainer.value) {
    const container = messagesContainer.value;
    // 使用平滑滚动效果
    container.scrollTo({
      top: container.scrollHeight,
      behavior: 'smooth'
    });
  }
};

// 生成唯一ID
const generateUniqueId = () => {
  return Date.now().toString() + Math.random().toString(36).substring(2, 15)
}

// 清空当前对话消息
const clearMessages = async () => {
  if (!activeConversationId.value) return;
  const newId = await createAIConversation(selectedModel.value, "新对话") as string
  await loadConversations()
  activeConversationId.value = newId
  await loadMessages(String(newId))
}

// 格式化时间
const formatTime = (timestamp: number): string => {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 格式化日期
const formatDate = (timestamp: number): string => {
  return new Date(timestamp).toLocaleString('zh-CN', {
    month: 'numeric',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 修改后端返回的有序对话列表计算属性
const orderedConversations = computed(() => {
  // 过滤有效对话并按更新时间排序

  return conversations.value.filter(c => c && c.id).sort((a, b) => {
    const timeA = a.updated_at || 0;
    const timeB = b.updated_at || 0;
    return timeB - timeA; // 降序排列，最新的在前面
  });
})

// 获取模型名称
const getModelNameById = (modelId: string): string => {
  const model = availableModels.find(m => m.id === modelId)
  return model ? model.name : modelId
}

// 获取选中的模型名称
const getSelectedModelName = () => {
  if (!selectedModel.value) return ''
  const model = availableModels.find(m => m.id === selectedModel.value)
  return model ? model.name : ''
}

// 打开对话列表抽屉
async function openConversationsList() {
  showConversationsDrawer.value = true // 先显示抽屉，同时显示加载状态
  isLoadingConversations.value = true // 设置为加载中
  try {
    await loadConversations() // 加载对话
    console.log('对话列表加载完成:', conversations.value.length)
  } catch (error) {
    console.error('加载对话列表失败:', error)
  } finally {
    isLoadingConversations.value = false // 无论成功失败都完成加载
  }
}

// 组件挂载时加载
onMounted(async () => {
  console.log('组件挂载，开始加载数据...');

  // 加载模型配置
  await loadModelSuggestions();

  // 设置流式输出监听
  await setupStreamListeners();

  // 加载API密钥配置
  await checkApiKey();

  // 加载角色数据
  await loadRoles();

  // 加载所有对话
  await loadConversations();

  // 设置代码复制功能
  setupCodeCopyFeature()

  // 加载分类、标签和笔记数据（用于保存笔记和引用笔记）
  await Promise.all([
    tipsStore.fetchAllCategories(),
    tipsStore.fetchAllTags(),
    tipsStore.fetchAllTips() // 加载所有笔记数据
  ]);

  console.log('加载的笔记数量:', tipsStore.tips.length);

  // 检查是否需要自动创建新对话
  const isFirstVisit = localStorage.getItem('ai-assistant-visited') !== 'true';
  if (isFirstVisit && conversations.value.length === 0) {
    console.log('首次访问且无对话，自动创建新对话');
    await createNewConversation();
    localStorage.setItem('ai-assistant-visited', 'true');
  }

  // 滚动到最新消息
  nextTick(() => {
    scrollToBottom();
  });

  console.log('组件挂载完成，数据加载完毕');
});

// 在组件卸载前保存笔记状态
onBeforeUnmount(() => {
  localStorage.setItem('ai-show-note-panel', showNotePanel.value.toString())
  localStorage.setItem('ai-note-title', noteTitle.value)
  localStorage.setItem('ai-note-content', noteContent.value)
  
  // 清理性能优化相关的缓存
  messageRenderCache.value.clear()
  formatMessageDebounced.value.forEach(timeout => clearTimeout(timeout))
  formatMessageDebounced.value.clear()
})

// 打开API设置对话框
const openApiSettings = async () => {
  editingModel.value = selectedModel.value || availableModels[0].id

  try {
    // 获取当前选择的模型的API密钥
    const result = await invoke('get_api_key', { modelId: editingModel.value })
    apiKey.value = result as string

    // 获取自定义模型名称
    const modelNameResult = await invoke('get_model_name_config', { modelId: editingModel.value })
    customModelName.value = modelNameResult as string

    // 获取max_tokens配置
    try {
      const maxTokensResult = await invoke('get_max_tokens_config', { modelId: editingModel.value })
      maxTokens.value = maxTokensResult as number || getDefaultMaxTokens(editingModel.value)
    } catch (error) {
      console.warn('获取max_tokens配置失败，使用默认值:', error)
      maxTokens.value = getDefaultMaxTokens(editingModel.value)
    }

    // 如果是自定义模型，获取端点
    if (editingModel.value === 'custom') {
      const endpoint = await invoke('get_api_endpoint')
      apiEndpoint.value = endpoint as string
    }
  } catch (error) {
    console.error('获取API配置失败:', error)
    apiKey.value = ''
    customModelName.value = ''
    apiEndpoint.value = ''
    maxTokens.value = getDefaultMaxTokens(editingModel.value)
  }

  // 初始化模型建议
  filterModelSuggestions()

  apiSettingsModal.value?.showModal()
}

// 关闭API设置对话框
const closeApiSettings = () => {
  showApiKey.value = false
  apiSettingsModal.value?.close()
}

// 保存API设置
const saveApiSettings = async () => {
  try {
    await invoke('save_api_key', {
      modelId: editingModel.value,
      apiKey: apiKey.value
    })

    // 保存自定义模型名称
    await invoke('save_model_name', {
      modelId: editingModel.value,
      modelName: customModelName.value
    })

    // 保存max_tokens配置
    await invoke('save_max_tokens_config', {
      modelId: editingModel.value,
      maxTokens: maxTokens.value
    })

    if (editingModel.value === 'custom' && apiEndpoint.value) {
      await invoke('save_api_endpoint', { endpoint: apiEndpoint.value })
    }

    // 更新当前选择的模型
    selectedModel.value = editingModel.value

    // 检查API密钥
    await checkApiKey()

    closeApiSettings()
  } catch (error) {
    console.error('保存API配置失败:', error)
  }
}

// 将内容复制到剪贴板
const copyToClipboard = async (content: string) => {
  try {
    await navigator.clipboard.writeText(content)
  } catch (error) {
    console.error('复制到剪贴板失败:', error)
  }
}

// 处理Enter键事件
const handleEnterKey = (e: KeyboardEvent) => {
  // 如果按下Shift+Enter，则插入换行符
  if (e.shiftKey) {
    return;
  }

  // 如果正在流式输出，则取消生成
  if (isStreaming.value) {
    cancelGeneration();
    return;
  }

  // 检查是否可以发送消息
  if (!userInput.value.trim() || !selectedModel.value || !hasApiKey.value) {
    return;
  }

  // 发送消息
  sendMessage();
};

// 取消内容生成
const cancelGeneration = async () => {
  if (!currentStreamingId.value) return;

  try {
    // 通知后端取消生成
    await invoke('cancel_ai_generation', {
      streamId: currentStreamingId.value
    });

    // 如果已经有一些内容，将其添加为完整消息
    if (streamingContent.value.trim()) {
      await addAIMessage(activeConversationId.value, 'assistant', streamingContent.value + ' [已取消]')
      await loadMessages(activeConversationId.value)
    }

    // 重置状态
    isStreaming.value = false;
    streamingContent.value = '';
    currentStreamingId.value = null;
    isLoading.value = false;
  } catch (error) {
    console.error('取消生成失败:', error);
  }
};

// 编辑对话标题
const editConversationTitle = (conversationId: string) => {
  const conversation = conversations.value.find(c => c.id === conversationId)
  if (conversation) {
    editingTitle.value = conversation.title
    editingConversationId.value = conversationId
    editTitleModal.value?.showModal()
  }
}

// 关闭编辑标题对话框
const closeEditTitleModal = () => {
  editTitleModal.value?.close()
}

// 保存编辑后的标题
const saveEditedTitle = async () => {
  if (editingTitle.value.trim() && editingConversationId.value) {
    await updateAIConversationTitle(editingConversationId.value, editingTitle.value.trim())
    await loadConversations()
    closeEditTitleModal()
  }
}

// 确认删除对话
const confirmDeleteConversation = (conversationId: string) => {
  deletingConversationId.value = conversationId
  deleteConfirmModal.value?.showModal()
}

// 关闭删除确认对话框
const closeDeleteConfirmModal = () => {
  deleteConfirmModal.value?.close()
}

// 确认删除
const confirmDelete = async () => {
  if (deletingConversationId.value) {
    await deleteConversation(deletingConversationId.value)
    closeDeleteConfirmModal()
  }
}

// 添加右键菜单相关状态
const showContextMenu = ref(false)
const contextMenuPos = ref({ x: 0, y: 0 })
const selectedText = ref('')

// 处理文本选择
const handleTextSelection = () => {
  const selection = window.getSelection()
  if (selection && selection.toString().trim()) {
    selectedText.value = selection.toString().trim()
  } else {
    selectedText.value = ''
  }
}

// 处理右键菜单
const handleContextMenu = (e: MouseEvent) => {
  // 如果有选中的文本，显示右键菜单
  if (selectedText.value) {
    // 防止默认菜单显示
    e.preventDefault()

    // 设置菜单位置
    contextMenuPos.value = {
      x: e.clientX,
      y: e.clientY
    }

    // 显示自定义菜单
    showContextMenu.value = true
  }
}

// 隐藏右键菜单
const hideContextMenu = (event?: MouseEvent) => {
  showContextMenu.value = false

  // 如果点击的不是笔记选择器内部，则隐藏笔记选择器
  if (showNoteSelector.value && event) {
    const target = event.target as HTMLElement
    const noteSelector = document.querySelector('.note-selector')
    if (noteSelector && !noteSelector.contains(target)) {
      hideNoteSelector()
    }
  } else if (showNoteSelector.value && !event) {
    // 如果没有事件对象，直接隐藏（比如ESC键触发）
    hideNoteSelector()
  }
}

// 将内容添加到笔记
const addToNote = (content: string) => {
  showNotePanel.value = true

  // 如果笔记内容为空，直接设置；否则添加到末尾
  if (!noteContent.value) {
    noteContent.value = content
  } else {
    noteContent.value += '\n\n' + content
  }

  // 如果标题为空且内容不为空，尝试生成一个标题
  if (!noteTitle.value && content) {
    // 尝试从内容的第一行或前50个字符生成标题
    const firstLine = content.split('\n')[0].trim()
    noteTitle.value = firstLine.length > 50 ? firstLine.substring(0, 50) + '...' : firstLine
  }
}

// 添加选中文本到笔记
const addSelectedTextToNote = () => {
  if (selectedText.value) {
    addToNote(selectedText.value)
    hideContextMenu()
  }
}

// 复制选中文本
const copySelectedText = async () => {
  if (selectedText.value) {
    try {
      await navigator.clipboard.writeText(selectedText.value)
      hideContextMenu()
    } catch (error) {
      console.error('复制到剪贴板失败:', error)
    }
  }
}

// 导出对话
const exportMessages = async () => {
  if (!activeConversationId.value) return

  // 确保消息已加载
  await loadMessages(activeConversationId.value)

  const activeConv = conversations.value.find(c => c.id === activeConversationId.value)
  if (!activeConv || messages.value.length === 0) return

  const modelName = getModelNameById(activeConv.model)
  const exportText = messages.value
    .map((msg: any) => `${msg.role === 'user' ? '用户' : modelName}: ${msg.content}`)
    .join('\n\n')

  // 创建下载链接
  const blob = new Blob([exportText], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `ai-conversation-${activeConv.title}-${new Date().toISOString().slice(0, 10)}.txt`
  document.body.appendChild(a)
  a.click()

  // 清理
  setTimeout(() => {
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }, 0)
}

// 保存笔记
const saveNoteAsTip = async () => {
  if (!noteTitle.value.trim() || isNoteSaving.value) {
    return
  }

  isNoteSaving.value = true

  try {
    // 获取第一个分类作为默认分类
    const defaultCategoryId = tipsStore.categories.length > 0 ? tipsStore.categories[0].id : undefined

    // 创建新的tip
    const newTip = {
      id: undefined as string | undefined,
      title: noteTitle.value,
      content: noteContent.value,
      tip_type: 'markdown', // 默认为markdown
      language: undefined, // 不需要单独指定语言
      category_id: defaultCategoryId,
      tags: []
    }

    // 保存到数据库
    await tipsStore.saveTip(newTip)

    // 显示成功提示
    showSaveSuccess.value = true
    setTimeout(() => {
      showSaveSuccess.value = false
    }, 3000)

    // 清空笔记内容
    noteTitle.value = ''
    noteContent.value = ''
  } catch (error) {
    console.error('保存笔记失败:', error)
  } finally {
    isNoteSaving.value = false
  }
}

// 添加getDefaultModelName函数，避免API设置时的错误
const getDefaultModelName = (modelId: string): string => {
  switch (modelId) {
    case 'chatgpt': return 'gpt-3.5-turbo'
    case 'gemini': return 'gemini-2.0-flash'
    case 'deepseek': return 'deepseek-chat'
    case 'claude': return 'claude-3.5-sonnet'
    case 'qwen': return 'qwen-max'
    case 'doubao': return 'doubao-1.5-pro-32k'
    case 'grok': return 'grok-beta'
    case 'custom': return 'gpt-3.5-turbo'
    default: return 'gpt-3.5-turbo'
  }
}

// 添加缓存相关的生命周期钩子
onActivated(() => {
  console.log('AIAssistant组件被激活');
});

onDeactivated(() => {
  console.log('AIAssistant组件被停用');
  // 停止任何可能的网络请求或流式传输
  if (isStreaming.value) {
    cancelGeneration();
  }
});

// 文件上传相关函数
const triggerFileUpload = () => {
  fileInput.value?.click()
}

const handleFileUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files) return

  Array.from(files).forEach(file => {
    // 检查文件大小（限制为10MB）
    if (file.size > 10 * 1024 * 1024) {
      alert(`文件 ${file.name} 超过10MB限制`)
      return
    }

    const fileId = generateUniqueId()
    const fileItem: {
      id: string
      name: string
      type: string
      size: number
      file: File
      preview?: string
    } = {
      id: fileId,
      name: file.name,
      type: file.type,
      size: file.size,
      file: file
    }

    // 如果是图片，生成预览
    if (file.type.startsWith('image/')) {
      const reader = new FileReader()
      reader.onload = (e) => {
        fileItem.preview = e.target?.result as string
        uploadedFiles.value.push(fileItem)
      }
      reader.readAsDataURL(file)
    } else {
      uploadedFiles.value.push(fileItem)
    }
  })

  // 清空input
  target.value = ''
}

const removeFile = (index: number) => {
  uploadedFiles.value.splice(index, 1)
}

const clearAllFiles = () => {
  uploadedFiles.value = []
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const previewImage = (url: string) => {
  // 创建图片预览模态框
  const modal = document.createElement('div')
  modal.className = 'fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50 cursor-pointer'
  modal.onclick = () => document.body.removeChild(modal)

  const img = document.createElement('img')
  img.src = url
  img.className = 'max-w-full max-h-full object-contain'

  modal.appendChild(img)
  document.body.appendChild(modal)
}

const downloadFile = (attachment: any) => {
  // 创建下载链接
  const a = document.createElement('a')
  a.href = attachment.url
  a.download = attachment.name
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

// 角色管理相关
const roles = ref<any[]>([])
const showRoleManager = ref(false)
const roleManagerModal = ref<HTMLDialogElement | null>(null)
const showRoleForm = ref(false)
const editingRole = ref<any>(null)
const newRoleName = ref('')
const newRoleDescription = ref('')
const isLoadingRoles = ref(false)

// 角色选择相关
const selectedRole = ref<any>(null)
const showRoleSelector = ref(false)
const roleSelectorPosition = ref({ x: 0, y: 0 })

// 角色管理方法
const openRoleManager = async () => {
  showRoleManager.value = true
  roleManagerModal.value?.showModal()
  await loadRoles()
}

const closeRoleManager = () => {
  showRoleManager.value = false
  showRoleForm.value = false
  editingRole.value = null
  newRoleName.value = ''
  newRoleDescription.value = ''
  roleManagerModal.value?.close()
}

const loadRoles = async () => {
  isLoadingRoles.value = true
  try {
    const result = await listAIRoles()
    roles.value = Array.isArray(result) ? result : []
  } catch (error) {
    console.error('加载角色列表失败:', error)
    roles.value = []
  } finally {
    isLoadingRoles.value = false
  }
}

const openRoleForm = (role?: any) => {
  if (role) {
    editingRole.value = role
    newRoleName.value = role.name
    newRoleDescription.value = role.description
  } else {
    editingRole.value = null
    newRoleName.value = ''
    newRoleDescription.value = ''
  }
  showRoleForm.value = true
}

const closeRoleForm = () => {
  showRoleForm.value = false
  editingRole.value = null
  newRoleName.value = ''
  newRoleDescription.value = ''
}

const saveRole = async () => {
  if (!newRoleName.value.trim() || !newRoleDescription.value.trim()) {
    return
  }

  try {
    if (editingRole.value) {
      // 更新角色
      await updateAIRole(editingRole.value.id, newRoleName.value.trim(), newRoleDescription.value.trim())
    } else {
      // 创建新角色
      await createAIRole(newRoleName.value.trim(), newRoleDescription.value.trim())
    }

    await loadRoles()
    closeRoleForm()
  } catch (error) {
    console.error('保存角色失败:', error)
  }
}

const confirmDeleteRole = async (roleId: string) => {
  if (confirm('确定要删除这个角色吗？')) {
    try {
      await deleteAIRole(roleId)
      await loadRoles()
    } catch (error) {
      console.error('删除角色失败:', error)
    }
  }
}

// 角色选择相关方法
const handleInputKeydown = (event: KeyboardEvent) => {
  // 检测@符号用于角色选择
  if (event.key === '@' && !showRoleSelector.value) {
    // 显示角色选择器
    const input = event.target as HTMLTextAreaElement
    const rect = input.getBoundingClientRect()
    roleSelectorPosition.value = {
      x: rect.left,
      y: rect.bottom
    }
    showRoleSelector.value = true
    loadRoles()
    event.preventDefault()
    return
  }

  // 检测#符号用于笔记选择
  if (event.key === '#' && !showNoteSelector.value) {
    // 显示笔记选择器
    const input = event.target as HTMLTextAreaElement
    // @ts-ignore - 保留此变量供未来使用
    const rect = input.getBoundingClientRect()
    noteSelectorPosition.value = {
      x: 0, // 相对于输入框容器的位置
      y: 0
    }
    showNoteSelector.value = true
    loadNotesForSelector()
    event.preventDefault()
    return
  }

  // ESC键关闭选择器
  if (event.key === 'Escape') {
    if (showRoleSelector.value) {
      showRoleSelector.value = false
    }
    if (showNoteSelector.value) {
      hideNoteSelector()
    }
    event.preventDefault()
    return
  }
}

const selectRole = (role: any) => {
  selectedRole.value = role
  showRoleSelector.value = false

  // 移除输入框中的@符号
  userInput.value = userInput.value.replace(/@$/, '')
}

const clearSelectedRole = () => {
  selectedRole.value = null
}

// 笔记选择相关
const showNoteSelector = ref(false)
const noteSelectorPosition = ref({ x: 0, y: 0 })
const noteSearchQuery = ref('')
const selectedNotes = ref<any[]>([])
const filteredNotes = ref<any[]>([])
const messageTextarea = ref<HTMLTextAreaElement | null>(null)
const noteSearchInput = ref<HTMLInputElement | null>(null)

// 加载笔记用于选择器
const loadNotesForSelector = async () => {
  try {
    // 确保tips数据已加载
    if (tipsStore.tips.length === 0) {
      await tipsStore.fetchAllTips()
    }

    // 按添加时间排序（最新的在前）并限制为10篇
    const sortedTips = [...tipsStore.tips]
      .sort((a, b) => {
        const timeA = a.updated_at || a.created_at || 0
        const timeB = b.updated_at || b.created_at || 0
        return timeB - timeA // 降序排列，最新的在前
      })
      .slice(0, 10) // 只取前10篇

    filteredNotes.value = sortedTips
    noteSearchQuery.value = ''
    console.log('加载笔记列表:', filteredNotes.value.length, '总笔记数:', tipsStore.tips.length)

    // 延迟聚焦到搜索框
    await nextTick()
    setTimeout(() => {
      noteSearchInput.value?.focus()
    }, 100)
  } catch (error) {
    console.error('加载笔记失败:', error)
    filteredNotes.value = []
  }
}

// 选择笔记
const selectNote = (note: any) => {
  // 在输入框当前光标位置插入 #标题
  const noteReference = `#${note.title} `

  // 将选中的笔记添加到列表中（用于发送时处理）
  const existingIndex = selectedNotes.value.findIndex(n => n.id === note.id)
  if (existingIndex < 0) {
    selectedNotes.value.push(note)
  }

  // 在输入框中插入笔记引用
  const currentInput = userInput.value
  // 移除末尾的#号（如果存在）
  const cleanInput = currentInput.endsWith('#') ? currentInput.slice(0, -1) : currentInput
  userInput.value = cleanInput + noteReference

  console.log('选中笔记:', note.title, '插入引用:', noteReference)

  // 关闭选择器
  hideNoteSelector()

  // 聚焦回输入框
  setTimeout(() => {
    messageTextarea.value?.focus()
  }, 100)
}

// 选择第一个笔记
const selectFirstNote = () => {
  if (filteredNotes.value.length > 0) {
    selectNote(filteredNotes.value[0])
    // 选择第一个后关闭选择器
    hideNoteSelector()
  }
}

// 隐藏笔记选择器
const hideNoteSelector = () => {
  showNoteSelector.value = false
  noteSearchQuery.value = ''
  // 重新聚焦到输入框
  setTimeout(() => {
    messageTextarea.value?.focus()
  }, 100)
}

// 移除选中的笔记
// @ts-ignore - 保留此函数供未来使用
const removeSelectedNote = (noteId: string) => {
  const index = selectedNotes.value.findIndex(n => n.id === noteId)
  if (index >= 0) {
    selectedNotes.value.splice(index, 1)
  }
}

// 清空所有选中的笔记
const clearAllSelectedNotes = () => {
  selectedNotes.value = []
  noteSearchQuery.value = ''
}

// 获取输入框占位符
const getInputPlaceholder = () => {
  if (selectedRole.value) {
    return `以 ${selectedRole.value.name} 的身份与您对话...输入#可引用笔记`
  } else {
    return '输入您的问题或指令，支持上传图片和文档...输入#可引用笔记'
  }
}

// 监听笔记搜索查询
watch(noteSearchQuery, (newQuery) => {
  if (newQuery.trim()) {
    // 搜索匹配的笔记
    const searchResults = tipsStore.tips.filter(note =>
      note.title.toLowerCase().includes(newQuery.toLowerCase()) ||
      note.content.toLowerCase().includes(newQuery.toLowerCase())
    )

    // 按时间排序并限制为10篇
    filteredNotes.value = searchResults
      .sort((a, b) => {
        const timeA = a.updated_at || a.created_at || 0
        const timeB = b.updated_at || b.created_at || 0
        return timeB - timeA // 降序排列，最新的在前
      })
      .slice(0, 10) // 只取前10篇
  } else {
    // 没有搜索词时，显示最新的10篇笔记
    const sortedTips = [...tipsStore.tips]
      .sort((a, b) => {
        const timeA = a.updated_at || a.created_at || 0
        const timeB = b.updated_at || b.created_at || 0
        return timeB - timeA // 降序排列，最新的在前
      })
      .slice(0, 10) // 只取前10篇

    filteredNotes.value = sortedTips
  }
})

// 监听输入框内容变化，检测#号
watch(userInput, (newValue, oldValue) => {
  console.log('输入变化:', { newValue, oldValue, endsWithHash: newValue.endsWith('#') })

  // 检查是否刚输入了#号
  if (newValue.endsWith('#') && !oldValue.endsWith('#') && !showNoteSelector.value) {
    console.log('检测到#号输入，显示笔记选择器')
    // 显示笔记选择器
    showNoteSelector.value = true
    loadNotesForSelector()
  }
})

// 监听笔记选择器显示状态
watch(showNoteSelector, (newValue) => {
  console.log('笔记选择器显示状态:', newValue)
  if (newValue) {
    console.log('当前笔记数量:', tipsStore.tips.length)
    console.log('过滤后笔记数量:', filteredNotes.value.length)
  }
})

// 笔记详情显示相关
const showNoteDetailModal = ref(false)
const noteDetailModal = ref<HTMLDialogElement | null>(null)
const currentNoteDetail = ref<any>(null)

// 显示笔记详情
const showNoteDetail = (note: any) => {
  currentNoteDetail.value = note
  showNoteDetailModal.value = true
  noteDetailModal.value?.showModal()
}

// 关闭笔记详情
const closeNoteDetail = () => {
  showNoteDetailModal.value = false
  currentNoteDetail.value = null
  noteDetailModal.value?.close()
}

// 保存笔记详情
// @ts-ignore - 保留此函数供未来使用
const saveNoteDetail = async () => {
  if (!currentNoteDetail.value) return

  try {
    // 更新笔记内容
    await tipsStore.saveTip({
      id: currentNoteDetail.value.id,
      title: currentNoteDetail.value.title,
      content: currentNoteDetail.value.content,
      tip_type: currentNoteDetail.value.tip_type,
      category_id: currentNoteDetail.value.category_id,
      tags: currentNoteDetail.value.tags || []
    })

    closeNoteDetail()
  } catch (error) {
    console.error('保存笔记失败:', error)
  }
}

// 复制笔记内容
const copyNoteContent = async () => {
  try {
    await navigator.clipboard.writeText(currentNoteDetail.value.content)
  } catch (error) {
    console.error('复制笔记内容失败:', error)
  }
}
</script>

<style scoped>
.ai-assistant-page {
  max-width: 100%;
  margin: 0 auto;
}

/* 确保头像尺寸和样式一致 */
.chat-image img {
  width: 40px;
  height: 40px;
  object-fit: cover;
}

/* 确保聊天气泡中的文本自动换行 */
:deep(.chat-bubble) {
  word-wrap: break-word;
  overflow-wrap: break-word;
  max-width: 100%;
}

/* 深度选择器，使得Markdown内容样式正确 */
:deep(.chat-bubble pre) {
  background-color: rgba(0, 0, 0, 0.1);
  padding: 1rem;
  border-radius: 0.5rem;
  overflow-x: auto;
}

:deep(.chat-bubble code) {
  background-color: rgba(0, 0, 0, 0.1);
  padding: 0.2rem 0.4rem;
  border-radius: 0.3rem;
}

:deep(.chat-bubble p) {
  margin-bottom: 0.5rem;
}

:deep(.chat-bubble ul, .chat-bubble ol) {
  padding-left: 1.5rem;
  margin-bottom: 0.5rem;
}

/* 打字机效果 */
@keyframes blink {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0;
  }
}

.typing-cursor {
  display: inline-block;
  width: 0.5em;
  height: 1em;
  background-color: currentColor;
  margin-left: 0.1em;
  vertical-align: text-bottom;
  animation: blink 1s step-end infinite;
}

/* 抽屉动画和样式 */
.drawer-side {
  transition: transform 0.3s ease;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  overflow-y: auto;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
}

.conversations-drawer {
  display: flex;
}

/* 确保对话列表项有足够的间距和样式 */
.drawer-side .flex-1.overflow-y-auto {
  padding-bottom: 20px;
  /* 添加底部间距 */
}

/* 消息容器平滑滚动 */
.chat-container .flex-grow {
  scroll-behavior: smooth;
}

/* 自定义右键菜单样式 */
.custom-context-menu {
  min-width: 180px;
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.custom-context-menu ul li button {
  padding: 0.5rem 1rem;
  transition: background-color 0.2s;
  width: 100%;
  text-align: left;
}

.custom-context-menu ul li button:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

/* 过渡动画 fade+滑动 */
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.35s cubic-bezier(.4, 0, .2, 1);
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(40px);
}

.fade-slide-leave-from,
.fade-slide-enter-to {
  opacity: 1;
  transform: translateX(0);
}

/* 文件上传相关样式 */
.message-input-container {
  transition: all 0.2s ease;
}

.message-input-container:focus-within {
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
}

.uploaded-files-preview {
  border: 2px dashed hsl(var(--base-300));
  transition: all 0.2s ease;
}

.file-preview-item {
  transition: transform 0.2s ease;
}

.file-preview-item:hover {
  transform: scale(1.02);
}

.attachment-item {
  transition: all 0.2s ease;
}

.image-attachment img {
  transition: transform 0.2s ease;
}

.image-attachment img:hover {
  transform: scale(1.05);
}

.document-attachment {
  transition: all 0.2s ease;
}

.document-attachment:hover {
  background-color: hsl(var(--base-200));
}

/* 工具栏样式 */
.input-toolbar {
  background: linear-gradient(to right, hsl(var(--base-100)), hsl(var(--base-50)));
}

/* 文件上传按钮动画 */
.file-upload-container button:hover svg {
  transform: rotate(5deg) scale(1.1);
  transition: transform 0.2s ease;
}

/* 模型建议下拉框样式 */
.model-suggestions-dropdown {
  border: 1px solid hsl(var(--base-300));
  background: hsl(var(--base-100));
  backdrop-filter: blur(8px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.model-suggestion-item {
  transition: all 0.2s ease;
}

.model-suggestion-item:hover {
  background-color: hsl(var(--base-200));
  transform: translateX(2px);
}

.model-suggestion-item .font-medium {
  color: hsl(var(--base-content));
}

.model-suggestion-item .text-sm {
  color: hsl(var(--base-content) / 0.7);
}

/* 暗色主题下的模型建议样式优化 - 这是AIAssistant特有的 */
[data-theme="dark"] .model-suggestions-dropdown,
[data-theme="night"] .model-suggestions-dropdown,
[data-theme="black"] .model-suggestions-dropdown,
[data-theme="dracula"] .model-suggestions-dropdown,
[data-theme="halloween"] .model-suggestions-dropdown,
[data-theme="business"] .model-suggestions-dropdown,
[data-theme="luxury"] .model-suggestions-dropdown,
[data-theme="coffee"] .model-suggestions-dropdown,
[data-theme="forest"] .model-suggestions-dropdown,
[data-theme="synthwave"] .model-suggestions-dropdown {
  background-color: rgba(0, 0, 0, 0.8) !important;
  border-color: rgba(255, 255, 255, 0.2) !important;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5) !important;
}

[data-theme="dark"] .model-suggestion-item:hover,
[data-theme="night"] .model-suggestion-item:hover,
[data-theme="black"] .model-suggestion-item:hover,
[data-theme="dracula"] .model-suggestion-item:hover,
[data-theme="halloween"] .model-suggestion-item:hover,
[data-theme="business"] .model-suggestion-item:hover,
[data-theme="luxury"] .model-suggestion-item:hover,
[data-theme="coffee"] .model-suggestion-item:hover,
[data-theme="forest"] .model-suggestion-item:hover,
[data-theme="synthwave"] .model-suggestion-item:hover {
  background-color: rgba(255, 255, 255, 0.1) !important;
}

/* AI聊天界面特有的样式 */
.chat-bubble-ai {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(147, 51, 234, 0.1));
  backdrop-filter: blur(8px);
}

.chat-bubble-user {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(6, 182, 212, 0.1));
  backdrop-filter: blur(8px);
}

/* 流式传输动画 */
.streaming-indicator {
  animation: pulse 2s infinite;
}

@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}

/* 模型切换按钮的特殊样式 */
.model-switch-btn {
  transition: all 0.3s ease;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1), transparent);
}

.model-switch-btn:hover {
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.2), rgba(255, 255, 255, 0.1));
  transform: translateY(-1px);
}

/* 角色管理相关样式 */
.role-card {
  transition: all 0.2s ease;
  cursor: pointer;
}

.role-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.role-badge {
  transition: all 0.2s ease;
  backdrop-filter: blur(8px);
}

.role-badge button {
  transition: all 0.2s ease;
}

.role-badge button:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.role-prompt-container {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.05), rgba(147, 51, 234, 0.05));
  backdrop-filter: blur(8px);
  transition: all 0.2s ease;
}

.role-prompt-container:hover {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(147, 51, 234, 0.1));
}

/* 角色选择器样式 */
.role-selector {
  backdrop-filter: blur(12px);
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.role-selector-item {
  transition: all 0.2s ease;
}

.role-selector-item:hover {
  background-color: rgba(59, 130, 246, 0.1);
  transform: translateX(2px);
}

.note-selector {
  position: absolute;
  z-index: 50;
  width: 100%;
  /* background: hsl(var(--base-100)); */
  border: 1px solid hsl(var(--base-300));
  border-radius: 0.5rem;
  box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  max-height: 60vh;
  overflow-y: auto;
  /* 确保在输入框上方显示 */
  bottom: 100%;
  margin-bottom: 8px;
}

.note-selector-item {
  padding: 0.75rem 1rem;
  transition: all 0.2s ease;
  border-bottom: 1px solid hsl(var(--base-300));
  cursor: pointer;
  /* background: hsl(var(--base-100)); */
}

.note-selector-item:hover {
  background-color: hsl(var(--base-200));
  transform: translateX(2px);
}

.note-selector-item:last-child {
  border-bottom: none;
}

.note-selector input[type="text"] {
  background: hsl(var(--base-100));
  border: 1px solid hsl(var(--base-300));
}

.note-selector input[type="text"]:focus {
  outline: none;
  border-color: hsl(var(--primary));
  box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
}

.note-selector .p-2 {
  background: hsl(var(--base-100));
}

/* 暗色主题下的笔记选择器优化 */
[data-theme="dark"] .note-selector,
[data-theme="night"] .note-selector,
[data-theme="black"] .note-selector,
[data-theme="dracula"] .note-selector,
[data-theme="halloween"] .note-selector,
[data-theme="business"] .note-selector,
[data-theme="luxury"] .note-selector,
[data-theme="coffee"] .note-selector,
[data-theme="forest"] .note-selector,
[data-theme="synthwave"] .note-selector {
  background-color: hsl(var(--base-100)) !important;
  border-color: hsl(var(--base-300)) !important;
  box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.5), 0 10px 10px -5px rgba(0, 0, 0, 0.3) !important;
}

[data-theme="dark"] .note-selector-item,
[data-theme="night"] .note-selector-item,
[data-theme="black"] .note-selector-item,
[data-theme="dracula"] .note-selector-item,
[data-theme="halloween"] .note-selector-item,
[data-theme="business"] .note-selector-item,
[data-theme="luxury"] .note-selector-item,
[data-theme="coffee"] .note-selector-item,
[data-theme="forest"] .note-selector-item,
[data-theme="synthwave"] .note-selector-item {
  background-color: hsl(var(--base-100)) !important;
}

[data-theme="dark"] .note-selector-item:hover,
[data-theme="night"] .note-selector-item:hover,
[data-theme="black"] .note-selector-item:hover,
[data-theme="dracula"] .note-selector-item:hover,
[data-theme="halloween"] .note-selector-item:hover,
[data-theme="business"] .note-selector-item:hover,
[data-theme="luxury"] .note-selector-item:hover,
[data-theme="coffee"] .note-selector-item:hover,
[data-theme="forest"] .note-selector-item:hover,
[data-theme="synthwave"] .note-selector-item:hover {
  background-color: hsl(var(--base-200)) !important;
}

[data-theme="dark"] .note-selector .p-2,
[data-theme="night"] .note-selector .p-2,
[data-theme="black"] .note-selector .p-2,
[data-theme="dracula"] .note-selector .p-2,
[data-theme="halloween"] .note-selector .p-2,
[data-theme="business"] .note-selector .p-2,
[data-theme="luxury"] .note-selector .p-2,
[data-theme="coffee"] .note-selector .p-2,
[data-theme="forest"] .note-selector .p-2,
[data-theme="synthwave"] .note-selector .p-2 {
  background: hsl(var(--base-100)) !important;
}

.note-badge {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem;
  background-color: hsl(var(--info));
  color: hsl(var(--base-100));
  border-radius: 9999px;
  font-size: 0.875rem;
}

.note-badge svg {
  width: 1rem;
  height: 1rem;
}

.selected-notes-container {
  margin-bottom: 1rem;
  padding: 0.75rem;
  background-color: hsl(var(--info) / 0.1);
  border: 1px solid hsl(var(--info) / 0.2);
  border-radius: 0.375rem;
}

.selected-notes-container .text-sm {
  color: hsl(var(--base-content));
}

.selected-notes-container .flex {
  justify-content: space-between;
  align-items: center;
}

.selected-notes-container .flex .text-sm {
  margin-left: 0.5rem;
}

.selected-notes-container .flex .ml-auto {
  margin-right: 0.5rem;
}

.selected-notes-container .flex .btn {
  padding: 0.25rem 0.5rem;
  background-color: hsl(var(--info) / 0.2);
  color: hsl(var(--base-100));
  border-radius: 9999px;
  font-size: 0.875rem;
}

.selected-notes-container .flex .btn:hover {
  background-color: hsl(var(--info) / 0.3);
}

/* 引用笔记显示样式 */
.referenced-note-item {
  transition: all 0.2s ease;
}

.referenced-note-item:hover {
  transform: translateX(2px);
}

.referenced-note-item .flex {
  border: 1px solid hsl(var(--info) / 0.3);
  background: linear-gradient(135deg, hsl(var(--info) / 0.05), hsl(var(--info) / 0.1));
}

.referenced-note-item .flex:hover {
  border-color: hsl(var(--info) / 0.5);
  background: linear-gradient(135deg, hsl(var(--info) / 0.1), hsl(var(--info) / 0.2));
  box-shadow: 0 2px 8px hsl(var(--info) / 0.2);
}

/* 暗色主题下的引用笔记样式 */
[data-theme="dark"] .referenced-note-item .flex,
[data-theme="night"] .referenced-note-item .flex,
[data-theme="black"] .referenced-note-item .flex,
[data-theme="dracula"] .referenced-note-item .flex,
[data-theme="halloween"] .referenced-note-item .flex,
[data-theme="business"] .referenced-note-item .flex,
[data-theme="luxury"] .referenced-note-item .flex,
[data-theme="coffee"] .referenced-note-item .flex,
[data-theme="forest"] .referenced-note-item .flex,
[data-theme="synthwave"] .referenced-note-item .flex {
  background: linear-gradient(135deg, hsl(var(--info) / 0.1), hsl(var(--info) / 0.15)) !important;
  border-color: hsl(var(--info) / 0.4) !important;
}

[data-theme="dark"] .referenced-note-item .flex:hover,
[data-theme="night"] .referenced-note-item .flex:hover,
[data-theme="black"] .referenced-note-item .flex:hover,
[data-theme="dracula"] .referenced-note-item .flex:hover,
[data-theme="halloween"] .referenced-note-item .flex:hover,
[data-theme="business"] .referenced-note-item .flex:hover,
[data-theme="luxury"] .referenced-note-item .flex:hover,
[data-theme="coffee"] .referenced-note-item .flex:hover,
[data-theme="forest"] .referenced-note-item .flex:hover,
[data-theme="synthwave"] .referenced-note-item .flex:hover {
  background: linear-gradient(135deg, hsl(var(--info) / 0.15), hsl(var(--info) / 0.25)) !important;
  border-color: hsl(var(--info) / 0.6) !important;
}

/* 大代码块样式 */
.large-code-block {
  margin: 1rem 0;
  border-radius: 0.5rem;
  overflow: hidden;
  border: 1px solid hsl(var(--base-300));
}

.large-code-block .code-warning {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-weight: 500;
  border-bottom: 1px solid hsl(var(--warning) / 0.3);
}

.large-code-block .code-content {
  margin: 0;
  border-radius: 0;
  border: none;
  max-height: 500px;
  overflow-y: auto;
}

.large-code-block .code-content code {
  font-size: 0.875rem;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
}

/* 大代码块在暗色主题下的样式 */
[data-theme="dark"] .large-code-block,
[data-theme="night"] .large-code-block,
[data-theme="black"] .large-code-block,
[data-theme="dracula"] .large-code-block,
[data-theme="halloween"] .large-code-block,
[data-theme="business"] .large-code-block,
[data-theme="luxury"] .large-code-block,
[data-theme="coffee"] .large-code-block,
[data-theme="forest"] .large-code-block,
[data-theme="synthwave"] .large-code-block {
  border-color: hsl(var(--base-300)) !important;
}

/* 代码块滚动条样式 */
.large-code-block .code-content::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.large-code-block .code-content::-webkit-scrollbar-track {
  background: hsl(var(--base-200));
  border-radius: 4px;
}

.large-code-block .code-content::-webkit-scrollbar-thumb {
  background: hsl(var(--base-300));
  border-radius: 4px;
}

.large-code-block .code-content::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--primary));
}

/* 代码块容器样式 */
:deep(.chat-bubble .code-block-container) {
  position: relative;
  margin: 1rem 0;
  border-radius: 0.375rem;
  overflow: hidden;
  background: var(--prism-background, #242424);
  border: 1px solid var(--prism-border, #242424);
}

:deep(.chat-bubble .code-block-header) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  background: var(--prism-header-background, #242424);
  font-size: 0.875rem;
  border-bottom: 1px solid var(--prism-border, #242424);
}

:deep(.chat-bubble .code-language) {
  font-weight: 600;
  color: var(--prism-header-text, #cbc9c9);
  text-transform: uppercase;
  font-size: 0.75rem;
}

:deep(.chat-bubble .copy-code-btn) {
  padding: 0.25rem;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--prism-header-text, #cbc9c9);
}

:deep(.chat-bubble .copy-code-btn:hover) {
  background: var(--prism-button-hover, rgba(0, 0, 0, 0.1));
}

:deep(.chat-bubble .copy-code-btn svg) {
  width: 1rem;
  height: 1rem;
}

/* 代码块容器内的pre元素样式调整 */
:deep(.chat-bubble .code-block-container pre) {
  margin: 0;
  background: transparent;
  border: none;
  border-radius: 0;
}

:deep(.chat-bubble .code-block-container pre code) {
  background: transparent;
  padding: 0;
  border-radius: 0;
  font-size: 0.9em;
  color: inherit;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  display: block;
  width: 100%;
  line-height: 1.5;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  word-break: break-all;
}

/* 暗色主题下的代码块样式 */
[data-theme="dark"] :deep(.chat-bubble .code-block-container),
[data-theme="night"] :deep(.chat-bubble .code-block-container),
[data-theme="black"] :deep(.chat-bubble .code-block-container),
[data-theme="dracula"] :deep(.chat-bubble .code-block-container),
[data-theme="halloween"] :deep(.chat-bubble .code-block-container),
[data-theme="business"] :deep(.chat-bubble .code-block-container),
[data-theme="luxury"] :deep(.chat-bubble .code-block-container),
[data-theme="coffee"] :deep(.chat-bubble .code-block-container),
[data-theme="forest"] :deep(.chat-bubble .code-block-container),
[data-theme="synthwave"] :deep(.chat-bubble .code-block-container) {
  --prism-background: #2d3748;
  --prism-border: #4a5568;
  --prism-header-background: #1a202c;
  --prism-header-text: #a0aec0;
  --prism-button-hover: rgba(255, 255, 255, 0.1);
}

/* 行内代码样式修复 */
:deep(.prose code:not([class*="language-"])) {
  border-radius: 0.25rem;
  padding: 0.125rem 0.25rem;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  background-color: hsl(var(--b3));
  color: hsl(var(--bc));
  border: 1px solid hsl(var(--b3));
}

/* 暗色主题下的行内代码特殊优化 */
[data-theme="dark"] :deep(.prose code:not([class*="language-"])),
[data-theme="night"] :deep(.prose code:not([class*="language-"])),
[data-theme="black"] :deep(.prose code:not([class*="language-"])),
[data-theme="dracula"] :deep(.prose code:not([class*="language-"])),
[data-theme="halloween"] :deep(.prose code:not([class*="language-"])),
[data-theme="business"] :deep(.prose code:not([class*="language-"])),
[data-theme="luxury"] :deep(.prose code:not([class*="language-"])),
[data-theme="coffee"] :deep(.prose code:not([class*="language-"])),
[data-theme="forest"] :deep(.prose code:not([class*="language-"])),
[data-theme="synthwave"] :deep(.prose code:not([class*="language-"])) {
  background-color: rgba(255, 255, 255, 0.1) !important;
  color: rgb(251, 191, 36) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
}

/* 亮色主题下的行内代码优化 */
[data-theme="light"] :deep(.prose code:not([class*="language-"])),
[data-theme="cupcake"] :deep(.prose code:not([class*="language-"])),
[data-theme="bumblebee"] :deep(.prose code:not([class*="language-"])),
[data-theme="emerald"] :deep(.prose code:not([class*="language-"])),
[data-theme="corporate"] :deep(.prose code:not([class*="language-"])),
[data-theme="retro"] :deep(.prose code:not([class*="language-"])),
[data-theme="cyberpunk"] :deep(.prose code:not([class*="language-"])),
[data-theme="valentine"] :deep(.prose code:not([class*="language-"])),
[data-theme="garden"] :deep(.prose code:not([class*="language-"])),
[data-theme="aqua"] :deep(.prose code:not([class*="language-"])),
[data-theme="lofi"] :deep(.prose code:not([class*="language-"])),
[data-theme="pastel"] :deep(.prose code:not([class*="language-"])),
[data-theme="fantasy"] :deep(.prose code:not([class*="language-"])),
[data-theme="wireframe"] :deep(.prose code:not([class*="language-"])),
[data-theme="cmyk"] :deep(.prose code:not([class*="language-"])),
[data-theme="autumn"] :deep(.prose code:not([class*="language-"])),
[data-theme="acid"] :deep(.prose code:not([class*="language-"])),
[data-theme="lemonade"] :deep(.prose code:not([class*="language-"])),
[data-theme="winter"] :deep(.prose code:not([class*="language-"])) {
  background-color: rgba(0, 0, 0, 0.08) !important;
  color: rgb(124, 58, 237) !important;
  border: 1px solid rgba(0, 0, 0, 0.15) !important;
}

/* 行内代码样式修复 */
:deep(.prose) {
  font-size: var(--base-font-size); /* 继承全局字体大小 */
}

/* 确保prose内的所有元素都能正确继承字体大小 */
:deep(.prose *) {
  font-size: inherit;
}

/* 为不同标题级别设置相对大小 */
:deep(.prose h1) {
  font-size: calc(var(--base-font-size) * 2.25); /* 相当于 text-4xl */
}

:deep(.prose h2) {
  font-size: calc(var(--base-font-size) * 1.875); /* 相当于 text-3xl */
}

:deep(.prose h3) {
  font-size: calc(var(--base-font-size) * 1.5); /* 相当于 text-2xl */
}

:deep(.prose h4) {
  font-size: calc(var(--base-font-size) * 1.25); /* 相当于 text-xl */
}

:deep(.prose h5) {
  font-size: calc(var(--base-font-size) * 1.125); /* 相当于 text-lg */
}

:deep(.prose h6) {
  font-size: var(--base-font-size); /* 相当于 text-base */
}

</style>

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

            <template v-for="(message, _messageIndex) in messages" :key="messageIndex">
              <div :class="['chat', message.role === 'user' ? 'chat-end' : 'chat-start']">
                <div class="chat-image avatar">
                  <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                    <img v-if="message.role === 'user'" src="/img/user-avatar.svg" alt="User" />
                    <img v-else :src="`/img/${selectedModel}-avatar.svg`" :alt="getSelectedModelName()" />
                  </div>
                </div>
                <div class="chat-header">
                  {{ message.role === 'user' ? '您' : getSelectedModelName() }}
                  <time class="text-xs opacity-50 ml-1">{{ formatTime(message.timestamp) }}</time>
                </div>
                <div :class="['chat-bubble', message.failed ? 'border border-red-500' : '']">
                  <!-- 显示附件 -->
                  <div v-if="message.attachments && message.attachments.length > 0" class="mb-2">
                    <div v-for="attachment in message.attachments" :key="attachment.id" class="attachment-item mb-2">
                      <!-- 图片附件 -->
                      <div v-if="attachment.type.startsWith('image/')" class="image-attachment">
                        <img :src="attachment.url" :alt="attachment.name" class="max-w-xs max-h-48 rounded-lg cursor-pointer" 
                             @click="previewImage(attachment.url)" />
                        <p class="text-xs text-base-content/70 mt-1">{{ attachment.name }}</p>
                      </div>
                      <!-- 文档附件 -->
                      <div v-else class="document-attachment flex items-center p-2 bg-base-300 rounded-lg">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                        </svg>
                        <div class="flex-1">
                          <p class="text-sm font-medium">{{ attachment.name }}</p>
                          <p class="text-xs text-base-content/70">{{ formatFileSize(attachment.size) }}</p>
                        </div>
                        <button class="btn btn-xs btn-ghost" @click="downloadFile(attachment)">
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                          </svg>
                        </button>
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
                {{ getSelectedModelName() }}
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
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
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
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <!-- 文档预览 -->
                  <div v-else class="relative bg-base-200 rounded-lg p-2 h-20 flex flex-col items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-primary mb-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <span class="text-xs text-center truncate w-full">{{ file.name }}</span>
                    <button class="absolute -top-1 -right-1 btn btn-xs btn-circle btn-error" @click="removeFile(index)">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <p class="text-xs text-center mt-1 truncate">{{ file.name }}</p>
                </div>
              </div>
            </div>

            <!-- 工具按钮 -->
            <div class="flex justify-end gap-2 mb-1">
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
                                  <!-- 右侧工具 -->
                <div class="flex items-center gap-1">
                  <button @click="openApiSettings" class="btn btn-sm btn-ghost tooltip tooltip-top" data-tip="API设置">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                      stroke="currentColor" class="w-4 h-4">
                      <path stroke-linecap="round" stroke-linejoin="round"
                        d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" />
                      <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                  </button>
                </div>
                  <!-- 文件上传按钮 -->
                  <div class="file-upload-container">
                    <input 
                      ref="fileInput" 
                      type="file" 
                      multiple 
                      accept="image/jpeg,image/jpg,image/png,image/gif,image/webp,image/bmp,image/tiff,image/x-icon,.jpg,.jpeg,.png,.apng,.gif,.webp,.bmp,.dib,.tiff,.tif,.ico,.icns,.sgi,.j2c,.j2k,.jp2,.jpc,.jpf,.jpx,.pdf,.doc,.docx,.txt,.md,.json,.csv,.xlsx,.xls"
                      @change="handleFileUpload"
                      class="hidden"
                    />
                    <button 
                      class="btn btn-sm btn-ghost tooltip tooltip-top" 
                      data-tip="上传文件"
                      @click="triggerFileUpload"
                      :disabled="isLoading || isStreaming"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                      </svg>
                    </button>
                  </div>
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
                <textarea 
                  v-model="userInput" 
                  @keydown.enter.prevent="handleEnterKey" 
                  placeholder="输入您的问题或指令，支持上传图片和文档..."
                  class="textarea w-full resize-none border-0 focus:outline-none bg-transparent"
                  :disabled="!selectedModel || !hasApiKey || isLoading || isStreaming" 
                  rows="3"
                ></textarea>
                
                <!-- 发送按钮 -->
                <button 
                  @click="isStreaming ? cancelGeneration() : sendMessage()" 
                  class="btn absolute right-5 bottom-5"
                :class="isStreaming ? 'btn-error' : 'btn-primary'"
                  :disabled="(!userInput.trim() && uploadedFiles.length === 0 && !isStreaming) || !selectedModel || !hasApiKey"
                  :title="isStreaming ? '取消生成' : '发送消息'"
                >
                <!-- 发送图标（右箭头） -->
                <svg v-if="!isStreaming" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none"
                  viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
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
            <button 
              class="btn btn-xs btn-outline" 
              @click="openModelConfigModal(editingModel)"
              :disabled="!editingModel"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4" />
              </svg>
              配置模型
            </button>
          </label>
          
          <!-- 可搜索的模型名称选择器 -->
          <div class="relative">
            <input 
              type="text" 
              v-model="customModelName" 
              :placeholder="getDefaultModelName(editingModel)"
              class="input input-bordered w-full" 
              @focus="showModelSuggestions = true"
              @input="filterModelSuggestions"
              @blur="hideModelSuggestions"
              autocomplete="off"
            />
            
            <!-- 模型建议下拉框 -->
            <div 
              v-if="showModelSuggestions && filteredModelSuggestions.length > 0"
              class="absolute z-10 w-full bg-base-100 border border-base-300 rounded-lg shadow-lg max-h-60 overflow-y-auto mt-1 model-suggestions-dropdown"
            >
              <div 
                v-for="suggestion in filteredModelSuggestions" 
                :key="suggestion.name"
                class="px-3 py-2 hover:bg-base-200 cursor-pointer border-b border-base-200 last:border-b-0 model-suggestion-item"
                @mousedown.prevent="selectModelSuggestion(suggestion.name)"
              >
                <div class="font-medium">{{ suggestion.name }}</div>
                <div class="text-sm text-base-content/70">{{ suggestion.description }}</div>
              </div>
            </div>
          </div>
          
          <label class="label">
            <span class="label-text-alt">不同的API提供商可能使用不同的模型名称，您可以在此自定义或从建议中选择</span>
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
              <input 
                type="text" 
                v-model="newModelName" 
                placeholder="模型名称" 
                class="input input-bordered input-sm" 
                @keydown.enter="addNewModel"
              />
            </div>
            <div class="form-control">
              <input 
                type="text" 
                v-model="newModelDescription" 
                placeholder="模型描述" 
                class="input input-bordered input-sm" 
                @keydown.enter="addNewModel"
              />
            </div>
          </div>
          <div class="mt-3">
            <button 
              class="btn btn-sm btn-primary" 
              @click="addNewModel"
              :disabled="!newModelName.trim() || !newModelDescription.trim()"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              添加模型
            </button>
          </div>
        </div>

        <!-- 现有模型列表 -->
        <div class="max-h-96 overflow-y-auto">
          <div class="space-y-2">
            <div 
              v-for="(model, index) in editingModelList" 
              :key="index"
              class="flex items-center justify-between p-3 bg-base-100 rounded-lg border border-base-300"
            >
              <div class="flex-1">
                <div class="font-medium">{{ model.name }}</div>
                <div class="text-sm text-base-content/70">{{ model.description }}</div>
              </div>
              <button 
                class="btn btn-xs btn-error ml-3" 
                @click="removeModel(index)"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
          
          <div v-if="editingModelList.length === 0" class="text-center py-8 text-base-content/70">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto mb-2 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2m13-4V6a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M9 9V6a1 1 0 011-1h4a1 1 0 011 1v3" />
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, computed, watch, onBeforeUnmount, onActivated, onDeactivated } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import DOMPurify from 'dompurify'
import { useTipsStore } from '../stores/tipsStore'
import { useRouter } from 'vue-router'
import hljs from 'highlight.js'

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

// 引入路由
const router = useRouter()

// 返回主页
const goBack = () => {
  router.push('/')
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

// 是否显示对话列表抽屉
const showConversationsDrawer = ref(false)

// API 设置相关
const apiSettingsModal = ref<HTMLDialogElement | null>(null)
const editingModel = ref('')
const apiKey = ref('')
const apiEndpoint = ref('')
const customModelName = ref('')
const showApiKey = ref(false)

// 模型建议相关
const showModelSuggestions = ref(false)
const filteredModelSuggestions = ref<Array<{name: string, description: string}>>([])

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
const editingModelList = ref<Array<{name: string, description: string}>>([])
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

      // 流式输出完成，添加完整消息到数据库
      await addAIMessage(activeConversationId.value, 'assistant', streamingContent.value)

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
    if ((!userInput.value.trim() && uploadedFiles.value.length === 0) || !selectedModel.value || isLoading.value || isStreaming.value) {
      return
    }
    messageToSend = userInput.value
    
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
    
    // 保存用户消息到数据库（包含附件信息）
    // 将附件信息序列化为JSON字符串，附加到消息内容中
    let messageContent = messageToSend
    if (attachments.length > 0) {
      const attachmentsJson = JSON.stringify(attachments)
      messageContent += `\n\n__ATTACHMENTS__:${attachmentsJson}`
    }
    
    await addAIMessage(activeConversationId.value, 'user', messageContent)
    
    userInput.value = ''
    clearAllFiles() // 清空已上传的文件
    await loadMessages(activeConversationId.value)
    
    // 立即设置加载状态，让用户看到机器人正在思考
    isLoading.value = true
    
    await nextTick()
    scrollToBottom()
  }

  // 如果不是重新发送，也要设置加载状态
  if (!resendMessage) {
    isLoading.value = true
  }

  try {
    // 开始流式输出
    isStreaming.value = true
    streamingContent.value = ''
    currentStreamingId.value = generateUniqueId()
    
    // 立即滚动到底部，确保用户能看到加载状态
    await nextTick()
    scrollToBottom()
    
    let customModelName: string | undefined = undefined
    try {
      const modelNameResult = await invoke('get_model_name_config', { modelId: selectedModel.value })
      if (modelNameResult && typeof modelNameResult === 'string' && modelNameResult.trim()) {
        customModelName = modelNameResult
      }
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
        streamId: currentStreamingId.value
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
        customModelName: customModelName
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
  // 不再使用 marked，改为简单的正则表达式转换
  let html = content

  // 代码块
  html = html.replace(/```([a-z]*)\n([\s\S]*?)\n```/g, (_match, lang, code) => {
    try {
      let highlightedCode = code;
      if (lang && hljs.getLanguage(lang)) {
        highlightedCode = hljs.highlight(code, { language: lang }).value;
      } else {
        highlightedCode = hljs.highlightAuto(code).value;
      }
      return `<pre><code class="hljs language-${lang || ''}">${highlightedCode}</code></pre>`;
    } catch (e) {
      return `<pre><code>${DOMPurify.sanitize(code)}</code></pre>`;
    }
  });

  // 行内代码
  html = html.replace(/`([^`]+)`/g, '<code>$1</code>');

  // 标题
  html = html.replace(/^# (.+)$/gm, '<h1>$1</h1>');
  html = html.replace(/^## (.+)$/gm, '<h2>$1</h2>');
  html = html.replace(/^### (.+)$/gm, '<h3>$1</h3>');

  // 粗体和斜体
  html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
  html = html.replace(/\*([^*]+)\*/g, '<em>$1</em>');

  // 链接
  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>');

  // 图片
  html = html.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1">');

  // 无序列表
  html = html.replace(/^\s*-\s+(.+)$/gm, '<li>$1</li>');
  html = html.replace(/(<li>.*<\/li>\n)+/g, '<ul>$&</ul>');

  // 引用块
  html = html.replace(/^\s*>\s+(.+)$/gm, '<blockquote><p>$1</p></blockquote>');

  // 确保纯文本被包含在段落标签中
  html = html.replace(/^(?!<[a-z][^>]*>)(.+)$/gm, '<p>$1</p>');

  // 清理HTML以防止XSS
  return DOMPurify.sanitize(html);
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

  // 加载所有对话
  await loadConversations();

  // 加载分类和标签（用于保存笔记）
  await Promise.all([
    tipsStore.fetchAllCategories(),
    tipsStore.fetchAllTags()
  ]);

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
const hideContextMenu = () => {
  showContextMenu.value = false
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
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
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
</style>
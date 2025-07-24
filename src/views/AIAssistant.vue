<template>
  <div class="ai-assistant-page flex flex-col h-screen" @click="hideContextMenu">
    <!-- 顶部区域 -->
    <div class="page-header p-2 bg-base-200 flex items-center justify-between">
      <div class="flex items-center">
        <button class="btn btn-ghost mr-2" @click="goBack">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>
        <div>
          <h1 class="text-xl font-bold">{{ $t('ai.ai_assistant') }}</h1>
          <p v-if="!isMobile" class="text-base-content/70">{{ $t('ai.ai_assistant_description') }}</p>
        </div>
      </div>
      <div class="flex gap-1 md:gap-2">
        <button class="btn btn-sm" @click="openRoleManager">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 md:mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          <span v-if="!isMobile">{{ $t('ai.role_management') }}{{ roles.length > 0 ? ` (${roles.length})` : '' }}</span>
        </button>
        <button class="btn btn-sm" @click="openConversationsList">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 md:mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
          </svg>
          <span v-if="!isMobile">{{ $t('ai.conversation_list') }} ({{ orderedConversations.length }})</span>
        </button>
        <button class="btn btn-primary btn-sm" @click="createNewConversation">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 md:mr-1" fill="none" viewBox="0 0 24 24"
            stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span v-if="!isMobile">{{ $t('ai.new_conversation') }}</span>
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
                  <span class="loading loading-spinner loading-lg mr-2"></span> {{ $t('ai.loading_conversations') }}...
                </div>
                <template v-else>
                  <div v-if="orderedConversations.length === 0" class="text-center py-8 text-base-content/70">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto mb-2 opacity-50" fill="none"
                      viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                    <p class="font-medium">{{ $t('ai.no_conversation_record') }}</p>
                    <p class="text-sm mb-3">{{ $t('ai.create_new_conversation') }}</p>
                    <button class="btn btn-sm btn-primary" @click="createNewConversation">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                      </svg>
                      {{ $t('ai.create_new_conversation') }}
                    </button>
                  </div>
                  <div v-else class="space-y-3">
                    <div class="flex justify-between items-center text-sm text-base-content/70 mb-2 px-1">
                      <span>{{ orderedConversations.length }} {{ $t('ai.conversations') }}</span>
                      <span>{{ $t('ai.sort_by_update_time') }}</span>
                    </div>
                    <div v-for="conversation in orderedConversations" :key="conversation.id"
                      class="p-3 rounded-lg cursor-pointer hover:bg-base-200 transition-colors border border-base-300 flex flex-col"
                      :class="{ 'bg-base-200 border-primary': conversation.id === activeConversationId }"
                      @click="switchConversation(conversation.id)">
                      <div class="flex justify-between items-start">
                        <div class="flex-1 min-w-0">
                          <div class="font-medium truncate">{{ conversation.title || $t('ai.no_title_conversation') }}
                          </div>
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
      <div class="flex-1 flex flex-col p-1 md:p-4 border-r border-base-300 overflow-hidden">
        <!-- 聊天界面 -->
        <div class="chat-container bg-base-200 rounded-lg p-2 md:p-4 flex-1 flex flex-col overflow-hidden">
          <!-- 消息显示区域 -->
          <div class="flex-grow overflow-y-auto mb-4 flex flex-col-reverse" ref="messagesContainer"
            @mouseup="handleTextSelection" @contextmenu.prevent="handleContextMenu" @scroll="handleScroll">

            <!-- 加载中指示器和流式输出 - 在反向布局中显示在顶部 -->
            <div class="space-y-4">
              <!-- 加载中指示器 - 优化显示条件 -->
              <div v-if="isLoading || (isStreaming && !streamingContent)" class="chat chat-start">
                <div class="chat-image avatar">
                  <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                    <img :src="getModelAvatarPath(selectedModel)" :alt="getSelectedModelName()" />
                  </div>
                </div>
                <div class="chat-header">
                  {{ selectedRole ? selectedRole.name : getSelectedModelName() }}
                  <time class="text-xs opacity-50 ml-1">{{ formatTime(Date.now()) }}</time>
                </div>
                <div class="chat-bubble bg-base-150 ai-thinking">
                  <div class="flex items-center gap-2">
                    <span class="loading loading-dots loading-sm"></span>
                    <!-- <span class="text-sm opacity-70">...</span> -->
                  </div>
                </div>
              </div>

              <!-- 流式输出响应 -->
              <div v-if="isStreaming && streamingContent" class="chat chat-start">
                <div class="chat-image avatar">
                  <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                    <img :src="getModelAvatarPath(selectedModel)" :alt="getSelectedModelName()" />
                  </div>
                </div>
                <div class="chat-header">
                  {{ selectedRole ? selectedRole.name : getSelectedModelName() }}
                  <time class="text-xs opacity-50 ml-1">{{ formatTime(Date.now()) }}</time>
                </div>
                <div class="chat-bubble" v-html="formatMessage(streamingContent)"></div>
              </div>
            </div>

            <!-- 消息列表 - 正常时间顺序显示 -->
            <div class="space-y-4">
              <template v-for="(message, _messageIndex) in visibleMessages" :key="_messageIndex">
                <div :class="['chat', message.role === 'user' ? 'chat-end' : 'chat-start']">
                  <div class="chat-image avatar">
                    <div class="w-10 rounded-full bg-base-300 overflow-hidden">
                      <img v-if="message.role === 'user'" src="/img/user-avatar.svg" alt="User" />
                      <img v-else :src="getModelAvatarPath(selectedModel)" :alt="getSelectedModelName()" />
                    </div>
                  </div>
                  <div class="chat-header">
                    {{ message.role === 'user' ? $t('ai.you') : (message.role_name || getSelectedModelName()) }}
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
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-info flex-shrink-0"
                            fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                          </svg>
                          <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-info ">{{ note.title }}</p>
                            <p class="text-xs text-base-content/70">{{ note.tip_type }} • {{
                              $t('ai.click_to_view_details') }}</p>
                          </div>
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-info flex-shrink-0 ml-1"
                            fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                          </svg>
                        </div>
                      </div>
                    </div>

                    <!-- 消息内容 -->
                    <div v-html="formatMessage(message.content)"></div>
                  </div>
                  <div class="chat-footer opacity-50 flex gap-1" v-if="message.role === 'assistant'">
                    <button class="btn btn-xs btn-ghost"
                      :class="{ 'copy-success': copyingStates[`${_messageIndex}_assistant`] === 'success' }"
                      @click="copyToClipboardWithFeedback(message.content, $event)"
                      :disabled="!!copyingStates[`${_messageIndex}_assistant`]">
                      <span v-if="copyingStates[`${_messageIndex}_assistant`] === 'copying'">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 animate-spin" fill="none"
                          viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                        </svg>
                      </span>
                      <span v-else-if="copyingStates[`${_messageIndex}_assistant`] === 'success'"
                        class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                          stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                        {{ $t('ai.copied') }}
                      </span>
                      <span v-else>{{ $t('ai.copy') }}</span>
                    </button>
                    <button class="btn btn-xs btn-ghost" @click="addToNote(message.content)">{{ $t('ai.add_to_note')
                      }}</button>
                  </div>
                  <div class="chat-footer opacity-50 flex gap-1" v-if="message.role === 'user'">
                    <button class="btn btn-xs btn-ghost" @click="resendMessage(message)">{{ $t('ai.resend') }}</button>
                    <button class="btn btn-xs btn-ghost"
                      :class="{ 'copy-success': copyingStates[`${_messageIndex}_user`] === 'success' }"
                      @click="copyToClipboardWithFeedback(message.content, $event)"
                      :disabled="!!copyingStates[`${_messageIndex}_user`]">
                      <span v-if="copyingStates[`${_messageIndex}_user`] === 'copying'">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 animate-spin" fill="none"
                          viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                        </svg>
                      </span>
                      <span v-else-if="copyingStates[`${_messageIndex}_user`] === 'success'"
                        class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24"
                          stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                        {{ $t('ai.copied') }}
                      </span>
                      <span v-else>{{ $t('ai.copy') }}</span>
                    </button>
                  </div>
                  <div class="chat-footer opacity-80 flex gap-1" v-if="message.failed && message.role === 'user'">
                    <span class="text-red-500">{{ $t('ai.send_failed') }}</span>
                    <button class="btn btn-xs btn-error" @click="sendMessage(message)">{{ $t('ai.resend') }}</button>
                  </div>
                </div>
              </template>
            </div>

            <!-- 消息数量提示 - 在反向布局中显示在底部 -->
            <div v-if="messages.length > MAX_VISIBLE_MESSAGES"
              class="message-count-notice bg-warning/10 border border-warning/30 rounded-lg p-3 mb-4 text-center">
              <div class="flex items-center justify-center gap-2 text-warning-content">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span class="text-sm font-medium">
                  {{ $t('ai.only_show_latest_messages') }} {{ MAX_VISIBLE_MESSAGES }} {{ $t('ai.messages') }} ({{
                  messages.length }} {{ $t('ai.messages') }})
                </span>
              </div>
              <p class="text-xs text-warning-content/70 mt-1">
                {{ $t('ai.earlier_messages_hidden') }}
              </p>
            </div>

            <!-- 空状态显示 - 在反向布局中显示在底部 -->
            <div v-if="messages.length === 0"
              class="flex flex-col items-center justify-center h-full text-base-content/60">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="w-16 h-16 mb-4">
                <path stroke-linecap="round" stroke-linejoin="round"
                  d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
              </svg>
              <p class="text-lg">{{ $t('ai.select_ai_model_to_start_conversation') }}</p>
              <p class="text-sm">{{ $t('ai.your_conversation_is_only_saved_locally') }}</p>
            </div>
          </div>

          <!-- 输入区域 -->
          <div class="flex flex-col gap-3">
            <!-- 已上传文件预览区域 -->
            <div v-if="uploadedFiles.length > 0" class="uploaded-files-preview bg-base-100 rounded-lg p-3">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">{{ $t('ai.selected_files') }} ({{ uploadedFiles.length }})</span>
                <button class="btn btn-xs btn-ghost" @click="clearAllFiles">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                  {{ $t('ai.clear') }}
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
                  <select v-model="selectedModel" class="select select-sm select-bordered" @change="handleModelChange">
                    <option disabled value="">{{ $t('ai.select_ai_model') }}</option>
                    <option v-for="model in availableModels" :key="model.id" :value="model.id">
                      {{ model.name }}
                    </option>
                    <option value="__configure__"> - {{ $t('ai.configure_model') }}</option>
                  </select>

                  <!-- 设置按钮 - 跳转到设置页面 -->


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
                    <button class="btn btn-xs btn-outline tooltip tooltip-bottom flex items-center"
                      :data-tip="$t('ai.select_role')" @click="openRoleManager">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" :class="{ 'mr-1': !isMobile }" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                      <span v-if="!isMobile">{{ selectedRole ? $t('ai.change_role') : $t('ai.select_role') }}</span>
                    </button>
                  </div>

                  <!-- 文件上传按钮 -->
                  <div class="file-upload-container">
                    <input ref="fileInput" type="file" multiple
                      accept="image/jpeg,image/jpg,image/png,image/gif,image/webp,image/bmp,image/tiff,image/x-icon,.jpg,.jpeg,.png,.apng,.gif,.webp,.bmp,.dib,.tiff,.tif,.ico,.icns,.sgi,.j2c,.j2k,.jp2,.jpc,.jpf,.jpx,.pdf,.doc,.docx,.txt,.md,.json,.csv,.xlsx,.xls"
                      @change="handleFileUpload" class="hidden" />
                    <button class="btn btn-sm btn-ghost tooltip tooltip-top" :data-tip="$t('ai.upload_file')"
                      @click="triggerFileUpload" :disabled="isLoading || isStreaming">
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                          d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                      </svg>
                    </button>
                  </div>
                </div>

                <!-- 右侧工具 -->
                <div v-if="!isMobile" class="flex items-center gap-1">
                  <!-- 工具按钮 -->
                  <button class="btn btn-sm btn-outline" @click="createNewConversation"
                    :title="$t('ai.new_conversation')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    {{ $t('ai.new_conversation') }}
                  </button>
                  <button class="btn btn-sm btn-outline" @click="clearMessages"
                    :title="$t('ai.clear_conversation_history')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    {{ $t('ai.clear_conversation') }}
                  </button>
                  <button class="btn btn-sm btn-outline" @click="exportMessages"
                    :title="$t('ai.export_conversation_content')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                    </svg>
                    {{ $t('ai.export_conversation') }}
                  </button>
                </div>
              </div>

              <!-- API密钥警告 -->
              <div v-if="!hasApiKey && !selectedModel.startsWith('custom_')" class="alert alert-warning m-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                  viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <span>{{ $t('ai.you_have_not_configured_the_api_key_for') }} {{ getSelectedModelName() }}。<button
                    @click="goToAISettings" class="link link-primary">{{ $t('ai.go_to_settings') }}</button></span>
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
                    <span class="text-sm font-medium text-primary">{{ $t('ai.current_role') }}：{{ selectedRole.name
                      }}</span>
                  </div>
                  <p class="text-xs text-base-content/70 line-clamp-2">{{ selectedRole.description }}</p>
                </div>

                <textarea ref="messageTextarea" v-model="userInput" @keydown="handleInputKeydown"
                  @keydown.enter.prevent="handleEnterKey" :placeholder="getInputPlaceholder()"
                  class="textarea w-full resize-none border-0 focus:outline-none bg-transparent"
                  :disabled="!selectedModel || (!selectedModel.startsWith('custom_') && !hasApiKey) || isLoading || isStreaming"
                  rows="3"></textarea>

                <!-- 笔记选择器 -->
                <div v-if="showNoteSelector"
                  class="note-selector absolute z-50 w-full bg-base-100 border border-base-300 rounded-lg shadow-xl max-h-60 overflow-y-auto"
                  style="bottom: 100%; margin-bottom: 8px;" @click.stop>
                  <!-- 搜索框 -->
                  <div class="p-2 border-b border-base-300">
                    <input v-model="noteSearchQuery" type="text"
                      :placeholder="`${$t('ai.search_notes')}...（${$t('ai.enter_to_select_first')}，ESC${$t('ai.close')}）`"
                      class="input input-xs input-bordered w-full" @keydown.enter.prevent="selectFirstNote"
                      @keydown.escape.prevent="hideNoteSelector" @keydown.tab.prevent="hideNoteSelector"
                      ref="noteSearchInput" />
                    <div class="text-xs text-base-content/50 mt-1 flex justify-between">
                      <span>{{ $t('ai.display') }} {{ filteredNotes.length }} {{ $t('ai.notes') }}{{
                        noteSearchQuery.trim() ?
                          ' (' + $t('ai.search_results') + ')' : ' (' + $t('ai.latest') + ')' }}</span>
                      <span>{{ $t('ai.enter_to_select') }} • ESC{{ $t('ai.close') }}</span>
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
                            {{ (note.content || '').substring(0, 100) }}{{ (note.content || '').length > 100 ? '...' :
                            '' }}
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
                      <p class="text-sm">{{ tipsStore.tips.length === 0 ? $t('ai.no_notes_please_create_some_notes') :
                        $t('ai.no_matching_notes') }}</p>
                    </div>
                  </div>
                </div>

                <!-- 发送按钮 -->
                <button @click="isStreaming ? cancelGeneration() : sendMessage()" class="btn absolute right-5 bottom-5"
                  :class="isStreaming ? 'btn-error' : 'btn-primary'"
                  :disabled="(!userInput.trim() && uploadedFiles.length === 0 && !isStreaming) || !selectedModel || (!selectedModel.startsWith('custom_') && !hasApiKey)"
                  :title="isStreaming ? $t('ai.cancel_generation') : $t('ai.send_message')">
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

        <!-- 右侧笔记区域 (仅桌面) -->
        <transition name="fade-slide">
          <div class="w-1/3 min-w-[350px] flex-col p-4 overflow-hidden hidden md:flex"
            v-show="showNotePanel && !isMobile">
            <div class="flex justify-between items-center mb-4">
              <h2 class="text-xl font-bold">{{ $t('ai.notes') }}</h2>
              <div class="flex gap-2">
                <button class="btn btn-sm" @click="showNotePanel = false">{{ $t('ai.close') }}</button>
                <button class="btn btn-sm btn-primary" @click="saveNoteAsTip"
                  :disabled="!noteTitle.trim() || isNoteSaving">
                  <span v-if="isNoteSaving" class="loading loading-spinner loading-xs"></span>
                  {{ $t('ai.save_note') }}
                </button>
              </div>
            </div>

            <div class="form-control w-full mb-4">
              <label class="label">
                <span class="label-text">{{ $t('ai.note_title') }}</span>
              </label>
              <input type="text" :placeholder="$t('ai.enter_note_title') + '...'" class="input input-bordered w-full"
                v-model="noteTitle" />
            </div>

            <div class="flex-1 overflow-hidden flex flex-col">
              <div class="flex justify-between items-center mb-2">
                <label class="label">
                  <span class="label-text">{{ $t('ai.note_content') }} ({{ $t('ai.support_markdown_format') }})</span>
                </label>
                <div class="btn-group btn-group-sm">
                  <button class="btn btn-sm " :class="{ 'btn-active': !isNotePreviewMode }"
                    @click="() => { console.log('切换 到编辑模式'); isNotePreviewMode = false }" :title="$t('ai.edit_mode')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button class="btn btn-sm " :class="{ 'btn-active': isNotePreviewMode }"
                    @click="() => { console.log('切换到预览模式', 'noteContent:', noteContent); isNotePreviewMode = true }"
                    :title="$t('ai.preview_mode')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                    </svg>
                  </button>
                </div>
              </div>

              <!-- 编辑模式 -->
              <textarea v-if="!isNotePreviewMode" v-model="noteContent"
                :placeholder="$t('ai.add_content_from_left_ai_conversation') + '...'"
                class="textarea textarea-bordered w-full flex-1 font-mono resize-none"></textarea>

              <!-- 预览模式 -->
              <div v-else
                class="flex-1 p-4 overflow-auto prose prose-sm max-w-none bg-base-200 rounded-lg border border-base-300"
                v-html="renderedNoteContent"></div>
            </div>
          </div>
        </transition>
      </div>

      <!-- 笔记保存成功提示 -->
      <div class="toast" v-if="showSaveSuccess">
        <div class="alert alert-success">
          <span>{{ $t('ai.note_saved_successfully') }}</span>
        </div>
      </div>




      <!-- 编辑对话标题对话框 -->
      <dialog ref="editTitleModal" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">{{ $t('ai.edit_conversation_title') }}</h3>

          <div class="form-control w-full">
            <input type="text" v-model="editingTitle" :placeholder="$t('ai.enter_conversation_title')"
              class="input input-bordered w-full" />
          </div>

          <div class="modal-action">
            <button class="btn" @click="closeEditTitleModal">{{ $t('ai.cancel') }}</button>
            <button class="btn btn-primary" @click="saveEditedTitle">{{ $t('ai.save') }}</button>
          </div>
        </div>
      </dialog>

      <!-- 删除确认对话框 -->
      <dialog ref="deleteConfirmModal" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg mb-4">{{ $t('ai.confirm_delete') }}</h3>
          <p>{{ $t('ai.confirm_delete_conversation') }}</p>
          <div class="modal-action">
            <button class="btn" @click="closeDeleteConfirmModal">{{ $t('ai.cancel') }}</button>
            <button class="btn btn-error" @click="confirmDelete">{{ $t('ai.delete') }}</button>
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
              {{ $t('ai.add_to_note') }}
            </button>
          </li>
          <li>
            <button @click="copySelectedText" class="flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
              </svg>
              {{ $t('ai.copy_selected_content') }}
            </button>
          </li>
        </ul>
      </div>

      <!-- 角色管理对话框 -->
      <dialog ref="roleManagerModal" class="modal">
        <div class="modal-box w-11/12 max-w-4xl">
          <h3 class="font-bold text-lg mb-4">{{ $t('ai.role_management') }}</h3>

          <!-- 角色列表 -->
          <div v-if="!showRoleForm" class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm text-base-content/70">
                {{ roles.length }} {{ $t('ai.roles') }}
              </span>
              <button class="btn btn-sm btn-primary" @click="openRoleForm()">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                {{ $t('ai.new_role') }}
              </button>
            </div>

            <!-- 加载状态 -->
            <div v-if="isLoadingRoles" class="flex items-center justify-center py-8">
              <span class="loading loading-spinner loading-lg mr-2"></span>
              {{ $t('ai.loading_roles') }}...
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
                      {{ $t('ai.select_this_role') }}
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
              <h3 class="text-lg font-medium mb-2">{{ $t('ai.no_roles') }}</h3>
              <p class="text-base-content/70 mb-4">{{
                $t('ai.create_your_first_ai_role_to_start_role_playing_conversation') }}
              </p>
              <button class="btn btn-primary" @click="openRoleForm()">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                {{ $t('ai.create_role') }}
              </button>
            </div>
          </div>

          <!-- 角色表单 -->
          <div v-if="showRoleForm" class="space-y-4">
            <div class="flex items-center gap-2 mb-4">
              <button class="btn btn-sm btn-ghost" @click="closeRoleForm">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                </svg>
              </button>
              <h4 class="text-lg font-medium">
                {{ editingRole ? $t('ai.edit_role') : $t('ai.create_new_role') }}
              </h4>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('ai.role_name') }}</span>
              </label>
              <input type="text" v-model="newRoleName"
                :placeholder="`${$t('ai.for_example')}：${$t('ai.programming_mentor')}, ${$t('ai.creative_writer')}, ${$t('ai.data_analyst')}...`"
                class="input input-bordered w-full" maxlength="50" />
              <label class="label">
                <span class="label-text-alt">{{ newRoleName.length }}/50</span>
              </label>
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">{{ $t('ai.role_description') }}</span>
              </label>
              <textarea v-model="newRoleDescription"
                :placeholder="`${$t('ai.describe_the_characteristics_of_the_role')}。${$t('ai.for_example')}：${$t('ai.you_are_an_experienced_programming_mentor')}, ${$t('ai.creative_writer')}, ${$t('ai.data_analyst')}...`"
                class="textarea textarea-bordered w-full h-32 resize-none" maxlength="100000"></textarea>
              <label class="label">
                <span class="label-text-alt">{{ newRoleDescription.length }}/100000</span>
              </label>
            </div>

            <div class="flex gap-2 pt-4">
              <button class="btn btn-ghost" @click="closeRoleForm">{{ $t('ai.cancel') }}</button>
              <button class="btn btn-primary" @click="saveRole"
                :disabled="!newRoleName.trim() || !newRoleDescription.trim()">
                {{ editingRole ? $t('ai.save_changes') : $t('ai.create_role') }}
              </button>
            </div>
          </div>

          <div class="modal-action" v-if="!showRoleForm">
            <button class="btn" @click="closeRoleManager">{{ $t('ai.close') }}</button>
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
            <button class="btn" @click="closeNoteDetail">{{ $t('ai.close') }}</button>
            <button class="btn btn-outline" @click="copyNoteContent">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
              </svg>
              {{ $t('ai.copy_content') }}
            </button>
          </div>
        </div>
      </dialog>

      <!-- 笔记面板 (移动端模态框) -->
      <dialog ref="notePanelModal" class="modal">
        <div class="modal-box w-11/12 max-w-5xl">
          <div class="flex flex-col h-[80vh]">
            <div class="flex justify-between items-center mb-4 flex-shrink-0">
              <h2 class="text-xl font-bold">{{ $t('ai.notes') }}</h2>
              <div class="flex gap-2">
                <button class="btn btn-sm" @click="closeNotePanelModal">{{ $t('ai.close') }}</button>
                <button class="btn btn-sm btn-primary" @click="saveNoteAsTip"
                  :disabled="!noteTitle.trim() || isNoteSaving">
                  <span v-if="isNoteSaving" class="loading loading-spinner loading-xs"></span>
                  {{ $t('ai.save_note') }}
                </button>
              </div>
            </div>

            <div class="form-control w-full mb-4 flex-shrink-0">
              <label class="label">
                <span class="label-text">{{ $t('ai.note_title') }}</span>
              </label>
              <input type="text" :placeholder="$t('ai.enter_note_title') + '...'" class="input input-bordered w-full"
                v-model="noteTitle" />
            </div>

            <div class="flex-1 overflow-hidden flex flex-col">
              <div class="flex justify-between items-center mb-2 flex-shrink-0">
                <label class="label">
                  <span class="label-text">{{ $t('ai.note_content') }} ({{ $t('ai.support_markdown_format') }})</span>
                </label>
                <div class="btn-group btn-group-sm">
                  <button class="btn btn-sm" :class="{ 'btn-active': !isNotePreviewMode }"
                    @click="isNotePreviewMode = false" :title="$t('ai.edit_mode')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button class="btn btn-sm" :class="{ 'btn-active': isNotePreviewMode }"
                    @click="isNotePreviewMode = true" :title="$t('ai.preview_mode')">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                      stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                    </svg>
                  </button>
                </div>
              </div>

              <!-- 编辑模式 -->
              <textarea v-if="!isNotePreviewMode" v-model="noteContent"
                :placeholder="$t('ai.add_content_from_left_ai_conversation') + '...'"
                class="textarea textarea-bordered w-full flex-1 font-mono resize-none"></textarea>

              <!-- 预览模式 -->
              <div v-else
                class="flex-1 p-4 overflow-auto prose prose-sm max-w-none bg-base-200 rounded-lg border border-base-300"
                v-html="renderedNoteContent"></div>
            </div>
          </div>
        </div>
      </dialog>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, computed, watch, onActivated, onDeactivated, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { showConfirm, showAlert } from '../services/dialog'
import { getAIConfig } from '../services/aiService'

import 'highlight.js/styles/github.css'
import { useTipsStore } from '../stores/tipsStore'
import { invoke } from '@tauri-apps/api/core'
import DOMPurify from 'dompurify'
import Prism from 'prismjs'
import { Marked } from 'marked'
import { markedHighlight } from 'marked-highlight'
import { listen } from '@tauri-apps/api/event'
import { useI18n } from 'vue-i18n'
import { useAIStore } from '../stores/aiStore'
import { storeToRefs } from 'pinia'
import { open } from '@tauri-apps/plugin-shell'


const { t } = useI18n()
const aiStore = useAIStore()

const router = useRouter()
const tipsStore = useTipsStore()


// --- Responsive state ---
const windowWidth = ref(window.innerWidth)
const isMobile = computed(() => windowWidth.value < 768)
const onResize = () => { windowWidth.value = window.innerWidth }

// 全局配置变更监听器
let unlistenSettings: any = null
let themeObserver: MutationObserver | null = null

// 重新加载AI配置的函数
const reloadAIConfig = async () => {
  console.log('Reloading AI config...')
  try {
    const config = await getAIConfig()
    if (config && config.providers) {
      aiConfigs.value = config.providers
      console.log('AI configs reloaded:', aiConfigs.value)
    }
    await checkApiKey()
    await loadCustomModels() // 重新加载自定义模型列表
  } catch (error) {
    console.error('Failed to reload AI config:', error)
  }
}

// 返回主页
const goBack = () => {

  // 使用两种方式：localStorage 和路由参数
  localStorage.setItem('need-refresh-tips', 'true')
  console.log('AI助手 - 已设置刷新标记:', localStorage.getItem('need-refresh-tips'))

  // 验证设置是否成功
  const verifyMark = localStorage.getItem('need-refresh-tips')
  console.log('AI助手 - 验证刷新标记:', verifyMark)
  console.log('AI助手 - 验证刷新标记类型:', typeof verifyMark)

  // 使用路由参数传递刷新信号
  console.log('AI助手 - 开始路由跳转（带参数）')
  router.push({ path: '/', query: { refresh: 'tips', timestamp: Date.now().toString() } })
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
const { selectedModel } = storeToRefs(aiStore)
const hasApiKey = ref(true)
const aiConfigs = ref<Record<string, any>>({})
const userInput = ref('')
const isLoading = ref(false)
const messagesContainer = ref<HTMLElement | null>(null)

// 滚动控制相关 - 适配反向布局
const isAtBottom = ref(true)
const shouldAutoScroll = ref(true)
const intersectionObserver = ref<IntersectionObserver | null>(null)
const lastScrollTop = ref(0)
const isScrollingUp = ref(false)
const scrollThreshold = ref(50) // 滚动阈值，用于判断是否接近底部

// 流式输出相关
const isStreaming = ref(false)
const streamingContent = ref('')
const currentStreamingId = ref<string | null>(null)

// 性能优化相关
const messageRenderCache = ref(new Map<string, string>()) // 消息渲染缓存
const formatMessageDebounced = ref(new Map<string, number>()) // 格式化防抖
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
const maxTokens = ref<number>(3000) // 默认值3000


// 笔记面板相关
const showNotePanel = ref(Boolean(localStorage.getItem('ai-show-note-panel') === 'true'))
const noteTitle = ref(localStorage.getItem('ai-note-title') || '')
const noteContent = ref(localStorage.getItem('ai-note-content') || '')
const isNoteSaving = ref(false)
const showSaveSuccess = ref(false)
const isNotePreviewMode = ref(true) // 笔记预览模式
const notePanelModal = ref<HTMLDialogElement | null>(null)



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
const availableModels = ref([
  { id: 'chatgpt', name: 'OpenAI ChatGPT' },
  { id: 'gemini', name: 'Google Gemini' },
  { id: 'deepseek', name: 'DeepSeek' },
  { id: 'qwen', name: 'Qwen' },
  { id: 'claude', name: 'Anthropic Claude' },
  { id: 'doubao', name: 'Doubao' },
  { id: 'grok', name: 'Grok' },
])

// 加载自定义模型列表
async function loadCustomModels(): Promise<void> {
  try {
    const customModels = await invoke('list_custom_model_configs')
    const customModelList = customModels as Array<{
      id: string
      name: string
      endpoint: string
      model_name: string
      adapter_type: string
    }>

    // 清除之前的自定义模型
    availableModels.value = availableModels.value.filter(m => !m.id.startsWith('custom_'))

    // 添加新的自定义模型
    customModelList.forEach(model => {
      availableModels.value.push({
        id: `custom_${model.id}`,
        name: `${model.name} (${t('ai.custom')})`
      })
    })

    console.log('加载了自定义模型:', customModelList.length, '个')
  } catch (error) {
    console.error('加载自定义模型失败:', error)
  }
}

// 加载对话列表
async function loadConversations() {
  isLoadingConversations.value = true
  try {
    console.log('开始加载对话列表...')
    const result = await invoke('list_ai_conversations')
    
    // 过滤掉名为"浮动聊天"的对话
    const allConversations = Array.isArray(result) ? result : []
    const floatingConvTitle = t('floatingAI.floatingChat');
    conversations.value = allConversations.filter(c => c.title !== floatingConvTitle);

    console.log(`加载到 ${conversations.value.length} 个对话 (已过滤)`)

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
    const result = await invoke('list_ai_messages', { conversationId: conversationId })
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

    // 加载消息后滚动到底部 - 适配反向布局
    await nextTick()
    if (messagesContainer.value && messages.value.length > 0) {
      // 在反向布局中，scrollTop = 0 表示最底部（最新消息）
      messagesContainer.value.scrollTop = 0
      // 确保滚动状态正确
      shouldAutoScroll.value = true
      isAtBottom.value = true
    }
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
  // 后端返回的是包含 id 等信息的 Conversation 对象，这里需要提取 id 字段
  const newConversation = await invoke('create_ai_conversation', { model, title: t('ai.no_title_conversation') }) as { id: string }

  await loadConversations()
  activeConversationId.value = newConversation.id
  await loadMessages(newConversation.id)
  showConversationsDrawer.value = false

  // 新建对话后确保滚动状态正确 - 适配反向布局
  await nextTick()
  shouldAutoScroll.value = true
  isAtBottom.value = true
}

// 切换对话
async function switchConversation(conversationId: string) {
  activeConversationId.value = conversationId
  await loadMessages(conversationId)
  showConversationsDrawer.value = false

  // 切换对话后强制滚动到底部 - 适配反向布局
  await nextTick()
  setTimeout(() => {
    forceScrollToBottom()
  }, 100)
}

// 删除对话
async function deleteConversation(conversationId: string) {
  await invoke('delete_ai_conversation', { conversationId })
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

        await invoke('add_ai_message', { conversationId: activeConversationId.value, role: 'assistant', content: finalContent })

        // 重新加载消息列表
        await loadMessages(activeConversationId.value)

        // 重置状态
        isStreaming.value = false;
        streamingContent.value = '';
        currentStreamingId.value = null;
        isLoading.value = false;

        // 滚动到底部
        await nextTick();
        scrollToBottom();
      } else {
        // 收到内容chunk，累加到流式内容中
        streamingContent.value += payload.chunk;

        // 收到第一个有内容的chunk时，关闭加载状态
        if (isLoading.value && streamingContent.value.trim()) {
          isLoading.value = false;
        }

        // 如果在底部且用户没有在滚动，自动滚动
        if (shouldAutoScroll.value && !isUserScrolling.value) {
          scrollToBottom();
        }
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
          await showAlert(t('ai.reprocess_image_file_failed'), { title: t('ai.error') })
          return
        }
      }
    }

    // 先移除旧的失败消息
    if (resendIndex !== -1) messages.value.splice(resendIndex, 1)
  } else {
    // 正常发送新消息
    if (!userInput.value.trim() || !selectedModel.value || (!selectedModel.value.startsWith('custom_') && !hasApiKey.value)) {
      return;
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
          notesContent += `\n\n--- 笔记：${note.title} ---\n${note.content || ''}\n--- 笔记内容结束 ---\n`
          referencedNotes.push({
            id: note.id,
            title: note.title,
            content: note.content || '',
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
          await showAlert('处理图片文件失败，请重试', { title: '错误' })
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

    await invoke('add_ai_message', { conversationId: activeConversationId.value, role: 'user', content: messageContent })

    userInput.value = ''
    clearAllFiles() // 清空已上传的文件
    clearAllSelectedNotes() // 清空选中的笔记（清空内部记录，不影响输入框）
    await loadMessages(activeConversationId.value)

    // 立即设置加载状态，让用户看到机器人正在思考
    isLoading.value = true

    await nextTick()
    // 确保在开始生成时滚动到底部
    shouldAutoScroll.value = true
    forceScrollToBottom()
  }

  // 设置加载状态
  isLoading.value = true

  try {
    // 开始流式输出准备
    isStreaming.value = true
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
    console.log(customModelName)

    if (selectedModel.value.startsWith('custom_')) {
      const config = aiConfigs.value[selectedModel.value]
      if (config && config.model_name) {
        customModelName = config.model_name
      } else {
        console.warn(`未找到自定义模型 ${selectedModel.value} 的配置`)
      }
    }

    // 检查是否有图片文件
    if (imageFileData.length > 0) {
      // 有图片文件，使用支持图片的API
      console.log(`发送包含${imageFileData.length}张图片的消息`)

      await invoke('send_ai_message_with_images_stream', {
        providerId: selectedModel.value,
        textMessage: messageToSend,
        imageFiles: imageFileData,
        streamId: currentStreamingId.value,
        conversationId: activeConversationId.value,
        roleId: selectedRole.value?.id || null
      })
    } else {
      // 没有图片文件，使用普通API
      let finalMessage = messageToSend
      if (attachments.length > 0) {
        finalMessage += `\n\n[${t('ai.user_uploaded')} ${attachments.length} {{ $t('ai.files') }}: ${attachments.map(a => a.name).join(', ')}，{{ $t('ai.but_the_current_model_does_not_support_file_processing') }}]`
      }

      await invoke('send_ai_message_stream', {
        providerId: selectedModel.value,
        message: finalMessage,
        streamId: currentStreamingId.value,
        conversationId: activeConversationId.value,
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
    forceScrollToBottom()
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

  // 对于自定义模型，允许空的 API 密钥（或在自己的端点中处理）
  if (selectedModel.value.startsWith('custom_')) {
    hasApiKey.value = true
    return
  }

  // For built-in models, check our loaded config.
  const providerId = selectedModel.value
  const config = aiConfigs.value[providerId]
  hasApiKey.value = !!(config && config.api_key && config.api_key.trim() !== '')

  console.log(`检查 ${providerId} API Key: ${hasApiKey.value}`)
}

// 监听消息变化，自动滚动到底部
watch(messages, async (newMessages, oldMessages) => {
  // 如果有新消息且用户在底部且没有在手动滚动，自动滚动
  if (newMessages.length > (oldMessages?.length || 0) && shouldAutoScroll.value && !isUserScrolling.value) {
    console.log('shouldAutoScroll', shouldAutoScroll.value)
    console.log('isUserScrolling', isUserScrolling.value)
    await nextTick()
    scrollToBottom()
  }

  // 如果是第一次加载消息或消息数量从0变为有消息，强制滚动到底部 - 适配反向布局
  if ((oldMessages?.length || 0) === 0 && newMessages.length > 0) {
    await nextTick()
    setTimeout(() => {
      if (messagesContainer.value) {
        // 在反向布局中，scrollTop = 0 表示最底部（最新消息）
        messagesContainer.value.scrollTop = 0
        shouldAutoScroll.value = true
        isAtBottom.value = true
      }
    }, 50)
  }
}, { deep: false })

// 监听流式内容变化，确保在底部时自动滚动
watch(streamingContent, async () => {
  console.log('shouldAutoScroll:', shouldAutoScroll.value, "  isUserScrolling:", isUserScrolling.value)
  if (shouldAutoScroll.value && !isUserScrolling.value) {
    await nextTick()
    scrollToBottom()
  }
})

// 监听模型选择变化
watch(selectedModel, async (newModel) => {
  await checkApiKey()
  aiStore.setSelectedModel(newModel)
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
              ⚠️ {{ $t('ai.large_code_block') }} (${codeLines.length} {{ t('ai.lines') }}, ${(code.length / 1024).toFixed(1)}KB) - {{ $t('ai.syntax_highlighting_disabled_to_improve_performance') }}
              <button class="btn btn-xs btn-outline ml-2" onclick="this.parentElement.nextElementSibling.style.display = this.parentElement.nextElementSibling.style.display === 'none' ? 'block' : 'none'; this.textContent = this.textContent.includes('显示') ? '隐藏代码' : '显示代码'">
                ${codeLines.length > MAX_LINES ? t('ai.show_full_code') : t('ai.hide_code')}
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
    return `<div class="text-error">${t('ai.markdown_rendering_error')}: ${errorMessage}</div>
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
      <button class="copy-code-btn" data-code="${encodeURIComponent(codeText)}" title="${t('ai.copy_code')}">
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

  // 应用代码块主题样式
  applyCodeBlockTheme()
}

// 新增函数：应用代码块主题样式
function applyCodeBlockTheme() {
  // 创建样式元素
  let styleElement = document.getElementById('ai-prism-theme-styles') as HTMLStyleElement
  if (!styleElement) {
    styleElement = document.createElement('style')
    styleElement.id = 'ai-prism-theme-styles'
    document.head.appendChild(styleElement)
  }


  // 根据当前主题生成CSS样式
  const cssContent = `
    /* AI助手代码块主题样式修复 - Okaidia 默认主题 */
    .chat-bubble pre {
      border: 1px solid rgba(255, 255, 255, 0.1) !important;
      border-radius: 0.5rem !important;
      overflow: auto !important;
      margin: 1rem 0 !important;
      padding: 0 !important;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1) !important;
    }

    .chat-bubble pre code {
      background: transparent !important;
      padding: 1rem !important;
      border: none !important;
      border-radius: 0 !important;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
      font-size: 0.875rem !important;
      line-height: 1.5 !important;
      white-space: pre-wrap !important;
      word-wrap: break-word !important;
      overflow-wrap: break-word !important;
      word-break: break-all !important;
      display: block !important;
      width: 100% !important;
      /* 修复重影问题 */
      text-shadow: none !important;
      font-weight: normal !important;
      text-decoration: none !important;
      font-style: normal !important;
    }

    .chat-bubble .code-block-container {
      margin: 1rem 0 !important;
      border-radius: 0.5rem !important;
      overflow: hidden !important;
      border: 1px solid rgba(255, 255, 255, 0.1) !important;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1) !important;
    }

    .chat-bubble .code-block-header {
      background: rgba(0, 0, 0, 0.2) !important;
      padding: 0.5rem 1rem !important;
      display: flex !important;
      justify-content: space-between !important;
      align-items: center !important;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
      font-size: 0.75rem !important;
    }

    .chat-bubble .code-language {
      font-weight: 500 !important;
      text-transform: uppercase !important;
      opacity: 0.8 !important;
    }

    .chat-bubble .copy-code-btn {
      opacity: 0.6 !important;
      transition: opacity 0.2s ease !important;
      background: transparent !important;
      border: none !important;
      cursor: pointer !important;
      padding: 0.25rem !important;
      border-radius: 0.25rem !important;
    }

    .chat-bubble .copy-code-btn:hover {
      opacity: 1 !important;
    }

    /* 行内代码样式修复 */
    .chat-bubble code:not(pre code) {
      background-color: rgba(255, 255, 255, 0.1) !important;
      color: rgb(251, 191, 36) !important;
      padding: 0.125rem 0.25rem !important;
      border-radius: 0.25rem !important;
      font-size: 0.875em !important;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
      border: 1px solid rgba(255, 255, 255, 0.2) !important;
      /* 修复重影问题 */
      text-shadow: none !important;
      font-weight: normal !important;
      text-decoration: none !important;
      font-style: normal !important;
    }

    /* 暗色主题适配 - 继续使用 okaidia 主题 */
    [data-theme="dark"] .chat-bubble pre,
    [data-theme="night"] .chat-bubble pre,
    [data-theme="black"] .chat-bubble pre,
    [data-theme="dracula"] .chat-bubble pre,
    [data-theme="halloween"] .chat-bubble pre,
    [data-theme="business"] .chat-bubble pre,
    [data-theme="luxury"] .chat-bubble pre,
    [data-theme="coffee"] .chat-bubble pre,
    [data-theme="forest"] .chat-bubble pre,
    [data-theme="synthwave"] .chat-bubble pre {
      border-color: rgba(255, 255, 255, 0.1) !important;
    }

    [data-theme="dark"] .chat-bubble pre code,
    [data-theme="night"] .chat-bubble pre code,
    [data-theme="black"] .chat-bubble pre code,
    [data-theme="dracula"] .chat-bubble pre code,
    [data-theme="halloween"] .chat-bubble pre code,
    [data-theme="business"] .chat-bubble pre code,
    [data-theme="luxury"] .chat-bubble pre code,
    [data-theme="coffee"] .chat-bubble pre code,
    [data-theme="forest"] .chat-bubble pre code,
    [data-theme="synthwave"] .chat-bubble pre code {
    }

    [data-theme="dark"] .chat-bubble .code-block-header,
    [data-theme="night"] .chat-bubble .code-block-header,
    [data-theme="black"] .chat-bubble .code-block-header,
    [data-theme="dracula"] .chat-bubble .code-block-header,
    [data-theme="halloween"] .chat-bubble .code-block-header,
    [data-theme="business"] .chat-bubble .code-block-header,
    [data-theme="luxury"] .chat-bubble .code-block-header,
    [data-theme="coffee"] .chat-bubble .code-block-header,
    [data-theme="forest"] .chat-bubble .code-block-header,
    [data-theme="synthwave"] .chat-bubble .code-block-header {
      background: rgba(0, 0, 0, 0.3) !important;
    }

    /* 亮色主题适配 - 使用 GitHub 风格 */
    [data-theme="light"] .chat-bubble pre,
    [data-theme="cupcake"] .chat-bubble pre,
    [data-theme="bumblebee"] .chat-bubble pre,
    [data-theme="emerald"] .chat-bubble pre,
    [data-theme="corporate"] .chat-bubble pre,
    [data-theme="retro"] .chat-bubble pre,
    [data-theme="cyberpunk"] .chat-bubble pre,
    [data-theme="valentine"] .chat-bubble pre,
    [data-theme="garden"] .chat-bubble pre,
    [data-theme="aqua"] .chat-bubble pre,
    [data-theme="lofi"] .chat-bubble pre,
    [data-theme="pastel"] .chat-bubble pre,
    [data-theme="fantasy"] .chat-bubble pre,
    [data-theme="wireframe"] .chat-bubble pre,
    [data-theme="cmyk"] .chat-bubble pre,
    [data-theme="autumn"] .chat-bubble pre,
    [data-theme="acid"] .chat-bubble pre,
    [data-theme="lemonade"] .chat-bubble pre,
    [data-theme="winter"] .chat-bubble pre {
      border-color: rgba(0, 0, 0, 0.1) !important;
    }

    [data-theme="light"] .chat-bubble pre code,
    [data-theme="cupcake"] .chat-bubble pre code,
    [data-theme="bumblebee"] .chat-bubble pre code,
    [data-theme="emerald"] .chat-bubble pre code,
    [data-theme="corporate"] .chat-bubble pre code,
    [data-theme="retro"] .chat-bubble pre code,
    [data-theme="cyberpunk"] .chat-bubble pre code,
    [data-theme="valentine"] .chat-bubble pre code,
    [data-theme="garden"] .chat-bubble pre code,
    [data-theme="aqua"] .chat-bubble pre code,
    [data-theme="lofi"] .chat-bubble pre code,
    [data-theme="pastel"] .chat-bubble pre code,
    [data-theme="fantasy"] .chat-bubble pre code,
    [data-theme="wireframe"] .chat-bubble pre code,
    [data-theme="cmyk"] .chat-bubble pre code,
    [data-theme="autumn"] .chat-bubble pre code,
    [data-theme="acid"] .chat-bubble pre code,
    [data-theme="lemonade"] .chat-bubble pre code,
    [data-theme="winter"] .chat-bubble pre code {
    }

    [data-theme="light"] .chat-bubble .code-block-header,
    [data-theme="cupcake"] .chat-bubble .code-block-header,
    [data-theme="bumblebee"] .chat-bubble .code-block-header,
    [data-theme="emerald"] .chat-bubble .code-block-header,
    [data-theme="corporate"] .chat-bubble .code-block-header,
    [data-theme="retro"] .chat-bubble .code-block-header,
    [data-theme="cyberpunk"] .chat-bubble .code-block-header,
    [data-theme="valentine"] .chat-bubble .code-block-header,
    [data-theme="garden"] .chat-bubble .code-block-header,
    [data-theme="aqua"] .chat-bubble .code-block-header,
    [data-theme="lofi"] .chat-bubble .code-block-header,
    [data-theme="pastel"] .chat-bubble .code-block-header,
    [data-theme="fantasy"] .chat-bubble .code-block-header,
    [data-theme="wireframe"] .chat-bubble .code-block-header,
    [data-theme="cmyk"] .chat-bubble .code-block-header,
    [data-theme="autumn"] .chat-bubble .code-block-header,
    [data-theme="acid"] .chat-bubble .code-block-header,
    [data-theme="lemonade"] .chat-bubble .code-block-header,
    [data-theme="winter"] .chat-bubble .code-block-header {
      border-bottom: 1px solid rgba(0, 0, 0, 0.1) !important;
    }

    [data-theme="light"] .chat-bubble .code-language,
    [data-theme="cupcake"] .chat-bubble .code-language,
    [data-theme="bumblebee"] .chat-bubble .code-language,
    [data-theme="emerald"] .chat-bubble .code-language,
    [data-theme="corporate"] .chat-bubble .code-language,
    [data-theme="retro"] .chat-bubble .code-language,
    [data-theme="cyberpunk"] .chat-bubble .code-language,
    [data-theme="valentine"] .chat-bubble .code-language,
    [data-theme="garden"] .chat-bubble .code-language,
    [data-theme="aqua"] .chat-bubble .code-language,
    [data-theme="lofi"] .chat-bubble .code-language,
    [data-theme="pastel"] .chat-bubble .code-language,
    [data-theme="fantasy"] .chat-bubble .code-language,
    [data-theme="wireframe"] .chat-bubble .code-language,
    [data-theme="cmyk"] .chat-bubble .code-language,
    [data-theme="autumn"] .chat-bubble .code-language,
    [data-theme="acid"] .chat-bubble .code-language,
    [data-theme="lemonade"] .chat-bubble .code-language,
    [data-theme="winter"] .chat-bubble .code-language {
      opacity: 0.7 !important;
    }

    [data-theme="light"] .chat-bubble .copy-code-btn,
    [data-theme="cupcake"] .chat-bubble .copy-code-btn,
    [data-theme="bumblebee"] .chat-bubble .copy-code-btn,
    [data-theme="emerald"] .chat-bubble .copy-code-btn,
    [data-theme="corporate"] .chat-bubble .copy-code-btn,
    [data-theme="retro"] .chat-bubble .copy-code-btn,
    [data-theme="cyberpunk"] .chat-bubble .copy-code-btn,
    [data-theme="valentine"] .chat-bubble .copy-code-btn,
    [data-theme="garden"] .chat-bubble .copy-code-btn,
    [data-theme="aqua"] .chat-bubble .copy-code-btn,
    [data-theme="lofi"] .chat-bubble .copy-code-btn,
    [data-theme="pastel"] .chat-bubble .copy-code-btn,
    [data-theme="fantasy"] .chat-bubble .copy-code-btn,
    [data-theme="wireframe"] .chat-bubble .copy-code-btn,
    [data-theme="cmyk"] .chat-bubble .copy-code-btn,
    [data-theme="autumn"] .chat-bubble .copy-code-btn,
    [data-theme="acid"] .chat-bubble .copy-code-btn,
    [data-theme="lemonade"] .chat-bubble .copy-code-btn,
    [data-theme="winter"] .chat-bubble .copy-code-btn {
      opacity: 0.6 !important;
    }

    [data-theme="light"] .chat-bubble .copy-code-btn:hover,
    [data-theme="cupcake"] .chat-bubble .copy-code-btn:hover,
    [data-theme="bumblebee"] .chat-bubble .copy-code-btn:hover,
    [data-theme="emerald"] .chat-bubble .copy-code-btn:hover,
    [data-theme="corporate"] .chat-bubble .copy-code-btn:hover,
    [data-theme="retro"] .chat-bubble .copy-code-btn:hover,
    [data-theme="cyberpunk"] .chat-bubble .copy-code-btn:hover,
    [data-theme="valentine"] .chat-bubble .copy-code-btn:hover,
    [data-theme="garden"] .chat-bubble .copy-code-btn:hover,
    [data-theme="aqua"] .chat-bubble .copy-code-btn:hover,
    [data-theme="lofi"] .chat-bubble .copy-code-btn:hover,
    [data-theme="pastel"] .chat-bubble .copy-code-btn:hover,
    [data-theme="fantasy"] .chat-bubble .copy-code-btn:hover,
    [data-theme="wireframe"] .chat-bubble .copy-code-btn:hover,
    [data-theme="cmyk"] .chat-bubble .copy-code-btn:hover,
    [data-theme="autumn"] .chat-bubble .copy-code-btn:hover,
    [data-theme="acid"] .chat-bubble .copy-code-btn:hover,
    [data-theme="lemonade"] .chat-bubble .copy-code-btn:hover,
    [data-theme="winter"] .chat-bubble .copy-code-btn:hover {
      background: rgba(0, 0, 0, 0.1) !important;
      opacity: 1 !important;
    }

    /* 行内代码在亮色主题下的样式 */
    [data-theme="light"] .chat-bubble code:not(pre code),
    [data-theme="cupcake"] .chat-bubble code:not(pre code),
    [data-theme="bumblebee"] .chat-bubble code:not(pre code),
    [data-theme="emerald"] .chat-bubble code:not(pre code),
    [data-theme="corporate"] .chat-bubble code:not(pre code),
    [data-theme="retro"] .chat-bubble code:not(pre code),
    [data-theme="cyberpunk"] .chat-bubble code:not(pre code),
    [data-theme="valentine"] .chat-bubble code:not(pre code),
    [data-theme="garden"] .chat-bubble code:not(pre code),
    [data-theme="aqua"] .chat-bubble code:not(pre code),
    [data-theme="lofi"] .chat-bubble code:not(pre code),
    [data-theme="pastel"] .chat-bubble code:not(pre code),
    [data-theme="fantasy"] .chat-bubble code:not(pre code),
    [data-theme="wireframe"] .chat-bubble code:not(pre code),
    [data-theme="cmyk"] .chat-bubble code:not(pre code),
    [data-theme="autumn"] .chat-bubble code:not(pre code),
    [data-theme="acid"] .chat-bubble code:not(pre code),
    [data-theme="lemonade"] .chat-bubble code:not(pre code),
    [data-theme="winter"] .chat-bubble code:not(pre code) {
      background-color: rgba(175, 184, 193, 0.2) !important;
      color: rgb(124, 58, 237) !important;
      border: 1px solid rgba(0, 0, 0, 0.15) !important;
    }
  `

  styleElement.textContent = cssContent
  console.log('已应用AI助手代码块主题样式')
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

// 滚动事件防抖
let scrollTimeout: number | null = null
let isUserScrolling = ref(false)

// 处理滚动事件 - 适配反向布局
const handleScroll = () => {
  if (!messagesContainer.value) return

  const container = messagesContainer.value
  const currentScrollTop = container.scrollTop

  // 在反向布局中，scrollTop = 0 表示在最底部（最新消息）
  // scrollTop > 0 表示向上滚动查看历史消息
  const distanceFromBottom = currentScrollTop

  // 检测滚动方向
  isScrollingUp.value = currentScrollTop > lastScrollTop.value
  lastScrollTop.value = currentScrollTop

  // 判断是否在底部附近（显示最新消息的位置）
  const nearBottom = distanceFromBottom <= scrollThreshold.value
  isAtBottom.value = nearBottom

  // 标记用户正在滚动
  isUserScrolling.value = true

  // 如果用户向上滚动查看历史消息，禁用自动滚动
  if (isScrollingUp.value && distanceFromBottom > scrollThreshold.value) {
    shouldAutoScroll.value = false
  }

  // 如果用户滚动回底部附近，恢复自动滚动
  if (nearBottom) {
    shouldAutoScroll.value = true
  }

  // 清除之前的定时器
  if (scrollTimeout) {
    clearTimeout(scrollTimeout)
  }


}



// 清理 IntersectionObserver
const cleanupIntersectionObserver = () => {
  if (intersectionObserver.value) {
    intersectionObserver.value.disconnect()
    intersectionObserver.value = null
  }
}

// 智能滚动到底部 - 适配反向布局
const scrollToBottom = () => {
  if (!messagesContainer.value || !shouldAutoScroll.value || isUserScrolling.value) return

  const container = messagesContainer.value

  // 使用 requestAnimationFrame 优化性能
  requestAnimationFrame(() => {
    // 再次检查用户是否在滚动，避免干扰用户操作
    if (isUserScrolling.value) return

    // 在反向布局中，scrollTop = 0 表示最底部（最新消息）
    container.scrollTo({
      top: 0,
      behavior: isAtBottom.value ? 'smooth' : 'auto'
    })
  })
}

// 强制滚动到底部（用于新消息） - 适配反向布局
const forceScrollToBottom = () => {
  if (!messagesContainer.value) return

  const container = messagesContainer.value

  // 重置所有滚动状态
  shouldAutoScroll.value = true
  isUserScrolling.value = false
  isScrollingUp.value = false
  isAtBottom.value = true

  requestAnimationFrame(() => {
    // 在反向布局中，scrollTop = 0 表示最底部（最新消息）
    container.scrollTo({
      top: 0,
      behavior: 'smooth'
    })

    // 更新最后滚动位置
    setTimeout(() => {
      lastScrollTop.value = container.scrollTop
    }, 100)
  })
}

// 生成唯一ID
const generateUniqueId = () => {
  return Date.now().toString() + Math.random().toString(36).substring(2, 15)
}

// 清空当前对话消息
const clearMessages = async () => {
  console.log('clearMessages 被调用，activeConversationId:', activeConversationId.value);

  // 如果没有活动对话ID，直接清空当前显示的消息
  if (!activeConversationId.value) {
    console.log('没有活动对话ID，直接清空消息数组');
    messages.value = [];
    return;
  }

  try {
    console.log('调用后端清空对话API...');
    // 调用后端清空当前对话的消息
    await invoke('clear_ai_conversation', { conversation_id: activeConversationId.value });

    console.log('后端API调用成功，重新加载消息...');
    // 重新加载消息以更新UI
    await loadMessages(activeConversationId.value);
    console.log('消息重新加载完成');

    // 清空后确保滚动状态正确 - 适配反向布局
    await nextTick()
    shouldAutoScroll.value = true
    isAtBottom.value = true
  } catch (error) {
    console.error('清空对话失败:', error);
    // 即使后端调用失败，也尝试清空前端显示的消息
    messages.value = [];
    // 确保滚动状态正确 - 适配反向布局
    shouldAutoScroll.value = true
    isAtBottom.value = true
  }
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
  const model = availableModels.value.find((m: any) => m.id === modelId)
  return model ? model.name : modelId
}

// 获取选中的模型名称
const getSelectedModelName = () => {
  if (!selectedModel.value) return ''
  const model = availableModels.value.find((m: any) => m.id === selectedModel.value)
  return model ? model.name : ''
}

// 获取模型头像路径
const getModelAvatarPath = (modelId: string) => {
  // 对于自定义模型（以custom_开头），使用通用的custom头像
  if (modelId.startsWith('custom_')) {
    return '/img/custom-avatar.svg'
  }
  // 对于其他模型，使用对应的头像文件
  return `/img/${modelId}-avatar.svg`
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
  console.log('AI Assistant component mounted.')
  window.addEventListener('resize', onResize)

  await reloadAIConfig() // 初始加载配置
  await loadConversations()

  setupStreamListeners()
  setupCodeCopyFeature()
  enhanceCodeBlocks() // 初始代码块增强

  // 监听全局设置变化
  unlistenSettings = await listen('global-settings-changed', (event: any) => {
    if (event.payload.key === 'aiConfig') {
      reloadAIConfig()
    }
  })

  // 立即加载一次笔记
  if (tipsStore.tips.length === 0) {
    tipsStore.fetchTips(true)
  }

  // 观察文档元素的 data-theme 属性变化
  themeObserver = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === 'attributes' && mutation.attributeName === 'data-theme') {
        console.log('Theme changed, reapplying code block styles.')
        setTimeout(() => {
          applyCodeBlockTheme()
        }, 100)
      }
    })
  })

  if (document.documentElement) {
    themeObserver.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme']
    })
  }

  console.log('Component mount logic finished.')

  if (messagesContainer.value) {
    messagesContainer.value.addEventListener('click', handleLinkClick)
  }
})

onActivated(async () => {
  console.log('AI Assistant component activated.')
  window.addEventListener('resize', onResize)
  await reloadAIConfig() // 激活时重新加载配置

  // 重新设置监听器
  if (!unlistenSettings) {
    unlistenSettings = await listen('global-settings-changed', (event: any) => {
      if (event.payload.key === 'aiConfig') {
        reloadAIConfig()
      }
    })
  }

  // 强制滚动到底部
  await nextTick()
  forceScrollToBottom()
})

onDeactivated(() => {
  console.log('AI Assistant component deactivated.')
  window.removeEventListener('resize', onResize)
  // 移除监听器
  if (unlistenSettings) {
    unlistenSettings()
    unlistenSettings = null
  }

  if (messagesContainer.value) {
    messagesContainer.value.removeEventListener('click', handleLinkClick)
  }
})

onBeforeUnmount(() => {
  console.log('AI Assistant component unmounting.')
  window.removeEventListener('resize', onResize)

  // 保存笔记状态
  localStorage.setItem('ai-show-note-panel', showNotePanel.value.toString())
  localStorage.setItem('ai-note-title', noteTitle.value)
  localStorage.setItem('ai-note-content', noteContent.value)

  // 清理滚动观察器和定时器
  cleanupIntersectionObserver()
  if (scrollTimeout) {
    window.clearTimeout(scrollTimeout)
    scrollTimeout = null
  }

  // 清理性能优化相关的缓存
  messageRenderCache.value.clear()
  formatMessageDebounced.value.forEach(timeout => window.clearTimeout(timeout))
  formatMessageDebounced.value.clear()

  // 清理主题样式和观察器
  const styleElement = document.getElementById('ai-prism-theme-styles')
  if (styleElement) {
    styleElement.remove()
  }
  if (themeObserver) {
    themeObserver.disconnect()
    themeObserver = null
  }

  // 移除监听器
  if (unlistenSettings) {
    unlistenSettings()
    unlistenSettings = null
  }

  if (messagesContainer.value) {
    messagesContainer.value.removeEventListener('click', handleLinkClick)
  }
})




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
  if (!userInput.value.trim() || !selectedModel.value || (!selectedModel.value.startsWith('custom_') && !hasApiKey.value)) {
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
    await invoke('cancel_ai_stream', {
      streamId: currentStreamingId.value
    });

    // 如果已经有一些内容，将其添加为完整消息
    if (streamingContent.value.trim()) {
      await invoke('add_ai_message', { conversationId: activeConversationId.value, role: 'assistant', content: streamingContent.value + ' [已取消]' })
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
    await invoke('update_ai_conversation_title', { conversationId: editingConversationId.value, newTitle: editingTitle.value.trim() })
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
  if (isMobile.value) {
    openNotePanelModal()
  } else {
    showNotePanel.value = true
  }

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

const openNotePanelModal = () => {
  if (notePanelModal.value) {
    notePanelModal.value.showModal()
  }
}

const closeNotePanelModal = () => {
  if (notePanelModal.value) {
    notePanelModal.value.close()
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
    console.log('保存笔记 - 条件不满足，取消保存')
    return
  }

  console.log('保存笔记 - 开始保存:', { title: noteTitle.value, contentLength: noteContent.value.length })
  isNoteSaving.value = true

  try {
    // 获取第一个分类作为默认分类
    const defaultCategoryId = tipsStore.categories.length > 0 ? tipsStore.categories[0].id : undefined
    console.log('保存笔记 - 使用分类ID:', defaultCategoryId, '分类列表长度:', tipsStore.categories.length)

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

    console.log('保存笔记 - 准备保存的数据:', newTip)

    // 保存到数据库
    const savedTip = await tipsStore.saveTip(newTip)
    console.log('保存笔记 - 保存结果:', savedTip)

    if (savedTip) {
      console.log('保存笔记 - 保存成功，笔记ID:', savedTip.id)

      // 显示成功提示
      showSaveSuccess.value = true
      setTimeout(() => {
        showSaveSuccess.value = false
      }, 3000)

      // 清空笔记内容并隐藏笔记面板
      noteTitle.value = ''
      noteContent.value = ''
      isNotePreviewMode.value = false // 重置预览模式
      showNotePanel.value = false // 隐藏笔记面板

      console.log('保存笔记 - 已清空输入内容并隐藏笔记面板')
    } else {
      console.error('保存笔记 - 保存失败，返回值为空')
    }
  } catch (error) {
    console.error('保存笔记失败:', error)
  } finally {
    isNoteSaving.value = false
    console.log('保存笔记 - 完成保存流程')
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
      showAlert(`文件 ${file.name} 超过10MB限制`, { title: '文件过大' })
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
    const result = await invoke('list_ai_roles')
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
      await invoke('update_ai_role', { roleId: editingRole.value.id, name: newRoleName.value.trim(), description: newRoleDescription.value.trim() })
    } else {
      // 创建新角色
      await invoke('create_ai_role', { name: newRoleName.value.trim(), description: newRoleDescription.value.trim() })
    }

    await loadRoles()
    closeRoleForm()
  } catch (error) {
    console.error('保存角色失败:', error)
  }
}

const confirmDeleteRole = async (roleId: string) => {
  const confirmed = await showConfirm(t('ai.confirm_delete_role'), {
    title: t('ai.confirm_delete_role'),
    confirmText: t('ai.delete'),
    cancelText: t('ai.cancel')
  })

  if (confirmed) {
    try {
      await invoke('delete_ai_role', { roleId })
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
    return `${t('ai.as')} ${selectedRole.value.name} ${t('ai.with_you')}...${t('ai.input_hash_to_reference_notes')}`
  } else {
    return `${t('ai.enter_your_question_or_instruction')}...${t('ai.input_hash_to_reference_notes')}`
  }
}

// 监听笔记搜索查询
watch(noteSearchQuery, (newQuery) => {
  if (newQuery.trim()) {
    // 搜索匹配的笔记
    const searchResults = tipsStore.tips.filter(note =>
      (note.content || '').toLowerCase().includes(newQuery.toLowerCase())
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

// 复制状态管理
const copyingStates = ref<Record<string, 'copying' | 'success' | null>>({})

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

function getDefaultMaxTokens(modelId: string): number {
  const defaults: Record<string, number> = {
    chatgpt: 4000,
    gemini: 4000,
    deepseek: 3000,
    claude: 4000,
    qwen: 3000,
    doubao: 3000,
    grok: 4000,
    custom: 3000
  }
  return defaults[modelId] || 3000
}

// 将内容复制到剪贴板
// const copyToClipboard = async (content: string) => {
//   try {
//     await navigator.clipboard.writeText(content)
//   } catch (error) {
//     console.error('复制到剪贴板失败:', error)
//   }
// }

// 带反馈的复制到剪贴板
const copyToClipboardWithFeedback = async (content: string, event: Event) => {
  const target = event.target as HTMLElement
  const button = target.closest('button')
  if (!button) return

  // 通过消息索引和角色生成唯一键
  const messageElement = button.closest('.chat')
  const messageIndex = Array.from(messageElement?.parentElement?.children || []).indexOf(messageElement!)
  const role = messageElement?.classList.contains('chat-end') ? 'user' : 'assistant'
  const key = `${messageIndex}_${role}`

  try {
    copyingStates.value[key] = 'copying'
    await navigator.clipboard.writeText(content)
    copyingStates.value[key] = 'success'

    // 2秒后重置状态
    setTimeout(() => {
      copyingStates.value[key] = null
    }, 2000)
  } catch (error) {
    console.error('复制到剪贴板失败:', error)
    copyingStates.value[key] = null
  }
}

// 重发消息
const resendMessage = async (originalMessage: any) => {
  if (!originalMessage || !originalMessage.content) return

  // 创建新的用户消息
  const newMessage = {
    role: 'user',
    content: originalMessage.content,
    timestamp: Date.now(),
    attachments: originalMessage.attachments || [],
    referencedNotes: originalMessage.referencedNotes || []
  }

  // 添加到消息列表
  messages.value.push(newMessage)

  // 保存到数据库
  let messageContent = originalMessage.content
  if (newMessage.attachments.length > 0) {
    const attachmentsJson = JSON.stringify(newMessage.attachments)
    messageContent += `\n\n__ATTACHMENTS__:${attachmentsJson}`
  }
  if (newMessage.referencedNotes.length > 0) {
    const notesJson = JSON.stringify(newMessage.referencedNotes)
    messageContent += `\n\n__REFERENCED_NOTES__:${notesJson}`
  }

  await invoke('add_ai_message', { conversationId: activeConversationId.value, role: 'user', content: messageContent })

  // 滚动到底部
  await nextTick()
  forceScrollToBottom()

  // 发送AI请求
  await sendAIRequest(originalMessage.content, newMessage.attachments || [])
}

// 发送AI请求（从原有sendMessage函数中提取的逻辑）
const sendAIRequest = async (messageContent: string, attachments: any[] = []) => {
  // 设置加载状态
  isLoading.value = true

  try {
    // 开始流式输出准备
    isStreaming.value = true
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
    console.log(customModelName)
    if (selectedModel.value.startsWith('custom_')) {
      const config = aiConfigs.value[selectedModel.value]
      if (config && config.model_name) {
        customModelName = config.model_name
      } else {
        console.warn(`未找到自定义模型 ${selectedModel.value} 的配置`)
      }
    }

    // 检查是否有图片附件
    const imageAttachments = attachments.filter((att: any) => att.type.startsWith('image/'))
    if (imageAttachments.length > 0) {
      // 重新处理图片数据
      const imageFileData = await Promise.all(
        imageAttachments.map(async (att: any) => {
          const response = await fetch(att.url)
          const arrayBuffer = await response.arrayBuffer()
          const uint8Array = new Uint8Array(arrayBuffer)
          return [att.name, Array.from(uint8Array)] as [string, number[]]
        })
      )

      await invoke('send_ai_message_with_images_stream', {
        providerId: selectedModel.value,
        textMessage: messageContent,
        imageFiles: imageFileData,
        streamId: currentStreamingId.value,
        conversationId: activeConversationId.value,
        roleId: selectedRole.value?.id || null
      })
    } else {
      // 没有图片文件，使用普通API
      // 如果有非图片附件，添加提示信息
      let finalMessage = messageContent
      if (attachments.length > 0) {
        finalMessage += `\n\n[${t('ai.user_uploaded')} ${attachments.length} {{ $t('ai.files') }}: ${attachments.map(a => a.name).join(', ')}，{{ $t('ai.but_the_current_model_does_not_support_file_processing') }}]`
      }

      await invoke('send_ai_message_stream', {
        providerId: selectedModel.value,
        message: finalMessage,
        streamId: currentStreamingId.value,
        conversationId: activeConversationId.value,
        roleId: selectedRole.value?.id || null
      })
    }
  } catch (error) {
    console.error('AI响应失败:', error)
    isStreaming.value = false
    streamingContent.value = ''
    currentStreamingId.value = null
    isLoading.value = false
  }
}

// 笔记内容的markdown渲染
const renderedNoteContent = computed(() => {
  if (!noteContent.value) return `<p class="text-base-content/50">${t('ai.no_content')}</p>`

  try {
    // 创建 marked 实例并配置高亮
    const marked = new Marked()

    // 使用 marked-highlight 扩展
    marked.use(markedHighlight({
      langPrefix: 'language-',
      highlight(code: string, lang: string) {
        const actualLang = lang || 'plaintext'

        if (actualLang && isPrismLanguageAvailable(actualLang)) {
          try {
            return Prism.highlight(code, Prism.languages[actualLang], actualLang)
          } catch (error) {
            console.warn(`Prism 高亮失败 (${actualLang}):`, error)
            return escapeHtml(code)
          }
        }

        return escapeHtml(code)
      }
    }))

    // 配置 marked 选项
    marked.setOptions({
      breaks: true,
      gfm: true,
      pedantic: false,
      silent: true,
    })

    // 使用 marked 渲染 Markdown
    const htmlContent = marked.parse(noteContent.value) as string

    // 使用DOMPurify清理HTML，防止XSS
    return DOMPurify.sanitize(htmlContent, {
      ADD_TAGS: ['iframe', 'pre', 'code'],
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled']
    })
  } catch (err) {
    console.error('笔记Markdown渲染错误:', err)
    const errorMessage = err instanceof Error ? err.message : String(err)
    return `<div class="text-error">${t('ai.markdown_rendering_error')}: ${errorMessage}</div>
            <pre>${DOMPurify.sanitize(noteContent.value)}</pre>`
  }
})

// 监听预览模式变化
watch(isNotePreviewMode, (newValue) => {
  console.log('预览模式状态变化:', newValue)
  console.log('当前笔记内容长度:', noteContent.value.length)
})

// 监听笔记内容变化
watch(noteContent, (newValue) => {
  console.log('笔记内容变化，新长度:', newValue.length)
})

// 处理模型选择变化
const handleModelChange = () => {
  const currentValue = selectedModel.value
  if (currentValue === '__configure__') {
    // 恢复到上一个选择的模型（如果有的话）
    const lastModel = localStorage.getItem('ai-selected-model')
    if (lastModel && lastModel !== '__configure__' && availableModels.value.some((m: any) => m.id === lastModel)) {
      selectedModel.value = lastModel
    } else if (availableModels.value.length > 0) {
      selectedModel.value = availableModels.value[0].id
    } else {
      selectedModel.value = ''
    }
    // 跳转到设置页面的AI助手部分
    goToAISettings()
  }
}

// 前往设置页面
const goToAISettings = () => {
  router.push({ path: '/settings', query: { page: 'ai' } })
}

// 处理链接点击事件
const handleLinkClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  const anchor = target.closest('a')
  if (anchor && anchor.href) {
    const href = anchor.href
    if (href.startsWith('http://') || href.startsWith('https://')) {
      event.preventDefault()
      open(href).catch((err: any) => console.error('Failed to open link:', err))
    }
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
  /* 优化渲染性能 */
  contain: layout style paint;
  /* 避免布局抖动 */
  transform: translateZ(0);
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

/* 消息容器优化滚动 - 反向布局 */
.chat-container .flex-grow {
  scroll-behavior: smooth;
  /* 使用 GPU 加速 */
  transform: translateZ(0);
  /* 优化滚动性能 */
  will-change: scroll-position;
  /* 反向布局：最新消息在顶部，历史消息向下滚动 */
  display: flex;
  flex-direction: column-reverse;
}

/* 消息间距保持正常 */
.chat-container .space-y-4>*+* {
  margin-top: 1rem;
  margin-bottom: 0;
}



/* 确保按钮在聊天框内的正确定位 */
.chat-container {
  position: relative;
}



/* 底部哨兵元素样式 */
.h-1.w-full {
  pointer-events: none;
  user-select: none;
  /* 确保不影响布局 */
  flex-shrink: 0;
}

/* 复制按钮动画效果 */
.chat-footer .btn {
  transition: all 0.2s ease;
}

.chat-footer .btn:hover {
  transform: translateY(-1px);
}

.chat-footer .btn:disabled {
  opacity: 0.7;
}

/* 复制成功状态样式 */
.chat-footer .btn:disabled.copy-success {
  opacity: 1 !important;
  background-color: rgba(16, 185, 129, 0.1) !important;
  border-color: rgba(16, 185, 129, 0.3) !important;
  color: #10b981 !important;
  animation: copy-success-pulse 0.3s ease-out;
}

@keyframes copy-success-pulse {
  0% {
    transform: scale(1);
  }

  50% {
    transform: scale(1.05);
  }

  100% {
    transform: scale(1);
  }
}

.chat-footer .btn .text-success {
  color: #10b981 !important;
}

.chat-footer .btn .animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
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

/* AI thinking状态的脉冲动画 */
.ai-thinking {
  animation: ai-pulse 2s ease-in-out infinite;
}

@keyframes ai-pulse {

  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }

  50% {
    opacity: 0.8;
    transform: scale(1.02);
  }
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
  /* 修复重影问题 */
  text-shadow: none !important;
  font-weight: normal !important;
  text-decoration: none !important;
  font-style: normal !important;
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
  /* 修复重影问题 */
  text-shadow: none !important;
  font-weight: normal !important;
  text-decoration: none !important;
  font-style: normal !important;
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
  font-size: var(--base-font-size);
  /* 继承全局字体大小 */
}

/* 确保prose内的所有元素都能正确继承字体大小 */
:deep(.prose *) {
  font-size: inherit;
}

/* 为不同标题级别设置相对大小 */
:deep(.prose h1) {
  font-size: calc(var(--base-font-size) * 2.25);
  /* 相当于 text-4xl */
}

:deep(.prose h2) {
  font-size: calc(var(--base-font-size) * 1.875);
  /* 相当于 text-3xl */
}

:deep(.prose h3) {
  font-size: calc(var(--base-font-size) * 1.5);
  /* 相当于 text-2xl */
}

:deep(.prose h4) {
  font-size: calc(var(--base-font-size) * 1.25);
  /* 相当于 text-xl */
}

:deep(.prose h5) {
  font-size: calc(var(--base-font-size) * 1.125);
  /* 相当于 text-lg */
}

:deep(.prose h6) {
  font-size: var(--base-font-size);
  /* 相当于 text-base */
}

/* 编辑/预览按钮组样式 */
.btn-group .btn {
  transition: all 0.2s ease;
}

.btn-group .btn:hover {
  transform: translateY(-1px);
}

.btn-group .btn-active {
  background-color: hsl(var(--primary));
  color: hsl(var(--primary-content));
}

.btn-group .btn svg {
  transition: transform 0.2s ease;
}

.btn-group .btn:hover svg {
  transform: scale(1.1);
}

/* 确保编辑/预览按钮在所有主题下都有正确的颜色 */
[data-theme="dark"] .btn-group .btn-active,
[data-theme="night"] .btn-group .btn-active,
[data-theme="black"] .btn-group .btn-active,
[data-theme="dracula"] .btn-group .btn-active,
[data-theme="halloween"] .btn-group .btn-active,
[data-theme="business"] .btn-group .btn-active,
[data-theme="luxury"] .btn-group .btn-active,
[data-theme="coffee"] .btn-group .btn-active,
[data-theme="forest"] .btn-group .btn-active,
[data-theme="synthwave"] .btn-group .btn-active {
  background-color: hsl(var(--primary)) !important;
  color: hsl(var(--primary-content)) !important;
}
</style>
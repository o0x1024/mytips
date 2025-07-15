<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Marked } from 'marked'
import { markedHighlight } from 'marked-highlight'
import DOMPurify from 'dompurify'
import Prism from 'prismjs'
import 'prismjs/themes/prism-okaidia.css'
import { getDefaultAIModel } from '../services/aiService'

// 检查是否在浏览器环境
const isBrowser = typeof window !== 'undefined'

// 窗口尺寸
const windowWidth = ref(isBrowser ? window.innerWidth : 0)
const windowHeight = ref(isBrowser ? window.innerHeight : 0)
const isMobile = computed(() => windowWidth.value < 768)

// 控制浮动窗口的显示
const showFloatingChat = ref(false)
// 控制按钮是否半隐藏
const isButtonHidden = ref(false)
// 控制按钮是否在右边缘
const isRightEdge = ref(false)
// 控制按钮位置
const position = ref({ 
  x: isBrowser ? windowWidth.value - 70 : 0, 
  y: isBrowser ? windowHeight.value / 1.2 : 0 
})
// 拖动状态
const isDragging = ref(false)
const dragOffset = ref({ x: 0, y: 0 })
// 记录拖动开始位置，用于判断是否真正拖动了
const dragStartPosition = ref({ x: 0, y: 0 })
// 消息和对话内容
const userMessage = ref('')
const chatMessages = ref<Array<{role: string, content: string, timestamp: number}>>([])
// 加载状态
const isLoading = ref(false)
// 取消生成相关
const isCancelling = ref(false)

// 浮动聊天框对话ID
const floatingConversationId = ref<string | null>(null)
const isLoadingHistory = ref(false)

// 聊天框大小调整
const chatWindowSize = ref({ width: 500, height: 600 })
const isResizing = ref(false)
const resizeHandle = ref('')
const resizeStartPos = ref({ x: 0, y: 0 })
const resizeStartSize = ref({ width: 0, height: 0 })
const resizeStartRect = ref({ left: 0, top: 0, right: 0, bottom: 0 })

// 消息区域引用
const messagesContainer = ref<HTMLElement | null>(null)

// 选择的模型
const selectedModel = ref('chatgpt')
// 可用的AI模型
const availableModels = ref([
  { id: 'chatgpt', name: 'ChatGPT', avatar: '/img/chatgpt-avatar.svg' },
  { id: 'gemini', name: 'Google Gemini', avatar: '/img/gemini-avatar.svg' },
  { id: 'claude', name: 'Claude', avatar: '/img/claude-avatar.svg' },
  { id: 'qwen', name: '通义千问', avatar: '/img/qwen-avatar.svg' },
  { id: 'deepseek', name: 'DeepSeek', avatar: '/img/deepseek-avatar.svg' },
  { id: 'doubao', name: '豆包', avatar: '/img/doubao-avatar.svg' },
  { id: 'grok', name: 'Grok', avatar: '/img/grok-avatar.svg' },
])

// 计算聊天窗口位置
const chatWindowPosition = computed(() => {
  if (isMobile.value) {
    return {
      top: '0px',
      left: '0px',
      width: '100vw',
      height: '100vh',
      borderRadius: '0px',
      maxWidth: '100vw',
      maxHeight: '100vh',
    }
  }

  const buttonSize = 50; // AI按钮的尺寸
  const margin = 10; // 窗口与屏幕边缘的间距

  // 默认情况下，弹出框的底部与按钮的顶部对齐
  let top = position.value.y - chatWindowSize.value.height;

  // 根据按钮在左侧还是右侧，决定弹出框的水平对齐方式
  let left;
  if (isRightEdge.value) {
    // 如果按钮在右侧，则弹出框的右边缘与按钮的右边缘对齐
    left = position.value.x + buttonSize - chatWindowSize.value.width;
  } else {
    // 否则，弹出框的左边缘与按钮的左边缘对齐
    left = position.value.x;
  }

  // --- 边界碰撞检测 ---
  
  // 确保弹出框不会超出屏幕顶部
  if (top < margin) {
    top = margin;
  }
  
  // 确保弹出框不会超出屏幕底部
  if (top + chatWindowSize.value.height > windowHeight.value - margin) {
    top = windowHeight.value - chatWindowSize.value.height - margin;
  }
  
  // 确保弹出框不会超出屏幕左边
  if (left < margin) {
    left = margin;
  }
  
  // 确保弹出框不会超出屏幕右边
  if (left + chatWindowSize.value.width > windowWidth.value - margin) {
    left = windowWidth.value - chatWindowSize.value.width - margin;
  }

  return {
    left: `${left}px`,
    top: `${top}px`,
    width: `${chatWindowSize.value.width}px`,
    height: `${chatWindowSize.value.height}px`
  }
})

// 获取模型头像
const getModelAvatar = (modelId: string) => {
  const model = availableModels.value.find(m => m.id === modelId)
  return model ? model.avatar : '/img/custom-avatar.svg'
}

// 获取模型名称
const getModelName = (modelId: string) => {
  const model = availableModels.value.find(m => m.id === modelId)
  return model ? model.name : '自定义模型'
}

// 加载全局默认AI模型
const loadDefaultModel = async () => {
  let modelId = 'chatgpt' // Fallback model

  try {
    const defaultModel = await getDefaultAIModel('chat')
    if (defaultModel && defaultModel.provider && availableModels.value.some(m => m.id === defaultModel.provider)) {
      modelId = defaultModel.provider
    } else if (defaultModel && defaultModel.provider) {
        console.warn(`FloatingAI: Global default model '${defaultModel.provider}' is not available, using fallback.`)
    } else {
        console.log(`FloatingAI: No global default model configured, using fallback.`)
    }
  } catch (error) {
    console.error('FloatingAI: Failed to get global default AI model:', error)
  }

  selectedModel.value = modelId
}

// 获取或创建浮动聊天框对话
const getOrCreateFloatingConversation = async () => {
  try {
    // 检查是否有现有的浮动聊天框对话
    const conversations = await invoke('list_ai_conversations') as Array<{id: string, title: string, model: string}>
    const floatingConv = conversations.find(c => c.title === '浮动聊天框')
    
    if (floatingConv) {
      floatingConversationId.value = floatingConv.id
      console.log('FloatingAI: 找到现有浮动聊天框对话:', floatingConv.id)
    } else {
      // 创建新的浮动聊天框对话
      const newConvId = await invoke('create_ai_conversation', { model: selectedModel.value, title: '浮动聊天框' }) as string
      floatingConversationId.value = newConvId
      console.log('FloatingAI: 创建新的浮动聊天框对话:', newConvId)
    }
  } catch (error) {
    console.error('FloatingAI: 获取或创建浮动聊天框对话失败:', error)
    // 即使失败也不阻止聊天功能，只是不会持久化
  }
}

// 加载聊天历史
const loadChatHistory = async () => {
  if (!floatingConversationId.value) return
  
  isLoadingHistory.value = true
  try {
    const messages = await invoke('list_ai_messages', { conversationId: floatingConversationId.value }) as Array<{
      id: string,
      role: string, 
      content: string,
      timestamp: number
    }>
    
    chatMessages.value = messages.map(msg => ({
      role: msg.role,
      content: msg.content,
      timestamp: msg.timestamp
    }))
    
    console.log(`FloatingAI: 加载了 ${chatMessages.value.length} 条聊天历史`)
    
    // 滚动到底部
    scrollToBottom()
  } catch (error) {
    console.error('FloatingAI: 加载聊天历史失败:', error)
  } finally {
    isLoadingHistory.value = false
  }
}

// 保存消息到数据库
const saveMessageToDatabase = async (role: string, content: string) => {
  if (!floatingConversationId.value) {
    await getOrCreateFloatingConversation()
  }
  
  if (floatingConversationId.value) {
    try {
      await invoke('add_ai_message', { 
        conversationId: floatingConversationId.value, 
        role, 
        content 
      })
    } catch (error) {
      console.error('FloatingAI: 保存消息到数据库失败:', error)
    }
  }
}

// 自动滚动到底部
const scrollToBottom = () => {
  if (messagesContainer.value) {
    setTimeout(() => {
      if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
      }
    }, 10)
  }
}

// 当前流式回复状态
const currentStreamingId = ref('')
const streamingContent = ref('')
const isStreaming = ref(false)

// 生成唯一ID
const generateUniqueId = () => {
  return Date.now().toString(36) + Math.random().toString(36).substr(2)
}

// 监听流式输出事件
const handleStreamChunk = async (event: any) => {
  const { id, chunk, done } = event.payload
  
  if (id !== currentStreamingId.value) return
  
  if (done) {
    // 流结束，将流式内容添加到聊天记录
    if (streamingContent.value.trim()) {
      const assistantMsg = {
        role: 'assistant',
        content: streamingContent.value,
        timestamp: Date.now()
      }
      
      chatMessages.value.push(assistantMsg)
      
      // 保存AI回复到数据库
      await saveMessageToDatabase('assistant', streamingContent.value)
    }
    
    // 重置状态
    isLoading.value = false
    isStreaming.value = false
    streamingContent.value = ''
    currentStreamingId.value = ''
    
    // 稍后滚动到底部，确保新消息已渲染
    setTimeout(() => {
      scrollToBottom()
    }, 50)
  } else {
    // 接收到新的内容块
    streamingContent.value += chunk
    
    // 定期滚动，但不太频繁以避免性能问题
    if (chunk.length > 10 || streamingContent.value.length % 100 === 0) {
      scrollToBottom()
    }
  }
}

// 发送消息（使用流式输出）
const sendMessage = async () => {
  if (!userMessage.value.trim() || isLoading.value) return
  
  const message = userMessage.value
  
  // 添加用户消息到聊天记录
  const userMsg = {
    role: 'user',
    content: message,
    timestamp: Date.now()
  }
  chatMessages.value.push(userMsg)
  
  // 保存用户消息到数据库
  await saveMessageToDatabase('user', message)
  
  // 滚动到底部
  scrollToBottom()
  
  userMessage.value = ''
  isLoading.value = true
  isStreaming.value = true
  streamingContent.value = ''
  
  // 生成流式输出ID
  currentStreamingId.value = generateUniqueId()
  
  try {
    // 调用流式AI API
    await invoke('send_ai_message_stream', {
      providerId: selectedModel.value,
      message: message,
      streamId: currentStreamingId.value,
      conversationId: floatingConversationId.value,
      roleId: null
    })
  } catch (error) {
    console.error('发送消息失败:', error)
    
    const errorMsg = {
      role: 'assistant',
      content: '抱歉，发生了错误：' + (error as string),
      timestamp: Date.now()
    }
    
    // 添加错误消息到聊天记录
    chatMessages.value.push(errorMsg)
    
    // 保存错误消息到数据库
    await saveMessageToDatabase('assistant', errorMsg.content)
    
    // 重置状态
    isLoading.value = false
    isStreaming.value = false
    streamingContent.value = ''
    currentStreamingId.value = ''
    scrollToBottom()
  }
}

// 处理按钮鼠标按下事件
const handleMouseDown = (event: MouseEvent) => {
  event.preventDefault()
  isDragging.value = true
  
  dragStartPosition.value = { x: event.clientX, y: event.clientY }
  
  dragOffset.value = {
    x: event.clientX - position.value.x,
    y: event.clientY - position.value.y
  }
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup', handleMouseUp)
}

// 拖动中
const onDrag = (event: MouseEvent) => {
  if (!isDragging.value) return;
  position.value = {
    x: event.clientX - dragOffset.value.x,
    y: event.clientY - dragOffset.value.y
  }
}

// 处理按钮鼠标释放事件
const handleMouseUp = (event: MouseEvent) => {
  isDragging.value = false
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup', handleMouseUp)
  
  const dragDistance = Math.sqrt(
    Math.pow(event.clientX - dragStartPosition.value.x, 2) + 
    Math.pow(event.clientY - dragStartPosition.value.y, 2)
  )

  if (dragDistance < 5) {
    toggleChat()
  }
  
  checkEdgeSnap()
}

// 检查是否贴边
const checkEdgeSnap = () => {
  if (!isBrowser) return
  
  const edgeThreshold = 20
  const buttonWidth = 50
  
  // 检查左边缘
  if (position.value.x < edgeThreshold) {
    position.value.x = -buttonWidth / 3
    isButtonHidden.value = true
    isRightEdge.value = false
  } 
  // 检查右边缘
  else if (position.value.x > windowWidth.value - edgeThreshold - buttonWidth) {
    position.value.x = windowWidth.value - buttonWidth + buttonWidth / 3
    isButtonHidden.value = true
    isRightEdge.value = true
  } else {
    isButtonHidden.value = false
    isRightEdge.value = false
  }
}

// 显示完整按钮
const showButton = () => {
  if (!isBrowser) return
  
  if (isButtonHidden.value) {
    if (position.value.x < 0) {
      position.value.x = 10
    } else if (position.value.x > windowWidth.value - 50) {
      position.value.x = windowWidth.value - 60
    }
    isButtonHidden.value = false
  }
}

// 切换聊天窗口
const toggleChat = async () => {
  showFloatingChat.value = !showFloatingChat.value
  if (showFloatingChat.value) {
    // 确保有对话ID
    if (!floatingConversationId.value) {
      await getOrCreateFloatingConversation()
    }
    
    // 加载聊天历史（如果还没有加载过或者消息为空）
    if (floatingConversationId.value && chatMessages.value.length === 0) {
      await loadChatHistory()
    }
    
    // 显示聊天窗口时滚动到底部
    scrollToBottom()
  }
}


// 取消AI生成
const cancelGeneration = async () => {
  if (isLoading.value && currentStreamingId.value) {
    isCancelling.value = true
    
    try {
      // 调用后端取消API
      await invoke('cancel_ai_stream', { streamId: currentStreamingId.value })
    } catch (error) {
      console.error('取消生成失败:', error)
    }
    
    let cancelMsg: {role: string, content: string, timestamp: number}
    
    // 如果有流式内容，保存它
    if (streamingContent.value.trim()) {
      cancelMsg = {
        role: 'assistant',
        content: streamingContent.value + '\n\n*[生成已被用户取消]*',
        timestamp: Date.now()
      }
    } else {
      // 添加取消消息到聊天记录
      cancelMsg = {
        role: 'assistant',
        content: '*生成已被用户取消*',
        timestamp: Date.now()
      }
    }
    
    chatMessages.value.push(cancelMsg)
    
    // 保存取消消息到数据库
    await saveMessageToDatabase('assistant', cancelMsg.content)
    
    // 重置状态
    isLoading.value = false
    isStreaming.value = false
    streamingContent.value = ''
    currentStreamingId.value = ''
    
    setTimeout(() => {
      isCancelling.value = false
      scrollToBottom()
    }, 300)
  }
}

// 清空所有聊天内容
const clearAllMessages = async () => {
  // 如果正在生成，先取消
  if (isLoading.value) {
    await cancelGeneration()
  }
  
  // 清空前端消息
  chatMessages.value = []
  streamingContent.value = ''
  
  // 清空数据库中的消息
  if (floatingConversationId.value) {
    try {
      await invoke('clear_ai_conversation', { conversationId: floatingConversationId.value })
      console.log('已清空浮动聊天框的数据库记录')
    } catch (error) {
      console.error('清空数据库消息失败:', error)
    }
  }
}

// 格式化时间
const formatTime = (timestamp: number) => {
  return new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

// Markdown 渲染缓存
const messageRenderCache = ref(new Map<string, string>())

// 渲染单个消息内容的计算属性工厂函数
const createMessageRenderer = (content: string) => {
  if (!content) return '<p class="text-base-content/50">暂无内容</p>'

  const cacheKey = content.substring(0, 1000) + content.length
  
  // 检查缓存
  if (messageRenderCache.value.has(cacheKey)) {
    return messageRenderCache.value.get(cacheKey)!
  }

  try {
    // 创建 marked 实例并配置高亮
    const markedInstance = new Marked()

    // 使用 marked-highlight 扩展
    markedInstance.use(markedHighlight({
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
    markedInstance.setOptions({
      breaks: true,
      gfm: true,
      pedantic: false,
      silent: true,
    })

    // 使用 marked 渲染 Markdown
    const htmlContent = markedInstance.parse(content) as string

    // 使用DOMPurify清理HTML，防止XSS
    const result = DOMPurify.sanitize(htmlContent, {
      ADD_TAGS: ['iframe', 'pre', 'code'],
      ADD_ATTR: ['allowfullscreen', 'frameborder', 'target', 'src', 'alt', 'class', 'style', 'data-highlighted', 'checked', 'disabled']
    })

    // 缓存结果，限制缓存大小
    if (messageRenderCache.value.size > 50) {
      // 清理最旧的缓存项
      const firstKey = messageRenderCache.value.keys().next().value
      if (firstKey) {
        messageRenderCache.value.delete(firstKey)
      }
    }
    messageRenderCache.value.set(cacheKey, result)

    return result
  } catch (err) {
    console.error('Markdown渲染错误:', err)
    const errorMessage = err instanceof Error ? err.message : String(err)
    return `<div class="text-error">Markdown渲染错误: ${errorMessage}</div>
            <pre>${DOMPurify.sanitize(content)}</pre>`
  }
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

// 渲染 Markdown 内容
const renderMarkdown = (content: string): string => {
  const result = createMessageRenderer(content)
  
  // 在下一个 tick 中处理代码块UI增强
  nextTick(() => {
    enhanceCodeBlocks()
  })
  
  return result
}

// 新增函数：应用代码块主题样式
function applyCodeBlockTheme() {
  // 创建样式元素
  let styleElement = document.getElementById('floating-prism-theme-styles') as HTMLStyleElement
  if (!styleElement) {
    styleElement = document.createElement('style')
    styleElement.id = 'floating-prism-theme-styles'
    document.head.appendChild(styleElement)
  }



  // 根据当前主题生成CSS样式
  const cssContent = `
    /* 浮动聊天框代码块主题样式修复 - Okaidia 默认主题 */
    .floating-chat-window pre {
      border: 1px solid rgba(255, 255, 255, 0.1) !important;
      border-radius: 0.5rem !important;
      overflow: auto !important;
      margin: 1rem 0 !important;
      padding: 0 !important;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1) !important;
    }

    .floating-chat-window pre code {
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

    .floating-chat-window .code-block-header {
      background: rgba(0, 0, 0, 0.2) !important;
      padding: 0.5rem 1rem !important;
      display: flex !important;
      justify-content: space-between !important;
      align-items: center !important;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
      font-size: 0.75rem !important;
    }

    .floating-chat-window .code-language {
      font-weight: 500 !important;
      text-transform: uppercase !important;
      opacity: 0.8 !important;
    }

    .floating-chat-window .copy-code-btn {
      opacity: 0.6 !important;
      transition: opacity 0.2s ease !important;
      background: transparent !important;
      border: none !important;
      cursor: pointer !important;
      padding: 0.25rem !important;
      border-radius: 0.25rem !important;
    }

    .floating-chat-window .copy-code-btn:hover {
      opacity: 1 !important;
    }

    /* 暗色主题适配 - 继续使用 okaidia 主题 */
    [data-theme="dark"] .floating-chat-window pre,
    [data-theme="night"] .floating-chat-window pre,
    [data-theme="black"] .floating-chat-window pre,
    [data-theme="dracula"] .floating-chat-window pre,
    [data-theme="halloween"] .floating-chat-window pre,
    [data-theme="business"] .floating-chat-window pre,
    [data-theme="luxury"] .floating-chat-window pre,
    [data-theme="coffee"] .floating-chat-window pre,
    [data-theme="forest"] .floating-chat-window pre,
    [data-theme="synthwave"] .floating-chat-window pre {
      border-color: rgba(255, 255, 255, 0.1) !important;
    }

    [data-theme="dark"] .floating-chat-window pre code,
    [data-theme="night"] .floating-chat-window pre code,
    [data-theme="black"] .floating-chat-window pre code,
    [data-theme="dracula"] .floating-chat-window pre code,
    [data-theme="halloween"] .floating-chat-window pre code,
    [data-theme="business"] .floating-chat-window pre code,
    [data-theme="luxury"] .floating-chat-window pre code,
    [data-theme="coffee"] .floating-chat-window pre code,
    [data-theme="forest"] .floating-chat-window pre code,
    [data-theme="synthwave"] .floating-chat-window pre code {
    }

    [data-theme="dark"] .floating-chat-window .code-block-header,
    [data-theme="night"] .floating-chat-window .code-block-header,
    [data-theme="black"] .floating-chat-window .code-block-header,
    [data-theme="dracula"] .floating-chat-window .code-block-header,
    [data-theme="halloween"] .floating-chat-window .code-block-header,
    [data-theme="business"] .floating-chat-window .code-block-header,
    [data-theme="luxury"] .floating-chat-window .code-block-header,
    [data-theme="coffee"] .floating-chat-window .code-block-header,
    [data-theme="forest"] .floating-chat-window .code-block-header,
    [data-theme="synthwave"] .floating-chat-window .code-block-header {
      background: rgba(0, 0, 0, 0.3) !important;
    }

    /* 亮色主题适配 - 使用 GitHub 风格 */
    [data-theme="light"] .floating-chat-window pre,
    [data-theme="cupcake"] .floating-chat-window pre,
    [data-theme="bumblebee"] .floating-chat-window pre,
    [data-theme="emerald"] .floating-chat-window pre,
    [data-theme="corporate"] .floating-chat-window pre,
    [data-theme="retro"] .floating-chat-window pre,
    [data-theme="cyberpunk"] .floating-chat-window pre,
    [data-theme="valentine"] .floating-chat-window pre,
    [data-theme="garden"] .floating-chat-window pre,
    [data-theme="aqua"] .floating-chat-window pre,
    [data-theme="lofi"] .floating-chat-window pre,
    [data-theme="pastel"] .floating-chat-window pre,
    [data-theme="fantasy"] .floating-chat-window pre,
    [data-theme="wireframe"] .floating-chat-window pre,
    [data-theme="cmyk"] .floating-chat-window pre,
    [data-theme="autumn"] .floating-chat-window pre,
    [data-theme="acid"] .floating-chat-window pre,
    [data-theme="lemonade"] .floating-chat-window pre,
    [data-theme="winter"] .floating-chat-window pre {
      border-color: rgba(0, 0, 0, 0.1) !important;
    }

    [data-theme="light"] .floating-chat-window pre code,
    [data-theme="cupcake"] .floating-chat-window pre code,
    [data-theme="bumblebee"] .floating-chat-window pre code,
    [data-theme="emerald"] .floating-chat-window pre code,
    [data-theme="corporate"] .floating-chat-window pre code,
    [data-theme="retro"] .floating-chat-window pre code,
    [data-theme="cyberpunk"] .floating-chat-window pre code,
    [data-theme="valentine"] .floating-chat-window pre code,
    [data-theme="garden"] .floating-chat-window pre code,
    [data-theme="aqua"] .floating-chat-window pre code,
    [data-theme="lofi"] .floating-chat-window pre code,
    [data-theme="pastel"] .floating-chat-window pre code,
    [data-theme="fantasy"] .floating-chat-window pre code,
    [data-theme="wireframe"] .floating-chat-window pre code,
    [data-theme="cmyk"] .floating-chat-window pre code,
    [data-theme="autumn"] .floating-chat-window pre code,
    [data-theme="acid"] .floating-chat-window pre code,
    [data-theme="lemonade"] .floating-chat-window pre code,
    [data-theme="winter"] .floating-chat-window pre code {
    }

    [data-theme="light"] .floating-chat-window .code-block-header,
    [data-theme="cupcake"] .floating-chat-window .code-block-header,
    [data-theme="bumblebee"] .floating-chat-window .code-block-header,
    [data-theme="emerald"] .floating-chat-window .code-block-header,
    [data-theme="corporate"] .floating-chat-window .code-block-header,
    [data-theme="retro"] .floating-chat-window .code-block-header,
    [data-theme="cyberpunk"] .floating-chat-window .code-block-header,
    [data-theme="valentine"] .floating-chat-window .code-block-header,
    [data-theme="garden"] .floating-chat-window .code-block-header,
    [data-theme="aqua"] .floating-chat-window .code-block-header,
    [data-theme="lofi"] .floating-chat-window .code-block-header,
    [data-theme="pastel"] .floating-chat-window .code-block-header,
    [data-theme="fantasy"] .floating-chat-window .code-block-header,
    [data-theme="wireframe"] .floating-chat-window .code-block-header,
    [data-theme="cmyk"] .floating-chat-window .code-block-header,
    [data-theme="autumn"] .floating-chat-window .code-block-header,
    [data-theme="acid"] .floating-chat-window .code-block-header,
    [data-theme="lemonade"] .floating-chat-window .code-block-header,
    [data-theme="winter"] .floating-chat-window .code-block-header {
      border-bottom: 1px solid rgba(0, 0, 0, 0.1) !important;
    }

    [data-theme="light"] .floating-chat-window .code-language,
    [data-theme="cupcake"] .floating-chat-window .code-language,
    [data-theme="bumblebee"] .floating-chat-window .code-language,
    [data-theme="emerald"] .floating-chat-window .code-language,
    [data-theme="corporate"] .floating-chat-window .code-language,
    [data-theme="retro"] .floating-chat-window .code-language,
    [data-theme="cyberpunk"] .floating-chat-window .code-language,
    [data-theme="valentine"] .floating-chat-window .code-language,
    [data-theme="garden"] .floating-chat-window .code-language,
    [data-theme="aqua"] .floating-chat-window .code-language,
    [data-theme="lofi"] .floating-chat-window .code-language,
    [data-theme="pastel"] .floating-chat-window .code-language,
    [data-theme="fantasy"] .floating-chat-window .code-language,
    [data-theme="wireframe"] .floating-chat-window .code-language,
    [data-theme="cmyk"] .floating-chat-window .code-language,
    [data-theme="autumn"] .floating-chat-window .code-language,
    [data-theme="acid"] .floating-chat-window .code-language,
    [data-theme="lemonade"] .floating-chat-window .code-language,
    [data-theme="winter"] .floating-chat-window .code-language {
      opacity: 0.7 !important;
    }

    [data-theme="light"] .floating-chat-window .copy-code-btn,
    [data-theme="cupcake"] .floating-chat-window .copy-code-btn,
    [data-theme="bumblebee"] .floating-chat-window .copy-code-btn,
    [data-theme="emerald"] .floating-chat-window .copy-code-btn,
    [data-theme="corporate"] .floating-chat-window .copy-code-btn,
    [data-theme="retro"] .floating-chat-window .copy-code-btn,
    [data-theme="cyberpunk"] .floating-chat-window .copy-code-btn,
    [data-theme="valentine"] .floating-chat-window .copy-code-btn,
    [data-theme="garden"] .floating-chat-window .copy-code-btn,
    [data-theme="aqua"] .floating-chat-window .copy-code-btn,
    [data-theme="lofi"] .floating-chat-window .copy-code-btn,
    [data-theme="pastel"] .floating-chat-window .copy-code-btn,
    [data-theme="fantasy"] .floating-chat-window .copy-code-btn,
    [data-theme="wireframe"] .floating-chat-window .copy-code-btn,
    [data-theme="cmyk"] .floating-chat-window .copy-code-btn,
    [data-theme="autumn"] .floating-chat-window .copy-code-btn,
    [data-theme="acid"] .floating-chat-window .copy-code-btn,
    [data-theme="lemonade"] .floating-chat-window .copy-code-btn,
    [data-theme="winter"] .floating-chat-window .copy-code-btn {
      opacity: 0.6 !important;
    }

    [data-theme="light"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="cupcake"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="bumblebee"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="emerald"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="corporate"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="retro"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="cyberpunk"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="valentine"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="garden"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="aqua"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="lofi"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="pastel"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="fantasy"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="wireframe"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="cmyk"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="autumn"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="acid"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="lemonade"] .floating-chat-window .copy-code-btn:hover,
    [data-theme="winter"] .floating-chat-window .copy-code-btn:hover {
      background: rgba(0, 0, 0, 0.1) !important;
      opacity: 1 !important;
    }

    /* 行内代码在亮色主题下的样式 */
    [data-theme="light"] .floating-chat-window code:not(pre code),
    [data-theme="cupcake"] .floating-chat-window code:not(pre code),
    [data-theme="bumblebee"] .floating-chat-window code:not(pre code),
    [data-theme="emerald"] .floating-chat-window code:not(pre code),
    [data-theme="corporate"] .floating-chat-window code:not(pre code),
    [data-theme="retro"] .floating-chat-window code:not(pre code),
    [data-theme="cyberpunk"] .floating-chat-window code:not(pre code),
    [data-theme="valentine"] .floating-chat-window code:not(pre code),
    [data-theme="garden"] .floating-chat-window code:not(pre code),
    [data-theme="aqua"] .floating-chat-window code:not(pre code),
    [data-theme="lofi"] .floating-chat-window code:not(pre code),
    [data-theme="pastel"] .floating-chat-window code:not(pre code),
    [data-theme="fantasy"] .floating-chat-window code:not(pre code),
    [data-theme="wireframe"] .floating-chat-window code:not(pre code),
    [data-theme="cmyk"] .floating-chat-window code:not(pre code),
    [data-theme="autumn"] .floating-chat-window code:not(pre code),
    [data-theme="acid"] .floating-chat-window code:not(pre code),
    [data-theme="lemonade"] .floating-chat-window code:not(pre code),
    [data-theme="winter"] .floating-chat-window code:not(pre code) {
      background-color: rgba(175, 184, 193, 0.2) !important;
      color: rgb(124, 58, 237) !important;
      border: 1px solid rgba(0, 0, 0, 0.15) !important;
    }
  `

  styleElement.textContent = cssContent
}

// 增强代码块UI的函数
function enhanceCodeBlocks() {
  // 查找浮动聊天框内的代码块
  const codeElements = document.querySelectorAll('.floating-chat-window code[class*="language-"], .floating-chat-window pre > code:not([class*="language-"])')

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
      
      // 确保应用 okaidia 主题样式
      pre.classList.add('prism-okaidia')
      if (langClass) {
        pre.classList.add(langClass)
      }
    }
  })

  // 应用代码块主题样式
  applyCodeBlockTheme()
}

// 设置代码复制功能
const setupCodeCopyFeature = () => {
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

// 开始调整大小
const startResize = (event: MouseEvent, handle: string) => {
  event.preventDefault()
  event.stopPropagation()
  
  isResizing.value = true
  resizeHandle.value = handle
  resizeStartPos.value = { x: event.clientX, y: event.clientY }
  resizeStartSize.value = { ...chatWindowSize.value }
  
  // 记录窗口的初始边界
  resizeStartRect.value = {
    left: position.value.x,
    top: position.value.y,
    right: position.value.x + chatWindowSize.value.width,
    bottom: position.value.y + chatWindowSize.value.height
  }
  
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

// 调整大小中
const onResize = (event: MouseEvent) => {
  if (!isResizing.value) return
  
  const deltaX = event.clientX - resizeStartPos.value.x
  const deltaY = event.clientY - resizeStartPos.value.y
  
  let left = resizeStartRect.value.left
  let top = resizeStartRect.value.top
  let right = resizeStartRect.value.right
  let bottom = resizeStartRect.value.bottom
  
  // 根据拖拽方向调整边界
  if (resizeHandle.value.includes('left')) {
    left = resizeStartRect.value.left + deltaX
  }
  if (resizeHandle.value.includes('right')) {
    right = resizeStartRect.value.right + deltaX
  }
  if (resizeHandle.value.includes('top')) {
    top = resizeStartRect.value.top + deltaY
  }
  if (resizeHandle.value.includes('bottom')) {
    bottom = resizeStartRect.value.bottom + deltaY
  }
  
  // 计算新的宽高
  let newWidth = right - left
  let newHeight = bottom - top
  
  // 应用尺寸限制
  newWidth = Math.max(250, Math.min(1100, newWidth))
  newHeight = Math.max(300, Math.min(800, newHeight))
  
  // 如果尺寸被限制，需要调整对应的边界
  if (resizeHandle.value.includes('left') && newWidth !== right - left) {
    left = right - newWidth
  }
  if (resizeHandle.value.includes('top') && newHeight !== bottom - top) {
    top = bottom - newHeight
  }
  
  // 更新尺寸和位置
  chatWindowSize.value = { width: newWidth, height: newHeight }
  position.value = { x: left, y: top }
}

// 结束调整大小
const stopResize = () => {
  isResizing.value = false
  resizeHandle.value = ''
  
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}

// 监听窗口大小变化
const handleResize = () => {
  if (!isBrowser) return
  
  windowWidth.value = window.innerWidth
  windowHeight.value = window.innerHeight
  
  // 确保按钮在窗口内
  if (position.value.x > windowWidth.value - 50) {
    position.value.x = windowWidth.value - 60
  }
  if (position.value.y > windowHeight.value - 50) {
    position.value.y = windowHeight.value - 60
  }
}



// 主题观察器引用
let themeObserver: MutationObserver | null = null
let unlistenSettingsChanged: (() => void) | null = null;

onMounted(async () => {
  if (isBrowser) {
    window.addEventListener('resize', handleResize)
    await loadDefaultModel()
    setupCodeCopyFeature()
    
    // 应用代码块主题样式
    applyCodeBlockTheme()
    
    // 监听主题变化
    themeObserver = new MutationObserver(() => {
      applyCodeBlockTheme()
    })
    
    themeObserver.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme']
    })
    
    // 初始化浮动对话（如果需要的话）
    await getOrCreateFloatingConversation()
    
    // 监听流式输出事件
    await listen('ai-stream-chunk', handleStreamChunk)

    // 监听全局设置变化
    unlistenSettingsChanged = await listen('global-settings-changed', (event) => {
      const payload = event.payload as { key: string };
      if (payload.key === 'defaultAIModel') {
        console.log('FloatingAI: 检测到全局默认AI模型变更，正在重新加载...');
        loadDefaultModel();
      }
    });

    // 新增：监听 Alt+S 全局快捷键
    window.addEventListener('keydown', handleGlobalShortcut)
  }
})

onUnmounted(() => {
  if (isBrowser) {
    window.removeEventListener('resize', handleResize)
    document.removeEventListener('mousemove', onDrag)
    document.removeEventListener('mouseup', handleMouseUp)
    document.removeEventListener('mousemove', onResize)
    document.removeEventListener('mouseup', stopResize)
    // 新增：移除 Alt+S 全局快捷键监听
    window.removeEventListener('keydown', handleGlobalShortcut)
  }
  
  // 清理主题观察器
  if (themeObserver) {
    themeObserver.disconnect()
    themeObserver = null
  }
  
  // 清理设置变化监听器
  if (unlistenSettingsChanged) {
    unlistenSettingsChanged();
  }

  // 清理样式元素
  const styleElement = document.getElementById('floating-prism-theme-styles')
  if (styleElement) {
    styleElement.remove()
  }
  
  // 清理正在进行的流式输出
  if (currentStreamingId.value) {
    cancelGeneration()
  }
})

// 全局快捷键 Alt+S 打开浮动聊天框
const handleGlobalShortcut = (event: KeyboardEvent) => {
  // 仅在按下 Shift+S 时触发
  if (event.shiftKey && event.key.toLowerCase() === 's') {
    event.preventDefault()
    // 切换聊天窗口（打开或关闭）
    toggleChat()
  }
}
</script>

<template>
  <div>
    <!-- 浮动AI按钮 -->
    <div 
      v-if="!isMobile || !showFloatingChat"
      class="floating-ai-button"
      :class="{ 
        'semi-hidden': isButtonHidden,
        'right-edge': isRightEdge
      }"
      :style="{ 
        left: `${position.x}px`, 
        top: `${position.y}px` 
      }"
      @mousedown="handleMouseDown"
      @mouseenter="showButton"
    >
      <div class="btn-content">
        <img :src="getModelAvatar(selectedModel)" :alt="getModelName(selectedModel)" class="model-avatar" />
        <span class="btn-text">AI</span>
      </div>
    </div>

    <!-- 浮动聊天窗口 -->
    <div 
      v-if="showFloatingChat" 
      class="floating-chat-window"
      :style="chatWindowPosition"
    >
      <!-- 聊天窗口头部 -->
      <div class="chat-header">
        <div class="flex items-center">
          <img :src="getModelAvatar(selectedModel)" :alt="getModelName(selectedModel)" class="h-6 w-6 mr-2" />
          <h3>{{ getModelName(selectedModel) }}</h3>
        </div>
        <div class="flex items-center gap-2">
          <button 
            class="clear-btn" 
            @click="clearAllMessages"
            :disabled="chatMessages.length === 0 && !streamingContent"
            title="清空所有内容"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
          <button class="close-btn" @click="showFloatingChat = false">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 聊天内容区域 -->
      <div ref="messagesContainer" class="chat-messages">
        <div v-if="isLoadingHistory" class="empty-chat">
          <span class="loading loading-spinner loading-lg mr-2"></span>
          <p>正在加载聊天历史...</p>
        </div>
        <div v-else-if="chatMessages.length === 0" class="empty-chat">
          <p>开始与AI助手对话</p>
        </div>
        
        <template v-else>
          <div 
            v-for="(message, index) in chatMessages" 
            :key="index" 
            :class="['message', message.role === 'user' ? 'user-message' : 'ai-message']"
          >
            <div class="message-header">
              <span>{{ message.role === 'user' ? '您' : getModelName(selectedModel) }}</span>
              <time>{{ formatTime(message.timestamp) }}</time>
            </div>
            <div class="message-content">
              <!-- 用户消息使用普通文本 -->
              <template v-if="message.role === 'user'">
                {{ message.content }}
              </template>
              <!-- AI消息使用Markdown渲染 -->
              <div v-else v-html="renderMarkdown(message.content)" class="markdown-content"></div>
            </div>
          </div>
          
          <!-- 流式输出显示 -->
          <div v-if="isStreaming" class="ai-message streaming-message">
            <div class="message-header">
              <span>{{ getModelName(selectedModel) }}</span>
              <time>{{ formatTime(Date.now()) }}</time>
            </div>
            <div class="message-content">
              <div v-if="streamingContent" v-html="renderMarkdown(streamingContent)" class="markdown-content"></div>
              <div v-else class="flex items-center gap-2">
                <span class="loading loading-dots loading-sm"></span>
                <!-- <span>{{ isCancelling ? '正在取消...' : '正在生成回复...' }}</span> -->
              </div>
              <div v-if="streamingContent" class="streaming-cursor">▌</div>
            </div>
          </div>
        </template>

        <!-- 正在输入指示器 -->
        <div v-if="isLoading && !isStreaming" class="ai-message">
          <div class="message-header">
            <span>{{ getModelName(selectedModel) }}</span>
            <time>{{ formatTime(Date.now()) }}</time>
          </div>
          <div class="message-content ai-thinking">
            <div class="flex items-center gap-2">
              <span class="loading loading-dots loading-sm"></span>
              <!-- <span>{{ isCancelling ? '正在取消...' : '正在思考中...' }}</span> -->
            </div>
          </div>
        </div>
      </div>

      <!-- 输入区域 -->
      <div class="chat-input">
        <input 
          type="text" 
          v-model="userMessage" 
          placeholder="输入消息..." 
          @keyup.enter="isLoading ? cancelGeneration() : sendMessage()"
          :disabled="isCancelling"
        />
        
        <!-- 发送/取消按钮容器 -->
        <div class="action-btn-container">
          <Transition name="btn-switch" mode="out-in">
            <!-- 发送按钮 -->
            <button 
              v-if="!isLoading" 
              key="send"
              class="action-btn send-btn" 
              @click="sendMessage" 
              :disabled="!userMessage.trim()"
              title="发送消息"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
              </svg>
            </button>
            
            <!-- 取消按钮 -->
            <button 
              v-else
              key="cancel"
              class="action-btn cancel-btn" 
              @click="cancelGeneration" 
              :disabled="isCancelling"
              title="取消生成"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </Transition>
        </div>
      </div>

      <!-- 调整大小手柄 -->
      <template v-if="!isMobile">
        <!-- 四边手柄 -->
        <div class="resize-handle resize-handle-top" @mousedown="startResize($event, 'top')"></div>
        <div class="resize-handle resize-handle-right" @mousedown="startResize($event, 'right')"></div>
        <div class="resize-handle resize-handle-bottom" @mousedown="startResize($event, 'bottom')"></div>
        <div class="resize-handle resize-handle-left" @mousedown="startResize($event, 'left')"></div>
        
        <!-- 四角手柄 -->
        <div class="resize-handle resize-handle-top-left" @mousedown="startResize($event, 'top-left')"></div>
        <div class="resize-handle resize-handle-top-right" @mousedown="startResize($event, 'top-right')"></div>
        <div class="resize-handle resize-handle-bottom-left" @mousedown="startResize($event, 'bottom-left')"></div>
        <div class="resize-handle resize-handle-bottom-right" @mousedown="startResize($event, 'bottom-right')"></div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.floating-ai-button {
  position: fixed;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background-color: #3b82f6 !important; /* 确保有回退颜色 */
  background: hsl(var(--p)) !important; /* 使用主题色 */
  color: white !important; /* 确保有回退颜色 */
  color: hsl(var(--pc)) !important; /* 使用主题文字色 */
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: move;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
  transition: transform 0.3s, box-shadow 0.3s, background-color 0.3s;
  z-index: 9999;
  user-select: none;
  opacity: 1 !important;
  backdrop-filter: none !important;
  -webkit-backdrop-filter: none !important;
  /* 确保纯色背景，不使用任何透明度 */
  background-clip: padding-box;
  mix-blend-mode: normal;
  isolation: isolate;
}

.floating-ai-button:hover {
  box-shadow: 0 6px 14px rgba(0, 0, 0, 0.25);
  transform: translateY(-2px);
  background: hsl(var(--pf)) !important;
  background-color: hsl(var(--pf)) !important;
  /* 确保悬停时也是纯色背景 */
  backdrop-filter: none !important;
  -webkit-backdrop-filter: none !important;
}

/* 确保在不同主题下都有纯色不透明背景 */
[data-theme="dark"] .floating-ai-button,
[data-theme="night"] .floating-ai-button,
[data-theme="coffee"] .floating-ai-button,
[data-theme="black"] .floating-ai-button,
[data-theme="dracula"] .floating-ai-button,
[data-theme="halloween"] .floating-ai-button,
[data-theme="business"] .floating-ai-button,
[data-theme="luxury"] .floating-ai-button,
[data-theme="forest"] .floating-ai-button,
[data-theme="synthwave"] .floating-ai-button {
  background: hsl(var(--p)) !important;
  background-color: hsl(var(--p)) !important;
  color: hsl(var(--pc)) !important;
  opacity: 1 !important;
  backdrop-filter: none !important;
  -webkit-backdrop-filter: none !important;
  /* 强制纯色，防止任何透明度 */
  mix-blend-mode: normal !important;
}

[data-theme="light"] .floating-ai-button,
[data-theme="cupcake"] .floating-ai-button,
[data-theme="bumblebee"] .floating-ai-button,
[data-theme="emerald"] .floating-ai-button,
[data-theme="corporate"] .floating-ai-button,
[data-theme="retro"] .floating-ai-button,
[data-theme="cyberpunk"] .floating-ai-button,
[data-theme="valentine"] .floating-ai-button,
[data-theme="garden"] .floating-ai-button,
[data-theme="aqua"] .floating-ai-button,
[data-theme="lofi"] .floating-ai-button,
[data-theme="pastel"] .floating-ai-button,
[data-theme="fantasy"] .floating-ai-button,
[data-theme="wireframe"] .floating-ai-button,
[data-theme="cmyk"] .floating-ai-button,
[data-theme="autumn"] .floating-ai-button,
[data-theme="acid"] .floating-ai-button,
[data-theme="lemonade"] .floating-ai-button,
[data-theme="winter"] .floating-ai-button {
  background: hsl(var(--p)) !important;
  background-color: hsl(var(--p)) !important;
  color: hsl(var(--pc)) !important;
  opacity: 1 !important;
  backdrop-filter: none !important;
  -webkit-backdrop-filter: none !important;
  /* 强制纯色，防止任何透明度 */
  mix-blend-mode: normal !important;
}

.floating-ai-button.semi-hidden {
  transition: transform 0.3s;
}

.floating-ai-button.semi-hidden:not(:hover) {
  transform: translateX(-15px);
}

.floating-ai-button.semi-hidden.right-edge:not(:hover) {
  transform: translateX(15px);
}

.btn-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 1;
  position: relative;
}

.model-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  object-fit: cover;
  filter: none;
  opacity: 1;
}

.btn-text {
  font-size: 10px;
  font-weight: bold;
  margin-top: 1px;
  color: inherit;
  opacity: 1;
}

.floating-chat-window {
  position: fixed;
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  z-index: 9998;
  overflow: hidden;
  border: 1px solid #e5e7eb;
  backdrop-filter: none;
  opacity: 1;
  min-width: 250px;
  min-height: 300px;
  max-width: 600px;
  max-height: 700px;
}

/* 深色主题下的背景 */
[data-theme="dark"] .floating-chat-window,
[data-theme="night"] .floating-chat-window,
[data-theme="coffee"] .floating-chat-window,
[data-theme="black"] .floating-chat-window {
  background: #1f2937;
  border-color: #374151;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  background: hsl(var(--p));
  color: hsl(var(--pc));
}

.chat-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: bold;
}

.close-btn, .clear-btn {
  background: none;
  border: none;
  color: hsl(var(--pc));
  cursor: pointer;
  padding: 5px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.close-btn:hover, .clear-btn:hover:not(:disabled) {
  background: hsl(var(--pc) / 0.1);
  border-radius: 4px;
}

.clear-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 15px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  background: #ffffff;
}

/* 深色主题下的消息区域背景 */
[data-theme="dark"] .chat-messages,
[data-theme="night"] .chat-messages,
[data-theme="coffee"] .chat-messages,
[data-theme="black"] .chat-messages {
  background: #1f2937;
}

.empty-chat {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #6b7280;
}

/* 深色主题下的空聊天提示 */
[data-theme="dark"] .empty-chat,
[data-theme="night"] .empty-chat,
[data-theme="coffee"] .empty-chat,
[data-theme="black"] .empty-chat {
  color: #9ca3af;
}

.message {
  max-width: 85%;
  padding: 8px 12px;
  border-radius: 12px;
  margin-bottom: 8px;
}

.user-message {
  align-self: flex-end;
  background: hsl(var(--p) / 0.2);
  border-bottom-right-radius: 4px;
  color: #111827;
}

/* 深色主题下的用户消息 */
[data-theme="dark"] .user-message,
[data-theme="night"] .user-message,
[data-theme="coffee"] .user-message,
[data-theme="black"] .user-message {
  color: #f9fafb;
}

.ai-message {
  align-self: flex-start;
  background: #f3f4f6;
  border-bottom-left-radius: 4px;
  color: #111827;
}

/* 深色主题下的AI消息 */
[data-theme="dark"] .ai-message,
[data-theme="night"] .ai-message,
[data-theme="coffee"] .ai-message,
[data-theme="black"] .ai-message {
  background: #374151;
  color: #f9fafb;
}

.message-header {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  margin-bottom: 4px;
  opacity: 0.7;
}

.message-content {
  word-break: break-word;
}

.ai-thinking {
  display: flex;
  align-items: center;
  gap: 8px;
}

.chat-input {
  display: flex;
  padding: 10px;
  border-top: 1px solid #e5e7eb;
  background: #f9fafb;
}

/* 深色主题下的输入区域背景 */
[data-theme="dark"] .chat-input,
[data-theme="night"] .chat-input,
[data-theme="coffee"] .chat-input,
[data-theme="black"] .chat-input {
  background: #111827;
  border-top-color: #374151;
}

.chat-input input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 20px;
  outline: none;
  background: #ffffff;
  color: #111827;
}

/* 深色主题下的输入框 */
[data-theme="dark"] .chat-input input,
[data-theme="night"] .chat-input input,
[data-theme="coffee"] .chat-input input,
[data-theme="black"] .chat-input input {
  background: #1f2937;
  border-color: #4b5563;
  color: #f9fafb;
}

.chat-input input:focus {
  border-color: hsl(var(--p));
  outline: none;
}

.action-btn-container {
  position: relative;
  width: 36px;
  height: 36px;
  margin-left: 8px;
  flex-shrink: 0;
}

.action-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;
  position: absolute;
  top: 0;
  left: 0;
}

.send-btn {
  background-color: hsl(var(--p));
  color: hsl(var(--pc));
}

.send-btn:hover:not(:disabled) {
  background-color: hsl(var(--pf));
  transform: scale(1.05);
}

.send-btn:active:not(:disabled) {
  transform: scale(0.95);
}

.send-btn:disabled {
  background-color: #d1d5db;
  cursor: not-allowed;
  color: #6b7280;
}

.cancel-btn {
  background-color: hsl(var(--er));
  color: hsl(var(--erc));
}

.cancel-btn:hover:not(:disabled) {
  background-color: hsl(var(--er) / 0.8);
  transform: scale(1.05);
}

.cancel-btn:active:not(:disabled) {
  transform: scale(0.95);
}

.cancel-btn:disabled {
  background-color: #d1d5db;
  cursor: not-allowed;
  color: #6b7280;
  opacity: 0.6;
}

/* 深色主题下的禁用按钮 */
[data-theme="dark"] .action-btn:disabled,
[data-theme="night"] .action-btn:disabled,
[data-theme="coffee"] .action-btn:disabled,
[data-theme="black"] .action-btn:disabled {
  background-color: #4b5563;
  color: #9ca3af;
}

/* 按钮切换动画 */
.btn-switch-enter-active,
.btn-switch-leave-active {
  transition: all 0.3s ease;
}

.btn-switch-enter-from {
  opacity: 0;
  transform: scale(0.8) rotate(90deg);
}

.btn-switch-leave-to {
  opacity: 0;
  transform: scale(0.8) rotate(-90deg);
}

.btn-switch-enter-to,
.btn-switch-leave-from {
  opacity: 1;
  transform: scale(1) rotate(0deg);
}

.dropdown-content {
  max-height: 300px;
  overflow-y: auto;
}

.resize-handle {
  position: absolute;
  background-color: hsl(var(--p) / 0.3);
  border: 1px solid hsl(var(--p));
  transition: all 0.2s;
}

.resize-handle:hover {
  background-color: hsl(var(--p));
  transform: scale(1.1);
}

.resize-handle-top {
  top: -5px;
  left: 50%;
  width: 30px;
  height: 10px;
  cursor: ns-resize;
  border-radius: 5px 5px 0 0;
  transform: translateX(-50%);
}

.resize-handle-right {
  right: -5px;
  top: 50%;
  width: 10px;
  height: 30px;
  cursor: ew-resize;
  border-radius: 0 5px 5px 0;
  transform: translateY(-50%);
}

.resize-handle-bottom {
  bottom: -5px;
  left: 50%;
  width: 30px;
  height: 10px;
  cursor: ns-resize;
  border-radius: 5px 5px 0 0;
  transform: translateX(-50%);
}

.resize-handle-left {
  left: -5px;
  top: 50%;
  width: 10px;
  height: 30px;
  cursor: ew-resize;
  border-radius: 5px 0 0 5px;
  transform: translateY(-50%);
}

.resize-handle-top-left {
  left: -5px;
  top: -5px;
  width: 15px;
  height: 15px;
  cursor: nw-resize;
  border-radius: 50%;
}

.resize-handle-top-right {
  right: -5px;
  top: -5px;
  width: 15px;
  height: 15px;
  cursor: ne-resize;
  border-radius: 50%;
}

.resize-handle-bottom-left {
  left: -5px;
  bottom: -5px;
  width: 15px;
  height: 15px;
  cursor: sw-resize;
  border-radius: 50%;
}

.resize-handle-bottom-right {
  right: -5px;
  bottom: -5px;
  width: 15px;
  height: 15px;
  cursor: se-resize;
  border-radius: 50%;
}

/* Markdown 内容样式 */
.markdown-content {
  word-break: break-word;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
  margin: 1rem 0 0.5rem 0;
  font-weight: bold;
}

.markdown-content h1 { font-size: 1.5rem; }
.markdown-content h2 { font-size: 1.3rem; }
.markdown-content h3 { font-size: 1.1rem; }
.markdown-content h4 { font-size: 1rem; }

.markdown-content p {
  margin: 0.5rem 0;
  line-height: 1.6;
}

.markdown-content ul,
.markdown-content ol {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
}

.markdown-content li {
  margin: 0.25rem 0;
}

.markdown-content blockquote {
  border-left: 4px solid hsl(var(--p) / 0.3);
  padding-left: 1rem;
  margin: 1rem 0;
  background: hsl(var(--b2));
  border-radius: 0 0.5rem 0.5rem 0;
}

.markdown-content code {
  background: hsl(var(--b2));
  padding: 0.2rem 0.4rem;
  border-radius: 0.25rem;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', 'Source Code Pro', 'Menlo', 'Consolas', monospace;
  font-size: 0.9em;
  font-weight: normal;
  text-shadow: none;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.markdown-content pre {
  background: hsl(var(--b3));
  padding: 1rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin: 1rem 0;
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', 'Source Code Pro', 'Menlo', 'Consolas', monospace;
  font-weight: normal;
  text-shadow: none;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.markdown-content pre code {
  background: none;
  padding: 0;
  font-family: inherit;
  font-weight: inherit;
  text-shadow: none;
}

.markdown-content a {
  color: hsl(var(--p));
  text-decoration: underline;
}

.markdown-content a:hover {
  color: hsl(var(--pf));
}

.markdown-content table {
  border-collapse: collapse;
  width: 100%;
  margin: 1rem 0;
}

.markdown-content th,
.markdown-content td {
  border: 1px solid hsl(var(--b3));
  padding: 0.5rem;
  text-align: left;
}

.markdown-content th {
  background: hsl(var(--b2));
  font-weight: bold;
}

/* 代码块容器样式 */
.floating-chat-window .code-block-container {
  position: relative;
  margin: 1rem 0;
  border-radius: 0.375rem;
  overflow: hidden;
  background: var(--prism-background, #2d3748);
  border: 1px solid var(--prism-border, #4a5568);
}

.floating-chat-window .code-block-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 1rem;
  background: var(--prism-header-background, #1a202c);
  font-size: 0.875rem;
  border-bottom: 1px solid var(--prism-border, #4a5568);
}

.floating-chat-window .code-language {
  font-weight: 600;
  color: var(--prism-header-text, #a0aec0);
  text-transform: uppercase;
  font-size: 0.75rem;
}

.floating-chat-window .copy-code-btn {
  padding: 0.25rem;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--prism-header-text, #a0aec0);
}

.floating-chat-window .copy-code-btn:hover {
  background: var(--prism-button-hover, rgba(255, 255, 255, 0.1));
}

.floating-chat-window .copy-code-btn svg {
  width: 1rem;
  height: 1rem;
}

.floating-chat-window .copy-code-btn:disabled {
  cursor: not-allowed;
}

/* 代码块容器内的pre元素样式调整 */
.floating-chat-window .code-block-container pre {
  margin: 0;
  background: transparent;
  border: none;
  border-radius: 0;
}

.floating-chat-window .code-block-container pre code {
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
  font-weight: normal;
  text-shadow: none;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 默认使用 okaidia 主题样式 */
.floating-chat-window .code-block-container {
  --prism-background: #272822;
  --prism-border: #3c3c3c;
  --prism-header-background: rgba(0, 0, 0, 0.2);
  --prism-header-text: #f8f8f2;
  --prism-button-hover: #49483e;
}

/* 亮色主题下的代码块样式 - 使用 GitHub 风格 */
[data-theme="light"] .floating-chat-window .code-block-container,
[data-theme="cupcake"] .floating-chat-window .code-block-container,
[data-theme="bumblebee"] .floating-chat-window .code-block-container,
[data-theme="emerald"] .floating-chat-window .code-block-container,
[data-theme="corporate"] .floating-chat-window .code-block-container,
[data-theme="retro"] .floating-chat-window .code-block-container,
[data-theme="cyberpunk"] .floating-chat-window .code-block-container,
[data-theme="valentine"] .floating-chat-window .code-block-container,
[data-theme="garden"] .floating-chat-window .code-block-container,
[data-theme="aqua"] .floating-chat-window .code-block-container,
[data-theme="lofi"] .floating-chat-window .code-block-container,
[data-theme="pastel"] .floating-chat-window .code-block-container,
[data-theme="fantasy"] .floating-chat-window .code-block-container,
[data-theme="wireframe"] .floating-chat-window .code-block-container,
[data-theme="cmyk"] .floating-chat-window .code-block-container,
[data-theme="autumn"] .floating-chat-window .code-block-container,
[data-theme="acid"] .floating-chat-window .code-block-container,
[data-theme="lemonade"] .floating-chat-window .code-block-container,
[data-theme="winter"] .floating-chat-window .code-block-container {
  --prism-background: #f6f8fa;
  --prism-border: #e2e8f0;
  --prism-header-background: #e1e4e8;
  --prism-header-text: #24292e;
  --prism-button-hover: rgba(0, 0, 0, 0.1);
}

/* 暗色主题下保持 okaidia 风格 */
[data-theme="dark"] .floating-chat-window .code-block-container,
[data-theme="night"] .floating-chat-window .code-block-container,
[data-theme="black"] .floating-chat-window .code-block-container,
[data-theme="coffee"] .floating-chat-window .code-block-container,
[data-theme="dracula"] .floating-chat-window .code-block-container,
[data-theme="halloween"] .floating-chat-window .code-block-container,
[data-theme="business"] .floating-chat-window .code-block-container,
[data-theme="luxury"] .floating-chat-window .code-block-container,
[data-theme="forest"] .floating-chat-window .code-block-container,
[data-theme="synthwave"] .floating-chat-window .code-block-container {
  --prism-background: #272822;
  --prism-border: #3c3c3c;
  --prism-header-background: rgba(0, 0, 0, 0.3);
  --prism-header-text: #f8f8f2;
  --prism-button-hover: #49483e;
}

/* 深色主题下的 Markdown 样式 */
[data-theme="dark"] .markdown-content blockquote,
[data-theme="night"] .markdown-content blockquote,
[data-theme="coffee"] .markdown-content blockquote,
[data-theme="black"] .markdown-content blockquote {
  background: #374151;
}

/* 流式输出样式 */
.streaming-message {
  border-left: 3px solid hsl(var(--p));
  background: linear-gradient(90deg, hsl(var(--p) / 0.05), transparent);
  animation: pulse 2s ease-in-out infinite;
}

.streaming-cursor {
  display: inline-block;
  animation: blink 1s infinite;
  color: hsl(var(--p));
  font-weight: bold;
  margin-left: 2px;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

@keyframes pulse {
  0%, 100% { 
    border-left-color: hsl(var(--p));
    background: linear-gradient(90deg, hsl(var(--p) / 0.05), transparent);
  }
  50% { 
    border-left-color: hsl(var(--p) / 0.7);
    background: linear-gradient(90deg, hsl(var(--p) / 0.1), transparent);
  }
}

/* 确保 PrismJS okaidia 主题正确应用 */
.floating-chat-window pre[class*="language-"],
.floating-chat-window code[class*="language-"] {
  background: #272822 !important;
  color: #f8f8f2 !important;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace !important;
  font-weight: normal !important;
  text-shadow: none !important;
  -webkit-font-smoothing: antialiased !important;
  -moz-osx-font-smoothing: grayscale !important;
  font-variant-ligatures: none !important;
}

/* PrismJS token 样式修复 */
.floating-chat-window .token {
  font-family: inherit !important;
  font-weight: normal !important;
  text-shadow: none !important;
  -webkit-font-smoothing: antialiased !important;
  -moz-osx-font-smoothing: grayscale !important;
}

/* 确保 okaidia 主题的代码块内容背景 */
.floating-chat-window .code-block-container pre[class*="language-"] {
  background: #272822 !important;
  margin: 0 !important;
  padding: 1rem !important;
}

/* 亮色主题下的代码块覆盖 - GitHub 风格 */
[data-theme="light"] .floating-chat-window pre[class*="language-"],
[data-theme="cupcake"] .floating-chat-window pre[class*="language-"],
[data-theme="bumblebee"] .floating-chat-window pre[class*="language-"],
[data-theme="emerald"] .floating-chat-window pre[class*="language-"],
[data-theme="corporate"] .floating-chat-window pre[class*="language-"],
[data-theme="retro"] .floating-chat-window pre[class*="language-"],
[data-theme="cyberpunk"] .floating-chat-window pre[class*="language-"],
[data-theme="valentine"] .floating-chat-window pre[class*="language-"],
[data-theme="garden"] .floating-chat-window pre[class*="language-"],
[data-theme="aqua"] .floating-chat-window pre[class*="language-"],
[data-theme="lofi"] .floating-chat-window pre[class*="language-"],
[data-theme="pastel"] .floating-chat-window pre[class*="language-"],
[data-theme="fantasy"] .floating-chat-window pre[class*="language-"],
[data-theme="wireframe"] .floating-chat-window pre[class*="language-"],
[data-theme="cmyk"] .floating-chat-window pre[class*="language-"],
[data-theme="autumn"] .floating-chat-window pre[class*="language-"],
[data-theme="acid"] .floating-chat-window pre[class*="language-"],
[data-theme="lemonade"] .floating-chat-window pre[class*="language-"],
[data-theme="winter"] .floating-chat-window pre[class*="language-"] {
  background: #f6f8fa !important;
  color: #24292e !important;
}



/* 行内代码样式修复 */
.floating-chat-window .markdown-content code:not([class*="language-"]) {
  background: hsl(var(--b2));
  padding: 0.2rem 0.4rem;
  border-radius: 0.25rem;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 0.9em;
  font-weight: normal !important;
  text-shadow: none !important;
  text-decoration: none !important;
  font-style: normal !important;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 深色主题下的行内代码 */
[data-theme="dark"] .floating-chat-window .markdown-content code:not([class*="language-"]),
[data-theme="night"] .floating-chat-window .markdown-content code:not([class*="language-"]),
[data-theme="coffee"] .floating-chat-window .markdown-content code:not([class*="language-"]),
[data-theme="black"] .floating-chat-window .markdown-content code:not([class*="language-"]) {
  background: rgba(255, 255, 255, 0.1);
  color: rgb(251, 191, 36);
  border: 1px solid rgba(255, 255, 255, 0.2);
}


</style> 
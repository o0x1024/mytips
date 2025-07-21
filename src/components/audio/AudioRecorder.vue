<template>
  <div class="audio-recorder">
    <!-- 录制控制面板 -->
    <div v-if="showRecordingPanel" 
         class="recording-panel fixed top-20 right-4 bg-base-100 rounded-lg shadow-lg border border-base-300 p-4 z-50"
         style="width: 320px;">
      
      <!-- 面板标题 -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-medium flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
          音频录制
        </h3>
        <button @click="closePanel" class="btn btn-ghost btn-sm btn-square">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 录制状态显示 -->
      <div class="mb-4">
        <div class="flex items-center justify-between text-sm text-base-content/70 mb-2">
          <span>状态: {{ recordingStatusText }}</span>
          <span v-if="isRecording || isPaused">{{ formatTime(recordingTime) }}</span>
        </div>
        
        <!-- 进度条 -->
        <div class="w-full bg-base-300 rounded-full h-2">
          <div class="bg-primary h-2 rounded-full transition-all duration-300" 
               :style="{ width: `${Math.min(recordingTime / maxRecordingTime * 100, 100)}%` }">
          </div>
        </div>
      </div>

      <!-- 波形显示区域 -->
      <div class="mb-4">
        <canvas ref="waveformCanvas" 
                class="w-full h-16 bg-base-200 rounded border"
                :class="{ 'animate-pulse': isRecording }">
        </canvas>
      </div>

      <!-- 实时转录显示 -->
      <div v-if="enableRealTimeTranscription && (isRecording || isPaused)" class="mb-4">
        <div class="text-sm font-medium mb-2 flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M7 4V2a1 1 0 011-1h8a1 1 0 011 1v2M7 4h10M7 4l-2 16h14L17 4M11 9v4m4-4v4" />
          </svg>
          实时转录
          <span v-if="isTranscribing" class="loading loading-spinner loading-xs"></span>
        </div>
        <div class="p-3 bg-base-200 rounded text-sm max-h-24 overflow-y-auto border"
             :class="{ 'opacity-50': !realtimeTranscriptionText.trim() }">
          <span v-if="realtimeTranscriptionText.trim()">{{ realtimeTranscriptionText }}</span>
          <span v-else class="text-base-content/50 italic">开始说话，转录文本将在这里显示...</span>
          <span v-if="isTranscribing" class="inline-block w-2 h-4 bg-primary animate-pulse ml-1"></span>
        </div>
      </div>

      <!-- 录制控制按钮 -->
      <div class="flex justify-center gap-2 mb-4">
        <!-- 开始/暂停录制 -->
        <button v-if="!isRecording && !isPaused" 
                @click="startRecording"
                :disabled="isLoading"
                class="btn btn-primary btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
        </button>

        <!-- 暂停按钮 -->
        <button v-if="isRecording" 
                @click="pauseRecording"
                class="btn btn-warning btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6" />
          </svg>
        </button>

        <!-- 继续录制按钮 -->
        <button v-if="isPaused" 
                @click="resumeRecording"
                class="btn btn-success btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1" />
          </svg>
        </button>

        <!-- 停止录制 -->
        <button v-if="isRecording || isPaused" 
                @click="stopRecording"
                class="btn btn-error btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10h6v4H9V10z" />
          </svg>
        </button>

        <!-- 重新录制 -->
        <button v-if="hasRecordedAudio && !isRecording && !isPaused" 
                @click="resetRecording"
                class="btn btn-ghost btn-circle"
                title="重新录制">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>
      </div>

      <!-- 音频预览和操作 -->
      <div v-if="hasRecordedAudio" class="space-y-3">
        <!-- 音频播放器 -->
        <div class="bg-base-200 rounded p-3">
          <div class="flex items-center gap-2 mb-2">
            <button @click="togglePlayback" class="btn btn-sm btn-ghost btn-circle">
              <svg v-if="!isPlaying" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6" />
              </svg>
            </button>
            <div class="flex-1">
              <input type="range" 
                     v-model="playbackProgress"
                     @input="seekAudio"
                     min="0" 
                     max="100" 
                     class="range range-primary range-xs" />
            </div>
            <span class="text-xs text-base-content/70">{{ formatTime(audioDuration) }}</span>
          </div>
        </div>

        <!-- 转录选项 -->
        <div class="space-y-2">
          <div class="flex items-center gap-2">
            <input type="checkbox" v-model="enableTranscription" class="checkbox checkbox-primary checkbox-sm" />
            <label class="text-sm">启用语音转文字</label>
          </div>
          
          <div v-if="enableTranscription" class="space-y-2">
            <div>
              <label class="block text-xs text-base-content/70 mb-1">识别语言</label>
              <select v-model="transcriptionLanguage" class="select select-bordered select-sm w-full">
                <option value="auto">自动检测</option>
                <option value="zh">中文 (简体)</option>
                <option value="zh-TW">中文 (繁体)</option>
                <option value="en">English</option>
                <option value="ja">日本語</option>
                <option value="ko">한국어</option>
                <option value="es">Español</option>
                <option value="fr">Français</option>
                <option value="de">Deutsch</option>
                <option value="it">Italiano</option>
                <option value="pt">Português</option>
                <option value="ru">Русский</option>
                <option value="ar">العربية</option>
                <option value="hi">हिन्दी</option>
                <option value="th">ไทย</option>
                <option value="vi">Tiếng Việt</option>
              </select>
            </div>
            
            <div>
              <label class="block text-xs text-base-content/70 mb-1">识别服务</label>
              <select v-model="transcriptionService" class="select select-bordered select-sm w-full">
                <option value="openai">OpenAI Whisper (推荐)</option>
                <option value="azure">Azure Speech Services</option>
                <option value="google">Google Speech-to-Text</option>
                <option value="local">本地 Whisper 模型</option>
              </select>
            </div>
            
            <div v-if="transcriptionService === 'local'" class="alert alert-info py-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-4 w-4" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <span class="text-xs">需要安装 whisper CLI 工具</span>
            </div>
            
            <div class="flex items-center gap-2">
              <input type="checkbox" v-model="enableRealTimeTranscription" class="checkbox checkbox-primary checkbox-sm" />
              <label class="text-sm">实时转录显示</label>
            </div>
            
            <div class="flex items-center gap-2">
              <input type="checkbox" v-model="enableAIAnalysis" class="checkbox checkbox-primary checkbox-sm" />
              <label class="text-sm">启用AI智能分析</label>
            </div>
            
            <div v-if="enableAIAnalysis" class="space-y-2 pl-6">
              <div class="flex flex-wrap gap-2">
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.generateTitle" class="checkbox checkbox-xs" />
                  智能标题
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.extractTags" class="checkbox checkbox-xs" />
                  自动标签
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.generateSummary" class="checkbox checkbox-xs" />
                  内容总结
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.extractKeywords" class="checkbox checkbox-xs" />
                  关键词提取
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.sentimentAnalysis" class="checkbox checkbox-xs" />
                  情感分析
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-2">
          <button @click="insertAudioToNote" 
                  :disabled="isLoading"
                  class="btn btn-primary btn-sm flex-1">
            <span v-if="isLoading" class="loading loading-spinner loading-xs"></span>
            插入到笔记
          </button>
          <button @click="saveAudioFile" 
                  :disabled="isLoading"
                  class="btn btn-ghost btn-sm">
            保存文件
          </button>
        </div>
      </div>

      <!-- 加载状态 -->
      <div v-if="isLoading" class="text-center py-4">
        <span class="loading loading-spinner loading-md"></span>
        <p class="text-sm text-base-content/70 mt-2">{{ loadingMessage }}</p>
      </div>
    </div>

    <!-- 隐藏的音频元素 -->
    <audio ref="audioPlayer" @loadedmetadata="onAudioLoaded" @timeupdate="onTimeUpdate"></audio>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Props
const props = defineProps<{
  visible: boolean
  noteId?: string
}>()

// Emits
const emit = defineEmits<{
  close: []
  audioInserted: [audioId: string, transcription?: string, aiAnalysis?: any]
}>()

// 录制状态
const isRecording = ref(false)
const isPaused = ref(false)
const isLoading = ref(false)
const loadingMessage = ref('')
const hasRecordedAudio = ref(false)
const recordingTime = ref(0)
const maxRecordingTime = ref(600) // 10分钟限制

// 音频相关
const audioBlob = ref<Blob | null>(null)
const audioUrl = ref<string | null>(null)
const isPlaying = ref(false)
const playbackProgress = ref(0)
const audioDuration = ref(0)

// 转录设置
const enableTranscription = ref(true)
const transcriptionLanguage = ref('zh')
const transcriptionService = ref('openai')

// AI分析设置
const enableAIAnalysis = ref(false)
const aiAnalysisOptions = ref({
  generateTitle: true,
  extractTags: true,
  generateSummary: false,
  extractKeywords: false,
  sentimentAnalysis: false
})

// 实时转录设置
const enableRealTimeTranscription = ref(false)
const isTranscribing = ref(false)
const realtimeTranscriptionText = ref('')

// DOM 引用
const waveformCanvas = ref<HTMLCanvasElement | null>(null)
const audioPlayer = ref<HTMLAudioElement | null>(null)

// MediaRecorder 相关
let mediaRecorder: MediaRecorder | null = null
let audioContext: AudioContext | null = null
let analyser: AnalyserNode | null = null
let microphone: MediaStreamAudioSourceNode | null = null
let recordingTimer: ReturnType<typeof setInterval> | null = null
let animationId: number | null = null

// 音频数据
const audioChunks: Blob[] = []

// 计算属性
const showRecordingPanel = computed(() => props.visible)

const recordingStatusText = computed(() => {
  if (isLoading.value) return '处理中...'
  if (isRecording.value) return '录制中'
  if (isPaused.value) return '已暂停'
  if (hasRecordedAudio.value) return '录制完成'
  return '准备录制'
})

// 方法
const closePanel = () => {
  stopRecording()
  emit('close')
}

const startRecording = async () => {
  try {
    isLoading.value = true
    loadingMessage.value = '请求麦克风权限...'

    const stream = await navigator.mediaDevices.getUserMedia({
      audio: {
        echoCancellation: true,
        noiseSuppression: true,
        autoGainControl: true,
        sampleRate: 44100
      }
    })

    // 设置音频分析
    await setupAudioAnalysis(stream)

    // 创建 MediaRecorder
    const options = {
      mimeType: 'audio/webm;codecs=opus',
      audioBitsPerSecond: 128000
    }

    if (!MediaRecorder.isTypeSupported(options.mimeType)) {
      options.mimeType = 'audio/webm'
    }

    mediaRecorder = new MediaRecorder(stream, options)

    mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        audioChunks.push(event.data)
      }
    }

    mediaRecorder.onstop = () => {
      audioBlob.value = new Blob(audioChunks, { type: 'audio/webm' })
      audioUrl.value = URL.createObjectURL(audioBlob.value)
      hasRecordedAudio.value = true
      
      if (audioPlayer.value) {
        audioPlayer.value.src = audioUrl.value
      }
    }

    // 开始录制
    mediaRecorder.start(1000) // 每秒收集数据
    isRecording.value = true
    recordingTime.value = 0

    // 启动计时器
    startTimer()
    startWaveformVisualization()
    
    // 启动实时转录模拟（如果启用）
    if (enableRealTimeTranscription.value) {
      startRealtimeTranscription()
    }

    isLoading.value = false
  } catch (error) {
    console.error('Failed to start recording:', error)
    isLoading.value = false
    // TODO: 显示错误消息
  }
}

const pauseRecording = () => {
  if (mediaRecorder && mediaRecorder.state === 'recording') {
    mediaRecorder.pause()
    isRecording.value = false
    isPaused.value = true
    stopTimer()
    if (enableRealTimeTranscription.value) {
      isTranscribing.value = false
      realtimeTranscriptionText.value = '录制已暂停...'
    }
  }
}

const resumeRecording = () => {
  if (mediaRecorder && mediaRecorder.state === 'paused') {
    mediaRecorder.resume()
    isRecording.value = true
    isPaused.value = false
    startTimer()
    if (enableRealTimeTranscription.value) {
      startRealtimeTranscription()
    }
  }
}

const stopRecording = () => {
  if (mediaRecorder && (mediaRecorder.state === 'recording' || mediaRecorder.state === 'paused')) {
    mediaRecorder.stop()
    
    // 停止所有音频轨道
    if (mediaRecorder.stream) {
      mediaRecorder.stream.getTracks().forEach(track => track.stop())
    }
  }

  isRecording.value = false
  isPaused.value = false
  stopTimer()
  stopWaveformVisualization()
  stopRealtimeTranscription()
}

const resetRecording = () => {
  // 清理音频数据
  audioChunks.length = 0
  audioBlob.value = null
  if (audioUrl.value) {
    URL.revokeObjectURL(audioUrl.value)
    audioUrl.value = null
  }
  
  hasRecordedAudio.value = false
  recordingTime.value = 0
  playbackProgress.value = 0
  audioDuration.value = 0
  isPlaying.value = false

  if (audioPlayer.value) {
    audioPlayer.value.src = ''
  }
}

const setupAudioAnalysis = async (stream: MediaStream) => {
  audioContext = new AudioContext()
  analyser = audioContext.createAnalyser()
  microphone = audioContext.createMediaStreamSource(stream)
  
  analyser.fftSize = 512
  analyser.smoothingTimeConstant = 0.8
  
  microphone.connect(analyser)
}

const startWaveformVisualization = () => {
  if (!analyser || !waveformCanvas.value) return

  const canvas = waveformCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const bufferLength = analyser.frequencyBinCount
  const dataArray = new Uint8Array(bufferLength)

  const draw = () => {
    if (!isRecording.value || !analyser) return

    animationId = requestAnimationFrame(draw)
    
    analyser.getByteFrequencyData(dataArray)

    ctx.fillStyle = 'rgb(30, 30, 30)'
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    const barWidth = canvas.width / bufferLength
    let x = 0

    for (let i = 0; i < bufferLength; i++) {
      const barHeight = (dataArray[i] / 255) * canvas.height * 0.8
      
      const r = barHeight + 25
      const g = 250 * (i / bufferLength)
      const b = 50

      ctx.fillStyle = `rgb(${r},${g},${b})`
      ctx.fillRect(x, canvas.height - barHeight, barWidth, barHeight)

      x += barWidth + 1
    }
  }

  draw()
}

const stopWaveformVisualization = () => {
  if (animationId) {
    cancelAnimationFrame(animationId)
    animationId = null
  }
}

const startTimer = () => {
  recordingTimer = setInterval(() => {
    recordingTime.value += 1
    if (recordingTime.value >= maxRecordingTime.value) {
      stopRecording()
    }
  }, 1000)
}

const stopTimer = () => {
  if (recordingTimer) {
    clearInterval(recordingTimer)
    recordingTimer = null
  }
}

const togglePlayback = () => {
  if (!audioPlayer.value) return

  if (isPlaying.value) {
    audioPlayer.value.pause()
  } else {
    audioPlayer.value.play()
  }
}

const seekAudio = () => {
  if (!audioPlayer.value) return
  
  const seekTime = (playbackProgress.value / 100) * audioDuration.value
  audioPlayer.value.currentTime = seekTime
}

const onAudioLoaded = () => {
  if (audioPlayer.value) {
    audioDuration.value = audioPlayer.value.duration
  }
}

const onTimeUpdate = () => {
  if (audioPlayer.value && audioDuration.value > 0) {
    playbackProgress.value = (audioPlayer.value.currentTime / audioDuration.value) * 100
  }
}

const insertAudioToNote = async () => {
  if (!audioBlob.value || !props.noteId) return

  try {
    isLoading.value = true
    loadingMessage.value = '保存音频文件...'

    // 转换音频为 base64
    const arrayBuffer = await audioBlob.value.arrayBuffer()
    const uint8Array = new Uint8Array(arrayBuffer)
    const base64Audio = btoa(String.fromCharCode(...uint8Array))

    // 保存音频文件
    const audioId = await invoke<string>('save_audio_file', {
      audioData: {
        tip_id: props.noteId,
        audio_data: base64Audio,
        file_format: 'webm',
        duration: Math.round(audioDuration.value * 1000)
      }
    })

    let transcription = ''
    let aiAnalysisResult = null
    
    // 如果启用转录，执行语音转文字
    if (enableTranscription.value) {
      loadingMessage.value = '转录音频内容...'
      
      transcription = await invoke<string>('transcribe_audio', {
        audioId,
        language: transcriptionLanguage.value === 'auto' ? null : transcriptionLanguage.value,
        service: transcriptionService.value
      })
      
      // 如果启用AI分析，执行内容分析
      if (enableAIAnalysis.value && transcription.trim()) {
        loadingMessage.value = '分析音频内容...'
        
        const analysisTypes = []
        if (aiAnalysisOptions.value.generateTitle) analysisTypes.push('title')
        if (aiAnalysisOptions.value.extractTags) analysisTypes.push('tags')
        if (aiAnalysisOptions.value.generateSummary) analysisTypes.push('summary')
        if (aiAnalysisOptions.value.extractKeywords) analysisTypes.push('keywords')
        if (aiAnalysisOptions.value.sentimentAnalysis) analysisTypes.push('sentiment')
        
        if (analysisTypes.length > 0) {
          try {
            aiAnalysisResult = await invoke('analyze_audio_content', {
              audioId,
              transcriptionText: transcription,
              analysisTypes,
              language: transcriptionLanguage.value === 'auto' ? null : transcriptionLanguage.value
            })
          } catch (error) {
            console.warn('AI analysis failed:', error)
          }
        }
      }
    }

    emit('audioInserted', audioId, transcription, aiAnalysisResult)
    resetRecording()
    closePanel()

  } catch (error) {
    console.error('Failed to insert audio:', error)
    // TODO: 显示错误消息
  } finally {
    isLoading.value = false
  }
}

const saveAudioFile = async () => {
  if (!audioBlob.value) return

  try {
    // 创建下载链接
    const url = URL.createObjectURL(audioBlob.value)
    const a = document.createElement('a')
    a.href = url
    a.download = `recording_${new Date().toISOString().replace(/[:.]/g, '-')}.webm`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  } catch (error) {
    console.error('Failed to save audio file:', error)
  }
}

const formatTime = (seconds: number): string => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
}

// 实时转录相关函数
let realtimeTranscriptionTimer: ReturnType<typeof setInterval> | null = null

const startRealtimeTranscription = () => {
  if (!enableRealTimeTranscription.value) return
  
  realtimeTranscriptionText.value = ''
  isTranscribing.value = true
  
  // 模拟实时转录过程
  const mockTranscriptionSegments = [
    '正在监听...',
    '检测到语音输入',
    '开始识别语音内容',
    '转录中...',
    '处理语音信号',
    '生成文本...'
  ]
  
  let segmentIndex = 0
  
  realtimeTranscriptionTimer = setInterval(() => {
    if (segmentIndex < mockTranscriptionSegments.length) {
      realtimeTranscriptionText.value = mockTranscriptionSegments[segmentIndex]
      segmentIndex++
    } else {
      // 循环显示提示
      segmentIndex = 0
    }
  }, 2000)
}

const stopRealtimeTranscription = () => {
  if (realtimeTranscriptionTimer) {
    clearInterval(realtimeTranscriptionTimer)
    realtimeTranscriptionTimer = null
  }
  isTranscribing.value = false
  realtimeTranscriptionText.value = ''
}

// 生命周期
onMounted(() => {
  // 设置canvas尺寸
  nextTick(() => {
    if (waveformCanvas.value) {
      const canvas = waveformCanvas.value
      canvas.width = canvas.offsetWidth
      canvas.height = canvas.offsetHeight
    }
  })

  // 监听音频播放事件
  if (audioPlayer.value) {
    audioPlayer.value.addEventListener('play', () => { isPlaying.value = true })
    audioPlayer.value.addEventListener('pause', () => { isPlaying.value = false })
    audioPlayer.value.addEventListener('ended', () => { 
      isPlaying.value = false
      playbackProgress.value = 0
    })
  }
})

onBeforeUnmount(() => {
  stopRecording()
  stopWaveformVisualization()
  stopRealtimeTranscription()
  
  if (audioUrl.value) {
    URL.revokeObjectURL(audioUrl.value)
  }
  
  if (audioContext) {
    audioContext.close()
  }
})
</script>

<style scoped>
.recording-panel {
  backdrop-filter: blur(10px);
  animation: fadeInScale 0.2s ease-out;
}

@keyframes fadeInScale {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.range {
  height: 4px;
}

.range::-webkit-slider-thumb {
  height: 16px;
  width: 16px;
}

canvas {
  image-rendering: pixelated;
}

@media (max-width: 768px) {
  .recording-panel {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    border-radius: 0;
    z-index: 9999;
  }
}
</style> 
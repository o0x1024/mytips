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
          {{ t('audioRecorder.title') }}
        </h3>
        <button @click="closePanel" class="btn btn-ghost btn-sm btn-square">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 设备和权限状态指示器 -->
      <div class="mb-4 p-3 bg-base-200 rounded-lg">
        <div class="flex items-center justify-between text-sm">
          <div class="flex items-center gap-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
            </svg>
            <span>{{ t('audioRecorder.micStatus') }}</span>
            <button @click="initializeAudioEnvironment" 
                    class="btn btn-ghost btn-xs" 
                    :title="t('audioRecorder.refreshStatus')"
                    :disabled="deviceState === 'checking'">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"
                   :class="{ 'animate-spin': deviceState === 'checking' }">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </button>
          </div>
          <div class="flex items-center gap-3">
            <!-- 设备状态 -->
            <div class="flex items-center gap-1">
              <div class="w-2 h-2 rounded-full" 
                   :class="{
                     'bg-success': deviceState === 'available',
                     'bg-error': deviceState === 'unavailable',
                     'bg-warning animate-pulse': deviceState === 'checking'
                   }"></div>
              <span class="text-xs">{{ 
                deviceState === 'available' ? t('audioRecorder.deviceState.available') : 
                deviceState === 'unavailable' ? t('audioRecorder.deviceState.unavailable') : t('audioRecorder.deviceState.checking')
              }}</span>
            </div>
            <!-- 权限状态 -->
            <div class="flex items-center gap-1">
              <div class="w-2 h-2 rounded-full" 
                   :class="{
                     'bg-success': permissionState === 'granted',
                     'bg-warning': permissionState === 'prompt',
                     'bg-error': permissionState === 'denied',
                     'bg-gray-400': permissionState === 'unknown'
                   }"></div>
              <span class="text-xs">{{ 
                permissionState === 'granted' ? t('audioRecorder.permissionState.granted') : 
                permissionState === 'denied' ? t('audioRecorder.permissionState.denied') : 
                permissionState === 'prompt' ? t('audioRecorder.permissionState.prompt') : t('audioRecorder.permissionState.unknown')
              }}</span>
            </div>
          </div>
        </div>
        <!-- 权限提示 -->
        <div v-if="permissionState === 'denied'" class="mt-2 text-xs text-error">
          <div class="flex items-start gap-1 mb-1">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.728-.833-2.498 0L4.316 15.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
            <span>{{ t('audioRecorder.permissionDenied.title') }}</span>
          </div>
          <div class="pl-4 text-xs text-base-content/60" v-html="t('audioRecorder.permissionDenied.description')">
          </div>
        </div>
        <div v-else-if="deviceState === 'unavailable'" class="mt-2 text-xs text-warning">
          <div class="flex items-start gap-1 mb-1">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.728-.833-2.498 0L4.316 15.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
            <span>{{ t('audioRecorder.deviceUnavailable.title') }}</span>
          </div>
          <div class="pl-4 text-xs text-base-content/60 mb-2" v-html="t('audioRecorder.deviceUnavailable.description')">
          </div>
                     <button @click="testMicrophone" 
                   :disabled="isLoading"
                   class="btn btn-xs btn-outline btn-warning">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
            </svg>
            {{ t('audioRecorder.testMicrophone') }}
          </button>
        </div>
        <div v-else-if="permissionState === 'prompt'" class="mt-2 text-xs text-info">
          <div class="flex items-start gap-1 mb-1">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mt-0.5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{{ t('audioRecorder.permissionPrompt.title') }}</span>
          </div>
          <div class="pl-4 text-xs text-base-content/60" v-html="t('audioRecorder.permissionPrompt.description')">
          </div>
        </div>
      </div>

      <!-- 录制状态显示 -->
      <div class="mb-4">
        <div class="flex items-center justify-between text-sm text-base-content/70 mb-2">
          <span>{{ t('audioRecorder.recordingStatus', { status: recordingStatusText }) }}</span>
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
          {{ t('audioRecorder.realtimeTranscription') }}
          <span v-if="isTranscribing" class="loading loading-spinner loading-xs"></span>
        </div>
        <div class="p-3 bg-base-200 rounded text-sm max-h-24 overflow-y-auto border"
             :class="{ 'opacity-50': !realtimeTranscriptionText.trim() }">
          <span v-if="realtimeTranscriptionText.trim()">{{ realtimeTranscriptionText }}</span>
          <span v-else class="text-base-content/50 italic">{{ t('audioRecorder.transcriptionPlaceholder') }}</span>
          <span v-if="isTranscribing" class="inline-block w-2 h-4 bg-primary animate-pulse ml-1"></span>
        </div>
      </div>

      <!-- 录制控制按钮 -->
      <div class="flex justify-center gap-2 mb-4">
        <!-- 开始/暂停录制 -->
        <button v-if="!isRecording && !isPaused" 
                @click="startRecording"
                :disabled="isLoading || deviceState !== 'available' || permissionState === 'denied'"
                class="btn btn-primary btn-circle"
                :class="{ 
                  'btn-disabled': deviceState !== 'available' || permissionState === 'denied',
                  'tooltip': deviceState !== 'available' || permissionState === 'denied' 
                }"
                :data-tip="deviceState !== 'available' ? t('audioRecorder.micUnavailable') : permissionState === 'denied' ? t('audioRecorder.micPermissionDenied') : ''">
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
                :title="t('audioRecorder.recordAgain')">
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
            <label class="text-sm">{{ t('audioRecorder.enableTranscription') }}</label>
          </div>
          
          <div v-if="enableTranscription" class="space-y-2">
            <div>
              <label class="block text-xs text-base-content/70 mb-1">{{ t('audioRecorder.recognitionLanguage') }}</label>
              <select v-model="transcriptionLanguage" class="select select-bordered select-sm w-full">
                <option value="auto">{{ t('audioRecorder.languages.auto') }}</option>
                <option value="zh">{{ t('audioRecorder.languages.zh') }}</option>
                <option value="zh-TW">{{ t('audioRecorder.languages.zh-TW') }}</option>
                <option value="en">{{ t('audioRecorder.languages.en') }}</option>
                <option value="ja">{{ t('audioRecorder.languages.ja') }}</option>
                <option value="ko">{{ t('audioRecorder.languages.ko') }}</option>
                <option value="es">{{ t('audioRecorder.languages.es') }}</option>
                <option value="fr">{{ t('audioRecorder.languages.fr') }}</option>
                <option value="de">{{ t('audioRecorder.languages.de') }}</option>
                <option value="it">{{ t('audioRecorder.languages.it') }}</option>
                <option value="pt">{{ t('audioRecorder.languages.pt') }}</option>
                <option value="ru">{{ t('audioRecorder.languages.ru') }}</option>
                <option value="ar">{{ t('audioRecorder.languages.ar') }}</option>
                <option value="hi">{{ t('audioRecorder.languages.hi') }}</option>
                <option value="th">{{ t('audioRecorder.languages.th') }}</option>
                <option value="vi">{{ t('audioRecorder.languages.vi') }}</option>
              </select>
            </div>
            
            <div>
              <label class="block text-xs text-base-content/70 mb-1">{{ t('audioRecorder.recognitionService') }}</label>
              <select v-model="transcriptionService" class="select select-bordered select-sm w-full">
                <option value="openai">{{ t('audioRecorder.services.openai') }}</option>
                <option value="azure">{{ t('audioRecorder.services.azure') }}</option>
                <option value="google">{{ t('audioRecorder.services.google') }}</option>
                <option value="local">{{ t('audioRecorder.services.local') }}</option>
              </select>
            </div>
            
            <div v-if="transcriptionService === 'local'" class="alert alert-info py-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-4 w-4" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <span class="text-xs">{{ t('audioRecorder.localModelWarning') }}</span>
            </div>
            
            <div class="flex items-center gap-2">
              <input type="checkbox" v-model="enableRealTimeTranscription" class="checkbox checkbox-primary checkbox-sm" />
              <label class="text-sm">{{ t('audioRecorder.showRealtimeTranscription') }}</label>
            </div>
            
            <div class="flex items-center gap-2">
              <input type="checkbox" v-model="enableAIAnalysis" class="checkbox checkbox-primary checkbox-sm" />
              <label class="text-sm">{{ t('audioRecorder.enableAIAnalysis') }}</label>
            </div>
            
            <div v-if="enableAIAnalysis" class="space-y-2 pl-6">
              <div class="flex flex-wrap gap-2">
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.generateTitle" class="checkbox checkbox-xs" />
                  {{ t('audioRecorder.aiAnalysisOptions.generateTitle') }}
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.extractTags" class="checkbox checkbox-xs" />
                  {{ t('audioRecorder.aiAnalysisOptions.extractTags') }}
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.generateSummary" class="checkbox checkbox-xs" />
                  {{ t('audioRecorder.aiAnalysisOptions.generateSummary') }}
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.extractKeywords" class="checkbox checkbox-xs" />
                  {{ t('audioRecorder.aiAnalysisOptions.extractKeywords') }}
                </label>
                <label class="flex items-center gap-1 text-xs">
                  <input type="checkbox" v-model="aiAnalysisOptions.sentimentAnalysis" class="checkbox checkbox-xs" />
                  {{ t('audioRecorder.aiAnalysisOptions.sentimentAnalysis') }}
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
            {{ t('audioRecorder.insertToNote') }}
          </button>
          <button @click="saveAudioFile" 
                  :disabled="isLoading"
                  class="btn btn-ghost btn-sm">
            {{ t('audioRecorder.saveFile') }}
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
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

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

// 权限状态
const permissionState = ref<'granted' | 'denied' | 'prompt' | 'unknown'>('unknown')
const deviceState = ref<'available' | 'unavailable' | 'checking'>('checking')

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
  if (isLoading.value) return t('audioRecorder.status.processing')
  if (isRecording.value) return t('audioRecorder.status.recording')
  if (isPaused.value) return t('audioRecorder.status.paused')
  if (hasRecordedAudio.value) return t('audioRecorder.status.completed')
  return t('audioRecorder.status.ready')
})

// 方法
const closePanel = () => {
  stopRecording()
  emit('close')
}

// 检查权限状态
const checkPermissionStatus = async () => {
  if (!navigator.permissions || !navigator.permissions.query) {
    return 'unknown'
  }
  
  try {
    const result = await navigator.permissions.query({ name: 'microphone' as PermissionName })
    return result.state
  } catch (error) {
    console.warn('无法查询麦克风权限状态:', error)
    return 'unknown'
  }
}

// 检查音频设备可用性
const checkAudioDeviceAvailability = async () => {
  if (!navigator.mediaDevices || !navigator.mediaDevices.enumerateDevices) {
    return false
  }
  
  try {
    // 首先尝试不带权限的设备枚举
    let devices = await navigator.mediaDevices.enumerateDevices()
    let hasAudioInput = devices.some(device => device.kind === 'audioinput')
    
    // 如果没有找到音频输入设备，可能是权限问题
    // 尝试请求临时权限来获取更准确的设备信息
    if (!hasAudioInput) {
      try {
        const tempStream = await navigator.mediaDevices.getUserMedia({ 
          audio: true 
        })
        
        // 立即停止流，我们只是为了触发权限请求
        tempStream.getTracks().forEach(track => track.stop())
        
        // 重新枚举设备
        devices = await navigator.mediaDevices.enumerateDevices()
        hasAudioInput = devices.some(device => device.kind === 'audioinput')
        
        // 更新权限状态
        permissionState.value = 'granted'
        
      } catch (permError: any) {
        console.warn('无法获取麦克风权限进行设备检测:', permError)
        
        // 根据错误类型更新权限状态
        if (permError.name === 'NotAllowedError') {
          permissionState.value = 'denied'
        } else if (permError.name === 'NotFoundError') {
          // 确实没有设备
          return false
        }
        
        // 对于其他错误，假设设备存在但需要权限
        return true
      }
    }
    
    return hasAudioInput
  } catch (error) {
    console.warn('无法枚举音频设备:', error)
    // 设备枚举失败，但不代表没有设备，可能是权限或API问题
    return true
  }
}

// 检查浏览器是否支持音频录制
const checkAudioRecordingSupport = () => {
  const errors = []
  
  // 检查 navigator.mediaDevices 是否可用
  if (!navigator.mediaDevices) {
    errors.push(t('audioRecorder.errors.browserNotSupported'))
  }
  
  // 检查 getUserMedia 是否可用
  if (!navigator.mediaDevices?.getUserMedia) {
    errors.push(t('audioRecorder.errors.microphoneAccessNotSupported'))
  }
  
  // 检查 MediaRecorder 是否可用
  if (!window.MediaRecorder) {
    errors.push(t('audioRecorder.errors.audioRecordingNotSupported'))
  }
  
  // 检查是否在安全上下文中运行（HTTPS 或 localhost）
  if (location.protocol !== 'https:' && 
      !location.hostname.includes('localhost') && 
      location.hostname !== '127.0.0.1' &&
      location.protocol !== 'tauri:') {
    errors.push(t('audioRecorder.errors.audioRecordingRequiresSecureContext'))
  }
  
  return {
    supported: errors.length === 0,
    errors
  }
}

// 初始化音频环境检查
const initializeAudioEnvironment = async () => {
  try {
    deviceState.value = 'checking'
    
    // 检查权限状态
    permissionState.value = await checkPermissionStatus()
    
    // 检查设备可用性
    const hasAudioDevices = await checkAudioDeviceAvailability()
    deviceState.value = hasAudioDevices ? 'available' : 'unavailable'
    
    console.log(`音频环境状态 - 权限: ${permissionState.value}, 设备: ${deviceState.value}`)
  } catch (error) {
    console.error('初始化音频环境检查失败:', error)
    deviceState.value = 'unavailable'
  }
}

// 测试麦克风功能
const testMicrophone = async () => {
  try {
    deviceState.value = 'checking'
    
    // 直接尝试获取麦克风访问权限
    const stream = await navigator.mediaDevices.getUserMedia({
      audio: {
        echoCancellation: true,
        noiseSuppression: true,
        autoGainControl: true
      }
    })
    
    // 立即停止流
    stream.getTracks().forEach(track => track.stop())
    
    // 更新状态
    permissionState.value = 'granted'
    deviceState.value = 'available'
    
    // 显示成功消息
    const successDiv = document.createElement('div')
    successDiv.className = 'toast toast-top toast-center z-[9999]'
    successDiv.innerHTML = `
      <div class="alert alert-success">
        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <span>${t('audioRecorder.testSuccess')}</span>
      </div>
    `
    
    document.body.appendChild(successDiv)
    
    setTimeout(() => {
      if (successDiv.parentNode) {
        successDiv.parentNode.removeChild(successDiv)
      }
    }, 3000)
    
    // 重新初始化环境检查
    setTimeout(() => {
      initializeAudioEnvironment()
    }, 1000)
    
  } catch (error: any) {
    let errorMessage = t('audioRecorder.testFailed')
    
    switch (error.name) {
      case 'NotAllowedError':
        errorMessage += t('audioRecorder.errors.notAllowed')
        permissionState.value = 'denied'
        deviceState.value = 'unavailable'
        break
      case 'NotFoundError':
        errorMessage += t('audioRecorder.errors.notFound')
        permissionState.value = 'unknown'
        deviceState.value = 'unavailable'
        break
      case 'NotReadableError':
        errorMessage += t('audioRecorder.errors.notReadable')
        permissionState.value = 'unknown'
        deviceState.value = 'unavailable'
        break
      case 'OverconstrainedError':
        errorMessage += t('audioRecorder.errors.overconstrained')
        permissionState.value = 'unknown'
        deviceState.value = 'unavailable'
        break
      case 'SecurityError':
        errorMessage += t('audioRecorder.errors.security')
        permissionState.value = 'denied'
        deviceState.value = 'unavailable'
        break
      default:
        errorMessage += error.message || t('audioRecorder.errors.unknown')
        deviceState.value = 'unavailable'
    }
    
    showErrorMessage(errorMessage)
    console.error('Microphone test failed:', error)
  }
}

// 显示错误消息
const showErrorMessage = (message: string) => {
  // 创建一个临时的错误提示元素
  const errorDiv = document.createElement('div')
  errorDiv.className = 'toast toast-top toast-center z-[9999]'
  errorDiv.innerHTML = `
    <div class="alert alert-error">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>${message}</span>
    </div>
  `
  
  document.body.appendChild(errorDiv)
  
  // 3秒后移除提示
  setTimeout(() => {
    if (errorDiv.parentNode) {
      errorDiv.parentNode.removeChild(errorDiv)
    }
  }, 5000)
}

const startRecording = async () => {
  try {
    isLoading.value = true
    loadingMessage.value = t('audioRecorder.loading.checkingSupport')

    // 首先检查浏览器支持情况
    const supportCheck = checkAudioRecordingSupport()
    if (!supportCheck.supported) {
      const errorMessage = t('audioRecorder.errors.unsupported') + supportCheck.errors.join('\n')
      showErrorMessage(errorMessage)
      console.error('Audio recording not supported:', supportCheck.errors)
      return
    }

    loadingMessage.value = t('audioRecorder.loading.requestingPermission')

    // 尝试获取麦克风权限
    let stream: MediaStream
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          autoGainControl: true,
          sampleRate: 44100
        }
      })
    } catch (mediaError: any) {
      let errorMessage = t('audioRecorder.errors.unsupported')
      
      switch (mediaError.name) {
        case 'NotAllowedError':
          errorMessage += t('audioRecorder.errors.notAllowed')
          break
        case 'NotFoundError':
          errorMessage += t('audioRecorder.errors.notFound')
          break
        case 'NotReadableError':
          errorMessage += t('audioRecorder.errors.notReadable')
          break
        case 'OverconstrainedError':
          errorMessage += t('audioRecorder.errors.overconstrained')
          break
        case 'SecurityError':
          errorMessage += t('audioRecorder.errors.security')
          break
        default:
          errorMessage += mediaError.message || t('audioRecorder.errors.unknown')
      }
      
      showErrorMessage(errorMessage)
      console.error('Media access error:', mediaError)
      return
    }

    // 设置音频分析
    await setupAudioAnalysis(stream)

    // 创建 MediaRecorder
    const options = {
      mimeType: 'audio/webm;codecs=opus',
      audioBitsPerSecond: 128000
    }

    if (!MediaRecorder.isTypeSupported(options.mimeType)) {
      options.mimeType = 'audio/webm'
      if (!MediaRecorder.isTypeSupported(options.mimeType)) {
        // 尝试其他格式
        const fallbackTypes = ['audio/mp4', 'audio/ogg', 'audio/wav']
        let supportedType = null
        
        for (const type of fallbackTypes) {
          if (MediaRecorder.isTypeSupported(type)) {
            supportedType = type
            break
          }
        }
        
        if (supportedType) {
          options.mimeType = supportedType
        } else {
          showErrorMessage(t('audioRecorder.errors.unsupportedFormat'))
          stream.getTracks().forEach(track => track.stop())
          return
        }
      }
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
  } catch (error: any) {
    console.error('Failed to start recording:', error)
    showErrorMessage(`${t('audioRecorder.errors.recordingFailed')}: ${error.message || t('audioRecorder.errors.unknown')}`)
    isLoading.value = false
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
      realtimeTranscriptionText.value = t('audioRecorder.statusPaused')
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
    loadingMessage.value = t('audioRecorder.loading.saving')

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
      loadingMessage.value = t('audioRecorder.loading.transcribing')
      
      transcription = await invoke<string>('transcribe_audio', {
        audioId,
        language: transcriptionLanguage.value === 'auto' ? null : transcriptionLanguage.value,
        service: transcriptionService.value
      })
      
      // 如果启用AI分析，执行内容分析
      if (enableAIAnalysis.value && transcription.trim()) {
        loadingMessage.value = t('audioRecorder.loading.analyzing')
        
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
    t('audioRecorder.transcribing'),
    t('audioRecorder.status.processing'),
    t('audioRecorder.transcriptionPlaceholder')
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
  
  // 初始化音频环境检查
  initializeAudioEnvironment()
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
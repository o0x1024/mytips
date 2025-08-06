<template>
  <div class="audio-player">
    <!-- 播放器面板 -->
    <div v-if="showPlayerPanel" 
         class="player-panel fixed top-20 right-4 bg-base-100 rounded-lg shadow-lg border border-base-300 p-4 z-50"
         style="width: 350px;">
      
      <!-- 面板标题 -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-medium flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-secondary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z" />
          </svg>
          {{ t('audioPlayer.title') }}
        </h3>
        <button @click="closePanel" class="btn btn-ghost btn-sm btn-square">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 音频文件信息 -->
      <div v-if="currentAudio" class="mb-4 p-3 bg-base-200 rounded-lg">
        <div class="text-sm font-medium truncate">{{ currentAudio.file_name }}</div>
        <div class="text-xs text-base-content/60 mt-1">
          {{ t('audioPlayer.info', { duration: formatDuration(currentAudio.duration), format: currentAudio.file_format.toUpperCase() }) }}
        </div>
      </div>

      <!-- 播放状态显示 -->
      <div class="mb-4">
        <div class="flex items-center justify-between text-sm text-base-content/70 mb-2">
          <span>{{ formatTime(currentTime) }}</span>
          <span class="text-center">{{ playbackStatusText }}</span>
          <span>{{ formatTime(duration) }}</span>
        </div>
        
        <!-- 进度条 -->
        <div class="w-full bg-base-300 rounded-full h-3 cursor-pointer" @click="seekTo">
          <div class="bg-secondary h-3 rounded-full transition-all duration-300 relative" 
               :style="{ width: `${progress}%` }">
            <div class="absolute right-0 top-1/2 transform translate-x-1/2 -translate-y-1/2 w-3 h-3 bg-secondary-focus rounded-full shadow-sm"></div>
          </div>
        </div>
      </div>

      <!-- 波形显示区域 -->
      <div class="mb-4">
        <canvas ref="waveformCanvas" 
                class="w-full h-16 bg-base-200 rounded border cursor-pointer"
                @click="seekToWaveform">
        </canvas>
      </div>

      <!-- 播放控制按钮 -->
      <div class="flex justify-center items-center gap-3 mb-4">
        <!-- 快退15秒 -->
        <button @click="skipBackward" 
                :disabled="!currentAudio"
                class="btn btn-ghost btn-circle btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12.066 11.2a1 1 0 000 1.6l5.334 4A1 1 0 0019 16V8a1 1 0 00-1.6-.8l-5.334 4zM4.066 11.2a1 1 0 000 1.6l5.334 4A1 1 0 0011 16V8a1 1 0 00-1.6-.8l-5.334 4z" />
          </svg>
        </button>

        <!-- 播放/暂停按钮 -->
        <button v-if="!isPlaying" 
                @click="playAudio"
                :disabled="!currentAudio || isLoading"
                class="btn btn-secondary btn-circle">
                     <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
             <path d="M8 5v14l11-7z"/>
           </svg>
        </button>

        <button v-else 
                @click="pauseAudio"
                class="btn btn-warning btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6" />
          </svg>
        </button>

        <!-- 停止按钮 -->
        <button @click="stopAudio" 
                :disabled="!currentAudio"
                class="btn btn-ghost btn-circle btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10h6v4H9z" />
          </svg>
        </button>

        <!-- 快进15秒 -->
        <button @click="skipForward" 
                :disabled="!currentAudio"
                class="btn btn-ghost btn-circle btn-sm">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 4v3a1 1 0 001.6.8L12 4l5.4 3.8A1 1 0 0019 7V4a1 1 0 00-1-1H6a1 1 0 00-1 1zM5 20v-3a1 1 0 011.6-.8L12 20l5.4-3.8a1 1 0 001.6.8v3a1 1 0 01-1 1H6a1 1 0 01-1-1z" />
          </svg>
        </button>
      </div>

      <!-- 音量控制 -->
      <div class="mb-4">
        <div class="flex items-center gap-3">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-base-content/60" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 14.142M6.343 6.343A8 8 0 004.222 12a8 8 0 002.121 5.657" />
          </svg>
          <input type="range" 
                 v-model="volume" 
                 min="0" 
                 max="100" 
                 @input="updateVolume"
                 class="range range-secondary range-sm flex-1">
          <span class="text-xs text-base-content/60 min-w-8">{{ volume }}%</span>
        </div>
      </div>

      <!-- 播放速度控制 -->
      <div class="mb-4">
        <div class="flex items-center justify-between">
          <label class="text-sm text-base-content/70">{{ t('audioPlayer.playbackSpeed') }}</label>
          <select v-model="playbackRate" @change="updatePlaybackRate" class="select select-sm select-bordered">
            <option value="0.5">0.5x</option>
            <option value="0.75">0.75x</option>
            <option value="1.0">1.0x</option>
            <option value="1.25">1.25x</option>
            <option value="1.5">1.5x</option>
            <option value="2.0">2.0x</option>
          </select>
        </div>
      </div>

      <!-- 音频文件列表 -->
      <div v-if="audioFiles.length > 0" class="mb-4">
        <div class="text-sm font-medium mb-2">{{ t('audioPlayer.fileList') }}</div>
        <div class="max-h-32 overflow-y-auto space-y-1">
          <div v-for="audio in audioFiles" 
               :key="audio.id"
               @click="loadAudio(audio)"
               class="p-2 bg-base-200 rounded cursor-pointer hover:bg-base-300 transition-colors"
               :class="{ 'ring-2 ring-secondary': currentAudio?.id === audio.id }">
            <div class="text-sm truncate">{{ audio.file_name }}</div>
            <div class="text-xs text-base-content/60">{{ formatDuration(audio.duration) }}</div>
          </div>
        </div>
      </div>

      <!-- 转录文本显示 -->
      <div v-if="currentAudio?.transcription" class="mb-4">
        <div class="text-sm font-medium mb-2">{{ t('audioPlayer.transcription') }}</div>
        <div class="p-3 bg-base-200 rounded text-sm max-h-24 overflow-y-auto">
          {{ currentAudio.transcription }}
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex gap-2">
        <button @click="insertAudioLink" 
                :disabled="!currentAudio"
                class="btn btn-secondary btn-sm flex-1">
          {{ t('audioPlayer.insertLink') }}
        </button>
        <button @click="insertTranscription" 
                :disabled="!currentAudio?.transcription"
                class="btn btn-accent btn-sm flex-1">
          {{ t('audioPlayer.insertTranscription') }}
        </button>
        <button @click="deleteAudio" 
                :disabled="!currentAudio"
                class="btn btn-error btn-sm">
          {{ t('audioPlayer.delete') }}
        </button>
      </div>
    </div>

    <!-- 音频元素 -->
    <audio ref="audioElement" 
           @loadedmetadata="onAudioLoaded"
           @timeupdate="onTimeUpdate"
           @ended="onAudioEnded"
           @error="onAudioError"
           preload="metadata">
    </audio>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCachedAudioUrl } from '../../utils/audioCache'
import { useI18n } from 'vue-i18n'
import { showConfirm } from '../../services/dialog'

const { t } = useI18n()
// Props
interface Props {
  show?: boolean
  tipId?: string
}

const props = withDefaults(defineProps<Props>(), {
  show: false,
  tipId: ''
})

// Emits
interface Emits {
  (e: 'close'): void
  (e: 'insert-audio', data: { text: string, type: 'link' | 'transcription' }): void
}

const emit = defineEmits<Emits>()

// 响应式数据
const showPlayerPanel = ref(props.show)
const audioElement = ref<HTMLAudioElement>()
const waveformCanvas = ref<HTMLCanvasElement>()

// 音频播放状态
const isPlaying = ref(false)
const isLoading = ref(false)
const currentTime = ref(0)
const duration = ref(0)
const volume = ref(80)
const playbackRate = ref(1.0)

// 音频数据
const currentAudio = ref<any>(null)
const audioFiles = ref<any[]>([])
const audioBuffer = ref<ArrayBuffer | null>(null)

// 波形数据
const waveformData = ref<number[]>([])

// 计算属性
const progress = computed(() => {
  if (duration.value === 0) return 0
  return (currentTime.value / duration.value) * 100
})

const playbackStatusText = computed(() => {
  if (isLoading.value) return t('audioPlayer.status.loading')
  if (isPlaying.value) return t('audioPlayer.status.playing')
  if (currentTime.value > 0) return t('audioPlayer.status.paused')
  return t('audioPlayer.status.ready')
})

// 方法
const closePanel = () => {
  stopAudio()
  showPlayerPanel.value = false
  emit('close')
}

const formatTime = (seconds: number): string => {
  const mins = Math.floor(seconds / 60)
  const secs = Math.floor(seconds % 60)
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
}

const formatDuration = (ms: number): string => {
  const seconds = Math.floor(ms / 1000)
  return formatTime(seconds)
}

const loadAudioFiles = async () => {
  if (!props.tipId) return
  
  try {
    isLoading.value = true
    const files = await invoke('get_tip_audio_files', { tipId: props.tipId })
    
    // 过滤掉视频文件，只保留音频文件
    const audioOnlyFiles = (files as any[]).filter(file => {
      const format = file.file_format?.toLowerCase()
      // 排除常见的视频格式
      const videoFormats = ['mp4', 'avi', 'mov', 'mkv', 'wmv', 'flv', 'webm']
      return !videoFormats.includes(format)
    })
    
    audioFiles.value = audioOnlyFiles
    
    // 如果有音频文件且没有当前音频，加载第一个
    if (audioFiles.value.length > 0 && !currentAudio.value) {
      await loadAudio(audioFiles.value[0])
    }
  } catch (error) {
    console.error('Error loading audio files:', error)
  } finally {
    isLoading.value = false
  }
}

const loadAudio = async (audio: any) => {
  try {
    isLoading.value = true
    stopAudio()
    
    const cached = getCachedAudioUrl(audio.audio_id)
    if (cached) {
      if (audioElement.value) {
        audioElement.value.src = cached
        audioElement.value.load()
      }
      currentAudio.value = audio
      await generateWaveform()
      isLoading.value = false
      return
    }

    currentAudio.value = audio
    
    // 获取音频数据
    const audioData: { audio_data: string; file_format: string } = await invoke(
      'get_audio_file', 
      { audioId: audio.audio_id }
    )
    
    if (audioData && audioData.audio_data) {
      // Base64 解码
      const byteCharacters = atob(audioData.audio_data)
      const byteNumbers = new Array(byteCharacters.length)
      for (let i = 0; i < byteCharacters.length; i++) {
        byteNumbers[i] = byteCharacters.charCodeAt(i)
      }
      const byteArray = new Uint8Array(byteNumbers)
      audioBuffer.value = byteArray.buffer

      // 创建 Blob URL - 根据文件格式设置正确的 MIME 类型
      let mimeType = `audio/${audioData.file_format || 'webm'}`
      if (audioData.file_format === 'mp4') {
        mimeType = 'video/mp4'
      }
      const blob = new Blob([audioBuffer.value], { type: mimeType })
      const url = URL.createObjectURL(blob)
      
      if (audioElement.value) {
        audioElement.value.src = url
        audioElement.value.load()
      }
      
      // 生成波形数据
      await generateWaveform()
    } else {
      throw new Error('Failed to retrieve audio data.')
    }
    
  } catch (error) {
    console.error('Error loading audio:', error)
  } finally {
    isLoading.value = false
  }
}

const playAudio = async () => {
  if (audioElement.value && currentAudio.value) {
    try {
      await audioElement.value.play()
      isPlaying.value = true
    } catch (error) {
      console.error('Error playing audio:', error)
      isPlaying.value = false
      // 如果是 NotSupportedError，可能是因为文件格式不支持
      if (error instanceof DOMException && error.name === 'NotSupportedError') {
        console.warn('Audio format not supported for playback')
      }
    }
  }
}

const pauseAudio = () => {
  if (audioElement.value) {
    audioElement.value.pause()
    isPlaying.value = false
  }
}

const stopAudio = () => {
  if (audioElement.value) {
    audioElement.value.pause()
    audioElement.value.currentTime = 0
    isPlaying.value = false
    currentTime.value = 0
  }
}

const skipForward = () => {
  if (audioElement.value) {
    audioElement.value.currentTime = Math.min(
      audioElement.value.currentTime + 15,
      duration.value
    )
  }
}

const skipBackward = () => {
  if (audioElement.value) {
    audioElement.value.currentTime = Math.max(
      audioElement.value.currentTime - 15,
      0
    )
  }
}

const seekTo = (event: MouseEvent) => {
  if (!audioElement.value || !duration.value) return
  
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  const clickX = event.clientX - rect.left
  const percentage = clickX / rect.width
  const newTime = percentage * duration.value
  
  audioElement.value.currentTime = newTime
}

const seekToWaveform = (event: MouseEvent) => {
  if (!audioElement.value || !duration.value) return
  
  const rect = waveformCanvas.value!.getBoundingClientRect()
  const clickX = event.clientX - rect.left
  const percentage = clickX / rect.width
  const newTime = percentage * duration.value
  
  audioElement.value.currentTime = newTime
}

const updateVolume = () => {
  if (audioElement.value) {
    audioElement.value.volume = volume.value / 100
  }
}

const updatePlaybackRate = () => {
  if (audioElement.value) {
    audioElement.value.playbackRate = parseFloat(playbackRate.value.toString())
  }
}

const generateWaveform = async () => {
  if (!audioBuffer.value || !waveformCanvas.value) return
  
  try {
    // 检查是否为视频文件
    if (currentAudio.value?.file_format === 'mp4') {
      // 对于视频文件，生成简单的占位波形
      const canvas = waveformCanvas.value
      const ctx = canvas.getContext('2d')!
      const width = canvas.width = canvas.offsetWidth * 2
      canvas.height = canvas.offsetHeight * 2
      
      ctx.scale(2, 2)
      
      // 生成简单的波形数据
      const waveform: number[] = []
      for (let i = 0; i < width / 4; i++) {
        waveform.push(Math.random() * 0.5 + 0.1) // 随机高度的简单波形
      }
      
      waveformData.value = waveform
      drawWaveform()
      return
    }
    
    const audioContext = new AudioContext()
    const audioData = await audioContext.decodeAudioData(audioBuffer.value.slice(0))
    
    const canvas = waveformCanvas.value
    const ctx = canvas.getContext('2d')!
    const width = canvas.width = canvas.offsetWidth * 2
    canvas.height = canvas.offsetHeight * 2
    
    ctx.scale(2, 2)
    
    const samples = audioData.getChannelData(0)
    const blockSize = Math.floor(samples.length / (width / 4))
    const waveform: number[] = []
    
    for (let i = 0; i < width / 4; i++) {
      const start = i * blockSize
      const end = start + blockSize
      let max = 0
      
      for (let j = start; j < end; j++) {
        const sample = Math.abs(samples[j])
        if (sample > max) max = sample
      }
      
      waveform.push(max)
    }
    
    waveformData.value = waveform
    drawWaveform()
    
  } catch (error) {
    console.error('Error generating waveform:', error)
    // 如果解码失败，生成简单的占位波形
    const canvas = waveformCanvas.value
    if (canvas) {
      const ctx = canvas.getContext('2d')!
      const width = canvas.width = canvas.offsetWidth * 2
      canvas.height = canvas.offsetHeight * 2
      
      ctx.scale(2, 2)
      
      const waveform: number[] = []
      for (let i = 0; i < width / 4; i++) {
        waveform.push(Math.random() * 0.3 + 0.1)
      }
      
      waveformData.value = waveform
      drawWaveform()
    }
  }
}

const drawWaveform = () => {
  if (!waveformCanvas.value || waveformData.value.length === 0) return
  
  const canvas = waveformCanvas.value
  const ctx = canvas.getContext('2d')!
  const width = canvas.offsetWidth
  const height = canvas.offsetHeight
  
  ctx.clearRect(0, 0, width, height)
  
  const barWidth = width / waveformData.value.length
  const progressX = (currentTime.value / duration.value) * width
  
  waveformData.value.forEach((value, index) => {
    const barHeight = value * height * 0.8
    const x = index * barWidth
    const y = (height - barHeight) / 2
    
    // 根据播放进度设置颜色
    ctx.fillStyle = x < progressX ? '#ff6b35' : '#94a3b8'
    ctx.fillRect(x, y, barWidth - 1, barHeight)
  })
}

const insertAudioLink = () => {
  if (!currentAudio.value) return
  
  const text = `[🎵 ${currentAudio.value.file_name}](audio:${currentAudio.value.audio_id})`
  emit('insert-audio', { text, type: 'link' })
}

const insertTranscription = () => {
  if (!currentAudio.value?.transcription) return
  
  emit('insert-audio', { 
    text: currentAudio.value.transcription, 
    type: 'transcription' 
  })
}

const deleteAudio = async () => {
  if (!currentAudio.value) return
  
  const confirmed = await showConfirm(t('audioPlayer.deleteConfirmationMessage'), {
    title: t('audioPlayer.deleteConfirmationTitle')
  })
  if (!confirmed) return
  
  try {
    await invoke('delete_audio_file', { audioId: currentAudio.value.audio_id })
    
    // 重新加载音频文件列表
    await loadAudioFiles()
    
    // 如果删除的是当前播放的音频，清除当前音频
    if (audioFiles.value.length === 0) {
      currentAudio.value = null
      stopAudio()
    } else {
      await loadAudio(audioFiles.value[0])
    }
    
  } catch (error) {
    console.error('Error deleting audio:', error)
  }
}

// 音频事件处理
const onAudioLoaded = () => {
  if (audioElement.value) {
    duration.value = audioElement.value.duration
    updateVolume()
    updatePlaybackRate()
  }
}

const onTimeUpdate = () => {
  if (audioElement.value) {
    currentTime.value = audioElement.value.currentTime
    drawWaveform()
  }
}

const onAudioEnded = () => {
  isPlaying.value = false
  currentTime.value = 0
  
  // 自动播放下一个音频（如果有）
  const currentIndex = audioFiles.value.findIndex(audio => audio.id === currentAudio.value?.id)
  if (currentIndex >= 0 && currentIndex < audioFiles.value.length - 1) {
    loadAudio(audioFiles.value[currentIndex + 1])
  }
}

const onAudioError = (error: Event) => {
  console.error('Audio error:', error)
  isPlaying.value = false
  isLoading.value = false
  
  // 获取错误详情
  const audioElement = error.target as HTMLAudioElement
  if (audioElement && audioElement.error) {
    const mediaError = audioElement.error
    let errorMessage = 'Unknown audio error'
    
    switch (mediaError.code) {
      case MediaError.MEDIA_ERR_ABORTED:
        errorMessage = 'Audio playback was aborted'
        break
      case MediaError.MEDIA_ERR_NETWORK:
        errorMessage = 'Network error occurred while loading audio'
        break
      case MediaError.MEDIA_ERR_DECODE:
        errorMessage = 'Audio decoding failed - file may be corrupted or unsupported format'
        break
      case MediaError.MEDIA_ERR_SRC_NOT_SUPPORTED:
        errorMessage = 'Audio format not supported'
        break
    }
    
    console.warn(`Audio playback error: ${errorMessage}`, mediaError)
    
    // 对于视频文件，这是预期的行为
    if (currentAudio.value?.file_format === 'mp4') {
      console.info('MP4 video file detected - audio playback not supported in audio player')
    }
  }
}

// 生命周期
onMounted(async () => {
  showPlayerPanel.value = props.show
  if (props.tipId) {
    await loadAudioFiles()
  }
})

onUnmounted(() => {
  stopAudio()
  
  // 清理 Blob URL
  if (audioElement.value?.src) {
    URL.revokeObjectURL(audioElement.value.src)
  }
})

// 监听 props 变化
watch(() => props.show, (newValue: boolean) => {
  showPlayerPanel.value = newValue
  if (newValue && props.tipId) {
    loadAudioFiles()
  }
})

watch(() => props.tipId, (newTipId: string) => {
  if (newTipId) {
    loadAudioFiles()
  }
})


</script>

<style scoped>
.audio-player {
  position: relative;
}

.player-panel {
  animation: slideInFromRight 0.3s ease-out;
}

@keyframes slideInFromRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.range {
  background: transparent;
}

.range::-webkit-slider-track {
  background: hsl(var(--b3));
}

.range::-webkit-slider-thumb {
  background: hsl(var(--s));
}

.range::-moz-range-track {
  background: hsl(var(--b3));
}

.range::-moz-range-thumb {
  background: hsl(var(--s));
}

/* 自定义滚动条 */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: hsl(var(--b3));
  border-radius: 2px;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: hsl(var(--bc) / 0.3);
  border-radius: 2px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--bc) / 0.5);
}
</style>
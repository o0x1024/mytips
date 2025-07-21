// 音频 Blob URL 缓存工具
// 使用简单 Map 存储，可在前端多个组件共享

const audioUrlCache = new Map<string, string>()

export function getCachedAudioUrl(id: string): string | undefined {
  return audioUrlCache.get(id)
}

export function setCachedAudioUrl(id: string, url: string): void {
  audioUrlCache.set(id, url)
}

export function revokeAudioUrl(id: string): void {
  const url = audioUrlCache.get(id)
  if (url) {
    URL.revokeObjectURL(url)
    audioUrlCache.delete(id)
  }
} 
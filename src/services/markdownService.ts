import { unified } from 'unified'
import remarkParse from 'remark-parse'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import rehypeSanitize, { defaultSchema } from 'rehype-sanitize'
import rehypeStringify from 'rehype-stringify'
import rehypeSlug from 'rehype-slug'
import rehypePrism from 'rehype-prism-plus'
import rehypeRaw from 'rehype-raw'
import rehypeMermaid from 'rehype-mermaid'
import { visit } from 'unist-util-visit'
import type { Element, Root as HastRoot } from 'hast'
import { invoke } from '@tauri-apps/api/core'
import { getCachedAudioUrl, setCachedAudioUrl } from '../utils/audioCache'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import emoji from 'remark-emoji';
// 移除 remarkMarkmap 导入
// import remarkMarkmap from 'remark-markmap'

// rehype-mermaid 插件会自动处理 Mermaid 图表渲染

// 定义TOC条目类型
export interface TocItem {
  id: string;
  level: number;
  text: string;
}

/**
 * 这是一个临时的 rehype 插件，用于提取TOC。
 * 它会在处理过程中收集标题信息。
 *
 * @param toc - 用于存储TOC条目的数组。
 */
function rehypeToc(toc: TocItem[]) {
  return (tree: HastRoot) => {
    visit(tree, 'element', (node) => {
      if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(node.tagName)) {
        const id = node.properties?.id as string || ''
        const level = parseInt(node.tagName.charAt(1), 10)
        // 提取纯文本内容
        let text = ''
        visit(node, 'text', (textNode) => {
          text += textNode.value
        })

        if (id && text) {
          toc.push({ id, level, text: text.trim() })
        }
      }
    })
  }
}

/**
 * 自定义Rehype插件，用于处理 local:// 协议的媒体文件（图片和视频）
 * @param images - 笔记中的媒体文件数据
 */
function rehypeLocalImages(images: Record<string, string>) {
  return (tree: HastRoot) => {
    visit(tree, 'element', (node: Element) => {
      // 处理图片
      if (node.tagName === 'img' && typeof node.properties?.src === 'string' && node.properties.src.startsWith('local://')) {
        const imageId = node.properties.src.replace('local://', '')
        if (images[imageId]) {
          node.properties.src = images[imageId]
        } else {
          console.warn(`Image with id "${imageId}" not found in provided images map.`)
          // 可选：设置一个占位符图片或移除该图片
          node.properties.src = ''
        }
      }
      
      // 处理视频源
      if (node.tagName === 'source' && typeof node.properties?.src === 'string' && node.properties.src.startsWith('local://')) {
        const mediaId = node.properties.src.replace('local://', '')
        console.log(`Processing video source with mediaId: ${mediaId}`, { availableImages: Object.keys(images) })
        if (images[mediaId]) {
          node.properties.src = images[mediaId]
          console.log(`Successfully replaced video source for ${mediaId}`)
        } else {
          console.warn(`Media file with id "${mediaId}" not found in provided images map.`, { availableImages: Object.keys(images) })
          // 设置空源
          node.properties.src = ''
        }
      }
    })
  }
}


/**
 * 异步自定义Rehype插件，用于处理 audio:// 和 local:// 协议的音频/视频
 */
function rehypeLocalAudio() {
  return async (tree: HastRoot) => {
    const nodesToProcess: { node: Element, audioId: string }[] = []
         console.log('11111111')

    visit(tree, 'element', (node: Element) => {
      if (
        (node.tagName === 'source' || node.tagName === 'video' || node.tagName === 'audio') &&
        typeof node.properties?.src === 'string'
      ) {
        let audioId = ''
        
        // 处理带协议前缀的URL
        if ( node.properties.src.startsWith('local://')) {
          audioId = node.properties.src.replace(/^local:\/\//, '')
        }
        // 处理直接的audio_id或media_id格式（没有协议前缀）
        else if (node.properties.src.match(/^(audio_[a-f0-9]{32})/)) {
          audioId = node.properties.src
        }
        
        if (audioId) {
          console.log(`[rehypeLocalAudio] Found ${node.tagName} tag with audioId: ${audioId}`)
          nodesToProcess.push({ node, audioId })
        }
      }
    })

    for (const { node, audioId } of nodesToProcess) {
      let objectUrl = getCachedAudioUrl(audioId)
      console.log(`[rehypeLocalAudio] Cached URL for ${audioId}: ${objectUrl}`)
      if (!objectUrl) {
        try {
          console.log(`[rehypeLocalAudio] Processing audio/video ID: ${audioId}`)
          const audioData: { audio_data: string; file_format: string } = await invoke('get_audio_file', { audioId })
          if (audioData && audioData.audio_data) {
            const format = audioData.file_format || 'webm'
            console.log(`[rehypeLocalAudio] File format: ${format}, data length: ${audioData.audio_data.length}`)
            
            // 验证base64数据
            try {
              const byteCharacters = atob(audioData.audio_data)
              const byteNumbers = new Array(byteCharacters.length)
              for (let i = 0; i < byteCharacters.length; i++) {
                byteNumbers[i] = byteCharacters.charCodeAt(i)
              }
              const byteArray = new Uint8Array(byteNumbers)
              console.log(`[rehypeLocalAudio] Decoded byte array length: ${byteArray.length}`)
              
              // 根据格式确定MIME类型
                let mimeType = `audio/${format}`
                if (['mp4', 'webm', 'ogg', 'avi', 'mov'].includes(format)) {
                  // 对于视频格式，先尝试不带编解码器的基本MIME类型
                  if (format === 'mp4') {
                    mimeType = 'video/mp4'
                  } else if (format === 'webm') {
                    mimeType = 'video/webm'
                  } else if (format === 'ogg') {
                    mimeType = 'video/ogg'
                  } else if (format === 'mov') {
                    mimeType = 'video/quicktime'
                  } else {
                    mimeType = `video/${format}`
                  }
                }
              console.log(`[rehypeLocalAudio] Creating blob with MIME type: ${mimeType}`)
              
              // 检查浏览器是否支持该MIME类型
               const testVideo = document.createElement('video')
               const canPlay = testVideo.canPlayType(mimeType)
               console.log(`[rehypeLocalAudio] Browser support for ${mimeType}: ${canPlay}`)
               
               // 如果不支持，尝试备选MIME类型
               if (!canPlay && format === 'mp4') {
                 mimeType = 'video/mp4; codecs="avc1.42E01E"'
                 console.log(`[rehypeLocalAudio] Trying fallback MIME type: ${mimeType}`)
                 const canPlayFallback = testVideo.canPlayType(mimeType)
                 console.log(`[rehypeLocalAudio] Browser support for fallback: ${canPlayFallback}`)
                 if (!canPlayFallback) {
                   mimeType = 'video/mp4'
                   console.log(`[rehypeLocalAudio] Using basic MP4 MIME type`)
                 }
               }
               
               const blob = new Blob([byteArray], { type: mimeType })
               console.log(`[rehypeLocalAudio] Blob created - size: ${blob.size}, type: ${blob.type}`)
               
               objectUrl = URL.createObjectURL(blob)
               console.log(`[rehypeLocalAudio] Created object URL: ${objectUrl}`)
               
               // 验证blob URL是否有效
               testVideo.src = objectUrl
               testVideo.addEventListener('loadedmetadata', () => {
                 console.log(`[rehypeLocalAudio] Video metadata loaded - duration: ${testVideo.duration}s, dimensions: ${testVideo.videoWidth}x${testVideo.videoHeight}`)
               })
               testVideo.addEventListener('error', (e) => {
                 console.error(`[rehypeLocalAudio] Video load error:`, e)
                 if (testVideo.error) {
                   const errorCodes: { [key: number]: string } = {
                      1: 'MEDIA_ERR_ABORTED - 播放被中止',
                      2: 'MEDIA_ERR_NETWORK - 网络错误',
                      3: 'MEDIA_ERR_DECODE - 解码错误',
                      4: 'MEDIA_ERR_SRC_NOT_SUPPORTED - 格式不支持'
                    }
                   console.error(`[rehypeLocalAudio] Error code: ${testVideo.error.code} - ${errorCodes[testVideo.error.code] || '未知错误'}`)
                   console.error(`[rehypeLocalAudio] Error message: ${testVideo.error.message}`)
                 }
               })
              
              setCachedAudioUrl(audioId, objectUrl)
            } catch (decodeError) {
              console.error(`[rehypeLocalAudio] Failed to decode base64 data:`, decodeError)
              objectUrl = ''
            }
          } else {
            console.warn(`[rehypeLocalAudio] No audio data found for ID: ${audioId}`)
          }
        } catch (error) {
          console.error(`[rehypeLocalAudio] Failed to load audio/video data for ID ${audioId}:`, error)
          objectUrl = '' // or a placeholder URL
        }
      }
      
      if (node.properties) {
        node.properties.src = objectUrl
      }
    }
  }
}

// 使用标准的 rehype-mermaid 插件替代自定义实现


/**
 * 渲染Markdown为HTML，并提取TOC。
 * @param markdown - Markdown原文.
 * @param images - 笔记中包含的图片数据.
 * @returns 一个包含HTML和TOC数组的对象.
 */
export async function renderMarkdown(markdown: string, images: Record<string, string> = {}): Promise<{ html: string; toc: TocItem[] }> {
  const toc: TocItem[] = []

  const file = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkMath)
    .use(remarkRehype, { allowDangerousHtml: true, math: true })
    .use(rehypeLocalImages, images)
    .use(rehypeLocalAudio)
    .use(rehypeMermaid, {
      strategy: 'img-svg',
      mermaidConfig: {
        theme: 'base',
        themeVariables: {
          primaryColor: '#ffffff',
          primaryTextColor: '#000000',
          primaryBorderColor: '#333333',
          lineColor: '#333333',
          sectionBkgColor: '#ffffff',
          altSectionBkgColor: '#f8f9fa',
          gridColor: '#e0e0e0',
          secondaryColor: '#f0f0f0',
          tertiaryColor: '#ffffff',
          background: '#ffffff',
          mainBkg: '#ffffff',
          secondBkg: '#f8f9fa',
          tertiaryBkg: '#ffffff'
        },
        flowchart: {
          useMaxWidth: true,
          htmlLabels: true
        },
        sequence: {
          useMaxWidth: true,
          wrap: true
        },
        gantt: {
          useMaxWidth: true
        }
      }
    })
    .use(rehypeRaw)
    .use(rehypeSlug)
    .use(emoji)
    .use(rehypeToc, toc)
    .use(rehypeKatex)
    .use(rehypeSanitize, {
      ...defaultSchema,
      attributes: {
        ...defaultSchema.attributes,
        '*': [...(defaultSchema.attributes?.['*'] || []), 'class', 'style', 'data-enhanced', 'data-highlighted', 'loading', 'id', 'xmlns', 'viewBox', 'width', 'height', 'fill', 'stroke', 'stroke-width', 'stroke-dasharray', 'stroke-linecap', 'stroke-linejoin', 'transform', 'd', 'x', 'y', 'x1', 'y1', 'x2', 'y2', 'cx', 'cy', 'r', 'rx', 'ry', 'points', 'text-anchor', 'dominant-baseline', 'font-family', 'font-size', 'font-weight', 'title'],
        code: [...(defaultSchema.attributes?.code || []), 'className', 'title'],
        source: ['src', 'type'],
        audio: ['controls', 'src'],
        video: ['controls', 'src', 'width', 'height', 'style', 'autoplay', 'loop', 'muted', 'poster'],
        img: [...(defaultSchema.attributes?.img || []), 'loading'],
        div: [...(defaultSchema.attributes?.div || []), 'className', 'style'],
        svg: ['xmlns', 'viewBox', 'width', 'height', 'style', 'class', 'id'],
        g: ['transform', 'class', 'id'],
        path: ['d', 'fill', 'stroke', 'stroke-width', 'class'],
        rect: ['x', 'y', 'width', 'height', 'fill', 'stroke', 'stroke-width', 'rx', 'ry', 'class'],
        circle: ['cx', 'cy', 'r', 'fill', 'stroke', 'stroke-width', 'class'],
        ellipse: ['cx', 'cy', 'rx', 'ry', 'fill', 'stroke', 'stroke-width', 'class'],
        line: ['x1', 'y1', 'x2', 'y2', 'stroke', 'stroke-width', 'class'],
        polyline: ['points', 'fill', 'stroke', 'stroke-width', 'class'],
        polygon: ['points', 'fill', 'stroke', 'stroke-width', 'class'],
        text: ['x', 'y', 'text-anchor', 'dominant-baseline', 'font-family', 'font-size', 'font-weight', 'fill', 'class'],
        tspan: ['x', 'y', 'dx', 'dy', 'text-anchor', 'class'],
        marker: ['id', 'markerWidth', 'markerHeight', 'refX', 'refY', 'orient', 'markerUnits'],
        defs: [],
        style: ['type']
      },
      protocols: {
        ...defaultSchema.protocols,
        src: [...((defaultSchema as any).protocols?.src || ['http', 'https', 'mailto']), 'data', 'audio', 'local'],
      },
      tagNames: [
        ...(defaultSchema.tagNames || []),
        'audio', 'source', 'video',
        // KaTeX / MathML tags
        'span', 'math', 'annotation', 'semantics', 'mrow', 'mi', 'mo', 'mn', 'ms', 'mtext',
        // Mermaid / SVG tags
        'svg', 'g', 'path', 'rect', 'circle', 'ellipse', 'line', 'polyline', 'polygon', 'text', 'tspan', 'marker', 'defs', 'style'
      ],
    })
    // rehypeKatex produces already sanitized HTML, keep after sanitize to avoid stripping classes
    .use(rehypePrism, { 
        showLineNumbers: true,
        ignoreMissing: true
     })
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(markdown)

  return {
    html: String(file),
    toc: toc,
  }
}

/**
 * 渲染一个简化的、行内的Markdown字符串，主要用于弹窗等场景。
 * 不会处理TOC、代码高亮等复杂功能。
 *
 * @param markdown - Markdown原文.
 * @returns 渲染后的HTML字符串.
 */
export async function renderInlineMarkdown(markdown: string): Promise<string> {
    const file = await unified()
      .use(remarkParse)
      .use(remarkGfm)
      // 移除 remarkMarkmap 使用
      // .use(remarkMarkmap)
      .use(remarkRehype, { allowDangerousHtml: true })
      .use(rehypeSanitize, {
        ...defaultSchema,
        attributes: {
            ...defaultSchema.attributes,
            a: ['href', 'target', 'rel'],
        }
      })
      .use(rehypeStringify)
      .process(markdown);
  
    return String(file);
}// Force HMR update

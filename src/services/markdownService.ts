import { unified } from 'unified'
import remarkParse from 'remark-parse'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import rehypeSanitize, { defaultSchema } from 'rehype-sanitize'
import rehypeStringify from 'rehype-stringify'
import rehypeSlug from 'rehype-slug'
import rehypePrism from 'rehype-prism-plus'
import { visit } from 'unist-util-visit'
import type { Element, Root as HastRoot } from 'hast'
import { invoke } from '@tauri-apps/api/core'
import { getCachedAudioUrl, setCachedAudioUrl } from '../utils/audioCache'
import remarkMath from 'remark-math'
import rehypeKatex from 'rehype-katex'
import emoji from 'remark-emoji';
// 移除 remarkMarkmap 导入
// import remarkMarkmap from 'remark-markmap'

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
 * 异步自定义Rehype插件，用于处理 audio:// 协议的音频
 */
function rehypeLocalAudio() {
  return async (tree: HastRoot) => {
    const nodesToProcess: { node: Element, audioId: string }[] = []

    visit(tree, 'element', (node: Element) => {
      if (
        node.tagName === 'source' &&
        typeof node.properties?.src === 'string' &&
        node.properties.src.startsWith('audio://')
      ) {
        const audioId = node.properties.src.replace('audio://', '')
        nodesToProcess.push({ node, audioId })
      }
    })

    for (const { node, audioId } of nodesToProcess) {
      let objectUrl = getCachedAudioUrl(audioId)

      if (!objectUrl) {
        try {
          const audioData: { audio_data: string; file_format: string } = await invoke('get_audio_file', { audioId })
          if (audioData && audioData.audio_data) {
            const format = audioData.file_format || 'webm'
            const byteCharacters = atob(audioData.audio_data)
            const byteNumbers = new Array(byteCharacters.length)
            for (let i = 0; i < byteCharacters.length; i++) {
              byteNumbers[i] = byteCharacters.charCodeAt(i)
            }
            const byteArray = new Uint8Array(byteNumbers)
            const blob = new Blob([byteArray], { type: `audio/${format}` })
            objectUrl = URL.createObjectURL(blob)
            setCachedAudioUrl(audioId, objectUrl)
          }
        } catch (error) {
          console.error(`Failed to load audio data for ID ${audioId}:`, error)
          objectUrl = '' // or a placeholder URL
        }
      }
      
      if (node.properties) {
        node.properties.src = objectUrl
      }
    }
  }
}


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
    .use(rehypeSlug)
    .use(emoji)
    .use(rehypeToc, toc)
    .use(rehypeKatex)
    .use(rehypeSanitize, {
      ...defaultSchema,
      attributes: {
        ...defaultSchema.attributes,
        '*': [...(defaultSchema.attributes?.['*'] || []), 'class', 'style', 'data-enhanced', 'data-highlighted', 'loading'],
        code: [...(defaultSchema.attributes?.code || []), 'className'],
        source: ['src', 'type'],
        audio: ['controls', 'src'],
        video: ['controls', 'src', 'width', 'height', 'style', 'autoplay', 'loop', 'muted', 'poster'],
        img: [...(defaultSchema.attributes?.img || []), 'loading'],
      },
      protocols: {
        ...defaultSchema.protocols,
        src: [...((defaultSchema as any).protocols?.src || ['http', 'https', 'mailto']), 'data', 'audio', 'local'],
      },
      tagNames: [
        ...(defaultSchema.tagNames || []),
        'audio', 'source', 'video',
        // KaTeX / MathML tags
        'span', 'math', 'annotation', 'semantics', 'mrow', 'mi', 'mo', 'mn', 'ms', 'mtext'
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
}
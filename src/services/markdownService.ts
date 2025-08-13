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
import rehypeMermaid from 'rehype-mermaid'
import rehypeRaw from 'rehype-raw'

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
 * 规范化代码块语言，避免未知语言导致高亮报错
 * 将 language-markmap 等未知语言重写为 language-text
 */
function rehypeNormalizeCodeLanguage() {
  return (tree: HastRoot) => {
    visit(tree, 'element', (node: Element) => {
      if (node.tagName === 'code' && node.properties) {
        const className = node.properties.className as unknown
        const classes: string[] = Array.isArray(className)
          ? (className as string[])
          : typeof className === 'string'
            ? [className]
            : []

        if (classes.some(cls => /language-markmap/i.test(cls))) {
          node.properties.className = classes
            .map(cls => /language-markmap/i.test(cls) ? 'language-text' : cls)
        }
      }
    })
  }
}


/**
 * 将自定义的 <think> 标签转换为可折叠的 details/summary 结构
 * 用于隐藏模型的思考过程，避免干扰主要回复内容
 */
function rehypeTransformThinkTag() {
  return (tree: HastRoot) => {
    visit(tree, 'element', (node: Element) => {
      if (node.tagName === 'think') {
        const originalChildren = Array.isArray(node.children) ? node.children : []

        // 用 <details class="think-block"> 包裹，并提供一个 summary 说明
        const transformed: Element = {
          type: 'element',
          tagName: 'details',
          properties: { className: ['think-block'], open: true },
          children: [
            {
              type: 'element',
              tagName: 'summary',
              properties: {},
              children: [
                { type: 'text', value: '🤔 已深度思考（点击收起）' }
              ]
            },
            ...originalChildren
          ]
        }

        // 将当前节点替换为转换后的节点
        Object.assign(node, transformed)
      }
    })
  }
}

/**
 * 在进入 unified 流水线之前，对 <think>...</think> 进行稳健的字符串级替换。
 * 使用跨行正则，包裹为 <details><summary>...</summary><pre class="think-content">...</pre></details>
 * 支持流式渲染中的不完整think标签
 */
function preprocessThinkBlocks(input: string): string {
  if (!input) return ''
  
  // 处理完整的think标签
  let result = input.replace(/<think\b[^>]*>([\s\S]*?)<\/think>/gi, (_m, inner: string) => {
    return `<details class="think-block" open><summary>🤔 已深度思考（点击收起）</summary><pre class="think-content">${inner}</pre></details>`
  })
  
  // 处理不完整的think标签（用于流式渲染）
  result = result.replace(/<think\b[^>]*>([\s\S]*?)$/gi, (_m, inner: string) => {
    return `<details class="think-block" open><summary>🤔 正在深度思考...</summary><pre class="think-content">${inner}</pre></details>`
  })
  
  return result
}

/**
 * 自定义Rehype插件，用于处理 local:// 协议的图片
 * @param images - 笔记中的图片数据
 */
function rehypeLocalImages(images: Record<string, string>) {
  return (tree: HastRoot) => {
    visit(tree, 'element', (node: Element) => {
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
  // 在进入解析前，将 <think> 标签替换为可折叠的 details/summary 结构
  const preprocessed = preprocessThinkBlocks(markdown || '')

  const file = await unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkMath)
    .use(remarkRehype, { allowDangerousHtml: true, math: true })
    // 解析 Markdown 中的原始 HTML（例如预处理产生的 <details>）
    .use(rehypeRaw)
    .use(rehypeTransformThinkTag)
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
        // 允许保留通用属性及 id，否则标题上的 id 会被移除，目录无法定位
        '*': [
          ...((defaultSchema.attributes?.['*'] as any[]) || []),
          'id',
          'class',
          'style',
          'data-enhanced',
          'data-highlighted',
          'loading'
        ],
        code: [...(defaultSchema.attributes?.code || []), 'className'],
        source: ['src', 'type'],
        audio: ['controls', 'src'],
        img: [...(defaultSchema.attributes?.img || []), 'loading'],
      },
      protocols: {
        ...defaultSchema.protocols,
        src: [...((defaultSchema as any).protocols?.src || ['http', 'https', 'mailto']), 'data', 'audio', 'local'],
      },
      tagNames: [
        ...(defaultSchema.tagNames || []),
        'audio', 'source', 'details', 'summary',
        // KaTeX / MathML tags
        'span', 'math', 'annotation', 'semantics', 'mrow', 'mi', 'mo', 'mn', 'ms', 'mtext'
      ],
    })
    .use(rehypeNormalizeCodeLanguage)
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
    // rehypeKatex produces already sanitized HTML, keep after sanitize to avoid stripping classes
    .use(rehypePrism, { 
        showLineNumbers: true,
        ignoreMissing: true
     })
    .use(rehypeStringify, { allowDangerousHtml: true })
    .process(preprocessed)

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
    const preprocessed = preprocessThinkBlocks(markdown || '')
    // console.log(`[renderInlineMarkdown] Original: "${markdown}"`);
    // console.log(`[renderInlineMarkdown] Preprocessed: "${preprocessed}"`);

    const file = await unified()
      .use(remarkParse)
      .use(remarkGfm)
      // 移除 remarkMarkmap 使用
      // .use(remarkMarkmap)
      .use(remarkRehype, { allowDangerousHtml: true })
      // 解析 Markdown 中的原始 HTML（例如预处理产生的 <details>）
      .use(rehypeRaw)
      .use(rehypeTransformThinkTag)
      .use(rehypeSanitize, {
        ...defaultSchema,
        tagNames: [
          ...(defaultSchema.tagNames || []),
          'details', 'summary', 'pre'
        ],
        attributes: {
            ...defaultSchema.attributes,
            '*': [
              ...((defaultSchema.attributes?.['*'] as any[]) || []),
              'class'
            ],
            a: ['href', 'target', 'rel'],
            details: ['class'],
            summary: [],
            pre: ['class']
        }
      })
      .use(rehypeStringify, { allowDangerousHtml: true })
      .process(preprocessed);
  
    const result = String(file);
    // console.log(`[renderInlineMarkdown] Final result: "${result}"`);
    return result;
} 
<template>
  <div class="flex-1 flex overflow-hidden relative">
    <div ref="editorContainer" class="p-4 overflow-auto w-full" @wheel="setUserScrollingPane('editor')" @scroll.passive="emitEditorScroll" @click="handleEditorContainerClick">
      <EditorContent :editor="(editor as any)" class="prose max-w-none" />
    </div>
    <!-- Render preview content from parent when not in edit-only mode -->
    <slot name="preview" v-if="mode !== 'editOnly'" />
  </div>
  
</template>

<script setup lang="ts">
import { onBeforeUnmount, ref, watch, nextTick, onMounted } from 'vue'
import { EditorContent, useEditor } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import Link from '@tiptap/extension-link'
import Underline from '@tiptap/extension-underline'
import Strike from '@tiptap/extension-strike'
import Highlight from '@tiptap/extension-highlight'
// Lists & Tasks
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'

// Tables
import { Table } from '@tiptap/extension-table'
import TableRow from '@tiptap/extension-table-row'
import TableHeader from '@tiptap/extension-table-header'
import TableCell from '@tiptap/extension-table-cell'
// Code highlighting inside editor
import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight'
// Import lowlight with common languages
import { createLowlight, common } from 'lowlight'

// Create lowlight instance with common languages
const lowlight = createLowlight(common)
// Images
import Image from '@tiptap/extension-image'
// Custom image extension to handle local:// protocol
const CustomImage = Image.extend({
  renderHTML({ HTMLAttributes }: { HTMLAttributes: Record<string, any> }) {
    return ['img', { ...HTMLAttributes }]
  },
  addOptions() {
    return {
      ...(super.addOptions?.() || {}),
      allowBase64: true,
    }
  },
})
// Optional Markdown round-trip (if available in deps)
// @ts-ignore - optional dependency; guarded at runtime
import { Markdown } from 'tiptap-markdown'
import { invoke } from '@tauri-apps/api/core'

// Note: Markdown import/export will be wired in follow-ups. For now, treat content as plain text.

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  noteId: {
    type: String,
    required: true,
  },
  images: {
    type: Object as () => Record<string, string>,
    default: () => ({}),
  },
  placeholder: {
    type: String,
    default: 'Start typing…',
  },
  mode: {
    type: String as () => 'editOnly' | 'preview' | 'split',
    default: 'split',
  },
})

const emit = defineEmits(['update:modelValue', 'editor-scroll', 'preview-scroll', 'contextmenu', 'paste', 'keydown', 'add-image'])

const editor = useEditor({
  extensions: [
    StarterKit.configure({
      strike: false,  // 移除strike，因为我们会手动添加
      codeBlock: false,  // 移除codeBlock，使用CodeBlockLowlight替代
    }),
    Placeholder.configure({ placeholder: (props.placeholder as string) || 'Start typing…' }),
    Link,
    Underline,
    Strike,
    Highlight,
    TaskList,
    TaskItem.configure({ nested: true }),
    Table.configure({ resizable: true }),
    TableRow,
    TableHeader,
    TableCell,
    CodeBlockLowlight.configure({ lowlight }),
    CustomImage,
    // Markdown is optional; guard by try/catch in runtime usage
    // @ts-ignore
    ...(typeof Markdown !== 'undefined' ? [Markdown.configure({
      html: true,
      tightLists: true,
      bulletListMarker: '-',
      linkify: true,
      breaks: true,
    })] : []),
  ],
  content: (typeof window !== 'undefined' ? props.modelValue : ''),
  editorProps: {
    attributes: {
      class: 'prose max-w-none focus:outline-none',
    },
    handleDOMEvents: {
      // contextmenu: (_view: any, event: any) => {
      //   emit('contextmenu', event as unknown as MouseEvent)
      //   return false
      // },
      paste: (_view: any, event: any) => {
        // Handle image paste locally for Tiptap
        const e = event as ClipboardEvent
        const items = e.clipboardData?.items
        if (items) {
          for (let i = 0; i < items.length; i++) {
            const item = items[i]
            if (item.type && item.type.indexOf('image') !== -1) {
              e.preventDefault()
              const file = item.getAsFile()
              if (!file) return true
              fileToBase64(file)
                .then((base64) => {
                  const imageId = `img_${Date.now()}_${Math.floor(Math.random() * 1000)}`
                  return invoke('save_tip_image', {
                    imageData: {
                      tip_id: props.noteId,
                      image_id: imageId,
                      image_data: base64,
                    },
                  }).then(() => ({ imageId, base64 }))
                })
                .then(({ imageId, base64 }) => {
                  emit('add-image', { id: imageId, data: base64 })
                  editor?.value?.chain().focus().insertContent(`![图片](local://${imageId})`).run()
                })
                .catch((err) => {
                  console.error('Paste image failed:', err)
                })
              return true
            }
          }
        }
        // Fallback: let parent handle other paste types
        emit('paste', event as unknown as ClipboardEvent)
        return false
      },
      keydown: (_view: any, event: any) => {
        emit('keydown', event as unknown as KeyboardEvent)
        return false
      },
    },
  },
  onUpdate: ({ editor }) => {
    // Try to export content in the best available format
    // Priority: Markdown (preserves image references) > HTML > Plain text
    
    // First, try to get the raw markdown from Markdown extension if available
    // The Markdown extension may store the source markdown somewhere
    try {
      // @ts-ignore
      if (editor.extensionManager?.extensions) {
        // @ts-ignore
        const markdownExt = editor.extensionManager.extensions.find((ext: any) => ext.name === 'markdown')
        if (markdownExt && markdownExt.options && markdownExt.options.tiptapMarkdown) {
          // This might have conversion functions
          console.warn('Found markdown extension, attempting conversion')
        }
      }
    } catch (e) {
      // Silent fail
    }
    
    // For now, use a hybrid approach:
    // 1. Get the editor content as JSON (preserves structure)
    // 2. Try to reconstruct markdown from it if possible, otherwise use HTML
    // 3. Fall back to HTML which at least preserves images and formatting
    
    try {
      const html = editor.getHTML()
      if (html && html !== '<p></p>') {
        // Check if the HTML contains local:// image references
        // If it does, it means the markdown was parsed correctly
        // Export as HTML since it preserves the local:// URLs
        lastEmittedValue = html  // Update lastEmittedValue to prevent watch from triggering
        emit('update:modelValue', html)
        return
      }
    } catch (e) {
      console.warn('Failed to export HTML:', e)
    }
    
    // Last resort: plain text (loses formatting and images!)
    const plainText = editor.getText()
    console.warn('Falling back to plain text export (warning: images will be lost)')
    lastEmittedValue = plainText  // Update lastEmittedValue to prevent watch from triggering
    emit('update:modelValue', plainText)
  },
})
const editorContainer = ref<HTMLElement | null>(null)
// preview is handled by parent; this component focuses on the editor only
// no preview element here

const userScrollingPane = ref<'editor' | 'preview' | null>(null)
let scrollEndTimer: number | null = null

function setUserScrollingPane(pane: 'editor' | 'preview') {
  userScrollingPane.value = pane
  if (scrollEndTimer) window.clearTimeout(scrollEndTimer)
  scrollEndTimer = window.setTimeout(() => (userScrollingPane.value = null), 200)
}

function handleEditorContainerClick(e: MouseEvent) {
  // If the click target is not the editor content itself, focus the editor
  const target = e.target as HTMLElement
  if (target === editorContainer.value || target.classList.contains('prose')) {
    editor?.value?.commands.focus()
  }
}

function emitEditorScroll(_e: Event) {
  if (userScrollingPane.value !== 'editor') return
  const el = editorContainer.value
  if (!el) return
  if (el.scrollHeight <= el.clientHeight) return
  const ratio = el.scrollTop / (el.scrollHeight - el.clientHeight)
  emit('editor-scroll', ratio)
}

// no-op: useEditor handles mount

// Watch for external model value changes (not from our own updates)
let lastEmittedValue = props.modelValue
watch(
  () => props.modelValue,
  (val) => {
    if (!editor?.value) return
    // Skip if this is an echo of our own emit
    if (val === lastEmittedValue) return
    
    // Save cursor position before content update
    const currentPos = editor.value.state.selection.$anchor.pos
    
    // If markdown support exists, use it; otherwise fallback to plain content
    // @ts-ignore
    if (editor.value.commands.setMarkdown && typeof editor.value.commands.setMarkdown === 'function') {
      // @ts-ignore
      editor.value.commands.setMarkdown(val || '')
    } else {
      const current = editor.value.getText()
      // Use proper setContent API
      if (val !== current) {
        editor.value.commands.setContent(val || '')
      }
    }
    
    // Restore cursor position if possible
    nextTick(() => {
      if (editor.value && currentPos) {
        try {
          const newDoc = editor.value.state.doc
          const clampedPos = Math.min(currentPos, newDoc.content.size)
          editor.value.commands.setTextSelection(clampedPos)
        } catch (e) {
          // Selection may be invalid, just focus at end
          editor.value.commands.focus('end')
        }
      }
    })
  }
)

function updateImages() {
    if (!editorContainer.value) return
    
    // Replace local:// URLs with actual base64 images in the editor
    const imgElements = editorContainer.value.querySelectorAll('img')
    imgElements.forEach((img: Element) => {
      const imgEl = img as HTMLImageElement
      const src = imgEl.getAttribute('src')
      if (!src || !src.startsWith('local://')) return
      
      const imageId = src.replace('local://', '')
      if (props.images && props.images[imageId]) {
        imgEl.src = props.images[imageId]
        console.log(`Replaced image URL for ${imageId}`)
      } else {
        console.warn(`Image ${imageId} not found in images map`)
      }
    })
}

// Watch for image prop changes and update editor images
watch(
  () => props.images,
  updateImages,
  { deep: true }
)

onMounted(() => {
  // Try to update images after mount and content render
  setTimeout(updateImages, 100)
  setTimeout(updateImages, 1000) // Retry a bit later just in case
})

onBeforeUnmount(() => {
  editor?.value?.destroy()
})

// Expose minimal API for upstream integration
function focus() {
  editor?.value?.commands.focus()
}
function insertAtCursor(text: string) {
  editor?.value?.commands.insertContent(text)
}
function getSelectedText(): string {
  const sel = editor?.value?.state.selection
  if (!editor?.value || !sel) return ''
  return editor.value.state.doc.textBetween(sel.from, sel.to, '\n')
}
function scrollToRatio(ratio: number) {
  const el = editorContainer.value
  if (!el) return
  const max = Math.max(0, el.scrollHeight - el.clientHeight)
  el.scrollTop = Math.max(0, Math.min(max, ratio * max))
}

// Toolbar command bridge
function execToolbar(command: string, payload?: any) {
  const ed = editor?.value
  if (!ed) return
  const chain = ed.chain().focus()
  switch (command) {
    case 'undo':
      ed.commands.undo(); break
    case 'redo':
      ed.commands.redo(); break
    case 'bold':
      chain.toggleBold().run(); break
    case 'italic':
      chain.toggleItalic().run(); break
    case 'strike':
      chain.toggleStrike().run(); break
    case 'underline':
      chain.toggleUnderline().run(); break
    case 'code':
      chain.toggleCode().run(); break
    case 'toggleHighlight':
      chain.toggleHighlight().run(); break
    case 'heading': {
      const level = payload?.level ?? 1
      chain.toggleHeading({ level }).run(); break
    }
    case 'paragraph':
      chain.setParagraph().run(); break
    case 'blockquote':
      chain.toggleBlockquote().run(); break
    case 'codeblock':
      chain.toggleCodeBlock().run(); break
    case 'bulletList':
      chain.toggleBulletList().run(); break
    case 'orderedList':
      chain.toggleOrderedList().run(); break
    case 'taskList':
      chain.toggleTaskList().run(); break
    case 'table': {
      const rows = payload?.rows ?? 3
      const cols = payload?.cols ?? 3
      // @ts-ignore
      chain.insertTable({ rows, cols, withHeaderRow: true }).run(); break
    }
    case 'link': {
      const href = payload?.href
      if (!href) { chain.unsetLink().run(); break }
      chain.extendMarkRange('link').setLink({ href }).run(); break
    }
    default:
      // Fallback: insert content as text
      if (typeof payload === 'string') chain.insertContent(payload).run()
  }
}

function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as string)
    reader.onerror = (e) => reject(e)
    reader.readAsDataURL(file)
  })
}

defineExpose({ focus, insertAtCursor, getSelectedText, scrollToRatio, execToolbar })
</script>
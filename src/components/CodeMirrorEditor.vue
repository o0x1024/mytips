<template>
  <div class="flex-1 flex overflow-hidden relative">

    <!-- CodeMirror编辑器 -->
    <div v-if="!isPreviewMode || isSplitMode"
      ref="editorContainer"
      class="h-full overflow-hidden"
      :class="isSplitMode ? 'editor-split' : 'flex-1'"
    ></div>

    <!-- Markdown预览 -->
    <div v-if="isPreviewMode || isSplitMode"
      ref="previewDiv"
      class="p-4 prose dark:prose-invert max-w-none"
      :class="{
        'preview-split overflow-auto': isSplitMode,
        'flex-1 overflow-auto': !isSplitMode,
        'markdown-body': true,
        'markdown-preview': true
      }"
      @scroll="handlePreviewScroll"
      v-html="renderedContent"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, watch, onMounted, onBeforeUnmount } from 'vue';

import { EditorView, keymap, drawSelection, highlightActiveLine } from '@codemirror/view';
import { EditorState, Compartment } from '@codemirror/state';
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { markdown } from '@codemirror/lang-markdown';
import { oneDark } from '@codemirror/theme-one-dark';
import { syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
import { search, searchKeymap } from '@codemirror/search';
import { open } from '@tauri-apps/plugin-shell';



const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  renderedContent: {
    type: String,
    required: true,
  },
  isSplitMode: {
    type: Boolean,
    default: true,
  },
  isPreviewMode: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'contextmenu', 'paste', 'keydown', 'blur', 'ready']);

const editorContainer = ref<HTMLDivElement | null>(null);
const previewDiv = ref<HTMLDivElement | null>(null);
let editorView: EditorView | null = null;

// 滚动同步相关状态
let isScrollingEditor = false;
let isScrollingPreview = false;
let scrollSyncTimeout: number | null = null;
const SCROLL_SYNC_DELAY = 100; // 防抖延迟

// 主题配置
const themeCompartment = new Compartment();

// 滚动同步函数
const calculateScrollRatio = (element: HTMLElement): number => {
  const { scrollTop, scrollHeight, clientHeight } = element;
  const maxScroll = scrollHeight - clientHeight;
  return maxScroll > 0 ? scrollTop / maxScroll : 0;
};

const setScrollRatio = (element: HTMLElement, ratio: number) => {
  const { scrollHeight, clientHeight } = element;
  const maxScroll = scrollHeight - clientHeight;
  const targetScrollTop = Math.max(0, Math.min(maxScroll, ratio * maxScroll));
  
  // 使用requestAnimationFrame提供更平滑的滚动
  requestAnimationFrame(() => {
    element.scrollTop = targetScrollTop;
  });
};

// 编辑器滚动处理
const handleEditorScroll = () => {
  if (isScrollingPreview || !props.isSplitMode || !editorView || !previewDiv.value) return;
  
  isScrollingEditor = true;
  
  // 清除之前的定时器
  if (scrollSyncTimeout) {
    clearTimeout(scrollSyncTimeout);
  }
  
  // 获取编辑器滚动容器
  const editorScroller = editorView.scrollDOM;
  const scrollRatio = calculateScrollRatio(editorScroller);
  
  // 同步到预览区
  setScrollRatio(previewDiv.value, scrollRatio);
  
  // 设置防抖定时器
  scrollSyncTimeout = window.setTimeout(() => {
    isScrollingEditor = false;
  }, SCROLL_SYNC_DELAY);
};

// 预览区滚动处理
const handlePreviewScroll = () => {
  if (isScrollingEditor || !props.isSplitMode || !editorView || !previewDiv.value) return;
  
  isScrollingPreview = true;
  
  // 清除之前的定时器
  if (scrollSyncTimeout) {
    clearTimeout(scrollSyncTimeout);
  }
  
  const scrollRatio = calculateScrollRatio(previewDiv.value);
  
  // 同步到编辑器
  const editorScroller = editorView.scrollDOM;
  setScrollRatio(editorScroller, scrollRatio);
  
  // 设置防抖定时器
  scrollSyncTimeout = window.setTimeout(() => {
    isScrollingPreview = false;
  }, SCROLL_SYNC_DELAY);
};

// 初始化CodeMirror编辑器
const initEditor = () => {
  if (!editorContainer.value) return;

  const isDark = document.documentElement.classList.contains('dark');
  
  const state = EditorState.create({
    doc: props.modelValue,
    extensions: [
      // 移除 lineNumbers() 以不显示行号
      highlightActiveLine(),
      drawSelection(),
      history(),
      EditorView.lineWrapping, // 启用自动换行
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        ...searchKeymap
      ]),
      search(),
      markdown(),
      syntaxHighlighting(defaultHighlightStyle),
      themeCompartment.of(isDark ? oneDark : []),
      EditorView.theme({
        '&': {
          fontSize: 'var(--base-font-size)',
          fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Consolas, "Liberation Mono", Menlo, monospace',
          height: '100%',
        },
        '.cm-content': {
          padding: '16px',
          minHeight: '100%',
        },
        '.cm-focused': {
          outline: 'none',
        },
        '.cm-editor': {
          height: '100%',
          overflow: 'hidden',
        },
        '.cm-scroller': {
          fontFamily: 'inherit',
          overflow: 'auto',
          height: '100%',
        },
      }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          const newValue = update.state.doc.toString();
          emit('update:modelValue', newValue);
        }
      }),
      EditorView.domEventHandlers({
        contextmenu: (event) => {
          event.preventDefault();
          emit('contextmenu', event);
        },
        paste: (event) => {
          emit('paste', event);
        },
        blur: (event) => {
          emit('blur', event);
        },
      }),
    ],
  });

  editorView = new EditorView({
    state,
    parent: editorContainer.value,
  });

  // 确保 editorView 在下一个 tick 中可用，然后发出 ready 事件
  nextTick(() => {
    emit('ready');
  });
};

// 暴露 editorView 和 previewDiv 给父组件
defineExpose({
  get editorView() { return editorView; },
  previewDiv
});

// 更新主题
const updateTheme = () => {
  if (!editorView) return;
  
  const isDark = document.documentElement.classList.contains('dark');
  editorView.dispatch({
    effects: themeCompartment.reconfigure(isDark ? oneDark : [])
  });
  // 主题变化后重新测量布局
  nextTick(() => {
    if (editorView) {
      editorView.requestMeasure();
    }
  });
};


// 监听props变化
watch(() => props.modelValue, (newValue) => {
  if (editorView && editorView.state.doc.toString() !== newValue) {
    editorView.dispatch({
      changes: { from: 0, to: editorView.state.doc.length, insert: newValue }
    });
  }
});

// 监听编辑器容器的可见性变化
watch(() => [props.isPreviewMode, props.isSplitMode], () => {
  nextTick(() => {
    if (editorContainer.value && (!props.isPreviewMode || props.isSplitMode)) {
      // 如果编辑器应该显示但当前没有实例，重新初始化
      if (!editorView) {
        initEditor();
        // 为新创建的编辑器添加滚动事件监听器
        nextTick(() => {
          if (props.isSplitMode && editorView) {
            const editorScroller = editorView.scrollDOM;
            editorScroller.addEventListener('scroll', handleEditorScroll);
          }
        });
      } else {
        // 如果编辑器存在但可能因为容器变化而需要重新布局
        editorView.requestMeasure();
        // 确保编辑器内容与props同步
        if (editorView.state.doc.toString() !== props.modelValue) {
          editorView.dispatch({
            changes: { from: 0, to: editorView.state.doc.length, insert: props.modelValue }
          });
        }
        
        // 根据分屏模式状态管理滚动事件监听器
        const editorScroller = editorView.scrollDOM;
        if (props.isSplitMode) {
          // 移除旧的监听器（避免重复绑定）
          editorScroller.removeEventListener('scroll', handleEditorScroll);
          // 添加新的监听器
          editorScroller.addEventListener('scroll', handleEditorScroll);
        } else {
          // 非分屏模式下移除滚动监听器
          editorScroller.removeEventListener('scroll', handleEditorScroll);
        }
      }
    } else if (props.isPreviewMode && !props.isSplitMode && editorView) {
      // 在纯预览模式下，先清理滚动事件监听器，然后销毁编辑器实例
      const editorScroller = editorView.scrollDOM;
      editorScroller.removeEventListener('scroll', handleEditorScroll);
      editorView.destroy();
      editorView = null;
    }
    
    // 清理滚动同步状态
    if (!props.isSplitMode) {
      isScrollingEditor = false;
      isScrollingPreview = false;
      if (scrollSyncTimeout) {
        clearTimeout(scrollSyncTimeout);
        scrollSyncTimeout = null;
      }
    }
    
    // 当预览模式变化时，重新绑定预览区域的事件监听器
    if ((props.isPreviewMode || props.isSplitMode) && previewDiv.value) {
      // 先移除旧的监听器（如果存在）
      previewDiv.value.removeEventListener('click', handlePreviewClick);
      // 添加新的监听器
      previewDiv.value.addEventListener('click', handlePreviewClick);
    }
  });
}, { immediate: true });

// 监听渲染内容变化，重新绑定事件监听器
watch(() => props.renderedContent, () => {
  nextTick(() => {
    if ((props.isPreviewMode || props.isSplitMode) && previewDiv.value) {
      // 内容更新后重新绑定事件监听器
      previewDiv.value.removeEventListener('click', handlePreviewClick);
      previewDiv.value.addEventListener('click', handlePreviewClick);
    }
  });
});



// 监听主题变化
const observeThemeChange = () => {
  const observer = new MutationObserver((mutations) => {
    mutations.forEach((mutation) => {
      if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
        updateTheme();
      }
    });
  });
  
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['class']
  });
  
  return observer;
};

let themeObserver: MutationObserver | null = null;

// 处理预览区域的链接点击
const handlePreviewClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement;
  
  // 检查是否点击了链接
  if (target.tagName === 'A' || target.closest('a')) {
    const link = target.tagName === 'A' ? target as HTMLAnchorElement : target.closest('a') as HTMLAnchorElement;
    
    if (link && link.href) {
      // 检查是否是外部链接（http/https开头）
      if (link.href.startsWith('http://') || link.href.startsWith('https://')) {
        event.preventDefault();
        // 使用Tauri的shell插件在默认浏览器中打开链接
        open(link.href).catch((err: any) => {
          console.error('Failed to open link:', err);
        });
      }
      // 对于其他类型的链接（如锚点链接），保持默认行为
    }
  }
};

onMounted(() => {
  nextTick(() => {
    initEditor();
    // 确保编辑器正确测量布局
    if (editorView) {
      editorView.requestMeasure();
      
      // 为编辑器添加滚动事件监听器
      if (props.isSplitMode) {
        const editorScroller = editorView.scrollDOM;
        editorScroller.addEventListener('scroll', handleEditorScroll);
      }
    }
    
    // 为预览区域添加点击事件监听器
    if (previewDiv.value) {
      previewDiv.value.addEventListener('click', handlePreviewClick);
    }
  });
  themeObserver = observeThemeChange();
});

onBeforeUnmount(() => {
  // 清理滚动同步定时器
  if (scrollSyncTimeout) {
    clearTimeout(scrollSyncTimeout);
  }
  
  // 清理编辑器滚动事件监听器
  if (editorView) {
    const editorScroller = editorView.scrollDOM;
    editorScroller.removeEventListener('scroll', handleEditorScroll);
    editorView.destroy();
  }
  
  if (themeObserver) {
    themeObserver.disconnect();
  }
  
  // 清理预览区域的事件监听器
  if (previewDiv.value) {
    previewDiv.value.removeEventListener('click', handlePreviewClick);
  }
});
</script>

<style scoped>
/* 分屏模式下的固定宽度：编辑器55%，预览区45% */
.editor-split {
  width: 55% !important;
  flex: none !important;
  min-width: 0;
  overflow: hidden;
}

.preview-split {
  width: 45% !important;
  flex: none !important;
  min-width: 0;
  overflow: auto;
  word-wrap: break-word;
  word-break: break-word;
  /* white-space: pre-wrap; */
}

/* 确保预览区内容自动换行 */
.preview-split .prose {
  max-width: none !important;
  word-wrap: break-word;
  word-break: break-word;
  overflow-wrap: break-word;
}

.preview-split .prose * {
  max-width: 100% !important;
  word-wrap: break-word;
  word-break: break-word;
}
</style>
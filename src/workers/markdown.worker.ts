import { Marked, Renderer } from "marked";

// DOMPurify 在 Web Worker 中无法正常工作，因为缺少 `document` 对象。
// 因此在 Worker 里只负责把 Markdown 转为 HTML，
// 再交由主线程进行安全清洗。

self.onmessage = (event: MessageEvent<{ markdown: string; images?: Record<string, string> }>) => {
  const { markdown, images } = event.data;

  const marked = new Marked({
    pedantic: false,
    gfm: true,
    breaks: false,
  });

  // 如果有图片数据，则使用自定义渲染器
  if (images && Object.keys(images).length > 0) {
    const renderer = new Renderer();
    const originalImageRenderer = renderer.image.bind(renderer);

    // 按运行时 token 形式重写 image 渲染，以支持 local:// 协议
    renderer.image = (token: any): string => {
      const { href, title, text } = token ?? {};

      if (typeof href === 'string' && href.startsWith('local://')) {
        const imageId = href.substring('local://'.length);
        if (images && images[imageId]) {
          return `<img src="${images[imageId]}" alt="${text || ''}" ${title ? `title="${title}"` : ''} class="embedded-image" loading="lazy">`;
        }
        return `<span>[Image not found: ${href}]</span>`;
      }

      // 其他情况使用默认渲染器
      return (originalImageRenderer as any)(token);
    };
    marked.use({ renderer });
  }

  try {
    const rawHtml = marked.parse(markdown) as string;
    self.postMessage({ html: rawHtml });
  } catch (error) {
    self.postMessage({ error: (error as Error).message });
  }
}; 
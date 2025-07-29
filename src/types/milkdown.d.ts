// 覆盖 Milkdown 的类型定义，以解决类型错误
import { Ctx } from '@milkdown/core';

declare module '@milkdown/core' {
  interface Editor {
    config(fn: (ctx: Ctx) => void): Editor;
  }
} 
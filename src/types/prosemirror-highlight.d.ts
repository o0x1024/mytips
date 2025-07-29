declare module 'prosemirror-highlight' {
    import { Plugin } from 'prosemirror-state';
    import { Refractor } from 'refractor';
  
    export function highlightPlugin(refractor: Refractor): Plugin;
  
    export class HighlightPluginState {
      static get(state: any): any;
    }
} 
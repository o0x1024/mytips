<template>
  <div ref="editorRef" class="w-full h-full"></div>
</template>

<script setup lang="ts">
import 'prosemirror-view/style/prosemirror.css';
import { onMounted, onBeforeUnmount, ref, watch } from 'vue';
import { EditorState, TextSelection } from 'prosemirror-state';
import { EditorView } from 'prosemirror-view';
import { Schema, Node, Mark } from 'prosemirror-model';
import { schema as basicSchema } from "prosemirror-schema-basic";
import { history, undo, redo } from 'prosemirror-history';
import { keymap } from 'prosemirror-keymap';
import { baseKeymap, exitCode } from 'prosemirror-commands';
import { addListNodes } from "prosemirror-schema-list";
import { MarkdownParser, MarkdownSerializer, MarkdownSerializerState } from 'prosemirror-markdown';
import { inputRules, textblockTypeInputRule, wrappingInputRule } from 'prosemirror-inputrules';
import markdownit from 'markdown-it';

// CodeMirror imports...
import { EditorView as CodeMirrorView, keymap as cmKeymap, drawSelection } from "@codemirror/view"
import { Compartment } from "@codemirror/state"
import { oneDark } from "@codemirror/theme-one-dark"
import { javascript } from "@codemirror/lang-javascript"
import { css } from "@codemirror/lang-css"
import { html } from "@codemirror/lang-html"
import { markdown } from "@codemirror/lang-markdown"
import { rust } from "@codemirror/lang-rust"
import { python } from "@codemirror/lang-python"
import { java} from "@codemirror/lang-java"
import { cpp } from "@codemirror/lang-cpp"
import { sql } from "@codemirror/lang-sql"
import { defaultKeymap } from "@codemirror/commands"

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  images: {
    type: Object as () => Record<string, string>,
    default: () => ({})
  }
});

const emit = defineEmits(['update:modelValue']);

const editorRef = ref<HTMLDivElement | null>(null);
let view: EditorView | null = null;

const mySchema = new Schema({
    nodes: addListNodes(basicSchema.spec.nodes, "paragraph block*", "block").append({
        image: {
            inline: true,
            attrs: {
                src: { default: "" },
                alt: { default: null },
                title: { default: null }
            },
            group: "inline",
            draggable: true,
            parseDOM: [{
                tag: "img[src]",
                getAttrs(dom: HTMLElement) {
                    return {
                        src: dom.getAttribute("src"),
                        title: dom.getAttribute("title"),
                        alt: dom.getAttribute("alt")
                    };
                }
            }],
            toDOM(node: Node) {
                const { src, alt, title } = node.attrs;
                let displaySrc = src;
                if (src && src.startsWith('local://')) {
                    const imageId = src.replace('local://', '');
                    if (props.images && props.images[imageId]) {
                        displaySrc = props.images[imageId];
                    } else {
                        // You can set a placeholder for images that are not yet loaded
                        // displaySrc = '/placeholder-image.png';
                        console.warn(`Image with id "${imageId}" not found in provided images map.`);
                    }
                }
                return ["img", { src: displaySrc, alt, title, class: "inline-block" }];
            }
        },
        code_block: {
            content: "text*",
            marks: "",
            group: "block",
            code: true,
            defining: true,
            attrs: { language: { default: "" } },
            parseDOM: [{
                tag: "pre",
                preserveWhitespace: "full",
                getAttrs: (node: HTMLElement) => ({ language: node.getAttribute("data-language") || '' })
            }],
            toDOM(node: Node) {
                return ["pre", { "data-language": node.attrs.language }, ["code", 0]];
            }
        }
    }),
    marks: basicSchema.spec.marks
});


class CodeBlockView {
    node: Node;
    view: EditorView;
    getPos: () => number;
    cm: CodeMirrorView;
    updating: boolean;
    dom: HTMLElement;
    language: Compartment;

    constructor(node: Node, view: EditorView, getPos: () => number) {
        this.node = node;
        this.view = view;
        this.getPos = getPos;
        this.updating = false;
        this.language = new Compartment();

        const langExt = this.getLanguageExtension(node.attrs.language);

        this.cm = new CodeMirrorView({
            doc: this.node.textContent,
            extensions: [
                cmKeymap.of([
                    ...this.codeMirrorKeymap(),
                    ...defaultKeymap
                ]),
                drawSelection(),
                this.language.of(langExt),
                oneDark,
                CodeMirrorView.lineWrapping,
                CodeMirrorView.theme({
                    '&': {
                        fontSize: 'inherit',
                    },
                    '.cm-scroller': {
                        fontFamily: 'inherit',
                    }
                }),
                CodeMirrorView.updateListener.of(update => this.forwardUpdate(update))
            ]
        });
        
        this.dom = document.createElement("div")
        this.dom.className = "code-block-wrapper"

        const copyButton = document.createElement("button")
        copyButton.className = "copy-button"
        copyButton.textContent = "Copy"
        copyButton.addEventListener("click", () => {
            navigator.clipboard.writeText(this.cm.state.doc.toString()).then(() => {
                copyButton.textContent = "Copied!"
                setTimeout(() => {
                    copyButton.textContent = "Copy"
                }, 2000)
            }).catch(err => {
                console.error("Failed to copy text: ", err)
                copyButton.textContent = "Error"
                setTimeout(() => {
                    copyButton.textContent = "Copy"
                }, 2000)
            })
        })

        this.dom.appendChild(this.cm.dom)
        this.dom.appendChild(copyButton)
    }

    getLanguageExtension(language: string) {
        switch (language) {
            case 'javascript': return javascript();
            case 'css': return css();
            case 'html': return html();
            case 'markdown': return markdown();
            case 'rust': return rust();
            case 'python': return python();
            case 'java': return java();
            case 'cpp': return cpp();
            case 'sql': return sql();
            default: return javascript();
        }
    }

    forwardUpdate(update: any) {
        if (this.updating || !this.cm.hasFocus) return;

        const offset = this.getPos() + 1;
        const { main } = update.state.selection;
        const selFrom = offset + main.from;
        const selTo = offset + main.to;
        const pmSel = this.view.state.selection;

        if (update.docChanged || pmSel.from !== selFrom || pmSel.to !== selTo) {
            let tr = this.view.state.tr;
            update.changes.iterChanges((fromA: number, toA: number, _fromB: number, _toB: number, text: any) => {
                if (text.length)
                    tr.replaceWith(offset + fromA, offset + toA, mySchema.text(text.toString()));
                else
                    tr.delete(offset + fromA, offset + toA);
            });
            tr.setSelection(TextSelection.create(tr.doc, selFrom, selTo));
            this.view.dispatch(tr);
        }
    }

    update(node: Node) {
        if (node.type !== this.node.type) return false;
        this.node = node;

        if (this.updating) return true;

        const newText = node.textContent;
        const curText = this.cm.state.doc.toString();
        if (newText !== curText) {
            let start = 0, curEnd = curText.length, newEnd = newText.length;
            while (start < curEnd && curText.charCodeAt(start) === newText.charCodeAt(start)) {
                ++start;
            }
            while (curEnd > start && newEnd > start && curText.charCodeAt(curEnd - 1) === newText.charCodeAt(newEnd - 1)) {
                curEnd--;
                newEnd--;
            }
            this.updating = true;
            this.cm.dispatch({
                changes: { from: start, to: curEnd, insert: newText.slice(start, newEnd) }
            });
            this.updating = false;
        }
        return true;
    }

    setSelection(anchor: number, head: number) {
        this.cm.focus();
        this.updating = true;
        this.cm.dispatch({ selection: { anchor, head } });
        this.updating = false;
    }

    codeMirrorKeymap() {
        const view = this.view;
        return [
            { key: "Ctrl-Enter", run: () => { if (!exitCode(view.state, view.dispatch)) return false; view.focus(); return true; }},
            { key: "Ctrl-z", mac: "Cmd-z", run: () => undo(view.state, view.dispatch) },
            { key: "Shift-Ctrl-z", mac: "Shift-Cmd-z", run: () => redo(view.state, view.dispatch) },
            { key: "Ctrl-y", mac: "Cmd-y", run: () => redo(view.state, view.dispatch) }
        ];
    }
    
    destroy() { this.cm.destroy(); }
    selectNode() { this.cm.focus(); }
    stopEvent() { return true; }
}


const md = markdownit('commonmark', { html: false, linkify: true });

const parser = new MarkdownParser(mySchema, md, {
    blockquote: { block: "blockquote" },
    paragraph: { block: "paragraph" },
    list_item: { block: "list_item" },
    bullet_list: { block: "bullet_list" },
    ordered_list: { block: "ordered_list", getAttrs: (tok: any) => ({ order: +tok.attrGet('start') || 1 }) },
    heading: { block: "heading", getAttrs: (tok: any) => ({ level: +tok.tag.slice(1) }) },
    code_block: { block: "code_block", getAttrs: (tok: any) => ({ language: tok.info || '' }) },
    fence: {
        block: 'code_block',
        getAttrs: (tok: any) => ({ language: tok.info || '' }),
    },
    hr: { node: "horizontal_rule" },
    image: {
        node: "image",
        getAttrs(tok: any) {
            return {
                src: tok.attrGet('src'),
                title: tok.attrGet('title') || null,
                alt: tok.content,
            };
        }
    },
    hard_break: { node: "hard_break" },
    em: { mark: "em" },
    strong: { mark: "strong" },
    link: {
        mark: "link",
        getAttrs: (tok: any) => ({
            href: tok.attrGet('href'),
            title: tok.attrGet('title') || null,
        }),
    },
    code_inline: { mark: "code" },
});

function quote(str: string) {
    const wrap = str.includes('"') ? "'" : '"';
    return wrap + str + wrap;
}

const serializer = new MarkdownSerializer({
    blockquote(state: MarkdownSerializerState, node: Node) {
        state.wrapBlock("> ", null, node, () => state.renderContent(node));
    },
    code_block(state: MarkdownSerializerState, node: Node) {
        state.write("```" + (node.attrs.language || "") + "\n");
        state.text(node.textContent, false);
        state.ensureNewLine();
        state.write("```\n");
    },
    heading(state: MarkdownSerializerState, node: Node) {
        state.write(state.repeat("#", node.attrs.level) + " ");
        state.renderInline(node);
        state.closeBlock(node);
    },
    horizontal_rule(state: MarkdownSerializerState, node: Node) {
        state.write((node.attrs.markup as string) || "---");
        state.closeBlock(node);
    },
    bullet_list(state: MarkdownSerializerState, node: Node) {
        state.renderList(node, "  ", () => (node.attrs.bullet || "*") + " ");
    },
    ordered_list(state: MarkdownSerializerState, node: Node) {
        const start = node.attrs.order || 1;
        const maxW = String(start + node.childCount - 1).length;
        const space = state.repeat(" ", maxW + 2);
        state.renderList(node, space, (i: number) => {
            const nStr = String(start + i);
            return state.repeat(" ", maxW - nStr.length) + nStr + ". ";
        });
    },
    list_item(state: MarkdownSerializerState, node: Node) {
        state.renderContent(node);
    },
    paragraph(state: MarkdownSerializerState, node: Node) {
        state.renderInline(node);
        state.closeBlock(node);
    },
    image(state: MarkdownSerializerState, node: Node) {
        state.write("![" + state.esc(node.attrs.alt || "") + "](" + state.esc(node.attrs.src) +
            (node.attrs.title ? " " + quote(node.attrs.title) : "") + ")");
    },
    hard_break(state: MarkdownSerializerState, node: Node, parent: Node, index: number) {
        for (let i = index + 1; i < parent.childCount; i++)
            if (parent.child(i).type !== node.type) {
                state.write("\\\n");
                return;
            }
    },
    text(state: MarkdownSerializerState, node: Node) {
        state.text(node.text ?? '');
    }
}, {
    em: { open: "*", close: "*" },
    strong: { open: "**", close: "**" },
    link: {
        open: "[",
        close(state: MarkdownSerializerState, mark: Mark) {
            return "](" + state.esc(mark.attrs.href) + (mark.attrs.title ? " " + quote(mark.attrs.title) : "") + ")";
        }
    },
    code: { open: "`", close: "`" }
});


onMounted(() => {
    if (editorRef.value) {
        const state = EditorState.create({
            doc: parser.parse(props.modelValue) ?? undefined,
            plugins: [
                history(),
                keymap({ 'Mod-z': undo, 'Mod-y': redo }),
                inputRules({
                    rules: [
                        wrappingInputRule(/^```(\S*)\s$/, mySchema.nodes.code_block, match => ({ language: match[1] })),
                        wrappingInputRule(/^\s*>\s$/, mySchema.nodes.blockquote),
                        wrappingInputRule(/^\s*([-*+])\s$/, mySchema.nodes.bullet_list),
                        wrappingInputRule(/^(\d+)\.\s$/, mySchema.nodes.ordered_list, match => ({ order: +match[1] })),
                        textblockTypeInputRule(/^(#{1,6})\s$/, mySchema.nodes.heading, match => ({ level: match[1].length }))
                    ]
                }),
                keymap(baseKeymap),
            ],
            schema: mySchema
        });

        view = new EditorView(editorRef.value, {
            state,
            nodeViews: {
                code_block: (node, view, getPos) => new CodeBlockView(node, view, getPos as () => number),
            },
            attributes: {
                class: 'prose dark:prose-invert max-w-none w-full h-full focus:outline-none overflow-y-auto p-4',
            },
            dispatchTransaction(transaction) {
                if (!view) return;
                
                const newState = view.state.apply(transaction);
                view.updateState(newState);

                if (transaction.docChanged) {
                    const markdown = serializer.serialize(newState.doc);
                    if (props.modelValue !== markdown) {
                        emit('update:modelValue', markdown);
                    }
                }
            }
        });
    }
});

watch(() => props.images, () => {
    if (view) {
        // Force a re-render of the view to update image srcs by creating a new state
        const { state } = view;
        const newState = EditorState.create({
            doc: state.doc,
            plugins: state.plugins,
            schema: state.schema
        });
        view.updateState(newState);
    }
}, { deep: true });

watch(() => props.modelValue, (newValue) => {
  if (view && !view.composing) {
    const currentContentInEditor = serializer.serialize(view.state.doc);
    if (newValue === currentContentInEditor) {
      return;
    }

    const newDoc = parser.parse(newValue);
    if (newDoc && !newDoc.eq(view.state.doc)) {
        const newState = EditorState.create({
            doc: newDoc ?? undefined,
            plugins: view.state.plugins,
            schema: mySchema
        });
        view.updateState(newState);
    }
  }
});


onBeforeUnmount(() => {
  if (view) {
    view.destroy();
    view = null;
  }
});
</script>

<style>
.ProseMirror {
  outline: none;
  white-space: pre-wrap;
}

.ProseMirror p.is-editor-empty:first-child::before {
  content: attr(data-placeholder);
  float: left;
  color: #adb5bd;
  pointer-events: none;
  height: 0;
}

/* Wrapper for positioning the copy button */
.code-block-wrapper {
    position: relative;
    margin: 1rem 0;
}

/* Style the CodeMirror editor within ProseMirror */
.ProseMirror .cm-editor {
    /* Override the oneDark theme's border */
    border: none !important;
    border-radius: 8px;
    overflow: hidden; /* Ensures children conform to border-radius */
}

/* The copy button */
.copy-button {
    position: absolute;
    top: 8px;
    right: 8px;
    z-index: 10;
    background-color: #404754;
    color: #d0d8e8;
    border: none;
    padding: 4px 10px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 13px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    opacity: 0;
    transition: opacity 0.2s ease-in-out;
}

.code-block-wrapper:hover .copy-button {
    opacity: 1;
}

.copy-button:hover {
    background-color: #525a69;
}
</style> 
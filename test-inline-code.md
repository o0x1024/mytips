# 行内代码显示测试

## 测试不同的行内代码

1. 使用 `marked.Renderer` 自定义代码块渲染
2. 配置 `marked-highlight` 扩展进行语法高亮
3. 调用 `Prism.highlight()` 方法处理代码
4. 设置 `white-space: pre-wrap` 支持自动换行
5. 使用 `word-break: break-all` 强制换行

## 技术栈测试

- 前端框架：`Vue.js`
- 构建工具：`Vite`
- UI库：`DaisyUI`
- 语法高亮：`Prism.js`
- Markdown解析：`marked`

## 函数和方法测试

调用 `isPrismLanguageAvailable()` 函数检查语言支持，然后使用 `escapeHtml()` 转义HTML字符，最后通过 `DOMPurify.sanitize()` 清理内容。

## 配置选项测试

设置 `langPrefix: 'language-'` 和 `breaks: true` 选项，启用 `gfm: true` 模式。

## 长代码测试

超长的配置项：`this.is.a.very.long.configuration.option.that.should.wrap.properly.in.all.themes.including.dark.and.light.modes`

## 特殊字符测试

包含特殊字符的代码：`@Component({selector: 'app-test', template: '<div>{{value}}</div>'})` 
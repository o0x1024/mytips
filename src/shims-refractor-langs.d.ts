   // src/shims-refractor-langs.d.ts
   declare module 'refractor/lang/*' {
    import { RefractorSyntax } from 'refractor'
    const syntax: RefractorSyntax
    export default syntax
  }
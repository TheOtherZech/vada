# Vada Core

**TL;DR:** A collection of crates for parsing, interpreting, and evaluating Vada. It's the canonical Vada implementation that powers the language server, CLI, and even the python plugin.

> None of this works in a meaningful way. Vada's still in the 'experimental specification' stage, which means we're currently figuring out how it _should_ work. The fun bits will be here

Vada Core is intentionally siloed into separate crates, such that the language server and CLI can exist side-by-side without being overly enmeshed.

```Tree
ZDG  
├─ Crates 
│  ├─ ast       // Rowan
│  │  └─ ...
│  ├─ dl-core   // Garbage
│  │  └─ ...
│  ├─ hir       // Abstraction
│  │  └─ ...
│  ├─ lexer     // Logos Tokens
│  │  └─ ...
│  ├─ parser    // Grammar Here
│  │  └─ ...
│  ├─ syntax    // Glue layer between Logos and Rowan
│  │  └─ ...
│  └─ vada      // There really isn't much in this one. 
│     └─ ...
├─ cargo.toml
├─ README.md   // You Are Here
└─ ...
```

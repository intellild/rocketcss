# rocketcss_nano

`rocketcss_nano` walks compilation-owned flat rule and declaration stores and
applies normalization in place. Within one
declaration block, and across physically adjacent style rules with structurally
equal selectors, a shared single-pass declaration IR removes exact duplicates,
merges compatible physical margin/padding longhands, and folds simple longhand
overrides into an earlier shorthand. It also merges compatible `column-width`
and `column-count` declarations into `columns`. Adjacent blocks retain logical
source-order ranges until terminal compaction. Nested
content remains an ordering barrier. Fallback chains, logical-property barriers,
and declarations with different importance are preserved. Compact output
formatting is selected separately with
`rocketcss_codegen::PrinterOptions { prettify: false }`.

```rust
use rocketcss_common::GhostToken;
use rocketcss_codegen::{PrinterOptions, ToCss, ToCssContext};
use rocketcss_nano::{MinifyOptions, minify};
use rocketcss_parser::{Compiler, ParserOptions};

let mut compiler = Compiler::new();
let mut stylesheet = GhostToken::scope(|token| {
    compiler.parse(
        "a { width: 16px; margin: 1px 1px }",
        token,
        ParserOptions::default(),
    )
})?;

let (stats, css) = GhostToken::scope(|token| {
    let stats = minify(&mut stylesheet, token, MinifyOptions::default());
    let css = stylesheet.to_css_string(
        PrinterOptions { prettify: false },
        &ToCssContext::new(token),
    )?;
    Ok::<_, Box<dyn std::error::Error>>((stats, css))
})?;
assert_eq!(css, "a{width:1pc;margin:1px}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`MinifyPlugin` provides the same transform for a `rocketcss_visitor::Plugins`
pipeline and stores `MinifyStats` in the shared plugin context.

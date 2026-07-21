# rocketcss_nano

`rocketcss_nano` walks an arena-backed `rocketcss_ast::StyleSheet` and applies
normalization in place without allocating replacement AST nodes. Within one
declaration block, and across physically adjacent style rules with structurally
equal selectors, a shared single-pass declaration IR removes exact duplicates,
merges compatible physical margin/padding longhands, and folds simple longhand
overrides into an earlier shorthand. It also merges compatible `column-width`
and `column-count` declarations into `columns`. Adjacent blocks retain their
arena allocations and are serialized through a backward merge chain. Nested
content remains an ordering barrier. Fallback chains, logical-property barriers,
and declarations with different importance are preserved. Compact output
formatting is selected separately with
`rocketcss_codegen::PrinterOptions { minify: true }`.

```rust
use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_nano::{MinifyOptions, minify};
use rocketcss_parser::{ParserOptions, parse};

let allocator = Allocator::new();
let mut stylesheet = parse(
    "a { width: 16px; margin: 1px 1px }",
    &allocator,
    ParserOptions::default(),
)?;

let stats = minify(&mut stylesheet, MinifyOptions::default());
let css = stylesheet.to_css_string(PrinterOptions { minify: true })?;
assert_eq!(css, "a{width:1pc;margin:1px}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`MinifyPlugin` provides the same transform for a `rocketcss_visitor::Plugins`
pipeline and stores `MinifyStats` in the shared plugin context.

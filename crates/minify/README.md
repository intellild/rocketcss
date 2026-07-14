# rocketcss_minify

`rocketcss_minify` walks an arena-backed `rocketcss_ast::StyleSheet` and applies only
simple, node-local normalization in place. It does not merge or remove rules,
combine longhands, or allocate replacement AST nodes. It also removes exact
duplicate declarations within one declaration block, while preserving fallback
chains and declarations with different importance. Compact output formatting is
selected separately with `rocketcss_codegen::PrinterOptions { minify: true }`.

```rust
use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, minify};
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

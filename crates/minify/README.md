# rs_css_minify

`rs_css_minify` mutates an arena-backed `rs_css_ast::StyleSheet` in place. It
performs semantic minification passes; compact whitespace output is selected
separately with `rs_css_codegen::PrinterOptions { minify: true }`.

```rust
use rs_css_allocator::Allocator;
use rs_css_codegen::{PrinterOptions, ToCss};
use rs_css_minify::{MinifyOptions, minify};
use rs_css_parser::{ParserOptions, parse};

let allocator = Allocator::new();
let mut stylesheet = parse(
    "a { color: yellow; width: 16px }",
    &allocator,
    ParserOptions::default(),
)?;

let stats = minify(&mut stylesheet, &allocator, MinifyOptions::default());
let css = stylesheet.to_css_string(PrinterOptions { minify: true })?;
assert_eq!(css, "a{color:#ff0;width:1pc}");
# Ok::<(), Box<dyn std::error::Error>>(())
```

`MinifyPlugin` provides the same transform for a `rs_css_visitor::Plugins`
pipeline and stores `MinifyStats` in the shared plugin context.

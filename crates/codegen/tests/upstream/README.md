# Upstream AST-to-CSS coverage

The `upstream.rs` integration test reuses every one of the 4,223
Lightning CSS stylesheet inputs recorded in the parser corpus and compares
default, non-minified AST serialization against the pinned
`lightningcss 1.0.0-alpha.71` implementation.

The current rs-css parser deliberately leaves many property values as raw or
unparsed nodes, whereas Lightning CSS creates typed and normalized nodes. To
keep that parser representation difference out of the printer assertion, the
test reparses rs-css's output with Lightning CSS and compares the resulting
canonical CSS with the canonical CSS from the original upstream input. This
still catches lost or changed tokens, invalid output, escaping errors, and
other semantic serialization differences.

All 4,223 inputs are audited. Lightning CSS's default parser accepts 4,220 of
them, and every one is compared. The remaining three require upstream custom
at-rule parser behavior, so there is no default Lightning AST-to-CSS result to
compare.

`trait_coverage.rs` separately asserts at compile time that every public CSS
AST node (including `StyleSheet`) implements `ToCss`. `to_css.rs` ports the
direct Lightning CSS public API serialization assertions and adds focused
checks for the typed value printers that are not currently reachable through
the rs-css parser.

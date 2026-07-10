# Repository Guidelines

## AST-aligned feature structure

When implementing a feature whose modules or implementations correspond one-to-one with a large number of AST structures, keep that feature's directory and file structure aligned with the AST directory and file structure.

This applies to features such as parsers, visitors, code generators, transforms, and similar AST-wide functionality. For example, implementations for AST nodes defined in `crates/ast/src/color.rs` and `crates/ast/src/rules.rs` should live in corresponding `color.rs` and `rules.rs` modules within the feature crate.

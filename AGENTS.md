# Repository Guidelines

## AST-aligned feature structure

When implementing a feature whose modules or implementations correspond one-to-one with a large number of AST structures, keep that feature's directory and file structure aligned with the AST directory and file structure.

This applies to features such as parsers, visitors, code generators, transforms, and similar AST-wide functionality. For example, implementations for AST nodes defined in `crates/ast/src/color.rs` and `crates/ast/src/rules.rs` should live in corresponding `color.rs` and `rules.rs` modules within the feature crate.

Alignment applies to the owning parent module, not to a strict one-file limit. When the implementation corresponding to one AST file becomes too large, represent that file as a directory module in the feature crate and split the implementation into focused child modules. For example, implementations owned by `crates/ast/src/rules/stylesheet.rs` may live under `rules/stylesheet/mod.rs` with child modules such as `calc.rs`, `color.rs`, or `transform.rs`. Keep the parent path aligned with the AST; use child modules only to separate substantial implementation concerns, and do not treat them as peer AST modules.

## Known property AST coverage

Every statically known `PropertyId` variant must be generated from the same property metadata entry as a corresponding typed `Declaration` AST variant. Do not add ID-only known-property lists, hand-written known property variants, or parallel discriminant/name mappings. `Declaration::Unparsed` is only a lossless fallback when a known property's value cannot be represented by its typed parser, such as values containing variables; it must not substitute for a missing AST node. Dynamic `PropertyId::Custom` names and internal sentinel variants are exempt from this one-to-one requirement.

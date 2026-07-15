# Repository Guidelines

## AST-aligned modules

- AST-wide features such as parsers, visitors, code generators, and transforms must mirror the owning AST module path.
- For example, code for `crates/ast/src/color.rs` and `crates/ast/src/rules.rs` belongs in the feature's corresponding `color.rs` and `rules.rs` modules.
- If one implementation grows too large, turn its corresponding feature module into a directory and split it into focused child modules. Keep those children under the owning AST path; do not treat them as peer AST modules.

## Known property coverage

- Define each statically known property once in the property metadata. The same entry must generate its `PropertyId` and typed `Declaration` variants.
- Do not add ID-only property lists, hand-written known variants, or parallel name/discriminant mappings.
- Use `Declaration::Unparsed` only as a lossless fallback for values the typed parser cannot represent, such as variables—not as a substitute for a missing AST node.
- `PropertyId::Custom` and internal sentinel variants are exempt.

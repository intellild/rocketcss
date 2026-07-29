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

## Hash collections

- Use `rustc_hash::FxHashMap` and `rustc_hash::FxHashSet` for compiler-internal hash collections instead of the standard library's `HashMap` and `HashSet`.
- RocketCSS is a compiler pipeline, so these transient collections optimize for throughput and do not require a denial-of-service-resistant hasher.
- Keep a stronger hasher only when a collection crosses a security boundary or its collision resistance is part of the API contract, and document that exception.

## Formatting

- After modifying files, run the appropriate formatter before considering the work complete. For Rust changes, run `cargo fmt --all`; for other file types, use the repository-configured formatter when available.

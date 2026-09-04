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

## Lossless AST and serialization

- Typed AST nodes must preserve authored syntactic distinctions that plain parser-to-codegen output can reproduce, including keyword identity and aliases, component order, duplicate authored components, and whether optional syntax was written explicitly.
- AST simplifications must be information-preserving. Do not replace a source-bearing enum, ordered list, or explicit-presence marker with a canonical semantic representation when doing so changes codegen output without a transform.
- Parsing followed directly by codegen must not perform value normalization, deduplication, reordering, or shortening. Those changes belong in nano or another explicitly requested transform, which must update the AST before codegen.
- When changing a source-bearing AST shape, add a parser-to-codegen regression test with nano disabled, plus transform tests for any intended normalized output.

## Hash collections

- Use `rustc_hash::FxHashMap` and `rustc_hash::FxHashSet` for compiler-internal hash collections instead of the standard library's `HashMap` and `HashSet`.
- RocketCSS is a compiler pipeline, so these transient collections optimize for throughput and do not require a denial-of-service-resistant hasher.
- Keep a stronger hasher only when a collection crosses a security boundary or its collision resistance is part of the API contract, and document that exception.

## Formatting

- After modifying files, run the appropriate formatter before considering the work complete. For Rust changes, run `cargo fmt --all`; for other file types, use the repository-configured formatter when available.

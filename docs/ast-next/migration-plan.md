# Flat AST migration plan

Status after `b46e068` (`feat(ast): add flat AST and migrate value parsers`).

## Current state

- `crates/ast_next` provides the swc-next-style storage:
  - 4-byte `NodeId<'ast, T>` handles for every declared node.
  - `Ast` owns three SoA columns: spans, kinds, payloads.
  - 16-byte `NodePayload`: small nodes inline, larger nodes spilled to arenas.
  - Arena slices for list fields; `&'ast str` and `Atom` for strings.
  - Generated typed accessors, setters, `NodeStorage`, `*Kind`, and `alloc_*`.
  - Rule tree types: `StyleSheet`, `CssRule`, `DeclarationBlock`,
    `DeclarationPayload`, `CssRulePayload`, and `Compilation`.
- `crates/parser/src/next` contains all migrated value parsers:
  - length, color, box model, border, background, image, multicol, flex,
    font, svg, ui, text, mask, transform, alignment, animation.
  - `collect_tokens` produces `&'ast [TokenOrValue]` directly.
  - `Compiler` owns both the old compilation state and the new `Ast`.
- Nano is excluded from the workspace; minify benchmarks were removed so the
  parse → codegen path can be measured independently.
- Old parser and old AST remain the active `parse` path. The new modules are
  compiled and unit-tested in parallel.

## Remaining work

### 1. Selector parsing

Port `crates/parser/src/parser/selector.rs` into `parser/src/next/selector.rs`.

- `SelectorList`, `Selector`, `Component`, `Combinator`, attribute selectors,
  pseudo classes/elements, and `An+B` values.
- Return `SelectorList` node handles and arena slices.
- Keep pseudo-class/value enums plain where they contain no node references.
- Add selector round-trip tests (simple, compound, functional pseudo,
  namespace, attribute operators, nesting `&`).

### 2. Declaration/property layer

Port `crates/parser/src/parser/properties.rs` and the declaration path of
`parser/src/parser/values.rs`.

- Build `DeclarationPayload` (or a new `Declaration` node) from
  `PropertyId` plus the migrated value nodes.
- Port custom properties (`--x`) and unparsed declarations as
  `&'ast [TokenOrValue]`.
- Port CSS-wide keywords (`initial`, `inherit`, `unset`, `revert`,
  `revert-layer`) and `!important`.
- Port shorthand parsing and the property strategy table. `PropertyId` lookup
  already covers 366 known names; vendor-prefixed property handling still needs
  to be wired to the prefixed variants.
- Validate against the existing property fixtures before deleting old modules.

### 3. Rule and at-rule layer

Port `rules.rs`, `media.rs`, and the remaining `radix_ast` rule construction.

- Build `CssRule` nodes with `CssRulePayload` variants:
  style, media, supports, layer, container, scope, keyframes, page, property,
  font-face, counter-style, view-transition, nesting.
- Build `DeclarationBlock` slices and nested `children` slices.
- Port media query parsing (`media.rs`, 775 lines) to `MediaList` /
  `MediaQuery` nodes.
- Replace old parser-context/effective-key bookkeeping with direct tree
  construction. Source order is the arena allocation order; no dense rule
  store or radix insertion is required for parse → codegen.
- Port keyframe selectors, `@page` margin rules, and `@property` descriptors.

### 4. Parse entry cutover

- Make `Compiler::parse` return `rocketcss_ast_next::Compilation`.
- Remove the old `rocketcss_ast` dependency from `crates/parser`.
- Delete or archive `parser/src/parser/{radix_ast,rules,media,properties,...}`
  once parity tests pass.
- Thread real source spans: the migrated value parsers currently allocate with
  `DUMMY_SP`; capture token start/end positions and pass a real `Span` before
  codegen/source-map work is considered complete.
- Keep `ParserOptions.filename`, source map URL handling, and error recovery
  behavior identical.

### 5. Codegen migration

- Port `crates/codegen` writers to read through `Ast` accessors and `*Data`
  matches.
- Update every value/rule writer to the handle-based tree.
- Preserve lossless output for unparsed values and custom properties.
- Emit source maps from the span column.

### 6. Tests, CI, and performance

- Switch parser/upstream tests to the new `Compilation`.
- Run:
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets`
  - Prettier checks used by CI
- Compare `tasks/benchmark/benches/pipeline.rs` parse → codegen against the
  base branch (`main`/`master`) with the same inputs and iteration counts.
- Acceptance criteria:
  - all existing parser and upstream fixtures pass;
  - lossless round-trip for the fixture corpus;
  - no regression in parse-only throughput;
  - parse → codegen performance report attached to the final change.

## Known gaps to resolve during the remaining work

- `PropertyId::from_name` currently maps known names but does not yet split
  vendor prefixes onto the prefixed variants.
- `Token`, `Unit`, `PropertyId`, `AnimationName`, and other leaf enums are kept
  plain; any node/plain classification change must be reflected in the
  declaration files and the generator.
- Some hand-written helpers in `ast_next/src/impls_values.rs` duplicate old AST
  behavior (`KnownFunction`, `KnownColor`, `FontFamily`, `VendorPrefix`,
  `FunctionFlags`); add remaining helpers as codegen needs them.
- The old parser stays buildable until the new rule tree and codegen are wired,
  so both AST crates are present in the workspace.

## Completion report (2026-10-22)

The migration is complete. `Compiler::parse` now returns
`rocketcss_ast_next::Compilation`, and the old `rocketcss_ast` dependency has
been removed from the parser, codegen, visitor, and fixture-test crates.

### Landed work

1. **Selector parsing** — `parser/src/next/selector.rs` ports selector lists,
   compound selectors, combinators, namespaces, attribute operators, functional
   pseudo classes, pseudo elements, `An+B`, and nesting `&`. Selector
   round-trip and structure tests cover each family.
2. **Declarations/properties** — `parser/src/next/properties.rs` builds
   `DeclarationData` nodes through the generated 366-entry property table.
   Custom properties, unparsed declarations, CSS-wide keywords, `!important`
   (`DeclarationBlock.important`), shorthands, vendor-prefixed properties, and real token
   spans are preserved. `PropertyId::from_name` now splits vendor prefixes onto
   the prefixed variants.
3. **Rule/at-rule layer** — `parser/src/next/rules/` constructs
   `CssRule`/`CssRulePayload`/`DeclarationBlock` directly, including media
   queries, supports conditions, layers, containers, scopes, keyframes, `@page`
   margin rules, `@property`, `@font-*`, `@view-transition`, nesting, and
   unknown at-rules. The dense rule store and effective-key bookkeeping are
   gone; source order is arena allocation order.
4. **Parse cutover** — `Compiler::parse` returns the flattened `Compilation`.
   The parser has its own plain lexer token/Span types and no longer depends on
   `rocketcss_ast`. Value allocations thread the most recently consumed token
   span instead of `DUMMY_SP`.
5. **Codegen** — every writer reads through `Ast` accessors and `*Data` matches,
   including rules, selectors, media, declarations, values, tokens, and source
   spans. Lossless fallback serialization for unparsed values, custom
   properties, unknown descriptors, and license comments is retained.
6. **Visitor/tests/CI** — the visitor crate traverses the flattened tree and
   exposes mutable selector/declaration callbacks. Workspace tests, clippy with
   `-D warnings`, and Prettier all pass.

### Verification

- `cargo test --workspace` — pass.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `pnpm exec prettier --check .` — pass.
- `cargo run -p rocketcss_ast_next_tools` regenerates checked-in AST files.
- Parse → codegen lossless round-trip: `crates/codegen/tests/radix_ast.rs`
  covers rules, declarations, at-rules, page margins, nesting, keyframes,
  font descriptors, and property descriptors with nano disabled.
- Upstream Lightning CSS AST-to-CSS corpus (4,220 cases) — pass.

### Benchmark comparison

`cargo bench -p rocketcss_benchmark --bench pipeline -- --min-time 0.2`,
median of 100 samples, same machine:

| Stage   | Corpus    | `master` (`c2013dc`) | this branch |  Delta |
| ------- | --------- | -------------------: | ----------: | -----: |
| parse   | bootstrap |             63.56 ms |    68.43 ms |  +7.7% |
| parse   | tailwind  |            104.90 ms |   112.60 ms |  +7.3% |
| codegen | bootstrap |              6.16 ms |     8.41 ms | +36.5% |
| codegen | tailwind  |             11.78 ms |    20.96 ms | +77.9% |

The parse path is 7–8% slower than the radix-store baseline. The flattened AST
computes and stores a real span for every node and keeps a separate
`DeclarationEntry` node for authored `!important`, both of which the baseline
did not do. Node capacity is preallocated at `source.len() / 4`, matching the
measured ~4 bytes per node on both corpora. Codegen is slower because every
field is read through the `Ast` columns instead of directly from dense records;
the rule writer was changed to a single-pass peekable traversal to avoid
quadratic sibling scans and per-block allocation.

### Resolved known gaps

- Vendor-prefixed property names resolve to prefixed `PropertyId` variants.
- Leaf enums remain plain; node/plain classification is generated from the
  declaration files.
- Codegen helpers (`KnownFunction`, `KnownColor`, `FontFamily`,
  `VendorPrefix`, `FunctionFlags`, `AnimationKeywordClass`) now live on the
  flattened AST.
- The old parser modules and the old AST parse path were removed; the old AST
  crate remains only for the excluded nano crate and AST-size tooling.

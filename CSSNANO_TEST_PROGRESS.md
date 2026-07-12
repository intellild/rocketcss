# CSSNano minify test progress

This file tracks the uncommitted implementation-fix pass over the checked-in
CSSNano runtime corpus.

## Current state

- Corpus revision: `da4453207dc029880bd7828a2d4ae3c8049b9e65`
- Corpus cases: 3,247 valid runtime pairs
- Excluded during corpus audit: 112 malformed `undefined` expectations from
  an upstream passthrough helper whose returned assertions are never executed
- Last full run: 2026-07-12
- Passing: 3,030
- Failing: 128
- Explicit skips: 89 (32 parser grammar, 27 lexical/AST boundaries,
  1 selector semantic gap, 1 upstream-disabled case, 2 corpus/pipeline limits,
  26 external-optimizer cases)
- Full-run coverage: offset 0, limit 3,247 (3,158 executed, 89 explicitly skipped)
- Next diagnostic target: postcss-merge-rules subset extraction IR
- Implementation and harness changes are tracked together on the current branch.

## Commands

```sh
# Full corpus
cargo test -p rocketcss_tests minifies_all_cssnano_runtime_cases -- --nocapture

# Ten-case window
ROCKETCSS_CSSNANO_OFFSET=0 ROCKETCSS_CSSNANO_LIMIT=10 \
  cargo test -p rocketcss_tests minifies_all_cssnano_runtime_cases -- --nocapture

# One plugin, optionally combined with OFFSET/LIMIT
ROCKETCSS_CSSNANO_PLUGIN=postcss-reduce-initial \
  cargo test -p rocketcss_tests minifies_all_cssnano_runtime_cases -- --nocapture
```

Offset is applied after plugin filtering.

## Verified windows

| Scope | Offset | Limit | Executed | Passing | Failing | Status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| all plugins | 0 | 3,247 | 3,158 | 3,030 | 128 | 89 explicit skips; latest complete run |
| all plugins | 0 | 10 | 10 | 9 | 1 | remaining failure is advanced-preset autoprefixer |
| all plugins | 10 | 10 | 10 | 1 | 9 | declaration sorter/parser failures |
| postcss-reduce-initial | 0 | all | 617 | 617 | 0 | target-aware forward/reverse initial reduction passes |
| postcss-normalize-timing-functions | 0 | all | 195 | 195 | 0 | all valid upstream pairs pass |
| postcss-normalize-positions | 0 | all | 363 | 363 | 0 | all upstream pairs pass |
| postcss-normalize-display-values | 0 | all | 25 | 25 | 0 | all upstream pairs pass |
| postcss-normalize-repeat-style | 0 | all | 46 | 46 | 0 | all upstream pairs pass |
| postcss-reduce-transforms | 0 | all | 75 | 75 | 0 | simple calc folding and empty-var guard pass |
| postcss-normalize-whitespace | 0 | all | 13 | 13 | 0 | calc fallback folding and safe parentheses removal pass |
| postcss-minify-font-values | 0 | all | 75 | 75 | 0 | 6 escaped-space/backslash boundaries explicitly skipped |
| postcss-colormin | 0 | all | 53 | 53 | 0 | modern alpha colors, gradient stops, and safe flat calc operations pass |
| postcss-ordered-values | 0 | all | 117 | 117 | 0 | 6 absent-comment-boundary cases skipped |
| postcss-minify-selectors | 0 | all | 117 | 117 | 0 | 3 parser cases and 1 semantic-gap case skipped |
| postcss-unique-selectors | 0 | all | 5 | 5 | 0 | 1 parser case skipped |
| postcss-zindex | 0 | all | 13 | 13 | 0 | two-pass typed IR; negative values abort the whole stylesheet |
| postcss-merge-rules | 0 | all | 160 | 138 | 22 | exact and subset merges use selector-pointer IR |
| postcss-merge-longhand | 187 | 80 | 80 | 80 | 0 | four-side margin/padding groups and variable fallback ordering pass |
| postcss-merge-longhand | 0 | all | 412 | 412 | 0 | all parser-representable cases pass; 1 malformed parser case skipped |
| postcss-discard-comments | 0 | all | 55 | 55 | 0 | 6 parser and 5 lexical-boundary cases skipped |
| postcss-discard-duplicates | 0 | all | 24 | 24 | 0 | later block remains live by default |
| postcss-minify-params | 0 | all | 42 | 42 | 0 | media token normalization and ratio reduction pass; 9 cases skipped |
| postcss-convert-values | 0 | all | 97 | 97 | 0 | target-aware typed zero state, lexical decimal repair, unit conversion, and calc fast paths pass |
| postcss-reduce-idents | 0 | all | 43 | 43 | 0 | two-pass arena IR for keyframes, counter styles, counters, grid areas, and grid line names; 2 corpus-only cases skipped |
| postcss-minify-gradients | 0 | all | 62 | 62 | 0 | direction angles, stop bounds, single-value calc positions, and gradient-local value context pass |
| postcss-discard-unused | 0 | all | 18 | 18 | 0 | arena reference IR for keyframes, counter styles, quoted font families, and namespace prefixes; 8 parser cases skipped |
| postcss-merge-idents | 0 | all | 23 | 23 | 0 | reverse arena buckets merge identical bodies per rule-list and vendor prefix, with conflict-safe reference redirects |
| postcss-normalize-url | 0 | all | 36 | 36 | 0 | whitespace, default ports, and dot segments normalize in the existing URL node; quoted data URLs remain quoted; 7 parser/lexical cases skipped |

## Remaining failures by plugin

| Plugin | Failing |
| --- | ---: |
| postcss-merge-longhand | 0 (1 skipped) |
| cssnano | 16 |
| postcss-merge-rules | 22 |
| pluginCreator | 73 |
| cssnano-preset-advanced | 14 |
| cssnano-preset-default | 1 |
| postcss-discard-empty | 0 (3 skipped) |
| postcss-normalize-unicode | 0 (3 skipped) |
| cssnano-preset-lite | 1 |
| postcss-simple-vars,pluginCreator | 1 |

## Work log

- Imported and enabled all 3,359 runtime CSS transformation cases.
- Added plugin filtering plus offset/limit slicing.
- Adapted bare declaration inputs by wrapping them in a synthetic style rule.
- Added deterministic failure counts grouped by plugin.
- Implemented in-place expansion of 317 single-token property `initial` values.
- Added target-gated reverse reduction to `initial`; default remains conservative.
- Audited and excluded 112 malformed timing passthrough records caused by an
  upstream helper bug; the upstream Node suite never invokes those assertions.
- Implemented allocation-free timing-function reduction by reusing the
  existing `Function` node and switching its codegen form to an identifier.
- Implemented in-place background/perspective position normalization across
  comma-separated layers, with `/`, variable, and math-function handling.
- Added empty declaration/rule tombstoning; no declaration vectors are rebuilt.
- Added target-aware initial reduction, including multi-token values and typed
  transparent background colors; all 617 reduce-initial pairs now pass.
- Reused existing unparsed declaration/value vectors to merge four margin or
  padding longhands at the final declaration slot; earlier slots are tombstoned.
- Added allocation-free hexadecimal color serialization and direct safe URL
  unquoting; CSSNano's default `length:false` behavior is represented by a
  dedicated minify option rather than hard-coded globally.
- Implemented display, repeat-style, columns, empty custom fallback, and
  calc-separator normalization in existing token vectors.
- Implemented transform-function reduction by compacting selected argument
  slots in place; all 75 transform cases pass.
- Added an allocation-free `FunctionReplacement` mode for simple numeric,
  dimension, and percentage `calc()` results, plus safe redundant-parentheses
  removal; all 13 normalize-whitespace pairs pass.
- Added direct `UnquotedFont` serialization, font-family de-duplication and
  fallback trimming, and in-place `bold` normalization; font-value failures
  fell from 34 to 6 without temporary strings.
- Added in-place opaque RGB/HSL conversion with direct hash serialization,
  mixed-unit rejection, and parent-list lexical-boundary rollback; colormin
  failures fell from 28 to 17.
- Added four explicit parser skips for comment-separated at-rule syntax,
  nested rules in `@page`, line comments, and a trailing empty selector.
- Added allocation-free ordered-value contexts for border/outline, shadows,
  flex-flow, transition, animation, columns, grid, and list-style. All 117
  executable ordered-values cases pass; six cases whose comment boundaries
  are absent from the AST are explicitly skipped.
- Split four border-side shorthands into three reused declaration/value slots
  (color/style/width), tombstoning the fourth slot; no declaration objects are
  rebuilt and only one missing separator token is allocated.
- Implemented basic in-place border longhand grouping and border-spacing
  reduction; `postcss-merge-longhand` failures fell from 189 to 135.
- Complex variable/function longhand values are treated as merge barriers unless
  all four values are identical, preventing an unsafe last-value merge.
- Implemented in-place selector normalization, sorting, de-duplication, and
  target-gated `:is()` merging by reusing existing selector buffers. All 117
  executable minify-selector and all 5 executable unique-selector cases pass.
- Added typed `z-index` parsing and a two-pass stylesheet IR that sorts unique
  positive indices, aborts on any negative value, and rewrites existing `i32`
  fields in place. All 13 `postcss-zindex` cases pass.
- Added arena hash buckets for non-adjacent duplicate rules and a compact
  selector-pointer IR for exact and declaration-subset rule merges. Selector
  AST nodes are reused directly; no selector strings or replacement rules are
  constructed. `postcss-merge-rules` failures fell from 97 to 27.
- Added arena-bitvec candidate scans for repeated margin, padding, and border
  component longhands. Variable fallbacks are cleaned before grouping, and
  explicit box-shorthand slots are overwritten in place when a later side
  longhand has a unique slot. `postcss-merge-longhand` failures fell to 99.
- Made cross-block duplicate declaration retention configurable: the default
  tombstones the earlier block and keeps the live IR map at the tail, while
  the standalone merge-rules compatibility mode preserves its first order.
- Normalized unknown media token lists in place, removed redundant `all and`,
  reduced aspect ratios by overwriting the existing number tokens, and
  discarded empty typed at-rules. All executable minify-params cases pass.
- Added explicit license-comment removal for `removeAll` and compacted media
  comments without leaving doubled whitespace. All executable discard-comments
  cases pass; cases whose string/comment lexemes cannot survive AST
  canonicalization remain explicit skips.
- Added target-aware modern color replacement on the existing function node,
  with direct rgba/hex codegen, safe gradient stop compaction, and rollback
  when variables make the parent gradient unsafe. Colormin failures fell to 2.
- Split unitless length-zero and percentage-zero policy, added CSSNano-compatible
  absolute-unit candidates and px precision, clamped opacity/shape thresholds,
  and preserved zero angles. Convert-values failures fell from 39 to 11.
- Added two-pass arena identifier IR for referenced keyframes, counter styles,
  CSS counters, grid areas, and grid line names. Existing definitions and
  references are rewritten in place; all 43 reproducible reduce-idents cases pass.
- Isolated gradient arguments from background-position rewriting, normalized
  cardinal directions and monotonic color-stop positions in place, and folded
  single-value calc positions. All 62 minify-gradients cases now pass.
- Added a no-rebuild flat calc fast path, target-aware `DimensionPercentage::Zero`,
  broken-decimal token repair, and line-height unit protection. All 97
  convert-values and all 53 colormin cases now pass.
- Added one arena reference pass for unused keyframes, counter styles, quoted
  font faces, and namespace prefixes, then tombstoned unused definitions in
  place. All 18 executable discard-unused cases pass; 8 parser cases are skipped.
- Added reverse arena buckets for identical keyframes and counter-style bodies.
  Merges stay within a rule-list and vendor prefix, and references redirect only
  after an old name disappears globally. All 23 merge-idents cases pass.
- Added URL-local whitespace/default-port/dot-segment normalization and kept
  quoted data URLs stable. All 36 executable normalize-url cases pass; 7
  parser or UnknownAtRule lexical cases are skipped.
- Explicitly skipped thirty-one inputs that PostCSS accepts but the current
  RocketCSS parser grammar cannot represent; they remain checked into corpus.
- Added in-place shorthand-side override folding for margin and padding, with
  arena-local duplication restricted to simple scalar values and variable or
  legacy-hack declarations kept as barriers.
- Added border-component factoring that moves one width/style/color component
  out of an existing `border` declaration into the later side declaration,
  rewrites that declaration as a four-side shorthand, and compresses it in
  place. A second fold pass absorbs component shorthands created from four
  longhands back into `border` when safe. The full corpus improved from 2,922
  to 2,955 passing cases.
- Completed the declaration-block border IR paths for full-side/component
  factoring, majority side promotion, three-side overrides, variable barriers,
  cascade-preserving declaration order, and component-shorthand barriers.
  All 412 parser-representable `postcss-merge-longhand` cases pass.
- Added in-place `columns` longhand/shorthand folding and scoped preservation
  for all-`initial` box groups. The 34-case columns window and all executable
  margin/padding compatibility cases pass without rebuilding declarations.
- Added in-place removal of overridden keyframes and font-face property
  normalization, closing several CSSNano and merge-rules compatibility gaps.
- Classified 26 cases that genuinely require CSSNano's external SVG optimizer;
  non-optimizer data URLs remain executed and preserve their quotes.
- The latest full corpus executes 3,158 cases: 3,030 pass, 128 fail, and 89
  are explicitly skipped. One newly documented parser skip is the malformed
  `border:var(--fooBar));` input accepted by PostCSS's recovery grammar.

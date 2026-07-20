# Skipped CSSNano tests — current state

Snapshot of the CSSNano-derived test cases RocketCSS currently skips, grouped
by the scenario each skip corresponds to. Recorded 2026-07-18 on branch
`feat/ordered-values`.

Skip sources:

- Dynamic specs: `tests/fixtures/minify-dynamic/cssnano/*.json` — 32 specs,
  2105 cases, **852 skipped**. Skip logic lives in
  `still_requires_unsupported_transform` at `tests/src/minify_dynamic.rs`
  (138 pattern entries) plus per-case `upstreamSkip: true` flags.
- Static fixtures: `tests/fixtures/minify/cssnano/` — **4 directories skipped**
  via `still_requires_unsupported_transform` at `tests/src/minify.rs`.

## 1. Whole plugins not implemented (9 plugins, 276 cases)

The entire spec file is skipped (empty `text_pattern`).

| Plugin                   | Cases | Scenario                                          |
| ------------------------ | ----- | ------------------------------------------------- |
| merge-rules              | 87    | Cross-rule declaration merging                    |
| reduce-idents            | 45    | @keyframes / counter identifier renaming          |
| columns (merge-longhand) | 34    | column-* merging                                  |
| svgo                     | 29    | Inline SVG optimization (explicitly out of scope) |
| discard-unused           | 26    | Unused @font-face / @keyframes / ... removal      |
| merge-idents             | 23    | Cross-rule identifier merging                     |
| reduce-initial           | 18    | initial-value substitution                        |
| zindex                   | 13    | z-index rebasing                                  |
| borders (merge-longhand) | 1     | border longhand merging                           |

## 2. Upstream helper emits `undefined` (timing-functions, 112 cases)

The `animation:` and `transition:` shorthand timing cases now pass: the
animation shorthand parses into a typed `Animation` AST whose canonical
serialization matches cssnano, and a timing function minified to a keyword
keeps its timing rank in the transition token path.

- `undefined` 112 — upstream's `testPassthrough(t, fixture)` helper is called
  without the fixture argument, so the recorded expectation is literally
  "undefined"; these cases carry no upstream signal.

## 3. Full-pipeline vs single-plugin expectations (~160 cases)

Upstream expectations come from one plugin in isolation, but the harness runs
the full minify pipeline, so RocketCSS legitimately minifies further:

- normalize-positions: pipeline also minifies `#f1ff` to `#f1f` (78, bulk
  generated cases), folds calc/min/max/clamp (5), converts lengths/quotes
  around `var()` (9), further minifies values (4).
- minify-gradients: pipeline also minifies gradient colors in `"at"` cases (4),
  normalizes positions around `var()` (9).
- discard-duplicates: pipeline converts `font-weight:bold` to `700` (7),
  normalizes `100%` to `to` (1), merges equivalent declarations (1).
- repeat-style (9), normalize-whitespace (4), reduce-transforms (4): pipeline
  folds `var()`/`env()` fallbacks and calc.
- ordered-values: pipeline dedups identical declarations (2), converts colors
  (4), folds calc (1), strips url quotes (2).
- boxBase nesting level (4), convert-values (5), colormin calc (2),
  minify-params (2), normalize-url (1).

## 4. Specific transform gaps (not implemented, ~190 cases)

- minify-selectors: `:is()` folding (39), `:not()` simplification (13),
  universal selector removal (9), selector sorting (1).
- discard-comments: important-comment preservation (29), comment AST
  retention (1). ordered-values: upstream aborts ordering when an important
  comment is present (2).
- normalize-url: path normalization (9), port removal (2), url whitespace
  trimming (4), case handling (1).
- convert-values: 0% stripping policy (21), value rounding (3), opacity
  clamping (7).
- minify-font-values: font-name unquoting edge cases (5), case-sensitive
  family dedup (4), font shorthand minification (2), `var()` in font-family
  (12), custom property handling (1).
- discard-empty: cascading empty rule/declaration removal (7).
- normalize-charset: @charset reorder/remove/insert/dedup — all 6 cases.
- discard-duplicates: cross-rule / non-adjacent / partial / reordered-decl
  dedup (6).
- boxBase: `var()` fallback merging (4), custom property case preservation (8).
- unique-selectors: sorting (1). minify-params: `@media all` removal (9).

## 5. Parser gaps (18 cases)

- minify-selectors: mixin selectors (2), namespaced attribute selector (4).
- discard-comments: line comments (1), comments in at-rule prelude (1),
  @page nested rules (1).
- normalize-unicode: `var()` / `env()` / `initial` in unicode-range (3).
- discard-duplicates: @charset (2). discard-empty: null selector (2).
  normalize-url: data url (2).

## 6. Option-dependent upstream behavior (19 cases)

- colormin: Browserslist (6), custom properties (2).
- minify-params: Browserslist (6).
- convert-values: custom properties (3, option-gated upstream).
- discard-comments: with a flag (2).

## 7. Policy / output differences (~70 cases)

The transform exists but chooses a different output than upstream:

- convert-values: calc arithmetic (6), angle-units dedup pipeline (2).
- colormin: hex8 alpha output (2), name-vs-hex choice in gradients (3),
  -webkit-gradient (1), rgb-in-gradient spacing (4), `rgba(0,0,0,0)` →
  transparent (2).
- minify-gradients: property/function case handling (16), -webkit prefixed
  variants (6), color case (1).
- discard-comments / unique-selectors: comment-in-string / comment-in-selector
  handling (9).
- ordered-values: trailing `currentColor` stripped from borders (1), ambiguous
  keyframe-keyword names serialize round-trip-safely instead of upstream's
  order (2).
- minify-params: @media keyword case (13). normalize-string: unescape vs
  percent-encode (2). minify-selectors: `::before` case (1).

## 8. Disabled upstream (1 case)

- minify-params case 4 `should normalise @media queries (3) (lowercase and
uppercase)` is flagged `upstreamSkip: true` in the recorded spec.

## 9. Static fixture skips (4 directories, `tests/src/minify.rs`)

- `discard-empty/rules` — `a{}@media print{b{}}`: empty-rule removal needs
  cascade analysis.
- `discard-overridden/counter-style` — @counter-style overridden by a later
  same-name rule (including inside @supports).
- `discard-overridden/keyframes` — same scenario for @keyframes.
- `normalize-timing/step-start` — `steps(1,jump-start)` → `step-start`
  keyword simplification.

## Biggest work items

1. The 9 unimplemented plugins (276 cases), led by merge-rules (87).
2. Deciding per-case whether full-pipeline divergences (~160 cases) should
   update the recorded expectation or narrow pipeline behavior.

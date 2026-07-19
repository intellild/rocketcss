# Lightning CSS compatibility issue notes

This document tracks Lightning CSS issues that are primarily about
non-standard syntax, browser compatibility, or syntax downleveling. These
issues are documented rather than turned into feature tests until RocketCSS
implements the corresponding compatibility behavior.

## 2026-07-16: oldest open issues, batch 1

### [#6: Support for compiling CSS modules](https://github.com/parcel-bundler/lightningcss/issues/6)

- Category: non-standard CSS Modules and ICSS syntax.
- Status: module compilation is not implemented.
- Current preservation:
  - :local(...) and :global(...) are preserved as custom functions.
  - bare :local and :global are preserved as custom pseudo-classes.
  - composes has a known property ID but retains its value as an unparsed
    declaration.
  - :import, :export, and @value retain their generic rule/token form.
- Missing behavior: scoped-name rewriting, exports, composition, imports,
  @value replacement, and dependency analysis.
- [#404](https://github.com/parcel-bundler/lightningcss/issues/404):
  `:export` syntax round-trips generically, but no explicit module mode
  extracts an ordered export map for a JavaScript or Rust consumer.
- Scoped-name requirements:
  - [#351](https://github.com/parcel-bundler/lightningcss/issues/351) requires
    each local symbol in one file to remain unique, including coherent
    `grid-area` derived `-start` and `-end` names.
  - [#420](https://github.com/parcel-bundler/lightningcss/issues/420) requires
    stable project-relative module identity rather than absolute paths or an
    implicit process working directory.
  - [#460](https://github.com/parcel-bundler/lightningcss/issues/460) requires
    an interoperable escape/import model for global custom identifiers.
    Candidate CSS Modules syntax is still unsettled; local, global, and imported
    grid-area names and their implicit `-start`/`-end` lines must share one
    symbol mapping. Outside module mode, unknown wrappers and at-rules must
    remain generic lossless syntax.
  - [#491](https://github.com/parcel-bundler/lightningcss/issues/491) requests
    pattern callbacks or truncation for module, local, and hash components.
    Truncation must define Unicode/CSS identifier escaping and detect collisions
    in the reduced namespace; arbitrary host callbacks should not define the
    core Rust naming contract.
  - [#503](https://github.com/parcel-bundler/lightningcss/issues/503) requires
    SSR and browser toolchains to share a versioned hash contract, including
    canonical input bytes, module/local identity, project-root normalization,
    salt, algorithm, digest encoding, truncation, escaping, and collision
    handling. Choosing an algorithm alone cannot guarantee matching class names.
  - [#516](https://github.com/parcel-bundler/lightningcss/issues/516) requires
    `[name]` to derive from an explicit normalized module filename or identity.
    A missing identity must produce a structured configuration error rather
    than a panic; basename, extension, separators, and virtual filenames need
    defined cross-platform semantics.
  - [#541](https://github.com/parcel-bundler/lightningcss/issues/541) exposes
    platform-dependent scoped-name hashes. Canonicalize project-relative paths
    to `/` before hashing and define drive-letter, UNC, case, dot-segment, and
    Unicode normalization; never hash host-native `Path` display bytes.
  - [#627](https://github.com/parcel-bundler/lightningcss/issues/627) needs an
    explicit keyframe-symbol reference before an identifier inside a custom
    property can participate in module renaming. Ordinary untyped tokens must
    remain unchanged; definitions, animation declarations, and explicit
    references must share one local/import/global symbol mapping.
  - [#633](https://github.com/parcel-bundler/lightningcss/issues/633) requests
    consumer-facing export-key conventions such as camelCase. Apply these only
    after symbol resolution: never rename the source selector or scoped value.
    Define Unicode/escape/case rules, stable alias ordering, and report
    collisions such as `foo-bar` versus `fooBar` rather than overwriting.
  - [#636](https://github.com/parcel-bundler/lightningcss/issues/636) requires
    an explicit, versioned PostCSS/css-loader compatibility profile rather than
    silently changing the default hash. Freeze captured golden vectors for the
    precise algorithm, digest, salt, per-local input, path bytes, truncation,
    and CSS-identifier repair contract.
  - [#659](https://github.com/parcel-bundler/lightningcss/issues/659) requires a
    cross-file symbol graph for `@value` or `:import`. Imported classes must
    reuse the exporter's scoped identity rather than be hashed again locally;
    alias, missing-export, cycle, composition, and resolver errors need stable
    semantics. Without module compilation, preserve the complete generic rule.
  - [#753](https://github.com/parcel-bundler/lightningcss/issues/753) proposes
    file aliases for long CSS Modules paths. The alias grammar is not
    standardized; preserve `@alias`, `@value`, and `var(... from ...)`
    generically until an explicit compatibility profile and cross-file
    resolver define their scope and meaning.
  - [#761](https://github.com/parcel-bundler/lightningcss/issues/761) requires
    composition discovery inside `@layer`. Layer ancestry is not selector
    nesting: composition may change exports only and must never move either
    class's styles between cascade layers. The layered rule and unparsed
    `composes` declaration are currently retained but not compiled.
  - [#762](https://github.com/parcel-bundler/lightningcss/issues/762) requires
    grid area and line definitions and references to share one atomic symbol
    mapping even when track sizing contains `var()`. Dynamic tokens must remain
    intact; if consistent typed rewriting cannot be proved, skip the entire grid
    transform rather than partially hashing names.
  - [#763](https://github.com/parcel-bundler/lightningcss/issues/763) requires
    imported dashed identifiers to resolve identically inside arbitrary nested
    functions and group rules. The parser currently preserves these as generic
    function tokens; a future shared Variable node and symbol traversal must
    cover every nesting depth and reuse the cross-file symbol graph.
  - [#854](https://github.com/parcel-bundler/lightningcss/issues/854) extends
    that requirement to native nested StyleRules. Parent and child rules share
    one module scope: imported and local dashed identifiers in `&.info`,
    nested group rules, and recursively nested functions must use the same
    symbol graph. If the non-standard `from` grammar cannot be resolved,
    retain its complete function tokens rather than partially renaming them.
  - [#764](https://github.com/parcel-bundler/lightningcss/issues/764) proposes
    module-qualified custom-property definitions. The left-hand grammar is
    still at risk; strict parsing currently rejects it and recovery discards the
    declaration. Before module compilation exists, recovery needs a raw
    declaration form so the complete name, specifier, value, and importance are
    not lost.
  - [#825](https://github.com/parcel-bundler/lightningcss/issues/825) proposes
    the non-standard `@keyframes :global(name)` wrapper used by some CSS
    Modules toolchains. Strict parsing currently rejects it and recovery can
    lose the whole rule. Outside module mode, preserve the complete raw rule;
    inside an explicit compatibility profile, the global name must remain
    unhashed and animation references must use the same symbol mapping.
  - [#900](https://github.com/parcel-bundler/lightningcss/issues/900) requires
    scoped and global selectors to be resolved before any cross-rule merge.
    Local symbols keep their scoped identity, global symbols remain literal,
    and unparsed selectors, conditional context, layers, scopes, or authored
    cascade barriers must prevent coalescing.
  - [#908](https://github.com/parcel-bundler/lightningcss/issues/908) requires
    `default` to remain an ordinary class, composition, and export identifier.
    CSS symbol identity and string-keyed exports must not inherit JavaScript
    reserved-word rules; definitions, `composes`, imports, and exports must
    share one symbol mapping.
  - [#660](https://github.com/parcel-bundler/lightningcss/issues/660) makes the
    local/export identity an opt-in part of `[hash]` input so two locals in one
    file can differ when the pattern omits `[local]`. Keep defaults stable and
    distinguish this input choice from algorithm configuration and full
    cross-tool byte parity.
- Guardrail: outside an explicit CSS Modules mode, typed variants must not
  erase module markers or `:export` rules. Future naming must be deterministic,
  collision-resistant, cross-platform, and include module and local identity.

### [#9: Parse remaining CSS properties](https://github.com/parcel-bundler/lightningcss/issues/9)

- Category: Grid legacy syntax and vendor-prefix compatibility.
- Status: Grid vendor prefix generation and target-based compatibility
  transforms are not implemented.
- Current preservation: known but unsupported declarations retain their known
  property ID and original value tokens through Declaration::Unparsed.
- Missing behavior: target-aware legacy Grid prefixing. The broader issue also
  contains unrelated typed-property work and should be split by property
  family before implementation.
- [#523](https://github.com/parcel-bundler/lightningcss/issues/523) requests
  removal of authored `-ms-grid` fallbacks. This is safe only with explicit
  targets that prove the legacy declaration redundant. Without targets,
  preserve both old and modern declarations, including their order and values.

### [#21: Downlevel :any-link selector](https://github.com/parcel-bundler/lightningcss/issues/21)

- Category: selector downleveling.
- Status: downleveling is not implemented.
- Current preservation: :any-link round-trips as a custom pseudo-class.
- Missing behavior: target-aware expansion to :link and :visited, including
  the legacy browser differences for link-bearing elements.
- Guardrail: retain the original selector when no downlevel target is enabled;
  do not silently normalize it merely because a typed AST variant exists.

### [#87: Support all Selectors Level 4 selectors](https://github.com/parcel-bundler/lightningcss/issues/87)

- Category: evolving selector syntax and compatibility.
- Status: the representative :nth-col(), :nth-last-col(),
  :nth-child(... of S), and :nth-last-child(... of S) forms have typed AST,
  parser, and codegen support. Full Selectors Level 4 validation and
  target-based downleveling are not implemented.
- Current preservation: selectors rejected by typed parsing can use
  Selector::Unparsed so recovery mode serializes the original selector text.
- Existing coverage: the imported Lightning CSS corpus checks that the
  representative forms parse; this is not evidence of full Selectors Level 4
  compatibility.
- [#759](https://github.com/parcel-bundler/lightningcss/issues/759):
  unknown custom pseudos such as `.bar:baz` stay typed and keep their original
  unforgiving selector list. Recovered `Selector::Unparsed` branches are still
  removed by minification, which can activate the remaining selectors.
  An unparsed branch must instead block default splitting, deletion, or
  `:is()` wrapping; target-aware graceful degradation is a separate opt-in.
- Guardrail: unsupported or invalid selector content must remain recoverable
  and lossless rather than being partially rewritten into different semantics.

## AST preservation policy

Prefer typed nodes when RocketCSS can represent the complete syntax and
serialize it without losing source-significant content. Otherwise retain known
property identity with unparsed value tokens, generic custom functions/rules,
or explicit unparsed recovery nodes. Compatibility transforms must be opt-in
and target-aware; parsing alone must not discard prefixes, markers, unknown
arguments, rule order, or fallback declarations.

When syntax has a semantic representation but source spelling remains
observable, keep the original form alongside the semantic value and only
normalize it during an explicitly enabled transform or minification pass.

## 2026-07-16: oldest open issues, batch 2

### [#99: CSS Color Level 5](https://github.com/parcel-bundler/lightningcss/issues/99)

- Category: evolving color syntax and color-management compatibility.
- Status: typed semantics for color-mix(), color-contrast(), relative colors,
  @color-profile, device-cmyk(), and ICC conversion are not implemented.
- Current preservation: unsupported color functions remain recursive function
  tokens in unparsed declarations; @color-profile remains an unknown at-rule.
  The color-mix() name is classified for one percentage-minification guard,
  which is not an implementation of color mixing.
- Guardrail: unsupported functions, channel expressions, profiles, and
  fallbacks must round-trip until a complete typed representation exists.
- [#592](https://github.com/parcel-bundler/lightningcss/issues/592) requests
  target fallbacks for OKLab/OKLCH with variables. Static color channels plus
  a dynamic alpha can eventually share the original alpha token across
  generated sRGB/P3 values, but variable color channels cannot be converted at
  build time. Preserve the authored modern declaration and all variable tokens;
  generated fallbacks must be target-gated and remain in the same cascade
  context and order.
- [#618](https://github.com/parcel-bundler/lightningcss/issues/618) shows that
  color notation shortening is not always context-free: powerless or missing
  HSL channels can become observable during interpolation in `color-mix()`.
  Value normalization must be independently disableable, and unsupported color
  functions must prevent recursive color-space folding until their interpolation
  semantics are represented. Preserve authored channel information rather than
  replacing a visually identical standalone color.

### [#102: Cannot use CSS modules with bundle option](https://github.com/parcel-bundler/lightningcss/issues/102)

- Category: CSS Modules bundling and exports compatibility.
- Status: neither bundling nor CSS Modules compilation/exports is implemented.
- Missing behavior: exports must be namespaced and returned per source file
  rather than collapsing identical local names into one mapping.
- Guardrail: a future merged AST must retain source identity for every node
  participating in name generation and dependency analysis.

### [#109: Preserve hand-picked color fallbacks](https://github.com/parcel-bundler/lightningcss/issues/109)

- Category: target-aware color fallback generation.
- Status: browser targets, color conversion, and fallback synthesis are not
  implemented.
- Current preservation: author declaration chains, custom-property tokens, and
  nested media/supports rules remain ordered.
- Guardrail: generated values must not overwrite, reorder, deduplicate, or move
  author-selected fallbacks across conditional-rule boundaries.

### [#155: Use non-ASCII ident code points](https://github.com/parcel-bundler/lightningcss/issues/155)

- Category: evolving CSS Syntax identifier grammar.
- Status: the tokenizer currently accepts every non-ASCII UTF-8 code point in
  identifiers and therefore does not implement the newer restricted set.
- Missing behavior: a shared code-point-aware predicate must be used for token
  starts, hyphen lookahead, and name continuation.
- Guardrail: restrictions on directly written code points must remain separate
  from escaped code points. Raw spans and token boundaries must be preserved
  even when escapes are decoded into semantic identifier strings.

### [#202: Nesting downlevel for older browsers](https://github.com/parcel-bundler/lightningcss/issues/202)

- Category: CSS nesting and :is() target downleveling.
- Status: RocketCSS has no browser-target model or nesting transform.
- Current preservation: native nesting rules and & round-trip without implicit
  expansion. Most :is() selectors remain typed, but codegen currently unwraps
  some single simple items without a target or forgivingness proof.
- [#472](https://github.com/parcel-bundler/lightningcss/issues/472): modern
  native nesting is accepted without a feature flag and parser failures use a
  structured `Result`; RocketCSS has no CLI that could unwrap and crash.
  Preserve this default while keeping invalid syntax errors explicit and never
  converting downstream caller misuse into a parser compatibility promise.
- [#623](https://github.com/parcel-bundler/lightningcss/issues/623) proposes
  synthesizing native nesting from repeated flat selector prefixes. Only do so
  for targets supporting the required nesting grammar, after typed selector and
  specificity analysis proves equivalence and the configured size metric wins.
  Never create a parent selector list that raises branch specificity, cross a
  conditional/layer boundary, or rewrite recovered selectors. Default output
  must preserve the authored flat rule hierarchy.
- [#777](https://github.com/parcel-bundler/lightningcss/issues/777) requires
  source serialization to distinguish authored implicit nesting from explicit
  `&`. Bare class and type selectors remain unchanged by default; only an
  explicit target-aware lowering may add `&` to selector forms unsupported by
  those targets, and it must never duplicate an authored nesting selector.
- Guardrail: only transform a subset whose matching and specificity are proven
  equivalent. Without targets, preserve the original syntax; if a requested
  target cannot be served safely, retain it and report an unsupported case.

- [#962](https://github.com/parcel-bundler/lightningcss/issues/962) concerns an
  `@property` rule nested directly in a style rule. That placement is invalid,
  so RocketCSS rejects it in strict mode rather than hoisting it and changing
  scope or cascade order. Future recovery diagnostics must report the invalid
  at-rule without manufacturing a top-level registration.

### [#213: Negating custom media queries](https://github.com/parcel-bundler/lightningcss/issues/213)

- Category: custom-media expansion and media-query compatibility.
- Status: custom-media definition expansion and target transforms are not
  implemented.
- Current preservation: custom-media definitions retain their query tokens,
  while references and condition-level not retain typed precedence.
- Guardrail: expansion must negate the substituted query as a whole, preserve
  the distinction between a query qualifier and condition-level not, detect
  cycles, and retain unknown references rather than rewriting them incorrectly.
- [#582](https://github.com/parcel-bundler/lightningcss/issues/582) combines
  custom-media definitions with layers. Nested definitions currently fail in
  strict mode and recovery discards them; layered imports retain their syntax
  but no bundler exists. Until scope semantics are standardized, do not hoist a
  conditional definition implicitly. Recovery should preserve the full nested
  rule, and a future bundler must build the definition graph before wrapping
  imported rules in layers so relocation cannot change visibility.

## 2026-07-16: oldest open issues, batch 3

- [#951](https://github.com/parcel-bundler/lightningcss/issues/951) requires a
  stylesheet replacement returned by a visitor to enter the same custom-media
  expansion phase as the original stylesheet. Until visitor replacement and
  expansion exist, retain both `@custom-media` definitions and references
  unchanged; future processing must not make expansion depend on lifecycle
  path or silently discard the definition.

### [#218: Provide a workaround for :where](https://github.com/parcel-bundler/lightningcss/issues/218)

- Category: selector target compatibility.
- Status: :where() has typed parser/codegen support, but no downlevel transform
  or browser-target model exists.
- Current preservation: :where() and its ordered selector list round-trip
  without expansion.
- Guardrail: plain-selector or :is() expansion cannot preserve zero
  specificity and therefore changes cascade semantics. Such a workaround must
  never be implicit; at most it can be an explicitly unsafe opt-in.

- [#963](https://github.com/parcel-bundler/lightningcss/issues/963) requests
  downleveling for older targets. Since an equivalent expansion does not
  exist, a future target layer must retain `:where()` and provide a structured
  unsupported-target diagnostic instead of rewriting it to `:is()` or a plain
  selector.

### [#221: Support custom units](https://github.com/parcel-bundler/lightningcss/issues/221)

- Category: experimental CSS Variables Level 2 custom-unit syntax.
- Status: the proposal remains open and native desugaring is not implemented.
- Current preservation: dashed unknown units remain UnknownDimension tokens in
  unparsed declarations, and custom-property definitions retain their tokens.
- [#767](https://github.com/parcel-bundler/lightningcss/issues/767) extends
  lossless unknown-unit handling to media features. An identifier placeholder
  or unknown dimension inside `max-width` must retain the complete raw feature
  and rule body; never infer a known unit from a suffix such as `customPx`.
- Guardrail: only a finalized dashed-ident unit grammar may trigger definition
  resolution. Ordinary unknown units, missing definitions, invalid values, and
  cycles must remain unconverted rather than being guessed.

### [#224: Export empty CSS Module classes](https://github.com/parcel-bundler/lightningcss/issues/224)

- Category: CSS Modules empty-class exports.
- Status: scoped-name compilation and JavaScript exports are not implemented.
- Current preservation: ordinary empty style rules remain in parser/codegen
  and current minification.
- Guardrail: future module export collection must run before any discard-empty
  transform, so removing an empty CSS rule cannot turn a valid local class
  symbol into an undefined JavaScript export.

### [#232: Add linear() easing](https://github.com/parcel-bundler/lightningcss/issues/232)

- Category: CSS Easing Level 2 syntax and downleveling.
- Status: the linear keyword exists, but functional linear() stop semantics
  and target transforms are not implemented.
- Current preservation: linear() remains a recursive function token in an
  unparsed declaration; percentage offsets are protected from unsafe unit
  removal.
- Guardrail: generating @keyframes is not a general equivalent polyfill for
  transitions or arbitrary animation values. Any approximation must be
  restricted, explicit, and preserve unknown/variable stop expressions.

### [#251: Keep explicit filter function defaults](https://github.com/parcel-bundler/lightningcss/issues/251)

- Category: filter function and vendor-target compatibility.
- Status: typed Filter AST requires explicit values, the declaration parser is
  not connected, and there is no browser-target or prefix-generation layer.
- Current preservation: brightness(1) and similar function arguments remain in
  unparsed function tokens; the minifier does not remove them.
- Guardrail: explicit filter arguments must not be omitted by default. Any
  future target-gated optimization must verify both standard and prefixed
  implementations, including older browsers where an empty argument computes
  differently.

### [#268: default is not a custom-ident](https://github.com/parcel-bundler/lightningcss/issues/268)

- Category: reserved-keyword validation for the custom-ident grammar.
- Status: support is partial. Keyframe names reject default, but there is no
  shared context-aware validator; counter-style and view-transition paths are
  incomplete.
- Current preservation: unsupported or rejected declaration values reset to
  their original position and become Declaration::Unparsed.
- Guardrail: central validation must cover animation, grid, counter, font,
  view-transition, and relevant at-rules without treating quoted strings as
  identifiers. Recognized at-rules also need a lossless recovery node before
  stricter validation can safely reject reserved names.

## 2026-07-16: oldest open issues, batch 4

### [#289: Preserve selector token boundaries for legacy Chrome](https://github.com/parcel-bundler/lightningcss/issues/289)

- Category: legacy Chrome selector and unicode-range token-boundary behavior.
- Status: compact output emits the specification-correct u+a form; there is no
  browser-target workaround for Chrome versions before the parsing fix.
- Current preservation: the AST keeps the two type selectors and adjacent
  sibling combinator as separate components.
- Guardrail: any target-aware spacing workaround must reparse to the identical
  selector AST and only apply to affected browsers; modern output should not
  gain unconditional whitespace.

### [#291: Stable CSSModuleExports key ordering](https://github.com/parcel-bundler/lightningcss/issues/291)

- Category: deterministic CSS Modules exports.
- Status: CSS Modules compilation and exports serialization are not
  implemented.
- Guardrail: future public export order must have an explicit stable contract,
  such as source discovery order or lexical order. Hash tables may support
  lookup but must never determine serialized key order.

### [#306: Keep a final semicolon for IE targets](https://github.com/parcel-bundler/lightningcss/issues/306)

- Category: target-specific parser performance tradeoff.
- Status: compact output omits optional final semicolons and newlines; there is
  no browser-target model or public punctuation option.
- Current preservation: pretty output retains the final semicolon and newline,
  while compact output intentionally does not retain source punctuation.
- Guardrail: any IE-specific performance mode must be explicit and keep the
  semicolon choice separate from newline and AST minification behavior.

### [#310: Preserve charset and support ASCII-safe strings](https://github.com/parcel-bundler/lightningcss/issues/310)

- Category: stylesheet encoding and string serialization compatibility.
- Status: a leading typed @charset rule is preserved and tested; original
  Unicode escape spelling and an ASCII-only serializer are not implemented.
- Current preservation: CSS escapes are decoded to their Unicode scalar value,
  and literal backslashes remain distinguishable, but codegen emits legal
  UTF-8 rather than reconstructing the source escape spelling.
- Guardrail: @charset must remain first. ASCII-safe output must preserve scalar
  values and literal backslashes without double escaping, and must not confuse
  an escaped character with text that merely resembles an escape.

### [#312: Correct and safe :is() handling](https://github.com/parcel-bundler/lightningcss/issues/312)

- Category: forgiving selector lists and legacy-any target downleveling.
- Status: forgiving inner parsing, target support checks, legacy
  :-moz-any()/:-webkit-any() generation, and complex-selector fallback are not
  implemented.
- Current risk: codegen may unwrap a single simple :is() item without proving
  that the item is valid in the target or that forgiving-list behavior is
  irrelevant.
- Concrete duplicate [#345](https://github.com/parcel-bundler/lightningcss/issues/345):
  `:is(:does-not-exist),p` currently serializes as
  `:does-not-exist,p`, leaking invalidity into the unforgiving outer list.
- [#430](https://github.com/parcel-bundler/lightningcss/issues/430) may factor
  selector alternatives into `:is()` only when common structure is identical,
  every branch has equal specificity, and all arguments are valid for every
  target. Pseudo-elements and unknown or invalid arms remain barriers; see
  [#352](https://github.com/parcel-bundler/lightningcss/issues/352).
- Guardrail: preserve unknown or invalid arms without invalidating unrelated
  selectors; compare specificity before wrapper removal; only generate legacy
  any forms when their grammar and forgiving behavior are valid for the target.
  Unsafe complex cases must remain intact or produce an explicit unsupported
  result rather than silently losing matches.

## 2026-07-16: oldest open issues, batch 5

### [#314: Support `infinite` in media-query resolution ranges](https://github.com/parcel-bundler/lightningcss/issues/314)

- Category: Media Queries Level 5 range syntax.
- Status: the typed media AST has no resolution-specific `infinite` value.
  `(infinite > resolution)` is currently assigned the wrong typed roles,
  while `(resolution < infinite)` falls back to an unknown condition.
- Current preservation: codegen does not produce Lightning CSS's invalid
  `min-infinite` or `max-resolution` rewrite, but operand order determines
  whether the preserved source has incorrect typed meaning or unknown recovery.
- Guardrail: recognize `infinite` only as a resolution value in the relevant
  media-feature context, prefer the known `resolution` feature during
  disambiguation, and preserve unknown uses elsewhere.

### [#338: Handle a leading UTF-8 BOM](https://github.com/parcel-bundler/lightningcss/issues/338)

- Category: CSS encoding preprocessing.
- Status: a leading U+FEFF is not skipped. It can make a leading `@import`
  fail or become part of an unintended type selector.
- Current preservation: an interior U+FEFF remains identifier content, which
  is correct and must not be changed by global trimming.
- Guardrail: ignore only the UTF-8 signature at input offset zero, define how
  original byte spans relate to logical line and column locations, and do not
  claim UTF-16/32 decoding support for the public `&str` parser API.

### [#347: Preserve legacy `@nest` while reporting it](https://github.com/parcel-bundler/lightningcss/issues/347)

- Category: obsolete CSS nesting syntax and non-fatal diagnostics.
- Status: `@nest` is parsed into a typed nesting rule and serialized
  losslessly, but the parser exposes no warning channel and emits no warning.
- Current preservation: existing AST and codegen retain the complete legacy
  rule; adding diagnostics must not rewrite or discard it.
- Guardrail: introduce a structured non-fatal diagnostic API before emitting a
  deprecation warning, leave presentation to the host or CLI, and recommend
  native nesting syntax without turning preservation into a parse failure.

## 2026-07-16: oldest open issues, batch 6

### [#352: Do not lower nested pseudo-elements through `:is()`](https://github.com/parcel-bundler/lightningcss/issues/352)

- Category: pseudo-elements in CSS Nesting lowering.
- Status: RocketCSS preserves native nested StyleRule and `&` nodes and has
  no flattening transform or diagnostic. The four legacy pseudo-elements
  normalize from `::before`-style spelling to their valid single-colon form.
- [#829](https://github.com/parcel-bundler/lightningcss/issues/829) demonstrates
  that `&:not(.class)` nested under a pseudo-element cannot be flattened by
  appending `:not()` after that pseudo-element or by moving it before the
  pseudo-element, because either output is invalid or changes authored
  nesting semantics. Validation must be context-aware: permit only
  pseudo-classes that the current Selectors specification explicitly allows
  after that pseudo-element, and propagate the featureless matching
  restriction into logical pseudo-class arguments.
- Guardrail: `&` cannot represent pseudo-elements, and pseudo-elements are
  invalid inside `:is()`. Never generate `:is(parent-pseudo)` or textually
  splice a non-equivalent flattened selector; retain native syntax or report
  an explicit unsupported result.

### [#366: Expose comments to visitors without losing trivia](https://github.com/parcel-bundler/lightningcss/issues/366)

- Category: comment/trivia preservation and visitor parity.
- Status: the tokenizer recognizes comments, but structured parsing generally
  discards ordinary comments. Initial license comments and some unparsed token
  comments survive; there is no positioned Comment AST node or dedicated
  visitor callback.
- Current loss: compact output and minification may remove token-list comments,
  while typed-value, rule-between, and declaration-adjacent comments often
  never reach the AST.
- Guardrail: a future arena-backed trivia model must preserve raw content,
  spans, source order, attachment, directives, and token-boundary safety.
  License comments should be a policy on the same model, not a parallel map.

### [#369: Prune vendor fallbacks only with explicit targets](https://github.com/parcel-bundler/lightningcss/issues/369)

- Category: target-aware vendor-prefix compatibility.
- Status: there is no browser-target model or prefix generation/removal pass.
  Author-provided prefixed pseudos, values, and declarations are preserved;
  minification removes only structurally exact duplicates.
- [#436](https://github.com/parcel-bundler/lightningcss/issues/436) adds an
  idempotence/cardinality constraint for future `image-set()` prefixing:
  generate at most one required standard and one prefixed variant, preserve
  authored fallback order, and never enqueue a duplicate standard declaration.
  Parsing currently retains these functions in known-property token fallback.
- [#695](https://github.com/parcel-bundler/lightningcss/issues/695) requires a
  property's prefixed and standard forms to remain distinct fallbacks in both
  authored orders. Prefix identity is part of declaration identity; only an
  explicit target model may prove that either fallback can be removed.
- [#710](https://github.com/parcel-bundler/lightningcss/issues/710) applies the
  same rule inside `@supports`: `-webkit-box-orient` and its prefix are part
  of the capability query. Unknown conditions must retain their raw spelling,
  and a future typed condition must carry the prefix in its property identity.
- [#718](https://github.com/parcel-bundler/lightningcss/issues/718) also requires
  target-aware generation of Safari fallbacks such as
  `-webkit-text-decoration`; omitted targets must conservatively retain
  authored fallbacks, while explicit targets need stable merge semantics.
- [#872](https://github.com/parcel-bundler/lightningcss/issues/872) adds an
  idempotence and cardinality constraint to that generation. An equivalent
  authored `-webkit-text-decoration` must satisfy the requested prefix rather
  than cause another declaration to be inserted; repeated transform/minify
  passes must not accumulate prefixes. Prefix, value, importance, and authored
  fallback order remain part of declaration identity, so differing fallbacks
  cannot be removed by this duplicate check.
- [#925](https://github.com/parcel-bundler/lightningcss/issues/925) requires
  target data to generate `-webkit-user-select` when Safari still needs it.
  Authored prefixed and standard declarations may carry different values, so
  preserve prefix, value, importance, and fallback order. Generation must use
  the shared property metadata and remain idempotent.
- [#826](https://github.com/parcel-bundler/lightningcss/issues/826) requires
  `-webkit-fill-available`, `-moz-available`, and `stretch` to remain an
  ordered `width` fallback chain when targets are omitted or empty. Prefix
  identity may live in the value spelling rather than the property name, so
- [#934](https://github.com/parcel-bundler/lightningcss/issues/934) requires
  `-webkit-text-size-adjust` for the iOS Safari target rather than desktop
  Safari. A target-aware pass must emit the prefixed fallback before
  `text-size-adjust`, preserve authored prefix/value/order pairs, and never
  infer the mobile requirement from the desktop Safari version alone.
  only explicit resolved targets may prove that one of these values is
  redundant.
- Guardrail: require authoritative target data and proven semantic equivalence,
  preserve cascade and selector-list validity, retain differing fallback
  values, deduplicate generated variants, and make the transform idempotent.
  Never infer compatibility from an unknown string's vendor-like prefix.

### [#379: Preserve compatible media-query ratio syntax](https://github.com/parcel-bundler/lightningcss/issues/379)

- Category: target-sensitive media-query ratio serialization.
- Status: typed ratios exist, but the AST does not remember an explicit `/1`
  and codegen always emits the shorter single number. This is spec-equivalent
  for conforming engines but failed in older single-number implementations.
- Guardrail: affected targets need two integer components. Gate that separately
  from legacy `min-`/`max-` versus range syntax and from float, calc, and
  zero-denominator handling; modern targets may use the shortest equivalent.

### [#419: Downlevel logical properties without assuming writing mode](https://github.com/parcel-bundler/lightningcss/issues/419)

- Category: target-aware logical-property compatibility.
- Status: logical property metadata and AST variants exist, but parsing
  currently retains many values as known-ID unparsed declarations and no
  browser-target lowering pass exists.
- Current preservation: native logical syntax round-trips, and the minifier
  treats logical declarations as barriers to physical box folding.
- Guardrail: expand unsupported shorthands to logical start/end longhands when
  valid. Never map to physical top/right/bottom/left unless writing-mode and
  direction are proven for every matched element under cascade and inheritance;
  preserve order, importance, CSS-wide keywords, variables, and fallbacks.
- [#540](https://github.com/parcel-bundler/lightningcss/issues/540) extends this
  constraint to `overflow-inline` and `overflow-block`. They currently survive
  as generic unparsed declarations but should become metadata-generated known
  properties. Never assume inline maps to x and block maps to y: vertical
  writing modes reverse those axes. Any fallback must precede and retain the
  logical declaration unless the applicable writing mode is proven.

## 2026-07-16: oldest open issues, batch 7

- [#927](https://github.com/parcel-bundler/lightningcss/issues/927) requires
  dynamic logical shorthands to be transformed atomically. A whole-value
  `var()` may expand to one or two components at computed-value time; if the
  typed arity cannot be proved, preserve the complete logical declaration.
  Nested `calc(var(...))` values and fallbacks remain one structured value and
  must never be partially copied into physical declarations.

### [#424: Preserve and safely downlevel nested `@media`](https://github.com/parcel-bundler/lightningcss/issues/424)

- Category: nested conditional rules and old-browser downleveling.
- Status: one-level style-nested media parsing is typed, but recursive native
  round-trip is not yet reliable and there is no target-based flattening.
- Current loss: the issue's two-level example serializes a declaration directly
  before the inner `@media` without a required separator, producing invalid
  compact CSS. The nested hierarchy therefore cannot yet be treated as a
  complete preservation feature.
- Guardrail: fix native ordered `NestedDeclarations` serialization first.
  Any later lowering must be target-gated, lift the ancestor selector, preserve
  rule order, and combine comma query lists by Cartesian product with correct
  `not`/`or` precedence. Unknown conditions must remain native.

### [#426: Keep individual transform properties independent](https://github.com/parcel-bundler/lightningcss/issues/426)

- Category: Transform Level 2 target downleveling.
- Status: `translate`, `rotate`, `scale`, and `transform` are separate
  known properties and currently round-trip independently through lossless
  fallback; there is no browser-target lowering pass.
- Guardrail: never silently bake individual properties into `transform`.
  Their fixed composition order does not erase their independent cascade,
  transition, animation, variable, inheritance, or CSSOM behavior. If an old
  target cannot be served equivalently, preserve native syntax or report an
  explicit unsupported result.

## 2026-07-16: oldest open issues, batch 8

### [#477: Recover from declaration-like syntax with no colon](https://github.com/parcel-bundler/lightningcss/issues/477)

- Category: browser-compatible CSS error recovery.
- Status: strict parsing returns `InvalidDeclaration`. With
  `error_recovery` enabled, RocketCSS discards the malformed item through its
  terminating semicolon and continues parsing valid declarations and rules.
- Current loss: the invalid source has no AST node or diagnostic collection;
  `Declaration::Unparsed` is reserved for syntactically valid declarations
  whose values lack typed support.
- Guardrail: recovery must discard exactly one invalid item without swallowing
  following declarations or nested rules. A future lossless editor mode needs a
  dedicated invalid-declaration node with raw span/content rather than
  overloading the valid-value fallback.

### [#960: Preserve invalid CSS during recovery](https://github.com/parcel-bundler/lightningcss/issues/960)

- Category: lossless error recovery for compatibility syntax.
- Status: `error_recovery` intentionally follows browser-style discard for
  malformed declarations such as `--color-*:initial`; there is no raw invalid
  declaration or rule AST node.
- Guardrail: a future lossless mode must keep the complete raw item, including
  its name, delimiter, tokens, and location, and treat it as a minification
  barrier. Do not misuse `Declaration::Unparsed`, which represents a valid
  property name with an otherwise unsupported value.

## 2026-07-16: oldest open issues, batch 9

### [#532: Preserve non-standard Yahoo media-query syntax](https://github.com/parcel-bundler/lightningcss/issues/532)

- Category: email-client media-query compatibility.
- Status: `@media screen yahoo` is not valid Media Queries grammar, but
  RocketCSS retains the complete prelude as an unknown media condition and
  serializes it without interpreting the adjacent identifiers as media types.
- Guardrail: transforms and minification must conservatively skip unknown media
  conditions and preserve token order. Error recovery must not be required for
  this lossless fallback.

### [#534: Keep modern viewport units instead of unsafe lowering](https://github.com/parcel-bundler/lightningcss/issues/534)

- Category: viewport-unit target compatibility.
- Status: `dvh`, `svh`, `lvh` and their width, inline, block, min, and max
  families are typed and preserved; no browser-target lowering pass exists.
- Guardrail: these units are not equivalent to `vh` or `vw`. A future opt-in
  approximation may emit an older fallback before the authored declaration,
  but must retain the modern unit. Correct dynamic viewport behavior may need
  runtime support and cannot be achieved by destructive syntax replacement.

### [#538: Preserve non-standard `@important` wrappers](https://github.com/parcel-bundler/lightningcss/issues/538)

- Category: non-standard transform extension.
- Status: the core transform is intentionally absent. RocketCSS retains the
  wrapper as an `UnknownAtRule`, including its raw block, but does not expose
  the nested declarations as typed style rules to ordinary visitors.
- Guardrail: do not silently give non-standard syntax core semantics. An
  opt-in Rust plugin or custom-at-rule parser hook may implement it, while the
  default parser must continue preserving the complete wrapper and body.

## 2026-07-16: oldest open issues, batch 10

### [#568: Preserve authored system UI font fallbacks](https://github.com/parcel-bundler/lightningcss/issues/568)

- Category: font-family target compatibility.
- Status: RocketCSS does not inject an opinionated system font stack and keeps
  `system-ui` as a typed generic family. Minification now also retains later
  author fallbacks such as `sans-serif`; only the five universally established
  CSS2 generic families terminate the list.
- Guardrail: do not expand or replace `system-ui` implicitly. Any compatibility
  stack must be explicit and target-aware, while preserving both the authored
  generic and every fallback needed by engines that do not recognize it.

### [#569: Preserve pseudo-elements inside forgiving selectors](https://github.com/parcel-bundler/lightningcss/issues/569)

- Category: selector validation, diagnostics, and recovery compatibility.
- Status: RocketCSS currently accepts and preserves a pseudo-element inside
  `:where()` as typed syntax, so minification does not collapse the argument to
  an empty selector. There is no warning collection API.
- [#760](https://github.com/parcel-bundler/lightningcss/issues/760) also
  requires `:is(::before)` to remain non-empty and wrapped while emitting a
  non-fatal diagnostic. RocketCSS currently accepts the typed argument but
  singleton `:is()` codegen unwraps it to `:before`, changing invalid syntax
  into a potentially matching selector. Do not unwrap without a proof that
  forgiving-list validity and matching semantics are unchanged.
- Guardrail: if grammar validation is tightened, preserve each invalid argument
  as raw syntax and emit a structured non-fatal diagnostic. Never partially
  delete selector components or turn an authored argument into `:where()`.

### [#580: Preserve variable fallbacks and evolving revert semantics](https://github.com/parcel-bundler/lightningcss/issues/580)

- Category: custom-property fallback and cascade compatibility.
- Status: typed colors followed by `var()` declarations are retained for both
  `color` and `background-color`. `revert-rule` inside a variable fallback is
  currently preserved as an ordinary token, and adjacent rules are not merged.
- Guardrail: do not delete an authored fallback merely because a later value
  contains `var()`. Rule merging and declaration pruning must understand
  `revert-rule`, `first-valid()`, variable substitution, and rule boundaries;
  otherwise preserve the declarations and their original cascade order.

## 2026-07-16: oldest open issues, batch 11

### [#602 and #682: Keep unit normalization property-directed](https://github.com/parcel-bundler/lightningcss/issues/602)

- Category: CSS/SVG value grammar and unit normalization.
- Status: non-zero unit elision for SVG `stroke-width` is not implemented;
  although the property has generated metadata, its value is still parsed as a
  known-ID unparsed declaration. Invalid ordinary CSS such as `width:100` is
  preserved rather than silently repaired to `100px`.
- Guardrail: only a typed property grammar may prove that a unitless user unit
  and a CSS length are interchangeable. Never add or remove a non-zero unit in
  generic token minification, because doing so can change validity and cascade.

- [#682](https://github.com/parcel-bundler/lightningcss/issues/682) additionally
  asks for strict handling of `width:100`; RocketCSS keeps that invalid known
  value as an unparsed declaration. Deletion belongs to an explicit validator
  or sanitizer, not lossless parsing.

### [#617: Serialize zero media lengths safely for Safari](https://github.com/parcel-bundler/lightningcss/issues/617)

- Category: media-query browser compatibility.
- Status: RocketCSS currently canonicalizes legacy `(min-width:0)` to the MQ4
  range `(width>=0)`, which avoids the Safari 14 legacy-syntax bug. There is no
  browser-target model, so this conversion is unconditional.
- Guardrail: targets supporting range syntax may use `(width>=0)`; older targets
  that require legacy min/max syntax must receive `(min-width:0px)`. Never emit
  the affected `(min-width:0)` form. A target-aware serializer may need to retain
  or reconstruct whether legacy or range syntax is required.
- [#718](https://github.com/parcel-bundler/lightningcss/issues/718) requires
  omitted/default targets and an added Safari 14 range to compose rather than
  replace one another. The resolved target set must be queryable and versioned;
  until that model exists, RocketCSS cannot select the Safari-safe media form.

## 2026-07-16: oldest open issues, batch 12

### [#655: Preserve nested layers until safe lifting is available](https://github.com/parcel-bundler/lightningcss/issues/655)

- Category: nesting, cascade-layer lowering, and rule coalescing.
- Status: nested `@layer` blocks are retained under their original style rules,
  including direct declarations represented as `NestedDeclarations`. There is
  no lifting, same-name layer coalescing, or style-rule merging pass.
- Guardrail: future lifting must compose the parent selector and carry stable
  source-order provenance before grouping by the complete named layer path.
  Preserve first layer appearance, rule order, conditional context, anonymous
  layer identity, importance, and unlayered cascade. Identical declarations do
  not by themselves prove that two selectors or separated blocks may merge.

## 2026-07-16: oldest open issues, batch 13

### [#678: Preserve custom nested page regions](https://github.com/parcel-bundler/lightningcss/issues/678)

- Category: paged-media extension syntax and lossless recovery.
- Status: `@page` children are limited to the fixed `PageMarginBox` enum.
  Processor extensions such as `@footnote` and `@prince-overlay` are rejected
  in strict mode and discarded during recovery because there is no unknown
  nested page-region AST variant.
- Guardrail: represent known and unknown nested page regions in an extensible
  page-area node, retain each raw prelude and block, and round-trip authored
  order. Recovery must not silently delete processor-specific page content.

### [#690: Diagnose invalid pseudo-element continuations without data loss](https://github.com/parcel-bundler/lightningcss/issues/690)

- Category: invalid legacy selector diagnostics and recovery compatibility.
- Status: RocketCSS currently parses
  `[data-tooltip][data-inverted]:after .header` as a structured selector even
  though a pseudo-element must terminate its complex selector. Terminal
  pseudo-element validation is not implemented.
- Guardrail: strict parsing should report `InvalidSelector` at the illegal
  continuation. Recovery should retain the entire selector as `Unparsed` and
  preserve its declarations and neighboring selector-list entries; never
  silently delete or partially rewrite the authored selector.

## 2026-07-16: oldest open issues, batch 14

### [#744: Add typed support for the overlay property](https://github.com/parcel-bundler/lightningcss/issues/744)

- Category: evolving CSS Positioned Layout syntax.
- Status: `overlay` is currently preserved as a generic custom-ID unparsed
  declaration. It has no generated known property ID, typed keyword value, or
  typed parser dispatch.
- Guardrail: define the property once in metadata and parse `none | auto`
  without confusing it with the `overlay` blend-mode value. Variables and
  future values must fall back to a known-ID unparsed declaration, preserving
  authored transition and top-layer-related syntax.

## 2026-07-16: oldest open issues, batch 16

### [#788: Preserve CSS custom functions and mixins draft](https://github.com/parcel-bundler/lightningcss/issues/788)

- Category: evolving CSS Functions and Mixins draft syntax.
- Status: RocketCSS has no semantic custom-function or mixin implementation.
  Unknown at-rules and nested function tokens can preserve representative
  syntax, but draft behavior must not be treated as stable typed semantics.
- [#964](https://github.com/parcel-bundler/lightningcss/issues/964) requires
  visitor-driven `@apply` expansion to splice mixin declarations at the exact
  application point. Until custom-at-rule schemas and visitor replacement
  exist, retain `@mixin` and `@apply` syntax unchanged; a future expansion must
  never move a mixin `border-radius` after subsequent corner-radius overrides.
- Guardrail: retain complete preludes, parameter defaults, result bodies,
  invocation tokens, and source order. Add typed evaluation only against an
  explicit draft version, while unknown or newer constructs round-trip
  losslessly.

### [#821 and #828: Keep light-dark() sensitive to the effective color scheme](https://github.com/parcel-bundler/lightningcss/issues/821)

- Category: target-aware color-function lowering and cascade semantics.
- Status: RocketCSS preserves authored `light-dark()` tokens and distinct
  `color-scheme: inherit` and `color-scheme: normal` values; it performs no
  browser-target lowering. A variable-based space toggle inherited from an
  ancestor is not equivalent when a descendant changes its effective color
  scheme.
- [#828](https://github.com/parcel-bundler/lightningcss/issues/828) requests an
  escape hatch for generated light/dark variables. Any future lowering must be
  explicitly target-gated and configurable, and must retain `light-dark()`
  whenever the transform cannot model descendant color-scheme changes,
  inheritance, custom-property scope, and fallback behavior exactly.
- [#831](https://github.com/parcel-bundler/lightningcss/issues/831) requires the
  same shared color-function semantics inside border shorthands and
  `border-color`. A partially typed border parser must not select one branch
  or lose width, style, nested variables, importance, or future color syntax;
  retain the entire known-property unparsed declaration when equivalence is
  unavailable.
- [#873](https://github.com/parcel-bundler/lightningcss/issues/873) demonstrates
  the failure with light/dark custom properties defined on an ancestor and
  `color-scheme: only dark` or `only light` selected by descendants. Both
  `only dark` and `dark only` are valid authored orders and are currently
  retained as tokens. A `prefers-color-scheme` media query is not an
  equivalent replacement for element-level effective color-scheme selection.
- Guardrail: never collapse `inherit` and `normal`, and never replace a
  context-sensitive color choice with an inherited toggle merely because the
  immediate declaration appears equivalent.

## 2026-07-17: oldest open issues, batch 18

### [#904: Preserve invalid media queries and separately optimize proven false ranges](https://github.com/parcel-bundler/lightningcss/issues/904)

- Category: media-query recovery and compatibility optimization.
- Status: malformed feature values such as `1020 px` are preserved with their
  rule block, while contradictory but individually valid ranges are retained;
  RocketCSS has no satisfiability solver or dead-media elimination pass.
- Guardrail: strict parsing may diagnose malformed syntax, but recovery must
  retain the complete prelude and block. Only remove a query branch when all
  constraints are typed, comparable, and proven false; unknown features,
  mixed units, variables, calculations, and other comma-list branches are
  preservation barriers.

### [#907: Preserve complete selector lists in generated supports fallbacks](https://github.com/parcel-bundler/lightningcss/issues/907)

- Category: target-aware color fallback generation.
- Status: authored `@supports` containing `:root, :host` round-trips, but
  RocketCSS does not generate target-based color supports fallbacks.
- Guardrail: future generation must clone the complete ordered selector list
  as structured AST. The comma is OR semantics: never drop either selector or
  rewrite the list as compound `:root:host`; preserve conditional, layer,
  scope, and authored-order boundaries.

## 2026-07-17: issue batch 23

### [#975: Preserve pseudo-elements when lowering nested media rules](https://github.com/parcel-bundler/lightningcss/issues/975)

- Category: target-aware nesting lowering.
- Status: RocketCSS preserves native nesting and does not lower it for targets.
- Guardrail: any future lowering must keep pseudo-elements on their compound
  selector and must not wrap them in `:is()`, which invalidates selectors such
  as `.foo::after`.

### [#977: Preserve authored values for individual vendor prefixes](https://github.com/parcel-bundler/lightningcss/issues/977)

- Category: target-aware vendor-prefix generation.
- Status: RocketCSS preserves authored prefix/value pairs and their source
  order; it does not generate missing prefixes for browser targets.
- Guardrail: generated prefix declarations must use the intended fallback

## 2026-07-17: issue batch 24

### [#982: Keep pseudo-elements intact when lowering nested parent selectors](https://github.com/parcel-bundler/lightningcss/issues/982)

- Category: target-aware nesting lowering and pseudo-element validity.
- Status: RocketCSS preserves native nesting and does not lower it for browser
  targets.
- Guardrail: future lowering of `&` beneath a pseudo-element must retain the
  double-colon pseudo-element spelling and selector meaning; never rewrite
  `#b::after` to `#b:after` or discard the nested rule.

### [#987: Preserve unknown symbols in media calc() conditions](https://github.com/parcel-bundler/lightningcss/issues/987)

- Category: lossless media-query recovery for non-standard CSS Modules input.
- Status: unknown media conditions are stored as tokens and their rule bodies
  round-trip; RocketCSS does not resolve CSS Modules values.
- Guardrail: a bare symbol such as `baseUnit` in `calc()` is not a standard
  length. Keep the complete prelude and block, and treat it as a barrier to
  media evaluation, folding, removal, and reordering.

### [#990: Support valid before/after marker pseudo-element chains](https://github.com/parcel-bundler/lightningcss/issues/990)

- Category: pseudo-element compatibility syntax and selector validation.
- Status: RocketCSS preserves chained pseudo-elements but does not validate
  which continuations are legal or preserve source colon spelling.

## 2026-07-17: issue batch 25

### [#997: Avoid redundant legacy selector fallbacks](https://github.com/parcel-bundler/lightningcss/issues/997)

- Category: target-aware selector lowering.
- Status: RocketCSS preserves selector-list `:not()` but has no browser-target
  model or legacy `:is()`/`:-webkit-any()`/`:-moz-any()` fallback generation.
- Guardrail: preserve `:not(a,block)` whenever targets support that syntax.
  Generate legacy fallback only with target evidence, retain the modern branch
  and ordering, and never apply unconditional selector rewrites.

### [#998: Do not merge adjacent rules through forgiving selector wrappers](https://github.com/parcel-bundler/lightningcss/issues/998)

- Category: compatibility-safe cross-rule minification.
- Status: RocketCSS does not merge adjacent style rules; this preserves unknown
  pseudo selectors and their cascade behavior.
- Guardrail: a comma merge, `:is()`, or `:where()` wrapper requires proof that
  every selector arm has the same match set and specificity for all targets.
  Unknown/custom pseudos, recovered selectors, pseudo-elements, differing

## 2026-07-17: issue batch 26

### [#1006: Preserve legacy bare colors outside quirks mode](https://github.com/parcel-bundler/lightningcss/issues/1006)

- Category: quirks-mode color compatibility.
- Status: RocketCSS has no document mode and keeps an invalid bare value such
  as `333333` as lossless unparsed declaration tokens in standard CSS mode.
- Guardrail: do not guess bare numeric tokens as colors by default. A future
  explicit quirks mode must implement the complete legacy color algorithm and
  normalize only in that mode; strict mode must retain the original value.

### [#1012: Preserve auto animation duration for scroll timelines](https://github.com/parcel-bundler/lightningcss/issues/1012)

- Category: scroll-driven animation shorthand compatibility.
- Status: RocketCSS parses the animation shorthand into a typed `Animation`
  AST without expanding it or inserting `animation-duration:0s`; values the
  typed grammar cannot represent stay unparsed.
- Guardrail: typed shorthand expansion must represent `auto` duration before
  expanding scroll-driven animations. Never substitute `0s` for a value whose
  computed default is `auto`.

### [#1018: Preserve logical-property semantics before selector specificity](https://github.com/parcel-bundler/lightningcss/issues/1018)

- Category: target-aware logical-property lowering.
- Status: RocketCSS has no target-aware logical lowering and preserves native
  logical declarations.
- Guardrail: a legacy direction-selector fallback can raise specificity. Use
  `:where()` only when targets support it; otherwise retain native syntax or
  document the unavoidable semantic-versus-cascade tradeoff.

## 2026-07-17: issue batch 27

### [#1292: Chaining pseudo-class after ::picker is not allowed](https://github.com/parcel-bundler/lightningcss/issues/1292)

- Category: pseudo-element compatibility and selector validation.
- Status: RocketCSS parses `select::picker(select):not(:popover-open)` as a
  valid selector chain. The parser currently maps `::picker` to
  `PseudoElement::CustomFunction { name: "picker", .. }` and does not reject
  pseudo-classes following pseudo-elements. There is no chaining validation.
- Test coverage: `preserves_picker_pseudo_element_and_allows_chaining_pseudo_class`
  in `crates/parser/tests/parser.rs` verifies the full selector chain parses.
- Guardrail: if validation is tightened in the future, preserve the chained
  `:not(:popover-open)` and report a diagnostic rather than discarding it.

### [#1291: Unexpected display:flex transform with vendor prefixes](https://github.com/parcel-bundler/lightningcss/issues/1291)

- Category: target-aware vendor-prefix removal.
- Status: RocketCSS preserves authored `display: flex` alongside vendor-prefixed
  variants (`-webkit-box`, `-moz-box`, `-ms-flexbox`). It has no browser-target
  model and therefore does not remove prefix declarations.
- Guardrail: future target-aware prefix removal must keep the unprefixed
  declaration and drop only vendor-prefixed versions that are proven redundant
  for the target set. Never remove `display: flex` while keeping prefixed
  fallbacks.

### [#1286: Unexpected transformation of x units in image-set()](https://github.com/parcel-bundler/lightningcss/issues/1286)

- Category: target-aware resolution-unit lowering in image-set().
- Status: RocketCSS has typed `ImageSet` and `Resolution` AST nodes and
  preserves `1x` resolution units as authored. It has no browser-target model
  and therefore does not convert `x` to `dppx`.
- Guardrail: future target-aware unit conversion must only convert `x` to
  `dppx` when all targets support `dppx` in `image-set()`. When variables or
  unsupported targets are present, preserve authored `x` units.

### [#1283: animation-timeline:view() incorrectly merged into shorthand](https://github.com/parcel-bundler/lightningcss/issues/1283)

- Category: animation shorthand expansion and scroll-driven animation
  compatibility.
- Status: RocketCSS does **not** have this bug. Animation properties are parsed
  as unparsed tokens (no typed shorthand expansion). The minifier reorders
  values within each shorthand layer but does not merge `animation-timeline`
  into the `animation` shorthand. Minifier tests confirm `animation-timeline:
view()` and `animation-timeline: scroll()` are preserved as separate
  declarations.
- Test coverage: `preserves_cascade_sensitive_declaration_order` and
  `preserves_scroll_driven_animation_duration_auto_semantics` in
  `crates/minify/src/lib.rs`.

### [#1279: Simplify division of like units to unitless constant](https://github.com/parcel-bundler/lightningcss/issues/1279)

- Category: calc-value simplification optimization.
- Status: RocketCSS has typed calc values but does not perform unit-cancellation
  simplification (e.g., `calc(4px / 2px)` to `2`).
- Guardrail: any future simplification must only cancel identical units and
  preserve the original value when cancellation is not provable (variables,
  unknown units, mixed units). Never guess unit identity.

### [#1272: Support for color-interpolation-method in gradients](https://github.com/parcel-bundler/lightningcss/issues/1272)

- Category: evolving CSS Color Level 4 gradient syntax.
- Status: RocketCSS has no typed representation for `<color-interpolation-method>`
  in gradients. The `in <color-space>` syntax before gradient color stops is
  not parsed.
- Current preservation: gradient functions with unrecognized interpolation
  methods are retained as recursive function tokens in unparsed declarations.
- Guardrail: preserve the full gradient function tokens including the
  interpolation method, color space, and hue modifier until a typed
  representation exists. Never strip or reorder the method argument.

## 2026-07-17: issue batch 28

### [#1260: Incompatible selectors dropped after rule merging](https://github.com/parcel-bundler/lightningcss/issues/1260)

- Category: target-aware selector splitting and rule-merging ordering.
- Status: RocketCSS has no browser-target model, no selector splitting, and no
  rule merging. Incompatible selectors are preserved as authored.
- Guardrail: future rule merging must materialize incompatible selector rules
  before draining declarations into a merged rule. Never drop a selector branch
  because its declarations were consumed by a prior merge.

### [#1256: view-transition-name: match-element incorrectly transformed](https://github.com/parcel-bundler/lightningcss/issues/1256)

- Category: CSS Modules scoped-name transformation.
- Status: RocketCSS has no CSS Modules compilation. `view-transition-name` is a
  known property with typed values; `match-element` is a CSS keyword, not a
  custom identifier.
- Guardrail: future module scoping must treat `match-element` as a reserved
  keyword and never hash or rename it. `auto` and `none` have the same
  constraint.

### [#1255: Support Relative Alpha Colors (alpha() function)](https://github.com/parcel-bundler/lightningcss/issues/1255)

- Category: evolving CSS Color Level 5 relative color syntax.
- Status: RocketCSS has no typed `alpha()` function or relative color
  representation. The `alpha()` function is retained as a recursive function
  token in an unparsed declaration.
- Guardrail: preserve the complete `alpha()` function, its color argument, and
  optional alpha modifier until a typed representation exists.

### [#1252: Colour precision too high](https://github.com/parcel-bundler/lightningcss/issues/1252)

- Category: color value rounding and precision normalization.
- Status: RocketCSS has no color conversion or target-based color fallback
  generation. Authored color values are preserved as-is; no additional precision
  is introduced.
- Guardrail: future color conversion must round near-zero lab/oklab a/b channels
  to 0 when they are below the precision threshold. Never introduce floating-point
  noise that makes visually identical colors differ.

### [#1246: Logical border-radius polyfill hoisted into pseudo classes](https://github.com/parcel-bundler/lightningcss/issues/1246)

- Category: target-aware logical-property lowering and nesting.
- Status: RocketCSS has no logical-property lowering or browser-target model.
  `border-start-start-radius` and other logical border-radius properties are
  preserved as authored.
- Guardrail: future logical-property lowering must apply to the base rule and
  its nested pseudo-class branches independently. The `:lang()` wrapper required
  for direction detection must be added to each branch without removing the
  base-rule declaration.

### [#1244: ::details-content::before pseudo element chaining errors](https://github.com/parcel-bundler/lightningcss/issues/1244)

- Category: element-backed pseudo-element chaining compatibility.
- Status: RocketCSS parses `::details-content::before` successfully. The parser
  maps `::details-content` to `PseudoElement::Custom { name: "details-content" }`
  and chains `::before` via `PseudoElement::Before`. The parser does not reject
  pseudo-element chains.
- Test coverage: `preserves_details_content_chained_with_before_pseudo_element`
  in `crates/parser/tests/parser.rs` verifies the full chain parses.
- Guardrail: if validation is tightened, `::details-content` must be recognized
  as an element-backed pseudo-element that allows subsequent pseudo-elements and
  pseudo-classes, per the spec's element-backed exception.

## 2026-07-17: issue batch 29

### [#1239: Pseudo-element arg inside :has()/:is()/:where() is dropped](https://github.com/parcel-bundler/lightningcss/issues/1239)

- Category: selector validation and lossless preservation.
- Status: RocketCSS parses `video:not(:has(::backdrop))` as a valid selector
  chain and preserves the `::backdrop` pseudo-element inside `:has()`. The
  parser does not drop pseudo-element arguments from forgiving selectors.
- Test coverage: `preserves_pseudo_element_arg_inside_has_selector` in
  `crates/parser/tests/parser.rs` verifies the selector is preserved.
- Guardrail: never drop a pseudo-element argument from `:has()`, `:is()`, or
  `:where()` — an empty `()` is invalid and the original argument has defined
  browser behavior even if the spec is unsettled.

### [#1234: Transpile font-width to font-stretch](https://github.com/parcel-bundler/lightningcss/issues/1234)

- Category: target-aware property alias transpilation.
- Status: RocketCSS has no browser-target model and preserves `font-width` as a
  separate known property.
- Guardrail: `font-width` and `font-stretch` are aliases in the spec. A future
  transpilation pass may emit one or both based on targets, but never silently
  drop the authored property.

### [#1225: Minification reorders duplicate declarations with var()](https://github.com/parcel-bundler/lightningcss/issues/1225)

- Category: declaration deduplication and variable safety.
- Status: RocketCSS minifies declarations but does not reorder them across
  shorthand/longhand boundaries. `position`, `direction`, and `unicode-bidi`
  with `var()` are preserved in authored order.
- Guardrail: deduplication must never reorder a declaration containing `var()`
  after a shorthand that resets it. Variable-containing values are opaque and
  must preserve their cascade position.

### [#1224: New color-mix() features](https://github.com/parcel-bundler/lightningcss/issues/1224)

- Category: evolving CSS Color Level 5 syntax.
- Status: RocketCSS has no typed `color-mix()` representation. The function
  is retained as recursive function tokens in unparsed declarations.
- Guardrail: preserve the complete `color-mix()` tokens including optional
  interpolation method and variadic color arguments. When typed representation
  is added, the optional `<color-interpolation-method>` must default to `oklab`
  and variadic arguments must be supported.

### [#1222: @layer ordering declaration silently dropped](https://github.com/parcel-bundler/lightningcss/issues/1222)

- Category: cascade-layer ordering and rule merging.
- Status: RocketCSS preserves `@layer` statements and blocks in authored order.
  It does not merge or drop layer ordering declarations.
- Guardrail: `@layer one, two;` is a statement that establishes layer order and
  must never be dropped even when followed by `@layer` blocks. It is distinct
  from `@layer one { ... }` which defines layer contents.

### [#1221: font-family: monospace, monospace should not deduplicate](https://github.com/parcel-bundler/lightningcss/issues/1221)

- Category: font-family deduplication and browser quirks compatibility.
- Status: RocketCSS currently deduplicates `monospace, monospace` to
  `monospace` in its minifier. This is a known issue — the duplication is
  load-bearing for legacy browser font-size behavior where `monospace` alone
  defaults to a smaller size.
- Current behavior: the `font_family_deduplication_is_configurable` test
  already allows disabling deduplication via `DEDUPLICATE_LISTS` flag.
- Guardrail: `monospace` appearing as the sole family after deduplication
  must be preserved as `monospace, monospace` to avoid changing font-size
  rendering in browsers. This applies only to `monospace` as the only family;
  `monospace, serif` need not duplicate.

### [#1211: :-webkit-any(:lang()) and :-moz-any(:lang()) rules dropped](https://github.com/parcel-bundler/lightningcss/issues/1211)

- Category: target-aware vendor-prefix selector removal.
- Status: RocketCSS preserves `:-webkit-any()` and `:-moz-any()` as typed
  `SelectorComponent::Any` selectors. It has no browser-target model and
  therefore does not drop them.
- Guardrail: future target-aware prefix removal must only drop vendor-prefixed
  selectors when the unprefixed equivalent is supported by all targets. When
  all rules containing a selector would be dropped, preserve the complete rule.

### [#1210: Support multiple comma-separated queries in @container](https://github.com/parcel-bundler/lightningcss/issues/1210)

- Category: evolving CSS Containment Level 3 syntax.
- Status: RocketCSS has typed `@container` rule support. Multiple comma-separated
  container queries are not yet parsed.
- Current preservation: `@container` rules with multiple comma-separated
  queries may fall back to unparsed prelude tokens.
- Guardrail: comma-separated queries in `@container` represent OR semantics
  and must be parsed as a list of conditions. Until supported, preserve the
  complete prelude.

### [#1207: Property reordering causes font shorthand to override longhands](https://github.com/parcel-bundler/lightningcss/issues/1207)

- Category: minification property ordering and cascade safety.
- Status: RocketCSS does not reorder declarations across shorthand/longhand
  boundaries. The `font` shorthand and `font-variant-numeric` longhand are
  preserved in authored order.
- Guardrail: a shorthand must never be moved after a longhand it resets.
  `font` resets all `font-variant-*` sub-properties, so `font-variant-numeric`
  must remain after `font` in the cascade.

### [#1214: transition shorthand incorrectly combines inherit](https://github.com/parcel-bundler/lightningcss/issues/1214)

- Category: shorthand expansion with CSS-wide keywords.
- Status: RocketCSS parses transition properties as unparsed tokens and does
  not expand or collapse shorthands. `transition: inherit` and
  `transition-property: bar` are preserved as separate declarations.
- Guardrail: `inherit`, `initial`, `unset`, and `revert` are CSS-wide keywords
  that cannot be meaningfully combined with longhand sub-properties. A shorthand
  with a CSS-wide keyword must not be expanded or merged with longhands.

### [#1202: :has-slotted is flagged as invalid](https://github.com/parcel-bundler/lightningcss/issues/1202)

- Category: selector validation and evolving spec syntax.
- Status: RocketCSS parses `:has-slotted` as `PseudoClass::Custom` and
  preserves it without warnings. The parser has no validation step that flags
  unknown pseudo-classes.
- Test coverage: `preserves_has_slotted_pseudo_class` in
  `crates/parser/tests/parser.rs` verifies the pseudo-class is preserved.
- Guardrail: `:has-slotted` is a valid pseudo-class per the CSS Shadow Parts
  spec. Never suggest it should be a pseudo-element or flag it as a typo.

## 2026-07-17: issue batch 30

### [#1182: place-self shorthand incorrectly collapses justify-self:auto](https://github.com/parcel-bundler/lightningcss/issues/1182)

- Category: shorthand expansion and cascade safety.
- Status: RocketCSS parses `align-self` and `justify-self` as separate known
  properties and does not collapse them into `place-self`. No shorthand
  expansion or collapse exists for these properties.
- Guardrail: `place-self: self-start` is not equivalent to `align-self:
self-start; justify-self: auto` because `auto` resolves to `stretch` in grid
  layout. Never collapse distinct longhands into a shorthand without proving
  the default values match.

### [#1177: CSS Parsing error for ::scroll-marker](https://github.com/parcel-bundler/lightningcss/issues/1177)

- Category: evolving CSS Overflow Module Level 5 syntax.
- Status: RocketCSS parses `::scroll-marker`, `::scroll-marker-group`, and
  `::scroll-button` as `PseudoElement::Custom`, preserving them losslessly.
  No typed AST variants exist for these experimental pseudo-elements.
- Test coverage: `preserves_scroll_button_and_scroll_marker_pseudo_elements`
  in `crates/parser/tests/parser.rs`.
- Guardrail: experimental pseudo-elements must round-trip as custom variants
  until the spec stabilizes. Never flag them as errors or suggest they are
  pseudo-classes.

### [#1175: Without targets nesting produces invalid CSS](https://github.com/parcel-bundler/lightningcss/issues/1175)

- Category: CSS nesting serialization and target-aware lowering.
- Status: RocketCSS preserves native nesting without lowering. It does not add
  `&` to bare child selectors and does not produce invalid output.
- Guardrail: nesting serialization must distinguish authored implicit nesting
  from explicit `&`. Bare class and type selectors must remain unchanged by
  default. Only target-aware lowering may add `&` for forms unsupported by
  those targets.

### [#1146: border-image shorthand incorrectly parsed as empty value](https://github.com/parcel-bundler/lightningcss/issues/1146)

- Category: shorthand expansion and lossless preservation.
- Status: RocketCSS has no typed `border-image` shorthand expansion. The
  shorthand and its longhands are preserved as authored.
- Guardrail: `border-image: none; border-image-source: unknown` must not
  produce an empty `border-image` value. When a longhand value is unknown or
  invalid, preserve the complete declaration pair.

### [#1103: Keyframe including invalid code is removed](https://github.com/parcel-bundler/lightningcss/issues/1103)

- Category: error recovery and keyframe rule preservation.
- Status: RocketCSS preserves `@keyframes` rules with their keyframe selectors
  and declarations. Invalid `@supports` nested inside keyframes would be
  rejected in strict mode but preserved in recovery.
- Guardrail: browsers ignore invalid constructs inside keyframes (like
  `@supports`) but preserve the valid declarations. Recovery must preserve the
  valid keyframe content while discarding only the invalid nested rule.

### [#1084: CSS properties sorting breaks cascade](https://github.com/parcel-bundler/lightningcss/issues/1084)

- Category: minification property ordering and cascade safety.
- Status: RocketCSS does not reorder declarations across shorthand/longhand
  boundaries. `transition-behavior: allow-discrete` remains after `transition`
  shorthand in authored order.
- Guardrail: `transition` shorthand resets `transition-behavior`. A longhand
  that appears after the shorthand must not be moved before it. Declaration
  reordering must be constrained to same-property or cascade-safe groups.

### [#1073: Nested :has(+ &) is rewritten incorrectly](https://github.com/parcel-bundler/lightningcss/issues/1073)

- Category: nesting lowering and relative selector correctness.
- Status: RocketCSS preserves native nesting without lowering. `:has(+ &)` is
  preserved as authored.
- Guardrail: `:has(+ &)` in a nested context means "the parent has a preceding
  sibling matching this". Lifting this to `:has(+ .container .style-previous)`
  changes the selector semantics. Any future nesting lowering must prove
  selector equivalence before lifting.

### [#1072: CSS Parsing error for :scroll-button](https://github.com/parcel-bundler/lightningcss/issues/1072)

- Category: evolving CSS Overflow Module Level 5 pseudo-class syntax.
- Status: RocketCSS parses `:scroll-button` as `PseudoClass::Custom` and
  preserves it losslessly. The parser has no typed variant for this
  experimental pseudo-class.
- Test coverage: included in `preserves_scroll_button_and_scroll_marker_pseudo_elements`.
- Guardrail: `:scroll-button` is a pseudo-class (not a pseudo-element). Never
  suggest the `::` spelling or flag it as a typo.

### [#1069: Fails to parse range syntax in container style queries](https://github.com/parcel-bundler/lightningcss/issues/1069)

- Category: evolving CSS Conditional Rules Level 5 syntax.
- Status: RocketCSS has typed `@container` rule support but does not parse
  range syntax like `style(1em < 20px)` in container queries. Unknown prelude
  syntax falls back to generic token preservation.
- Guardrail: `@container style(1em < 20px)` must be preserved with its
  complete prelude and block. Until range syntax is supported, treat unknown
  style-query syntax as a preservation barrier.

### [#1067: Adjacent vendor pseudo-elements aren't merged](https://github.com/parcel-bundler/lightningcss/issues/1067)

- Category: rule merging and vendor-prefix pseudo-element identity.
- Status: RocketCSS does not merge adjacent style rules. Identical vendor
  pseudo-element selectors are preserved as separate rules.
- Guardrail: `::-webkit-scrollbar` is a vendor-specific pseudo-element. Rule
  merging must prove selector identity before collapsing declarations. Never
  merge rules with different pseudo-elements or different selector lists.

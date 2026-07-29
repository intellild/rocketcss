# Cross-rule declaration merging: non-goals

## Document map

- [Overall design](./overall.md)
- [S1: same-selector coalescing](./s1-same-selector-coalescing.md)
- [S2: declaration-effect pruning](./s2-declaration-effect-pruning.md)
- [S3: selector partial factoring](./s3-selector-partial-factoring.md)
- [S4: AST reification planning](./s4-ast-reification-planning.md)
- [S5: AST reification commit](./s5-ast-reification-commit.md)
- [Non-goals](./non-goal.md)
- [Detailed state machine](./detailed-state-machine.md)
- [Pseudocode](./pseudo-code.md)

## Conditional at-rules

The initial design does not:

- compare rules whose complete conditional at-rule context stacks differ;
- infer condition equivalence, implication, overlap, union, or negation;
- reorder condition terms;
- commute nested conditional frames;
- distribute, combine, or remove conditional frames;
- lift a style rule into or out of a conditional at-rule;
- merge non-adjacent conditional blocks;
- collapse repeated nested `@container` frames; or
- interpret whitespace or comments as condition identity.

Typed structural equality after existing canonicalization is the only accepted
condition equality.

## Cascade contexts

Declaration histories carry exact layer, origin, and cascade-phase keys, but
the pass does not interpret their semantics. Separately authored `@layer`
blocks remain distinct contexts even when their names are equal. The design
does not attempt to model layer order, important-layer reversal, cross-origin
ordering, phase ordering, or `revert-layer`.

`@scope` and `@starting-style` remain retained barriers. Scope proximity and
starting-style application semantics are not modeled.

## Non-style and global at-rules

The pass does not merge or infer selector identity for:

- `@keyframes`;
- `@font-face`;
- `@property`;
- `@counter-style`;
- `@font-palette-values`;
- `@page`;
- `@position-try`;
- `@import`;
- `@namespace`;
- `@charset`; or
- unknown/custom at-rules.

These rules have identifier-, descriptor-, ordering-, or grammar-specific
semantics and require separate typed designs.

## Nesting and wrapper transformations

The initial design does not:

- flatten native CSS nesting;
- lift a nested style rule into an ancestor list;
- mutate an existing rule's local or ancestor selector;
- create S1/S3 edges across sibling lists;
- optimize legacy `CssRule::Nesting` (`@nest`) wrappers;
- erase or synthesize wrapper kinds without an explicit emission plan;
- treat explicit `&` as equivalent to parent-match declarations; or
- provide cross-rule identity for recovered selectors whose nesting semantics
  cannot be validated.

`@nest` remains a retained adjacency and history barrier until
wrapper-preserving emission is designed.

## Selector transformations

The design does not:

- infer selector-list equivalence from reordered selector arms;
- split one authored selector list into independently owned selector arms;
- move selector AST ownership out of a live endpoint;
- factor selectors when arena-aware deep materialization is unavailable;
- defer synthesized-selector canonicalization to a second minify pass; or
- treat a parsed selector as valid without effective-selector validation.

For exact selector-list identity, `a,b` is different from `b,a` and from `a`.

## Declaration transformations

The design does not:

- remove a declaration based only on property-name equality;
- treat `HashMap<PropertyId, Declaration>` as the final semantic result of a
  declaration sequence;
- eagerly replace every authored shorthand with physical longhand AST nodes;
- collapse an ordered vendor or target fallback chain into one map value;
- discard vendor fallbacks that may still be required by configured targets;
- expand a shorthand containing variables or recovered/unknown syntax;
- assume a logical property overrides a physical property without proving
  writing-mode and direction behavior;
- treat `all` as resetting custom properties, `direction`, or `unicode-bidi`;
- optimize `revert` or `revert-layer` without the required cascade context;
- feed an S4 representation-only shorthand recombination back into the S1-S3
  semantic fixed point; or
- commit a partial declaration edit without a typed, lossless replacement
  plan.

Unknown declaration relationships produce `NoChange`.

Virtual shorthand expansion belongs to the declaration-effect IR. It preserves
the authored occurrence as an origin and records opaque affected-property
bundles when lossless component values cannot be derived.

## Structural movement

The design does not:

- merge non-adjacent different-selector rules;
- move declarations across retained child content;
- let an edge skip a live `NestedDeclarationsRule`, supported conditional
  wrapper, opaque node, or unsupported at-rule;
- merge rules with different `vendor_prefix` values;
- merge rules with different wrapper kinds or selector serialization contexts;
- expose a temporary bypass edge during S3 commit; or
- append a synthesized history entry without placing it at its semantic source
  position.

## Cleanup

The design does not:

- remove a style-rule ancestor that contains any retained opaque child;
- remove a selector-retired S1 storage node while another output owner still
  references its declaration block;
- treat an unsupported at-rule as empty merely because this pass cannot inspect
  it; or
- retain a selector-dead parent's descendants as independently outputting
  rules.

S4 determines logical retention and produces a complete AST reification plan,
including the lossless declaration representation for every non-empty IR. It
does not serialize CSS. S5 only commits that plan into the stylesheet AST.
Code generation remains outside the minify pipeline and never consumes merge IR
directly.

## Deferred implementation choices

The following choices belong to later implementation design and must preserve
the invariants in the other documents:

- packed versus direct state representation;
- dense known-property slots versus hash-indexed declaration effects;
- stable `RuleId` allocation;
- live-adjacency representation;
- interning for selector and conditional-context keys;
- ordered tree, order-maintenance labels, or history rebuild for semantic
  source order;
- eager versus lazy history maintenance;
- physical conditional-block coalescing versus logical regions;
- concrete storage encoding for metadata-generated affected-longhand and
  may-alias sets;
- target-aware selector compatibility;
- profitability thresholds;
- S4 representation-profitability and S5 physical-compaction policy;
- small-vector and hash-table thresholds; and
- concrete combined-span encoding after selector and declaration origins have
  been preserved.

These are not permission to weaken semantic identity, source ordering,
candidate invalidation, or lossless serialization.

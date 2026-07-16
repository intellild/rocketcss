# Adjacent Declaration Block Merge Design

## Goal

Merge declaration blocks from physically adjacent style rules when their selectors are structurally identical. The merged blocks must behave exactly like one declaration block for minification, while retaining their arena-backed storage and source order.

This first version intentionally does not merge different selectors with equal declarations, partially merge selector lists, or merge non-adjacent rules.

## Core Model

Declaration deduplication operates on a logical declaration sequence rather than on one concrete block:

- A normal declaration block forms a one-block sequence.
- Consecutive eligible style rules form a multi-block sequence.
- The existing declaration minifier processes the live declarations in sequence order without resetting its IR at a block boundary.

Consequently, a declaration block boundary has no special deduplication semantics. Exact duplicate removal, physical box shorthand/longhand handling, fallback preservation, logical-property barriers, and importance handling remain shared with the single-block implementation.

## AST Representation

`DeclarationBlock<'a>` gains an optional backward link to the preceding block in its merge sequence:

```rust
previous_merged: Option<Ref<'a, DeclarationBlock<'a>>>
```

For three merged rules, the links are:

```text
block C -> block B -> block A
```

The link is transient minifier/codegen state:

- visitors skip it;
- structural declaration-block equality ignores it;
- it is initialized to `None` by `DeclarationBlock::new`;
- mutation is exposed through a pinned API so adding a link never moves the block;
- the allocator provides a lifetime-correct immutable `Ref` for a pinned arena allocation.

The earlier style rule is suppressed with the existing selector tombstone representation after its declarations have been linked into the later rule. No new `CssRule` tombstone variant is introduced.

## Eligibility and Selector Matching

Two declaration blocks join the same sequence only when their owning style rules are physically adjacent siblings and:

- both rules have at least one live selector;
- their selector lists are structurally equal;
- their vendor prefixes are equal;
- joining them does not cross a nested-content ordering barrier;
- neither link would create a cycle or repeat an existing merge.

Matching is local to one sibling rule list. Identical textual selectors under different parent rules or nesting contexts are never compared with each other.

An unrelated style rule, at-rule, or other `CssRule` variant ends the current sequence. The pass does not search forward over such a rule, even if that rule currently emits no text.

## Shared Declaration IR

The existing declaration minifier uses block-local integer indices. It will be generalized to address a declaration by logical location:

```text
DeclarationLocation = (block, declaration_index)
```

All declaration-map and box-family IR entries store a location. The sequence abstraction provides the operations needed by the existing algorithms:

- iterate live declarations in source order;
- inspect declaration value and importance;
- replace a declaration at a location with a tombstone;
- mutably access two distinct declaration locations for shorthand folding.

The single-block path uses the same abstraction with one block. The multi-block path supplies all blocks in the eligible run. There is no separate cross-block duplicate algorithm.

IR state is cleared only at the start of a declaration sequence. It is not cleared between eligible blocks. Statistics continue to count every declaration replaced by a tombstone, regardless of which block owns it.

## Nested Content

Nested content participates in ordering and therefore defines merge barriers.

RocketCSS represents declarations before the first nested rule in `StyleRule::declarations`. Declarations appearing after nested content are represented by `CssRule::NestedDeclarations` entries in the rule list. This structure must not be flattened across a nested rule.

The first-version rules are:

- Each sibling rule list is optimized independently and recursively.
- A nested rule between declaration blocks is a hard sequence barrier.
- A preceding same-selector style rule with nested children cannot be suppressed and merged into the following rule, because doing so would move its declarations across those children.
- A following same-selector style rule may contain nested children. Earlier declarations are emitted before that rule's own declarations and before its children, preserving source order.
- Once a rule with nested children becomes the live tail, it cannot merge forward into another style rule.
- `NestedDeclarations` blocks are minified as their own blocks. They do not link across neighboring nested style or at-rules.
- Nested `@media`, `@supports`, style, and other rule containers recursively run the same sibling-list algorithm inside their own scope.

For example, this merge is valid:

```css
.a { color: red }
.a {
  color: blue;
  & .child { display: block }
}
```

The result retains declaration order before the nested child. This merge is not valid:

```css
.a {
  color: red;
  & .child { display: block }
}
.a { color: blue }
```

Merging it at the second rule would move `color: red` past the nested child.

## Traversal

Value-level and declaration-level normalization occurs before sibling sequences are formed. A list-aware pass then:

1. Recursively processes child rule lists.
2. Scans the current sibling list in source order.
3. Builds maximal eligible same-selector runs, respecting nested barriers.
4. Runs the shared declaration IR over each run as one logical sequence.
5. Links each later block to the previous block and tombstones the previous owning rule's selectors.

The pass is idempotent. Tombstoned owners are not eligible as new sequence heads, and an already-linked tail is not relinked into itself.

## Code Generation

The live tail owns the output for the complete sequence. Code generation:

1. Follows `previous_merged` links from the tail to the head.
2. Collects the chain iteratively.
3. Emits blocks in reverse traversal order, restoring source order.
4. Skips declaration tombstones in every block.
5. Applies the final-semicolon policy to the last live declaration in the entire sequence, not independently to each block.

Iterative traversal avoids recursion depth proportional to the number of adjacent rules.

Rule-list code generation skips fully tombstoned style rules before calculating whitespace, preventing empty lines in pretty output. A linked sequence whose declarations are all removed still preserves the live tail when it owns nested children.

## Configuration

Adjacent same-selector merging is controlled by a dedicated minify option and is enabled in the default option set. Disabling it retains the existing independent-block behavior and produces no merge links or selector tombstones.

## Verification

Tests cover:

- two- and three-block same-selector sequences;
- declaration source order across blocks;
- exact duplicates across a block boundary;
- equal properties with different values;
- different `!important` states;
- shorthand/longhand optimization across a boundary;
- fallback and logical-property barriers across a boundary;
- different and partially overlapping selectors;
- non-adjacent same-selector rules;
- a preceding rule with nested children as a barrier;
- a following rule with nested children as a valid tail;
- `NestedDeclarations` ordering barriers;
- independent merging inside nested rule lists;
- the disabled option path;
- repeated minification without cycles or additional changes;
- compact and pretty code generation without phantom separators.


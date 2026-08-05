# Flat source-order AST IR plan

## Chosen rule representation

Rules live in one `RadixIndexArena<RuleRecord>`, allocated in lexical preorder.
The tree does not use `RuleListId`, a rule-list store, child vectors, linked
siblings, source-order links, descendants vectors, or per-rule Radix ranges.

```rust,ignore
struct RuleRecord<P> {
    payload: P,
    parent: Option<RuleId<P>>,
    nested_rule_count: u32,
    declaration_block: Option<DeclarationBlockId<P>>,
    revision: u32,
    live: bool,
}
```

`nested_rule_count` counts every physical descendant record, excluding the
rule itself and including tombstones. In lexical preorder this one value is
enough to derive all tree movement:

```text
first child       = next(rule)
next sibling      = advance(child, 1 + child.nested_rule_count)
after subtree     = advance(rule, 1 + rule.nested_rule_count)
root span         = the complete rule arena
```

## Parser flow

```text
parse rule prelude
       ↓
AST append_rule(parent, payload)
  allocate owner immediately in source order
  increment nested_rule_count on every ancestor
       ↓
parse the rule body
  nested rules append with owner as parent
  post-nesting declarations use NestedDeclarations rules
       ↓
finish payload span and declaration block
```

The parser does not create child lists or manipulate subtree spans itself.

## Public AST boundary

Storage mechanics remain private to the AST crate. Other crates use:

- `root_rules()` for live top-level rules;
- `nested_rules(parent)` for live direct children;
- `has_nested_rules(parent)` for semantic child checks;
- `rules_in_source_order()` and scoped source-order transforms for lexical
  scans;
- `root_rule_edges()` / `nested_rule_edges(parent)` for opaque, revisioned
  adjacency contexts produced by one parent-list traversal; and
- validated mutation transactions for insertion, merging, and retirement.

`nested_rule_count`, subtree-tail calculation, Radix `advance_id`, tombstone
scanning, and ID-remap repair are not cross-crate APIs.

## Structural mutation

### Insert

1. Validate the live left endpoint and semantic parent.
2. Find the physical tail of the endpoint's complete subtree.
3. Find the next physical boundary, including retained tombstones.
4. Insert at the final Radix position.
5. Repair any local sibling-ID relabel throughout AST-owned references.
6. Increment every ancestor's physical descendant count.

### Retire

1. Validate that the rule has no live nested syntax.
2. Capture live semantic neighbors.
3. Mark the rule and its declaration block dead.
4. Keep their arena records and ancestor counts unchanged.

### Validate

A single lexical scan maintains a stack of `(ancestor, subtree_end)` pairs.
For every rule it verifies:

- the declared parent is the innermost active ancestor;
- the subtree end fits inside the arena and its parent's subtree; and
- declaration-block ownership remains bidirectional and live.

## Declaration representation

Authored declarations are primary values in one semantic `RadixIndexArena`.
Each block stores a `RadixRange<DeclarationSlot>`; `len == 0` represents an
empty block without consulting its placeholder start ID. Stable transformed
batch insertion preserves existing IDs while keeping every block in one
contiguous semantic range. This declaration representation is independent of
the rule subtree-count design.

## Verification gates

- parser tests prove primary rule IDs follow lexical preorder;
- AST tests cover nested spans, insertion after a nested subtree, tombstones,
  parent mismatches, and local Radix relabel repair;
- codegen and visitor tests traverse only semantic AST APIs;
- Nano tests prove direct-edge scheduling and retirement preserve output;
- `cargo fmt --all`, workspace tests, and workspace clippy pass.

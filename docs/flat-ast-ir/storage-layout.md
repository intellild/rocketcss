# Storage layout

## Compilation-owned stores

`Compiler` owns the source, source map, atom pool, and the stores that form one
parsed compilation. It is constructed without an allocator argument.
`StyleSheet` is a root ID plus stylesheet metadata; it does not own a recursive
object graph.

```rust,ignore
struct Compilation {
    syntax: DenseStore<SyntaxNode, RuleId>,
    style_rules: DenseStore<StyleRuleData, StyleRuleId>,
    media_rules: DenseStore<MediaRuleData, MediaRuleId>,
    selectors: DenseStore<SelectorData, SelectorId>,
    declaration_blocks: DenseStore<DeclarationBlockHeader, DeclarationBlockId>,
    declarations: DenseStore<DeclarationSlot, DeclarationId>,
    effective_keys: EffectiveKeyInterner,
}

struct StyleSheet {
    first_rule: Option<RuleId>,
    rule_count: u32,
}
```

`SyntaxNode` should remain compact. A tagged `RuleId` plus per-rule-kind payload
stores avoids making every rule slot as large as the largest inline enum
variant. The exact tag encoding is an implementation and benchmark decision.

## Flat tree topology

Preorder alone cannot answer direct-sibling queries because descendants are
interleaved. Each rule therefore stores enough topology to skip a subtree and
follow its owning list:

```rust,ignore
struct SyntaxNode {
    parent: Option<RuleId>,
    next_sibling: Option<RuleId>,
    subtree_end: RuleId,
    payload: RulePayloadId,
    flags: SyntaxNodeFlags,
}
```

`subtree_end` is the first ID after the subtree. Direct children are found from
the parent's first child and `next_sibling`; a complete subtree remains a dense
preorder interval. Equivalent encodings are acceptable if these operations
remain constant-time and do not require a recursive walk.

The flat topology replaces `rocketcss_common::boxed::Box`, boxed rule payloads,
and address-based identity. Visitors receive IDs plus a `Compilation`/store
view. Structural mutable visitors emit rewrite operations instead of retaining
mutable references while stores grow.

## Source-order allocation invariant

For every authored token belonging to an AST entity, the corresponding slot is
allocated when the parser reaches that entity. In particular, declaration IDs
must increase in lexical property order across the entire stylesheet.

```css
a {
  x: 1;
}
a {
  y: 1;
  & b {
    z: 1;
  }
}
```

The declaration tape must be `x, y, z`. Allocating the parent declaration block
only after recursively parsing its children would produce `x, z, y`; that is
forbidden even if serialization could later hide the difference.

An empty leading declaration run captures its cursor before a nested child:

```css
a {
  x: 1;
}
a {
  & b {
    z: 1;
  }
  w: 1;
}
```

The second `a` has an empty leading block at the position before `z`; `w`
belongs to a later `NestedDeclarationsRule` block. No range extension may
mistake `z` for part of either `a` declaration sequence.

This invariant is enforced by parser-level tests that inspect IDs and ranges,
not inferred from generated CSS. The implementation plan defines the complete
test matrix.

## Declaration tape and block headers

```rust,ignore
struct DeclarationBlockHeader {
    offset: u32,
    len: u32,
    effective_key: EffectiveKeyId,
    flags: DeclarationBlockFlags,
}

struct DeclarationSlot {
    value: Declaration,
    source: SourceLocationId,
}
```

A block initially owns the half-open interval
`offset..offset + len`. Importance and tombstones should be packed into
sidecars or compact flags so adding one boolean does not inflate every aligned
`Declaration` value.

A block header has exactly one semantic owner. If shared declaration storage is
ever required, model the owner edge explicitly:

```rust,ignore
struct DeclarationOccurrence {
    block: DeclarationBlockId,
    effective_key: EffectiveKeyId,
}
```

Do not make a freely copyable block ID imply that one physical header can carry
different contexts.

Two block ranges can be coalesced in place only when:

1. their semantic order equals their physical order;
2. the left range ends at or before the right range begins; and
3. every slot in a nonempty gap is already a tombstone owned by retired output.

Only exactly adjacent ranges allow the trivial `len += other.len` operation.
Otherwise the logical declaration sequence retains multiple ranges until S5
reification builds a compact tape. Live foreign declarations must never be
absorbed into a merged range.

## Selectors and values

Selector names continue to use compiler-scoped `Atom` values. Complete
selector values may be hash-consed to a canonical `SelectorValueId`, with exact
equality after hash-bucket selection. Occurrence IDs and canonical value IDs
are distinct: two authored `a` selectors have two occurrence locations but may
share one value identity.

Flattening each small selector component into a separately indexed store can
cost more indirection than it saves. The default candidate is a flat selector
component tape whose selector headers store `(offset, len)`. Compare that
against per-component IDs using representative parse, minify, and codegen
benchmarks before fixing the public layout.

## Removing `Box` and the AST allocator

The target AST has no stable-address requirement:

- stores own values directly;
- relationships use typed IDs;
- store growth may relocate memory without invalidating IDs; and
- reification builds new stores rather than mutating an address-linked tree.

Consequently, `rocketcss_common::boxed::Box` and
`rocketcss_common::Allocator` become dead infrastructure once their last node
type migrates. The migration removes them from parser constructors, AST types,
visitors, minifiers, codegen, and `Compiler::new`; it must not retain a
compatibility allocation layer.

`StringPool` changes from allocator-backed storage to an owned compiler store.
Its internal backing allocation is not AST ownership and must not expose AST
addresses. A future analysis may introduce an independent scratch facility only
when a benchmark justifies it; such a facility is outside the AST API and does
not preserve the existing allocator.

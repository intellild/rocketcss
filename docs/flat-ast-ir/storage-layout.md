# Storage layout

## Compiler ownership

`Compiler` owns one compilation's source, source map, atom pool, arena, AST
stores, and semantic interners. `StyleSheet` is a lightweight root-list handle;
it does not own nested Rust vectors that must later be rediscovered.

```rust,ignore
struct Compilation<'ast> {
    allocator: Allocator,
    string_pool: StringPool<'ast>,

    rules: RuleStore<'ast>,
    declaration_blocks: DeclarationBlockStore<'ast>,
    declarations: DeclarationStore<'ast>,

    selector_values: SelectorValueStore<'ast>,
    context_paths: ContextPathInterner,
    effective_keys: EffectiveKeyInterner,
}

type RuleStore<'ast> =
    RadixIndexArena<'ast, ArenaBox<'ast, CssRule<'ast>>>;

type DeclarationBlockStore<'ast> =
    RadixIndexArena<'ast, DeclarationBlock<'ast>>;

struct StyleSheet {
    root_rules: RuleListId,
}
```

Large payloads may remain arena boxed so local SoA insertion moves compact
pointers. Stable relationships use typed IDs, never arena addresses.

## Rules and topology

Rules are allocated in lexical preorder. A rule stores the structural facts
that cannot be inferred from global order alone:

```rust,ignore
struct CssRule<'ast> {
    parent: Option<RuleId>,
    parent_list: RuleListId,
    previous_sibling: Option<RuleId>,
    next_sibling: Option<RuleId>,
    first_child_list: Option<RuleListId>,
    payload: CssRulePayload<'ast>,
    revision: u32,
    flags: CssRuleFlags,
}
```

`RuleId` determines stable identity and source order. The links determine
direct-sibling adjacency across nested subtrees. A structural rewrite updates
the local topology transaction together with Radix insertion or retirement.

Rule lists keep compact endpoints and counts as needed for validation and
codegen, but they do not own a second vector of rule values.

```rust,ignore
struct RuleList {
    first: Option<RuleId>,
    last: Option<RuleId>,
    live_len: u32,
}
```

## Declaration blocks

Every live declaration syntax position owns one mutable block:

```rust,ignore
struct DeclarationBlock<'ast> {
    owner: DeclarationBlockOwner,
    declarations: DeclarationList<'ast>,
    effective_key: EffectiveKeyId,
    revision: u32,
    flags: DeclarationBlockFlags,
}

enum DeclarationBlockOwner {
    Rule(RuleId),
    Keyframe(KeyframeId),
    Descriptor(DescriptorOwnerId),
}
```

Ownership is unique. Two simultaneously live syntax positions do not share one
mutable block under different selectors or wrapper contexts. S1 may retire one
owner and move declarations into another block, but it updates owner/topology
and EffectiveKey invariants in the same transaction.

The block's `EffectiveKeyId` is ordinary AST data. Nano reads it directly; it
does not reconstruct selector and wrapper paths into
`DeclarationBlockEntry` values.

## Declaration storage

Authored declarations are appended in lexical source order to a dense primary
tape. A declaration run captures its exact range as it is parsed:

```rust,ignore
struct DeclarationRange {
    offset: u32,
    len: u32,
}

enum DeclarationList<'ast> {
    Range(DeclarationRange),
    Local4(LocalPropertySet<'ast>),
    Overflow(ArenaVec<'ast, ArenaBox<'ast, Declaration<'ast>>>),
}
```

The range is a physical slice of the authored declaration tape. Nested rules
close the current run before their declarations are parsed, so an enclosing
block never absorbs a descendant's properties.

Small synthesized blocks may use `Local4`, whose property IDs reuse the low
two bits of their `DeclarationBlockId`. Larger or nonconsecutive transformed
sequences use a complete overflow list. S4 may choose the smallest lossless
representation, but Nano never needs a second global reification store solely
to restore order.

## Source-order allocation invariant

The parser appends a declaration immediately after it parses that declaration.
For:

```css
a {
  --before: 0;
  & b {
    --nested: 1;
  }
  --after: 2;
}
```

the declaration tape is exactly `--before, --nested, --after`. The parent owns
the first range. The post-nesting declarations belong to the distinct
`NestedDeclarationsRule` syntax position and therefore to a second
`DeclarationBlock` with its own range. No block range crosses the nested rule;
a later transform that combines them must materialize a complete ordered
replacement.

Parser tests inspect IDs/ranges directly. Serialization alone is insufficient
because an incorrect physical allocation may still print correctly before a
merge exposes the bug.

## Effective-key ownership

While parsing, the compiler maintains parent-linked selector and wrapper
contexts. On declaration-block creation it writes either the final key or a
context seed directly into the block. Selector-local minification replaces the
selector value and recomputes the block key immediately when necessary.

Declaration-only edits leave the key unchanged. Moving a block to another
selector, layer, origin, cascade phase, or wrapper path requires a new key.
Unsupported wrappers use opaque occurrence identity; this design does not add
at-rule equivalence semantics.

## Nano sidecars

Persistent transformation state is indexed directly by AST IDs:

```rust,ignore
struct CrossRuleState {
    block_state: DenseMap<DeclarationBlockId, BlockState>,
    histories: FxHashMap<EffectiveKeyId, EffectiveHistory>,
    s1: SameSelectorCandidateList,
    s2: DeclarationOverrideCandidateList,
    s3: PartialMergeCandidateList,
}
```

`BlockState` contains only facts that are not permanent AST structure:
liveness during the pass, revisions, declaration-effect summaries, intrusive
history links, and queue membership. It does not duplicate owner, EffectiveKey,
or source-order identity.

Candidate endpoints are `DeclarationBlockId` or `RuleId`. A separate entry ID
or raw `&DeclarationBlock` is unnecessary.

## Iteration

The production traversal is store-native:

```rust,ignore
for block in compilation.declaration_blocks.semantic_iter() {
    consume(block.effective_key, block);
}
```

In practice callers also receive the typed ID from an ID-aware iterator. The
important property is that iteration does not recurse through the AST to build
a second flat vector.

Direct rule edges use rule-list topology. S2 histories use
`DeclarationBlockId` order within an EffectiveKey. Synthesized siblings are
already placed in Radix order, so ordered history insertion needs no
`SemanticSourceOrderKey` object.

## Mutation API

Payload mutation and structural mutation remain separate:

- declaration value replacement mutates through `DeclarationBlockId` and a
  declaration handle;
- rule/block insertion calls the owning store, chooses a local sibling key,
  and updates topology;
- retirement marks the node and unlinks it from live topology without reusing
  its ID; and
- local sibling relabel repairs IDs through one exact transaction.

Consumers cannot retain mutable references while a store may grow or insert.
They submit rewrite operations to `Compilation` and then reacquire values by ID.

## Code generation

Codegen starts from `StyleSheet.root_rules`, follows direct topology, and reads
values from the Radix stores. Primary-only lists use contiguous traversal;
transformed lists merge primary segments with sibling Radix segments.

Tombstones and merge-only sidecars are invisible. Terminal compaction is an
optional performance choice, not a correctness prerequisite.

## Required invariants

- Authored primary order equals lexical source order.
- `RadixIndexId` order equals semantic store order after local insertion.
- Direct sibling links agree with rule-list ownership.
- Every live block has one owner and one current EffectiveKey.
- Every authored declaration belongs to exactly one authored range before
  transforms.
- `Local4` never contains more than four properties.
- Overflow lists contain a complete semantic sequence, not a partial suffix.
- Nano does not maintain a second source-order identity.
- No arena pointer is used as semantic equality or persistent identity.

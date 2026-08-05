# Storage layout

## StyleSheet ownership

`StyleSheet` owns the authoritative rule tree and declaration data. Rule
topology is stored in one `RadixIndexArena`; it is not reconstructed by parser,
codegen, visitor, or Nano sidecars.

```rust,ignore
struct StyleSheet<'ast> {
    rules: RadixIndexArena<RuleRecord<'ast>>,
    declaration_blocks: RadixIndexArena<DeclarationBlock<'ast>>,
    declarations: DenseStore<DeclarationRecord<'ast>>,

    selector_values: SelectorValueInterner,
    context_paths: ContextPathInterner,
    effective_keys: EffectiveKeyInterner,
}
```

Every persistent reference uses a typed four-byte ID. Arena addresses are an
allocation detail and are never semantic identity.

## Rules use lexical preorder

The parser allocates a rule before parsing its body and immediately descends
into nested syntax. Primary rule IDs therefore follow CSS source order:

```text
source/tree                 rules arena

a                           a
  @media                    @media
    b                       b
  c                         c
d                           d
```

`RuleRecord` stores only its semantic parent and one compact physical subtree
span:

```rust,ignore
struct RuleRecord<'ast> {
    payload: CssRule<'ast>,
    parent: Option<RuleId>,
    nested_rule_count: u32,
    declaration_block: Option<DeclarationBlockId>,
    revision: u32,
    live: bool,
}
```

`nested_rule_count` is the number of arena records in the complete subtree,
excluding the rule itself. It includes children, deeper descendants, and
retained tombstones. Keeping tombstones in the span means retirement never
changes the boundaries of later subtrees.

There is no `RuleListId`, separate rule-list store, children vector, or
first/last/previous/next link. There is also no rule `RadixRange`: preorder
makes one count sufficient.

## Tree navigation

For a rule at position `R`:

- the first direct child, when present, is the next semantic arena ID;
- the first rule after its subtree is `advance(R, 1 + nested_rule_count)`;
- after visiting a direct child `C`, the next direct child is
  `advance(C, 1 + C.nested_rule_count)`; and
- the root span is the whole rules arena.

These calculations are private to the AST crate. Other crates use semantic
operations such as `root_rules`, `nested_rules`, `sibling_rules`,
`has_nested_rules`, source-order iteration, and scoped source-order transforms;
they do not read spans or drive Radix cursors themselves.

```text
Parser
  append_rule(parent, payload)
       ↓
AST
  append in lexical preorder
  increment every ancestor's nested_rule_count
       ↓
Codegen / Visitor / Nano
  root_rules / nested_rules / semantic source-order iteration
```

## Declarations

Declarations form one global semantic `RadixIndexArena`. Parsing appends
primary declarations in lexical order; structural minification inserts stable
sibling IDs at the declaration's final semantic position. Every declaration
block therefore remains one contiguous `RadixRange`.

`len == 0` is the only empty representation. Its `start_id` is an
unresolvable placeholder; the block's semantic position comes from
`DeclarationBlockStore` order, and the first declaration overwrites the
placeholder with its real ID.

Nested rules close the current declaration segment. Declarations that appear
after nested rules are represented by an explicit `NestedDeclarations` rule,
so parent and descendant declarations never become one ambiguous block.

Every declaration block permanently records its rule owner and effective key:

```rust,ignore
struct DeclarationBlock {
    owner: RuleId,
    declarations: DeclarationList, // RadixRange<DeclarationRecord<()>>
    effective_key: EffectiveKeyId,
    revision: u32,
    live: bool,
}
```

## Mutation

### Insertion

`insert_rule_after` finds the physical tail of the left rule's complete
subtree, inserts the synthesized rule at its final Radix position, repairs any
rare local ID relabel, and increments `nested_rule_count` for every ancestor of
the inserted rule. Callers receive a stable final `RuleId`; they do not repair
tree topology themselves.

### Retirement

Retirement marks a rule and its declaration block dead but retains their arena
records. Semantic iterators skip tombstones. Physical subtree counts stay
unchanged, so the source-order placement of every later rule remains stable.
A rule may be retired only when it has no live nested rules.

### Adjacency

Direct sibling adjacency is derived from parent equality and preorder subtree
spans, then revalidated by the AST mutation. Numeric ID closeness alone is
never proof of CSS adjacency because a sibling may have descendants or a local
Radix insertion may sit between primary IDs.

## Required invariants

- Primary parser allocation is lexical preorder.
- `nested_rule_count` exactly covers every physical descendant record.
- A record's `parent` equals the innermost subtree span containing it.
- Tombstones remain in physical spans but are absent from semantic traversal.
- All span and Radix cursor maintenance stays inside the AST implementation.
- Every live declaration block has exactly one live rule owner and one current
  `EffectiveKeyId`.
- Local Radix relabeling repairs every persistent ID reference atomically.

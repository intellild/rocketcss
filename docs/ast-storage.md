# AST context storage design

## Status

This document is the authoritative design for RocketCSS persistent AST storage.
It supersedes the earlier source-order storage proposal under
[`docs/flat-ast-ir`](./flat-ast-ir/README.md) and the implementation notes in
`.idea/task.md`.

The fixed-width node payload, shared extra slots, and range-based list layout
follow the flattened-AST storage approach used by the Yuku parser. RocketCSS
defines its layouts and APIs by hand: there is no schema-driven storage code
generation.

## Goals

- Remove persistent `Span` fields from flattened AST nodes and keep spans in
  context-owned sidecars.
- Replace owned AST `Box<T>` edges with typed `NodeId<T>` identities.
- Store the common node representation in a fixed-width table and spill only
  fields that do not fit into the shared extra-data table.
- Represent persistent lists as typed `start..end` ranges into that same
  extra-data table.
- Keep rule, rule-list, declaration-block, declaration, selector, context,
  layer, and effective-key records in their existing independent stores.
- Make `AstContext` the only API for reading, cloning, or mutating persistent
  AST data.
- Preserve lossless parse-to-codegen behavior and transform semantics.

## Terminology and naming

Storage names describe their concrete responsibility:

| Responsibility | Name |
| --- | --- |
| Compiler-owned AST lifetime and data access | `AstContext` |
| Fixed-width flattened node columns | `NodeData` |
| One fixed-width node payload | `NodePayload` |
| Shared overflow/list table | `ExtraDataStore` |
| One compact overflow/list slot | `ExtraData` |
| Existing rule and declaration structure | `RuleStore`, `RuleListStore`, `DeclarationStore`, and other domain names |
| Source-order relationships | `SourceOrderId` and explicit topology fields |

`AstContext` is the only public owner/access boundary. APIs, fields, tests, and
documentation use it directly rather than preserving compatibility aliases for
earlier storage experiments.

## Ownership and physical layout

`AstContext` owns all persistent AST storage for one compilation:

```text
AstContext
 ├─ allocator
 ├─ stylesheet root
 │
 ├─ nodes: NodeData
 │   ├─ spans:   [Span]
 │   ├─ kinds:   [NodeKind]
 │   └─ payloads:[NodePayload; N]   // 16 bytes per node
 │
 ├─ extra: ExtraDataStore
 │   └─ slots:   [ExtraData; M]     // 8 bytes per slot
 │
 └─ existing independent stores
     ├─ rules and rule lists
     ├─ declaration blocks and declarations
     ├─ selector values and paths
     ├─ context values and paths
     ├─ layer contexts
     └─ effective keys and their semantic intern indexes
```

The stores are private. Parser, visitor, codegen, Nano, tests, and downstream
callers obtain data only through `AstContext` methods.

The existing independent stores remain physically independent because they
encode domain-specific identity, topology, and interning. They do not become
entries in `NodeData`, and their IDs are not renumbered. When one of these
records needs a detached span, its store owns an ID-aligned span sidecar.

Existing hash maps are retained only where they implement semantic interning,
such as selector or effective-key lookup. Flattened nodes and list ranges never
add a hash map per Rust type.

## Typed identities and ranges

The common crate already provides generic, lifetime-bound dense identities and
ranges, so the AST adds aliases rather than wrapper structs:

```rust
pub type NodeId<'ast, T> = DenseId<'ast, T>;
pub type AstVec<'ast, T> = DenseRange<'ast, T>;
```

Properties of these handles:

- `NodeId<T>` is a four-byte identity. `T` is its compile-time domain.
- `AstVec<T>` is an eight-byte half-open `start..end` range.
- Neither handle dereferences itself or exposes storage access.
- An ID or range is meaningful only to the `AstContext` that created it.
- Identity equality is not structural equality.

`NodeData` may use one private raw dense domain internally. The only conversion
to `NodeId<T>` happens at the context's node-allocation boundary after it has
recorded `T::KIND`. No public integer-to-ID constructor is added.

## NodeData

### Parallel columns

Every flattened node consumes one aligned entry in each `NodeData` column:

```rust
struct NodeData<'ast> {
    spans: DenseStore<'ast, RawNodeDomain, Span>,
    kinds: DenseStore<'ast, RawNodeDomain, NodeKind>,
    payloads: DenseStore<'ast, RawNodeDomain, NodePayload>,
}

#[repr(transparent)]
struct NodePayload(u128);
```

The following invariant always holds:

```text
spans.len == kinds.len == payloads.len
```

Allocation reserves all three columns before publishing an ID. Speculative
parser rollback truncates all aligned node columns and the extra-data tail to a
single context checkpoint.

`NodeKind` is a compact, hand-maintained discriminant. It validates that a raw
slot is decoded with the node type that created it. There is no stored pointer,
Rust type name, trait object, per-type table, or per-type lookup map.

The upper 16 bits reserve a family range for the owning AST module and the
lower 16 bits identify a hand-written codec within that family. For example,
length, color, image, stylesheet-value, and token codecs currently use families
`0x0001` through `0x0005`. This keeps discriminants local to the owning module
without relying on a generated central registry.

### Inline and overflow layout

`NodePayload` is 16 bytes. Each node type has one manually defined, stable field
layout in the AST module that owns that type.

```text
logical encoded fields <= 16 bytes
  -> all fields live in NodePayload

logical encoded fields > 16 bytes
  -> bytes 0..12 contain selected inline fields
  -> bytes 12..16 contain the first ExtraData index
  -> remaining fields occupy a fixed, consecutive ExtraData sequence
```

Only overflowing fields go to `ExtraData`; a large node is not moved wholesale.
The number and meaning of overflow slots are determined by `NodeKind`, so no
runtime layout descriptor is stored beside a node.

For hand-written layouts, fields are ordered by these rules:

1. Preserve every source-bearing distinction required for lossless codegen.
2. Prefer putting compact, frequently read fields in the inline area.
3. Put every overflowing field in a fixed slot known by the node codec.
4. Use explicit sentinels for optional compact IDs; do not add a tag when the
   compact type already has an unused representation.
5. Assert `size_of::<NodePayload>() == 16` and document every byte/slot mapping
   next to the owning AST type.

Adding or changing an AST node therefore requires changing its local codec and
tests; it does not require editing a central generated schema.

## ExtraData

`ExtraData` is one untagged eight-byte storage slot:

```rust
#[repr(transparent)]
struct ExtraData(u64);
```

It may encode only representations whose interpretation is known from the
owning node field or typed range:

- `NodeId<T>` and optional node IDs;
- `AstVec<T>` ranges;
- compact string IDs or string ranges;
- scalar values and `repr` enums that fit in eight bytes;
- another explicitly documented compact representation.

It does not contain heap pointers, arena pointers, type names, or a dynamic
type tag. Encoding and decoding are centralized in checked, hand-written
traits/helpers:

```rust
trait AstNodeStorage<'ast>: Sized {
    const KIND: NodeKind;

    fn decode(payload: NodePayload, ctx: &AstContext<'ast>) -> Self;
    fn encode_new(self, ctx: &mut AstContext<'ast>) -> NodePayload;
    fn encode_existing(
        self,
        current: NodePayload,
        ctx: &mut AstContext<'ast>,
    ) -> NodePayload;
}

trait ExtraDataCompact<'ast>: Sized {
    fn encode_extra(self, ctx: &mut AstContext<'ast>) -> ExtraData;
    fn decode_extra(data: ExtraData, ctx: &AstContext<'ast>) -> Self;
}
```

The exact signatures may be split into private codec helpers, but the ownership
rule is fixed: all allocation and resolution still enter through `AstContext`.

## Persistent lists

An `AstVec<T>` directly addresses consecutive entries in `ExtraDataStore`:

```text
values:  T0              T1              T2
         ↓ encode        ↓ encode        ↓ encode
extra:  [ExtraData(i)]  [ExtraData(i+1)][ExtraData(i+2)]
         └──────────── AstVec<T> ────────────────┘
```

Each list element consumes exactly one `ExtraData` slot. Consequently there is
no list allocation pointer, range table, element-to-range table, runtime type
name, or hash map.

Not every current RocketCSS value fits into one slot. List element types are
classified before migration:

```text
compact scalar / repr enum / ID / range
  -> store the value directly in one ExtraData slot

composite value larger than 8 bytes
  -> allocate the value in NodeData
  -> store NodeId<Value> in one ExtraData slot
```

For example, large values such as selector components or token/value variants
must be flattened into nodes before their lists can move to `AstVec`. The field
type should make that ownership explicit, for example
`AstVec<NodeId<SelectorComponent>>`; an element codec must not hide a separate
per-type allocation.

The current borrowed-string `Atom` representation is wider than one slot. It
must become a compact string ID/range before it can be stored directly in
`ExtraData`, or remain behind a flattened node identity. Pointer packing is not
an accepted substitute.

### Current list-element inventory

The following inventory covers persistent `Vec<'ast, T>` fields in the current
AST. It classifies the compact logical representation, not Rust's in-memory
`size_of::<T>()`; enum padding and borrowed string pointers are never copied
into `ExtraData`.

Built-in one-slot representations:

```text
bool, u8, u16, u32, i32, f32
&str, Atom                         // context-owned compact string/atom ID
NodeId<T>                          // dense node index
Option<NodeId<T>>                  // dense node index or sentinel
AstVec<T>                          // nested start..end range
TokenOrValue                      // hand-written tagged 8-byte codec
```

One-slot codecs are required for these values, but no node promotion is
necessary because every variant has a lossless representation of at most eight
bytes:

```text
Animation, AnimationComponent
AnimationComposition, AnimationDirection, AnimationFillMode
AnimationIterationCount, AnimationName, AnimationPlayState
AnimationRange, AnimationRangeStart, AnimationRangeEnd, AnimationTimeline
BackgroundAttachment, BackgroundClip, BackgroundOrigin, BackgroundPosition
BackgroundRepeat
FamilyName, FontTechnology
GeometryBox, Image, KeyframeSelector, LengthPercentage
MaskClip, MaskComposite, MaskMode
Option<&str>, OtherTextDecorationLine, OverrideColors
PagePseudoClass, Point, Position
PositionComponent<HorizontalPositionKeyword>
PositionComponent<VerticalPositionKeyword>
PropertyId, Source, Symbol, SyntaxComponent, Time, UnicodeRange
WebKitColorStop, WebKitMaskComposite, WebKitMaskSourceType
```

These elements cannot be represented losslessly in one slot. Their owning list
field uses `AstVec<NodeId<T>>`, and parser construction allocates each element
through `AstContext::alloc_node`.

Promoted composite list elements:

```text
Background                              // Declaration::Background
BackgroundSize                         // background/mask size lists
BoxShadow                              // box-shadow lists
CursorImage                            // cursor image lists
EasingFunction                          // animation/transition timing-function lists
Filter                                 // filter lists
FontFamily                             // font-family lists
ImageSetOption                          // ImageSet::options
Calc                                   // MathFunction min/max/hypot lists
GradientItem                           // linear/radial/conic gradient stops and hints
Mask                                   // mask shorthand layers
ContainerCondition                    // recursive container-condition operations
MediaCondition                        // recursive media-condition operations
PageSelector                           // @page selector lists
ParsedComponent                        // recursive property-value components
ScrollStateQuery                      // recursive scroll-state operations
Selector, SelectorComponent           // selector roots and recursive selector components
StyleQuery                            // recursive style-query operations
SupportsCondition                     // recursive supports-condition operations
TextShadow                            // text-shadow lists
TrackListItem, TrackSize              // grid track lists
Transform                              // Declaration::Transform and parsed transform lists
Transition                            // transition shorthand lists
```

The inventory is intentionally based on the maximum encoded variant. For
example, `Selector::Unparsed` is small, but `Selector::Parsed` needs a tag plus
an eight-byte range, so every selector list stores `NodeId<Selector>`. In
contrast, `AnimationComponent` uses a tag plus at most one compact scalar or
node ID and therefore stays directly in `ExtraData`.

### List access and mutation

Because the physical range contains `ExtraData`, it cannot safely expose
`&[T]` or `&mut [T]`. The context provides typed value access:

```rust
ctx.vec_len(range)
ctx.vec_get(range, index)          // Option<T>
ctx.vec_iter(range)                // Iterator<Item = T>
ctx.vec_set(range, index, value)   // same-length replacement
ctx.clone_vec(range)               // context-aware element clone
ctx.rewrite_vec(&mut range, edit)  // length-changing rewrite
```

`vec_set` encodes one replacement into the existing slot. A length-changing
operation decodes values into temporary scratch storage, applies the edit,
appends a new compact sequence, and replaces the caller's range. Existing
ranges never change their bounds.

No reference into `ExtraDataStore` survives a call that can mutate
`AstContext`.

## Context-only node API

Flattened nodes are decoded to compact logical values. The public API does not
return a reference to an erased allocation because no such allocation exists:

```rust
ctx.alloc_node(value, span) -> NodeId<T>
ctx.node(id) -> T
ctx.node_span(id) -> Span
ctx.set_node_span(id, span)
ctx.clone_node(id) -> NodeId<T>
ctx.mutate_node(id, |value, ctx| ...)
```

Every flattened logical node must therefore be cheap to decode and must contain
only compact scalars, IDs, ranges, and other context-owned handles.

### `clone_node`

`clone_node` is the safe replacement for cloning an owned `Box<T>`. It performs
a context-aware deep clone according to the node type:

```text
decode source node
  ↓
clone owned child nodes, lists, and string storage through AstContext
  ↓
encode a new NodeData entry
  ↓
copy the detached span
```

A raw copy of `NodePayload` or `ExtraData` is only a shallow storage copy and is
kept private under an explicitly shallow name if it is needed internally.

### `mutate_node`

`mutate_node` decodes the current logical value, runs a closure with both the
value and its context, and encodes the result back into the same `NodeId`:

```text
validate NodeId<T> against NodeKind
  ↓
mark the node slot as mutating
  ↓
decode T to a local value
  ↓
closure(&mut T, &mut AstContext)
  ↓
rewrite the fixed inline/overflow layout
  ↓
publish the original NodeKind again
```

The temporary mutation marker rejects recursive access to the same node while
allowing the closure to read, allocate, clone, or mutate other nodes through
the context. Unwind handling republishes a valid encoded value before resuming
the panic. `NodeData` tracks the number of active mutations explicitly, so a
checkpoint rollback can reject an invalid rollback in constant time without
scanning the node columns.

Since one `NodeKind` always has a fixed number of overflow fields, updating a
node can overwrite its existing overflow slots after the closure completes.
It does not need a new per-type allocation or range lookup.

Independent rule/declaration/selector records receive equivalent context-owned
closure APIs. Their backing stores remain private.

## Structural operations

Identity and value semantics are separate:

- `NodeId<T>: Copy + Eq + Hash` compares node identity only.
- Structural equality recursively reads values through `AstContext`.
- Structural hashing recursively reads values through `AstContext`.
- Deep cloning recursively allocates through `AstContext`.

Code that previously relied on derived `Clone`, `PartialEq`, or `Hash` through a
`Box<T>` must migrate to the corresponding context operation. Interners may use
a fast fingerprint to select candidates, but exact structural equality remains
authoritative.

## Module ownership

Storage code follows the owning AST path:

```text
crates/ast/src/color.rs
  color node kinds and codecs

crates/ast/src/rules.rs
  rule-owned node kinds and codecs

crates/ast/src/rules/stylesheet/compilation/
  NodeData, ExtraDataStore, AstContext, checkpoints, and shared access machinery
```

There is no central list that duplicates property metadata. Known property
definitions remain sourced once from `crates/ast/src/properties.rs`; their
existing metadata drives which hand-written parser strategy allocates which
node type.

## Migration flow

```text
Define fixed NodePayload / ExtraData representations
  and context checkpoints
       ↓
Replace pointer-based node storage with aligned NodeData columns
       ↓
Add hand-written codecs beside the first boxed AST node families
       ↓
Replace Box<T> fields with NodeId<T>
       ↓
Inventory every persistent list element type
       ↓
Store compact elements directly;
  promote composite elements to NodeData and store their NodeId
       ↓
Replace pointer-based list storage with direct ExtraData ranges
       ↓
Migrate parser, visitor, codegen, and Nano to AstContext-only access
       ↓
Remove compatibility aliases and obsolete storage code
```

Existing independent stores are migrated only at their API boundary: make
their fields private, add detached span sidecars where needed, and route reads
and mutations through `AstContext`. Do not copy their records into `NodeData`.

## Required verification

Each migrated node family must cover:

- exact `NodePayload` and `ExtraData` size/layout assertions;
- inline-only and overflow-field round trips;
- wrong-kind ID rejection;
- detached span read/write and aligned rollback;
- deep `clone_node` behavior;
- nested and panic-path `mutate_node` behavior;
- compact list iteration, replacement, rewrite, and deep clone;
- parser-to-codegen losslessness with Nano disabled;
- intended transform output with Nano enabled;
- structural equality/hash behavior where the old AST used value semantics.

Repository-level completion requires `cargo fmt --all`, relevant Rust tests,
and Clippy. Benchmarks should compare parse, codegen, and representative Nano
passes before removing the old storage path.

## Non-goals

- Generating storage layouts or accessors from an external AST schema.
- Putting existing independent rule/declaration/selector stores into
  `NodeData` merely to make the representation uniform.
- Supporting arbitrary Rust values in `ExtraData` through pointers or erased
  allocations.
- Exposing `DenseId::index`, `DenseRange` bounds, stores, or raw slots as an AST
  access API.
- Normalizing, reordering, deduplicating, or shortening authored syntax during
  storage encoding.

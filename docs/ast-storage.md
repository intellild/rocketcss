# AST context storage design

## Status

This document is the authoritative design for RocketCSS persistent AST storage.
It supersedes the earlier source-order storage proposal under
[`docs/flat-ast-ir`](./flat-ast-ir/README.md). The active migration checklist is `.idea/task.md`.

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

| Responsibility                              | Name                                                                     |
| ------------------------------------------- | ------------------------------------------------------------------------ |
| Compiler-owned AST lifetime and data access | `AstContext`                                                             |
| Fixed-width flattened node columns          | `NodeData`                                                               |
| One fixed-width node payload                | `NodePayload`                                                            |
| Shared overflow/list table                  | `ExtraDataStore`                                                         |
| One compact overflow/list slot              | `ExtraData`                                                              |
| Existing rule and declaration structure     | `RuleStore`, `RuleListStore`, `DeclarationStore`, and other domain names |
| Source-order relationships                  | `SourceOrderId` and explicit topology fields                             |

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

## String ownership and direct values

`crates/common/src/string_pool.rs` stores an immutable root source, append-only
extra UTF-8 bytes, and an Fx intern table with independently allocated keys.
Ordinary `AstStr` insertion bypasses interning. Source subslices retain their
original offsets; other ordinary strings append to extra. Atom interning
canonicalizes by content, preserving the first range and copying only the
stable lookup key when that range already exists.

Source recognition checks that a borrowed string is fully contained in the root
address range. Its existing `&str` type establishes UTF-8 boundaries, so this
path constructs offsets without slicing the root again. The public integer
`source_range` entry and text reads still validate bounds and character boundaries.
A slice crossing either root boundary is external even when it shares the root's
backing allocation, and ordinary external strings append without interning.

Parser rollback truncates node and extra-data entries, but the string pool stays
append-only. A regression with 64 failed parses appends 640 bytes of ordinary
decoded text while restoring node/list checkpoints each time and retaining one
shared intern key. Root-source ranges, external temporary inputs and decoded
buffers remain valid through rollback and subsequent extra-buffer growth. This
measures retained text length, not arena capacity or physical resident memory.

AstStr and Atom retain range-identity Eq/Hash but do not implement Ord or
PartialOrd. Offset order and intern order are not lexical text order; callers
that need lexical ordering must resolve through the owning pool. Each type has
a compile-fail doctest guarding against accidental direct sorting. Neither range
type exposes context-free string dereferencing or implicit string comparison.

Content-sensitive equality must also resolve ordinary ranges. For example,
Nano's mask-image comparison preserves its URL source-span check and uses
`AstContext::nodes_eq` for URL text. Comparing two decoded Url values with their
derived PartialEq would incorrectly distinguish equal text at different ranges.

```text
Compiler / parser
  source subslice or decoded text
       ↓ AstContext::add_str / intern
StringPool (common/string_pool.rs)
  ordinary range / canonical Atom range
       ↓ completed field
Url (ast/rules/stylesheet.rs)
  direct padding-free slot value
       ↓ context.str(range)
Codegen (codegen/rules/stylesheet.rs)
  borrow text only while serializing
```

Text references borrow the pool, not the allocator lifetime. Temporary parser
sources do not replace the root source. Speculative rollback leaves strings
append-only so dedup entries and previously issued ranges remain valid.
Parsing returns the pool inside AstContext: plugins must intern new names in
that returned context.
Same-context node/list cloning copies string ranges while allocating only the
required cloned nodes/lists. Default typed visitor callbacks leave string ranges
unchanged; committing those node/list transactions does not re-intern text.
The no-op visitor regression traverses both Atom and AstStr fields three times
and verifies stable node/extra counts, pool bytes, intern count and CSS output. Ranges must not be transferred directly between pools;
resolve text in the original pool and add/intern it in the destination.

Slots now use aligned arrays of `MaybeUninit<u8>` (16-byte node payloads,
8-byte extra slots). Direct writes accept Copy types and check size at compile
time. Unaligned typed copies preserve padding initialization without reading
padding as integers; raw slot equality, hashing, and byte-printing are absent.
The storage traits are unsafe implementation contracts. Their decode and
replace operations require a slot written for the same type; public typed
context APIs establish this through NodeKind or typed list ranges. The legacy
byte-layout accessors have been removed.

Token is now 16 bytes and stores ordinary ranges. Token, Length, Angle, CssColor,
LightDark, URL and syntax literal nodes use native slot values. Numeric values,
bool, NodeId, optional NodeId, list ranges and string ranges use native extra
slots. Oversized logical values use module-owned native headers and overflow
fields; necessary compact enum/Option representations are documented below.

Function names are ordinary AstStr values. The payload stores a native header
containing the argument range, overflow index, flags, and KnownFunction. Reads
preserve the identity resolved at construction instead of reclassifying text.
The overflow holds the name range in one slot and the native optional replacement
across two opaque slots; padding is never read as integers. Unchanged writes reuse
all three slots and leave the string pool untouched. Function codegen streams
arguments directly from the shared table, including lossless fallback functions.

TokenOrValue is 12 bytes as a logical value. Its private native eight-byte list
enum flattens scalar units beside the tag; this is necessary layout compression,
not a byte serialization codec. DashedIdent contains an ordinary eight-byte
range in a typed node created before publication. Scalar alternatives remain
inline, and compact conversion cannot allocate or append string-table entries.
Animation names similarly contain ordinary ranges in nodes; animation-name
lists contain native NodeId slots. Specifier file names and environment-variable
unknown names are ranges, and DashedIdentReference is a native 12-byte node
with an optional Specifier node. Same-context deep clones copy ranges and clone
owned child nodes, while content equality resolves ordinary strings.

PropertyId::Custom and CustomPropertyName hold ordinary AstStr ranges. PropertyId,
CustomPropertyName, ParsedComponent, SyntaxString, and CustomProperty use native
node payloads. Transition-property lists publish PropertyId nodes before their
NodeId slots. Known property classification comes from the existing metadata;
unknown property names are resolved through the pool only when needed.
UnparsedProperty preserves optional raw text as Option<AstStr> in its native
header, with its remaining fields across two opaque extra slots. Writes reuse
those slots. Nano's property-name map keys compare string contents; scratch-owned
keys avoid retaining pool borrows across mutation.

Selector and ViewTransitionPartName are native 12-byte values in node payloads.
Their Atom fields no longer pass through the legacy atom reference table.
PseudoClass and PseudoElement also use native payloads. Only their oversized
custom-function variant refers to a CustomPseudoFunction node, which stores its
Atom name and argument range directly in 16 bytes. Common pseudo variants allocate
no overflow storage. Deep clone copies the custom-function node and its argument
list; generated visitors follow this child. AttrSelector uses a native header
(local name, overflow index, never-matches flag)
and five fixed opaque slots: one Atom for the lowercase matching name and four
for the native optional namespace/operation fields. Updates reuse all five slots
without adding atom references. AstContext::attr_selector_syntax reads only the
header and the latter four slots. Stored AttrSelector::to_css_node uses that
entry point, skipping the lowercase name; stored and owned values share the
same attribute serializer.

SelectorComponent uses a private native 16-byte enum. Its common alternatives
keep Atom ranges, NodeIds and list ranges inline. Namespace, local-name and
attribute-existence alternatives use one extra slot for the second Atom. The
attribute-value alternative uses two slots for its value/operator/case/flag tuple;
NthOf keeps its selector range inline and uses two slots for native nth data.
Same-variant writes reuse overflow; changes to another overflow variant allocate
its matching layout. This is necessary compression of oversized variants, not
byte serialization. No handwritten selector discriminant or primitive codecs
remain. The atom reference table, its accessors and checkpoint length are removed;
the ordinary string table and its checkpoint length have also been removed.

Stored selector components use AstContext::selector_component_syntax. Namespace
prefixes, local names and attribute-existence names come directly from the inline
slot, without reading namespace URLs or lowercase matching names. Class and ID
names also use this direct path, avoiding the full-component fallback and its
second serializer dispatch. All remaining variants now read their native slot
fields into authored syntax directly; attributes and nth-of read the native
overflow values needed for output. There is no Other/full-component fallback.
Stored and owned components share a syntax writer. Matching-only namespace URLs,
lowercase name copies and the never-matches flag are absent from the syntax view.
Selector::to_css calls the component NodeId's ToCss entry to reach this path.
SelectorComponentSyntax is a transient read view inside the owning selector
module, excluded from persistent AST visitor generation; stored layouts and
matching metadata remain intact.
The single-selector `:is()` omission check scans component tags once through
`selector_component_is_combinator_or_type`. It does not reconstruct components
or read attribute/name overflow merely to identify combinators or type selectors.

MediaCondition, MediaFeatureValue and SupportsCondition use native payloads.
Media identifiers and supports declaration/selector/unknown text use ordinary
AstStr ranges, resolved for codegen and context-aware content comparisons. Ratio
keeps its native optional denominator and floating-point bits. Supports parsing
publishes its raw range before storing the condition; temporary source borrows
remain in the parser. MediaType::Custom also holds AstStr. MediaQuery's logical
value is compressed into a native repr(u8) enum: qualifier follows the tag, then
aligned optional condition and custom-name range. It fits the 16-byte payload
without overflow or reference rows. MediaList stores its range directly.
MediaFeatureName::Custom and Unknown use AstStr. QueryFeature's native slot
flattens name kind and predicate kind into one enum, preserving native FeatureId,
comparison enums and NodeIds. All known-name forms and named plain/boolean/range
forms fit inline. Named interval forms use one extra AstStr slot and reuse it on
same-variant updates. Media, container size and scroll-state IDs only provide their
own NodeKind; their former integer codec mappings are removed. Context-aware
feature equality compares ordinary name contents without merging Custom/Unknown
variants. media.rs has no persistent borrowed strings or legacy string-table calls.

QueryFeature codegen uses a context-bound native-slot view. Predicate access
preserves the authored Boolean/Plain/Range/Interval form and comparison enums;
name access reads interval overflow only at its output position. Stored and
owned paths share the writer. The public QueryFeatureId bound is sealed to the
three supported feature ID enums, keeping typed slot reads tied to their owning
NodeKind. The transient predicate and read view are excluded from AST generation.

KeyframesName, ViewTransitionName and ViewTransitionGroup hold ordinary AstStr
and use native 12-byte node values. Context-aware equality compares their text
while keeping variant distinctions, including quoted versus identifier keyframes
names. Keyframes parsing uses the root Compiler's with_source so published ranges
belong to the root pool, including decoded escapes. It does not return a range
from a separate temporary Compiler.

TextEmphasisStyle::String and Appearance::NonStandard use ordinary AstStr and
native 12-byte enum values. Text emphasis keeps native fill and optional shape
variants. Their content equality and codegen resolve text through AstContext;
repeated reads and in-place updates do not append string-reference rows.

FontFamily::Custom holds ordinary AstStr in a native 12-byte FontFamily value.
from_known_name classifies static families without touching the pool. Parser
constructs ranges for custom names and appends assembled multiword names directly,
without an intermediate arena-owned copy. Codegen's quoting decision uses only
known-name classification. Font-family list equality ignores tombstones while
comparing ordinary text by content. Nano's explicit list minifier receives the
AstContext and preserves ASCII-insensitive custom-name deduplication.

FontFormat::String, FamilyName and FontFeatureDeclaration.name also use AstStr.
FontFormat is a native 12-byte node; FamilyName is a native eight-byte list slot.
Family-name parsing uses the root Compiler's with_source and adds assembled text
directly to its pool. Removing FontFeatureDeclaration's borrowed string also
reduces DeclarationRecord<DeclarationPayload> from 40 to 28 bytes on the verified
layout. rules/font.rs no longer contains persistent &str fields or string-table
operations.

PageSelector.name is Option<AstStr>. Its native 16-byte storage struct contains
an eight-byte optional-string slot and an eight-byte pseudo-class range. The
name slot reuses ExtraDataCompact's optional-string sentinel representation;
it is embedded in the payload and never appended to the extra-data store.
None remains distinct from Some(empty), and transitions between named and
unnamed selectors do not allocate overflow. PagePseudoClass list values are
native enums rather than manually mapped integers.

ImageSet stores its native option range and vendor prefix directly.
ImageSetOption codegen uses a context-borrowing `ImageSetOptionRead`: image and
resolution are read from its header, and the optional file-type slot is read
only when serialization reaches `type(...)`. Stored and owned values share one
writer; this avoids reconstructing the complete 24-byte logical node. The view
lives in the owning background module and is excluded from visitor generation.

ImageSetOption has a native 16-byte header containing image, Resolution and a
file-type extra index. u32::MAX means no slot has been allocated. The first
present value allocates one compact Option<AstStr> slot, preserving empty text
separately from None. Once allocated, the slot remains attached to the header;
present-to-absent-to-present writes reuse it. A newly created option without a
file type requires no extra allocation. The index is checked before publication.

GridLine's area and optional line/span names use AstStr. A native 16-byte
storage enum separates named and unnamed line/span variants, retaining signed
indices without byte conversion and preserving None versus Some(empty) with
no extra allocation. Context-aware equality resolves name contents. Nested grid
line-name lists and template-area optional-string lists still require migration.

Variable fits the 16-byte native payload, including its optional fallback range.
EnvironmentVariable codegen uses a context-borrowing field view. The name comes
from the header; the indices range and optional fallback are read at their output
positions. Stored and owned values share one serializer, preserving absent and
explicitly empty fallback syntax without assembling the complete logical node.

EnvironmentVariable uses a native name/index header, one extra slot for indices,
and one slot for the optional fallback range. Option<AstVec> uses a typed union
of the native range and its initialized u32 bounds: the reversed range [1, 0]
marks None, while valid ranges (including empty) retain their original bounds.
This is explicit compression of the larger Rust Option, not a raw copy of that
Option into eight bytes. Repeated writes reuse both overflow slots. Cloning an
optional list preserves absence and clones present list elements. Variable and environment codegen share a
streaming fallback serializer under codegen/rules/stylesheet.rs.

The parser's temporary ValueToken is Copy: every field is a borrowed string or
scalar. Cursor/replay and grammar consumers copy it directly instead of invoking
a variant-by-variant clone. This does not copy text or construct persistent ranges.
It retains borrowed text, with an explicit
persistent Token construction boundary in `parser/compiler.rs`. Plain codegen
only resolves text when emitting it. Context-aware node equality compares
ordinary strings by contents, preserving declaration-merging behavior for
identical text at different offsets.

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

#[repr(C, align(16))]
struct NodePayload([MaybeUninit<u8>; 16]);
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
#[repr(C, align(8))]
struct ExtraData([MaybeUninit<u8>; 8]);
```

It may encode only representations whose interpretation is known from the
owning node field or typed range:

- `NodeId<T>` and optional node IDs;
- `AstVec<T>` ranges;
- compact string IDs or string ranges;
- scalar values and `repr` enums that fit in eight bytes;
- another explicitly documented compact representation.

It does not contain heap pointers, arena pointers, type names, or a dynamic
type tag. Typed context APIs establish the layout preconditions required by these
internal storage traits:

```rust
unsafe trait AstNodeStorage<'ast>: Sized {
    const KIND: NodeKind;

    unsafe fn decode(payload: NodePayload, ctx: &AstContext<'ast>) -> Self;
    fn encode_new(self, ctx: &mut AstContext<'ast>) -> NodePayload;
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        ctx: &mut AstContext<'ast>,
    ) -> NodePayload;
}

unsafe trait ExtraDataCompact<'ast>: Sized {
    fn encode_extra(self) -> ExtraData;
    unsafe fn decode_extra(data: ExtraData) -> Self;
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

`Atom` and ordinary `AstStr` are distinct 8-byte `(start, end)` ranges. Both
fit directly in `ExtraData`. Atom equality compares canonical ranges within
one pool; ordinary ranges are not interned and need pool-resolved text for
content equality. Pointer packing is not used.

### Current list-element inventory

The following inventory covers persistent `Vec<'ast, T>` fields in the current
AST. It classifies the compact logical representation, not Rust's in-memory
`size_of::<T>()`; native padding stays opaque. Persistent string slots contain ranges directly.

Built-in one-slot representations:

```text
bool, u8, u16, u32, i32, f32
AstStr, Atom                       // direct 8-byte range
Option<AstStr>                     // range or reversed-range None sentinel
NodeId<T>                          // dense node index
Option<NodeId<T>>                  // dense node index or sentinel
AstVec<T>                          // nested start..end range
TokenOrValue                      // native compressed 8-byte enum
SyntaxComponent, Time              // native 8-byte values
```

One-slot codecs are required for these values, but no node promotion is
necessary because every variant has a lossless representation of at most eight
bytes:

```text
Animation, AnimationComponent
AnimationComposition, AnimationDirection, AnimationFillMode
AnimationIterationCount, AnimationPlayState
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
Source, Symbol, UnicodeRange
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

## Read-only serialization boundary

ToCssContext stores only `&AstContext` and `&GhostToken`; neither API exposes
mutable access. NodeData, ExtraDataStore and StringPool mutations require
`&mut self`. String reads borrow the pool and typed node/list reads copy stored
values without publishing anything. Codegen does not call the AST allocator,
node allocation, intern or add_str APIs. Its only explicit unsafe operation
converts the serializer's initialized hexadecimal buffer to UTF-8.

Consequently pure codegen cannot append node, extra-data or string-pool entries
through these APIs. Output strings and serializer scratch buffers may still
allocate; this boundary is about AST storage, not zero total allocations.
Construction and transforms perform string insertion before serialization.
The repeated-codegen tests in codegen/tests/ast.rs and to_css.rs additionally
check node checkpoints and pool byte lengths for migrated string-bearing paths.

## Context-only node API

Parser publication uses three small entry points in `parser/compiler.rs`:
`store_node` first converts parser-only ValueToken strings into persistent ranges,
then allocates the typed node. `store_vec` commits completed compact values
directly. `store_node_vec` receives already-constructed typed values, captures the
current span once, reserves the exact ID count, allocates the children, and only
then commits their IDs as a contiguous extra-data range. Keeping this one ID
buffer prevents child overflow allocations from interleaving with list slots.

`AstContext::alloc_vec` forwards its owned iterator to `alloc_encoded_vec`, which
maps the context-free `encode_extra` directly into `ExtraDataStore::alloc`.
The latter reserves the range and appends slots without an intermediate SmallVec.
Clone/edit buffers serve a different purpose: their callbacks can allocate or
mutate the context, so the source values are snapshotted before those callbacks.

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

Codegen's `ToCss::to_css_node` defaults to native typed-value access for inline
nodes. Overflow-bearing nodes can override it with their owning AST module's
field accessors. `NodeId::to_css` dispatches through this method, so callers do
not need a parallel list of node kinds or serialization implementations.

`AstContext::unparsed_property` returns a read-only `UnparsedPropertyRef`.
The header contains `raw_value`; the two overflow slots independently store
the token-list range and property metadata. Raw fallback output reads no
overflow. Declaration name/prefix dispatch reads only the metadata slot; token
fallback output reads only the list slot. `None` and `Some(empty)` remain distinct.

`AstContext::function` returns a read-only `FunctionRef`. Arguments, kind and
flags are inline. A header boolean, derived from the replacement on every write,
uses existing payload padding and avoids loading the two replacement slots when
there is no replacement. Replacement output does not load the name slot; opaque
fallback serialization does not load replacement slots. Name and replacement
storage still occupy the same three reused overflow slots. Both ordinary values
and stored-node serialization share the same function-writing helpers.

These views borrow the context while retaining the ID's range lifetime. Their
constructors validate NodeKind; their private fields prevent constructing a view
over arbitrary payload bytes. The context cannot be mutated while a live view is
used. Field reads return only scalars, IDs and string/list ranges, and never
modify the pool or allocate storage.

Empty variable fallbacks inspect the final list element by index after successful
serialization. Function arguments no longer use an `inspect` iterator that
decoded every token again solely to remember whether the final token was a comma.

### `clone_node`

`clone_node` is the safe replacement for cloning an owned `Box<T>`. It performs
a context-aware deep clone according to the node type:

```text
decode source node
  ↓
clone owned child nodes and lists within the same AstContext; copy string ranges
  ↓
encode a new NodeData entry
  ↓
copy the detached span
```

`clone_node` and `clone_vec` operate within their owning context. Copying an
`AstStr` or `Atom` there preserves its range without appending or interning text.
These APIs do not import a subtree from another context. When transferring a
string between pools, resolve it through the source pool and call `add` for an
ordinary string or `intern` for an atom on the destination pool. A copied offset
alone cannot identify the same text in another pool, even when both pools share
an allocator or lifetime.

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

crates/ast/src/rules/{background,border,font,layout,...}.rs
  rule-owned node kinds and storage layouts
       ↓ same owning path
crates/codegen/src/rules/{background,border,font,layout,...}.rs
  ToCss implementations and owner-specific printing helpers

crates/ast/src/values/{alignment,animation,box_model,image,...}.rs
       ↓ same owning path
crates/codegen/src/values/{alignment,animation,box_model,image,...}.rs
  typed value serializers and per-module keyword macro invocations

crates/parser/src/parser/rules/{at_rule,font,keyframes,page,...}.rs
  typed rule-prelude/descriptor parsing and owner-specific helpers
       ↓
crates/parser/src/parser/rules/stylesheet/compilation/mod.rs
  dispatch imports the owning rule module

crates/ast/src/rules/stylesheet/compilation/
  NodeData, ExtraDataStore, AstContext, checkpoints, and shared access machinery
       ↓ same owning path
crates/codegen/src/rules/stylesheet/compilation.rs
  streams the persistent rule and declaration stores
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

## Grid string lists and native track storage

Grid line names, including nested repeat line names, store ordinary `AstStr`
ranges. Area cells store `Option<AstStr>`. The optional-string extra slot uses a
private union of `AstStr` and two `u32` bounds: `(1, 0)` is the absent sentinel.
Valid StringPool ranges always have start <= end, so no valid range collides;
`Some(AstStr::EMPTY)` remains `(0, 0)`. Both union fields occupy eight initialized
bytes because AstStr has a C layout with two u32 fields and no padding. Reading
checks the sentinel before accessing the string field. This compression neither
interns strings nor appends string-table rows.

TrackSizing stores a native 16-byte enum. A newly created None has no overflow;
TrackList keeps its items range inline and its nested line-name range in one
extra slot. Switching to None retains the allocated slot index internally so
restoring a list reuses that slot. Codegen obtains an optional context-bound TrackListRead:
None prints directly without reading extra, while the list view exposes inline
items and the separate line-name range. Stored and owned lists share the same
writer, including names-only and trailing-name behavior. TrackListItem, TrackSize, TrackBreadth and
GridTemplateAreas fit directly in native payloads. Nested-list cloning copies
list storage while retaining the same context's string ranges. Grid-area codegen
iterates cells directly and reuses a row buffer instead of collecting every cell
into an intermediate vector.

## List, container and animation names

ContainerNameList entries, ListStyleType strings, CounterStyle names, Symbol
strings and AnimationTimeline dashed names use ordinary AstStr ranges. Their
native node values fit in 16-byte payloads. Content equality for standalone
string-bearing nodes resolves text through AstContext; different source/extra
ranges with the same text remain equal for node comparisons.

Symbol and AnimationTimeline exceed eight bytes after the range migration.
Their lists therefore contain NodeId values, allocated before publishing the
list. Encoding a list element only stores its existing handle. Deep cloning
CounterStyle symbols clones the symbol nodes and their image children;
AnimationTimeline cloning clones ViewTimeline's inset node. Same-context string
clones keep their ranges and do not intern or append text.

## Rule string lists and removal of the reference table

Composes names, view-transition part classes, import layer names and nested layer
statement/block paths use ordinary AstStr lists. Import URLs also store AstStr.
Parser construction adds source or decoded ranges before publishing the lists;
codegen resolves each range when writing identifiers or strings. Anonymous import
layers remain Some(empty list), distinct from an absent layer.

Composes and ViewTransitionPartSelector directly store their native Copy values
in a node payload, including optional node handles. StringData, StringDomain,
store_string/resolve_string, the borrowed-str ExtraDataCompact implementation and
the string-table checkpoint length have been removed. Node rollback truncates
nodes and extra slots only; StringPool remains append-only during parse. Rule payload fields and license comments now also use ordinary string ranges;
there is no second string reference table.

## Rule text and context source keys

Charset encoding, namespace prefix/URL, custom-media and rule payload names,
license comments, and persistent context source keys all use ordinary AstStr.
Charset, namespace and single-identifier parsing use the root Compiler's
with_source boundary. Unescaped text refers to root source offsets; decoded or
external text enters the same context's extra buffer. License comment construction
publishes a range before appending it to the comment list.

Container context hashing and comparison resolve name contents in both AST context
canonicalization and Nano equality. Source-key APIs accept a short-lived &str,
compare it to resolved stored keys, and add a range only for a new context record.
The caller can drop a temporary source-key buffer immediately. Pool growth leaves
stored source keys valid. The RuleRecord<CssRulePayload> layout is now 80 bytes
(previously 96); the layout assertion records the complete record size.

Generated visitors still provide a generic borrowed-str implementation, and
source input/string parsing helpers still take temporary &str arguments. Those
are not persistent AST text fields. WTF-8 atoms currently have no AST consumers.

## Direct list publication

ExtraDataCompact::encode_extra and decode_extra take no context. Decoding copies
the native value or performs necessary single-slot layout conversion; it does
not resolve child nodes or strings. AstContext::vec_get consequently reads the
typed slot without casting the context to the range's lifetime. Returned handles
and ranges still belong to the context that originally allocated them.

All list elements, including
node handles and string ranges, are prepared before publication; conversion
cannot insert nodes, strings or nested overflow into the destination context.
alloc_encoded_vec and rewrite_encoded_vec therefore map values directly into the
reserved ExtraDataStore range. The two SmallVec slot staging buffers were
removed. Decoding preserves handles without resolving them.
Editing and deep cloning retain construction buffers where child-node allocation
must finish before the new list range is published.

Native one-slot enums now include animation iteration/direction/play/fill/
composition, geometry boxes, text-decoration lines, background attachment/clip/
origin and mask mode/clip/compositing values. Mask's native header stores three
node handles and an overflow index; its typed field group occupies one extra
slot, reused on writeback. It does not reinterpret the native enum slots as
integers or read padding bytes.
Stored Mask codegen uses MaskRead for the three handles, then reads the native
keyword group once through MaskKeywordsRead after writing image geometry. Owned
and stored values share geometry and keyword writers. The size value is retained
between default detection and output; horizontal/vertical components are each
read once inside the position writer. Default omission and printer-specific
side-keyword spelling are unchanged.

## Native animation storage

Animation and AnimationRange store their native eight-byte range/handle-pair
values. AnimationAttachmentRange uses a compact repr(u8) native enum in both node
payloads and extra slots. AnimationComponent's nested Time and iteration-count
variants are flattened into a private native eight-byte enum; this preserves
units and float bits without allocating child scalar nodes. This flattening is
necessary layout compression, not a byte serializer.

EasingFunction uses a native 16-byte header with a data enum and an overflow index.
New keywords, frames and steps need no overflow. CubicBezier stores its two y
coordinates in one native [f32; 2] extra slot. The header retains that index when
switching away from cubic, so switching back reuses the same slot. u32::MAX means
no slot has been allocated; allocation checks that the index cannot equal it.
Stored codegen matches EasingFunctionRead; only its CubicBezierRead variant
accesses overflow, yielding coordinates in CSS order (x1, y1, x2, y2). Owned and
stored values share cubic, frames and steps writers, including the existing
keyword abbreviations. Reading the view does not rebuild the full logical enum.
Transition keeps float magnitudes and the property handle in a native header,
with timing-function handle and typed time-unit fields in one extra slot. Both
units remain authored, and no float arithmetic is performed during storage.
The old animation/time byte helpers were removed after their last users migrated.

Transition codegen reads a native header/extra-field snapshot. It reconstructs
the duration and only nonzero delay; either unit of positive or negative zero
retains the existing omission rule. Stored and owned paths share the writer,
which resolves the easing node once for both the Ease check and serialization.
The snapshot retains typed handle lifetimes and does not change storage layout.

Animation codegen iterates its component range directly. A byte records the five
keyword classes already emitted, preserving the rule that a colliding quoted
name stays quoted unless that class precedes it. This removes the temporary
component Vec and repeated prefix scans while keeping authored component order.

## Native background and keyframe layouts

Background's color-only shortcut checks inline repeat/attachment/origin/clip
values before following position and size handles. The image node is read once
and reused for ordinary output. Position and BackgroundPosition share one writer
that reads each coordinate once, preserving center omission and explicit side
offsets. Background's own multi-slot node still uses the default full read; this
consumer optimization does not complete its field-access migration.

Position and BackgroundPosition directly store native handle pairs in both
payloads and extra slots. WebKitColorStop stores its color handle and f32 position
directly; BackgroundRepeat stores its two native keyword values. KeyframeSelector
uses its native eight-byte enum directly, preserving From, To, Percentage and
the nested timeline-range variant without a separate slot enum.

Background stores a native header and two independent typed extra slots: a size
handle followed by the native keyword group. BackgroundRead exposes the header
and these slots without reassembling the full logical node. Stored and owned
values share a writer, preserving the color-only guard and full output order.
BoxShadow uses a native header plus separate y-offset/spread and bool
slots. Updates reuse these slots; no integer reinterpretation of padding is
involved. Their total slot counts are unchanged. BoxShadow codegen reads its
native field view rather than reassembling a full shadow. WebKitGradientPoint
directly stores its complete 16-byte
Copy value, including both native nested coordinate enums and typed horizontal/
vertical side values. There are no byte codecs left
in rules/background.rs or rules/keyframes.rs.

## Native layout storage

Inset, margin, padding and their logical/scroll variants, Flex, Gap, GridTemplate,
GridRow, GridColumn and GridArea directly store native Copy handle structures in
payloads. FlexFlow, ColumnRule and Columns also fit natively; optional fields keep
their Rust Option representation, and numeric values preserve their bits.

TrackRepeat keeps its native RepeatCount and overflow index in the header, with
line names and track sizes in two independently typed range slots. Its borrowed
view exposes these fields directly; codegen reads the lists after emitting the
count and uses the same ordered writer as owned values. Nested ordinary strings,
empty groups and trailing line names retain their previous output behavior.
Grid keeps its three node handles and index in the header. Auto-column range,
auto-row range and GridAutoFlow occupy three independently typed slots, replacing
the former multi-slot GridFields aggregate. Its read view exposes the header
directly and supplies auto fields after rows/columns are emitted. Stored and
owned values share the writer; the areas node is resolved once for both omission
checking and output. Writes reuse these overflow ranges. Slot counts and deep-clone
behavior are unchanged. rules/layout.rs contains no remaining byte codecs.

## Native border storage

Border radii, colors, styles and widths (physical and logical pairs), border
image slices, and both GenericBorder<LineStyle>/GenericBorder<OutlineStyle>
instantiations directly store native Copy payloads. The two generic
instantiations retain distinct NodeKind values. BorderSideWidth, LengthOrNumber
and BorderImageSideWidth likewise use native scalar/handle enums.

BorderImage keeps outset, slice, repeat keywords and the overflow index in its
native header; source and width handles occupy one typed extra slot reused on
writes. Stored codegen reads a context-bound BorderImageRead with header fields
and the native source/width pair, sharing its writer with owned values without
reconstructing the complete BorderImage. Deep cloning still clones all child nodes. Line-style integer helpers
and their re-exports were deleted after the last consumer migrated. Both
rules/border.rs and values/border.rs are free of byte codecs.

## Native image values

Image, EndingShape, Ellipse, Circle, NumberOrPercentage, BackgroundSize and
LengthPercentageOrAuto store their native Copy values directly in payloads.
Image also fits directly in an eight-byte list slot. PositionComponent uses
repr(u8) with side before offset so both horizontal and vertical instances fit
in eight bytes, including the distinction between None and Some(node zero).
GradientItem and DimensionPercentage nodes use native payloads with distinct
kinds for each supported dimension.

Both supported DimensionPercentage instances (LengthPercentage and
AnglePercentage) are natively eight bytes, so their lists use the same direct
Copy representation as nodes. Percentage, Zero, Calc and the dimension unit/value
remain typed; no separate list enum or dimension unit conversion is needed.
DimensionValue supplies only the associated kind metadata. Slot access enforces
capacity at compile time for each concrete generic instance. WebKitGradient uses a native header and two typed extra slots for
radii and the stop range; Linear/Radial transitions reuse the same overflow.
Stored codegen uses WebKitGradientRead: Linear skips the radii slot, Radial
reads its native pair, and the stop range is read after coordinate output.
Owned/stored variants share a writer that retains radius and stop order,
including duplicate stops and the original comma formatting.
Gradient also keeps a native 16-byte header and one extra slot. A private
12-byte enum holds its linear/radial/conic/WebKit data; repeating and vendor
prefix values remain typed. Linear directions flatten nested angle variants
into an eight-byte enum, while conic angles split into a typed unit and scalar
so the position handle still fits inline. This is necessary layout compression,
not byte serialization. The item range slot is written with the appropriate
length/angle type before its header is published and is reused on every variant
transition. WebKit variants retain the reserved slot but never read its content.
GradientRead exposes linear/radial/conic header fields with typed, context-bound
item-range views. Stored codegen reads items after direction/shape/position
output, using the same variant writers as owned values. Repeating flags, vendor
prefixes, angle units and item ordering are preserved; the WebKit view contains
only its child handle and does not access the reserved range slot.
There are no remaining byte codecs in values/image.rs; the dimension unit
conversion for compact DimensionPercentage lists is still required.

## Native shape and filter storage

ClipPath, BasicShape, ShapeRadius, InsetRect, CircleShape, EllipseShape, Polygon
and DropShadow directly store their native Copy value or handle aggregate in
one payload. Point lists store native eight-byte coordinate-handle pairs;
polygon point order and deep cloning of coordinates remain unchanged.

MaskBorder keeps outset, slice, mode, repeat and the overflow index in a native
16-byte header. Source and width occupy one typed extra slot, reducing the old
two-slot overflow to one. Repeated writes reuse that slot. The earlier Mask
native layout remains unchanged. Both shape modules are free of byte codecs.
MaskBorderRead exposes the native header and source/width pair to stored
codegen. Its image fields share BorderImage's writer, followed by the authored
mask-border mode; owned and stored values use the same serialization path.

Filter and FilterList also fit in one payload. Their full native enums preserve
number versus percentage, angle units, and None versus an empty filter range;
no scalar codec or additional list-element node is required. Filter child nodes
and lists retain their existing deep-clone implementations.

## Native box model and container storage

Size, MaxSize and PositionProperty directly store their native variants,
including vendor-prefix flags and typed children. Size2D and Rect store their
two/four NodeId fields directly. Their Copy implementations depend only on the
handles, not on the child types being Copy; each supported instantiation keeps
its distinct NodeKind and its existing recursive clone behavior.

Container, ContainerCondition, StyleQuery and ScrollStateQuery fit directly in
payloads. They store native operators, container types, declaration/node IDs and
list ranges without rebuilding fields from integer indices. Query-tree cloning
still clones node/list children; StyleQuery declaration references retain the
previous shared-reference behavior. values/box_model.rs and rules/container.rs
contain no byte codecs.

## Native SVG and UI storage

SVGPaint, SVGPaintFallback, StrokeDasharray and Marker directly store their native
Copy enums. Paint retains optional fallback handles, distinguishing no fallback
from an explicit None fallback node (including node index zero). Dash arrays
retain None versus an empty value range.

Cursor, CursorImage, Caret, ListStyle and ColorOrAuto likewise fit natively in
payloads. CursorImage stores Option<(f32, f32)> and its URL handle directly,
preserving optional presence and float bit patterns without reading padding as
integers. CursorKeyword, CaretShape and ListStylePosition are Copy. Existing
recursive clones of cursor images/URLs and other child nodes remain unchanged.
values/svg.rs, rules/ui.rs and values/ui.rs contain no byte codecs.

## Native text storage

TextIndent, TextDecoration and TextEmphasis directly store their native Copy
field aggregates. Spacing, TextDecorationLine, TextDecorationThickness and
Content likewise fit in payloads; decoration lists retain authored order and
duplicates, and TextIndent preserves both flags. TextEmphasisStyle retains its
existing native AstStr representation and context-aware string equality.

TextShadow stores blur/color/x-offset handles and its overflow index in the
native header. A single typed extra slot holds y-offset/spread handles and is
reused on writes. Its codegen field view supplies offsets in CSS order and the
color handle directly. TextShadow and BoxShadow share the final writer, preserving
x/y/blur/spread/color order and the existing box-shadow inset prefix. Child order
and deep cloning remain unchanged. Both text modules now have no byte codecs.

## Native font storage

FontWeight, FontStretch, FontSize, FontStyle, LineHeight and VerticalAlign store
native scalar/handle enums in payloads. Keyword identities, angle units and
float bit patterns remain intact. FontFamily and FontFormat retain their native
AstStr representation and context-aware content equality.

FontFaceStyle and BasePalette also fit inline. Source, FontTechnology,
UnicodeRange and OverrideColors use native eight-byte-or-smaller list slots,
including the padding in OverrideColors through opaque storage. FamilyName
continues to use its native string range.

Font has a native header plus three independently typed slots: family range,
style/weight handle pair, and FontStretch. Writes reuse all three slots. It no
longer reconstructs a multi-slot FontFields aggregate. The context-bound FontRead
view supplies header fields directly and reads the family/stretch slots at their
output positions; stored codegen does not assemble the full logical Font first.
UrlSource's optional format, technology range and URL handle fit in a single
16-byte payload, eliminating its former extra slot. Deep cloning still clones
family/source lists and their child nodes. Both font modules are free of byte
codecs; the font-family minifier algorithm is unchanged.

## Native matrices and transform properties

MatrixForFloat and Matrix3DForFloat retain three inline floats plus an overflow
index in a native header. Their remaining floats are native arrays across two
and seven opaque extra slots, respectively, replacing the former three and
thirteen slots that each held only one f32. Writes reuse the packed tail slots;
field order and floating-point bits remain unchanged.

Stored matrix codegen calls AstContext::matrix_components or
matrix_3d_components to read the header and native tail arrays directly. It
prints their chained values without rebuilding a MatrixForFloat/Matrix3DForFloat
and then gathering its fields into another array. The normal owned-value ToCss
path uses the same number-list writer. Both paths preserve output order and
formatting; no mutable context or storage allocation is involved.

Perspective, Translate and GapValue store their native enums directly. Scale
uses a private 16-byte native enum that flattens three NumberOrPercentage values
into three typed boolean flags and three f32 fields, preserving all eight type
combinations and the distinct None variant. This retains necessary compression
without byte serialization. The general Transform also uses a 16-byte native header and one reusable extra
slot. Its private data enum retains native small variants and flattens only
Scale/Scale3d, Rotate3d and Skew scalar metadata. Translate3d writes a typed z
handle, Scale3d writes an f32, and Rotate3d writes [z, angle_value] to overflow;
other variants never read it. The appropriate slot type is written before the
new header is published on every variant transition. All 21 public variants,
angle units and number/percentage distinctions remain separate. Both transform
modules are now free of byte codecs.

TransformRead retains the small native fields and provides typed tail readers
for Translate3d, Scale3d and Rotate3d. Only those variants access overflow.
Stored and owned values share one variant writer; owned readers hold values
directly, while stored readers borrow the context. Private construction ties
each tail reader to its matching native slot type. Matrix variants retain their
child handles and use the existing matrix codegen entry points.

## Native calculation storage

Calc and MathFunction directly store their native enums, including scalar factors,
ordered child tuples, list ranges and typed rounding strategies. All variants
fit within the existing 16-byte payload; no overflow or child-node layer is
added. Copy/Clone depend on handles rather than requiring the child value type
to be Copy. Value types retain distinct Calc/MathFunction kinds and recursive
clone behavior. length.rs no longer has byte codecs for these nodes; dimension
unit conversion used by compressed list values remains a separate concern.

## Native color storage and removal of byte APIs

PredefinedColor codegen reads a context-bound header view. Its color-space name
comes from the validated stored tag, and its components come directly from the
inline floats and overflow pair, preserving RGB versus XYZ component order.
Stored and owned values share the final color(...) writer; the stored path no
longer constructs an eight-variant logical color only to match it again. This
does not normalize or clamp components and does not change the storage layout.

LABColor and FloatColor also expose context-bound component views for codegen.
They preserve the stored color space and CSS component order without rebuilding
the full enum. LAB lightness scaling and HSL/HWB percentage serialization retain
the existing arithmetic and alpha omission rules. Regression tests compare owned
and stored output across every variant, both printer modes and same-node edits.

LABColor, PredefinedColor and FloatColor keep two inline components and a variant
index in a native header. Their last two f32 components now share one typed
extra slot instead of occupying two slots. Variant/component splitting remains
necessary because the full enums exceed 16 bytes; no bytes are serialized.
UnresolvedColor uses a typed variant header and two reused extra slots. RGB/HSL
publish a scalar and alpha range; LightDark publishes a light range and leaves
the second slot unread. Transitions write the matching slot types first.

UnresolvedColor codegen dispatches through borrowed variant views. RGB/HSL views
can read only their scalar and alpha-range slots; the LightDark view exposes its
light token range without interpreting the unused second slot. The views retain
the node-handle lifetime separately from the context borrow. Stored and owned
paths share serialization, including empty alpha lists and light/dark order.

After the final color consumer migrated, the legacy NodePayload inline,
with_extra, bytes and extra_start APIs and ExtraData byte/integer APIs were
removed. Storage tests now exercise native values and continue checking padding,
kind validation, aligned columns and shared range addressing. No AST storage
implementation uses little-endian byte codecs. Typed unit/value flattening for
oversized values is retained where it provides real layout compression.

## Context identity and ordinary strings

Token-list codegen carries the preceding color-replacement boundary as a boolean.
The stored Function writer returns that state from the replacement it already
read for output, avoiding a second header/overflow read by the following token.
An ordinary Token is also resolved once for both boundary classification and
serialization. Whitespace and closing delimiters retain their exemptions; other
values clear the state after output. Unparsed-token serialization stays separate
and retains its comment/whitespace and replacement-barrier behavior.

Supports context fingerprinting and equality resolve Selector/Unknown text and
declaration values through StringPool. They share SupportsCondition's existing
context-aware leaf equality, so two separately appended identical strings produce
the same context identity and can converge during source-key identity refresh.
A regression first reproduced the old range-identity failure before the fix.

Container names and parser source keys also resolve their stored ranges for
content comparison. The default context repair retains its previous node/list
identity behavior for nested conditions; callers requiring recursive semantic
comparison use the existing custom fingerprint/equality callbacks. This audit
fixes the string-range regression without changing that separate policy.

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

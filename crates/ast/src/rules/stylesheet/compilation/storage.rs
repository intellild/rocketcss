use rocketcss_common::{Allocator, Atom, DenseId, DenseRange, DenseStore};

use crate::Span;

use super::AstContext;

/// Raw identity domain shared by all flattened AST node kinds.
enum RawNodeDomain {}

/// Physical domain of the shared overflow and persistent-list table.
enum ExtraDataDomain {}

enum StringDomain {}

enum AtomDomain {}

/// Compact, hand-assigned discriminator for one logical AST node type.
///
/// Codecs keep their discriminants beside the owning AST type. Zero is
/// reserved so a freshly zeroed payload can never describe a published node;
/// `u16::MAX` marks the slot that is temporarily unavailable to `mutate_node`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NodeKind(u32);

impl NodeKind {
    pub(crate) const MUTATING: Self = Self(u32::MAX);

    #[inline]
    pub(crate) const fn new(discriminant: u32) -> Self {
        assert!(discriminant != 0 && discriminant != u32::MAX);
        Self(discriminant)
    }

    #[inline]
    pub(crate) const fn parameterized(family: u16, parameter: Self) -> Self {
        assert!(family != 0 && family != u16::MAX);
        assert!(parameter.0 <= u16::MAX as u32);
        Self(((family as u32) << 16) | parameter.0)
    }
}

/// One fixed-width node payload.
///
/// Inline nodes use all sixteen bytes. Overflowing nodes use the first twelve
/// bytes inline and store the first shared-extra slot in the final four bytes.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct NodePayload(u128);

impl NodePayload {
    pub(crate) const INLINE_BYTES: usize = 16;
    pub(crate) const PARTIAL_INLINE_BYTES: usize = 12;

    #[inline]
    pub(crate) fn inline(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= Self::INLINE_BYTES);
        let mut payload = [0; Self::INLINE_BYTES];
        payload[..bytes.len()].copy_from_slice(bytes);
        Self(u128::from_le_bytes(payload))
    }

    #[inline]
    pub(crate) fn with_extra(inline: &[u8], extra_start: usize) -> Self {
        assert!(inline.len() <= Self::PARTIAL_INLINE_BYTES);
        let extra_start = u32::try_from(extra_start)
            .expect("AST extra-data start exceeds the compact range capacity");
        let mut payload = [0; Self::INLINE_BYTES];
        payload[..inline.len()].copy_from_slice(inline);
        payload[Self::PARTIAL_INLINE_BYTES..].copy_from_slice(&extra_start.to_le_bytes());
        Self(u128::from_le_bytes(payload))
    }

    #[inline]
    pub(crate) const fn bytes(self) -> [u8; Self::INLINE_BYTES] {
        self.0.to_le_bytes()
    }

    #[inline]
    pub(crate) fn extra_start(self) -> usize {
        let bytes = self.bytes();
        u32::from_le_bytes(
            bytes[Self::PARTIAL_INLINE_BYTES..]
                .try_into()
                .expect("the extra-data offset occupies four bytes"),
        ) as usize
    }
}

/// Hand-written physical layout for one logical AST node type.
///
/// Implementations live beside the AST type they encode. They may only reach
/// backing storage through `AstContext`; the payload itself is a value object.
pub(crate) trait AstNodeStorage<'ast>: Sized {
    const KIND: NodeKind;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self;

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload;

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload;
}

/// Context-aware deep cloning for a node whose physical codec is available.
pub(crate) trait AstNodeClone<'ast>: AstNodeStorage<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self;
}

/// One untagged slot in the shared overflow and persistent-list table.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ExtraData(u64);

impl ExtraData {
    pub(crate) const BYTES: usize = 8;

    #[inline]
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= Self::BYTES);
        let mut slot = [0; Self::BYTES];
        slot[..bytes.len()].copy_from_slice(bytes);
        Self(u64::from_le_bytes(slot))
    }

    #[inline]
    pub(crate) const fn bytes(self) -> [u8; Self::BYTES] {
        self.0.to_le_bytes()
    }

    #[inline]
    pub(crate) const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) const fn as_u64(self) -> u64 {
        self.0
    }
}

/// Compact representation of one typed field or persistent-list element.
pub(crate) trait ExtraDataCompact<'ast>: Sized {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData;

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self;
}

/// Context-aware cloning for one logical value stored in `ExtraData`.
pub(crate) trait ExtraDataClone<'ast>: ExtraDataCompact<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self;
}

macro_rules! impl_scalar_extra {
    ($type:ty, $encode:expr, $decode:expr) => {
        impl ExtraDataCompact<'_> for $type {
            #[inline]
            fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
                ExtraData::from_u64($encode(self))
            }

            #[inline]
            fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
                $decode(data.as_u64())
            }
        }
    };
}

impl_scalar_extra!(u8, |value: u8| value as u64, |value: u64| value as u8);
impl_scalar_extra!(u16, |value: u16| value as u64, |value: u64| value as u16);
impl_scalar_extra!(u32, |value: u32| value as u64, |value: u64| value as u32);
impl_scalar_extra!(i32, |value: i32| value as u32 as u64, |value: u64| {
    value as u32 as i32
});
impl_scalar_extra!(f32, |value: f32| value.to_bits() as u64, |value: u64| {
    f32::from_bits(value as u32)
});

macro_rules! impl_copy_extra_clone {
    ($($type:ty),+ $(,)?) => {
        $(
            impl ExtraDataClone<'_> for $type {
                #[inline]
                fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
                    self
                }
            }
        )+
    };
}

impl_copy_extra_clone!(u8, u16, u32, i32, f32, bool);

impl ExtraDataCompact<'_> for bool {
    #[inline]
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(self as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => false,
            1 => true,
            _ => panic!("invalid encoded bool"),
        }
    }
}

impl<'ast, T> ExtraDataCompact<'ast> for DenseId<'ast, T> {
    #[inline]
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64(self.index() as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let index = usize::try_from(data.as_u64()).expect("AST node ID exceeds usize");
        context.encoded_node_id_at(index)
    }
}

impl<'ast, T: AstNodeClone<'ast>> ExtraDataClone<'ast> for DenseId<'ast, T> {
    #[inline]
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        context.clone_encoded_node(self)
    }
}

impl<'ast, T> ExtraDataCompact<'ast> for Option<DenseId<'ast, T>> {
    #[inline]
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let index = self.map_or(u32::MAX, |id| {
            u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
        });
        ExtraData::from_u64(index as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let index = u32::try_from(data.as_u64()).expect("optional AST node ID exceeds four bytes");
        (index != u32::MAX).then(|| context.encoded_node_id_at(index as usize))
    }
}

impl<'ast, T: AstNodeClone<'ast>> ExtraDataClone<'ast> for Option<DenseId<'ast, T>> {
    #[inline]
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.map(|id| context.clone_encoded_node(id))
    }
}

impl<'ast, T> ExtraDataCompact<'ast> for DenseRange<'ast, T> {
    #[inline]
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let start = u32::try_from(self.start_index()).expect("AST range start exceeds four bytes");
        let end = u32::try_from(self.end_index()).expect("AST range end exceeds four bytes");
        ExtraData::from_u64((end as u64) << 32 | start as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, _context: &AstContext<'ast>) -> Self {
        let start = data.as_u64() as u32 as usize;
        let end = (data.as_u64() >> 32) as u32 as usize;
        // SAFETY: the range was produced by `ExtraDataStore` and encoded as
        // one typed field. Access validates its bounds against the same store.
        unsafe { DenseRange::from_indices_unchecked(start, end) }
    }
}

impl<'ast, T> ExtraDataClone<'ast> for DenseRange<'ast, T>
where
    T: ExtraDataCompact<'ast> + ExtraDataClone<'ast>,
{
    #[inline]
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        context.clone_encoded_vec(self)
    }
}

impl<'ast> ExtraDataCompact<'ast> for &'ast str {
    #[inline]
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64(context.store_string(self) as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        context.resolve_string(data.as_u64())
    }
}

impl<'ast> ExtraDataClone<'ast> for &'ast str {
    #[inline]
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

impl<'ast> ExtraDataCompact<'ast> for Option<&'ast str> {
    #[inline]
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        let index = self.map_or(u32::MAX, |value| context.store_string(value));
        ExtraData::from_u64(index as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let index =
            u32::try_from(data.as_u64()).expect("optional AST string ID exceeds four bytes");
        (index != u32::MAX).then(|| context.resolve_string(index as u64))
    }
}

impl<'ast> ExtraDataClone<'ast> for Option<&'ast str> {
    #[inline]
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

impl<'ast> ExtraDataCompact<'ast> for Atom<'ast> {
    #[inline]
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64(context.store_atom(self) as u64)
    }

    #[inline]
    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        context.resolve_atom(data.as_u64())
    }
}

impl<'ast> ExtraDataClone<'ast> for Atom<'ast> {
    #[inline]
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

/// Context-owned tables referenced by compact string and atom IDs.
pub(crate) struct StringData<'ast> {
    strings: DenseStore<'ast, StringDomain, &'ast str>,
    atoms: DenseStore<'ast, AtomDomain, Atom<'ast>>,
}

impl<'ast> StringData<'ast> {
    pub(crate) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            strings: DenseStore::new_in(allocator),
            atoms: DenseStore::new_in(allocator),
        }
    }

    pub(crate) fn store_string(&mut self, value: &'ast str) -> u32 {
        let id = self.strings.push(value);
        u32::try_from(id.index()).expect("AST string ID exceeds four bytes")
    }

    pub(crate) fn resolve_string(&self, index: u64) -> &'ast str {
        let index = usize::try_from(index).expect("AST string ID exceeds usize");
        let id = self
            .strings
            .id_at_offset(0, index)
            .expect("AST string ID does not belong to this context");
        self.strings[id]
    }

    pub(crate) fn store_atom(&mut self, value: Atom<'ast>) -> u32 {
        let id = self.atoms.push(value);
        u32::try_from(id.index()).expect("AST atom ID exceeds four bytes")
    }

    pub(crate) fn resolve_atom(&self, index: u64) -> Atom<'ast> {
        let index = usize::try_from(index).expect("AST atom ID exceeds usize");
        let id = self
            .atoms
            .id_at_offset(0, index)
            .expect("AST atom ID does not belong to this context");
        self.atoms[id]
    }

    pub(crate) fn lengths(&self) -> (usize, usize) {
        (self.strings.len(), self.atoms.len())
    }

    pub(crate) fn truncate(&mut self, string_len: usize, atom_len: usize) {
        self.strings.truncate(string_len);
        self.atoms.truncate(atom_len);
    }
}

/// Aligned structure-of-arrays storage for every flattened AST node.
pub(crate) struct NodeData<'ast> {
    spans: DenseStore<'ast, RawNodeDomain, Span>,
    kinds: DenseStore<'ast, RawNodeDomain, NodeKind>,
    payloads: DenseStore<'ast, RawNodeDomain, NodePayload>,
}

impl<'ast> NodeData<'ast> {
    pub(crate) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            spans: DenseStore::new_in(allocator),
            kinds: DenseStore::new_in(allocator),
            payloads: DenseStore::new_in(allocator),
        }
    }

    pub(crate) fn alloc<T>(
        &mut self,
        kind: NodeKind,
        payload: NodePayload,
        span: Span,
    ) -> DenseId<'ast, T> {
        assert!(
            kind != NodeKind::MUTATING,
            "cannot publish a mutation marker"
        );
        assert!(
            self.spans.has_capacity_for(1)
                && self.kinds.has_capacity_for(1)
                && self.payloads.has_capacity_for(1),
            "AST node count exceeds dense ID capacity"
        );

        let span_id = self.spans.push(span);
        let kind_id = self.kinds.push(kind);
        let payload_id = self.payloads.push(payload);
        debug_assert_eq!(span_id, kind_id);
        debug_assert_eq!(kind_id, payload_id);
        self.assert_aligned();

        // SAFETY: all three columns use the same dense integer identity. This
        // method is the sole typed publication boundary and has recorded the
        // caller's hand-written `NodeKind` before returning the ID.
        unsafe { std::mem::transmute(payload_id) }
    }

    #[inline]
    pub(crate) fn payload<T>(&self, id: DenseId<'ast, T>, expected: NodeKind) -> NodePayload {
        self.validate(id.index(), expected);
        self.payloads.as_slice()[id.index()]
    }

    pub(crate) fn id_at<T>(&self, index: usize) -> DenseId<'ast, T> {
        let raw = self
            .payloads
            .id_at_offset(0, index)
            .expect("AST node ID does not belong to this context");
        // SAFETY: every typed AST identity uses this table's raw dense index.
        // The owning field codec determines `T`; a later resolution validates
        // its associated `NodeKind` before decoding the payload.
        unsafe { std::mem::transmute(raw) }
    }

    #[inline]
    pub(crate) fn span<T>(&self, id: DenseId<'ast, T>, expected: NodeKind) -> Span {
        self.validate(id.index(), expected);
        self.spans.as_slice()[id.index()]
    }

    #[inline]
    pub(crate) fn set_span<T>(&mut self, id: DenseId<'ast, T>, expected: NodeKind, span: Span) {
        self.validate(id.index(), expected);
        self.spans.as_mut_slice()[id.index()] = span;
    }

    /// Makes a node slot unavailable and returns its encoded payload.
    pub(crate) fn begin_mutation<T>(
        &mut self,
        id: DenseId<'ast, T>,
        expected: NodeKind,
    ) -> NodePayload {
        self.validate(id.index(), expected);
        self.kinds.as_mut_slice()[id.index()] = NodeKind::MUTATING;
        self.payloads.as_slice()[id.index()]
    }

    /// Publishes an updated payload under the original typed identity.
    pub(crate) fn finish_mutation<T>(
        &mut self,
        id: DenseId<'ast, T>,
        kind: NodeKind,
        payload: NodePayload,
    ) {
        assert!(
            kind != NodeKind::MUTATING,
            "cannot publish a mutation marker"
        );
        let index = id.index();
        assert_eq!(
            self.kinds.as_slice()[index],
            NodeKind::MUTATING,
            "only an active node mutation can publish an existing slot"
        );
        self.payloads.as_mut_slice()[index] = payload;
        self.kinds.as_mut_slice()[index] = kind;
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.assert_aligned();
        self.payloads.len()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(
            !self.kinds.as_slice().contains(&NodeKind::MUTATING),
            "cannot roll back AST storage during a node mutation"
        );
        self.spans.truncate(len);
        self.kinds.truncate(len);
        self.payloads.truncate(len);
        self.assert_aligned();
    }

    #[inline]
    fn validate(&self, index: usize, expected: NodeKind) {
        let actual = self
            .kinds
            .as_slice()
            .get(index)
            .copied()
            .expect("AST node ID does not belong to this context");
        assert!(
            actual != NodeKind::MUTATING,
            "recursive access to a mutably borrowed AST node"
        );
        assert_eq!(
            actual, expected,
            "AST node ID used with the wrong node kind"
        );
    }

    #[inline]
    fn assert_aligned(&self) {
        debug_assert_eq!(self.spans.len(), self.kinds.len());
        debug_assert_eq!(self.kinds.len(), self.payloads.len());
    }
}

/// Shared append-only backing table for node overflow fields and AST lists.
pub(crate) struct ExtraDataStore<'ast> {
    slots: DenseStore<'ast, ExtraDataDomain, ExtraData>,
}

impl<'ast> ExtraDataStore<'ast> {
    pub(crate) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            slots: DenseStore::new_in(allocator),
        }
    }

    pub(crate) fn alloc<T>(
        &mut self,
        slots: impl ExactSizeIterator<Item = ExtraData>,
    ) -> DenseRange<'ast, T> {
        let start = self.slots.len();
        let len = slots.len();
        assert!(
            self.slots.has_capacity_for(len),
            "AST extra-data count exceeds dense range capacity"
        );
        self.slots.reserve(len);
        for slot in slots {
            self.slots.push(slot);
        }
        let end = self.slots.len();
        // SAFETY: the range directly names the consecutive slots appended by
        // this call. Typed interpretation is supplied by the owning codec.
        unsafe { DenseRange::from_indices_unchecked(start, end) }
    }

    #[inline]
    pub(crate) fn get<T>(&self, range: DenseRange<'_, T>, index: usize) -> Option<ExtraData> {
        if index >= range.len() {
            return None;
        }
        self.slots
            .as_slice()
            .get(range.start_index() + index)
            .copied()
    }

    #[inline]
    pub(crate) fn set<T>(&mut self, range: DenseRange<'_, T>, index: usize, value: ExtraData) {
        assert!(index < range.len(), "AST list index out of bounds");
        self.slots.as_mut_slice()[range.start_index() + index] = value;
    }

    #[inline]
    pub(crate) fn get_at(&self, index: usize) -> ExtraData {
        self.slots.as_slice()[index]
    }

    #[inline]
    pub(crate) fn set_at(&mut self, index: usize, value: ExtraData) {
        self.slots.as_mut_slice()[index] = value;
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.slots.len()
    }

    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        self.slots.truncate(len);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        mem::size_of,
        panic::{AssertUnwindSafe, catch_unwind},
    };

    use rocketcss_common::Allocator;

    use crate::{AstContext, DUMMY_SP, Span, Token};

    use super::{ExtraData, ExtraDataStore, NodeData, NodeKind, NodePayload};

    enum InlineNode {}
    enum OverflowNode {}

    const INLINE_KIND: NodeKind = NodeKind::new(1);
    const OVERFLOW_KIND: NodeKind = NodeKind::new(2);

    #[test]
    fn physical_slots_have_fixed_widths() {
        assert_eq!(size_of::<NodePayload>(), 16);
        assert_eq!(size_of::<ExtraData>(), 8);
    }

    #[test]
    fn payload_preserves_inline_bytes_and_extra_start() {
        let inline = NodePayload::inline(&[1, 2, 3, 4]);
        assert_eq!(&inline.bytes()[..4], &[1, 2, 3, 4]);
        assert_eq!(&inline.bytes()[4..], &[0; 12]);

        let overflow = NodePayload::with_extra(&[9; 12], 0x0102_0304);
        assert_eq!(&overflow.bytes()[..12], &[9; 12]);
        assert_eq!(overflow.extra_start(), 0x0102_0304);
    }

    #[test]
    fn node_columns_stay_aligned_and_validate_kinds() {
        let allocator = Allocator::new();
        let mut nodes = NodeData::new_in(&allocator);
        let id = nodes.alloc::<InlineNode>(INLINE_KIND, NodePayload::inline(&[7]), DUMMY_SP);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes.payload(id, INLINE_KIND).bytes()[0], 7);

        let replacement = Span::new(3, 8);
        nodes.set_span(id, INLINE_KIND, replacement);
        assert_eq!(nodes.span(id, INLINE_KIND), replacement);

        let payload = nodes.begin_mutation(id, INLINE_KIND);
        nodes.finish_mutation(id, INLINE_KIND, payload);
        assert_eq!(nodes.payload(id, INLINE_KIND).bytes()[0], 7);

        nodes.truncate(0);
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    #[should_panic(expected = "wrong node kind")]
    fn node_kind_rejects_a_mismatched_typed_decode() {
        let allocator = Allocator::new();
        let mut nodes = NodeData::new_in(&allocator);
        let id =
            nodes.alloc::<OverflowNode>(OVERFLOW_KIND, NodePayload::with_extra(&[], 0), DUMMY_SP);
        nodes.payload(id, INLINE_KIND);
    }

    #[test]
    fn extra_ranges_directly_address_shared_slots() {
        let allocator = Allocator::new();
        let mut extra = ExtraDataStore::new_in(&allocator);
        let range = extra.alloc::<u16>(
            [
                ExtraData::from_bytes(&[1, 2]),
                ExtraData::from_bytes(&[3, 4]),
            ]
            .into_iter(),
        );
        assert_eq!(range.start_index(), 0);
        assert_eq!(range.end_index(), 2);
        assert_eq!(&extra.get(range, 1).unwrap().bytes()[..2], &[3, 4]);

        extra.set(range, 0, ExtraData::from_bytes(&[8, 9]));
        assert_eq!(&extra.get_at(0).bytes()[..2], &[8, 9]);
        extra.set_at(1, ExtraData::from_bytes(&[6]));
        assert_eq!(extra.get(range, 1).unwrap().bytes()[0], 6);

        extra.truncate(0);
        assert_eq!(extra.len(), 0);
    }

    #[test]
    fn context_encodes_compact_values_and_string_ids_into_shared_extra() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);

        let numbers = context.alloc_encoded_vec([3_u32, 5, 8].into_iter());
        assert_eq!(context.encoded_vec_get(numbers, 1), Some(5));
        context.encoded_vec_set(numbers, 1, 13);
        assert_eq!(context.encoded_vec_get(numbers, 1), Some(13));

        let checkpoint = context.node_checkpoint();
        let strings = context.alloc_encoded_vec(["alpha", "beta"].into_iter());
        assert_eq!(context.encoded_vec_get(strings, 0), Some("alpha"));
        assert_eq!(context.encoded_vec_get(strings, 1), Some("beta"));

        let optional_strings = context.alloc_encoded_vec([Some("named"), None::<&str>].into_iter());
        let cloned_optional_strings = context.clone_encoded_vec(optional_strings);
        assert_eq!(
            context
                .encoded_vec_iter(cloned_optional_strings)
                .collect::<std::vec::Vec<_>>(),
            [Some("named"), None]
        );

        context.restore_node_checkpoint(checkpoint);
        assert_eq!(context.encoded_vec_get(strings, 0), None);
    }

    #[test]
    fn compact_list_context_api_iterates_mutates_and_rewrites() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let mut numbers = context.alloc_encoded_vec([3_i32, 5, 8].into_iter());

        assert_eq!(
            context
                .encoded_vec_iter(numbers)
                .collect::<std::vec::Vec<_>>(),
            [3, 5, 8]
        );
        context.mutate_encoded_vec(numbers, |values, context| {
            values[1] = 13;
            let other = context.alloc_encoded_vec([21_i32].into_iter());
            assert_eq!(context.encoded_vec_get(other, 0), Some(21));
        });
        assert_eq!(
            context
                .encoded_vec_iter(numbers)
                .collect::<std::vec::Vec<_>>(),
            [3, 13, 8]
        );

        context.rewrite_encoded_vec(&mut numbers, |values, _| {
            values.remove(0);
            values.push(34);
        });
        assert_eq!(
            context
                .encoded_vec_iter(numbers)
                .collect::<std::vec::Vec<_>>(),
            [13, 8, 34]
        );
    }

    #[test]
    fn compact_list_mutation_commits_before_resuming_an_unwind() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let numbers = context.alloc_encoded_vec([1_i32, 2].into_iter());
        let result = catch_unwind(AssertUnwindSafe(|| {
            context.mutate_encoded_vec(numbers, |values, _| {
                values[0] = 9;
                panic!("stop list mutation");
            });
        }));

        assert!(result.is_err());
        assert_eq!(context.encoded_vec_get(numbers, 0), Some(9));
        assert_eq!(context.encoded_vec_get(numbers, 1), Some(2));
    }

    #[test]
    fn compact_list_clone_deep_clones_node_id_elements() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let source = context.alloc_encoded_node(Token::Ident("source"), Span::new(4, 10));
        let values = context.alloc_encoded_vec([source].into_iter());
        let cloned_values = context.clone_encoded_vec(values);
        let cloned = context.encoded_vec_get(cloned_values, 0).unwrap();

        assert_ne!(source, cloned);
        assert_eq!(context.encoded_node(cloned), Token::Ident("source"));
        assert_eq!(context.encoded_node_span(cloned), Span::new(4, 10));
    }
}

use rocketcss_common::{Allocator, AstStr, Atom, DenseId, DenseRange, DenseStore};

use crate::Span;

use super::AstContext;

/// Raw identity domain shared by all flattened AST node kinds.
enum RawNodeDomain {}

/// Physical domain of the shared overflow and persistent-list table.
enum ExtraDataDomain {}

/// Compact, hand-assigned discriminator for one logical AST node type.
///
/// Codecs keep their discriminants beside the owning AST type. Zero is
/// reserved so a freshly zeroed payload can never describe a published node;
/// `u32::MAX` marks the slot that is temporarily unavailable to `mutate_node`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub struct NodeKind(u32);

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

/// A fixed-width opaque payload. Copying preserves padding initialization.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct NodePayload([std::mem::MaybeUninit<u8>; 16]);

impl Default for NodePayload {
    fn default() -> Self {
        Self([std::mem::MaybeUninit::new(0); 16])
    }
}
impl std::fmt::Debug for NodePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NodePayload(..)")
    }
}
impl NodePayload {
    #[inline]
    pub(crate) fn from_value<T: Copy>(value: T) -> Self {
        const {
            assert!(std::mem::size_of::<T>() <= 16);
        }
        let mut slot = Self::default();
        // SAFETY: T fits; MaybeUninit preserves any padding in the typed write.
        unsafe {
            slot.0.as_mut_ptr().cast::<T>().write_unaligned(value);
        }
        slot
    }

    /// # Safety
    /// The slot must contain a value written as the same T by from_value.
    #[inline]
    pub(crate) unsafe fn read_value<T: Copy>(self) -> T {
        const {
            assert!(std::mem::size_of::<T>() <= 16);
        }
        // SAFETY: the caller provides a matching initialized T.
        unsafe { self.0.as_ptr().cast::<T>().read_unaligned() }
    }
}

/// Hand-written physical layout for one logical AST node type.
///
/// Implementations live beside the AST type they encode. They may only reach
/// backing storage through `AstContext`; the payload itself is a value object.
#[doc(hidden)]
/// # Safety
/// KIND must uniquely identify this storage layout. New writes, reads and
/// replacements must agree on initialized field types and overflow ownership.
///
/// A raw slot cannot be decoded through the safe public API:
/// ```compile_fail
/// use rocketcss_ast::{AstContext, AstNodeStorage, Token};
/// use rocketcss_common::Allocator;
/// let allocator = Allocator::new();
/// let mut ast = AstContext::new_in(&allocator);
/// let payload = Token::Number(1.0).encode_new(&mut ast);
/// let _ = Token::decode(payload, &ast);
/// ```
pub unsafe trait AstNodeStorage<'ast>: Sized {
    const KIND: NodeKind;

    /// Compares logical values, resolving any ordinary text ranges through their owner.
    fn eq_in_context(&self, other: &Self, _context: &AstContext<'_>) -> bool
    where
        Self: PartialEq,
    {
        self == other
    }

    /// # Safety
    /// The payload must have been written for Self in this context.
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self;

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload;

    /// # Safety
    /// Current must be a payload previously written for Self in this context.
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload;
}

/// Context-aware deep cloning for a node whose physical codec is available.
#[doc(hidden)]
pub trait AstNodeClone<'ast>: AstNodeStorage<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self;
}

/// One opaque slot in the shared overflow and persistent-list table.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct ExtraData([std::mem::MaybeUninit<u8>; 8]);
impl Default for ExtraData {
    fn default() -> Self {
        Self([std::mem::MaybeUninit::new(0); 8])
    }
}
impl std::fmt::Debug for ExtraData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ExtraData(..)")
    }
}
impl ExtraData {
    pub(crate) const BYTES: usize = 8;
    #[inline]
    pub(crate) fn from_value<T: Copy>(value: T) -> Self {
        const {
            assert!(std::mem::size_of::<T>() <= 8);
        }
        let mut slot = Self::default();
        // SAFETY: T fits and MaybeUninit preserves its padding.
        unsafe {
            slot.0.as_mut_ptr().cast::<T>().write_unaligned(value);
        }
        slot
    }
    /// # Safety
    /// The slot must contain a value written as the same T by from_value.
    #[inline]
    pub(crate) unsafe fn read_value<T: Copy>(self) -> T {
        const {
            assert!(std::mem::size_of::<T>() <= 8);
        }
        // SAFETY: the caller provides a matching initialized T.
        unsafe { self.0.as_ptr().cast::<T>().read_unaligned() }
    }
    /// Stores one Copy value across a fixed number of opaque slots.
    #[inline]
    pub(crate) fn from_value_array<T: Copy, const N: usize>(value: T) -> [Self; N] {
        const {
            assert!(std::mem::size_of::<T>() <= std::mem::size_of::<[Self; N]>());
        }
        let mut slots = [Self::default(); N];
        // SAFETY: the array has enough contiguous storage; padding stays opaque.
        unsafe {
            slots.as_mut_ptr().cast::<T>().write_unaligned(value);
        }
        slots
    }

    /// # Safety
    /// These slots must contain the same T written by from_value_array, in order.
    #[inline]
    pub(crate) unsafe fn read_value_array<T: Copy, const N: usize>(slots: [Self; N]) -> T {
        const {
            assert!(std::mem::size_of::<T>() <= std::mem::size_of::<[Self; N]>());
        }
        // SAFETY: the caller supplies the matching initialized value.
        unsafe { slots.as_ptr().cast::<T>().read_unaligned() }
    }
}

/// Compact representation of one typed field or persistent-list element.
#[doc(hidden)]
/// # Safety
/// Writes and reads must agree on the slot's initialized types and ownership.
pub unsafe trait ExtraDataCompact<'ast>: Sized {
    fn encode_extra(self) -> ExtraData;

    /// # Safety
    /// Data must have been written for Self. Decoding copies the stored value;
    /// referenced nodes and ranges retain their owning-context requirement.
    unsafe fn decode_extra(data: ExtraData) -> Self;
}

/// Context-aware cloning for one logical value stored in `ExtraData`.
#[doc(hidden)]
pub trait ExtraDataClone<'ast>: ExtraDataCompact<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self;
}

macro_rules! impl_scalar_extra {
    ($($type:ty),+ $(,)?) => { $(
        unsafe impl ExtraDataCompact<'_> for $type {
            #[inline]
            fn encode_extra(self) -> ExtraData {
                ExtraData::from_value(self)
            }
            #[inline]
            unsafe fn decode_extra(data: ExtraData) -> Self {
                unsafe { data.read_value() }
            }
        }
    )+ };
}
impl_scalar_extra!(u8, u16, u32, i32, f32, bool);

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

// SAFETY: the typed range stores and reads the same native DenseId<'ast, T> representation.
unsafe impl<'ast, T> ExtraDataCompact<'ast> for DenseId<'ast, T> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        // SAFETY: the list/field owner supplies a slot written for Self.
        unsafe { data.read_value() }
    }
}

impl<'ast, T: AstNodeClone<'ast>> ExtraDataClone<'ast> for DenseId<'ast, T> {
    #[inline]
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        context.clone_encoded_node(self)
    }
}

// SAFETY: the typed range stores and reads the same native Option<DenseId<'ast, T>> representation.
unsafe impl<'ast, T> ExtraDataCompact<'ast> for Option<DenseId<'ast, T>> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        // SAFETY: the list/field owner supplies a slot written for Self.
        unsafe { data.read_value() }
    }
}

impl<'ast, T: AstNodeClone<'ast>> ExtraDataClone<'ast> for Option<DenseId<'ast, T>> {
    #[inline]
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.map(|id| context.clone_encoded_node(id))
    }
}

// SAFETY: the typed range stores and reads the same native DenseRange<'ast, T> representation.
unsafe impl<'ast, T> ExtraDataCompact<'ast> for DenseRange<'ast, T> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        // SAFETY: the list/field owner supplies a slot written for Self.
        unsafe { data.read_value() }
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

// DenseRange is repr(C), two initialized u32 bounds with no padding. Its
// construction contract forbids reversed bounds, so [1, 0] represents None.
union OptionalRangeSlot<'ast, T> {
    value: DenseRange<'ast, T>,
    bounds: [u32; 2],
}

impl<T> Copy for OptionalRangeSlot<'_, T> {}
impl<T> Clone for OptionalRangeSlot<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: typed slots contain OptionalRangeSlot<T>. Only valid DenseRange
// values or the None sentinel are written; the sentinel is checked first.
unsafe impl<'ast, T> ExtraDataCompact<'ast> for Option<DenseRange<'ast, T>> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(match self {
            Some(value) => OptionalRangeSlot { value },
            None => OptionalRangeSlot { bounds: [1, 0] },
        })
    }

    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        let slot: OptionalRangeSlot<'ast, T> = unsafe { data.read_value() };
        // Both union fields occupy the same eight initialized bytes.
        if unsafe { slot.bounds } == [1, 0] {
            None
        } else {
            Some(unsafe { slot.value })
        }
    }
}

impl<'ast, T> ExtraDataClone<'ast> for Option<DenseRange<'ast, T>>
where
    T: ExtraDataCompact<'ast> + ExtraDataClone<'ast>,
{
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        self.map(|range| context.clone_encoded_vec(range))
    }
}

// AstStr is repr(C), two initialized u32 fields with no padding. A reversed
// range cannot be produced by StringPool, so (1, 0) represents None without
// consuming an extra slot or conflating Some(EMPTY) with None.
#[derive(Clone, Copy)]
union OptionalStringSlot<'ast> {
    value: AstStr<'ast>,
    bounds: [u32; 2],
}

// SAFETY: these slots are written as OptionalStringSlot. The only value not
// written from a valid AstStr is the None sentinel, checked before reading it.
unsafe impl<'ast> ExtraDataCompact<'ast> for Option<AstStr<'ast>> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(match self {
            Some(value) => OptionalStringSlot { value },
            None => OptionalStringSlot { bounds: [1, 0] },
        })
    }

    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        let slot: OptionalStringSlot<'ast> = unsafe { data.read_value() };
        // Both union fields occupy the same eight initialized bytes.
        if unsafe { slot.bounds } == [1, 0] {
            None
        } else {
            Some(unsafe { slot.value })
        }
    }
}

impl<'ast> ExtraDataClone<'ast> for Option<AstStr<'ast>> {
    #[inline]
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

macro_rules! impl_string_extra {
    ($type:ty) => {
        unsafe impl<'ast> ExtraDataCompact<'ast> for $type {
            #[inline]
            fn encode_extra(self) -> ExtraData {
                ExtraData::from_value(self)
            }
            #[inline]
            unsafe fn decode_extra(data: ExtraData) -> Self {
                unsafe { data.read_value() }
            }
        }
        impl<'ast> ExtraDataClone<'ast> for $type {
            #[inline]
            fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
                self
            }
        }
    };
}
impl_string_extra!(Atom<'ast>);
impl_string_extra!(rocketcss_common::AstStr<'ast>);

/// Aligned structure-of-arrays storage for every flattened AST node.
pub(crate) struct NodeData<'ast> {
    spans: DenseStore<'ast, RawNodeDomain, Span>,
    kinds: DenseStore<'ast, RawNodeDomain, NodeKind>,
    payloads: DenseStore<'ast, RawNodeDomain, NodePayload>,
    active_mutations: usize,
}

impl<'ast> NodeData<'ast> {
    pub(crate) fn new_in(allocator: &'ast Allocator) -> Self {
        Self {
            spans: DenseStore::new_in(allocator),
            kinds: DenseStore::new_in(allocator),
            payloads: DenseStore::new_in(allocator),
            active_mutations: 0,
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
        self.active_mutations += 1;
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
        self.active_mutations = self
            .active_mutations
            .checked_sub(1)
            .expect("publishing a node requires an active mutation");
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.assert_aligned();
        self.payloads.len()
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(
            self.active_mutations == 0,
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

    use rocketcss_common::{Allocator, AstStr};

    use crate::{AstContext, DUMMY_SP, Span, Token};

    use super::{ExtraData, ExtraDataStore, NodeData, NodeKind, NodePayload};

    enum InlineNode {}
    enum OverflowNode {}

    const INLINE_KIND: NodeKind = NodeKind::new(1);
    const OVERFLOW_KIND: NodeKind = NodeKind::new(2);

    #[test]
    fn opaque_slots_preserve_padded_native_values_through_moves() {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Padded {
            flag: bool,
            number: u32,
        }
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Value {
            Empty,
            Flag(bool),
            Number(u32),
        }

        let expected = Padded {
            flag: true,
            number: 0x1234_5678,
        };
        let payload = NodePayload::from_value(expected);
        let mut moved = std::vec::Vec::new();
        moved.push(payload);
        moved.reserve(1024);
        // SAFETY: this payload was written as Padded above, with padding preserved.
        assert_eq!(unsafe { moved[0].read_value::<Padded>() }, expected);
        for expected in [
            Value::Empty,
            Value::Flag(false),
            Value::Flag(true),
            Value::Number(u32::MAX),
        ] {
            let slot = ExtraData::from_value(expected);
            // SAFETY: the slot was written as the same Value type.
            assert_eq!(unsafe { slot.read_value::<Value>() }, expected);
        }
        assert_eq!(std::mem::size_of::<NodePayload>(), 16);
        assert_eq!(std::mem::size_of::<ExtraData>(), 8);
    }

    #[test]
    fn native_value_crosses_extra_slot_boundary_with_padding_intact() {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Padded {
            flag: bool,
            number: u64,
        }
        let expected = Padded {
            flag: true,
            number: u64::MAX,
        };
        let [first, second] = ExtraData::from_value_array(expected);
        let mut moved = std::vec::Vec::from([first, second]);
        moved.reserve(1024);
        // SAFETY: the same pair is reconstructed in its original order.
        let actual: Padded = unsafe { ExtraData::read_value_array([moved[0], moved[1]]) };
        assert_eq!(actual, expected);
    }

    #[test]
    fn physical_slots_have_fixed_widths() {
        assert_eq!(size_of::<NodePayload>(), 16);
        assert_eq!(size_of::<ExtraData>(), 8);
    }

    #[test]
    fn native_slots_preserve_references_and_high_alignment_across_moves() {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Borrowed<'a> {
            flag: bool,
            value: &'a u32,
        }
        #[repr(C, align(16))]
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Aligned([u8; 16]);

        assert_eq!(std::mem::align_of::<ExtraData>(), 8);
        assert_eq!(std::mem::align_of::<Aligned>(), 16);
        let value = 0x1234_5678;
        for flag in [false, true] {
            let expected = Borrowed {
                flag,
                value: &value,
            };
            let payload = NodePayload::from_value(expected);
            let mut payloads = std::vec::Vec::from([payload]);
            payloads.reserve(1024);
            // SAFETY: the moved payload still contains the same Borrowed type.
            let actual: Borrowed<'_> = unsafe { payloads[0].read_value() };
            assert_eq!(actual, expected);
            assert!(std::ptr::eq(actual.value, &value));

            let [first, second] = ExtraData::from_value_array(expected);
            let mut slots = std::vec::Vec::from([first, second]);
            slots.reserve(1024);
            // SAFETY: original slots are reassembled in their original order.
            let actual: Borrowed<'_> = unsafe { ExtraData::read_value_array([slots[0], slots[1]]) };
            assert_eq!(actual.flag, flag);
            assert!(std::ptr::eq(actual.value, &value));
            assert_eq!(*actual.value, value);
        }
        let expected = Aligned(std::array::from_fn(|index| index as u8));
        let slots: [ExtraData; 2] = ExtraData::from_value_array(expected);
        // SAFETY: the slots contain Aligned. The reader must support T's alignment
        // exceeding that of the slot array, without constructing an aligned &T.
        assert_eq!(
            unsafe { ExtraData::read_value_array::<Aligned, 2>(slots) },
            expected
        );
    }

    #[test]
    fn native_list_stride_preserves_small_valid_values_after_reallocation() {
        use std::num::NonZeroU32;

        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Small {
            Empty,
            Flag(bool),
            NonZero(NonZeroU32),
        }
        let allocator = Allocator::new();
        let mut extra = ExtraDataStore::new_in(&allocator);
        let bytes = extra.alloc::<u8>((0..=u8::MAX).map(ExtraData::from_value));
        let expected = [
            Small::Empty,
            Small::Flag(false),
            Small::Flag(true),
            Small::NonZero(NonZeroU32::new(1).unwrap()),
            Small::NonZero(NonZeroU32::new(u32::MAX).unwrap()),
        ];
        let values = extra.alloc::<Small>(expected.into_iter().map(ExtraData::from_value));
        let _growth =
            extra.alloc::<u64>((0..4096_u32).map(|value| ExtraData::from_value(u64::from(value))));
        assert_eq!(bytes.len(), 256);
        assert_eq!(values.start_index(), bytes.end_index());
        for (index, expected) in (0..=u8::MAX).enumerate() {
            // SAFETY: each element occupies one slot and was written as u8.
            assert_eq!(
                unsafe { extra.get(bytes, index).unwrap().read_value::<u8>() },
                expected
            );
        }
        for (index, expected) in expected.into_iter().enumerate() {
            // SAFETY: the range contains valid Small values, not arbitrary bytes.
            assert_eq!(
                unsafe { extra.get(values, index).unwrap().read_value::<Small>() },
                expected
            );
        }
        extra.set(bytes, 127, ExtraData::from_value(0_u8));
        assert_eq!(
            unsafe { extra.get(bytes, 126).unwrap().read_value::<u8>() },
            126
        );
        assert_eq!(
            unsafe { extra.get(bytes, 127).unwrap().read_value::<u8>() },
            0
        );
        assert_eq!(
            unsafe { extra.get(bytes, 128).unwrap().read_value::<u8>() },
            128
        );
        assert!(extra.get(bytes, bytes.len()).is_none());
    }

    #[test]
    fn node_columns_stay_aligned_and_validate_kinds() {
        let allocator = Allocator::new();
        let mut nodes = NodeData::new_in(&allocator);
        let id = nodes.alloc::<InlineNode>(INLINE_KIND, NodePayload::from_value(7u8), DUMMY_SP);
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            unsafe { nodes.payload(id, INLINE_KIND).read_value::<u8>() },
            7
        );

        let replacement = Span::new(3, 8);
        nodes.set_span(id, INLINE_KIND, replacement);
        assert_eq!(nodes.span(id, INLINE_KIND), replacement);

        let payload = nodes.begin_mutation(id, INLINE_KIND);
        nodes.finish_mutation(id, INLINE_KIND, payload);
        assert_eq!(
            unsafe { nodes.payload(id, INLINE_KIND).read_value::<u8>() },
            7
        );

        nodes.truncate(0);
        assert_eq!(nodes.len(), 0);
    }

    #[test]
    #[should_panic(expected = "wrong node kind")]
    fn node_kind_rejects_a_mismatched_typed_decode() {
        let allocator = Allocator::new();
        let mut nodes = NodeData::new_in(&allocator);
        let id =
            nodes.alloc::<OverflowNode>(OVERFLOW_KIND, NodePayload::from_value(0u32), DUMMY_SP);
        nodes.payload(id, INLINE_KIND);
    }

    #[test]
    fn extra_ranges_directly_address_shared_slots() {
        let allocator = Allocator::new();
        let mut extra = ExtraDataStore::new_in(&allocator);
        let range = extra.alloc::<u16>(
            [
                ExtraData::from_value(0x0201u16),
                ExtraData::from_value(0x0403u16),
            ]
            .into_iter(),
        );
        assert_eq!(range.start_index(), 0);
        assert_eq!(range.end_index(), 2);
        assert_eq!(
            unsafe { extra.get(range, 1).unwrap().read_value::<u16>() },
            0x0403
        );

        extra.set(range, 0, ExtraData::from_value(0x0908u16));
        assert_eq!(unsafe { extra.get_at(0).read_value::<u16>() }, 0x0908);
        extra.set_at(1, ExtraData::from_value(6u16));
        assert_eq!(
            unsafe { extra.get(range, 1).unwrap().read_value::<u16>() },
            6
        );

        extra.truncate(0);
        assert_eq!(extra.len(), 0);
    }

    #[test]
    fn context_stores_compact_values_and_string_ranges_into_shared_extra() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);

        let numbers = context.alloc_encoded_vec([3_u32, 5, 8].into_iter());
        assert_eq!(context.encoded_vec_get(numbers, 1), Some(5));
        context.encoded_vec_set(numbers, 1, 13);
        assert_eq!(context.encoded_vec_get(numbers, 1), Some(13));

        let checkpoint = context.node_checkpoint();
        let alpha = context.add_str("alpha");
        let beta = context.add_str("beta");
        let strings = context.alloc_encoded_vec([alpha, beta].into_iter());
        assert_eq!(context.encoded_vec_get(strings, 0), Some(alpha));
        assert_eq!(context.encoded_vec_get(strings, 1), Some(beta));

        let name = context.add_str("named");
        let optional_strings =
            context.alloc_encoded_vec([Some(name), None, Some(AstStr::EMPTY)].into_iter());
        let cloned_optional_strings = context.clone_encoded_vec(optional_strings);
        assert_eq!(
            context
                .encoded_vec_iter(cloned_optional_strings)
                .collect::<std::vec::Vec<_>>(),
            [Some(name), None, Some(AstStr::EMPTY)]
        );

        context.restore_node_checkpoint(checkpoint);
        assert_eq!(context.encoded_vec_get(strings, 0), None);
    }

    #[test]
    fn optional_range_slots_preserve_empty_and_clone_nested_values() {
        assert_eq!(std::mem::size_of::<super::OptionalRangeSlot<'_, u32>>(), 8);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let numbers = context.alloc_encoded_vec([3_u32, 5, 8].into_iter());
        let empty = crate::AstVec::empty();
        let before = context.encoded_extra_len();
        let ranges = context.alloc_encoded_vec([None, Some(empty), Some(numbers)].into_iter());
        assert_eq!(context.encoded_extra_len(), before + 3);
        assert_eq!(context.encoded_vec_get(ranges, 0), Some(None));
        assert_eq!(context.encoded_vec_get(ranges, 1), Some(Some(empty)));
        assert_eq!(context.encoded_vec_get(ranges, 2), Some(Some(numbers)));

        let cloned = context.clone_encoded_vec(ranges);
        assert_eq!(context.encoded_vec_get(cloned, 0), Some(None));
        assert!(
            context
                .encoded_vec_get(cloned, 1)
                .unwrap()
                .unwrap()
                .is_empty()
        );
        let cloned_numbers = context.encoded_vec_get(cloned, 2).unwrap().unwrap();
        assert_ne!(cloned_numbers, numbers);
        context.encoded_vec_set(numbers, 1, 13);
        assert_eq!(context.encoded_vec_get(cloned_numbers, 1), Some(5));
        let checkpoint = context.node_checkpoint();
        for value in [Some(empty), None, Some(numbers)] {
            context.encoded_vec_set(ranges, 0, value);
            assert_eq!(context.encoded_vec_get(ranges, 0), Some(value));
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
    }

    #[test]
    fn direct_scalar_slots_preserve_float_bits_and_signed_values() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        for bits in [0, 0x8000_0000, 1, 0x7f80_0000, 0xff80_0000, 0x7fc0_0123] {
            let value = f32::from_bits(bits);
            let values = context.alloc_encoded_vec([value].into_iter());
            assert_eq!(context.encoded_vec_get(values, 0).unwrap().to_bits(), bits);
        }
        let values = context.alloc_encoded_vec([i32::MIN, -1, 0, i32::MAX].into_iter());
        assert_eq!(
            context
                .encoded_vec_iter(values)
                .collect::<std::vec::Vec<_>>(),
            [i32::MIN, -1, 0, i32::MAX]
        );
    }

    #[test]
    fn string_slots_do_not_append_or_intern_text() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let atom = context.intern("interned");
        let text = context.add_str("ordinary");
        let bytes = context.string_pool().extra_len();
        let atoms = context.alloc_encoded_vec([atom].into_iter());
        let strings = context.alloc_encoded_vec([text].into_iter());
        for _ in 0..100 {
            assert_eq!(context.encoded_vec_get(atoms, 0), Some(atom));
            assert_eq!(context.encoded_vec_get(strings, 0), Some(text));
            context.mutate_encoded_vec(atoms, |_, _| {});
            context.mutate_encoded_vec(strings, |_, _| {});
        }
        assert_eq!(context.string_pool().extra_len(), bytes);
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
        let text = context.add_str("source");
        let source = context.alloc_encoded_node(Token::Ident(text), Span::new(4, 10));
        let values = context.alloc_encoded_vec([source].into_iter());
        let cloned_values = context.clone_encoded_vec(values);
        let cloned = context.encoded_vec_get(cloned_values, 0).unwrap();

        assert_ne!(source, cloned);
        assert_eq!(context.encoded_node(cloned), Token::Ident(text));
        assert_eq!(context.encoded_node_span(cloned), Span::new(4, 10));
    }
}

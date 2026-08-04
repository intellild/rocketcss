use std::hash::{Hash, Hasher};

use rocketcss_ast::{
    Declaration, DeclarationBlock, DeclarationBlockId, DeclarationBlockStore, PropertyId,
};
use rocketcss_common::{DenseId, DenseStore, define_dense_id};
use rustc_hash::{FxHashMap, FxHasher};
use smallvec::SmallVec;

define_dense_id!(pub(super) struct DeclarationOccurrenceId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CompactPropertyKey {
    Known(u32),
    Custom(u32),
}

impl CompactPropertyKey {
    const IMPORTANT_MASK: u32 = 1;
    const VENDOR_PREFIX_SHIFT: u32 = 1;
    const PROPERTY_ID_SHIFT: u32 = 6;

    fn known(declaration: &Declaration<'_>, important: bool) -> Option<Self> {
        if let Some((property_id, vendor_prefix)) = declaration.known_id_and_prefix() {
            let vendor_prefix = u32::from(vendor_prefix.bits());
            debug_assert!(property_id <= u32::MAX >> Self::PROPERTY_ID_SHIFT);
            debug_assert_eq!(vendor_prefix & !0b1_1111, 0);
            return Some(Self::Known(
                (property_id << Self::PROPERTY_ID_SHIFT)
                    | (vendor_prefix << Self::VENDOR_PREFIX_SHIFT)
                    | u32::from(important),
            ));
        }
        None
    }

    fn is_important(self) -> bool {
        match self {
            Self::Known(key) => key & Self::IMPORTANT_MASK != 0,
            Self::Custom(key) => key & Self::IMPORTANT_MASK != 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct DeclarationSlot {
    pub(super) block: DeclarationBlockId,
    pub(super) index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct OccurrenceOrder {
    range: u32,
    source: u32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DeclarationOccurrenceIr {
    pub(super) slot: DeclarationSlot,
    property_key: Option<CompactPropertyKey>,
    pub(super) movement_domain: Option<MovementDomain>,
    source_order: u32,
    live: bool,
}

#[derive(Clone, Copy, Debug)]
struct OccurrenceRange {
    start: u32,
    len: u32,
}

impl OccurrenceRange {
    fn ids(self) -> impl Iterator<Item = DeclarationOccurrenceId> {
        let end = self
            .start
            .checked_add(self.len)
            .expect("declaration occurrence tape exceeds u32::MAX");
        (self.start..end).map(|index| {
            DeclarationOccurrenceId::from_index(index as usize)
                .expect("an occurrence tape offset is a dense occurrence ID")
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct PropertyIndexRange {
    start: u32,
    len: u32,
}

#[derive(Clone, Copy, Debug)]
struct FrozenDeclarationRange {
    occurrences: OccurrenceRange,
    property_index: PropertyIndexRange,
}

#[derive(Clone, Debug, Default)]
struct FrozenDeclarationBlockIr {
    ranges: SmallVec<[FrozenDeclarationRange; 2]>,
    live_count: u32,
    property_bloom: PropertyBloom,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct PropertyBloom {
    normal: u64,
    important: u64,
}

impl PropertyBloom {
    fn insert(&mut self, key: CompactPropertyKey) {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let bits = (1 << (hash & 63)) | (1 << ((hash >> 6) & 63));
        if key.is_important() {
            self.important |= bits;
        } else {
            self.normal |= bits;
        }
    }

    pub(super) fn may_share_declaration(self, other: Self) -> bool {
        self.normal & other.normal != 0 || self.important & other.important != 0
    }

    fn union_with(&mut self, other: Self) {
        self.normal |= other.normal;
        self.important |= other.important;
    }
}

/// Persistent, source-ordered declaration metadata shared by S1, S2, and S3.
///
/// Physical declaration slots and property indexes live in flat tapes. A live
/// block summary is only a short sequence of ranges into those tapes, so S1 can
/// compose two blocks without copying or reclassifying their declarations.
#[derive(Debug, Default)]
pub(crate) struct FrozenDeclarationIrStore<'ast> {
    occurrences: DenseStore<DeclarationOccurrenceId, DeclarationOccurrenceIr>,
    property_index: std::vec::Vec<DeclarationOccurrenceId>,
    physical_ranges: std::vec::Vec<Option<OccurrenceRange>>,
    summaries: std::vec::Vec<Option<FrozenDeclarationBlockIr>>,
    custom_property_ids: FxHashMap<&'ast str, u32>,
    match_generations: std::vec::Vec<u32>,
    current_match_generation: u32,
}

impl<'ast> FrozenDeclarationIrStore<'ast> {
    pub(crate) fn with_block_capacity(block_capacity: usize) -> Self {
        Self {
            occurrences: DenseStore::with_capacity(block_capacity),
            property_index: std::vec::Vec::with_capacity(block_capacity),
            physical_ranges: std::vec::Vec::with_capacity(block_capacity),
            summaries: std::vec::Vec::with_capacity(block_capacity),
            custom_property_ids: FxHashMap::default(),
            match_generations: std::vec::Vec::with_capacity(block_capacity),
            current_match_generation: 0,
        }
    }

    #[cfg(test)]
    pub(super) fn occurrence_count(&self) -> usize {
        self.occurrences.len()
    }

    #[cfg(test)]
    pub(super) fn property_index_count(&self) -> usize {
        self.property_index.len()
    }

    pub(crate) fn freeze_physical_block(
        &mut self,
        id: DeclarationBlockId,
        block: &DeclarationBlock<'ast>,
    ) {
        self.ensure_block_capacity(id);
        if self.physical_ranges[id.index()].is_some() {
            return;
        }

        let start = u32::try_from(self.occurrences.len())
            .expect("declaration occurrence count exceeds u32::MAX");
        let declaration_count =
            u32::try_from(block.len()).expect("declaration count exceeds u32::MAX");
        start
            .checked_add(declaration_count)
            .expect("declaration occurrence tape exceeds u32::MAX");
        let mut property_occurrences = SmallVec::<[DeclarationOccurrenceId; 8]>::new();
        let mut bloom = PropertyBloom::default();
        let mut live_count = 0u32;
        for (index, (declaration, important)) in block.iter().enumerate() {
            let property_key = self.compact_property_key(declaration, important);
            let live = !declaration.is_tombstone();
            let movement_domain = live
                .then(|| self.compact_movement_domain(declaration))
                .flatten();
            if live {
                live_count = live_count
                    .checked_add(1)
                    .expect("declaration count exceeds u32::MAX");
            }
            if live && let Some(key) = property_key {
                bloom.insert(key);
            }
            let occurrence = self.occurrences.push(DeclarationOccurrenceIr {
                slot: DeclarationSlot { block: id, index },
                property_key,
                movement_domain,
                source_order: u32::try_from(index).expect("declaration index exceeds u32::MAX"),
                live,
            });
            self.match_generations.push(0);
            if live && property_key.is_some() {
                property_occurrences.push(occurrence);
            }
        }
        property_occurrences.sort_unstable_by_key(|&occurrence| {
            let occurrence = &self.occurrences[occurrence];
            (occurrence.property_key, occurrence.source_order)
        });
        let property_range = self.append_property_index(&property_occurrences);
        let range = OccurrenceRange {
            start,
            len: declaration_count,
        };
        self.physical_ranges[id.index()] = Some(range);
        self.summaries[id.index()] = Some(FrozenDeclarationBlockIr {
            ranges: SmallVec::from_slice(&[FrozenDeclarationRange {
                occurrences: range,
                property_index: property_range,
            }]),
            live_count,
            property_bloom: bloom,
        });
    }

    /// Reconstructs only an already-authored `previous_merged` chain. Normal
    /// first-pass blocks have no chain and retain the summary frozen during the
    /// declaration-block visit.
    pub(super) fn initialize_owner_chain(
        &mut self,
        active: DeclarationBlockId,
        store: &DeclarationBlockStore<'ast>,
    ) {
        self.freeze_physical_block(active, store.get(active));
        let Some(previous) = store.get(active).previous_merged() else {
            return;
        };

        let mut chain = SmallVec::<[DeclarationBlockId; 4]>::from_slice(&[active]);
        let mut current = Some(previous);
        while let Some(block) = current {
            if chain.contains(&block) {
                break;
            }
            self.freeze_physical_block(block, store.get(block));
            chain.push(block);
            current = store.get(block).previous_merged();
        }
        chain.reverse();
        let mut summary = FrozenDeclarationBlockIr::default();
        for block in chain {
            let base_range = self.physical_ranges[block.index()]
                .expect("a frozen physical block has an occurrence range");
            let base = self.summaries[block.index()]
                .as_ref()
                .expect("a frozen physical block has a summary");
            summary.ranges.push(FrozenDeclarationRange {
                occurrences: base_range,
                property_index: base.ranges[0].property_index,
            });
            summary.live_count += base
                .ranges
                .iter()
                .flat_map(|range| range.occurrences.ids())
                .filter(|&occurrence| self.occurrences[occurrence].live)
                .count() as u32;
            summary.property_bloom.union_with(base.property_bloom);
        }
        self.summaries[active.index()] = Some(summary);
    }

    pub(super) fn compose(&mut self, left: DeclarationBlockId, right: DeclarationBlockId) {
        let left = self.summary(left).clone();
        let right_summary = self.summary_mut(right);
        let mut ranges = left.ranges;
        ranges.extend_from_slice(&right_summary.ranges);
        right_summary.ranges = ranges;
        right_summary.live_count += left.live_count;
        right_summary.property_bloom.union_with(left.property_bloom);
    }

    pub(super) fn property_bloom(&self, block: DeclarationBlockId) -> PropertyBloom {
        self.summary(block).property_bloom
    }

    pub(super) fn live_count(&self, block: DeclarationBlockId) -> u32 {
        self.summary(block).live_count
    }

    pub(super) fn live_occurrences(
        &self,
        block: DeclarationBlockId,
    ) -> impl Iterator<Item = DeclarationOccurrenceId> + '_ {
        self.summary(block)
            .ranges
            .iter()
            .flat_map(|range| range.occurrences.ids())
            .filter(|&occurrence| self.occurrences[occurrence].live)
    }

    pub(super) fn matching_occurrences(
        &self,
        block: DeclarationBlockId,
        left: DeclarationOccurrenceId,
    ) -> impl Iterator<Item = (DeclarationOccurrenceId, OccurrenceOrder)> + '_ {
        let key = self.occurrences[left]
            .property_key
            .expect("only matchable occurrences enter property lookup");
        self.summary(block)
            .ranges
            .iter()
            .copied()
            .enumerate()
            .flat_map(move |(range_index, range)| {
                let range = range.property_index;
                let start = range.start as usize;
                let end = start
                    .checked_add(range.len as usize)
                    .expect("property occurrence index exceeds usize");
                let index = &self.property_index[start..end];
                let first = index.partition_point(|&occurrence| {
                    self.occurrences[occurrence].property_key < Some(key)
                });
                let last = index.partition_point(|&occurrence| {
                    self.occurrences[occurrence].property_key <= Some(key)
                });
                index[first..last]
                    .iter()
                    .copied()
                    .filter_map(move |occurrence| {
                        let occurrence_ir = &self.occurrences[occurrence];
                        occurrence_ir.live.then_some((
                            occurrence,
                            OccurrenceOrder {
                                range: u32::try_from(range_index)
                                    .expect("summary range count exceeds u32::MAX"),
                                source: occurrence_ir.source_order,
                            },
                        ))
                    })
            })
    }

    pub(super) fn begin_matching(&mut self) -> u32 {
        self.current_match_generation = self.current_match_generation.wrapping_add(1);
        if self.current_match_generation == 0 {
            self.match_generations.fill(0);
            self.current_match_generation = 1;
        }
        self.current_match_generation
    }

    pub(super) fn is_matched(&self, occurrence: DeclarationOccurrenceId, generation: u32) -> bool {
        self.match_generations[occurrence.index()] == generation
    }

    pub(super) fn mark_matched(&mut self, occurrence: DeclarationOccurrenceId, generation: u32) {
        debug_assert_eq!(generation, self.current_match_generation);
        self.match_generations[occurrence.index()] = generation;
    }

    pub(super) fn occurrence(
        &self,
        occurrence: DeclarationOccurrenceId,
    ) -> &DeclarationOccurrenceIr {
        &self.occurrences[occurrence]
    }

    pub(super) fn is_matchable(&self, occurrence: DeclarationOccurrenceId) -> bool {
        self.occurrences[occurrence].property_key.is_some()
    }

    pub(super) fn occurrence_for_slot(
        &self,
        slot: DeclarationSlot,
    ) -> Option<DeclarationOccurrenceId> {
        let range = self.physical_ranges.get(slot.block.index())?.as_ref()?;
        if slot.index >= range.len as usize {
            return None;
        }
        DeclarationOccurrenceId::from_index(range.start as usize + slot.index)
    }

    pub(super) fn mark_dead(
        &mut self,
        owner: DeclarationBlockId,
        occurrence: DeclarationOccurrenceId,
    ) {
        if !self.occurrences[occurrence].live {
            return;
        }
        self.occurrences[occurrence].live = false;
        let summary = self.summary_mut(owner);
        summary.live_count -= 1;
        // Bloom bits are deliberately retained. A false positive is safe and
        // avoids per-bit reference counts on deletion.
    }

    pub(super) fn transfer_common(
        &mut self,
        shared: DeclarationBlockId,
        left_owner: DeclarationBlockId,
        right_owner: DeclarationBlockId,
        pairs: &[(DeclarationOccurrenceId, DeclarationOccurrenceId)],
    ) {
        let transferred = pairs
            .iter()
            .map(|&(left, _)| {
                let occurrence = self.occurrences[left];
                (occurrence.property_key, occurrence.movement_domain)
            })
            .collect::<SmallVec<[_; 8]>>();
        for &(left, right) in pairs {
            self.mark_dead(left_owner, left);
            self.mark_dead(right_owner, right);
        }

        self.ensure_block_capacity(shared);
        let start = u32::try_from(self.occurrences.len())
            .expect("declaration occurrence count exceeds u32::MAX");
        let declaration_count =
            u32::try_from(pairs.len()).expect("declaration count exceeds u32::MAX");
        start
            .checked_add(declaration_count)
            .expect("declaration occurrence tape exceeds u32::MAX");
        let mut property_occurrences = SmallVec::<[DeclarationOccurrenceId; 8]>::new();
        let mut bloom = PropertyBloom::default();
        for (index, (property_key, movement_domain)) in transferred.into_iter().enumerate() {
            let occurrence = self.occurrences.push(DeclarationOccurrenceIr {
                slot: DeclarationSlot {
                    block: shared,
                    index,
                },
                property_key,
                movement_domain,
                source_order: u32::try_from(index).expect("declaration index exceeds u32::MAX"),
                live: true,
            });
            self.match_generations.push(0);
            if let Some(key) = property_key {
                bloom.insert(key);
                property_occurrences.push(occurrence);
            }
        }
        property_occurrences.sort_unstable_by_key(|&occurrence| {
            let occurrence = &self.occurrences[occurrence];
            (occurrence.property_key, occurrence.source_order)
        });
        let property_range = self.append_property_index(&property_occurrences);
        let range = OccurrenceRange {
            start,
            len: declaration_count,
        };
        self.physical_ranges[shared.index()] = Some(range);
        self.summaries[shared.index()] = Some(FrozenDeclarationBlockIr {
            ranges: SmallVec::from_slice(&[FrozenDeclarationRange {
                occurrences: range,
                property_index: property_range,
            }]),
            live_count: range.len,
            property_bloom: bloom,
        });
    }

    pub(super) fn reuse_left_as_shared(
        &mut self,
        left_owner: DeclarationBlockId,
        right_owner: DeclarationBlockId,
        pairs: &[(DeclarationOccurrenceId, DeclarationOccurrenceId)],
    ) {
        debug_assert_eq!(
            self.live_count(left_owner),
            u32::try_from(pairs.len()).expect("common declaration count exceeds u32::MAX"),
            "an exhausted left owner must transfer every live occurrence"
        );
        debug_assert!(
            pairs.iter().all(|&(left, right)| {
                self.occurrences[left].live && self.occurrences[right].live
            })
        );
        for &(_, right) in pairs {
            self.mark_dead(right_owner, right);
        }
    }

    fn append_property_index(
        &mut self,
        occurrences: &[DeclarationOccurrenceId],
    ) -> PropertyIndexRange {
        let start = u32::try_from(self.property_index.len())
            .expect("property occurrence index exceeds u32::MAX");
        let len =
            u32::try_from(occurrences.len()).expect("property occurrence count exceeds u32::MAX");
        start
            .checked_add(len)
            .expect("property occurrence index exceeds u32::MAX");
        self.property_index.extend_from_slice(occurrences);
        PropertyIndexRange { start, len }
    }

    fn compact_property_key(
        &mut self,
        declaration: &Declaration<'ast>,
        important: bool,
    ) -> Option<CompactPropertyKey> {
        if matches!(
            declaration,
            Declaration::Unparsed(_) | Declaration::Tombstone
        ) {
            return None;
        }
        if let Some(key) = CompactPropertyKey::known(declaration, important) {
            return Some(key);
        }
        let PropertyId::Custom(name) = declaration.property_id()? else {
            return None;
        };
        let id = self.intern_custom_property(name);
        let key = id
            .checked_mul(2)
            .and_then(|id| id.checked_add(u32::from(important)))
            .expect("custom property count exceeds compact key capacity");
        Some(CompactPropertyKey::Custom(key))
    }

    fn compact_movement_domain(
        &mut self,
        declaration: &Declaration<'ast>,
    ) -> Option<MovementDomain> {
        let property_id = declaration.property_id()?;
        if let PropertyId::Custom(name) = property_id {
            return Some(MovementDomain::Custom(self.intern_custom_property(name)));
        }
        movement_domain(property_id)
    }

    fn intern_custom_property(&mut self, name: &'ast str) -> u32 {
        let next_id = u32::try_from(self.custom_property_ids.len())
            .expect("custom property count exceeds u32::MAX");
        *self.custom_property_ids.entry(name).or_insert(next_id)
    }

    fn ensure_block_capacity(&mut self, block: DeclarationBlockId) {
        let len = block.index() + 1;
        if self.physical_ranges.len() < len {
            self.physical_ranges.resize(len, None);
            self.summaries.resize_with(len, || None);
        }
    }

    fn summary(&self, block: DeclarationBlockId) -> &FrozenDeclarationBlockIr {
        self.summaries[block.index()]
            .as_ref()
            .expect("a live declaration block has frozen declaration IR")
    }

    fn summary_mut(&mut self, block: DeclarationBlockId) -> &mut FrozenDeclarationBlockIr {
        self.summaries[block.index()]
            .as_mut()
            .expect("a live declaration block has frozen declaration IR")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MovementDomain {
    Custom(u32),
    Background,
    Border,
    Color,
    Display,
    Font,
    Height,
    Margin(u8),
    Mask,
    Opacity,
    Padding(u8),
    Position,
    TextAlign,
    TextDecoration,
    TextTransform,
    Visibility,
    Width,
}

impl MovementDomain {
    pub(super) fn overlaps(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Custom(left), Self::Custom(right)) => left == right,
            (Self::Margin(left), Self::Margin(right))
            | (Self::Padding(left), Self::Padding(right)) => left & right != 0,
            _ => self == other,
        }
    }
}

fn movement_domain(property_id: PropertyId<'_>) -> Option<MovementDomain> {
    use MovementDomain as Domain;
    use PropertyId as Id;

    Some(match property_id {
        Id::Custom(_) => unreachable!("custom properties are interned by the IR store"),
        Id::BackgroundColor
        | Id::BackgroundImage
        | Id::BackgroundPositionX
        | Id::BackgroundPositionY
        | Id::BackgroundPosition
        | Id::BackgroundSize
        | Id::BackgroundRepeat
        | Id::BackgroundAttachment
        | Id::BackgroundClip(_)
        | Id::BackgroundOrigin
        | Id::Background => Domain::Background,
        Id::BorderTopColor
        | Id::BorderBottomColor
        | Id::BorderLeftColor
        | Id::BorderRightColor
        | Id::BorderTopStyle
        | Id::BorderBottomStyle
        | Id::BorderLeftStyle
        | Id::BorderRightStyle
        | Id::BorderTopWidth
        | Id::BorderBottomWidth
        | Id::BorderLeftWidth
        | Id::BorderRightWidth
        | Id::BorderColor
        | Id::BorderStyle
        | Id::BorderWidth
        | Id::Border
        | Id::BorderTop
        | Id::BorderBottom
        | Id::BorderLeft
        | Id::BorderRight => Domain::Border,
        Id::Color => Domain::Color,
        Id::Display => Domain::Display,
        Id::FontWeight
        | Id::FontSize
        | Id::FontStretch
        | Id::FontFamily
        | Id::FontStyle
        | Id::FontVariantCaps
        | Id::LineHeight
        | Id::Font => Domain::Font,
        Id::Height | Id::MinHeight | Id::MaxHeight => Domain::Height,
        Id::MarginTop => Domain::Margin(0b0001),
        Id::MarginRight => Domain::Margin(0b0010),
        Id::MarginBottom => Domain::Margin(0b0100),
        Id::MarginLeft => Domain::Margin(0b1000),
        Id::Margin => Domain::Margin(0b1111),
        Id::MaskImage(_)
        | Id::MaskMode
        | Id::MaskRepeat(_)
        | Id::MaskPositionX
        | Id::MaskPositionY
        | Id::MaskPosition(_)
        | Id::MaskClip(_)
        | Id::MaskOrigin(_)
        | Id::MaskSize(_)
        | Id::MaskComposite
        | Id::MaskType
        | Id::Mask(_) => Domain::Mask,
        Id::Opacity => Domain::Opacity,
        Id::PaddingTop => Domain::Padding(0b0001),
        Id::PaddingRight => Domain::Padding(0b0010),
        Id::PaddingBottom => Domain::Padding(0b0100),
        Id::PaddingLeft => Domain::Padding(0b1000),
        Id::Padding => Domain::Padding(0b1111),
        Id::Position | Id::Top | Id::Right | Id::Bottom | Id::Left => Domain::Position,
        Id::TextAlign => Domain::TextAlign,
        Id::TextDecorationLine(_)
        | Id::TextDecorationStyle(_)
        | Id::TextDecorationColor(_)
        | Id::TextDecorationThickness
        | Id::TextDecoration(_) => Domain::TextDecoration,
        Id::TextTransform => Domain::TextTransform,
        Id::Visibility => Domain::Visibility,
        Id::Width | Id::MinWidth | Id::MaxWidth => Domain::Width,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use rocketcss_common::Allocator;
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;
    use crate::{MinifyContext, MinifyOptions, rules::DeclarationBlockMinifier};

    #[test]
    fn occurrence_ids_follow_final_source_order() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{width:1px;color:red!important;height:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let block = DeclarationBlockId::from_index(0).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(block, blocks.get(block));

            let slots = ir
                .live_occurrences(block)
                .map(|occurrence| ir.occurrence(occurrence).slot.index)
                .collect::<std::vec::Vec<_>>();
            assert_eq!(slots, [0, 1, 2]);
        });
    }

    #[test]
    fn empty_single_and_tombstoned_blocks_freeze_without_losing_slot_order() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation = parse(
                "a{}b{width:1px;color:red;height:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts_mut();
            let empty = DeclarationBlockId::from_index(0).unwrap();
            let declarations = DeclarationBlockId::from_index(1).unwrap();
            blocks.get_mut(declarations).declarations[1] = Declaration::Tombstone;
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(empty, blocks.get(empty));
            ir.freeze_physical_block(declarations, blocks.get(declarations));

            assert_eq!(ir.live_count(empty), 0);
            let slots = ir
                .live_occurrences(declarations)
                .map(|occurrence| ir.occurrence(occurrence).slot.index)
                .collect::<std::vec::Vec<_>>();
            assert_eq!(slots, [0, 2]);
            let tombstone = ir
                .occurrence_for_slot(DeclarationSlot {
                    block: declarations,
                    index: 1,
                })
                .unwrap();
            assert!(!ir.occurrence(tombstone).live);
        });
    }

    #[test]
    fn property_lookup_preserves_repeated_order_and_importance() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{display:-webkit-box;display:flex;color:red}b{display:flex;display:-webkit-box;color:red!important}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));

            let left_occurrences = ir.live_occurrences(left).collect::<std::vec::Vec<_>>();
            let display_slots = ir
                .matching_occurrences(right, left_occurrences[0])
                .map(|(occurrence, _)| ir.occurrence(occurrence).slot.index)
                .collect::<std::vec::Vec<_>>();
            assert_eq!(display_slots, [0, 1]);
            assert_eq!(
                ir.matching_occurrences(right, left_occurrences[2]).count(),
                0,
                "normal and important declarations must use different lookup keys"
            );
        });
    }

    #[test]
    fn known_vendor_prefixes_are_part_of_the_compact_property_key() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{-webkit-columns:auto}b{columns:auto}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            let left_occurrence = ir.live_occurrences(left).next().unwrap();

            assert_eq!(ir.matching_occurrences(right, left_occurrence).count(), 0);
        });
    }

    #[test]
    fn freeze_observes_block_local_shorthand_rewrites() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let mut compilation = parse(
                "a{margin-top:1px;margin-right:1px;margin-bottom:1px;margin-left:1px}b{margin-top:2px;margin-right:2px;margin-bottom:2px;margin-left:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let scratch = Allocator::new();
            let mut cx = MinifyContext::new(MinifyOptions::default(), &scratch);
            let mut minifier = DeclarationBlockMinifier::new(&scratch);
            let (_, blocks) = compilation.parts_mut();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            minifier.minify(blocks.get_mut(left), &mut cx);
            minifier.minify(blocks.get_mut(right), &mut cx);
            assert_eq!(blocks.get(left).iter_live().count(), 1);
            assert_eq!(blocks.get(right).iter_live().count(), 1);

            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            let left_occurrence = ir.live_occurrences(left).next().unwrap();
            assert_eq!(ir.live_count(left), 1);
            assert_eq!(ir.live_count(right), 1);
            assert_eq!(ir.occurrence(left_occurrence).slot.index, 3);
            assert!(matches!(
                blocks.get(left).declarations[3],
                Declaration::Unparsed(_)
            ));
            assert!(!ir.is_matchable(left_occurrence));
        });
    }

    #[test]
    fn s1_composes_ranges_and_s2_tombstones_occurrences() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{color:red;width:1px}b{height:2px;opacity:.5}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            ir.compose(left, right);

            let slots = ir
                .live_occurrences(right)
                .map(|occurrence| ir.occurrence(occurrence).slot)
                .collect::<std::vec::Vec<_>>();
            assert_eq!(
                slots,
                [
                    DeclarationSlot {
                        block: left,
                        index: 0
                    },
                    DeclarationSlot {
                        block: left,
                        index: 1
                    },
                    DeclarationSlot {
                        block: right,
                        index: 0
                    },
                    DeclarationSlot {
                        block: right,
                        index: 1
                    },
                ]
            );

            let removed = ir
                .occurrence_for_slot(DeclarationSlot {
                    block: left,
                    index: 1,
                })
                .unwrap();
            ir.mark_dead(right, removed);
            assert_eq!(ir.live_count(right), 3);
            let slots = ir
                .live_occurrences(right)
                .map(|occurrence| ir.occurrence(occurrence).slot)
                .collect::<std::vec::Vec<_>>();
            assert_eq!(slots[1].block, right);
            assert_eq!(slots[1].index, 0);
        });
    }

    #[test]
    fn s3_transfers_metadata_without_reclassifying_values() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{color:red;width:1px}b{color:red;height:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let shared = DeclarationBlockId::from_index(2).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            let left_color = ir.live_occurrences(left).next().unwrap();
            let right_color = ir.live_occurrences(right).next().unwrap();

            ir.transfer_common(shared, left, right, &[(left_color, right_color)]);

            assert_eq!(ir.live_count(left), 1);
            assert_eq!(ir.live_count(right), 1);
            assert_eq!(ir.live_count(shared), 1);
            let shared_occurrence = ir.live_occurrences(shared).next().unwrap();
            assert_eq!(
                ir.occurrence(shared_occurrence).slot,
                DeclarationSlot {
                    block: shared,
                    index: 0
                }
            );
            assert_eq!(
                ir.occurrence(shared_occurrence).movement_domain,
                Some(MovementDomain::Color)
            );
        });
    }

    #[test]
    fn s3_reuses_an_exhausted_left_summary_without_appending_occurrences() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{color:red;width:1px}b{color:red;width:1px;height:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            let occurrence_count = ir.occurrences.len();
            let property_index_count = ir.property_index.len();
            let left_occurrences = ir.live_occurrences(left).collect::<std::vec::Vec<_>>();
            let right_occurrences = ir.live_occurrences(right).collect::<std::vec::Vec<_>>();

            ir.reuse_left_as_shared(
                left,
                right,
                &[
                    (left_occurrences[0], right_occurrences[0]),
                    (left_occurrences[1], right_occurrences[1]),
                ],
            );

            assert_eq!(ir.occurrences.len(), occurrence_count);
            assert_eq!(ir.property_index.len(), property_index_count);
            assert_eq!(ir.live_count(left), 2);
            assert_eq!(ir.live_count(right), 1);
            assert_eq!(
                ir.live_occurrences(left).collect::<std::vec::Vec<_>>(),
                left_occurrences
            );
            assert_eq!(
                ir.live_occurrences(right)
                    .map(|occurrence| ir.occurrence(occurrence).slot.index)
                    .collect::<std::vec::Vec<_>>(),
                [2]
            );
        });
    }

    #[test]
    fn s3_reuses_an_exhausted_multi_range_s1_summary() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{color:red}a{width:1px}b{color:red;width:1px;height:2px}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let first = DeclarationBlockId::from_index(0).unwrap();
            let left = DeclarationBlockId::from_index(1).unwrap();
            let right = DeclarationBlockId::from_index(2).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            for block in [first, left, right] {
                ir.freeze_physical_block(block, blocks.get(block));
            }
            ir.compose(first, left);
            let occurrence_count = ir.occurrences.len();
            let property_index_count = ir.property_index.len();
            let left_occurrences = ir.live_occurrences(left).collect::<std::vec::Vec<_>>();
            let right_occurrences = ir.live_occurrences(right).collect::<std::vec::Vec<_>>();

            ir.reuse_left_as_shared(
                left,
                right,
                &[
                    (left_occurrences[0], right_occurrences[0]),
                    (left_occurrences[1], right_occurrences[1]),
                ],
            );

            assert_eq!(ir.occurrences.len(), occurrence_count);
            assert_eq!(ir.property_index.len(), property_index_count);
            assert_eq!(ir.live_count(left), 2);
            assert_eq!(ir.live_count(right), 1);
            assert_eq!(
                ir.live_occurrences(left)
                    .map(|occurrence| ir.occurrence(occurrence).slot)
                    .collect::<std::vec::Vec<_>>(),
                [
                    DeclarationSlot {
                        block: first,
                        index: 0,
                    },
                    DeclarationSlot {
                        block: left,
                        index: 0,
                    },
                ]
            );
        });
    }

    #[test]
    fn unparsed_and_case_distinct_custom_properties_do_not_match() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let compilation = parse(
                "a{display:table-cell flow;--x:1}b{display:table-cell flow;--X:1}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let (_, blocks) = compilation.parts();
            let left = DeclarationBlockId::from_index(0).unwrap();
            let right = DeclarationBlockId::from_index(1).unwrap();
            let mut ir = FrozenDeclarationIrStore::default();
            ir.freeze_physical_block(left, blocks.get(left));
            ir.freeze_physical_block(right, blocks.get(right));
            let left_occurrences = ir.live_occurrences(left).collect::<std::vec::Vec<_>>();

            assert!(!ir.is_matchable(left_occurrences[0]));
            assert_eq!(
                ir.matching_occurrences(right, left_occurrences[1]).count(),
                0
            );
        });
    }
}

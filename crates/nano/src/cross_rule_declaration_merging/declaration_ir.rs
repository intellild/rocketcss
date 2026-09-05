use std::hash::{Hash, Hasher};

use rocketcss_ast::DeclarationRecord;
use rocketcss_ast::{Compilation, Declaration, PropertyId};
use rocketcss_common::{
    Allocator,
    prelude::{HashMap, Vec},
};
use rustc_hash::FxHasher;

use crate::rules::layout::{
    ALL_BOX_SIDES, BoxFamily, BoxProperty, box_property, typed_box_property,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum CompactPropertyKey {
    Known(u32),
    Custom(u32),
}

#[derive(Debug)]
pub(super) struct DeclarationIrClassifier<'scratch, 'ast> {
    custom_property_ids: HashMap<'scratch, &'ast str, u32>,
}

impl<'scratch, 'ast> DeclarationIrClassifier<'scratch, 'ast> {
    pub(super) fn new_in(allocator: &'scratch Allocator) -> Self {
        Self {
            custom_property_ids: HashMap::new_in(allocator),
        }
    }

    pub(super) fn property_key(
        &mut self,
        ast: &Compilation<'ast>,
        declaration: &Declaration<'ast>,
        important: bool,
    ) -> Option<CompactPropertyKey> {
        if matches!(
            declaration,
            Declaration::Unparsed(_) | Declaration::Tombstone
        ) {
            return None;
        }
        if let Some(key) = CompactPropertyKey::known(ast, declaration, important) {
            return Some(key);
        }
        let PropertyId::Custom(name) = declaration.property_id(ast)? else {
            return None;
        };
        let id = self.intern_custom_property(name);
        let key = id
            .checked_mul(2)
            .and_then(|id| id.checked_add(u32::from(important)))
            .expect("custom property count exceeds compact key capacity");
        Some(CompactPropertyKey::Custom(key))
    }

    pub(super) fn movement_domain(
        &mut self,
        ast: &Compilation<'ast>,
        declaration: &Declaration<'ast>,
    ) -> Option<MovementDomain> {
        let property_id = declaration.property_id(ast)?;
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
}

impl CompactPropertyKey {
    const IMPORTANT_MASK: u32 = 1;
    const VENDOR_PREFIX_SHIFT: u32 = 1;
    const PROPERTY_ID_SHIFT: u32 = 6;

    fn known(
        ast: &Compilation<'_>,
        declaration: &Declaration<'_>,
        important: bool,
    ) -> Option<Self> {
        if let Some((property_id, vendor_prefix)) = declaration.known_id_and_prefix(ast) {
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
            Self::Known(key) | Self::Custom(key) => key & Self::IMPORTANT_MASK != 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct PropertyBloom {
    normal: u64,
    important: u64,
}

impl PropertyBloom {
    pub(super) fn insert(&mut self, key: CompactPropertyKey) {
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

    pub(super) fn union_with(&mut self, other: Self) {
        self.normal |= other.normal;
        self.important |= other.important;
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

#[derive(Debug)]
struct PropertyIndex<'scratch, 'ast> {
    by_property: HashMap<'scratch, CompactPropertyKey, Vec<'scratch, IndexedDeclaration<'ast>>>,
}

impl<'scratch> PropertyIndex<'scratch, '_> {
    fn new_in(allocator: &'scratch Allocator) -> Self {
        Self {
            by_property: HashMap::new_in(allocator),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct IndexedDeclaration<'ast> {
    pub(super) declaration: rocketcss_ast::DeclarationId<'ast>,
    pub(super) order: usize,
}

pub(super) struct DeclarationIrStore<'scratch, 'ast> {
    allocator: &'scratch Allocator,
    classifier: DeclarationIrClassifier<'scratch, 'ast>,
    occurrences: Vec<'scratch, Option<DeclarationOccurrenceIr<'ast>>>,
    serialized_values: Vec<'scratch, Option<&'scratch str>>,
    blocks: HashMap<'scratch, rocketcss_ast::ConcreteDeclarationBlockId<'ast>, DeclarationBlockIr>,
    property_index: HashMap<
        'scratch,
        rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        PropertyIndex<'scratch, 'ast>,
    >,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DeclarationOccurrenceIr<'ast> {
    pub(super) property_key: Option<CompactPropertyKey>,
    pub(super) movement_domain: Option<MovementDomain>,
    pub(super) owner: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    pub(super) expansion: EffectExpansion,
    live_effects: u8,
    effect_revision: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EffectExpansion {
    Exact,
    BoxShorthand(BoxFamily),
    BoxLonghand(BoxFamily, u8),
    Barrier(Option<BoxFamily>),
    Opaque,
}

impl DeclarationOccurrenceIr<'_> {
    #[inline]
    pub(super) const fn is_live(self) -> bool {
        self.live_effects != 0
    }

    #[inline]
    pub(super) const fn is_fully_live(self) -> bool {
        match self.expansion {
            EffectExpansion::BoxShorthand(_) => self.live_effects == ALL_BOX_SIDES,
            EffectExpansion::BoxLonghand(_, mask) => self.live_effects == mask,
            EffectExpansion::Exact | EffectExpansion::Opaque => self.live_effects == 1,
            EffectExpansion::Barrier(_) => true,
        }
    }

    #[inline]
    pub(super) const fn is_exact_match_candidate(self) -> bool {
        self.is_live() && self.is_fully_live() && self.property_key.is_some()
    }

    #[inline]
    pub(super) const fn live_effects(self) -> u8 {
        self.live_effects
    }

    #[inline]
    pub(super) const fn effect_revision(self) -> u32 {
        self.effect_revision
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DeclarationBlockIr {
    live_count: u32,
    property_bloom: PropertyBloom,
    has_box_effects: bool,
}

#[derive(Clone, Copy)]
struct PublishedOccurrence<'ast> {
    owner: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    declaration: rocketcss_ast::DeclarationId<'ast>,
    order: usize,
}

impl<'scratch, 'ast> DeclarationIrStore<'scratch, 'ast> {
    pub(super) fn new_in(
        allocator: &'scratch Allocator,
        declaration_capacity: usize,
        block_capacity: usize,
    ) -> Self {
        Self {
            allocator,
            classifier: DeclarationIrClassifier::new_in(allocator),
            occurrences: Vec::with_capacity_in(declaration_capacity, allocator),
            serialized_values: Vec::with_capacity_in(declaration_capacity, allocator),
            blocks: HashMap::with_capacity_in(block_capacity, allocator),
            property_index: HashMap::with_capacity_in(block_capacity, allocator),
        }
    }

    pub(super) fn freeze_block(
        &mut self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    ) -> Result<(), rocketcss_ast::ConcreteMutationError<'ast>> {
        let mut summary = DeclarationBlockIr::default();
        let mut property_index = PropertyIndex::new_in(self.allocator);
        for (order, (occurrence, record)) in compilation
            .declaration_occurrences_in_block(block)?
            .enumerate()
        {
            self.publish_occurrence(
                compilation,
                PublishedOccurrence {
                    owner: block,
                    declaration: occurrence.declaration(),
                    order,
                },
                record,
                &mut summary,
                &mut property_index,
            );
        }
        self.blocks.insert(block, summary);
        self.property_index.insert(block, property_index);
        Ok(())
    }

    fn publish_occurrence(
        &mut self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        occurrence: PublishedOccurrence<'ast>,
        record: &DeclarationRecord<'ast, rocketcss_ast::DeclarationPayload<'ast>>,
        summary: &mut DeclarationBlockIr,
        property_index: &mut PropertyIndex<'scratch, 'ast>,
    ) {
        let PublishedOccurrence {
            owner,
            declaration,
            order,
        } = occurrence;
        let (property_key, movement_domain, expansion, live_effects) = match record.payload() {
            rocketcss_ast::DeclarationPayload::Property(value) => {
                let live = !matches!(value, Declaration::Tombstone);
                let property_key = live
                    .then(|| {
                        self.classifier
                            .property_key(compilation, value, record.is_important())
                    })
                    .flatten();
                let movement_domain = live
                    .then(|| self.classifier.movement_domain(compilation, value))
                    .flatten();
                let expansion = if !live {
                    EffectExpansion::Opaque
                } else {
                    match typed_box_property(value) {
                        Some(BoxProperty::Shorthand(family)) => {
                            EffectExpansion::BoxShorthand(family)
                        }
                        Some(BoxProperty::Longhand(family, side)) => {
                            EffectExpansion::BoxLonghand(family, 1 << side)
                        }
                        Some(BoxProperty::Barrier(_) | BoxProperty::BarrierAll) => {
                            unreachable!("typed box declarations are never barriers")
                        }
                        None => box_property(value, compilation)
                            .map_or(EffectExpansion::Exact, |property| {
                                EffectExpansion::Barrier(property.family())
                            }),
                    }
                };
                let live_effects = if !live {
                    0
                } else {
                    match expansion {
                        EffectExpansion::BoxShorthand(_) => ALL_BOX_SIDES,
                        EffectExpansion::BoxLonghand(_, mask) => mask,
                        EffectExpansion::Exact | EffectExpansion::Opaque => 1,
                        EffectExpansion::Barrier(_) => 1,
                    }
                };
                (property_key, movement_domain, expansion, live_effects)
            }
            rocketcss_ast::DeclarationPayload::FontFace(_)
            | rocketcss_ast::DeclarationPayload::FontPaletteValues(_)
            | rocketcss_ast::DeclarationPayload::ViewTransition(_)
            | rocketcss_ast::DeclarationPayload::FontFeature(_)
            | rocketcss_ast::DeclarationPayload::PropertyRule(_) => {
                (None, None, EffectExpansion::Opaque, 1)
            }
        };
        if live_effects != 0 {
            summary.live_count = summary
                .live_count
                .checked_add(1)
                .expect("declaration count exceeds u32::MAX");
        }
        summary.has_box_effects |= matches!(
            expansion,
            EffectExpansion::BoxShorthand(_) | EffectExpansion::BoxLonghand(_, _)
        );
        if let Some(key) = property_key {
            summary.property_bloom.insert(key);
            property_index
                .by_property
                .entry(key)
                .or_insert_with(|| self.allocator.vec())
                .push(IndexedDeclaration { declaration, order });
        }
        if self.occurrences.len() <= declaration.index() {
            self.occurrences.resize(declaration.index() + 1, None);
            self.serialized_values.resize(declaration.index() + 1, None);
        }
        self.serialized_values[declaration.index()] = None;
        self.occurrences[declaration.index()] = Some(DeclarationOccurrenceIr {
            property_key,
            movement_domain,
            owner,
            expansion,
            live_effects,
            effect_revision: 0,
        });
    }

    pub(super) fn publish_synthesized_declaration(
        &mut self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        declaration: rocketcss_ast::DeclarationId<'ast>,
    ) -> Result<(), rocketcss_ast::ConcreteMutationError<'ast>> {
        let record = compilation.declaration(declaration).ok_or(
            rocketcss_ast::MutationError::UnknownDeclaration(declaration),
        )?;
        let order = compilation.declaration_ids_in_block(block)?.len() - 1;
        let mut summary = self.blocks.get(&block).copied().unwrap_or_default();
        let mut property_index = self
            .property_index
            .remove(&block)
            .unwrap_or_else(|| PropertyIndex::new_in(self.allocator));
        self.publish_occurrence(
            compilation,
            PublishedOccurrence {
                owner: block,
                declaration,
                order,
            },
            record,
            &mut summary,
            &mut property_index,
        );
        self.blocks.insert(block, summary);
        self.property_index.insert(block, property_index);
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn occurrence_count(&self) -> usize {
        self.occurrences.iter().flatten().count()
    }

    #[cfg(test)]
    pub(super) fn matchable_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.property_key.is_some())
            .count()
    }

    #[cfg(test)]
    pub(super) fn live_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.is_live())
            .count()
    }

    #[cfg(test)]
    pub(super) fn movement_domain_count(&self) -> usize {
        self.occurrences
            .iter()
            .flatten()
            .filter(|occurrence| occurrence.movement_domain.is_some())
            .count()
    }

    pub(super) fn compose(
        &mut self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        left: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        right: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    ) -> Result<(), rocketcss_ast::ConcreteMutationError<'ast>> {
        let left_id = left;
        let right_id = right;
        let left = self
            .blocks
            .remove(&left_id)
            .expect("an S1 endpoint has initialized declaration IR");
        let right = self
            .blocks
            .get_mut(&right_id)
            .expect("an S1 endpoint has initialized declaration IR");
        right.live_count = right
            .live_count
            .checked_add(left.live_count)
            .expect("declaration count exceeds u32::MAX");
        right.property_bloom.union_with(left.property_bloom);
        right.has_box_effects |= left.has_box_effects;
        // The merged sequence may contain a non-contiguous bridge. Its index
        // is therefore rebuilt lazily by the linear fallback after S1.
        self.property_index.remove(&left_id);
        self.property_index.remove(&right_id);
        for declaration in compilation.declaration_ids_in_block(right_id)? {
            if let Some(occurrence) = self
                .occurrences
                .get_mut(declaration.index())
                .and_then(Option::as_mut)
            {
                occurrence.owner = right_id;
            }
        }
        Ok(())
    }

    pub(super) fn occurrence(
        &self,
        declaration: rocketcss_ast::DeclarationId<'ast>,
    ) -> Option<&DeclarationOccurrenceIr<'ast>> {
        self.occurrences.get(declaration.index())?.as_ref()
    }

    pub(super) fn declarations_have_equal_css(
        &mut self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        left: rocketcss_ast::DeclarationId<'ast>,
        right: rocketcss_ast::DeclarationId<'ast>,
    ) -> bool {
        if left == right {
            return true;
        }
        let Some(left_cached) = self.serialized_values.get(left.index()).copied() else {
            return false;
        };
        let Some(right_cached) = self.serialized_values.get(right.index()).copied() else {
            return false;
        };
        if let (Some(left), Some(right)) = (left_cached, right_cached) {
            return left == right;
        }

        let value = |declaration| {
            let record = compilation.declaration(declaration)?;
            let rocketcss_ast::DeclarationPayload::Property(value) = record.payload() else {
                return None;
            };
            Some(value)
        };
        let (expected, uncached, expected_owner) = match (left_cached, right_cached) {
            (Some(expected), None) => (expected, right, None),
            (None, Some(expected)) => (expected, left, None),
            (None, None) => {
                let Some(serialized) = value(left)
                    .and_then(|value| crate::equality::css_value_serialization(compilation, value))
                else {
                    return false;
                };
                let expected = self.allocator.alloc_str(&serialized);
                (expected, right, Some(left))
            }
            (Some(_), Some(_)) => unreachable!(),
        };
        if let Some(owner) = expected_owner {
            self.serialized_values[owner.index()] = Some(expected);
        }
        let Some(uncached_value) = value(uncached) else {
            return false;
        };
        if !crate::equality::css_value_matches_serialization(compilation, expected, uncached_value)
        {
            return false;
        }
        self.serialized_values[uncached.index()] = Some(expected);
        true
    }

    pub(super) fn live_declarations(
        &self,
        compilation: &rocketcss_ast::Compilation<'ast>,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        output: &mut Vec<'scratch, rocketcss_ast::DeclarationId<'ast>>,
    ) -> Result<(), rocketcss_ast::ConcreteMutationError<'ast>> {
        output.clear();
        for declaration in compilation.declaration_ids_in_block(block)? {
            if self
                .occurrence(declaration)
                .is_some_and(|occurrence| occurrence.is_live())
            {
                output.push(declaration);
            }
        }
        Ok(())
    }

    pub(super) fn property_candidates(
        &self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        key: CompactPropertyKey,
    ) -> Option<&[IndexedDeclaration<'ast>]> {
        self.property_index
            .get(&block)
            .and_then(|index| index.by_property.get(&key))
            .map(AsRef::as_ref)
    }

    pub(super) fn mark_dead(
        &mut self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        declaration: rocketcss_ast::DeclarationId<'ast>,
    ) {
        self.mark_effects_dead(block, declaration, u8::MAX);
    }

    pub(super) fn mark_effects_dead(
        &mut self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
        declaration: rocketcss_ast::DeclarationId<'ast>,
        effects: u8,
    ) -> bool {
        let occurrence = self.occurrences[declaration.index()]
            .as_mut()
            .expect("an authored declaration has initialized IR");
        debug_assert_eq!(occurrence.owner, block);
        let previous = occurrence.live_effects;
        occurrence.live_effects &= !effects;
        if occurrence.live_effects == previous {
            return false;
        }
        occurrence.effect_revision = occurrence.effect_revision.wrapping_add(1);
        if occurrence.live_effects == 0 {
            self.blocks
                .get_mut(&block)
                .expect("a live block has initialized IR")
                .live_count -= 1;
        }
        true
    }

    pub(super) fn block_live_count(
        &self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    ) -> u32 {
        self.blocks
            .get(&block)
            .map_or(0, |summary| summary.live_count)
    }

    pub(super) fn property_bloom(
        &self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    ) -> PropertyBloom {
        self.blocks
            .get(&block)
            .map_or(PropertyBloom::default(), |summary| summary.property_bloom)
    }

    pub(super) fn block_has_box_effects(
        &self,
        block: rocketcss_ast::ConcreteDeclarationBlockId<'ast>,
    ) -> bool {
        self.blocks
            .get(&block)
            .is_some_and(|summary| summary.has_box_effects)
    }
}

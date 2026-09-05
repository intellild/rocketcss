use crate::MinifyContext;
use rocketcss_ast::{
    AstContext, CSSWideOr, Columns, ConcreteDeclarationBlockId as ContextDeclarationBlockId,
    CssColor, Declaration, DeclarationBlockMutationScope, DeclarationPayload, Margin, Padding,
    PropertyId, ScopedDeclarationHandle, VendorPrefix, Visit, VisitContext, Visitor,
};
use rocketcss_common::{
    GhostToken,
    prelude::{AdaptiveHashMap, Allocator, Vec},
};

use crate::rules::layout::{BoxFamily, BoxProperty, box_property};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UnknownDeclarationKey<'a> {
    property_id: PropertyId<'a>,
    important: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct KnownDeclarationKey(u32);

impl KnownDeclarationKey {
    const IMPORTANT_MASK: u32 = 1;
    const VENDOR_PREFIX_SHIFT: u32 = 1;
    const VENDOR_PREFIX_MASK: u32 = 0b1_1111 << Self::VENDOR_PREFIX_SHIFT;
    const PROPERTY_ID_SHIFT: u32 = 6;

    #[inline]
    fn new(property_id: u32, vendor_prefix: VendorPrefix, important: bool) -> Self {
        let vendor_prefix = u32::from(vendor_prefix.bits());
        debug_assert!(property_id <= u32::MAX >> Self::PROPERTY_ID_SHIFT);
        debug_assert_eq!(vendor_prefix & !0b1_1111, 0);
        Self(
            (property_id << Self::PROPERTY_ID_SHIFT)
                | (vendor_prefix << Self::VENDOR_PREFIX_SHIFT)
                | u32::from(important),
        )
    }

    #[inline]
    fn property_id(self) -> u32 {
        self.0 >> Self::PROPERTY_ID_SHIFT
    }

    #[inline]
    fn vendor_prefix(self) -> VendorPrefix {
        let bits = ((self.0 & Self::VENDOR_PREFIX_MASK) >> Self::VENDOR_PREFIX_SHIFT) as u8;
        VendorPrefix::from_bits_retain(bits)
    }

    #[inline]
    fn is_important(self) -> bool {
        self.0 & Self::IMPORTANT_MASK != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct DeclarationLocation(u32);

impl DeclarationLocation {
    const INDEX_BITS: u32 = 16;
    const INDEX_MASK: u32 = u16::MAX as u32;
    const MAX_COUNT: usize = u16::MAX as usize;
    const EMPTY: Self = Self(u32::MAX);

    #[inline]
    fn new(block: usize, declaration: usize) -> Self {
        debug_assert!(block < Self::MAX_COUNT);
        debug_assert!(declaration < Self::MAX_COUNT);
        Self(((block as u32) << Self::INDEX_BITS) | declaration as u32)
    }

    #[inline]
    fn block(self) -> usize {
        (self.0 >> Self::INDEX_BITS) as usize
    }

    #[inline]
    fn declaration(self) -> usize {
        (self.0 & Self::INDEX_MASK) as usize
    }
}

#[derive(Debug)]
struct DeclarationMap<'scratch, 'ast> {
    known: AdaptiveHashMap<'scratch, KnownDeclarationKey, DeclarationLocation>,
    unknown: AdaptiveHashMap<'scratch, UnknownDeclarationKey<'ast>, DeclarationLocation>,
}

impl<'scratch, 'ast> DeclarationMap<'scratch, 'ast> {
    fn new(allocator: &'scratch Allocator) -> Self {
        Self {
            known: AdaptiveHashMap::new_in(allocator),
            unknown: AdaptiveHashMap::new_in(allocator),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.known.clear();
        self.unknown.clear();
    }

    #[inline]
    fn insert_known(
        &mut self,
        property_id: u32,
        vendor_prefix: VendorPrefix,
        important: bool,
        location: DeclarationLocation,
    ) -> Option<DeclarationLocation> {
        let key = KnownDeclarationKey::new(property_id, vendor_prefix, important);
        debug_assert_eq!(key.property_id(), property_id);
        debug_assert_eq!(key.vendor_prefix(), vendor_prefix);
        debug_assert_eq!(key.is_important(), important);
        self.known.insert(key, location)
    }

    #[inline]
    fn insert_unknown(
        &mut self,
        property_id: PropertyId<'ast>,
        important: bool,
        location: DeclarationLocation,
    ) -> Option<DeclarationLocation> {
        self.unknown.insert(
            UnknownDeclarationKey {
                property_id,
                important,
            },
            location,
        )
    }
}

#[derive(Debug)]
struct BoxFamilyIr<'a> {
    pending_longhands: Vec<'a, DeclarationLocation>,
    sides: [DeclarationLocation; 4],
    shorthand: DeclarationLocation,
}

#[derive(Debug)]
struct ColumnsIr<'a> {
    pending_longhands: Vec<'a, DeclarationLocation>,
    count: DeclarationLocation,
    width: DeclarationLocation,
    shorthand: DeclarationLocation,
}

impl<'a> ColumnsIr<'a> {
    #[inline]
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            pending_longhands: allocator.vec(),
            count: DeclarationLocation::EMPTY,
            width: DeclarationLocation::EMPTY,
            shorthand: DeclarationLocation::EMPTY,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.pending_longhands.clear();
        self.count = DeclarationLocation::EMPTY;
        self.width = DeclarationLocation::EMPTY;
        self.shorthand = DeclarationLocation::EMPTY;
    }
}

impl<'a> BoxFamilyIr<'a> {
    #[inline]
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            pending_longhands: allocator.vec(),
            sides: [DeclarationLocation::EMPTY; 4],
            shorthand: DeclarationLocation::EMPTY,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.pending_longhands.clear();
        self.sides = [DeclarationLocation::EMPTY; 4];
        self.shorthand = DeclarationLocation::EMPTY;
    }
}

struct DeclarationSequence<'scope, 'scratch, 'ast> {
    declarations: Vec<'scratch, ScopedDeclarationHandle<'scope, 'ast>>,
    scope: DeclarationBlockMutationScope<'scope, 'ast>,
}

impl<'scope, 'scratch, 'ast> DeclarationSequence<'scope, 'scratch, 'ast> {
    #[inline]
    fn ast(
        scope: DeclarationBlockMutationScope<'scope, 'ast>,
        allocator: &'scratch Allocator,
    ) -> Self {
        let mut declarations = allocator.vec();
        declarations.extend(
            scope
                .declaration_handles()
                .expect("the declaration block scope remains valid"),
        );
        Self {
            declarations,
            scope,
        }
    }

    #[inline]
    fn block_count(&self) -> usize {
        1
    }

    #[inline]
    fn locations_fit(&self) -> bool {
        let block_count = self.block_count();
        block_count <= DeclarationLocation::MAX_COUNT
            && (0..block_count).all(|block| self.block_len(block) <= DeclarationLocation::MAX_COUNT)
    }

    #[inline]
    fn block_len(&self, index: usize) -> usize {
        debug_assert_eq!(index, 0);
        self.declarations.len()
    }

    #[inline]
    fn declaration_handle(
        &self,
        location: DeclarationLocation,
    ) -> ScopedDeclarationHandle<'scope, 'ast> {
        debug_assert_eq!(location.block(), 0);
        self.declarations[location.declaration()]
    }

    #[inline]
    fn declaration(&self, location: DeclarationLocation) -> &Declaration<'ast> {
        let handle = self.declaration_handle(location);
        let DeclarationPayload::Property(declaration) = self
            .scope
            .declaration(handle)
            .expect("a scoped declaration handle remains resolvable")
            .payload()
        else {
            unreachable!("local declaration minification only receives property blocks")
        };
        declaration
    }

    #[inline]
    fn declaration_mut(&mut self, location: DeclarationLocation) -> &mut Declaration<'ast> {
        let handle = self.declaration_handle(location);
        self.scope
            .property_declaration_mut(handle)
            .expect("a scoped property declaration remains mutable")
            .0
    }

    #[inline]
    fn replace(
        &mut self,
        location: DeclarationLocation,
        declaration: Declaration<'ast>,
    ) -> Declaration<'ast> {
        std::mem::replace(self.declaration_mut(location), declaration)
    }

    #[inline]
    fn is_important(&self, location: DeclarationLocation) -> bool {
        let handle = self.declaration_handle(location);
        self.scope
            .declaration(handle)
            .expect("a scoped declaration handle remains resolvable")
            .is_important()
    }

    #[inline]
    fn ast_context(&self) -> &AstContext<'ast> {
        self.scope.ast_context()
    }

    #[inline]
    fn ast_context_mut(&mut self) -> &mut AstContext<'ast> {
        self.scope.ast_context_mut()
    }
}

pub(crate) struct DeclarationBlockMinifier<'scratch, 'ast> {
    ir: DeclarationIr<'scratch, 'ast>,
}

impl<'scratch, 'ast> DeclarationBlockMinifier<'scratch, 'ast> {
    pub(crate) fn new(allocator: &'scratch Allocator) -> Self {
        Self {
            ir: DeclarationIr::new(allocator),
        }
    }

    pub(crate) fn minify_compilation_block(
        &mut self,
        compilation: &mut AstContext<'ast>,
        block: ContextDeclarationBlockId<'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) {
        compilation
            .with_declaration_block_mutations(block, |scope| {
                let mut sequence = DeclarationSequence::ast(scope, cx.allocator());
                if sequence.block_len(0) >= 2 {
                    self.minify_non_trivial(&mut sequence, cx);
                }
            })
            .expect("the scheduled declaration block remains live");
    }

    fn minify_non_trivial(
        &mut self,
        sequence: &mut DeclarationSequence<'_, '_, 'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) {
        if !sequence.locations_fit() {
            return;
        }
        self.ir.clear();
        deduplicate_declarations(sequence, &mut self.ir, cx);
    }
}

#[derive(Debug)]
struct DeclarationIr<'scratch, 'ast> {
    declarations: DeclarationMap<'scratch, 'ast>,
    boxes: [[BoxFamilyIr<'scratch>; 2]; BoxFamily::COUNT],
    dirty_boxes: u8,
    columns: [[ColumnsIr<'scratch>; 2]; 5],
    dirty_columns: u16,
}

impl<'scratch, 'ast> DeclarationIr<'scratch, 'ast> {
    fn new(allocator: &'scratch Allocator) -> Self {
        Self {
            declarations: DeclarationMap::new(allocator),
            boxes: std::array::from_fn(|_| std::array::from_fn(|_| BoxFamilyIr::new(allocator))),
            dirty_boxes: 0,
            columns: std::array::from_fn(|_| std::array::from_fn(|_| ColumnsIr::new(allocator))),
            dirty_columns: 0,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.declarations.clear();
        self.clear_boxes();
        self.clear_columns();
    }

    #[inline]
    fn box_family(&mut self, family: BoxFamily, important: bool) -> &mut BoxFamilyIr<'scratch> {
        let importance = usize::from(important);
        self.dirty_boxes |= 1 << (family.index() * 2 + importance);
        &mut self.boxes[family.index()][importance]
    }

    #[inline]
    fn clear_box_family(&mut self, family: BoxFamily) {
        for importance in 0..2 {
            let bit = 1 << (family.index() * 2 + importance);
            if self.dirty_boxes & bit != 0 {
                self.boxes[family.index()][importance].clear();
                self.dirty_boxes &= !bit;
            }
        }
    }

    #[inline]
    fn clear_boxes(&mut self) {
        for family in 0..BoxFamily::COUNT {
            for importance in 0..2 {
                let bit = 1 << (family * 2 + importance);
                if self.dirty_boxes & bit != 0 {
                    self.boxes[family][importance].clear();
                }
            }
        }
        self.dirty_boxes = 0;
    }

    #[inline]
    fn columns(
        &mut self,
        prefix: VendorPrefix,
        important: bool,
    ) -> Option<&mut ColumnsIr<'scratch>> {
        let prefix = vendor_prefix_index(prefix)?;
        let importance = usize::from(important);
        self.dirty_columns |= 1 << (prefix * 2 + importance);
        Some(&mut self.columns[prefix][importance])
    }

    #[inline]
    fn clear_columns(&mut self) {
        for prefix in 0..5 {
            for importance in 0..2 {
                let bit = 1 << (prefix * 2 + importance);
                if self.dirty_columns & bit != 0 {
                    self.columns[prefix][importance].clear();
                }
            }
        }
        self.dirty_columns = 0;
    }
}

#[inline]
fn vendor_prefix_index(prefix: VendorPrefix) -> Option<usize> {
    match prefix {
        VendorPrefix::NONE => Some(0),
        VendorPrefix::WEBKIT => Some(1),
        VendorPrefix::MOZ => Some(2),
        VendorPrefix::MS => Some(3),
        VendorPrefix::O => Some(4),
        _ => None,
    }
}

fn deduplicate_declarations<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    for block in 0..sequence.block_count() {
        let declaration_count = sequence.block_len(block);
        for declaration in 0..declaration_count {
            let current = DeclarationLocation::new(block, declaration);
            if sequence.declaration(current).is_tombstone() {
                continue;
            }
            if declaration_skips_minification(sequence.declaration(current), sequence.ast_context())
            {
                ir.clear();
                continue;
            }
            let important = sequence.is_important(current);
            if process_columns_declaration(sequence, current, important, ir, cx) {
                continue;
            }
            if process_box_declaration(sequence, current, important, ir, cx) {
                continue;
            }
            deduplicate_exact_declaration(sequence, current, important, &mut ir.declarations, cx);
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ColumnsProperty {
    Shorthand(VendorPrefix),
    Width(VendorPrefix),
    Count(VendorPrefix),
    BarrierAll,
}

fn process_columns_declaration<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    current: DeclarationLocation,
    important: bool,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    let Some(property) = columns_property(sequence.declaration(current), sequence.ast_context())
    else {
        return false;
    };
    if matches!(property, ColumnsProperty::BarrierAll) {
        ir.clear_columns();
        return false;
    }
    let prefix = match property {
        ColumnsProperty::Shorthand(prefix)
        | ColumnsProperty::Width(prefix)
        | ColumnsProperty::Count(prefix) => prefix,
        ColumnsProperty::BarrierAll => unreachable!(),
    };
    let Some(state) = ir.columns(prefix, important) else {
        return false;
    };

    match property {
        ColumnsProperty::Shorthand(_) => {
            let can_override = can_override_columns_longhands(sequence.declaration(current));
            if can_override {
                for &location in &state.pending_longhands {
                    if !sequence.declaration(location).is_tombstone() {
                        sequence.replace(location, Declaration::Tombstone);
                        cx.record_declaration_removed();
                    }
                }
            }
            state.clear();
            if can_override {
                state.shorthand = current;
            }
            false
        }
        ColumnsProperty::Width(_) | ColumnsProperty::Count(_) => {
            let component = match property {
                ColumnsProperty::Width(_) => &mut state.width,
                ColumnsProperty::Count(_) => &mut state.count,
                _ => unreachable!(),
            };
            let shorthand = state.shorthand;
            if *component == DeclarationLocation::EMPTY
                && shorthand != DeclarationLocation::EMPTY
                && !sequence.declaration(shorthand).is_tombstone()
                && fold_columns_override(sequence, shorthand, current, prefix)
            {
                cx.record_declaration_removed();
                return true;
            }

            state.pending_longhands.push(current);
            *component = current;
            if state.width == DeclarationLocation::EMPTY
                || state.count == DeclarationLocation::EMPTY
            {
                return false;
            }
            if merge_columns_longhands(sequence, state.width, state.count, prefix, cx) {
                let target = std::cmp::max(state.width, state.count);
                state.clear();
                state.shorthand = target;
            }
            false
        }
        ColumnsProperty::BarrierAll => unreachable!(),
    }
}

#[inline]
fn columns_property(
    declaration: &Declaration<'_>,
    ast: &AstContext<'_>,
) -> Option<ColumnsProperty> {
    let property_id = match declaration {
        Declaration::Columns(_, prefix) => return Some(ColumnsProperty::Shorthand(*prefix)),
        Declaration::ColumnWidth(_, prefix) => return Some(ColumnsProperty::Width(*prefix)),
        Declaration::ColumnCount(_, prefix) => return Some(ColumnsProperty::Count(*prefix)),
        Declaration::All(_) => return Some(ColumnsProperty::BarrierAll),
        Declaration::Unparsed(value) => ast.resolve_node(ast.resolve_node(*value).property_id),
        _ => return None,
    };
    match property_id {
        PropertyId::Columns(prefix) => Some(ColumnsProperty::Shorthand(*prefix)),
        PropertyId::ColumnWidth(prefix) => Some(ColumnsProperty::Width(*prefix)),
        PropertyId::ColumnCount(prefix) => Some(ColumnsProperty::Count(*prefix)),
        PropertyId::All => Some(ColumnsProperty::BarrierAll),
        _ => None,
    }
}

fn can_override_columns_longhands(declaration: &Declaration<'_>) -> bool {
    matches!(declaration, Declaration::Columns(..))
}

fn fold_columns_override(
    sequence: &mut DeclarationSequence<'_, '_, '_>,
    shorthand: DeclarationLocation,
    longhand: DeclarationLocation,
    prefix: VendorPrefix,
) -> bool {
    debug_assert!(shorthand < longhand);
    let shorthand_declaration = sequence.replace(shorthand, Declaration::Tombstone);
    let mut longhand_declaration = sequence.replace(longhand, Declaration::Tombstone);
    let folded = match (&shorthand_declaration, &mut longhand_declaration) {
        (
            Declaration::Columns(CSSWideOr::Value(value), shorthand_prefix),
            Declaration::ColumnWidth(CSSWideOr::Value(width), longhand_prefix),
        ) if *shorthand_prefix == prefix && *longhand_prefix == prefix => {
            sequence
                .ast_context_mut()
                .mutate_node(*value, |value, _| std::mem::swap(&mut value.width, width));
            true
        }
        (
            Declaration::Columns(CSSWideOr::Value(value), shorthand_prefix),
            Declaration::ColumnCount(CSSWideOr::Value(count), longhand_prefix),
        ) if *shorthand_prefix == prefix && *longhand_prefix == prefix => {
            sequence
                .ast_context_mut()
                .mutate_node(*value, |value, _| std::mem::swap(&mut value.count, count));
            true
        }
        _ => false,
    };
    sequence.replace(shorthand, shorthand_declaration);
    if !folded {
        sequence.replace(longhand, longhand_declaration);
    }
    folded
}

fn merge_columns_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    width: DeclarationLocation,
    count: DeclarationLocation,
    prefix: VendorPrefix,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    let target = std::cmp::max(width, count);
    if matches!(sequence.declaration(width), Declaration::ColumnWidth(CSSWideOr::Value(_), value_prefix) if *value_prefix == prefix)
        && matches!(sequence.declaration(count), Declaration::ColumnCount(CSSWideOr::Value(_), value_prefix) if *value_prefix == prefix)
    {
        let Declaration::ColumnWidth(CSSWideOr::Value(width_value), _) =
            sequence.replace(width, Declaration::Tombstone)
        else {
            unreachable!("typed columns IR validates column-width")
        };
        let Declaration::ColumnCount(CSSWideOr::Value(count_value), _) =
            sequence.replace(count, Declaration::Tombstone)
        else {
            unreachable!("typed columns IR validates column-count")
        };
        let value = sequence.ast_context_mut().alloc_node_without_span(Columns {
            count: count_value,
            width: width_value,
        });
        sequence.replace(
            target,
            Declaration::Columns(CSSWideOr::Value(value), prefix),
        );
        cx.record_declaration_removed();
        return true;
    }

    let values_are_equal_css_wide = matches!(
        (sequence.declaration(width), sequence.declaration(count)),
        (
            Declaration::ColumnWidth(CSSWideOr::CSSWide(width), width_prefix),
            Declaration::ColumnCount(CSSWideOr::CSSWide(count), count_prefix),
        ) if *width_prefix == prefix && *count_prefix == prefix && width == count
    );
    if !values_are_equal_css_wide {
        return false;
    }

    let width_declaration = sequence.replace(width, Declaration::Tombstone);
    let count_declaration = sequence.replace(count, Declaration::Tombstone);
    let Declaration::ColumnWidth(CSSWideOr::CSSWide(keyword), _) = width_declaration else {
        unreachable!("typed columns IR validates CSS-wide column-width")
    };
    let Declaration::ColumnCount(CSSWideOr::CSSWide(_), _) = count_declaration else {
        unreachable!("typed columns IR validates CSS-wide column-count")
    };
    sequence.replace(
        target,
        Declaration::Columns(CSSWideOr::CSSWide(keyword), prefix),
    );
    cx.record_declaration_removed();
    true
}

fn deduplicate_exact_declaration<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    current: DeclarationLocation,
    important: bool,
    declarations: &mut DeclarationMap<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> Option<DeclarationLocation>
where
    'ast: 'scratch,
{
    let declaration = sequence.declaration(current);
    let previous = if let Some((property_id, vendor_prefix)) =
        declaration.known_id_and_prefix(sequence.ast_context())
    {
        declarations.insert_known(property_id, vendor_prefix, important, current)
    } else {
        declarations.insert_unknown(
            declaration
                .property_id(sequence.ast_context())
                .expect("tombstones are skipped before exact deduplication"),
            important,
            current,
        )
    };
    if let Some(previous) = previous
        && !sequence.declaration(previous).is_tombstone()
        && crate::equality::declarations_are_equal(
            sequence.ast_context(),
            sequence.declaration(previous),
            sequence.declaration(current),
        )
    {
        sequence.replace(previous, Declaration::Tombstone);
        cx.record_declaration_removed();
        return Some(previous);
    }
    None
}

fn process_box_declaration<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    current: DeclarationLocation,
    important: bool,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    let Some(property) = box_property(sequence.declaration(current), sequence.ast_context()) else {
        return false;
    };
    match property {
        BoxProperty::BarrierAll => {
            ir.clear_boxes();
            false
        }
        BoxProperty::Barrier(family) => {
            ir.clear_box_family(family);
            false
        }
        BoxProperty::Shorthand(family) => {
            let can_override = can_override_box_longhands(sequence.declaration(current), family);
            let state = ir.box_family(family, important);
            if can_override {
                for &location in &state.pending_longhands {
                    if !sequence.declaration(location).is_tombstone() {
                        sequence.replace(location, Declaration::Tombstone);
                        cx.record_declaration_removed();
                    }
                }
            }
            state.clear();
            if can_override {
                state.shorthand = current;
            }
            false
        }
        BoxProperty::Longhand(family, side) => {
            let state = ir.box_family(family, important);
            let shorthand = state.shorthand;
            if state.sides[side] == DeclarationLocation::EMPTY
                && shorthand != DeclarationLocation::EMPTY
                && !sequence.declaration(shorthand).is_tombstone()
                && fold_box_side_override(sequence, shorthand, current, family, side)
            {
                cx.record_declaration_removed();
                return true;
            }

            state.pending_longhands.push(current);
            state.sides[side] = current;
            if state.sides.contains(&DeclarationLocation::EMPTY) {
                return false;
            }
            let locations = state.sides;
            if merge_box_longhands(sequence, locations, family, cx) {
                let target = *locations.iter().max().expect("four box sides");
                state.clear();
                state.shorthand = target;
            }
            false
        }
    }
}

fn can_override_box_longhands(declaration: &Declaration<'_>, _family: BoxFamily) -> bool {
    matches!(
        declaration,
        Declaration::Margin(..) | Declaration::Padding(..)
    )
}

fn fold_box_side_override(
    sequence: &mut DeclarationSequence<'_, '_, '_>,
    shorthand: DeclarationLocation,
    longhand: DeclarationLocation,
    family: BoxFamily,
    side: usize,
) -> bool {
    debug_assert!(shorthand < longhand);
    let shorthand_declaration = sequence.replace(shorthand, Declaration::Tombstone);
    let mut longhand_declaration = sequence.replace(longhand, Declaration::Tombstone);
    let folded = match (family, &shorthand_declaration, &mut longhand_declaration) {
        (BoxFamily::Margin, Declaration::Margin(value), longhand) => match (side, longhand) {
            (0, Declaration::MarginTop(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.top, target));
                true
            }
            (1, Declaration::MarginRight(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.right, target));
                true
            }
            (2, Declaration::MarginBottom(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.bottom, target));
                true
            }
            (3, Declaration::MarginLeft(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.left, target));
                true
            }
            _ => false,
        },
        (BoxFamily::Padding, Declaration::Padding(value), longhand) => match (side, longhand) {
            (0, Declaration::PaddingTop(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.top, target));
                true
            }
            (1, Declaration::PaddingRight(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.right, target));
                true
            }
            (2, Declaration::PaddingBottom(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.bottom, target));
                true
            }
            (3, Declaration::PaddingLeft(target)) => {
                sequence
                    .ast_context_mut()
                    .mutate_node(*value, |value, _| std::mem::swap(&mut value.left, target));
                true
            }
            _ => false,
        },
        _ => false,
    };
    sequence.replace(shorthand, shorthand_declaration);
    if !folded {
        sequence.replace(longhand, longhand_declaration);
    }
    folded
}

fn merge_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    locations: [DeclarationLocation; 4],
    family: BoxFamily,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    if let Some(keyword) = css_wide_box_longhands(sequence, locations) {
        let target = *locations.iter().max().expect("four box sides");
        let property_id = match family {
            BoxFamily::Margin => PropertyId::Margin,
            BoxFamily::Padding => PropertyId::Padding,
        };
        let property_id = sequence
            .ast_context_mut()
            .alloc_node_without_span(property_id);
        for &location in &locations {
            if location != target {
                sequence.replace(location, Declaration::Tombstone);
            }
        }
        sequence.replace(target, Declaration::CSSWide(property_id, keyword));
        record_merged_longhands(locations, target, cx);
        return true;
    }

    let typed = match family {
        BoxFamily::Margin => locations.iter().all(|&location| {
            matches!(
                sequence.declaration(location),
                Declaration::MarginTop(_)
                    | Declaration::MarginRight(_)
                    | Declaration::MarginBottom(_)
                    | Declaration::MarginLeft(_)
            )
        }),
        BoxFamily::Padding => locations.iter().all(|&location| {
            matches!(
                sequence.declaration(location),
                Declaration::PaddingTop(_)
                    | Declaration::PaddingRight(_)
                    | Declaration::PaddingBottom(_)
                    | Declaration::PaddingLeft(_)
            )
        }),
    };
    if typed {
        return merge_typed_box_longhands(sequence, locations, family, cx);
    }
    false
}

fn css_wide_box_longhands(
    sequence: &DeclarationSequence<'_, '_, '_>,
    locations: [DeclarationLocation; 4],
) -> Option<rocketcss_ast::CSSWideKeyword> {
    let Declaration::CSSWide(_, keyword) = sequence.declaration(locations[0]) else {
        return None;
    };
    locations[1..]
        .iter()
        .all(|&location| {
            matches!(
                sequence.declaration(location),
                Declaration::CSSWide(_, candidate) if candidate == keyword
            )
        })
        .then_some(*keyword)
}

fn merge_typed_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, '_, 'ast>,
    [top, right, bottom, left]: [DeclarationLocation; 4],
    family: BoxFamily,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    let target = [top, right, bottom, left]
        .into_iter()
        .max()
        .expect("four box sides");
    let top_declaration = sequence.replace(top, Declaration::Tombstone);
    let right_declaration = sequence.replace(right, Declaration::Tombstone);
    let bottom_declaration = sequence.replace(bottom, Declaration::Tombstone);
    let left_declaration = sequence.replace(left, Declaration::Tombstone);
    match family {
        BoxFamily::Margin => {
            let Declaration::MarginTop(top_value) = top_declaration else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginRight(right_value) = right_declaration else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginBottom(bottom_value) = bottom_declaration else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginLeft(left_value) = left_declaration else {
                unreachable!("typed margin IR validates every side")
            };
            let value = sequence.ast_context_mut().alloc_node_without_span(Margin {
                top: top_value,
                right: right_value,
                bottom: bottom_value,
                left: left_value,
            });
            sequence.replace(target, Declaration::Margin(value));
        }
        BoxFamily::Padding => {
            let Declaration::PaddingTop(top_value) = top_declaration else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingRight(right_value) = right_declaration else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingBottom(bottom_value) = bottom_declaration else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingLeft(left_value) = left_declaration else {
                unreachable!("typed padding IR validates every side")
            };
            let value = sequence.ast_context_mut().alloc_node_without_span(Padding {
                top: top_value,
                right: right_value,
                bottom: bottom_value,
                left: left_value,
            });
            sequence.replace(target, Declaration::Padding(value));
        }
    }
    record_merged_longhands([top, right, bottom, left], target, cx);
    true
}

fn record_merged_longhands(
    locations: [DeclarationLocation; 4],
    target: DeclarationLocation,
    cx: &mut MinifyContext,
) {
    for location in locations {
        if location != target {
            cx.record_declaration_removed();
        }
    }
}

fn declaration_skips_minification<'ast>(
    declaration: &Declaration<'ast>,
    ast: &AstContext<'ast>,
) -> bool {
    matches!(declaration, Declaration::Unparsed(_))
        || functional_color_requires_history_barrier(declaration, ast)
}

fn functional_color_requires_history_barrier<'ast>(
    declaration: &Declaration<'ast>,
    ast: &AstContext<'ast>,
) -> bool {
    let mut visitor = FunctionalColorBarrierVisitor::default();
    GhostToken::scope(|token| {
        declaration.visit(&mut visitor, &VisitContext::with_ast(&token, ast));
    });
    visitor.requires_barrier
}

#[derive(Default)]
struct FunctionalColorBarrierVisitor {
    requires_barrier: bool,
}

impl<'ast, 'ghost> Visitor<'ast, 'ghost> for FunctionalColorBarrierVisitor {
    fn visit_css_color(&mut self, node: &CssColor<'ast>, cx: &VisitContext<'_, 'ast, 'ghost>) {
        if matches!(
            node,
            CssColor::Function(function)
                if cx.ast_context().resolve_node(*function).replacement.is_none()
        ) {
            self.requires_barrier = true;
        } else {
            node.visit_children(self, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_ast::{
        BorderColor, Function, Gradient, GradientItem, Image, KnownColor, LineDirection, SVGPaint,
        VerticalPositionKeyword,
    };

    #[test]
    fn declaration_location_fits_in_one_word() {
        assert_eq!(std::mem::size_of::<DeclarationLocation>(), 4);

        let location = DeclarationLocation::new(0x1234, 0xabcd);
        assert_eq!(location.block(), 0x1234);
        assert_eq!(location.declaration(), 0xabcd);
        assert_ne!(location, DeclarationLocation::EMPTY);
    }

    #[test]
    fn known_declaration_key_round_trips_packed_fields() {
        let prefix = VendorPrefix::WEBKIT | VendorPrefix::MOZ;
        let key = KnownDeclarationKey::new(349, prefix, true);

        assert_eq!(key.property_id(), 349);
        assert_eq!(key.vendor_prefix(), prefix);
        assert!(key.is_important());
    }

    #[test]
    fn functional_color_barrier_walks_nested_color_values() {
        fn unresolved_color<'ast>(
            allocator: &'ast Allocator,
            ast: &mut AstContext<'ast>,
        ) -> rocketcss_ast::NodeId<'ast, CssColor<'ast>> {
            let arguments = ast.alloc_vec(rocketcss_common::vec::Vec::new_in(allocator));
            let function = ast.alloc_node_without_span(Function::new("color-mix", arguments));
            ast.alloc_node_without_span(CssColor::Function(function))
        }

        fn known_color<'ast>(
            ast: &mut AstContext<'ast>,
        ) -> rocketcss_ast::NodeId<'ast, CssColor<'ast>> {
            ast.alloc_node_without_span(CssColor::Known(KnownColor::Red))
        }

        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let direct = Declaration::Color(unresolved_color(&allocator, &mut ast));
        let bottom = known_color(&mut ast);
        let left = known_color(&mut ast);
        let right = known_color(&mut ast);
        let top = unresolved_color(&allocator, &mut ast);
        let border = Declaration::BorderColor(ast.alloc_node_without_span(BorderColor {
            bottom,
            left,
            right,
            top,
        }));
        let mut gradient_items = allocator.vec();
        gradient_items.push(GradientItem::ColorStop {
            color: unresolved_color(&allocator, &mut ast),
            position: None,
        });
        let gradient_items = ast.alloc_vec(gradient_items);
        let mut images = allocator.vec();
        let gradient = ast.alloc_node_without_span(Gradient::Linear {
            direction: LineDirection::Vertical(VerticalPositionKeyword::Bottom),
            items: gradient_items,
            vendor_prefix: VendorPrefix::NONE,
        });
        images.push(Image::Gradient(gradient));
        let images = ast.alloc_vec(images);
        let background_image = Declaration::BackgroundImage(images);
        let fill_color = unresolved_color(&allocator, &mut ast);
        let fill = Declaration::Fill(ast.alloc_node_without_span(SVGPaint::Color(fill_color)));
        let plain = Declaration::Color(known_color(&mut ast));

        for declaration in [&direct, &border, &background_image, &fill] {
            assert!(functional_color_requires_history_barrier(declaration, &ast));
        }
        assert!(!functional_color_requires_history_barrier(&plain, &ast));
    }
}

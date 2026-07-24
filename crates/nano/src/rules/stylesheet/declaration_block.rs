use crate::{Minify, MinifyContext, Options, OptionsOp};
use rocketcss_allocator::{
    GhostCell, GhostToken,
    prelude::{AdaptiveHashMap, Allocator, Vec},
};
use rocketcss_ast::{
    CSSWideOr, Columns, Declaration, DeclarationBlock, EqIgnoringTombstones, KnownFunction,
    LengthValue, Margin, Padding, PropertyId, Token, TokenOrValue, UnparsedProperty, VendorPrefix,
    match_ignore_ascii_case,
};

fn token_or_value_contains_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
        TokenOrValue::Function(function) => {
            matches!(function.kind(), KnownFunction::Var | KnownFunction::Env)
                || function
                    .arguments
                    .iter()
                    .any(token_or_value_contains_variable)
        }
        _ => false,
    }
}

impl<'a> Minify for DeclarationBlock<'a> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        let allocator = cx.allocator();
        let mut minifier = DeclarationBlockMinifier::new(allocator);
        minifier.minify(self, cx);
    }
}

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

#[derive(Clone, Copy, Debug)]
enum BoxFamily {
    Margin,
    Padding,
}

impl BoxFamily {
    const COUNT: usize = 2;

    #[inline]
    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug)]
enum BoxProperty {
    Shorthand(BoxFamily),
    Longhand(BoxFamily, usize),
    Barrier(BoxFamily),
    BarrierAll,
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

enum DeclarationBlocks<'sequence, 'ast, 'ghost> {
    Direct(&'sequence mut DeclarationBlock<'ast>),
    Ghost {
        blocks: &'sequence [&'ast GhostCell<'ghost, DeclarationBlock<'ast>>],
        token: &'sequence mut GhostToken<'ghost>,
    },
}

struct DeclarationSequence<'sequence, 'ast, 'ghost> {
    blocks: DeclarationBlocks<'sequence, 'ast, 'ghost>,
}

impl<'sequence, 'ast, 'ghost> DeclarationSequence<'sequence, 'ast, 'ghost> {
    #[inline]
    fn direct(block: &'sequence mut DeclarationBlock<'ast>) -> Self {
        Self {
            blocks: DeclarationBlocks::Direct(block),
        }
    }

    #[inline]
    fn ghost(
        blocks: &'sequence [&'ast GhostCell<'ghost, DeclarationBlock<'ast>>],
        token: &'sequence mut GhostToken<'ghost>,
    ) -> Self {
        Self {
            blocks: DeclarationBlocks::Ghost { blocks, token },
        }
    }

    #[inline]
    fn block_count(&self) -> usize {
        match &self.blocks {
            DeclarationBlocks::Direct(_) => 1,
            DeclarationBlocks::Ghost { blocks, .. } => blocks.len(),
        }
    }

    #[inline]
    fn locations_fit(&self) -> bool {
        let block_count = self.block_count();
        block_count <= DeclarationLocation::MAX_COUNT
            && (0..block_count)
                .all(|block| self.block(block).len() <= DeclarationLocation::MAX_COUNT)
    }

    #[inline]
    fn block(&self, index: usize) -> &DeclarationBlock<'ast> {
        match &self.blocks {
            DeclarationBlocks::Direct(block) => {
                debug_assert_eq!(index, 0);
                block
            }
            DeclarationBlocks::Ghost { blocks, token } => blocks[index].borrow(token),
        }
    }

    #[inline]
    fn block_mut(&mut self, index: usize) -> &mut DeclarationBlock<'ast> {
        match &mut self.blocks {
            DeclarationBlocks::Direct(block) => {
                debug_assert_eq!(index, 0);
                block
            }
            DeclarationBlocks::Ghost { blocks, token } => blocks[index].borrow_mut(token),
        }
    }

    #[inline]
    fn declaration(&self, location: DeclarationLocation) -> &Declaration<'ast> {
        &self.block(location.block()).declarations[location.declaration()]
    }

    #[inline]
    fn declaration_mut(&mut self, location: DeclarationLocation) -> &mut Declaration<'ast> {
        &mut self.block_mut(location.block()).declarations[location.declaration()]
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
        self.block(location.block())
            .is_important(location.declaration())
    }

    #[inline]
    fn allocator(&self, location: DeclarationLocation) -> &'ast Allocator {
        self.block(location.block()).declarations.bump()
    }
}

pub(crate) struct DeclarationBlockMinifier<'scratch, 'ast> {
    allocator: &'scratch Allocator,
    ir: DeclarationIr<'scratch, 'ast>,
}

impl<'scratch, 'ast> DeclarationBlockMinifier<'scratch, 'ast> {
    pub(crate) fn new(allocator: &'scratch Allocator) -> Self {
        Self {
            allocator,
            ir: DeclarationIr::new(allocator),
        }
    }

    #[inline]
    pub(crate) fn allocator(&self) -> &'scratch Allocator {
        self.allocator
    }

    #[inline]
    pub(crate) fn minify(
        &mut self,
        block: &mut DeclarationBlock<'ast>,
        cx: &mut MinifyContext<'scratch>,
    ) {
        if block.len() < 2 {
            return;
        }
        let mut sequence = DeclarationSequence::direct(block);
        self.minify_non_trivial(&mut sequence, cx);
    }

    fn minify_non_trivial(
        &mut self,
        sequence: &mut DeclarationSequence<'_, 'ast, '_>,
        cx: &mut MinifyContext<'scratch>,
    ) {
        if !sequence.locations_fit() {
            return;
        }
        self.ir.clear();
        deduplicate_declarations(sequence, &mut self.ir, cx);
    }

    pub(crate) fn minify_sequence<'ghost>(
        &mut self,
        blocks: &[&'ast GhostCell<'ghost, DeclarationBlock<'ast>>],
        token: &mut GhostToken<'ghost>,
        cx: &mut MinifyContext<'scratch>,
    ) {
        let mut sequence = DeclarationSequence::ghost(blocks, token);
        self.minify_non_trivial(&mut sequence, cx);
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
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    for block in 0..sequence.block_count() {
        let declaration_count = sequence.block(block).len();
        for declaration in 0..declaration_count {
            let current = DeclarationLocation::new(block, declaration);
            if sequence.declaration(current).is_tombstone() {
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
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    current: DeclarationLocation,
    important: bool,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    let Some(property) = columns_property(sequence.declaration(current)) else {
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
fn columns_property(declaration: &Declaration<'_>) -> Option<ColumnsProperty> {
    let property_id = match declaration {
        Declaration::Columns(_, prefix) => return Some(ColumnsProperty::Shorthand(*prefix)),
        Declaration::ColumnWidth(_, prefix) => return Some(ColumnsProperty::Width(*prefix)),
        Declaration::ColumnCount(_, prefix) => return Some(ColumnsProperty::Count(*prefix)),
        Declaration::All(_) => return Some(ColumnsProperty::BarrierAll),
        Declaration::Unparsed(value) => &*value.property_id,
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
    let mut longhand_declaration = sequence.replace(longhand, Declaration::Tombstone);
    let folded = match (
        sequence.declaration_mut(shorthand),
        &mut longhand_declaration,
    ) {
        (
            Declaration::Columns(CSSWideOr::Value(value), shorthand_prefix),
            Declaration::ColumnWidth(CSSWideOr::Value(width), longhand_prefix),
        ) if *shorthand_prefix == prefix && *longhand_prefix == prefix => {
            std::mem::swap(&mut value.width, width);
            true
        }
        (
            Declaration::Columns(CSSWideOr::Value(value), shorthand_prefix),
            Declaration::ColumnCount(CSSWideOr::Value(count), longhand_prefix),
        ) if *shorthand_prefix == prefix && *longhand_prefix == prefix => {
            std::mem::swap(&mut value.count, count);
            true
        }
        _ => false,
    };
    if !folded {
        sequence.replace(longhand, longhand_declaration);
    }
    folded
}

fn merge_columns_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    width: DeclarationLocation,
    count: DeclarationLocation,
    prefix: VendorPrefix,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    let target = std::cmp::max(width, count);
    let allocator = sequence.allocator(target);
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
        sequence.replace(
            target,
            Declaration::Columns(
                CSSWideOr::Value(allocator.boxed(Columns {
                    count: count_value,
                    width: width_value,
                })),
                prefix,
            ),
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
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    current: DeclarationLocation,
    important: bool,
    declarations: &mut DeclarationMap<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    let declaration = sequence.declaration(current);
    let previous = if let Some((property_id, vendor_prefix)) = declaration.known_id_and_prefix() {
        declarations.insert_known(property_id, vendor_prefix, important, current)
    } else {
        declarations.insert_unknown(
            declaration
                .property_id()
                .expect("tombstones are skipped before exact deduplication"),
            important,
            current,
        )
    };
    if let Some(previous) = previous
        && !sequence.declaration(previous).is_tombstone()
        && sequence
            .declaration(previous)
            .eq_ignoring_tombstones(sequence.declaration(current))
    {
        sequence.replace(previous, Declaration::Tombstone);
        cx.record_declaration_removed();
    }
}

fn process_box_declaration<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    current: DeclarationLocation,
    important: bool,
    ir: &mut DeclarationIr<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> bool
where
    'ast: 'scratch,
{
    let Some(property) = box_property(sequence.declaration(current)) else {
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
                minify_unparsed_declaration(sequence, shorthand, cx);
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

#[inline]
fn box_property(declaration: &Declaration<'_>) -> Option<BoxProperty> {
    let property_id = match declaration {
        Declaration::Margin(..) => return Some(BoxProperty::Shorthand(BoxFamily::Margin)),
        Declaration::MarginTop(..) => return Some(BoxProperty::Longhand(BoxFamily::Margin, 0)),
        Declaration::MarginRight(..) => return Some(BoxProperty::Longhand(BoxFamily::Margin, 1)),
        Declaration::MarginBottom(..) => {
            return Some(BoxProperty::Longhand(BoxFamily::Margin, 2));
        }
        Declaration::MarginLeft(..) => return Some(BoxProperty::Longhand(BoxFamily::Margin, 3)),
        Declaration::Padding(..) => return Some(BoxProperty::Shorthand(BoxFamily::Padding)),
        Declaration::PaddingTop(..) => return Some(BoxProperty::Longhand(BoxFamily::Padding, 0)),
        Declaration::PaddingRight(..) => {
            return Some(BoxProperty::Longhand(BoxFamily::Padding, 1));
        }
        Declaration::PaddingBottom(..) => {
            return Some(BoxProperty::Longhand(BoxFamily::Padding, 2));
        }
        Declaration::PaddingLeft(..) => return Some(BoxProperty::Longhand(BoxFamily::Padding, 3)),
        Declaration::All(..) => return Some(BoxProperty::BarrierAll),
        Declaration::Unparsed(value) => &*value.property_id,
        _ => return None,
    };
    match property_id {
        PropertyId::Margin => Some(BoxProperty::Shorthand(BoxFamily::Margin)),
        PropertyId::MarginTop => Some(BoxProperty::Longhand(BoxFamily::Margin, 0)),
        PropertyId::MarginRight => Some(BoxProperty::Longhand(BoxFamily::Margin, 1)),
        PropertyId::MarginBottom => Some(BoxProperty::Longhand(BoxFamily::Margin, 2)),
        PropertyId::MarginLeft => Some(BoxProperty::Longhand(BoxFamily::Margin, 3)),
        PropertyId::MarginBlockStart
        | PropertyId::MarginBlockEnd
        | PropertyId::MarginInlineStart
        | PropertyId::MarginInlineEnd
        | PropertyId::MarginBlock
        | PropertyId::MarginInline => Some(BoxProperty::Barrier(BoxFamily::Margin)),
        PropertyId::Padding => Some(BoxProperty::Shorthand(BoxFamily::Padding)),
        PropertyId::PaddingTop => Some(BoxProperty::Longhand(BoxFamily::Padding, 0)),
        PropertyId::PaddingRight => Some(BoxProperty::Longhand(BoxFamily::Padding, 1)),
        PropertyId::PaddingBottom => Some(BoxProperty::Longhand(BoxFamily::Padding, 2)),
        PropertyId::PaddingLeft => Some(BoxProperty::Longhand(BoxFamily::Padding, 3)),
        PropertyId::PaddingBlockStart
        | PropertyId::PaddingBlockEnd
        | PropertyId::PaddingInlineStart
        | PropertyId::PaddingInlineEnd
        | PropertyId::PaddingBlock
        | PropertyId::PaddingInline => Some(BoxProperty::Barrier(BoxFamily::Padding)),
        PropertyId::All => Some(BoxProperty::BarrierAll),
        _ => None,
    }
}

fn can_override_box_longhands(declaration: &Declaration<'_>, family: BoxFamily) -> bool {
    match declaration {
        Declaration::Margin(..) | Declaration::Padding(..) => true,
        Declaration::Unparsed(value) => {
            !declaration_contains_variable(declaration)
                && box_component_count(&value.value, family).is_some()
        }
        _ => false,
    }
}

fn box_component_count(values: &[TokenOrValue<'_>], family: BoxFamily) -> Option<usize> {
    let count = match values.len() {
        1 => 1,
        3 | 5 | 7
            if values.iter().enumerate().all(|(index, value)| {
                index % 2 == 0
                    || matches!(value, TokenOrValue::Token(token)
                        if matches!(&**token, Token::WhiteSpace(_)))
            }) =>
        {
            values.len().div_ceil(2)
        }
        _ => return None,
    };
    let mut components = values.iter().step_by(2);
    if components.clone().any(is_css_wide_value) && count != 1 {
        return None;
    }
    components
        .all(|value| is_box_component(value, family))
        .then_some(count)
}

fn is_box_component(value: &TokenOrValue<'_>, family: BoxFamily) -> bool {
    match value {
        TokenOrValue::Length(_) => true,
        TokenOrValue::Function(function) => match_ignore_ascii_case!(
            function.name(),
            "calc" | "min" | "max" | "clamp" | "anchor-size" => true,
            _ => false,
        ),
        TokenOrValue::Token(token) => match &**token {
            Token::Number(value) => *value == 0.0,
            Token::Percentage(_) => true,
            Token::Dimension { unit, .. } => unit.is_length(),
            Token::Ident(value) => {
                is_css_wide_keyword(value)
                    || matches!(family, BoxFamily::Margin)
                        && match_ignore_ascii_case!(value, "auto" => true, _ => false)
            }
            _ => false,
        },
        _ => false,
    }
}

fn fold_box_side_override(
    sequence: &mut DeclarationSequence<'_, '_, '_>,
    shorthand: DeclarationLocation,
    longhand: DeclarationLocation,
    family: BoxFamily,
    side: usize,
) -> bool {
    debug_assert!(shorthand < longhand);
    let mut longhand_declaration = sequence.replace(longhand, Declaration::Tombstone);
    let folded = match (
        family,
        sequence.declaration_mut(shorthand),
        &mut longhand_declaration,
    ) {
        (BoxFamily::Margin, Declaration::Margin(value), longhand) => match (side, longhand) {
            (0, Declaration::MarginTop(target)) => {
                std::mem::swap(&mut value.top, target);
                true
            }
            (1, Declaration::MarginRight(target)) => {
                std::mem::swap(&mut value.right, target);
                true
            }
            (2, Declaration::MarginBottom(target)) => {
                std::mem::swap(&mut value.bottom, target);
                true
            }
            (3, Declaration::MarginLeft(target)) => {
                std::mem::swap(&mut value.left, target);
                true
            }
            _ => false,
        },
        (BoxFamily::Padding, Declaration::Padding(value), longhand) => match (side, longhand) {
            (0, Declaration::PaddingTop(target)) => {
                std::mem::swap(&mut value.top, target);
                true
            }
            (1, Declaration::PaddingRight(target)) => {
                std::mem::swap(&mut value.right, target);
                true
            }
            (2, Declaration::PaddingBottom(target)) => {
                std::mem::swap(&mut value.bottom, target);
                true
            }
            (3, Declaration::PaddingLeft(target)) => {
                std::mem::swap(&mut value.left, target);
                true
            }
            _ => false,
        },
        (_, Declaration::Unparsed(shorthand), Declaration::Unparsed(longhand)) => {
            fold_unparsed_box_side(shorthand, longhand, family, side)
        }
        _ => false,
    };
    if !folded {
        sequence.replace(longhand, longhand_declaration);
    }
    folded
}

fn fold_unparsed_box_side<'a>(
    shorthand: &mut UnparsedProperty<'a>,
    longhand: &mut UnparsedProperty<'a>,
    family: BoxFamily,
    side: usize,
) -> bool {
    if longhand.value.len() != 1
        || !is_box_component(&longhand.value[0], family)
        || is_css_wide_value(&longhand.value[0])
        || longhand.value.iter().any(token_or_value_contains_variable)
        || shorthand.value.iter().any(token_or_value_contains_variable)
    {
        return false;
    }
    let Some(component_count) = box_component_count(&shorthand.value, family) else {
        return false;
    };
    let component = match (component_count, side) {
        (1, _) => 0,
        (2, 0 | 2) => 0,
        (2, 1 | 3) => 1,
        (3, 0) => 0,
        (3, 1 | 3) => 1,
        (3, 2) => 2,
        (4, side) => side,
        _ => unreachable!(),
    };
    let value_index = component * 2;
    if shorthand.value[value_index] == longhand.value[0] {
        return true;
    }
    if shorthand.value.iter().step_by(2).any(is_css_wide_value) {
        return false;
    }
    let component_is_unique = match component_count {
        1 | 2 => false,
        3 => side == 0 || side == 2,
        4 => true,
        _ => unreachable!(),
    };
    let allocator = shorthand.value.bump();
    let mut target_index = value_index;
    if component_count < 4 && !component_is_unique {
        let additions = match component_count {
            1 => [
                clone_simple_token_or_value(&shorthand.value[0], allocator),
                clone_simple_token_or_value(&shorthand.value[0], allocator),
                clone_simple_token_or_value(&shorthand.value[0], allocator),
            ],
            2 => [
                clone_simple_token_or_value(&shorthand.value[0], allocator),
                clone_simple_token_or_value(&shorthand.value[2], allocator),
                None,
            ],
            3 => [
                clone_simple_token_or_value(&shorthand.value[2], allocator),
                None,
                None,
            ],
            _ => unreachable!(),
        };
        if additions.iter().flatten().count() != 4 - component_count {
            return false;
        }
        for addition in additions.into_iter().flatten() {
            shorthand
                .value
                .push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
            shorthand.value.push(addition);
        }
        target_index = side * 2;
    }
    std::mem::swap(&mut shorthand.value[target_index], &mut longhand.value[0]);
    true
}

fn clone_simple_token_or_value<'a>(
    value: &TokenOrValue<'a>,
    allocator: &'a Allocator,
) -> Option<TokenOrValue<'a>> {
    match value {
        TokenOrValue::Token(token)
            if matches!(
                &**token,
                Token::Ident(_) | Token::Number(_) | Token::Percentage(_) | Token::Dimension { .. }
            ) =>
        {
            Some(TokenOrValue::Token(allocator.boxed((**token).clone())))
        }
        TokenOrValue::Length(length) => Some(TokenOrValue::Length(LengthValue {
            unit: length.unit,
            value: length.value,
        })),
        TokenOrValue::DashedIdent(value) => Some(TokenOrValue::DashedIdent(value)),
        _ => None,
    }
}

fn merge_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    locations: [DeclarationLocation; 4],
    family: BoxFamily,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
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
    merge_unparsed_box_longhands(sequence, locations, family, cx)
}

fn merge_typed_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
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
    let allocator = sequence.allocator(target);
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
            sequence.replace(
                target,
                Declaration::Margin(allocator.boxed(Margin {
                    top: top_value,
                    right: right_value,
                    bottom: bottom_value,
                    left: left_value,
                })),
            );
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
            sequence.replace(
                target,
                Declaration::Padding(allocator.boxed(Padding {
                    top: top_value,
                    right: right_value,
                    bottom: bottom_value,
                    left: left_value,
                })),
            );
        }
    }
    record_merged_longhands([top, right, bottom, left], target, cx);
    true
}

fn merge_unparsed_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    locations: [DeclarationLocation; 4],
    family: BoxFamily,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    if locations.iter().any(|&location| {
        !matches!(sequence.declaration(location), Declaration::Unparsed(value)
            if value.value.len() == 1 && is_box_component(&value.value[0], family))
    }) {
        return false;
    }
    let variable_count = locations
        .iter()
        .filter(|&&location| declaration_contains_variable(sequence.declaration(location)))
        .count();
    if variable_count != 0 && variable_count != locations.len() {
        return false;
    }
    let first_value = match sequence.declaration(locations[0]) {
        Declaration::Unparsed(value) => &value.value,
        _ => unreachable!(),
    };
    let all_equal = locations[1..].iter().all(|&location| {
        matches!(sequence.declaration(location), Declaration::Unparsed(value)
            if value.value == *first_value)
    });
    if !all_equal
        && locations.iter().any(|&location| {
            matches!(sequence.declaration(location), Declaration::Unparsed(value)
                if value.value.iter().any(is_css_wide_value))
        })
    {
        return false;
    }

    let target = *locations.iter().max().expect("four box sides");
    let allocator = sequence.allocator(target);
    let mut sides = locations.map(|location| {
        let Declaration::Unparsed(value) = sequence.declaration_mut(location) else {
            unreachable!("unparsed box IR validates every side")
        };
        std::mem::replace(&mut value.value, allocator.vec())
    });
    let mut value = std::mem::replace(&mut sides[0], allocator.vec());
    if !all_equal {
        for side in &mut sides[1..] {
            value.push(TokenOrValue::Token(allocator.boxed(Token::WhiteSpace(" "))));
            value.append(side);
        }
    }
    let Declaration::Unparsed(target_value) = sequence.declaration_mut(target) else {
        unreachable!("unparsed box IR target remains unparsed")
    };
    *target_value.property_id = match family {
        BoxFamily::Margin => PropertyId::Margin,
        BoxFamily::Padding => PropertyId::Padding,
    };
    target_value.value = value;
    for &location in &locations {
        if location != target {
            sequence.replace(location, Declaration::Tombstone);
        }
    }
    record_merged_longhands(locations, target, cx);
    minify_unparsed_declaration(sequence, target, cx);
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

fn minify_unparsed_declaration<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast, '_>,
    location: DeclarationLocation,
    cx: &mut MinifyContext<'cx>,
) where
    'ast: 'cx,
{
    let Declaration::Unparsed(value) = sequence.declaration_mut(location) else {
        return;
    };
    let previous = cx.value_context;
    cx.value_context = crate::properties::value_context(
        &value.property_id,
        cx.is_enabled(Options::ORDER_VALUES, OptionsOp::Any),
        cx.is_enabled(Options::CONVERT_ZERO_PERCENTAGES, OptionsOp::Any),
    );
    value.minify(cx);
    cx.value_context = previous;
}

fn is_css_wide_value(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token)
        if matches!(&**token, Token::Ident(value) if is_css_wide_keyword(value)))
}

fn is_css_wide_keyword(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "initial" | "inherit" | "unset" | "revert" | "revert-layer" => true,
        _ => false,
    )
}

fn declaration_contains_variable(declaration: &Declaration<'_>) -> bool {
    matches!(declaration, Declaration::Unparsed(value)
        if value.value.iter().any(token_or_value_contains_variable))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

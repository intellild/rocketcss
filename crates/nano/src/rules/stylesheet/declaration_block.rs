use crate::MinifyContext;
use rocketcss_ast::{
    CSSWideOr, Columns, CssColor, Declaration, EqIgnoringTombstones, Margin, Padding, PropertyId,
    VendorPrefix, Visit, VisitContext, Visitor,
    radix_ast::{
        Compilation, DeclarationBlockId as RadixDeclarationBlockId,
        DeclarationId as RadixDeclarationId, DeclarationPayload,
    },
};
use rocketcss_common::{
    GhostToken,
    prelude::{AdaptiveHashMap, Allocator, Vec},
};

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

struct DeclarationSequence<'sequence, 'ast> {
    blocks: &'sequence [RadixDeclarationBlockId],
    compilation: &'sequence mut Compilation<'ast>,
}

impl<'sequence, 'ast> DeclarationSequence<'sequence, 'ast> {
    #[inline]
    fn radix(
        blocks: &'sequence [RadixDeclarationBlockId],
        compilation: &'sequence mut Compilation<'ast>,
    ) -> Self {
        Self {
            blocks,
            compilation,
        }
    }

    #[inline]
    fn block_count(&self) -> usize {
        self.blocks.len()
    }

    #[inline]
    fn locations_fit(&self) -> bool {
        let block_count = self.block_count();
        block_count <= DeclarationLocation::MAX_COUNT
            && (0..block_count).all(|block| self.block_len(block) <= DeclarationLocation::MAX_COUNT)
    }

    #[inline]
    fn block_len(&self, index: usize) -> usize {
        self.compilation
            .declaration_occurrences_in_block(self.blocks[index])
            .map_or(0, |declarations| declarations.len())
    }

    #[inline]
    fn radix_declaration_id(
        blocks: &[RadixDeclarationBlockId],
        compilation: &Compilation<'_>,
        location: DeclarationLocation,
    ) -> RadixDeclarationId {
        compilation
            .declaration_id_at_in_block(blocks[location.block()], location.declaration())
            .expect("the declaration location was validated against the block length")
    }

    #[inline]
    fn declaration(&self, location: DeclarationLocation) -> &Declaration<'ast> {
        let id = Self::radix_declaration_id(self.blocks, self.compilation, location);
        let DeclarationPayload::Property(declaration) = self
            .compilation
            .declaration(id)
            .expect("a Radix declaration sequence ID remains resolvable")
            .payload()
        else {
            unreachable!("local declaration minification only receives property blocks")
        };
        declaration
    }

    #[inline]
    fn declaration_mut(&mut self, location: DeclarationLocation) -> &mut Declaration<'ast> {
        let id = Self::radix_declaration_id(self.blocks, self.compilation, location);
        self.compilation
            .property_declaration_mut(self.blocks[location.block()], id)
            .expect("a Radix property declaration remains mutable")
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
        let id = Self::radix_declaration_id(self.blocks, self.compilation, location);
        self.compilation
            .declaration(id)
            .expect("a Radix declaration sequence ID remains resolvable")
            .is_important()
    }

    #[inline]
    fn allocator(&self, _location: DeclarationLocation) -> &'ast Allocator {
        self.compilation.allocator()
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
        compilation: &mut Compilation<'ast>,
        block: RadixDeclarationBlockId,
        cx: &mut MinifyContext<'scratch>,
    ) {
        let blocks = [block];
        let mut sequence = DeclarationSequence::radix(&blocks, compilation);
        if sequence.block_len(0) < 2 {
            return;
        }
        self.minify_non_trivial(&mut sequence, cx);
    }

    fn minify_non_trivial(
        &mut self,
        sequence: &mut DeclarationSequence<'_, 'ast>,
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
    sequence: &mut DeclarationSequence<'_, 'ast>,
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
            if declaration_skips_minification(sequence.declaration(current)) {
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
    sequence: &mut DeclarationSequence<'_, 'ast>,
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
    sequence: &mut DeclarationSequence<'_, '_>,
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
    sequence: &mut DeclarationSequence<'_, 'ast>,
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
    sequence: &mut DeclarationSequence<'_, 'ast>,
    current: DeclarationLocation,
    important: bool,
    declarations: &mut DeclarationMap<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) -> Option<DeclarationLocation>
where
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
        return Some(previous);
    }
    None
}

fn process_box_declaration<'scratch, 'ast>(
    sequence: &mut DeclarationSequence<'_, 'ast>,
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
        Declaration::CSSWide(property_id, _) => &**property_id,
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

fn can_override_box_longhands(declaration: &Declaration<'_>, _family: BoxFamily) -> bool {
    matches!(
        declaration,
        Declaration::Margin(..) | Declaration::Padding(..)
    )
}

fn fold_box_side_override(
    sequence: &mut DeclarationSequence<'_, '_>,
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
        _ => false,
    };
    if !folded {
        sequence.replace(longhand, longhand_declaration);
    }
    folded
}

fn merge_box_longhands<'ast, 'cx>(
    sequence: &mut DeclarationSequence<'_, 'ast>,
    locations: [DeclarationLocation; 4],
    family: BoxFamily,
    cx: &mut MinifyContext<'cx>,
) -> bool
where
    'ast: 'cx,
{
    if let Some(keyword) = css_wide_box_longhands(sequence, locations) {
        let target = *locations.iter().max().expect("four box sides");
        let allocator = sequence.allocator(target);
        let property_id = match family {
            BoxFamily::Margin => PropertyId::Margin,
            BoxFamily::Padding => PropertyId::Padding,
        };
        for &location in &locations {
            if location != target {
                sequence.replace(location, Declaration::Tombstone);
            }
        }
        sequence.replace(
            target,
            Declaration::CSSWide(allocator.boxed(property_id), keyword),
        );
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
    sequence: &DeclarationSequence<'_, '_>,
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
    sequence: &mut DeclarationSequence<'_, 'ast>,
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

fn declaration_skips_minification(declaration: &Declaration<'_>) -> bool {
    matches!(declaration, Declaration::Unparsed(_))
        || functional_color_requires_history_barrier(declaration)
}

fn functional_color_requires_history_barrier(declaration: &Declaration<'_>) -> bool {
    let mut visitor = FunctionalColorBarrierVisitor::default();
    GhostToken::scope(|token| {
        declaration.visit(&mut visitor, &VisitContext::new(&token));
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
            CssColor::Function(function) if function.replacement.is_none()
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
        let allocator = Allocator::new();
        let unresolved_color = || {
            CssColor::Function(allocator.boxed(Function::new(
                "color-mix",
                rocketcss_common::vec::Vec::new_in(&allocator),
            )))
        };
        let known_color = || CssColor::Known(KnownColor::Red);

        let direct = Declaration::Color(allocator.boxed(unresolved_color()));
        let border = Declaration::BorderColor(allocator.boxed(BorderColor {
            bottom: allocator.boxed(known_color()),
            left: allocator.boxed(known_color()),
            right: allocator.boxed(known_color()),
            top: allocator.boxed(unresolved_color()),
        }));
        let mut gradient_items = allocator.vec();
        gradient_items.push(GradientItem::ColorStop {
            color: allocator.boxed(unresolved_color()),
            position: None,
        });
        let mut images = allocator.vec();
        images.push(Image::Gradient(allocator.boxed(Gradient::Linear {
            direction: LineDirection::Vertical(VerticalPositionKeyword::Bottom),
            items: gradient_items,
            vendor_prefix: VendorPrefix::NONE,
        })));
        let background_image = Declaration::BackgroundImage(images);
        let fill = Declaration::Fill(
            allocator.boxed(SVGPaint::Color(allocator.boxed(unresolved_color()))),
        );
        let plain = Declaration::Color(allocator.boxed(known_color()));

        for declaration in [&direct, &border, &background_image, &fill] {
            assert!(functional_color_requires_history_barrier(declaration));
        }
        assert!(!functional_color_requires_history_barrier(&plain));
    }
}

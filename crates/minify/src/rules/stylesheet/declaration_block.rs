use rocketcss_allocator::prelude::{Allocator, HashMap, Vec};
use rocketcss_ast::{
    Declaration, DeclarationBlock, KnownFunction, LengthValue, Margin, Padding, PropertyId, Token,
    TokenOrValue, UnparsedProperty, match_ignore_ascii_case,
};

use crate::{Minify, MinifyContext, Options, OptionsOp};

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
    fn minify(&mut self, cx: &mut MinifyContext) {
        if self.len() < 2 {
            return;
        }
        let mut minifier = DeclarationBlockMinifier::new(self.declarations.bump());
        minifier.minify_non_trivial(self, cx);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct DeclarationKey<'a> {
    property_id: PropertyId<'a>,
    important: bool,
}

const EMPTY_INDEX: u32 = u32::MAX;
// Most declaration blocks are small. Keep their single-pass IR inline and
// promote larger blocks to the reusable arena map below.
const INLINE_DECLARATION_CAPACITY: usize = 8;

#[derive(Clone, Copy, Debug)]
struct DeclarationEntry<'a> {
    key: DeclarationKey<'a>,
    index: u32,
}

#[derive(Debug)]
struct DeclarationMap<'a> {
    entries: [Option<DeclarationEntry<'a>>; INLINE_DECLARATION_CAPACITY],
    len: u8,
    overflow: HashMap<'a, DeclarationKey<'a>, u32>,
    overflowed: bool,
}

impl<'a> DeclarationMap<'a> {
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            entries: [None; INLINE_DECLARATION_CAPACITY],
            len: 0,
            overflow: HashMap::new_in(allocator),
            overflowed: false,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.len = 0;
        if self.overflowed {
            self.overflow.clear();
            self.overflowed = false;
        }
    }

    #[inline]
    fn insert(&mut self, key: DeclarationKey<'a>, index: u32) -> Option<u32> {
        if self.overflowed {
            return self.overflow.insert(key, index);
        }

        for entry in self.entries[..usize::from(self.len)].iter_mut().flatten() {
            if entry.key == key {
                return Some(std::mem::replace(&mut entry.index, index));
            }
        }

        if usize::from(self.len) < INLINE_DECLARATION_CAPACITY {
            self.entries[usize::from(self.len)] = Some(DeclarationEntry { key, index });
            self.len += 1;
            return None;
        }

        for entry in self.entries.iter().flatten() {
            self.overflow.insert(entry.key, entry.index);
        }
        self.overflowed = true;
        self.overflow.insert(key, index)
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
    pending_longhands: Vec<'a, u32>,
    sides: [u32; 4],
    shorthand: u32,
}

impl<'a> BoxFamilyIr<'a> {
    #[inline]
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            pending_longhands: allocator.vec(),
            sides: [EMPTY_INDEX; 4],
            shorthand: EMPTY_INDEX,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.pending_longhands.clear();
        self.sides = [EMPTY_INDEX; 4];
        self.shorthand = EMPTY_INDEX;
    }
}

pub(crate) struct DeclarationBlockMinifier<'a> {
    ir: DeclarationIr<'a>,
}

impl<'a> DeclarationBlockMinifier<'a> {
    pub(crate) fn new(allocator: &'a Allocator) -> Self {
        Self {
            ir: DeclarationIr::new(allocator),
        }
    }

    #[inline]
    pub(crate) fn minify(&mut self, block: &mut DeclarationBlock<'a>, cx: &mut MinifyContext) {
        if block.len() < 2 {
            return;
        }
        self.minify_non_trivial(block, cx);
    }

    fn minify_non_trivial(&mut self, block: &mut DeclarationBlock<'a>, cx: &mut MinifyContext) {
        self.ir.clear();
        deduplicate_declarations(block, &mut self.ir, cx);
    }
}

#[derive(Debug)]
struct DeclarationIr<'a> {
    declarations: DeclarationMap<'a>,
    boxes: [[BoxFamilyIr<'a>; 2]; BoxFamily::COUNT],
    dirty_boxes: u8,
}

impl<'a> DeclarationIr<'a> {
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            declarations: DeclarationMap::new(allocator),
            boxes: std::array::from_fn(|_| std::array::from_fn(|_| BoxFamilyIr::new(allocator))),
            dirty_boxes: 0,
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.declarations.clear();
        self.clear_boxes();
    }

    #[inline]
    fn box_family(&mut self, family: BoxFamily, important: bool) -> &mut BoxFamilyIr<'a> {
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
}

fn deduplicate_declarations<'a>(
    block: &mut DeclarationBlock<'a>,
    ir: &mut DeclarationIr<'a>,
    cx: &mut MinifyContext,
) {
    for current in 0..block.len() {
        if block.declarations[current].is_tombstone() {
            continue;
        }
        process_box_declaration(block, current, ir, cx);
        if block.declarations[current].is_tombstone() {
            continue;
        }
        deduplicate_exact_declaration(block, current, &mut ir.declarations, cx);
    }
}

fn deduplicate_exact_declaration<'a>(
    block: &mut DeclarationBlock<'a>,
    current: usize,
    declarations: &mut DeclarationMap<'a>,
    cx: &mut MinifyContext,
) {
    let declaration = &block.declarations[current];
    let key = DeclarationKey {
        property_id: declaration
            .property_id()
            .expect("tombstones are skipped before exact deduplication"),
        important: block.is_important(current),
    };
    let current_index = current as u32;
    if let Some(previous) = declarations.insert(key, current_index) {
        let previous = previous as usize;
        if !block.declarations[previous].is_tombstone()
            && block.declarations[previous] == block.declarations[current]
        {
            block.declarations[previous] = Declaration::Tombstone;
            cx.record_declaration_removed();
        }
    }
}

fn process_box_declaration<'a>(
    block: &mut DeclarationBlock<'a>,
    current: usize,
    ir: &mut DeclarationIr<'a>,
    cx: &mut MinifyContext,
) {
    let Some(property) = box_property(&block.declarations[current]) else {
        return;
    };
    match property {
        BoxProperty::BarrierAll => ir.clear_boxes(),
        BoxProperty::Barrier(family) => ir.clear_box_family(family),
        BoxProperty::Shorthand(family) => {
            let important = block.is_important(current);
            let can_override = can_override_box_longhands(&block.declarations[current], family);
            let state = ir.box_family(family, important);
            if can_override {
                for &index in &state.pending_longhands {
                    let index = index as usize;
                    if !block.declarations[index].is_tombstone() {
                        block.declarations[index] = Declaration::Tombstone;
                        cx.record_declaration_removed();
                    }
                }
            }
            state.clear();
            if can_override {
                state.shorthand = current as u32;
            }
        }
        BoxProperty::Longhand(family, side) => {
            let important = block.is_important(current);
            let state = ir.box_family(family, important);
            let shorthand = state.shorthand as usize;
            if state.shorthand != EMPTY_INDEX
                && !block.declarations[shorthand].is_tombstone()
                && fold_box_side_override(block, shorthand, current, family, side)
            {
                block.declarations[current] = Declaration::Tombstone;
                cx.record_declaration_removed();
                minify_unparsed_declaration(block, shorthand, cx);
                return;
            }

            let current_index = current as u32;
            state.pending_longhands.push(current_index);
            state.sides[side] = current_index;
            if state.sides.contains(&EMPTY_INDEX) {
                return;
            }
            let indices = state.sides.map(|index| index as usize);
            if merge_box_longhands(block, indices, family, cx) {
                let target = *indices.iter().max().expect("four box sides");
                state.clear();
                state.shorthand = target as u32;
            }
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
    block: &mut DeclarationBlock<'_>,
    shorthand: usize,
    longhand: usize,
    family: BoxFamily,
    side: usize,
) -> bool {
    let (shorthand_declaration, longhand_declaration) = if shorthand < longhand {
        let (before, after) = block.declarations.split_at_mut(longhand);
        (&mut before[shorthand], &mut after[0])
    } else {
        unreachable!("the shorthand IR always precedes its longhand")
    };
    match (family, shorthand_declaration, longhand_declaration) {
        (BoxFamily::Margin, Declaration::Margin(value), longhand) => {
            let target = match (side, longhand) {
                (0, Declaration::MarginTop(value)) => value,
                (1, Declaration::MarginRight(value)) => value,
                (2, Declaration::MarginBottom(value)) => value,
                (3, Declaration::MarginLeft(value)) => value,
                _ => return false,
            };
            let shorthand_side = match side {
                0 => &mut value.top,
                1 => &mut value.right,
                2 => &mut value.bottom,
                3 => &mut value.left,
                _ => unreachable!(),
            };
            std::mem::swap(shorthand_side, target);
            true
        }
        (BoxFamily::Padding, Declaration::Padding(value), longhand) => {
            let target = match (side, longhand) {
                (0, Declaration::PaddingTop(value)) => value,
                (1, Declaration::PaddingRight(value)) => value,
                (2, Declaration::PaddingBottom(value)) => value,
                (3, Declaration::PaddingLeft(value)) => value,
                _ => return false,
            };
            let shorthand_side = match side {
                0 => &mut value.top,
                1 => &mut value.right,
                2 => &mut value.bottom,
                3 => &mut value.left,
                _ => unreachable!(),
            };
            std::mem::swap(shorthand_side, target);
            true
        }
        (_, Declaration::Unparsed(shorthand), Declaration::Unparsed(longhand)) => {
            fold_unparsed_box_side(shorthand, longhand, family, side)
        }
        _ => false,
    }
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
        TokenOrValue::Length(length) => Some(TokenOrValue::Length(allocator.boxed(LengthValue {
            unit: length.unit,
            value: length.value,
        }))),
        TokenOrValue::DashedIdent(value) => Some(TokenOrValue::DashedIdent(value)),
        _ => None,
    }
}

fn merge_box_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    indices: [usize; 4],
    family: BoxFamily,
    cx: &mut MinifyContext,
) -> bool {
    let typed = match family {
        BoxFamily::Margin => indices.iter().all(|&index| {
            matches!(
                block.declarations[index],
                Declaration::MarginTop(_)
                    | Declaration::MarginRight(_)
                    | Declaration::MarginBottom(_)
                    | Declaration::MarginLeft(_)
            )
        }),
        BoxFamily::Padding => indices.iter().all(|&index| {
            matches!(
                block.declarations[index],
                Declaration::PaddingTop(_)
                    | Declaration::PaddingRight(_)
                    | Declaration::PaddingBottom(_)
                    | Declaration::PaddingLeft(_)
            )
        }),
    };
    if typed {
        return merge_typed_box_longhands(block, indices, family, cx);
    }
    merge_unparsed_box_longhands(block, indices, family, cx)
}

fn merge_typed_box_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    [top, right, bottom, left]: [usize; 4],
    family: BoxFamily,
    cx: &mut MinifyContext,
) -> bool {
    let allocator = block.declarations.bump();
    let target = [top, right, bottom, left]
        .into_iter()
        .max()
        .expect("four box sides");
    match family {
        BoxFamily::Margin => {
            let Declaration::MarginTop(top_value) =
                std::mem::replace(&mut block.declarations[top], Declaration::Tombstone)
            else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginRight(right_value) =
                std::mem::replace(&mut block.declarations[right], Declaration::Tombstone)
            else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginBottom(bottom_value) =
                std::mem::replace(&mut block.declarations[bottom], Declaration::Tombstone)
            else {
                unreachable!("typed margin IR validates every side")
            };
            let Declaration::MarginLeft(left_value) =
                std::mem::replace(&mut block.declarations[left], Declaration::Tombstone)
            else {
                unreachable!("typed margin IR validates every side")
            };
            block.declarations[target] = Declaration::Margin(allocator.boxed(Margin {
                top: top_value,
                right: right_value,
                bottom: bottom_value,
                left: left_value,
            }));
        }
        BoxFamily::Padding => {
            let Declaration::PaddingTop(top_value) =
                std::mem::replace(&mut block.declarations[top], Declaration::Tombstone)
            else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingRight(right_value) =
                std::mem::replace(&mut block.declarations[right], Declaration::Tombstone)
            else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingBottom(bottom_value) =
                std::mem::replace(&mut block.declarations[bottom], Declaration::Tombstone)
            else {
                unreachable!("typed padding IR validates every side")
            };
            let Declaration::PaddingLeft(left_value) =
                std::mem::replace(&mut block.declarations[left], Declaration::Tombstone)
            else {
                unreachable!("typed padding IR validates every side")
            };
            block.declarations[target] = Declaration::Padding(allocator.boxed(Padding {
                top: top_value,
                right: right_value,
                bottom: bottom_value,
                left: left_value,
            }));
        }
    }
    record_merged_longhands(indices_from_sides(top, right, bottom, left), target, cx);
    true
}

fn merge_unparsed_box_longhands<'a>(
    block: &mut DeclarationBlock<'a>,
    indices: [usize; 4],
    family: BoxFamily,
    cx: &mut MinifyContext,
) -> bool {
    if indices.iter().any(|&index| {
        !matches!(&block.declarations[index], Declaration::Unparsed(value)
            if value.value.len() == 1 && is_box_component(&value.value[0], family))
    }) {
        return false;
    }
    let variable_count = indices
        .iter()
        .filter(|&&index| declaration_contains_variable(&block.declarations[index]))
        .count();
    if variable_count != 0 && variable_count != indices.len() {
        return false;
    }
    let first_value = match &block.declarations[indices[0]] {
        Declaration::Unparsed(value) => &value.value,
        _ => unreachable!(),
    };
    let all_equal = indices[1..].iter().all(|&index| {
        matches!(&block.declarations[index], Declaration::Unparsed(value)
            if value.value == *first_value)
    });
    if !all_equal
        && indices.iter().any(|&index| {
            matches!(&block.declarations[index], Declaration::Unparsed(value)
                if value.value.iter().any(is_css_wide_value))
        })
    {
        return false;
    }

    let allocator = block.declarations.bump();
    let mut sides = indices.map(|index| {
        let Declaration::Unparsed(value) = &mut block.declarations[index] else {
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
    let target = *indices.iter().max().expect("four box sides");
    let Declaration::Unparsed(target_value) = &mut block.declarations[target] else {
        unreachable!("unparsed box IR target remains unparsed")
    };
    *target_value.property_id = match family {
        BoxFamily::Margin => PropertyId::Margin,
        BoxFamily::Padding => PropertyId::Padding,
    };
    target_value.value = value;
    for &index in &indices {
        if index != target {
            block.declarations[index] = Declaration::Tombstone;
        }
    }
    record_merged_longhands(indices, target, cx);
    minify_unparsed_declaration(block, target, cx);
    true
}

#[inline]
const fn indices_from_sides(top: usize, right: usize, bottom: usize, left: usize) -> [usize; 4] {
    [top, right, bottom, left]
}

fn record_merged_longhands(indices: [usize; 4], target: usize, cx: &mut MinifyContext) {
    for index in indices {
        if index != target {
            cx.record_declaration_removed();
        }
    }
}

fn minify_unparsed_declaration(
    block: &mut DeclarationBlock<'_>,
    index: usize,
    cx: &mut MinifyContext,
) {
    let Declaration::Unparsed(value) = &mut block.declarations[index] else {
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

use crate::*;

use bitflags::bitflags;
use rocketcss_common::{DenseId, DenseMap, DenseRange, DenseStore, define_dense_id};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

#[derive(Debug, Default, PartialEq, Visit)]
pub struct DefaultAtRule;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Visit)]
#[visit(skip)]
pub enum CascadeOrigin {
    UserAgent,
    User,
    #[default]
    Author,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StyleSheet<'a> {
    pub license_comments: std::vec::Vec<&'a str>,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Compilation<'a> {
    stylesheet: StyleSheet<'a>,
    #[visit(skip)]
    string_pool: StringPool,
    #[visit(skip)]
    declaration_blocks: DeclarationBlockStore<'a>,
    #[visit(skip)]
    rules: RuleStore<'a>,
    #[visit(skip)]
    origin: CascadeOrigin,
}

impl<'a> Compilation<'a> {
    #[inline]
    pub fn new(
        stylesheet: StyleSheet<'a>,
        string_pool: StringPool,
        declaration_blocks: DeclarationBlockStore<'a>,
        rules: RuleStore<'a>,
        origin: CascadeOrigin,
    ) -> Self {
        Self {
            stylesheet,
            string_pool,
            declaration_blocks,
            rules,
            origin,
        }
    }

    #[inline]
    pub fn intern(&mut self, value: &str) -> Atom<'a> {
        self.string_pool.intern(value)
    }

    #[inline]
    pub fn intern_ascii_lowercase(&mut self, value: &str) -> Atom<'a> {
        self.string_pool.intern_ascii_lowercase(value)
    }

    #[inline]
    pub fn take_string_pool(&mut self) -> StringPool {
        std::mem::take(&mut self.string_pool)
    }

    #[inline]
    pub fn replace_string_pool(&mut self, string_pool: StringPool) -> StringPool {
        std::mem::replace(&mut self.string_pool, string_pool)
    }

    #[inline]
    pub fn parts(&self) -> (&StyleSheet<'a>, &DeclarationBlockStore<'a>) {
        (&self.stylesheet, &self.declaration_blocks)
    }

    #[inline]
    pub fn parts_mut(&mut self) -> (&mut StyleSheet<'a>, &mut DeclarationBlockStore<'a>) {
        (&mut self.stylesheet, &mut self.declaration_blocks)
    }

    #[inline]
    pub fn all_parts_mut(
        &mut self,
    ) -> (
        &mut StyleSheet<'a>,
        &mut DeclarationBlockStore<'a>,
        &mut RuleStore<'a>,
    ) {
        (
            &mut self.stylesheet,
            &mut self.declaration_blocks,
            &mut self.rules,
        )
    }

    #[inline]
    pub fn rule_store(&self) -> &RuleStore<'a> {
        &self.rules
    }

    #[inline]
    pub fn origin(&self) -> CascadeOrigin {
        self.origin
    }

    #[inline]
    pub fn rule_store_mut(&mut self) -> &mut RuleStore<'a> {
        &mut self.rules
    }

    #[inline]
    pub fn rule(&self, id: RuleId) -> &CssRule<'a> {
        self.rules.get(id)
    }

    #[inline]
    pub fn rule_topology(&self, id: RuleId) -> RuleTopology {
        self.rules.topology(id)
    }

    #[inline]
    pub fn rules(&self, list: RuleListId) -> RuleChildren<'_, 'a> {
        self.rules.children(list)
    }

    #[inline]
    pub fn rule_list(&self, list: RuleListId) -> RuleListRef<'_, 'a> {
        RuleListRef::new(&self.rules, list)
    }

    #[inline]
    pub fn root_rules(&self) -> RuleListRef<'_, 'a> {
        self.rule_list(self.stylesheet.rules)
    }

    #[inline]
    pub fn selectors(&self, list: SelectorListId) -> &[Selector<'a>] {
        self.rules.selectors(list)
    }

    #[inline]
    pub fn selectors_mut(&mut self, list: SelectorListId) -> &mut [Selector<'a>] {
        self.rules.selectors_mut(list)
    }

    #[inline]
    pub fn selector_slots(&self) -> impl ExactSizeIterator<Item = (SelectorId, &Selector<'a>)> {
        self.rules.selector_slots()
    }

    #[inline]
    pub fn selector_range(&self, list: SelectorListId) -> DenseRange<SelectorId> {
        self.rules.selector_range(list)
    }

    #[inline]
    pub fn declaration_block(&self, id: DeclarationBlockId) -> DeclarationBlockRef<'_, 'a> {
        self.declaration_blocks.view(id)
    }

    #[inline]
    pub fn declaration_block_mut(&mut self, id: DeclarationBlockId) -> &mut DeclarationBlock<'a> {
        self.declaration_blocks.block_mut(id)
    }

    #[inline]
    pub fn declaration(&self, id: DeclarationId) -> &Declaration<'a> {
        self.declaration_blocks.declaration(id)
    }

    #[inline]
    pub fn declaration_slots(
        &self,
    ) -> impl ExactSizeIterator<Item = (DeclarationId, &Declaration<'a>)> {
        self.declaration_blocks.declaration_slots()
    }

    /// Validates the physical declaration tape and block-range ownership.
    pub fn validate_flat_ir(&self) -> Result<(), &'static str> {
        self.declaration_blocks.validate()?;
        self.rules.validate()
    }

    pub fn visit<'ghost, V: ?Sized + Visitor<'a, 'ghost>>(
        &self,
        visitor: &mut V,
        cx: &VisitContext<'_, 'a, 'ghost>,
    ) {
        let cx = VisitContext::new_with_stores(cx.token(), &self.declaration_blocks, &self.rules);
        Visit::visit(&self.stylesheet, visitor, &cx);
    }

    pub fn visit_mut<'ghost, V: ?Sized + VisitorMut<'a, 'ghost>>(
        &mut self,
        visitor: &mut V,
        cx: &mut VisitMutContext<'_, 'a, 'ghost>,
    ) {
        let stylesheet = &mut self.stylesheet;
        let declaration_blocks = &mut self.declaration_blocks;
        let rules = &mut self.rules;
        cx.with_stores(declaration_blocks, rules, |cx| {
            <StyleSheet<'a> as VisitMut<'a, 'ghost>>::visit_mut(stylesheet, visitor, cx);
        });
    }
}

impl<'a> std::ops::Deref for Compilation<'a> {
    type Target = StyleSheet<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.stylesheet
    }
}

impl std::ops::DerefMut for Compilation<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stylesheet
    }
}

define_dense_id!(pub struct DeclarationBlockId);
define_dense_id!(pub struct DeclarationId);
define_dense_id!(pub struct EffectiveKeyId);

#[derive(Visit)]
pub struct DeclarationBlockStore<'a> {
    #[visit(skip)]
    blocks: DenseStore<DeclarationBlockId, DeclarationBlock<'a>>,
    #[visit(skip)]
    declarations: DenseStore<DeclarationId, Declaration<'a>>,
    #[visit(skip)]
    importance: std::vec::Vec<bool>,
}

impl<'a> DeclarationBlockStore<'a> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            blocks: DenseStore::new(),
            declarations: DenseStore::new(),
            importance: std::vec::Vec::new(),
        }
    }

    #[inline]
    pub fn begin_block(&mut self) -> DeclarationBlockId {
        self.blocks
            .push(DeclarationBlock::new(self.declarations.cursor()))
    }

    #[inline]
    pub fn push(&mut self, block: DeclarationBlock<'a>) -> DeclarationBlockId {
        self.blocks.push(block)
    }

    #[inline]
    pub fn block(&self, id: DeclarationBlockId) -> &DeclarationBlock<'a> {
        self.blocks.get(id)
    }

    #[inline]
    pub fn block_mut(&mut self, id: DeclarationBlockId) -> &mut DeclarationBlock<'a> {
        self.blocks.get_mut(id)
    }

    #[inline]
    pub fn set_effective_key(&mut self, id: DeclarationBlockId, key: EffectiveKeyId) {
        self.blocks.get_mut(id).set_effective_key(key);
    }

    #[inline]
    pub fn view(&self, id: DeclarationBlockId) -> DeclarationBlockRef<'_, 'a> {
        DeclarationBlockRef {
            block: self.block(id),
            declarations: DeclarationValues {
                block: self.block(id),
                declarations: &self.declarations,
            },
            declarations_importance: DeclarationImportance {
                block: self.block(id),
                importance: &self.importance,
            },
        }
    }

    /// Moves the declaration sequence owned by `previous` in front of
    /// `current` without widening either sequence across foreign live slots.
    pub fn prepend_block(&mut self, current: DeclarationBlockId, previous: DeclarationBlockId) {
        let (current, previous) = self
            .blocks
            .get_two_mut(current, previous)
            .expect("a declaration block cannot be merged with itself");
        let previous_cursor = previous
            .ranges
            .first()
            .copied()
            .expect("a declaration block has an initial range");
        let mut ranges = std::mem::take(&mut previous.ranges);
        ranges.append(&mut current.ranges);
        current.ranges = ranges;
        previous.ranges = std::vec![
            DenseRange::from_bounds(previous_cursor.offset(), 0)
                .expect("an existing declaration offset fits the dense ID domain")
        ];
    }

    /// Retires the declaration occurrence owned by a rule that will not be
    /// copied into the committed rule tape.
    pub(crate) fn retire_block(&mut self, id: DeclarationBlockId) {
        self.blocks[id].ranges.clear();
    }

    #[inline]
    pub fn get(&self, id: DeclarationBlockId) -> &DeclarationBlock<'a> {
        self.block(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: DeclarationBlockId) -> &mut DeclarationBlock<'a> {
        self.block_mut(id)
    }

    pub fn get_two_mut(
        &mut self,
        left: DeclarationBlockId,
        right: DeclarationBlockId,
    ) -> Option<(&mut DeclarationBlock<'a>, &mut DeclarationBlock<'a>)> {
        self.blocks.get_two_mut(left, right)
    }

    #[inline]
    pub fn ids(&self) -> impl ExactSizeIterator<Item = DeclarationBlockId> + '_ {
        self.blocks.ids()
    }

    #[inline]
    pub fn map<T>(
        &self,
        init: impl FnMut(DeclarationBlockId) -> T,
    ) -> DenseMap<DeclarationBlockId, T> {
        DenseMap::from_store(&self.blocks, init)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    #[inline]
    pub fn declaration(&self, id: DeclarationId) -> &Declaration<'a> {
        self.declarations.get(id)
    }

    #[inline]
    pub fn declaration_mut(&mut self, id: DeclarationId) -> &mut Declaration<'a> {
        self.declarations.get_mut(id)
    }

    #[inline]
    pub fn declaration_slots(
        &self,
    ) -> impl ExactSizeIterator<Item = (DeclarationId, &Declaration<'a>)> {
        self.declarations.iter_enumerated()
    }

    #[inline]
    pub fn push_declaration(
        &mut self,
        block: DeclarationBlockId,
        declaration: Declaration<'a>,
        important: bool,
    ) -> DeclarationId {
        let id = self.declarations.push(declaration);
        self.importance.push(important);
        self.blocks
            .get_mut(block)
            .append_id(id)
            .expect("declaration IDs are appended in source order");
        id
    }

    #[inline]
    pub fn is_important(&self, id: DeclarationId) -> bool {
        self.importance[id.index()]
    }

    #[inline]
    pub fn declaration_count(&self, block: DeclarationBlockId) -> usize {
        self.block(block).len()
    }

    pub fn declaration_id_at(&self, block: DeclarationBlockId, mut index: usize) -> DeclarationId {
        for range in self.block(block).ranges() {
            if index < range.len() {
                return DeclarationId::from_index(range.offset() + index)
                    .expect("a block range contains valid declaration IDs");
            }
            index -= range.len();
        }
        panic!("declaration index is outside its block")
    }

    #[inline]
    pub fn block_declaration(&self, block: DeclarationBlockId, index: usize) -> &Declaration<'a> {
        self.declaration(self.declaration_id_at(block, index))
    }

    #[inline]
    pub fn block_declaration_mut(
        &mut self,
        block: DeclarationBlockId,
        index: usize,
    ) -> &mut Declaration<'a> {
        let id = self.declaration_id_at(block, index);
        self.declaration_mut(id)
    }

    #[inline]
    pub fn block_is_important(&self, block: DeclarationBlockId, index: usize) -> bool {
        self.is_important(self.declaration_id_at(block, index))
    }

    pub fn block_iter(
        &self,
        block: DeclarationBlockId,
    ) -> impl DoubleEndedIterator<Item = (&Declaration<'a>, bool)> {
        self.block(block).ranges().iter().flat_map(|range| {
            let values = self.declarations.get_range(*range);
            let importance = &self.importance[range.as_usize_range()];
            values.iter().zip(importance.iter().copied())
        })
    }

    #[inline]
    pub fn block_iter_live(
        &self,
        block: DeclarationBlockId,
    ) -> impl DoubleEndedIterator<Item = (&Declaration<'a>, bool)> {
        self.block_iter(block)
            .filter(|(declaration, _)| !declaration.is_tombstone())
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.declarations.len() != self.importance.len() {
            return Err("declaration importance sidecar length mismatch");
        }
        let mut owners = std::vec![None; self.declarations.len()];
        for (block_id, block) in self.blocks.iter_enumerated() {
            let mut previous_end = None;
            for range in block.ranges() {
                if range.end() > self.declarations.len() {
                    return Err("declaration block range exceeds the declaration tape");
                }
                if let Some(end) = previous_end
                    && range.offset() < end
                {
                    return Err("declaration block ranges are not in physical order");
                }
                previous_end = Some(range.end());
                for owner in &mut owners[range.as_usize_range()] {
                    if owner.replace(block_id).is_some() {
                        return Err("a declaration slot has multiple block owners");
                    }
                }
            }
        }
        if owners.iter().any(Option::is_none) {
            return Err("a declaration slot has no block owner");
        }
        Ok(())
    }

    /// Rebuilds the declaration tape in final semantic block order and drops
    /// tombstones left by local and cross-rule minification.
    pub fn compact(&mut self) {
        let mut source = std::mem::take(&mut self.declarations);
        let source_importance = std::mem::take(&mut self.importance);
        let mut declarations = DenseStore::new();
        let mut importance = std::vec::Vec::new();

        for block in self.blocks.iter_mut() {
            let cursor = declarations.cursor();
            for range in std::mem::take(&mut block.ranges) {
                for index in range.as_usize_range() {
                    let id = DeclarationId::from_index(index)
                        .expect("a declaration range contains valid dense IDs");
                    let declaration = std::mem::replace(source.get_mut(id), Declaration::Tombstone);
                    if declaration.is_tombstone() {
                        continue;
                    }
                    declarations.push(declaration);
                    importance.push(source_importance[index]);
                }
            }
            block.ranges = std::vec![declarations.range_since(cursor)];
        }

        self.declarations = declarations;
        self.importance = importance;
        debug_assert!(
            self.declarations
                .iter()
                .all(|declaration| !declaration.is_tombstone())
        );
        debug_assert!(self.validate().is_ok());
    }
}

impl Default for DeclarationBlockStore<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for DeclarationBlockStore<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeclarationBlockStore")
            .field("blocks", &self.blocks)
            .field("declarations", &self.declarations)
            .field("importance", &self.importance)
            .finish()
    }
}

impl PartialEq for DeclarationBlockStore<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.blocks == other.blocks
            && self.declarations == other.declarations
            && self.importance == other.importance
    }
}

#[derive(Clone, Copy, Visit)]
#[visit(skip)]
pub struct DeclarationBlockRef<'store, 'ast> {
    #[visit(skip)]
    block: &'store DeclarationBlock<'ast>,
    #[visit(skip)]
    pub declarations: DeclarationValues<'store, 'ast>,
    #[visit(skip)]
    pub declarations_importance: DeclarationImportance<'store, 'ast>,
}

impl<'store, 'ast> DeclarationBlockRef<'store, 'ast> {
    #[inline]
    pub fn effective_key(self) -> Option<EffectiveKeyId> {
        self.block.effective_key()
    }

    #[inline]
    pub fn ranges(self) -> &'store [DenseRange<DeclarationId>] {
        self.block.ranges()
    }

    #[inline]
    pub fn len(self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.block.is_empty()
    }

    #[inline]
    pub fn is_output_empty(self) -> bool {
        self.declarations.iter().all(Declaration::is_tombstone)
    }

    #[inline]
    pub fn is_important(self, index: usize) -> bool {
        self.declarations_importance.is_set(index)
    }

    #[inline]
    pub fn iter(self) -> impl DoubleEndedIterator<Item = (&'store Declaration<'ast>, bool)> {
        self.declarations
            .iter()
            .zip(self.declarations_importance.iter())
    }

    #[inline]
    pub fn iter_live(self) -> impl DoubleEndedIterator<Item = (&'store Declaration<'ast>, bool)> {
        self.iter()
            .filter(|(declaration, _)| !declaration.is_tombstone())
    }
}

impl EqIgnoringTombstones for DeclarationBlockRef<'_, '_> {
    fn eq_ignoring_tombstones(&self, other: &Self) -> bool {
        let mut left = self.iter_live();
        let mut right = other.iter_live();
        loop {
            match (left.next(), right.next()) {
                (None, None) => return true,
                (Some((left, left_important)), Some((right, right_important)))
                    if left_important == right_important && left.eq_ignoring_tombstones(right) => {}
                _ => return false,
            }
        }
    }
}

#[derive(Clone, Copy, Visit)]
#[visit(skip)]
pub struct DeclarationValues<'store, 'ast> {
    #[visit(skip)]
    block: &'store DeclarationBlock<'ast>,
    #[visit(skip)]
    declarations: &'store DenseStore<DeclarationId, Declaration<'ast>>,
}

impl<'store, 'ast> DeclarationValues<'store, 'ast> {
    #[inline]
    pub fn len(self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.block.is_empty()
    }

    pub fn iter(self) -> DeclarationValueIter<'store, 'ast> {
        DeclarationValueIter {
            values: self,
            front: 0,
            back: self.len(),
        }
    }

    pub fn get(self, mut index: usize) -> &'store Declaration<'ast> {
        for range in self.block.ranges() {
            if index < range.len() {
                let id = DeclarationId::from_index(range.offset() + index)
                    .expect("a block range contains valid declaration IDs");
                return self.declarations.get(id);
            }
            index -= range.len();
        }
        panic!("declaration index is outside its block")
    }

    fn contiguous_slice(self, range: Range<usize>) -> &'store [Declaration<'ast>] {
        assert!(range.start <= range.end && range.end <= self.len());
        let mut logical_offset = 0;
        for physical in self.block.ranges() {
            let logical_end = logical_offset + physical.len();
            if range.start >= logical_offset && range.end <= logical_end {
                let start = physical.offset() + range.start - logical_offset;
                let end = physical.offset() + range.end - logical_offset;
                return self.declarations.get_range(
                    DenseRange::from_bounds(start, end - start)
                        .expect("a declaration subrange fits its ID domain"),
                );
            }
            logical_offset = logical_end;
        }
        panic!("a slice spanning non-contiguous declaration runs is unavailable")
    }
}

impl<'store, 'ast> IntoIterator for &DeclarationValues<'store, 'ast> {
    type Item = &'store Declaration<'ast>;
    type IntoIter = DeclarationValueIter<'store, 'ast>;

    fn into_iter(self) -> Self::IntoIter {
        (*self).iter()
    }
}

#[derive(Visit)]
#[visit(skip)]
pub struct DeclarationValueIter<'store, 'ast> {
    #[visit(skip)]
    values: DeclarationValues<'store, 'ast>,
    #[visit(skip)]
    front: usize,
    #[visit(skip)]
    back: usize,
}

impl<'store, 'ast> Iterator for DeclarationValueIter<'store, 'ast> {
    type Item = &'store Declaration<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        Some(self.values.get(index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for DeclarationValueIter<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        Some(self.values.get(self.back))
    }
}

impl ExactSizeIterator for DeclarationValueIter<'_, '_> {}

impl<'ast> Index<usize> for DeclarationValues<'_, 'ast> {
    type Output = Declaration<'ast>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.contiguous_slice(index..index + 1)[0]
    }
}

impl<'ast> Index<Range<usize>> for DeclarationValues<'_, 'ast> {
    type Output = [Declaration<'ast>];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        self.contiguous_slice(range)
    }
}

impl<'ast> Index<RangeFrom<usize>> for DeclarationValues<'_, 'ast> {
    type Output = [Declaration<'ast>];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        self.contiguous_slice(range.start..self.len())
    }
}

impl<'ast> Index<RangeTo<usize>> for DeclarationValues<'_, 'ast> {
    type Output = [Declaration<'ast>];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        self.contiguous_slice(0..range.end)
    }
}

impl<'ast> Index<RangeFull> for DeclarationValues<'_, 'ast> {
    type Output = [Declaration<'ast>];

    fn index(&self, _: RangeFull) -> &Self::Output {
        self.contiguous_slice(0..self.len())
    }
}

#[derive(Clone, Copy, Visit)]
#[visit(skip)]
pub struct DeclarationImportance<'store, 'ast> {
    #[visit(skip)]
    block: &'store DeclarationBlock<'ast>,
    #[visit(skip)]
    importance: &'store [bool],
}

impl<'store, 'ast> DeclarationImportance<'store, 'ast> {
    #[inline]
    pub fn len(self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.block.is_empty()
    }

    pub fn is_set(self, mut index: usize) -> bool {
        for range in self.block.ranges() {
            if index < range.len() {
                return self.importance[range.offset() + index];
            }
            index -= range.len();
        }
        panic!("declaration importance index is outside its block")
    }

    pub fn iter(self) -> DeclarationImportanceIter<'store, 'ast> {
        DeclarationImportanceIter {
            importance: self,
            front: 0,
            back: self.len(),
        }
    }
}

#[derive(Visit)]
#[visit(skip)]
pub struct DeclarationImportanceIter<'store, 'ast> {
    #[visit(skip)]
    importance: DeclarationImportance<'store, 'ast>,
    #[visit(skip)]
    front: usize,
    #[visit(skip)]
    back: usize,
}

impl Iterator for DeclarationImportanceIter<'_, '_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        Some(self.importance.is_set(index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back - self.front;
        (len, Some(len))
    }
}

impl DoubleEndedIterator for DeclarationImportanceIter<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        Some(self.importance.is_set(self.back))
    }
}

impl ExactSizeIterator for DeclarationImportanceIter<'_, '_> {}

pub fn visit_declaration_block_id<'a, 'ghost, V: ?Sized + Visitor<'a, 'ghost>>(
    id: &DeclarationBlockId,
    visitor: &mut V,
    cx: &VisitContext<'_, 'a, 'ghost>,
) {
    cx.visit_declaration_block(*id, visitor);
}

pub fn visit_declaration_block_id_mut<'a, 'ghost, V: ?Sized + VisitorMut<'a, 'ghost>>(
    id: &mut DeclarationBlockId,
    visitor: &mut V,
    cx: &mut VisitMutContext<'_, 'a, 'ghost>,
) {
    cx.visit_declaration_block(*id, visitor);
}

pub fn visit_rule_list_id<'a, 'ghost, V: ?Sized + Visitor<'a, 'ghost>>(
    id: &RuleListId,
    visitor: &mut V,
    cx: &VisitContext<'_, 'a, 'ghost>,
) {
    cx.visit_rule_list(*id, visitor);
}

pub fn visit_rule_list_id_mut<'a, 'ghost, V: ?Sized + VisitorMut<'a, 'ghost>>(
    id: &mut RuleListId,
    visitor: &mut V,
    cx: &mut VisitMutContext<'_, 'a, 'ghost>,
) {
    cx.visit_rule_list(*id, visitor);
}

pub fn visit_selector_list_id<'a, 'ghost, V: ?Sized + Visitor<'a, 'ghost>>(
    id: &SelectorListId,
    visitor: &mut V,
    cx: &VisitContext<'_, 'a, 'ghost>,
) {
    cx.visit_selector_list(*id, visitor);
}

pub fn visit_selector_list_id_mut<'a, 'ghost, V: ?Sized + VisitorMut<'a, 'ghost>>(
    id: &mut SelectorListId,
    visitor: &mut V,
    cx: &mut VisitMutContext<'_, 'a, 'ghost>,
) {
    cx.visit_selector_list(*id, visitor);
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaRule<'a> {
    pub span: Span,
    pub query: MediaList<'a>,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaList<'a> {
    pub media_queries: std::vec::Vec<std::boxed::Box<MediaQuery<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaQuery<'a> {
    pub condition: Option<MediaCondition<'a>>,
    pub media_type: MediaType<'a>,
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LengthValue {
    pub unit: LengthUnit,
    pub value: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct EnvironmentVariable<'a> {
    pub fallback: Option<std::vec::Vec<TokenOrValue<'a>>>,
    pub indices: std::vec::Vec<i32>,
    pub name: EnvironmentVariableName<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Url<'a> {
    pub span: Span,
    pub url: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Variable<'a> {
    pub fallback: Option<std::vec::Vec<TokenOrValue<'a>>>,
    pub name: std::boxed::Box<DashedIdentReference<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DashedIdentReference<'a> {
    pub from: Option<Specifier<'a>>,
    pub ident: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Function<'a> {
    pub arguments: std::vec::Vec<TokenOrValue<'a>>,
    #[visit(skip)]
    flags: FunctionFlags,
    #[visit(skip)]
    kind: KnownFunction,
    name: Atom<'a>,
    /// A simple value serialized from this existing function node.
    pub replacement: Option<FunctionReplacement>,
}

/// A function name recognized by RocketCSS.
///
/// The original function name remains on [`Function`] so parsing and code
/// generation stay lossless. This enum gives downstream passes a shared,
/// ASCII case-insensitive identity without repeating string matching.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Visit)]
#[repr(u8)]
pub enum KnownFunction {
    Abs,
    Calc,
    Clamp,
    Color,
    ColorMix,
    Constant,
    ConicGradient,
    CubicBezier,
    Env,
    Frames,
    Hsl,
    Hsla,
    Hwb,
    Hypot,
    Lab,
    Lch,
    Linear,
    LinearGradient,
    Local,
    Matrix,
    Matrix3d,
    Max,
    Min,
    Mod,
    RadialGradient,
    Rem,
    RepeatingConicGradient,
    RepeatingLinearGradient,
    RepeatingRadialGradient,
    Rgb,
    Rgba,
    Rotate,
    RotateX,
    RotateY,
    Rotate3d,
    RotateZ,
    Round,
    Scale,
    ScaleX,
    ScaleY,
    ScaleZ,
    Scale3d,
    Sign,
    Steps,
    Translate,
    TranslateY,
    TranslateZ,
    Translate3d,
    Url,
    Var,
    #[default]
    Unknown,
}

impl KnownFunction {
    /// Resolves a function name using CSS ASCII case-insensitive matching.
    pub fn from_name(name: &str) -> Self {
        Self::classify(name).0
    }

    fn classify(name: &str) -> (Self, bool) {
        let kind = Self::from_unprefixed_name(name);
        if kind != Self::Unknown {
            return (kind, false);
        }

        let unprefixed_name = name
            .strip_prefix('-')
            .and_then(|name| name.split_once('-').map(|(_, name)| name));
        let Some(unprefixed_name) = unprefixed_name else {
            return (Self::Unknown, false);
        };
        let kind = Self::from_unprefixed_name(unprefixed_name);
        if kind.is_math() || kind.is_gradient() {
            (kind, true)
        } else {
            (Self::Unknown, false)
        }
    }

    fn from_unprefixed_name(name: &str) -> Self {
        match_ignore_ascii_case!(
            name,
            "abs" => Self::Abs,
            "calc" => Self::Calc,
            "clamp" => Self::Clamp,
            "color" => Self::Color,
            "color-mix" => Self::ColorMix,
            "constant" => Self::Constant,
            "conic-gradient" => Self::ConicGradient,
            "cubic-bezier" => Self::CubicBezier,
            "env" => Self::Env,
            "frames" => Self::Frames,
            "hsl" => Self::Hsl,
            "hsla" => Self::Hsla,
            "hwb" => Self::Hwb,
            "hypot" => Self::Hypot,
            "lab" => Self::Lab,
            "lch" => Self::Lch,
            "linear" => Self::Linear,
            "linear-gradient" => Self::LinearGradient,
            "local" => Self::Local,
            "matrix" => Self::Matrix,
            "matrix3d" => Self::Matrix3d,
            "max" => Self::Max,
            "min" => Self::Min,
            "mod" => Self::Mod,
            "radial-gradient" => Self::RadialGradient,
            "rem" => Self::Rem,
            "repeating-conic-gradient" => Self::RepeatingConicGradient,
            "repeating-linear-gradient" => Self::RepeatingLinearGradient,
            "repeating-radial-gradient" => Self::RepeatingRadialGradient,
            "rgb" => Self::Rgb,
            "rgba" => Self::Rgba,
            "rotate" => Self::Rotate,
            "rotatex" => Self::RotateX,
            "rotatey" => Self::RotateY,
            "rotate3d" => Self::Rotate3d,
            "rotatez" => Self::RotateZ,
            "round" => Self::Round,
            "scale" => Self::Scale,
            "scalex" => Self::ScaleX,
            "scaley" => Self::ScaleY,
            "scalez" => Self::ScaleZ,
            "scale3d" => Self::Scale3d,
            "sign" => Self::Sign,
            "steps" => Self::Steps,
            "translate" => Self::Translate,
            "translatey" => Self::TranslateY,
            "translatez" => Self::TranslateZ,
            "translate3d" => Self::Translate3d,
            "url" => Self::Url,
            "var" => Self::Var,
            _ => Self::Unknown,
        )
    }

    /// Returns whether this function participates in math value parsing.
    pub const fn is_math(self) -> bool {
        matches!(
            self,
            Self::Abs
                | Self::Calc
                | Self::Clamp
                | Self::Hypot
                | Self::Max
                | Self::Min
                | Self::Mod
                | Self::Rem
                | Self::Round
                | Self::Sign
        )
    }

    /// Returns whether this function is accepted as a basic calculated value.
    pub const fn is_math_value(self) -> bool {
        matches!(self, Self::Calc | Self::Min | Self::Max | Self::Clamp)
    }

    /// Returns whether this is a gradient function.
    pub const fn is_gradient(self) -> bool {
        matches!(
            self,
            Self::LinearGradient
                | Self::RepeatingLinearGradient
                | Self::RadialGradient
                | Self::RepeatingRadialGradient
                | Self::ConicGradient
                | Self::RepeatingConicGradient
        )
    }

    /// Returns whether this function resolves a variable or environment value.
    pub const fn is_variable(self) -> bool {
        matches!(self, Self::Var | Self::Env | Self::Constant)
    }

    /// Returns whether this is a color function handled by the minifier.
    pub const fn is_color(self) -> bool {
        matches!(
            self,
            Self::Rgb
                | Self::Rgba
                | Self::Hsl
                | Self::Hsla
                | Self::Hwb
                | Self::Lab
                | Self::Lch
                | Self::Color
        )
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct FunctionFlags: u8 {
        /// This node was reduced to an identifier during minification.
        ///
        /// Keeping the replacement in the existing function allocation avoids
        /// allocating a new token solely to change the surrounding enum variant.
        const IS_IDENTIFIER = 1 << 0;
        /// Emit a quoted `url()` argument directly when it is safe to unquote.
        const UNQUOTED_URL = 1 << 1;
        /// The known identity was resolved after removing a vendor prefix.
        const VENDOR_PREFIXED = 1 << 2;
        /// The parser proved that this `rgb()` or `rgba()` token list is a
        /// statically valid form supported by the color minifier.
        const VALID_RGB = 1 << 3;
    }
}

impl<'a> Function<'a> {
    /// Creates a function with no minifier serialization state.
    #[inline]
    pub fn new(name: Atom<'a>, arguments: std::vec::Vec<TokenOrValue<'a>>) -> Self {
        let (kind, vendor_prefixed) = KnownFunction::classify(&name);
        let mut flags = FunctionFlags::empty();
        flags.set(FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
        Self {
            arguments,
            flags,
            kind,
            name,
            replacement: None,
        }
    }

    /// Returns the original function name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the shared identity for a recognized function name.
    #[inline]
    pub const fn kind(&self) -> KnownFunction {
        self.kind
    }

    /// Updates the lossless function name and its recognized identity together.
    #[inline]
    pub fn set_name(&mut self, name: Atom<'a>) {
        let (kind, vendor_prefixed) = KnownFunction::classify(&name);
        self.name = name;
        self.kind = kind;
        self.flags
            .set(FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
        self.flags.remove(FunctionFlags::VALID_RGB);
    }

    /// Returns whether the known identity came from a vendor-prefixed name.
    #[inline]
    pub const fn is_vendor_prefixed(&self) -> bool {
        self.flags.contains(FunctionFlags::VENDOR_PREFIXED)
    }

    /// Returns whether this `rgb()` or `rgba()` token list was validated by
    /// the parser and can be consumed by the color minifier.
    #[inline]
    pub const fn is_valid_rgb(&self) -> bool {
        self.flags.contains(FunctionFlags::VALID_RGB)
    }

    /// Records the parser's validation result for an `rgb()` or `rgba()`
    /// function without changing its lossless token representation.
    #[inline]
    pub fn set_valid_rgb(&mut self, valid: bool) {
        self.flags.set(FunctionFlags::VALID_RGB, valid);
    }

    /// Returns whether this function serializes as an identifier.
    #[inline]
    pub const fn is_identifier(&self) -> bool {
        self.flags.contains(FunctionFlags::IS_IDENTIFIER)
    }

    /// Controls whether this function serializes as an identifier.
    #[inline]
    pub fn set_identifier(&mut self, is_identifier: bool) {
        self.flags.set(FunctionFlags::IS_IDENTIFIER, is_identifier);
    }

    /// Returns whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub const fn is_unquoted_url(&self) -> bool {
        self.flags.contains(FunctionFlags::UNQUOTED_URL)
    }

    /// Controls whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub fn set_unquoted_url(&mut self, unquoted_url: bool) {
        self.flags.set(FunctionFlags::UNQUOTED_URL, unquoted_url);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum FunctionReplacement {
    GrayAlpha {
        alpha: f32,
        lightness: f32,
    },
    Number(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    Percentage(f32),
    Rgb {
        blue: u8,
        green: u8,
        red: u8,
    },
    Rgba {
        alpha: f32,
        blue: u8,
        green: u8,
        red: u8,
        use_hex: bool,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImportRule<'a> {
    pub layer: Option<std::vec::Vec<Atom<'a>>>,
    pub span: Span,
    pub media: Option<std::boxed::Box<MediaList<'a>>>,
    pub supports: Option<std::boxed::Box<SupportsCondition<'a>>>,
    pub url: Atom<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StyleRule<'a> {
    #[visit(
        with = visit_declaration_block_id,
        with_mut = visit_declaration_block_id_mut
    )]
    pub declarations: DeclarationBlockId,
    pub span: Span,
    #[visit(with = visit_rule_list_id, with_mut = visit_rule_list_id_mut)]
    pub rules: RuleListId,
    #[visit(with = visit_selector_list_id, with_mut = visit_selector_list_id_mut)]
    pub selectors: SelectorListId,
    pub vendor_prefix: VendorPrefix,
    #[visit(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> StyleRule<'a> {
    #[inline]
    pub fn new(
        declarations: DeclarationBlockId,
        span: Span,
        rules: RuleListId,
        selectors: SelectorListId,
        vendor_prefix: VendorPrefix,
    ) -> Self {
        Self {
            declarations,
            span,
            rules,
            selectors,
            vendor_prefix,
            marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Visit)]
pub struct DeclarationBlock<'a> {
    #[visit(skip)]
    ranges: std::vec::Vec<DenseRange<DeclarationId>>,
    #[visit(skip)]
    effective_key: Option<EffectiveKeyId>,
    #[visit(skip)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> DeclarationBlock<'a> {
    #[inline]
    pub fn new(cursor: DenseRange<DeclarationId>) -> Self {
        debug_assert!(cursor.is_empty());
        Self {
            ranges: std::vec![cursor],
            effective_key: None,
            marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn ranges(&self) -> &[DenseRange<DeclarationId>] {
        &self.ranges
    }

    #[inline]
    pub fn effective_key(&self) -> Option<EffectiveKeyId> {
        self.effective_key
    }

    #[inline]
    pub(crate) fn set_effective_key(&mut self, key: EffectiveKeyId) {
        self.effective_key = Some(key);
    }

    fn append_id(&mut self, id: DeclarationId) -> Result<(), ()> {
        let one = DenseRange::from_bounds(id.index(), 1).map_err(|_| ())?;
        let last = self
            .ranges
            .last_mut()
            .expect("a block has an initial cursor");
        if let Some(joined) = last.try_join_adjacent(one) {
            *last = joined;
        } else {
            self.ranges.push(one);
        }
        Ok(())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ranges.iter().map(|range| range.len()).sum()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl PartialEq for DeclarationBlock<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges && self.effective_key == other.effective_key
    }
}

impl EqIgnoringTombstones for DeclarationBlock<'_> {
    fn eq_ignoring_tombstones(&self, other: &Self) -> bool {
        self == other
    }
}

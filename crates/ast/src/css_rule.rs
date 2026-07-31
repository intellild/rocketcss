use super::*;

use rocketcss_common::{DenseMap, DenseStore, define_dense_id};
use std::ops::Index;

define_dense_id!(pub struct RuleId);
define_dense_id!(pub struct RuleListId);
define_dense_id!(pub struct RulePayloadId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub struct RuleTopology {
    #[visit(skip)]
    pub parent: Option<RuleId>,
    #[visit(skip)]
    pub list: RuleListId,
    #[visit(skip)]
    pub next_sibling: Option<RuleId>,
    #[visit(skip)]
    pub subtree_end: u32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct SyntaxNode {
    #[visit(skip)]
    payload: Option<RulePayloadId>,
    #[visit(skip)]
    topology: RuleTopology,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RuleList {
    parent: Option<RuleId>,
    first: Option<RuleId>,
    last: Option<RuleId>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct RuleStore<'a> {
    #[visit(skip)]
    nodes: DenseStore<RuleId, SyntaxNode>,
    #[visit(skip)]
    payloads: DenseStore<RulePayloadId, Option<CssRule<'a>>>,
    #[visit(skip)]
    selectors: SelectorStore<'a>,
    #[visit(skip)]
    lists: DenseStore<RuleListId, RuleList>,
}

impl<'a> RuleStore<'a> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            nodes: DenseStore::new(),
            payloads: DenseStore::new(),
            selectors: SelectorStore::new(),
            lists: DenseStore::new(),
        }
    }

    #[inline]
    pub fn begin_list(&mut self, parent: Option<RuleId>) -> RuleListId {
        self.lists.push(RuleList {
            parent,
            first: None,
            last: None,
        })
    }

    #[inline]
    pub fn push_selector_list(&mut self, selectors: SelectorList<'a>) -> SelectorListId {
        self.selectors.push_list(selectors)
    }

    #[inline]
    pub fn selectors(&self, list: SelectorListId) -> &[Selector<'a>] {
        self.selectors.get(list)
    }

    #[inline]
    pub fn selectors_mut(&mut self, list: SelectorListId) -> &mut [Selector<'a>] {
        self.selectors.get_mut(list)
    }

    #[inline]
    pub fn selector_slots(&self) -> impl ExactSizeIterator<Item = (SelectorId, &Selector<'a>)> {
        self.selectors.slots()
    }

    #[inline]
    pub fn selector_range(&self, list: SelectorListId) -> DenseRange<SelectorId> {
        self.selectors.range(list)
    }

    pub fn reserve(&mut self, list: RuleListId) -> RuleId {
        let parent = self.lists[list].parent;
        let payload = self.payloads.push(None);
        let id = self.nodes.push(SyntaxNode {
            payload: Some(payload),
            topology: RuleTopology {
                parent,
                list,
                next_sibling: None,
                subtree_end: 0,
            },
        });
        if let Some(previous) = self.lists[list].last {
            self.nodes[previous].topology.next_sibling = Some(id);
        } else {
            self.lists[list].first = Some(id);
        }
        self.lists[list].last = Some(id);
        id
    }

    #[inline]
    pub fn finish(&mut self, id: RuleId, rule: CssRule<'a>) {
        let payload = self.payload_id(id);
        assert!(self.payloads[payload].is_none(), "a rule is finalized once");
        self.payloads[payload] = Some(rule);
        self.nodes[id].topology.subtree_end = u32::try_from(self.nodes.len())
            .expect("rule count exceeds the flat u32 topology domain");
    }

    #[inline]
    pub fn get(&self, id: RuleId) -> &CssRule<'a> {
        self.payloads[self.payload_id(id)]
            .as_ref()
            .expect("a reserved rule must be finalized before observation")
    }

    #[inline]
    pub fn get_mut(&mut self, id: RuleId) -> &mut CssRule<'a> {
        let payload = self.payload_id(id);
        self.payloads[payload]
            .as_mut()
            .expect("a reserved rule must be finalized before mutation")
    }

    pub fn get_two_mut(
        &mut self,
        left: RuleId,
        right: RuleId,
    ) -> Option<(&mut CssRule<'a>, &mut CssRule<'a>)> {
        let left = self.payload_id(left);
        let right = self.payload_id(right);
        self.payloads
            .get_two_mut(left, right)
            .and_then(|(left, right)| Some((left.as_mut()?, right.as_mut()?)))
    }

    #[inline]
    pub fn node(&self, id: RuleId) -> &SyntaxNode {
        &self.nodes[id]
    }

    #[inline]
    pub fn payload_id(&self, id: RuleId) -> RulePayloadId {
        self.nodes[id]
            .payload
            .expect("a reserved rule must be finalized before observation")
    }

    #[inline]
    pub fn topology(&self, id: RuleId) -> RuleTopology {
        self.nodes[id].topology
    }

    #[inline]
    pub fn ids(&self) -> impl ExactSizeIterator<Item = RuleId> + '_ {
        self.nodes.ids()
    }

    #[inline]
    pub fn map<T>(&self, init: impl FnMut(RuleId) -> T) -> DenseMap<RuleId, T> {
        DenseMap::from_store(&self.nodes, init)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[inline]
    pub fn list_is_empty(&self, list: RuleListId) -> bool {
        self.lists[list].first.is_none()
    }

    #[inline]
    pub fn list_len(&self, list: RuleListId) -> usize {
        self.children(list).count()
    }

    #[inline]
    pub fn children(&self, list: RuleListId) -> RuleChildren<'_, 'a> {
        RuleChildren {
            store: self,
            next: self.lists[list].first,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.nodes.len() != self.payloads.len() {
            return Err("rule topology and payload tape lengths differ");
        }
        self.selectors.validate()?;
        for (id, node) in self.nodes.iter_enumerated() {
            let Some(payload) = node.payload else {
                return Err("reserved rule has no payload");
            };
            if self
                .payloads
                .try_get(payload)
                .and_then(Option::as_ref)
                .is_none()
            {
                return Err("rule payload ID is out of bounds");
            }
            if node.topology.subtree_end as usize <= id.index()
                || node.topology.subtree_end as usize > self.nodes.len()
            {
                return Err("invalid rule subtree boundary");
            }
            if let Some(next) = node.topology.next_sibling {
                if next.index() < node.topology.subtree_end as usize {
                    return Err("next sibling does not skip the preceding subtree");
                }
                if self.nodes[next].topology.parent != node.topology.parent {
                    return Err("next sibling has a different parent");
                }
            }
        }
        Ok(())
    }

    /// Rebuilds rule and payload tapes in final preorder, dropping style-rule
    /// subtrees whose selectors were retired by minification. Rule-list IDs
    /// stay stable; their parent and sibling endpoints are rebuilt.
    pub fn compact(&mut self, declaration_blocks: &mut DeclarationBlockStore<'a>) {
        let old_len = self.nodes.len();
        let old_nodes = std::mem::take(&mut self.nodes);
        let mut old_payloads = std::mem::take(&mut self.payloads);
        let old_lists = std::mem::take(&mut self.lists);
        let mut selectors = std::mem::take(&mut self.selectors);

        let mut nodes = DenseStore::with_capacity(old_len);
        let mut payloads = DenseStore::with_capacity(old_payloads.len());
        let mut lists = DenseStore::with_capacity(old_lists.len());
        for _ in old_lists.iter() {
            lists.push(RuleList {
                parent: None,
                first: None,
                last: None,
            });
        }

        let mut mapping = std::vec![None; old_len];
        let mut boundaries = std::vec![0_usize; old_len + 1];
        let mut retained = std::vec::Vec::with_capacity(old_len);
        let mut old_index = 0;
        while old_index < old_len {
            boundaries[old_index] = nodes.len();
            let old_id = RuleId::from_index(old_index).expect("rule index fits its dense domain");
            let old_node = &old_nodes[old_id];
            let payload_id = old_node
                .payload
                .expect("all rules are finalized before compaction");
            let old_payload = old_payloads[payload_id]
                .as_ref()
                .expect("all rules are finalized before compaction");
            if rule_payload_is_retired(old_payload, &selectors) {
                let subtree_end = old_node.topology.subtree_end as usize;
                for retired_index in old_index..subtree_end {
                    let retired_id = RuleId::from_index(retired_index)
                        .expect("rule index fits its dense domain");
                    let retired_payload = old_nodes[retired_id]
                        .payload
                        .expect("all rules are finalized before compaction");
                    retire_rule_declaration_blocks(
                        old_payloads[retired_payload]
                            .as_ref()
                            .expect("all rules are finalized before compaction"),
                        declaration_blocks,
                    );
                }
                for boundary in &mut boundaries[old_index..subtree_end] {
                    *boundary = nodes.len();
                }
                old_index = subtree_end;
                continue;
            }

            let parent = old_node.topology.parent.map(|parent| {
                mapping[parent.index()].expect("a retained preorder node has a retained parent")
            });
            let payload = old_payloads[payload_id]
                .take()
                .expect("a retained payload is moved once");
            let new_payload = payloads.push(Some(payload));
            let list = old_node.topology.list;
            let new_id = nodes.push(SyntaxNode {
                payload: Some(new_payload),
                topology: RuleTopology {
                    parent,
                    list,
                    next_sibling: None,
                    subtree_end: 0,
                },
            });
            mapping[old_index] = Some(new_id);
            if let Some(previous) = lists[list].last {
                nodes[previous].topology.next_sibling = Some(new_id);
            } else {
                lists[list].first = Some(new_id);
            }
            lists[list].last = Some(new_id);
            retained.push((new_id, old_node.topology.subtree_end as usize));
            old_index += 1;
        }
        boundaries[old_len] = nodes.len();

        for (id, old_subtree_end) in retained {
            nodes[id].topology.subtree_end = u32::try_from(boundaries[old_subtree_end])
                .expect("compacted rule count fits its u32 topology domain");
        }
        for (list_id, old_list) in old_lists.iter_enumerated() {
            lists[list_id].parent = old_list.parent.and_then(|parent| mapping[parent.index()]);
        }

        self.nodes = nodes;
        self.payloads = payloads;
        self.lists = lists;
        selectors.compact();
        self.selectors = selectors;
        debug_assert!(self.validate().is_ok());
    }
}

fn retire_rule_declaration_blocks(
    rule: &CssRule<'_>,
    declaration_blocks: &mut DeclarationBlockStore<'_>,
) {
    match rule {
        CssRule::Style(rule) => declaration_blocks.retire_block(rule.declarations),
        CssRule::Keyframes(rule) => {
            for keyframe in &rule.keyframes {
                declaration_blocks.retire_block(keyframe.declarations);
            }
        }
        CssRule::Page(rule) => {
            declaration_blocks.retire_block(rule.declarations);
            for margin in &rule.rules {
                declaration_blocks.retire_block(margin.declarations);
            }
        }
        CssRule::CounterStyle(rule) => declaration_blocks.retire_block(rule.declarations),
        CssRule::Nesting(rule) => declaration_blocks.retire_block(rule.style.declarations),
        CssRule::NestedDeclarations(rule) => {
            declaration_blocks.retire_block(rule.declarations);
        }
        CssRule::Viewport(rule) => declaration_blocks.retire_block(rule.declarations),
        CssRule::PositionTry(rule) => declaration_blocks.retire_block(rule.declarations),
        _ => {}
    }
}

fn rule_payload_is_retired(rule: &CssRule<'_>, selectors: &SelectorStore<'_>) -> bool {
    let selector_list = match rule {
        CssRule::Style(style) => Some(style.selectors),
        CssRule::Nesting(rule) => Some(rule.style.selectors),
        _ => None,
    };
    selector_list.is_some_and(|list| selectors.get(list).iter().all(Selector::is_tombstone))
}

impl Default for RuleStore<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Visit)]
#[visit(skip)]
pub struct RuleChildren<'store, 'ast> {
    #[visit(skip)]
    store: &'store RuleStore<'ast>,
    #[visit(skip)]
    next: Option<RuleId>,
}

#[derive(Clone, Copy, Visit)]
#[visit(skip)]
pub struct RuleListRef<'store, 'ast> {
    #[visit(skip)]
    store: &'store RuleStore<'ast>,
    #[visit(skip)]
    list: RuleListId,
}

impl<'store, 'ast> RuleListRef<'store, 'ast> {
    #[inline]
    pub fn new(store: &'store RuleStore<'ast>, list: RuleListId) -> Self {
        Self { store, list }
    }

    #[inline]
    pub fn len(self) -> usize {
        self.store.list_len(self.list)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.store.list_is_empty(self.list)
    }

    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'store CssRule<'ast>> {
        self.store.children(self.list).map(|(_, rule)| rule)
    }
}

impl<'ast> Index<usize> for RuleListRef<'_, 'ast> {
    type Output = CssRule<'ast>;

    fn index(&self, index: usize) -> &Self::Output {
        self.store
            .children(self.list)
            .nth(index)
            .map(|(_, rule)| rule)
            .expect("rule index is outside its direct-child list")
    }
}

impl<'store, 'ast> Iterator for RuleChildren<'store, 'ast> {
    type Item = (RuleId, &'store CssRule<'ast>);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next?;
        self.next = self.store.topology(id).next_sibling;
        Some((id, self.store.get(id)))
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum CssRule<'a> {
    Media(MediaRule<'a>),
    Import(ImportRule<'a>),
    Style(StyleRule<'a>),
    Keyframes(KeyframesRule<'a>),
    FontFace(FontFaceRule<'a>),
    FontPaletteValues(FontPaletteValuesRule<'a>),
    FontFeatureValues(FontFeatureValuesRule<'a>),
    Page(PageRule<'a>),
    Supports(SupportsRule<'a>),
    CounterStyle(CounterStyleRule<'a>),
    Charset(CharsetRule<'a>),
    Namespace(NamespaceRule<'a>),
    MozDocument(MozDocumentRule<'a>),
    Nesting(NestingRule<'a>),
    NestedDeclarations(NestedDeclarationsRule),
    Viewport(ViewportRule),
    CustomMedia(CustomMediaRule<'a>),
    LayerStatement(LayerStatementRule<'a>),
    LayerBlock(LayerBlockRule<'a>),
    Property(PropertyRule<'a>),
    Container(ContainerRule<'a>),
    Scope(ScopeRule<'a>),
    StartingStyle(StartingStyleRule<'a>),
    ViewTransition(ViewTransitionRule<'a>),
    PositionTry(PositionTryRule<'a>),
    Unknown(UnknownAtRule<'a>),
    Custom(DefaultAtRule),
}

impl CssRule<'_> {
    #[inline]
    pub fn span<'ghost>(&self, _token: &GhostToken<'ghost>) -> Span {
        match self {
            Self::Media(rule) => rule.span(),
            Self::Import(rule) => rule.span(),
            Self::Style(rule) => rule.span,
            Self::Keyframes(rule) => rule.span(),
            Self::FontFace(rule) => rule.span(),
            Self::FontPaletteValues(rule) => rule.span(),
            Self::FontFeatureValues(rule) => rule.span(),
            Self::Page(rule) => rule.span(),
            Self::Supports(rule) => rule.span(),
            Self::CounterStyle(rule) => rule.span(),
            Self::Charset(rule) => rule.span(),
            Self::Namespace(rule) => rule.span(),
            Self::MozDocument(rule) => rule.span(),
            Self::Nesting(rule) => rule.span(),
            Self::NestedDeclarations(rule) => rule.span(),
            Self::Viewport(rule) => rule.span(),
            Self::CustomMedia(rule) => rule.span(),
            Self::LayerStatement(rule) => rule.span(),
            Self::LayerBlock(rule) => rule.span(),
            Self::Property(rule) => rule.span(),
            Self::Container(rule) => rule.span(),
            Self::Scope(rule) => rule.span(),
            Self::StartingStyle(rule) => rule.span(),
            Self::ViewTransition(rule) => rule.span(),
            Self::PositionTry(rule) => rule.span(),
            Self::Unknown(rule) => rule.span(),
            Self::Custom(_) => DUMMY_SP,
        }
    }
}

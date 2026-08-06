use std::hash::{Hash, Hasher};

use rocketcss_common::{Allocator, DenseId};
use rustc_hash::FxHasher;

use crate::VendorPrefix;

use super::{CssRuleId as RuleId, StyleSheetMutationError, *};

type MutationError<'ast> = StyleSheetMutationError<'ast>;

/// The selector-bearing syntax that contributes one selector path frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectorFrameKind {
    Style,
    Nesting,
}

/// A conditional wrapper whose typed value is canonicalized, or whose
/// occurrence remains deliberately opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContextFrameKind {
    Media,
    Supports,
    Container,
    MozDocument,
    Scope,
    StartingStyle,
}

impl ContextFrameKind {
    #[inline]
    const fn is_opaque(self) -> bool {
        matches!(self, Self::MozDocument | Self::Scope | Self::StartingStyle)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CascadeOrigin {
    Author,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CascadePhase {
    AuthorNormalAndImportant,
}

pub enum HistorySegment<P> {
    StyleCascade,
    Isolated(super::RuleId<P>),
}

impl<P> Clone for HistorySegment<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for HistorySegment<P> {}

impl<P> std::fmt::Debug for HistorySegment<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StyleCascade => f.write_str("StyleCascade"),
            Self::Isolated(rule) => f.debug_tuple("Isolated").field(rule).finish(),
        }
    }
}

impl<P> PartialEq for HistorySegment<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StyleCascade, Self::StyleCascade) => true,
            (Self::Isolated(left), Self::Isolated(right)) => left == right,
            _ => false,
        }
    }
}

impl<P> Eq for HistorySegment<P> {}

impl<P> Hash for HistorySegment<P> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        if let Self::Isolated(rule) = self {
            rule.hash(state);
        }
    }
}

/// Exact selector and cascade context stored by every declaration block.
pub struct EffectiveKeyData<P> {
    selector_path: Option<SelectorPathId>,
    context_path: Option<ContextPathId>,
    layer: Option<LayerContextId>,
    origin: CascadeOrigin,
    cascade_phase: CascadePhase,
    history_segment: HistorySegment<P>,
}

impl<P> Clone for EffectiveKeyData<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for EffectiveKeyData<P> {}

impl<P> std::fmt::Debug for EffectiveKeyData<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectiveKeyData")
            .field("selector_path", &self.selector_path)
            .field("context_path", &self.context_path)
            .field("layer", &self.layer)
            .field("origin", &self.origin)
            .field("cascade_phase", &self.cascade_phase)
            .field("history_segment", &self.history_segment)
            .finish()
    }
}

impl<P> PartialEq for EffectiveKeyData<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.selector_path == other.selector_path
            && self.context_path == other.context_path
            && self.layer == other.layer
            && self.origin == other.origin
            && self.cascade_phase == other.cascade_phase
            && self.history_segment == other.history_segment
    }
}

impl<P> Eq for EffectiveKeyData<P> {}

impl<P> Hash for EffectiveKeyData<P> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.selector_path.hash(state);
        self.context_path.hash(state);
        self.layer.hash(state);
        self.origin.hash(state);
        self.cascade_phase.hash(state);
        self.history_segment.hash(state);
    }
}

impl<P> EffectiveKeyData<P> {
    #[inline]
    pub const fn selector_path(self) -> Option<SelectorPathId> {
        self.selector_path
    }

    #[inline]
    pub const fn context_path(self) -> Option<ContextPathId> {
        self.context_path
    }

    #[inline]
    pub const fn layer(self) -> Option<LayerContextId> {
        self.layer
    }

    #[inline]
    pub const fn origin(self) -> CascadeOrigin {
        self.origin
    }

    #[inline]
    pub const fn cascade_phase(self) -> CascadePhase {
        self.cascade_phase
    }

    #[inline]
    pub const fn history_segment(self) -> HistorySegment<P> {
        self.history_segment
    }
}

impl<P> RuleIdReferences<P> for EffectiveKeyData<P> {
    fn remap_rule_ids(&mut self, remaps: &[RadixIdRemap<super::RuleId<P>>]) {
        match &mut self.history_segment {
            HistorySegment::Isolated(rule) => {
                *rule = remap_id(*rule, remaps);
            }
            HistorySegment::StyleCascade => {}
        }
    }
}

/// Parser-local semantic context. It contains only compact IDs and can be
/// copied through recursive descent without retaining AST borrows.
pub struct EffectiveContext<P> {
    style_rule: Option<super::RuleId<P>>,
    selector_path: Option<SelectorPathId>,
    context_path: Option<ContextPathId>,
    layer: Option<LayerContextId>,
    origin: CascadeOrigin,
    cascade_phase: CascadePhase,
    history_segment: HistorySegment<P>,
}

impl<P> Clone for EffectiveContext<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for EffectiveContext<P> {}

impl<P> std::fmt::Debug for EffectiveContext<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectiveContext")
            .field("style_rule", &self.style_rule)
            .field("selector_path", &self.selector_path)
            .field("context_path", &self.context_path)
            .field("layer", &self.layer)
            .field("origin", &self.origin)
            .field("cascade_phase", &self.cascade_phase)
            .field("history_segment", &self.history_segment)
            .finish()
    }
}

impl<P> PartialEq for EffectiveContext<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.style_rule == other.style_rule
            && self.selector_path == other.selector_path
            && self.context_path == other.context_path
            && self.layer == other.layer
            && self.origin == other.origin
            && self.cascade_phase == other.cascade_phase
            && self.history_segment == other.history_segment
    }
}

impl<P> Eq for EffectiveContext<P> {}

impl<P> Default for EffectiveContext<P> {
    fn default() -> Self {
        Self {
            style_rule: None,
            selector_path: None,
            context_path: None,
            layer: None,
            origin: CascadeOrigin::Author,
            cascade_phase: CascadePhase::AuthorNormalAndImportant,
            history_segment: HistorySegment::StyleCascade,
        }
    }
}

impl<P> EffectiveContext<P> {
    #[inline]
    pub const fn style_rule(self) -> Option<super::RuleId<P>> {
        self.style_rule
    }

    #[inline]
    pub const fn selector_path(self) -> Option<SelectorPathId> {
        self.selector_path
    }

    #[inline]
    pub const fn context_path(self) -> Option<ContextPathId> {
        self.context_path
    }

    #[inline]
    pub const fn layer(self) -> Option<LayerContextId> {
        self.layer
    }

    #[inline]
    pub const fn effective_key(self) -> EffectiveKeyData<P> {
        EffectiveKeyData {
            selector_path: self.selector_path,
            context_path: self.context_path,
            layer: self.layer,
            origin: self.origin,
            cascade_phase: self.cascade_phase,
            history_segment: self.history_segment,
        }
    }

    #[inline]
    pub const fn isolated(rule: super::RuleId<P>) -> EffectiveKeyData<P> {
        EffectiveKeyData {
            selector_path: None,
            context_path: None,
            layer: None,
            origin: CascadeOrigin::Author,
            cascade_phase: CascadePhase::AuthorNormalAndImportant,
            history_segment: HistorySegment::Isolated(rule),
        }
    }
}

#[derive(Debug)]
pub struct SelectorValueRecord<'ast> {
    selectors: crate::SelectorList<'ast>,
    kind: SelectorFrameKind,
    vendor_prefix: VendorPrefix,
    fingerprint: u64,
}

impl<'ast> SelectorValueRecord<'ast> {
    #[inline]
    pub fn selectors(&self) -> &crate::SelectorList<'ast> {
        &self.selectors
    }

    #[inline]
    pub const fn kind(&self) -> SelectorFrameKind {
        self.kind
    }

    #[inline]
    pub const fn vendor_prefix(&self) -> VendorPrefix {
        self.vendor_prefix
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SelectorPathKey {
    parent: Option<SelectorPathId>,
    value: SelectorValueId,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SelectorPathRecord {
    pub(super) parent: Option<SelectorPathId>,
    pub(super) value: SelectorValueId,
    pub(super) fingerprint: u64,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContextValueRecord<P> {
    pub(super) representative: super::RuleId<P>,
    pub(super) fingerprint: u64,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContextValueState<P> {
    pub(super) id: ContextValueId,
    pub(super) representative: super::RuleId<P>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ContextPathKey {
    parent: Option<ContextPathId>,
    value: ContextValueId,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContextPathRecord {
    pub(super) parent: Option<ContextPathId>,
    pub(super) value: ContextValueId,
    pub(super) fingerprint: u64,
}

/// The result of refreshing wrapper identities, including the remap needed by
/// a transient consumer that published block metadata before the refresh.
#[doc(hidden)]
pub struct ContextIdentityRepair<'scratch> {
    changed: bool,
    effective_key_remaps: rocketcss_common::vec::Vec<'scratch, EffectiveKeyId>,
}

impl ContextIdentityRepair<'_> {
    #[inline]
    pub const fn changed(&self) -> bool {
        self.changed
    }

    #[inline]
    pub fn effective_key_remaps(&self) -> &[EffectiveKeyId] {
        &self.effective_key_remaps
    }
}

pub(crate) struct LayerContextKey<P> {
    pub(super) parent: Option<LayerContextId>,
    pub(super) occurrence: super::RuleId<P>,
}

impl<P> Clone for LayerContextKey<P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for LayerContextKey<P> {}

impl<P> std::fmt::Debug for LayerContextKey<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerContextKey")
            .field("parent", &self.parent)
            .field("occurrence", &self.occurrence)
            .finish()
    }
}

impl<P> PartialEq for LayerContextKey<P> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.occurrence == other.occurrence
    }
}

impl<P> Eq for LayerContextKey<P> {}

impl<P> Hash for LayerContextKey<P> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.occurrence.hash(state);
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LayerContextRecord<P> {
    pub(super) parent: Option<LayerContextId>,
    pub(super) occurrence: super::RuleId<P>,
}

impl<'ast> StyleSheet<'ast> {
    /// Applies one local transform to every canonical selector value, then
    /// repairs selector paths and EffectiveKeys whose identities converge.
    ///
    /// Selector values are shared by rules, so mutating them through a rule
    /// would either repeat work or leave interner fingerprints stale. This
    /// transaction keeps the shared representation authoritative and does not
    /// expose partially rebuilt key state to callers.
    #[doc(hidden)]
    pub fn transform_selector_values(
        &mut self,
        mut transform: impl FnMut(SelectorValueId, &mut crate::SelectorList<'ast>),
    ) {
        let allocator = self.allocator;
        self.transform_selector_values_in(allocator, &mut transform);
    }

    /// Variant of [`Self::transform_selector_values`] whose repair scratch is
    /// owned by the caller's minify arena.
    #[doc(hidden)]
    pub fn transform_selector_values_in(
        &mut self,
        allocator: &Allocator,
        mut transform: impl FnMut(SelectorValueId, &mut crate::SelectorList<'ast>),
    ) -> bool {
        let mut changed = false;
        for (index, value) in self.selector_values.iter_mut().enumerate() {
            let id = SelectorValueId::from_index(index)
                .expect("an existing selector value has a representable dense ID");
            let previous_fingerprint = value.fingerprint;
            transform(id, &mut value.selectors);
            value.fingerprint =
                selector_value_fingerprint(&value.selectors, value.kind, value.vendor_prefix);
            changed |= value.fingerprint != previous_fingerprint;
        }

        // The common minify path leaves the typed selector values untouched.
        // In that case the existing canonical IDs and all dependent revisions
        // are already authoritative, so repairing them would only recreate
        // transient maps and invalidate every candidate for no reason.
        if !changed {
            return false;
        }

        let mut value_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.selector_values.len(), allocator);
        self.selector_value_buckets.clear();
        for (id, value) in self.selector_values.iter_enumerated() {
            debug_assert_eq!(id.index(), value_remaps.len());
            let canonical = self
                .selector_value_buckets
                .get(&value.fingerprint)
                .and_then(|bucket| {
                    bucket.iter().copied().find(|&candidate| {
                        let candidate = &self.selector_values[candidate];
                        candidate.kind == value.kind
                            && candidate.vendor_prefix == value.vendor_prefix
                            && candidate.selectors == value.selectors
                    })
                })
                .unwrap_or(id);
            value_remaps.push(canonical);
            if canonical == id {
                self.selector_value_buckets
                    .entry(value.fingerprint)
                    .or_default()
                    .push(id);
            }
        }

        let mut current = self.first_rule_in_source;
        while let Some(id) = current {
            let next = self.rules.get(id).and_then(|rule| rule.next_in_source);
            let rule = self
                .rules
                .get_mut(id)
                .expect("an enumerated selector owner remains resolvable");
            let selector = match rule.payload_mut() {
                CssRule::Style(payload) => Some(&mut payload.selector_value),
                CssRule::Nesting(payload) => Some(&mut payload.selector_value),
                _ => None,
            };
            if let Some(selector) = selector {
                *selector = value_remaps[selector.index()];
                rule.revision = rule.revision.wrapping_add(1);
            }
            current = next;
        }

        let mut path_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.selector_paths.len(), allocator);
        self.selector_path_ids.clear();
        self.root_selector_paths.fill(None);
        for index in 0..self.selector_paths.len() {
            let id = SelectorPathId::from_index(index)
                .expect("an existing selector path has a representable dense ID");
            debug_assert_eq!(id.index(), path_remaps.len());
            let record = self.selector_paths[id];
            let parent = record.parent.map(|parent| path_remaps[parent.index()]);
            let value = value_remaps[record.value.index()];
            let key = SelectorPathKey { parent, value };
            let canonical = if parent.is_none() {
                self.root_selector_paths[value.index()].unwrap_or(id)
            } else {
                self.selector_path_ids.get(&key).copied().unwrap_or(id)
            };
            let mut hasher = FxHasher::default();
            parent
                .map_or(0, |parent| self.selector_paths[parent].fingerprint)
                .hash(&mut hasher);
            self.selector_values[value].fingerprint.hash(&mut hasher);
            let record = &mut self.selector_paths[id];
            record.parent = parent;
            record.value = value;
            record.fingerprint = hasher.finish();
            path_remaps.push(canonical);
            if canonical == id {
                if parent.is_none() {
                    self.root_selector_paths[value.index()] = Some(id);
                } else {
                    self.selector_path_ids.insert(key, id);
                }
            }
        }

        let mut key_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.effective_keys.len(), allocator);
        self.effective_key_ids.clear();
        for index in 0..self.effective_keys.len() {
            let id = EffectiveKeyId::from_index(index)
                .expect("an existing effective key has a representable dense ID");
            debug_assert_eq!(id.index(), key_remaps.len());
            let key = &mut self.effective_keys[id];
            key.selector_path = key.selector_path.map(|path| path_remaps[path.index()]);
            let canonical = self.effective_key_ids.get(key).copied().unwrap_or(id);
            key_remaps.push(canonical);
            if canonical == id {
                self.effective_key_ids.insert(*key, id);
            }
        }

        let mut current = self.first_rule_in_source;
        while let Some(rule_id) = current {
            let next = self.rules.get(rule_id).and_then(|rule| rule.next_in_source);
            let block_id = self
                .rules
                .get(rule_id)
                .and_then(|rule| rule.declaration_block);
            if let Some(block_id) = block_id {
                let block = self
                    .declaration_blocks
                    .get_mut(block_id)
                    .expect("an enumerated declaration block remains resolvable");
                if block.live {
                    block.effective_key = key_remaps[block.effective_key.index()];
                    block.revision = block.revision.wrapping_add(1);
                }
            }
            current = next;
        }
        true
    }

    /// Rebuilds typed wrapper identities after rule-local value transforms.
    ///
    /// Media, supports, and container payloads participate in EffectiveKey
    /// identity. Their AST values may be normalized in place, so the context
    /// interner must be canonicalized before cross-rule work observes it.
    #[doc(hidden)]
    pub fn refresh_context_value_identities(&mut self) -> Result<(), MutationError<'ast>> {
        let allocator = self.allocator;
        self.refresh_context_value_identities_in(allocator)
            .map(|_| ())
    }

    /// Variant of [`Self::refresh_context_value_identities`] whose repair
    /// scratch is owned by the caller's minify arena. Returns whether a
    /// context value changed semantically.
    #[doc(hidden)]
    pub fn refresh_context_value_identities_in(
        &mut self,
        allocator: &Allocator,
    ) -> Result<bool, MutationError<'ast>> {
        self.refresh_context_value_identities_with_remaps(allocator)
            .map(|repair| repair.changed())
    }

    /// Variant that exposes the final EffectiveKey remap to a transient
    /// consumer which published block metadata before wrapper repair.
    #[doc(hidden)]
    pub fn refresh_context_value_identities_with_remaps<'scratch>(
        &mut self,
        allocator: &'scratch Allocator,
    ) -> Result<ContextIdentityRepair<'scratch>, MutationError<'ast>> {
        let mut changed = false;
        for index in 0..self.context_values.len() {
            let id = ContextValueId::from_index(index)
                .expect("an existing context value has a representable dense ID");
            let representative = self.context_values[id].representative;
            let kind = self
                .context_frame_kind(representative)
                .ok_or(MutationError::InvalidRuleTopology(representative))?;
            let fingerprint = self.hash_context_frame(representative, kind);
            changed |= fingerprint != self.context_values[id].fingerprint;
        }
        if !changed {
            return Ok(ContextIdentityRepair {
                changed: false,
                effective_key_remaps: rocketcss_common::vec::Vec::new_in(allocator),
            });
        }

        let mut value_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.context_values.len(), allocator);
        self.context_value_buckets.clear();
        for index in 0..self.context_values.len() {
            let id = ContextValueId::from_index(index)
                .expect("an existing context value has a representable dense ID");
            debug_assert_eq!(id.index(), value_remaps.len());
            let representative = self.context_values[id].representative;
            let kind = self
                .context_frame_kind(representative)
                .ok_or(MutationError::InvalidRuleTopology(representative))?;
            let fingerprint = self.hash_context_frame(representative, kind);
            self.context_values[id].fingerprint = fingerprint;
            let canonical = self
                .context_value_buckets
                .get(&fingerprint)
                .and_then(|bucket| {
                    bucket.iter().find_map(|state| {
                        (self.context_frame_kind(state.representative) == Some(kind)
                            && self.context_frames_equal(
                                state.representative,
                                representative,
                                kind,
                            ))
                        .then_some(state.id)
                    })
                })
                .unwrap_or(id);
            value_remaps.push(canonical);
            if canonical == id {
                self.context_value_buckets
                    .entry(fingerprint)
                    .or_default()
                    .push(ContextValueState { id, representative });
            }
        }

        let mut path_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.context_paths.len(), allocator);
        self.context_path_ids.clear();
        for index in 0..self.context_paths.len() {
            let id = ContextPathId::from_index(index)
                .expect("an existing context path has a representable dense ID");
            debug_assert_eq!(id.index(), path_remaps.len());
            let record = self.context_paths[id];
            let parent = record.parent.map(|parent| path_remaps[parent.index()]);
            let value = value_remaps[record.value.index()];
            let key = ContextPathKey { parent, value };
            let canonical = self.context_path_ids.get(&key).copied().unwrap_or(id);
            let mut hasher = FxHasher::default();
            parent
                .map_or(0, |parent| self.context_paths[parent].fingerprint)
                .hash(&mut hasher);
            self.context_values[value].fingerprint.hash(&mut hasher);
            let record = &mut self.context_paths[id];
            record.parent = parent;
            record.value = value;
            record.fingerprint = hasher.finish();
            path_remaps.push(canonical);
            if canonical == id {
                self.context_path_ids.insert(key, id);
            }
        }

        let mut key_remaps =
            rocketcss_common::vec::Vec::with_capacity_in(self.effective_keys.len(), allocator);
        self.effective_key_ids.clear();
        for index in 0..self.effective_keys.len() {
            let id = EffectiveKeyId::from_index(index)
                .expect("an existing effective key has a representable dense ID");
            debug_assert_eq!(id.index(), key_remaps.len());
            let key = &mut self.effective_keys[id];
            key.context_path = key.context_path.map(|path| path_remaps[path.index()]);
            let canonical = self.effective_key_ids.get(key).copied().unwrap_or(id);
            key_remaps.push(canonical);
            if canonical == id {
                self.effective_key_ids.insert(*key, id);
            }
        }

        let mut current = self.first_rule_in_source;
        while let Some(rule_id) = current {
            let next = self.rules.get(rule_id).and_then(|rule| rule.next_in_source);
            let block_id = self
                .rules
                .get(rule_id)
                .and_then(|rule| rule.declaration_block);
            if let Some(block_id) = block_id {
                let block = self
                    .declaration_blocks
                    .get_mut(block_id)
                    .expect("an enumerated declaration block remains resolvable");
                if block.live {
                    block.effective_key = key_remaps[block.effective_key.index()];
                    block.revision = block.revision.wrapping_add(1);
                }
            }
            current = next;
        }
        Ok(ContextIdentityRepair {
            changed: true,
            effective_key_remaps: key_remaps,
        })
    }

    pub fn enter_selector_context(
        &mut self,
        context: EffectiveContext<CssRule<'ast>>,
        rule: RuleId<'ast>,
    ) -> Result<EffectiveContext<CssRule<'ast>>, MutationError<'ast>> {
        let value = match self
            .rule(rule)
            .ok_or(MutationError::UnknownRule(rule))?
            .payload()
        {
            CssRule::Style(payload) => payload.selector_value,
            CssRule::Nesting(payload) => payload.selector_value,
            _ => return Err(MutationError::InvalidRuleTopology(rule)),
        };
        let selector_path = self.intern_selector_path(context.selector_path, value)?;
        Ok(EffectiveContext {
            style_rule: Some(rule),
            selector_path: Some(selector_path),
            ..context
        })
    }

    pub fn enter_wrapper_context(
        &mut self,
        context: EffectiveContext<CssRule<'ast>>,
        rule: RuleId<'ast>,
    ) -> Result<EffectiveContext<CssRule<'ast>>, MutationError<'ast>> {
        if matches!(
            self.rule(rule).map(RuleRecord::payload),
            Some(CssRule::LayerBlock(_))
        ) {
            let key = LayerContextKey {
                parent: context.layer,
                occurrence: rule,
            };
            let layer = if let Some(&id) = self.layer_context_ids.get(&key) {
                id
            } else {
                let id = self
                    .layer_contexts
                    .try_push(LayerContextRecord {
                        parent: key.parent,
                        occurrence: rule,
                    })
                    .map_err(|_| MutationError::SelectorContextCapacityExhausted)?;
                self.layer_context_ids.insert(key, id);
                id
            };
            return Ok(EffectiveContext {
                layer: Some(layer),
                ..context
            });
        }

        let value = self.intern_context_value(rule)?;
        let key = ContextPathKey {
            parent: context.context_path,
            value,
        };
        let context_path = if let Some(&id) = self.context_path_ids.get(&key) {
            id
        } else {
            let mut hasher = FxHasher::default();
            context
                .context_path
                .map_or(0, |id| self.context_paths[id].fingerprint)
                .hash(&mut hasher);
            self.context_values[value].fingerprint.hash(&mut hasher);
            let id = self
                .context_paths
                .try_push(ContextPathRecord {
                    parent: key.parent,
                    value,
                    fingerprint: hasher.finish(),
                })
                .map_err(|_| MutationError::SelectorContextCapacityExhausted)?;
            self.context_path_ids.insert(key, id);
            id
        };
        Ok(EffectiveContext {
            context_path: Some(context_path),
            ..context
        })
    }

    pub fn selector_path_record(
        &self,
        id: SelectorPathId,
    ) -> Option<(Option<SelectorPathId>, SelectorValueId)> {
        self.selector_paths
            .try_get(id)
            .map(|record| (record.parent, record.value))
    }

    pub fn selector_value(&self, id: SelectorValueId) -> Option<&SelectorValueRecord<'ast>> {
        self.selector_values.try_get(id)
    }

    pub fn context_path_record(
        &self,
        id: ContextPathId,
    ) -> Option<(Option<ContextPathId>, ContextValueId)> {
        self.context_paths
            .try_get(id)
            .map(|record| (record.parent, record.value))
    }

    pub fn context_value_representative(&self, id: ContextValueId) -> Option<RuleId<'ast>> {
        self.context_values
            .try_get(id)
            .map(|record| record.representative)
    }

    pub fn layer_context_record(
        &self,
        id: LayerContextId,
    ) -> Option<(Option<LayerContextId>, RuleId<'ast>)> {
        self.layer_contexts
            .try_get(id)
            .map(|record| (record.parent, record.occurrence))
    }

    /// Atomically replaces one style/nesting selector identity and all
    /// EffectiveKeys in that rule's subtree that inherit the old selector
    /// path. Other rules sharing the old canonical selector value are not
    /// changed.
    pub fn replace_rule_selector_value(
        &mut self,
        rule: RuleId<'ast>,
        new_value: SelectorValueId,
    ) -> Result<bool, MutationError<'ast>> {
        let allocator = self.allocator;
        self.replace_rule_selector_value_in(rule, new_value, allocator)
    }

    /// Arena-backed variant used by the minify scheduler for selector
    /// subtree repair.
    #[doc(hidden)]
    pub fn replace_rule_selector_value_in(
        &mut self,
        rule: RuleId<'ast>,
        new_value: SelectorValueId,
        allocator: &Allocator,
    ) -> Result<bool, MutationError<'ast>> {
        let record = self.rule(rule).ok_or(MutationError::UnknownRule(rule))?;
        if !record.is_live() {
            return Err(MutationError::RetiredRule(rule));
        }
        let (old_value, expected_kind, expected_prefix) = match record.payload() {
            CssRule::Style(payload) => (
                payload.selector_value,
                SelectorFrameKind::Style,
                payload.vendor_prefix,
            ),
            CssRule::Nesting(payload) => (
                payload.selector_value,
                SelectorFrameKind::Nesting,
                VendorPrefix::NONE,
            ),
            _ => return Err(MutationError::InvalidRuleTopology(rule)),
        };
        let new_record = self
            .selector_values
            .try_get(new_value)
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        if new_record.kind != expected_kind || new_record.vendor_prefix != expected_prefix {
            return Err(MutationError::InvalidRuleTopology(rule));
        }
        if old_value == new_value {
            return Ok(false);
        }
        let owner_block = record
            .declaration_block()
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        let owner_key = self
            .effective_key(
                self.declaration_block(owner_block)
                    .ok_or(MutationError::UnknownDeclarationBlock(owner_block))?
                    .effective_key(),
            )
            .copied()
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        let old_path = owner_key
            .selector_path
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        let old_path_record = self
            .selector_paths
            .try_get(old_path)
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        if old_path_record.value != old_value {
            return Err(MutationError::InvalidRuleTopology(rule));
        }
        let new_path = self.intern_selector_path(old_path_record.parent, new_value)?;

        let after_subtree = self.next_after_subtree(rule);
        let mut current = Some(rule);
        let mut blocks = rocketcss_common::vec::Vec::new_in(allocator);
        while let Some(id) = current {
            if Some(id) == after_subtree {
                break;
            }
            let record = self
                .rule(id)
                .ok_or(MutationError::InvalidRuleTopology(rule))?;
            if record.is_live()
                && let Some(block) = record.declaration_block()
            {
                blocks.push(block);
            }
            current = record.next_in_source();
        }

        let mut path_remaps = rocketcss_common::hash_map::HashMap::new_in(allocator);
        path_remaps.insert(old_path, new_path);
        let mut updates = rocketcss_common::vec::Vec::with_capacity_in(blocks.len(), allocator);
        for block in blocks {
            let old_key_id = self
                .declaration_block(block)
                .ok_or(MutationError::UnknownDeclarationBlock(block))?
                .effective_key();
            let old_key = *self
                .effective_key(old_key_id)
                .ok_or(MutationError::UnknownEffectiveKey(old_key_id))?;
            let Some(path) = old_key.selector_path else {
                continue;
            };
            let replacement = self.replace_selector_path_in(path, &mut path_remaps)?;
            if replacement == path {
                continue;
            }
            let new_key = self.append_effective_key(EffectiveKeyData {
                selector_path: Some(replacement),
                ..old_key
            })?;
            updates.push((block, new_key));
        }

        let rule_record = self
            .rule_mut(rule)
            .expect("the selector owner was validated before commit");
        match rule_record.payload_mut() {
            CssRule::Style(payload) => payload.selector_value = new_value,
            CssRule::Nesting(payload) => payload.selector_value = new_value,
            _ => unreachable!("the selector owner kind was validated before commit"),
        }
        rule_record.revision = rule_record.revision.wrapping_add(1);
        for (block, key) in updates {
            let block = self
                .declaration_block_mut(block)
                .expect("a collected subtree block remains resolvable");
            block.effective_key = key;
            block.revision = block.revision.wrapping_add(1);
        }
        Ok(true)
    }

    fn replace_selector_path_in(
        &mut self,
        path: SelectorPathId,
        remaps: &mut rocketcss_common::hash_map::HashMap<'_, SelectorPathId, SelectorPathId>,
    ) -> Result<SelectorPathId, MutationError<'ast>> {
        if let Some(&replacement) = remaps.get(&path) {
            return Ok(replacement);
        }
        let record =
            *self
                .selector_paths
                .try_get(path)
                .ok_or(MutationError::InvalidRuleTopology(
                    self.first_rule_in_source
                        .expect("a selector path requires at least one rule"),
                ))?;
        let Some(parent) = record.parent else {
            remaps.insert(path, path);
            return Ok(path);
        };
        let replacement_parent = self.replace_selector_path_in(parent, remaps)?;
        let replacement = if replacement_parent == parent {
            path
        } else {
            self.intern_selector_path(Some(replacement_parent), record.value)?
        };
        remaps.insert(path, replacement);
        Ok(replacement)
    }

    /// Interns the final selector-only union key for two S3 endpoints.
    ///
    /// Direct topology proves structural adjacency separately. This method
    /// requires every non-selector context field and the selector-path parent
    /// to match exactly; it never models at-rule equivalence.
    pub fn intern_selector_union_effective_key(
        &mut self,
        left: EffectiveKeyId,
        right: EffectiveKeyId,
        selector: SelectorValueId,
    ) -> Result<Option<EffectiveKeyId>, MutationError<'ast>> {
        if self.selector_values.try_get(selector).is_none() {
            return Err(MutationError::SelectorContextCapacityExhausted);
        }
        let left_key = *self
            .effective_key(left)
            .ok_or(MutationError::UnknownEffectiveKey(left))?;
        let right_key = *self
            .effective_key(right)
            .ok_or(MutationError::UnknownEffectiveKey(right))?;
        let (Some(left_path), Some(right_path)) = (left_key.selector_path, right_key.selector_path)
        else {
            return Ok(None);
        };
        let left_path = *self
            .selector_paths
            .try_get(left_path)
            .ok_or(MutationError::SelectorContextCapacityExhausted)?;
        let right_path = *self
            .selector_paths
            .try_get(right_path)
            .ok_or(MutationError::SelectorContextCapacityExhausted)?;
        if left_path.parent != right_path.parent
            || left_key.context_path != right_key.context_path
            || left_key.layer != right_key.layer
            || left_key.origin != right_key.origin
            || left_key.cascade_phase != right_key.cascade_phase
            || left_key.history_segment != right_key.history_segment
        {
            return Ok(None);
        }
        let selector_path = self.intern_selector_path(left_path.parent, selector)?;
        self.append_effective_key(EffectiveKeyData {
            selector_path: Some(selector_path),
            ..left_key
        })
        .map(Some)
    }

    pub fn intern_selector_value(
        &mut self,
        selectors: crate::SelectorList<'ast>,
        kind: SelectorFrameKind,
        vendor_prefix: VendorPrefix,
    ) -> Result<SelectorValueId, MutationError<'ast>> {
        let fingerprint = selector_value_fingerprint(&selectors, kind, vendor_prefix);
        self.intern_selector_value_with_fingerprint(selectors, kind, vendor_prefix, fingerprint)
    }

    fn intern_selector_value_with_fingerprint(
        &mut self,
        selectors: crate::SelectorList<'ast>,
        kind: SelectorFrameKind,
        vendor_prefix: VendorPrefix,
        fingerprint: u64,
    ) -> Result<SelectorValueId, MutationError<'ast>> {
        if let Some(bucket) = self.selector_value_buckets.get(&fingerprint)
            && let Some(&id) = bucket.iter().find(|&&id| {
                let value = &self.selector_values[id];
                value.kind == kind
                    && value.vendor_prefix == vendor_prefix
                    && value.selectors == selectors
            })
        {
            return Ok(id);
        }
        let id = self
            .selector_values
            .try_push(SelectorValueRecord {
                selectors,
                kind,
                vendor_prefix,
                fingerprint,
            })
            .map_err(|_| MutationError::SelectorContextCapacityExhausted)?;
        debug_assert_eq!(id.index(), self.root_selector_paths.len());
        self.root_selector_paths.push(None);
        self.selector_value_buckets
            .entry(fingerprint)
            .or_default()
            .push(id);
        Ok(id)
    }

    fn intern_selector_path(
        &mut self,
        parent: Option<SelectorPathId>,
        value: SelectorValueId,
    ) -> Result<SelectorPathId, MutationError<'ast>> {
        let key = SelectorPathKey { parent, value };
        if parent.is_none() {
            if let Some(id) = self.root_selector_paths[value.index()] {
                return Ok(id);
            }
        } else if let Some(&id) = self.selector_path_ids.get(&key) {
            return Ok(id);
        }
        let mut hasher = FxHasher::default();
        parent
            .map_or(0, |id| self.selector_paths[id].fingerprint)
            .hash(&mut hasher);
        self.selector_values[value].fingerprint.hash(&mut hasher);
        let id = self
            .selector_paths
            .try_push(SelectorPathRecord {
                parent,
                value,
                fingerprint: hasher.finish(),
            })
            .map_err(|_| MutationError::SelectorContextCapacityExhausted)?;
        if parent.is_none() {
            self.root_selector_paths[value.index()] = Some(id);
        } else {
            self.selector_path_ids.insert(key, id);
        }
        Ok(id)
    }

    fn intern_context_value(
        &mut self,
        rule: RuleId<'ast>,
    ) -> Result<ContextValueId, MutationError<'ast>> {
        let kind = self
            .context_frame_kind(rule)
            .ok_or(MutationError::InvalidRuleTopology(rule))?;
        let fingerprint = self.hash_context_frame(rule, kind);
        if let Some(bucket) = self.context_value_buckets.get(&fingerprint)
            && let Some(state) = bucket
                .iter()
                .find(|state| self.context_frames_equal(state.representative, rule, kind))
        {
            return Ok(state.id);
        }
        let id = self
            .context_values
            .try_push(ContextValueRecord {
                representative: rule,
                fingerprint,
            })
            .map_err(|_| MutationError::SelectorContextCapacityExhausted)?;
        self.context_value_buckets
            .entry(fingerprint)
            .or_default()
            .push(ContextValueState {
                id,
                representative: rule,
            });
        Ok(id)
    }

    fn context_frame_kind(&self, rule: RuleId<'ast>) -> Option<ContextFrameKind> {
        match self.rule(rule)?.payload() {
            CssRule::Media(_) => Some(ContextFrameKind::Media),
            CssRule::Supports(_) => Some(ContextFrameKind::Supports),
            CssRule::Container(_) => Some(ContextFrameKind::Container),
            CssRule::MozDocument(_) => Some(ContextFrameKind::MozDocument),
            CssRule::Scope(_) => Some(ContextFrameKind::Scope),
            CssRule::StartingStyle(_) => Some(ContextFrameKind::StartingStyle),
            _ => None,
        }
    }

    fn hash_context_frame(&self, rule: RuleId<'ast>, kind: ContextFrameKind) -> u64 {
        let mut hasher = FxHasher::default();
        kind.hash(&mut hasher);
        if kind.is_opaque() {
            rule.hash(&mut hasher);
        } else {
            match self.rule(rule).expect("the context rule exists").payload() {
                CssRule::Media(payload) => {
                    debug_hash(&mut hasher, &payload.query);
                }
                CssRule::Supports(payload) => {
                    debug_hash(&mut hasher, &payload.condition);
                }
                CssRule::Container(payload) => {
                    debug_hash(&mut hasher, &payload.name);
                    debug_hash(&mut hasher, &payload.condition);
                }
                _ => unreachable!("typed context kind and payload remain aligned"),
            }
        }
        hasher.finish()
    }

    fn context_frames_equal(
        &self,
        left: RuleId<'ast>,
        right: RuleId<'ast>,
        kind: ContextFrameKind,
    ) -> bool {
        if kind.is_opaque() {
            return left == right;
        }
        match (
            self.rule(left).map(RuleRecord::payload),
            self.rule(right).map(RuleRecord::payload),
        ) {
            (Some(CssRule::Media(left)), Some(CssRule::Media(right))) => left.query == right.query,
            (Some(CssRule::Supports(left)), Some(CssRule::Supports(right))) => {
                left.condition == right.condition
            }
            (Some(CssRule::Container(left)), Some(CssRule::Container(right))) => {
                left.name == right.name && left.condition == right.condition
            }
            _ => false,
        }
    }
}

/// Hashes a complete semantic context value without allocating a temporary
/// formatted string. The context payload types intentionally do not implement
/// `Hash`, while their derived `Debug` output includes every field used by the
/// exact equality checks above. The resulting value is still only a bucket
/// filter; canonicalization always calls `context_frames_equal` on collisions.
fn debug_hash<T: std::fmt::Debug>(hasher: &mut FxHasher, value: &T) {
    let mut writer = DebugHashWriter(hasher);
    std::fmt::write(&mut writer, format_args!("{value:?}"))
        .expect("formatting a context value into a hasher cannot fail");
}

struct DebugHashWriter<'a>(&'a mut FxHasher);

impl std::fmt::Write for DebugHashWriter<'_> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        self.0.write(value.as_bytes());
        Ok(())
    }
}

fn selector_value_fingerprint(
    selectors: &crate::SelectorList<'_>,
    kind: SelectorFrameKind,
    vendor_prefix: VendorPrefix,
) -> u64 {
    let mut hasher = FxHasher::default();
    kind.hash(&mut hasher);
    vendor_prefix.hash(&mut hasher);
    selectors.hash(&mut hasher);
    hasher.finish()
}

fn remap_id<P>(
    id: super::RuleId<P>,
    remaps: &[RadixIdRemap<super::RuleId<P>>],
) -> super::RuleId<P> {
    remaps
        .iter()
        .find_map(|remap| (remap.old == id).then_some(remap.new))
        .unwrap_or(id)
}

#[cfg(test)]
#[path = "../../../tests/unit/stylesheet/effective_key.rs"]
mod tests;

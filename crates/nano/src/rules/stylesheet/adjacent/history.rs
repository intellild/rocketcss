use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use super::{
    equal_live_selectors,
    scheduler::Stabilizer,
    state::{HistoryId, HistoryState, RuleId},
};

impl<'list, 'scratch, 'ast> Stabilizer<'list, 'scratch, 'ast>
where
    'ast: 'scratch,
{
    pub(super) fn register_rule_history(&mut self, rule: RuleId) -> HistoryId {
        let hash = self.selector_history_hash(rule);
        let vendor_prefix = self.rule(rule).vendor_prefix;
        let existing = self.history_buckets.get(&hash).and_then(|histories| {
            histories.iter().copied().find(|history| {
                let state = &self.histories[history.index()];
                state.vendor_prefix == vendor_prefix
                    && equal_live_selectors(
                        &self.rule(state.representative).selectors,
                        &self.rule(rule).selectors,
                    )
            })
        });
        let history = if let Some(history) = existing {
            history
        } else {
            let history = HistoryId(self.histories.len() as u32);
            self.histories.push(HistoryState {
                entries: std::vec::Vec::new(),
                representative: rule,
                vendor_prefix,
                generation: 1,
                consumed_generation: 0,
                queued: false,
            });
            self.history_buckets.entry(hash).or_default().push(history);
            history
        };
        let sequence = self.rule_states[rule.index()].sequence;
        let order_label = self.rule_states[rule.index()].order_label;
        let insertion_index = self.histories[history.index()]
            .entries
            .partition_point(|entry| {
                let sequence = self.declaration_entries[entry.index()].sequence;
                let owner = self.sequences[sequence.index()].owner;
                self.rule_states[owner.index()].order_label <= order_label
            });
        let new_entries = self.sequences[sequence.index()].blocks.clone();
        let entries = &mut self.histories[history.index()].entries;
        if insertion_index == entries.len() {
            entries.extend(new_entries);
        } else {
            entries.splice(insertion_index..insertion_index, new_entries);
        }
        history
    }

    pub(super) fn queue_history(&mut self, history: HistoryId) {
        let state = &mut self.histories[history.index()];
        if state.queued {
            return;
        }
        state.queued = true;
        self.queues.histories.push_back(history);
    }

    pub(super) fn history_changed(&mut self, history: HistoryId) {
        self.histories[history.index()].generation = self.histories[history.index()]
            .generation
            .checked_add(1)
            .expect("history generation overflow");
        self.queue_history(history);
    }

    pub(super) fn process_history(&mut self, history: HistoryId) {
        if !self.histories[history.index()].queued {
            return;
        }
        self.histories[history.index()].queued = false;
        let generation = self.histories[history.index()].generation;
        let entries = self.histories[history.index()].entries.clone();
        if entries.len() > 1 {
            let mut blocks = entries
                .iter()
                .map(|&entry| self.entry_block(entry))
                .collect::<std::vec::Vec<_>>();
            let result = self.minifier.minify_sequence(&mut blocks, self.cx);
            if !result.changed_block_indices.is_empty() {
                let mut changed_sequences = HashSet::new();
                for block_index in result.changed_block_indices {
                    if let Some(&entry) = entries.get(block_index) {
                        changed_sequences.insert(self.declaration_entries[entry.index()].sequence);
                    }
                }
                for sequence in changed_sequences {
                    self.mark_sequence_changed(sequence);
                }
                self.changed = true;
            }
        }
        self.histories[history.index()].consumed_generation = generation;
        if self.histories[history.index()].generation != generation {
            self.queue_history(history);
        }
    }

    fn selector_history_hash(&self, rule: RuleId) -> u64 {
        let rule = self.rule(rule);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        rule.vendor_prefix.bits().hash(&mut hasher);
        let mut len = 0usize;
        for selector in rule
            .selectors
            .iter()
            .filter(|selector| !selector.is_tombstone())
        {
            selector.hash(&mut hasher);
            len += 1;
        }
        len.hash(&mut hasher);
        hasher.finish()
    }
}

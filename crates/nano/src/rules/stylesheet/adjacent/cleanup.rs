use super::scheduler::Stabilizer;

impl<'list, 'scratch, 'ast> Stabilizer<'list, 'scratch, 'ast>
where
    'ast: 'scratch,
{
    pub(super) fn rebuild_rule_list(&mut self) {
        let mut slots = (0..self.storage.len()).collect::<std::vec::Vec<_>>();
        slots.sort_unstable_by_key(|&slot| self.slot_order_labels[slot]);
        for slot in slots {
            let retain =
                self.slot_to_rule[slot].is_none_or(|rule| self.rule_states[rule.index()].live);
            if retain {
                self.rules.push(
                    self.storage[slot]
                        .take()
                        .expect("stored rule is rebuilt once"),
                );
            }
        }
    }
}

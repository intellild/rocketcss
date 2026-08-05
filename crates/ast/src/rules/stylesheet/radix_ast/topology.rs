use super::*;

impl<R: Unpin, D, K> RadixCompilation<'_, R, D, K> {
    /// Returns the final live rule in `rule`'s lexical subtree.
    pub(crate) fn subtree_tail(&self, rule: RuleId<R>) -> Option<RuleId<R>> {
        let record = self.rules.get(rule)?;
        record
            .live
            .then(|| self.rules.advance_id(rule, record.nested_rule_count))
            .flatten()
    }

    /// Returns the first global-preorder rule after `rule`'s whole subtree.
    ///
    /// This may be a direct sibling, an ancestor's sibling, or `None` at the
    /// end of the stylesheet.
    pub(crate) fn next_after_subtree(&self, rule: RuleId<R>) -> Option<RuleId<R>> {
        let record = self.rules.get(rule)?;
        record
            .live
            .then(|| {
                record
                    .nested_rule_count
                    .checked_add(1)
                    .and_then(|span| self.rules.advance_id(rule, span))
            })
            .flatten()
    }
}

use super::*;

impl<'ast, R: Unpin, D, K, N: CompilationNodeStores<'ast>> RadixCompilation<'ast, R, D, K, N> {
    /// Returns the final live rule in `rule`'s lexical subtree.
    ///
    /// Direct siblings are not necessarily adjacent in global source order
    /// because all descendants of the left sibling appear first.
    pub fn subtree_tail(&self, rule: RuleId<'ast, R>) -> Option<RuleId<'ast, R>> {
        let mut current = rule;
        loop {
            let record = self.rules.try_get(current)?;
            if !record.live {
                return None;
            }
            let Some(children) = record.child_list else {
                return Some(current);
            };
            let child_list = self.rule_lists.try_get(children)?;
            let Some(last_child) = child_list.last else {
                return Some(current);
            };
            current = last_child;
        }
    }

    /// Returns the first global-preorder rule after `rule`'s whole subtree.
    ///
    /// This may be a direct sibling, an ancestor's sibling, or `None` at the
    /// end of the stylesheet.
    pub fn next_after_subtree(&self, rule: RuleId<'ast, R>) -> Option<RuleId<'ast, R>> {
        let mut current = rule;
        loop {
            let record = self.rules.try_get(current)?;
            if !record.live {
                return None;
            }
            if let Some(next) = record.next_sibling {
                return Some(next);
            }
            current = record.parent?;
        }
    }
}

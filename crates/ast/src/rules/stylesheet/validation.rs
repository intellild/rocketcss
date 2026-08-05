use super::*;

impl<R: Unpin, D: Unpin, K> StyleSheet<'_, R, D, K> {
    /// Checks preorder subtree spans, parent links, and block owners.
    pub fn validate_ast(&self) -> Result<(), ValidationError<R>> {
        let source_len = self.rules.len();
        let mut ancestors = std::vec::Vec::<(RuleId<R>, usize)>::new();
        for (index, (rule_id, rule)) in self.rules.iter_enumerated().enumerate() {
            while ancestors.last().is_some_and(|(_, end)| index > *end) {
                ancestors.pop();
            }
            let expected_parent = ancestors.last().map(|(id, _)| *id);
            if rule.parent != expected_parent {
                return Err(ValidationError::<R>::RuleHasWrongParent {
                    parent: expected_parent,
                    rule: rule_id,
                });
            }
            let end = index.checked_add(rule.nested_rule_count as usize).ok_or(
                ValidationError::<R>::NestedRuleCountMismatch {
                    rule: rule_id,
                    expected: (source_len - index - 1) as u32,
                    actual: rule.nested_rule_count,
                },
            )?;
            if end >= source_len && rule.nested_rule_count != 0 {
                return Err(ValidationError::<R>::NestedRuleCountMismatch {
                    rule: rule_id,
                    expected: (source_len - index - 1) as u32,
                    actual: rule.nested_rule_count,
                });
            }
            if let Some((_, parent_end)) = ancestors.last()
                && end > *parent_end
            {
                return Err(ValidationError::<R>::NestedRuleCountMismatch {
                    rule: rule_id,
                    expected: (*parent_end - index) as u32,
                    actual: rule.nested_rule_count,
                });
            }
            if rule.nested_rule_count != 0 {
                ancestors.push((rule_id, end));
            }
            if !rule.live {
                continue;
            }
            if let Some(block_id) = rule.declaration_block {
                let block = self.declaration_blocks.get(block_id).ok_or(
                    ValidationError::<R>::MissingOwnedDeclarationBlock {
                        rule: rule_id,
                        block: block_id,
                    },
                )?;
                let DeclarationBlockOwner::<R>::Rule(actual) = block.owner;
                if actual != rule_id {
                    return Err(ValidationError::<R>::DeclarationBlockHasWrongOwner {
                        rule: rule_id,
                        block: block_id,
                        actual,
                    });
                }
            }
        }
        let mut declaration_cursor = self.declarations.iter_enumerated().peekable();
        let mut owned_declarations = 0_u32;
        for (block_id, block) in self.declaration_blocks.iter_enumerated() {
            let range = block.declarations;
            if !range.is_empty() {
                let Some(&(expected, _)) = declaration_cursor.peek() else {
                    return Err(ValidationError::<R>::InvalidDeclarationRange {
                        block: block_id,
                        range,
                    });
                };
                let actual = range.start_id();
                if actual != expected {
                    return Err(ValidationError::<R>::DeclarationRangeStartsOutOfOrder {
                        block: block_id,
                        expected,
                        actual,
                    });
                }
                for _ in 0..range.len() {
                    if declaration_cursor.next().is_none() {
                        return Err(ValidationError::<R>::InvalidDeclarationRange {
                            block: block_id,
                            range,
                        });
                    }
                    owned_declarations += 1;
                }
            }
            if !block.live {
                continue;
            }
            let DeclarationBlockOwner::<R>::Rule(owner) = block.owner;
            let owner_record =
                self.rules
                    .get(owner)
                    .ok_or(ValidationError::<R>::MissingBlockOwner {
                        block: block_id,
                        owner,
                    })?;
            if !owner_record.live {
                return Err(ValidationError::<R>::RetiredBlockOwner {
                    block: block_id,
                    owner,
                });
            }
            if owner_record.declaration_block != Some(block_id) {
                return Err(ValidationError::<R>::OwnerDoesNotReferenceBlock {
                    block: block_id,
                    owner,
                    actual: owner_record.declaration_block,
                });
            }
            if self.effective_keys.try_get(block.effective_key).is_none() {
                return Err(ValidationError::<R>::MissingEffectiveKey {
                    block: block_id,
                    key: block.effective_key,
                });
            }
        }
        if declaration_cursor.next().is_some() {
            return Err(ValidationError::<R>::UnownedDeclarations {
                expected: owned_declarations,
                actual: self.declarations.len() as u32,
            });
        }
        Ok(())
    }
}

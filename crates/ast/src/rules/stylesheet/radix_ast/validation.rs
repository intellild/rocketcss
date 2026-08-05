use rustc_hash::FxHashSet;

use super::*;

impl<R: Unpin, D, K> RadixCompilation<'_, R, D, K> {
    /// Checks typed store IDs, list endpoints, mutual links, and block owners.
    pub fn validate_ast(&self) -> Result<(), ValidationError<R>> {
        let root = self.rule_lists.try_get(self.stylesheet.root_rules).ok_or(
            ValidationError::<R>::MissingRootRuleList(self.stylesheet.root_rules),
        )?;
        if let Some(parent) = root.parent {
            return Err(ValidationError::<R>::RootRuleListHasParent(parent));
        }

        let source_ids = self
            .rules
            .iter_enumerated()
            .map(|(id, _)| id)
            .collect::<std::vec::Vec<_>>();
        if self.first_rule_in_source != source_ids.first().copied()
            || self.last_rule_in_source != source_ids.last().copied()
        {
            return Err(ValidationError::<R>::InvalidSourceEndpoints);
        }
        for (index, &rule_id) in source_ids.iter().enumerate() {
            let rule = self
                .rules
                .get(rule_id)
                .expect("an enumerated source ID remains resolvable");
            let expected_previous = index.checked_sub(1).map(|index| source_ids[index]);
            if rule.previous_in_source != expected_previous {
                return Err(ValidationError::<R>::InvalidSourcePrevious {
                    rule: rule_id,
                    expected: expected_previous,
                    actual: rule.previous_in_source,
                });
            }
            let expected_next = source_ids.get(index + 1).copied();
            if rule.next_in_source != expected_next {
                return Err(ValidationError::<R>::InvalidSourceNext {
                    rule: rule_id,
                    expected: expected_next,
                    actual: rule.next_in_source,
                });
            }
        }

        let mut visited = FxHashSet::default();
        for (list_id, list) in self.rule_lists.iter_enumerated() {
            if let Some(parent) = list.parent {
                let parent_record =
                    self.rules
                        .get(parent)
                        .ok_or(ValidationError::<R>::MissingListParent {
                            list: list_id,
                            parent,
                        })?;
                if !parent_record.live {
                    return Err(ValidationError::<R>::RetiredListParent {
                        list: list_id,
                        parent,
                    });
                }
                if parent_record.child_list != Some(list_id) {
                    return Err(ValidationError::<R>::ParentDoesNotOwnList {
                        list: list_id,
                        parent,
                    });
                }
            }
            if (list.live_len == 0) != (list.first.is_none() && list.last.is_none())
                || (list.first.is_none() != list.last.is_none())
            {
                return Err(ValidationError::<R>::InvalidListEndpoints(list_id));
            }

            let mut current = list.first;
            let mut previous = None;
            let mut actual_len = 0_u32;
            while let Some(rule_id) = current {
                let rule = self
                    .rules
                    .get(rule_id)
                    .ok_or(ValidationError::<R>::MissingRule(rule_id))?;
                if !rule.live {
                    return Err(ValidationError::<R>::RetiredRuleInList {
                        list: list_id,
                        rule: rule_id,
                    });
                }
                if rule.parent_list != list_id {
                    return Err(ValidationError::<R>::RuleHasWrongParentList {
                        list: list_id,
                        rule: rule_id,
                    });
                }
                if rule.parent != list.parent {
                    return Err(ValidationError::<R>::RuleHasWrongParent {
                        list: list_id,
                        rule: rule_id,
                    });
                }
                if rule.previous_sibling != previous {
                    return Err(ValidationError::<R>::RuleHasWrongPrevious {
                        rule: rule_id,
                        expected: previous,
                    });
                }
                if !visited.insert(rule_id) {
                    return Err(ValidationError::<R>::LiveRuleIsNotInOneList(rule_id));
                }
                previous = Some(rule_id);
                current = rule.next_sibling;
                actual_len =
                    actual_len
                        .checked_add(1)
                        .ok_or(ValidationError::<R>::ListLengthMismatch {
                            list: list_id,
                            expected: list.live_len,
                            actual: u32::MAX,
                        })?;
                if actual_len > list.live_len {
                    return Err(ValidationError::<R>::ListLengthMismatch {
                        list: list_id,
                        expected: list.live_len,
                        actual: actual_len,
                    });
                }
            }
            if previous != list.last {
                return Err(ValidationError::<R>::ListDoesNotEndAtLast(list_id));
            }
            if actual_len != list.live_len {
                return Err(ValidationError::<R>::ListLengthMismatch {
                    list: list_id,
                    expected: list.live_len,
                    actual: actual_len,
                });
            }
        }

        for (rule_id, rule) in self.rules.iter_enumerated() {
            if rule.live && !visited.contains(&rule_id) {
                return Err(ValidationError::<R>::LiveRuleIsNotInOneList(rule_id));
            }
            if !rule.live {
                continue;
            }
            if let Some(list_id) = rule.child_list {
                let list = self.rule_lists.try_get(list_id).ok_or(
                    ValidationError::<R>::MissingOwnedChildList {
                        rule: rule_id,
                        list: list_id,
                    },
                )?;
                if list.parent != Some(rule_id) {
                    return Err(ValidationError::<R>::ChildListHasWrongParent {
                        rule: rule_id,
                        list: list_id,
                        actual: list.parent,
                    });
                }
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
        let mut declaration_owners = vec![None; self.declarations.len()];
        for (block_id, block) in self.declaration_blocks.iter_enumerated() {
            let declarations = match block.declarations {
                DeclarationList::Range(range) => {
                    if range.start as usize + range.len as usize > self.declarations.len() {
                        return Err(ValidationError::<R>::InvalidDeclarationRange {
                            block: block_id,
                            range,
                        });
                    }
                    DeclarationIdIter {
                        kind: DeclarationIdIterKind::Range(
                            range.start as usize..range.start as usize + range.len as usize,
                        ),
                    }
                }
                DeclarationList::Local4(local) => DeclarationIdIter {
                    kind: DeclarationIdIterKind::Local4 { local, index: 0 },
                },
                DeclarationList::Overflow(overflow) => DeclarationIdIter {
                    kind: DeclarationIdIterKind::Overflow(
                        self.declaration_overflows
                            .try_get(overflow)
                            .ok_or(ValidationError::<R>::InvalidDeclarationOverflow {
                                block: block_id,
                                overflow,
                            })?
                            .iter(),
                    ),
                },
            };
            for declaration in declarations {
                let Some(owner) = declaration_owners.get_mut(declaration.index()) else {
                    return Err(ValidationError::<R>::InvalidDeclarationReference {
                        block: block_id,
                        declaration,
                    });
                };
                if let Some(first) = *owner {
                    return Err(ValidationError::<R>::DuplicateDeclarationOwner {
                        declaration,
                        first,
                        second: block_id,
                    });
                }
                *owner = Some(block_id);
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
        let owned_declarations = declaration_owners.iter().flatten().count();
        if owned_declarations != self.declarations.len() {
            return Err(ValidationError::<R>::UnownedDeclarations {
                expected: owned_declarations as u32,
                actual: self.declarations.len() as u32,
            });
        }
        Ok(())
    }
}

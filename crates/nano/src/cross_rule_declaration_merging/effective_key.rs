use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::utils::{DeclarationBlockEntry, EffectiveKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct EffectiveKeyId(u32);

impl EffectiveKeyId {
    pub(super) fn index(self) -> usize {
        usize::try_from(self.0).expect("effective key ID fits usize")
    }
}

#[derive(Clone, Copy, Debug)]
struct EffectiveKeyRepresentative {
    entry: u32,
    id: EffectiveKeyId,
}

pub(super) fn intern_effective_keys(
    declaration_blocks: &[DeclarationBlockEntry<'_, '_, '_>],
) -> (std::vec::Vec<EffectiveKeyId>, usize) {
    let mut buckets: FxHashMap<u64, SmallVec<[EffectiveKeyRepresentative; 1]>> =
        FxHashMap::with_capacity_and_hasher(declaration_blocks.len(), Default::default());
    let mut ids = std::vec::Vec::with_capacity(declaration_blocks.len());
    let mut key_count = 0_u32;

    for (entry_index, entry) in declaration_blocks.iter().enumerate() {
        let bucket = buckets
            .entry(entry.effective_key.fingerprint())
            .or_default();
        let id = bucket
            .iter()
            .find_map(|representative| {
                let representative_key = &declaration_blocks
                    [usize::try_from(representative.entry).expect("entry index fits usize")]
                .effective_key;
                equal_effective_keys(representative_key, &entry.effective_key)
                    .then_some(representative.id)
            })
            .unwrap_or_else(|| {
                let id = EffectiveKeyId(key_count);
                key_count = key_count
                    .checked_add(1)
                    .expect("effective key count exceeds u32::MAX");
                bucket.push(EffectiveKeyRepresentative {
                    entry: u32::try_from(entry_index)
                        .expect("declaration block index exceeds u32::MAX"),
                    id,
                });
                id
            });
        ids.push(id);
    }

    (
        ids,
        usize::try_from(key_count).expect("effective key count fits usize"),
    )
}

#[inline]
fn equal_effective_keys(left: &EffectiveKey<'_, '_>, right: &EffectiveKey<'_, '_>) -> bool {
    left == right
}

#[cfg(test)]
mod tests {
    use rocketcss_allocator::Allocator;
    use rocketcss_parser::{ParserOptions, parse};

    use super::*;
    use crate::utils::walk_declaration_blocks;

    #[test]
    fn assigns_dense_ids_after_exact_collision_check() {
        let allocator = Allocator::new();
        allocator.with_ghost(|mut token| {
            let stylesheet = parse(
                "a{x:1}b{x:2}a{x:3}",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
            let declaration_blocks = walk_declaration_blocks(&stylesheet, &token);

            let (ids, key_count) = intern_effective_keys(&declaration_blocks);

            assert_eq!(ids[0], ids[2]);
            assert_ne!(ids[0], ids[1]);
            assert_eq!(key_count, 2);
        });
    }
}

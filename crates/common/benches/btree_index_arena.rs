#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::ItemsCount};
use rocketcss_common::{Allocator, BTreeIndexArena, RadixIndexArena, RadixIndexId};
use std::num::NonZeroU64;

const SIZES: &[usize] = &[256, 4_096, 65_536];
const RANDOM_ACCESS_COUNT: usize = 4_096;
const SPARSE_INSERT_STRIDE: usize = 256;
const SPARSE_SIBLING_KEY: u16 = 512;

type Payload = NonZeroU64;

fn main() {
    divan::main();
}

fn payload(value: usize) -> Payload {
    NonZeroU64::new(value as u64 + 1).unwrap()
}

fn inserted_payload(primary: usize) -> Payload {
    NonZeroU64::new(u64::MAX - primary as u64).unwrap()
}

fn tree_with_values(allocator: &Allocator, len: usize) -> BTreeIndexArena<'_, Payload> {
    let mut tree = BTreeIndexArena::new_in(allocator);
    for value in 0..len {
        tree.push(payload(value));
    }
    tree
}

fn radix_with_values(allocator: &Allocator, len: usize) -> RadixIndexArena<'_, Payload> {
    let mut values = RadixIndexArena::with_capacity_in(len, allocator);
    for value in 0..len {
        values.push_primary(payload(value));
    }
    values
}

fn sparse_insert_count(len: usize) -> usize {
    len.div_ceil(SPARSE_INSERT_STRIDE)
}

fn sparse_vec(len: usize) -> Vec<Payload> {
    let mut values = (0..len).map(payload).collect::<Vec<_>>();
    for (inserted, primary) in (0..len).step_by(SPARSE_INSERT_STRIDE).enumerate() {
        values.insert(primary + inserted + 1, inserted_payload(primary));
    }
    values
}

fn sparse_tree(allocator: &Allocator, len: usize) -> BTreeIndexArena<'_, Payload> {
    let mut values = tree_with_values(allocator, len);
    for (inserted, primary) in (0..len).step_by(SPARSE_INSERT_STRIDE).enumerate() {
        values.insert(primary + inserted + 1, inserted_payload(primary));
    }
    values
}

fn sparse_radix(allocator: &Allocator, len: usize) -> RadixIndexArena<'_, Payload> {
    sparse_radix_with_ids(allocator, len).0
}

fn sparse_radix_with_ids(
    allocator: &Allocator,
    len: usize,
) -> (RadixIndexArena<'_, Payload>, Vec<RadixIndexId>) {
    let mut values = radix_with_values(allocator, len);
    let mut ids = Vec::with_capacity(len + sparse_insert_count(len));
    for primary in 0..len {
        let id = values.primary_id(primary).unwrap();
        ids.push(id);
        if primary.is_multiple_of(SPARSE_INSERT_STRIDE) {
            ids.push(values.insert_sibling(id, SPARSE_SIBLING_KEY, inserted_payload(primary)));
        }
    }
    (values, ids)
}

fn random_indices(len: usize) -> Vec<usize> {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    (0..RANDOM_ACCESS_COUNT)
        .map(|_| {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            state = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            state as usize % len
        })
        .collect()
}

mod build {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut values = Vec::with_capacity(len);
            for value in 0..len {
                values.push(black_box(payload(value)));
            }
            black_box(&values);
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let allocator = Allocator::new();
            let mut values = BTreeIndexArena::<'_, Payload>::new_in(&allocator);
            for value in 0..len {
                values.push(black_box(payload(value)));
            }
            black_box(&values);
        });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let allocator = Allocator::new();
            let mut values = RadixIndexArena::with_capacity_in(len, &allocator);
            for value in 0..len {
                values.push_primary(black_box(payload(value)));
            }
            black_box(&values);
        });
    }
}

mod sparse_build {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let item_count = len + sparse_insert_count(len);
        bencher
            .counter(ItemsCount::new(item_count))
            .bench_local(|| black_box(sparse_vec(len)));
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let item_count = len + sparse_insert_count(len);
        bencher
            .counter(ItemsCount::new(item_count))
            .bench_local(|| {
                let allocator = Allocator::new();
                black_box(sparse_tree(&allocator, len));
            });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let item_count = len + sparse_insert_count(len);
        bencher
            .counter(ItemsCount::new(item_count))
            .bench_local(|| {
                let allocator = Allocator::new();
                black_box(sparse_radix(&allocator, len));
            });
    }
}

mod primary_sequential_iter {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = (0..len).map(payload).collect::<Vec<_>>();
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut sum = 0_u64;
            for &value in black_box(&values) {
                sum = sum.wrapping_add(value.get());
            }
            black_box(sum)
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = tree_with_values(&allocator, len);
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut sum = 0_u64;
            for &value in black_box(&values) {
                sum = sum.wrapping_add(value.get());
            }
            black_box(sum)
        });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = radix_with_values(&allocator, len);
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut sum = 0_u64;
            for &value in black_box(&values).primary_iter() {
                sum = sum.wrapping_add(value.get());
            }
            black_box(sum)
        });
    }
}

mod sparse_sequential_iter {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = sparse_vec(len);
        bencher
            .counter(ItemsCount::new(values.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &value in black_box(&values) {
                    sum = sum.wrapping_add(value.get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = sparse_tree(&allocator, len);
        bencher
            .counter(ItemsCount::new(values.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &value in black_box(&values) {
                    sum = sum.wrapping_add(value.get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = sparse_radix(&allocator, len);
        bencher
            .counter(ItemsCount::new(values.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &value in black_box(&values).semantic_iter() {
                    sum = sum.wrapping_add(value.get());
                }
                black_box(sum)
            });
    }
}

mod sparse_enumerated_iter {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = sparse_radix(&allocator, len);
        bencher
            .counter(ItemsCount::new(values.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for (id, &value) in black_box(&values).iter_enumerated() {
                    sum = sum.wrapping_add(u64::from(id.get()) ^ value.get());
                }
                black_box(sum)
            });
    }
}

mod random_get {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = (0..len).map(payload).collect::<Vec<_>>();
        let indices = random_indices(len);
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(values[index].get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = tree_with_values(&allocator, len);
        let indices = random_indices(len);
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(values.get(index).unwrap().get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = radix_with_values(&allocator, len);
        let ids = random_indices(len)
            .into_iter()
            .map(|index| values.primary_id(index).unwrap())
            .collect::<Vec<_>>();
        bencher.counter(ItemsCount::new(ids.len())).bench_local(|| {
            let mut sum = 0_u64;
            for &id in black_box(&ids) {
                sum = sum.wrapping_add(values.get(id).unwrap().get());
            }
            black_box(sum)
        });
    }
}

mod sparse_random_get {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = sparse_vec(len);
        let indices = random_indices(values.len());
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(values[index].get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let values = sparse_tree(&allocator, len);
        let indices = random_indices(values.len());
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(values.get(index).unwrap().get());
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let (values, ids) = sparse_radix_with_ids(&allocator, len);
        let sampled_ids = random_indices(ids.len())
            .into_iter()
            .map(|index| ids[index])
            .collect::<Vec<_>>();
        bencher
            .counter(ItemsCount::new(sampled_ids.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &id in black_box(&sampled_ids) {
                    sum = sum.wrapping_add(values.get(id).unwrap().get());
                }
                black_box(sum)
            });
    }
}

mod sibling_random_get {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let (values, ids) = sparse_radix_with_ids(&allocator, len);
        let sibling_ids = ids
            .into_iter()
            .filter(|id| !id.is_primary())
            .collect::<Vec<_>>();
        let sampled_ids = random_indices(sibling_ids.len())
            .into_iter()
            .map(|index| sibling_ids[index])
            .collect::<Vec<_>>();
        bencher
            .counter(ItemsCount::new(sampled_ids.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &id in black_box(&sampled_ids) {
                    sum = sum.wrapping_add(values.get(id).unwrap().get());
                }
                black_box(sum)
            });
    }
}

mod repeated_middle_insert_remove {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let mut values = (0..len).map(payload).collect::<Vec<_>>();
        let middle = len / 2;
        bencher.counter(ItemsCount::new(1_usize)).bench_local(|| {
            values.insert(middle, black_box(inserted_payload(middle)));
            black_box(values.remove(middle))
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let mut values = tree_with_values(&allocator, len);
        let middle = len / 2;
        bencher.counter(ItemsCount::new(1_usize)).bench_local(|| {
            values.insert(middle, black_box(inserted_payload(middle)));
            black_box(values.remove(middle))
        });
    }

    #[divan::bench(args = SIZES)]
    fn radix_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let mut values = radix_with_values(&allocator, len);
        let primary = values.primary_id(len / 2).unwrap();
        bencher.counter(ItemsCount::new(1_usize)).bench_local(|| {
            let inserted = values.insert_sibling(
                primary,
                SPARSE_SIBLING_KEY,
                black_box(inserted_payload(len / 2)),
            );
            black_box(values.remove_sibling(inserted).unwrap())
        });
    }
}

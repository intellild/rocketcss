#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::ItemsCount};
use rocketcss_common::{Allocator, BTreeIndexArena};

const SIZES: &[usize] = &[256, 4_096, 65_536];
const RANDOM_ACCESS_COUNT: usize = 4_096;

fn main() {
    divan::main();
}

fn tree_with_values(allocator: &Allocator, len: usize) -> BTreeIndexArena<'_, u64> {
    let mut tree = BTreeIndexArena::new_in(allocator);
    for value in 0..len as u64 {
        tree.push(value);
    }
    tree
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
            for value in 0..len as u64 {
                values.push(black_box(value));
            }
            black_box(&values);
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let allocator = Allocator::new();
            let mut tree = BTreeIndexArena::<'_, u64>::new_in(&allocator);
            for value in 0..len as u64 {
                tree.push(black_box(value));
            }
            black_box(&tree);
        });
    }
}

mod sequential_iter {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = (0..len as u64).collect::<Vec<_>>();
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut sum = 0_u64;
            for &value in black_box(&values) {
                sum = sum.wrapping_add(value);
            }
            black_box(sum)
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let tree = tree_with_values(&allocator, len);
        bencher.counter(ItemsCount::new(len)).bench_local(|| {
            let mut sum = 0_u64;
            for &value in black_box(&tree) {
                sum = sum.wrapping_add(value);
            }
            black_box(sum)
        });
    }
}

mod random_get {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let values = (0..len as u64).collect::<Vec<_>>();
        let indices = random_indices(len);
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(values[index]);
                }
                black_box(sum)
            });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let tree = tree_with_values(&allocator, len);
        let indices = random_indices(len);
        bencher
            .counter(ItemsCount::new(indices.len()))
            .bench_local(|| {
                let mut sum = 0_u64;
                for &index in black_box(&indices) {
                    sum = sum.wrapping_add(*tree.get(index).unwrap());
                }
                black_box(sum)
            });
    }
}

mod middle_insert_remove {
    use super::*;

    #[divan::bench(args = SIZES)]
    fn linear_vec(bencher: Bencher<'_, '_>, len: usize) {
        let mut values = (0..len as u64).collect::<Vec<_>>();
        let middle = len / 2;
        bencher.counter(ItemsCount::new(1_usize)).bench_local(|| {
            values.insert(middle, black_box(u64::MAX));
            black_box(values.remove(middle))
        });
    }

    #[divan::bench(args = SIZES)]
    fn btree_index_arena(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let mut tree = tree_with_values(&allocator, len);
        let middle = len / 2;
        bencher.counter(ItemsCount::new(1_usize)).bench_local(|| {
            tree.insert(middle, black_box(u64::MAX));
            black_box(tree.remove(middle))
        });
    }
}

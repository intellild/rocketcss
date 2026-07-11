#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::ItemsCount};
use hashbrown::HashSet;
use rustc_hash::FxBuildHasher;

const SIZES: &[usize] = &[
    1, 2, 3, 4, 5, 6, 8, 10, 12, 14, 16, 18, 20, 24, 28, 32, 40, 48, 64, 68, 72, 76, 80, 84, 88,
    92, 96, 100, 104, 108, 112, 116, 120, 124, 128, 192, 256,
];

const STRING_SIZES: &[usize] = &[
    1, 2, 3, 4, 5, 6, 8, 10, 12, 14, 16, 24, 32, 40, 48, 64, 80, 96, 128, 160, 192, 256, 384, 512,
];

fn main() {
    divan::main();
}

/// Measures the worst case for linear-scan deduplication: every item is unique.
#[divan::bench(args = SIZES)]
fn vec_quadratic(bencher: Bencher<'_, '_>, n: usize) {
    let values = unique_values(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = Vec::with_capacity(n);
        for &value in black_box(&values) {
            if !deduplicated.contains(&value) {
                deduplicated.push(value);
            }
        }
        black_box(deduplicated);
    });
}

#[divan::bench(args = SIZES)]
fn hashbrown_fx(bencher: Bencher<'_, '_>, n: usize) {
    let values = unique_values(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = HashSet::with_capacity_and_hasher(n, FxBuildHasher);
        for &value in black_box(&values) {
            deduplicated.insert(value);
        }
        black_box(deduplicated);
    });
}

#[divan::bench(args = STRING_SIZES)]
fn vec_string_distributed(bencher: Bencher<'_, '_>, n: usize) {
    let values = distributed_strings(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = Vec::with_capacity(n);
        for value in black_box(&values) {
            let value = value.as_str();
            if !deduplicated.contains(&value) {
                deduplicated.push(value);
            }
        }
        black_box(deduplicated);
    });
}

#[divan::bench(args = STRING_SIZES)]
fn hashbrown_fx_string_distributed(bencher: Bencher<'_, '_>, n: usize) {
    let values = distributed_strings(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = HashSet::with_capacity_and_hasher(n, FxBuildHasher);
        for value in black_box(&values) {
            deduplicated.insert(value.as_str());
        }
        black_box(deduplicated);
    });
}

#[divan::bench(args = STRING_SIZES)]
fn vec_string_common_prefix(bencher: Bencher<'_, '_>, n: usize) {
    let values = common_prefix_strings(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = Vec::with_capacity(n);
        for value in black_box(&values) {
            let value = value.as_str();
            if !deduplicated.contains(&value) {
                deduplicated.push(value);
            }
        }
        black_box(deduplicated);
    });
}

#[divan::bench(args = STRING_SIZES)]
fn hashbrown_fx_string_common_prefix(bencher: Bencher<'_, '_>, n: usize) {
    let values = common_prefix_strings(n);

    bencher.counter(ItemsCount::new(n)).bench_local(|| {
        let mut deduplicated = HashSet::with_capacity_and_hasher(n, FxBuildHasher);
        for value in black_box(&values) {
            deduplicated.insert(value.as_str());
        }
        black_box(deduplicated);
    });
}

fn unique_values(n: usize) -> Vec<u64> {
    (0..n)
        .map(|value| {
            // SplitMix64's finalizer gives deterministic, unique-looking keys and keeps
            // the benchmark from favoring either container with sequential input.
            let mut value = (value as u64).wrapping_add(0x9e37_79b9_7f4a_7c15);
            value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            value ^ (value >> 31)
        })
        .collect()
}

fn distributed_strings(n: usize) -> Vec<String> {
    unique_values(n)
        .into_iter()
        .map(|value| format!("{value:016x}{:016x}", value.rotate_left(29)))
        .collect()
}

fn common_prefix_strings(n: usize) -> Vec<String> {
    (0..n)
        .map(|value| format!("aaaaaaaaaaaaaaaaaaaaaaaa{value:08x}"))
        .collect()
}

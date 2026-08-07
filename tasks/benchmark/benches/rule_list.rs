#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::ItemsCount};
use rocketcss_ast::{RuleListId, StyleSheet};
use rocketcss_common::Allocator;

const FLAT_SIZES: &[usize] = &[1_024, 16_384];
const DEEP_SIZES: &[usize] = &[64, 512];
const SPARSE_SIZES: &[usize] = &[1_024, 16_384];
const SPARSE_STRIDE: usize = 64;

fn main() {
    divan::main();
}

fn flat_stylesheet(allocator: &Allocator, len: usize) -> StyleSheet<'_, u8, (), ()> {
    let mut stylesheet = StyleSheet::new_in(allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    for value in 0..len {
        stylesheet.append_rule(root, value as u8).unwrap();
    }
    stylesheet.finalize_parsed_rule_ranges();
    stylesheet
}

fn deep_stylesheet(allocator: &Allocator, depth: usize) -> StyleSheet<'_, u8, (), ()> {
    let mut stylesheet = StyleSheet::new_in(allocator);
    let mut list = stylesheet.stylesheet_root().root_rules();
    for value in 0..depth {
        let rule = stylesheet.append_rule(list, value as u8).unwrap();
        list = stylesheet.create_child_list(rule).unwrap();
    }
    stylesheet.finalize_parsed_rule_ranges();
    stylesheet
}

fn sparse_stylesheet(allocator: &Allocator, len: usize) -> StyleSheet<'_, u8, (), ()> {
    let mut stylesheet = StyleSheet::new_in(allocator);
    let root = stylesheet.stylesheet_root().root_rules();
    let mut anchors = std::vec::Vec::new();
    for value in 0..len {
        let rule = stylesheet.append_rule(root, value as u8).unwrap();
        if value.is_multiple_of(SPARSE_STRIDE) {
            anchors.push(rule);
        }
    }
    stylesheet.finalize_parsed_rule_ranges();
    for (value, anchor) in anchors.into_iter().enumerate() {
        let result = stylesheet
            .insert_rule_after(anchor, !(value as u8))
            .unwrap();
        assert!(result.remaps.is_empty());
    }
    stylesheet
}

fn range_checksum(stylesheet: &StyleSheet<'_, u8, (), ()>, list: RuleListId) -> u64 {
    stylesheet
        .rules_in_list(list)
        .unwrap()
        .fold(0, |checksum, (_, rule)| {
            checksum.rotate_left(7) ^ u64::from(*rule.payload())
        })
}

fn topology_checksum(stylesheet: &StyleSheet<'_, u8, (), ()>, list: RuleListId) -> u64 {
    let mut checksum = 0_u64;
    let mut current = stylesheet.rule_list(list).unwrap().first();
    while let Some(id) = current {
        let rule = stylesheet.rule(id).unwrap();
        checksum = checksum.rotate_left(7) ^ u64::from(*rule.payload());
        current = rule.next_sibling();
    }
    checksum
}

fn range_deep_checksum(stylesheet: &StyleSheet<'_, u8, (), ()>) -> u64 {
    let mut checksum = 0_u64;
    let mut list = stylesheet.stylesheet_root().root_rules();
    loop {
        let Some((_, rule)) = stylesheet.rules_in_list(list).unwrap().next() else {
            return checksum;
        };
        checksum = checksum.rotate_left(7) ^ u64::from(*rule.payload());
        let Some(children) = rule.child_list() else {
            return checksum;
        };
        list = children;
    }
}

fn topology_deep_checksum(stylesheet: &StyleSheet<'_, u8, (), ()>) -> u64 {
    let mut checksum = 0_u64;
    let mut list = stylesheet.stylesheet_root().root_rules();
    loop {
        let Some(id) = stylesheet.rule_list(list).unwrap().first() else {
            return checksum;
        };
        let rule = stylesheet.rule(id).unwrap();
        checksum = checksum.rotate_left(7) ^ u64::from(*rule.payload());
        let Some(children) = rule.child_list() else {
            return checksum;
        };
        list = children;
    }
}

mod flat {
    use super::*;

    #[divan::bench(args = FLAT_SIZES)]
    fn range(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let stylesheet = flat_stylesheet(&allocator, len);
        let root = stylesheet.stylesheet_root().root_rules();
        bencher
            .counter(ItemsCount::new(len))
            .bench_local(|| black_box(range_checksum(black_box(&stylesheet), black_box(root))));
    }

    #[divan::bench(args = FLAT_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let stylesheet = flat_stylesheet(&allocator, len);
        let root = stylesheet.stylesheet_root().root_rules();
        bencher
            .counter(ItemsCount::new(len))
            .bench_local(|| black_box(topology_checksum(black_box(&stylesheet), black_box(root))));
    }
}

mod deep {
    use super::*;

    #[divan::bench(args = DEEP_SIZES)]
    fn range(bencher: Bencher<'_, '_>, depth: usize) {
        let allocator = Allocator::new();
        let stylesheet = deep_stylesheet(&allocator, depth);
        bencher
            .counter(ItemsCount::new(depth))
            .bench_local(|| black_box(range_deep_checksum(black_box(&stylesheet))));
    }

    #[divan::bench(args = DEEP_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, depth: usize) {
        let allocator = Allocator::new();
        let stylesheet = deep_stylesheet(&allocator, depth);
        bencher
            .counter(ItemsCount::new(depth))
            .bench_local(|| black_box(topology_deep_checksum(black_box(&stylesheet))));
    }
}

mod sparse {
    use super::*;

    #[divan::bench(args = SPARSE_SIZES)]
    fn range(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let stylesheet = sparse_stylesheet(&allocator, len);
        let root = stylesheet.stylesheet_root().root_rules();
        let item_count = stylesheet.rule_list(root).unwrap().live_len() as usize;
        bencher
            .counter(ItemsCount::new(item_count))
            .bench_local(|| black_box(range_checksum(black_box(&stylesheet), black_box(root))));
    }

    #[divan::bench(args = SPARSE_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let stylesheet = sparse_stylesheet(&allocator, len);
        let root = stylesheet.stylesheet_root().root_rules();
        let item_count = stylesheet.rule_list(root).unwrap().live_len() as usize;
        bencher
            .counter(ItemsCount::new(item_count))
            .bench_local(|| black_box(topology_checksum(black_box(&stylesheet), black_box(root))));
    }
}

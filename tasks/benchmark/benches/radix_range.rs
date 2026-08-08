#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use divan::{Bencher, black_box, counter::ItemsCount};
use rocketcss_common::{
    Allocator,
    radix_index_arena::{RadixId, RadixRange, RadixRangeItem, TypedRadixIndexArena},
};

const LINEAR_SIZES: &[usize] = &[65_536, 262_144];
const DEEP_SIZES: &[usize] = &[16_384, 65_536];
const SPARSE_STRIDE: usize = 256;
const SIBLING_KEY: u16 = 512;

fn main() {
    divan::main();
}

#[derive(Clone, Copy, Debug)]
struct Node {
    payload: u64,
    next: Option<NodeId>,
    descendants: u32,
}

type NodeId = RadixId<Node>;
type Arena<'arena> = TypedRadixIndexArena<'arena, Node, NodeId>;

impl RadixRangeItem for Node {
    #[inline]
    fn descendants(&self) -> u32 {
        self.descendants
    }
}

struct LinearFixture<'arena> {
    arena: Arena<'arena>,
    range: RadixRange<Node>,
    first: NodeId,
    item_count: usize,
}

struct DeepFixture<'arena> {
    arena: Arena<'arena>,
    ranges: std::vec::Vec<RadixRange<Node>>,
    first: NodeId,
}

fn primary_nodes<'arena>(
    allocator: &'arena Allocator,
    len: usize,
) -> (Arena<'arena>, std::vec::Vec<NodeId>) {
    let mut arena = Arena::with_capacity_in(len, allocator);
    let mut ids = std::vec::Vec::with_capacity(len);
    let mut previous = None;
    for index in 0..len {
        let id = arena.push_primary(Node {
            payload: index as u64,
            next: None,
            descendants: 0,
        });
        if let Some(previous) = previous {
            arena
                .get_mut(previous)
                .expect("a previously appended primary node remains resolvable")
                .next = Some(id);
        }
        previous = Some(id);
        ids.push(id);
    }
    (arena, ids)
}

fn flat_fixture(allocator: &Allocator, len: usize) -> LinearFixture<'_> {
    let (arena, ids) = primary_nodes(allocator, len);
    let first = ids[0];
    let last = ids[len - 1];
    LinearFixture {
        arena,
        range: RadixRange::new(first, last, len as u32),
        first,
        item_count: len,
    }
}

fn sparse_fixture(allocator: &Allocator, len: usize) -> LinearFixture<'_> {
    let (mut arena, ids) = primary_nodes(allocator, len);
    let first = ids[0];
    let last = ids[len - 1];
    let mut inserted = 0_usize;

    for (index, &anchor) in ids.iter().enumerate().step_by(SPARSE_STRIDE) {
        if index + 1 == len {
            break;
        }
        let before = arena
            .get(anchor)
            .expect("a sparse insertion anchor remains resolvable")
            .next;
        let sibling = arena
            .sibling_entry(anchor)
            .expect("a compact primary anchor accepts sibling insertion")
            .try_insert(
                SIBLING_KEY,
                Node {
                    payload: !(index as u64),
                    next: before,
                    descendants: 0,
                },
            )
            .expect("one sibling fits in a fresh sibling group");
        arena
            .get_mut(anchor)
            .expect("a sparse insertion anchor remains mutable")
            .next = Some(sibling);
        inserted += 1;
    }

    LinearFixture {
        arena,
        range: RadixRange::new(first, last, (len + inserted) as u32),
        first,
        item_count: len + inserted,
    }
}

fn deep_fixture(allocator: &Allocator, depth: usize) -> DeepFixture<'_> {
    let mut arena = Arena::with_capacity_in(depth, allocator);
    let mut ids = std::vec::Vec::with_capacity(depth);
    let mut previous = None;
    for index in 0..depth {
        let id = arena.push_primary(Node {
            payload: index as u64,
            next: None,
            descendants: (depth - index - 1) as u32,
        });
        if let Some(previous) = previous {
            arena
                .get_mut(previous)
                .expect("a parent node remains resolvable")
                .next = Some(id);
        }
        previous = Some(id);
        ids.push(id);
    }

    let first = ids[0];
    let last = ids[depth - 1];
    let ranges = ids
        .iter()
        .enumerate()
        .map(|(index, &id)| RadixRange::new(id, last, (depth - index) as u32))
        .collect();
    DeepFixture {
        arena,
        ranges,
        first,
    }
}

#[inline]
fn mix(checksum: u64, id: NodeId, node: &Node) -> u64 {
    checksum.rotate_left(7) ^ node.payload ^ u64::from(id.get())
}

fn range_checksum(arena: &Arena<'_>, range: RadixRange<Node>) -> u64 {
    arena
        .iter_direct_range_enumerated(range)
        .expect("the benchmark range remains resolvable")
        .fold(0, |checksum, (id, node)| mix(checksum, id, node))
}

fn topology_checksum(arena: &Arena<'_>, first: NodeId) -> u64 {
    let mut checksum = 0_u64;
    let mut current = Some(first);
    while let Some(id) = current {
        let node = arena.get(id).expect("a topology link remains resolvable");
        checksum = mix(checksum, id, node);
        current = node.next;
    }
    checksum
}

fn range_deep_checksum(fixture: &DeepFixture<'_>) -> u64 {
    fixture.ranges.iter().fold(0, |checksum, &range| {
        let (id, node) = fixture
            .arena
            .iter_direct_range_enumerated(range)
            .expect("a nested benchmark range remains resolvable")
            .next()
            .expect("a nested benchmark range is non-empty");
        mix(checksum, id, node)
    })
}

mod flat {
    use super::*;

    #[divan::bench(args = LINEAR_SIZES)]
    fn range(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let fixture = flat_fixture(&allocator, len);
        bencher
            .counter(ItemsCount::new(fixture.item_count))
            .bench_local(|| {
                black_box(range_checksum(
                    black_box(&fixture.arena),
                    black_box(fixture.range),
                ))
            });
    }

    #[divan::bench(args = LINEAR_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let fixture = flat_fixture(&allocator, len);
        bencher
            .counter(ItemsCount::new(fixture.item_count))
            .bench_local(|| {
                black_box(topology_checksum(
                    black_box(&fixture.arena),
                    black_box(fixture.first),
                ))
            });
    }
}

mod sparse {
    use super::*;

    #[divan::bench(args = LINEAR_SIZES)]
    fn range(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let fixture = sparse_fixture(&allocator, len);
        bencher
            .counter(ItemsCount::new(fixture.item_count))
            .bench_local(|| {
                black_box(range_checksum(
                    black_box(&fixture.arena),
                    black_box(fixture.range),
                ))
            });
    }

    #[divan::bench(args = LINEAR_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, len: usize) {
        let allocator = Allocator::new();
        let fixture = sparse_fixture(&allocator, len);
        bencher
            .counter(ItemsCount::new(fixture.item_count))
            .bench_local(|| {
                black_box(topology_checksum(
                    black_box(&fixture.arena),
                    black_box(fixture.first),
                ))
            });
    }
}

mod deep {
    use super::*;

    #[divan::bench(args = DEEP_SIZES)]
    fn range(bencher: Bencher<'_, '_>, depth: usize) {
        let allocator = Allocator::new();
        let fixture = deep_fixture(&allocator, depth);
        bencher
            .counter(ItemsCount::new(depth))
            .bench_local(|| black_box(range_deep_checksum(black_box(&fixture))));
    }

    #[divan::bench(args = DEEP_SIZES)]
    fn topology(bencher: Bencher<'_, '_>, depth: usize) {
        let allocator = Allocator::new();
        let fixture = deep_fixture(&allocator, depth);
        bencher.counter(ItemsCount::new(depth)).bench_local(|| {
            black_box(topology_checksum(
                black_box(&fixture.arena),
                black_box(fixture.first),
            ))
        });
    }
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::fmt;

use bumpalo::Bump;
use divan::{Bencher, black_box, counter::ItemsCount};
use rocketcss_allocator::{Allocator, hash_map::HashMap, hash_set::HashSet, vec::Vec as ArenaVec};
use rocketcss_benchmark::{BENCH_CASES, BenchCase};
use rocketcss_parser::{Token, Tokenizer};

fn main() {
    assert_eq!(size_of::<IndexAtom>(), 4);
    assert_eq!(size_of::<PointerAtom<'_>>(), 16);
    divan::main();
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
struct IndexAtom(u32);

#[derive(Clone, Copy)]
#[repr(transparent)]
struct PointerAtom<'a>(&'a str);

impl PartialEq for PointerAtom<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for PointerAtom<'_> {}

/// Stores one copy of each string in a Vec and uses its index as the Atom.
struct IndexStringPool<'a> {
    arena: &'a Bump,
    strings: ArenaVec<'a, &'a str>,
    indices: HashMap<'a, &'a str, u32>,
}

impl<'a> IndexStringPool<'a> {
    fn new(arena: &'a Bump, allocator: &'a Allocator) -> Self {
        Self {
            arena,
            strings: allocator.vec(),
            indices: HashMap::new_in(allocator),
        }
    }

    #[inline]
    fn intern(&mut self, value: &str) -> IndexAtom {
        if let Some(&index) = self.indices.get(value) {
            return IndexAtom(index);
        }

        let index = u32::try_from(self.strings.len()).expect("string pool exceeds u32 capacity");
        let stored = self.arena.alloc_str(value);
        self.strings.push(stored);
        self.indices.insert(stored, index);
        IndexAtom(index)
    }

    #[inline]
    fn resolve(&self, atom: IndexAtom) -> &'a str {
        self.strings[atom.0 as usize]
    }
}

/// Stores interned strings directly in a set and uses their stable arena
/// addresses as Atoms, so no side Vec is needed for resolution.
struct PointerStringPool<'a> {
    arena: &'a Bump,
    strings: HashSet<'a, &'a str>,
}

impl<'a> PointerStringPool<'a> {
    fn new(arena: &'a Bump, allocator: &'a Allocator) -> Self {
        Self {
            arena,
            strings: HashSet::new_in(allocator),
        }
    }

    #[inline]
    fn intern(&mut self, value: &str) -> PointerAtom<'a> {
        if let Some(&stored) = self.strings.get(value) {
            return PointerAtom(stored);
        }

        let stored = self.arena.alloc_str(value);
        self.strings.insert(stored);
        PointerAtom(stored)
    }
}

#[divan::bench(args = BENCH_CASES)]
fn intern_index_u32(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    bencher
        .counter(ItemsCount::new(values.len()))
        .bench_local(|| {
            let allocator = Allocator::new();
            let arena = Bump::new();
            let mut pool = IndexStringPool::new(&arena, &allocator);
            let mut atoms = ArenaVec::with_capacity_in(values.len(), &allocator);
            for value in black_box(&values) {
                atoms.push(pool.intern(black_box(value)));
            }
            black_box(atoms);
            black_box(pool.strings.len());
        });
}

#[divan::bench(args = BENCH_CASES)]
fn intern_pointer(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    bencher
        .counter(ItemsCount::new(values.len()))
        .bench_local(|| {
            let allocator = Allocator::new();
            let arena = Bump::new();
            let mut pool = PointerStringPool::new(&arena, &allocator);
            let mut atoms = ArenaVec::with_capacity_in(values.len(), &allocator);
            for value in black_box(&values) {
                atoms.push(pool.intern(black_box(value)));
            }
            black_box(atoms);
            black_box(pool.strings.len());
        });
}

#[divan::bench(args = BENCH_CASES)]
fn resolve_index_u32(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    let allocator = Allocator::new();
    let arena = Bump::new();
    let mut pool = IndexStringPool::new(&arena, &allocator);
    let atoms = values
        .iter()
        .map(|value| pool.intern(value))
        .collect::<std::vec::Vec<_>>();

    bencher
        .counter(ItemsCount::new(atoms.len()))
        .bench_local(|| {
            let mut total_bytes = 0usize;
            for &atom in black_box(&atoms) {
                total_bytes = total_bytes.wrapping_add(black_box(pool.resolve(atom)).len());
            }
            black_box(total_bytes);
        });
}

#[divan::bench(args = BENCH_CASES)]
fn resolve_pointer(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    let allocator = Allocator::new();
    let arena = Bump::new();
    let mut pool = PointerStringPool::new(&arena, &allocator);
    let atoms = values
        .iter()
        .map(|value| pool.intern(value))
        .collect::<std::vec::Vec<_>>();

    bencher
        .counter(ItemsCount::new(atoms.len()))
        .bench_local(|| {
            let mut total_bytes = 0usize;
            for &atom in black_box(&atoms) {
                total_bytes = total_bytes.wrapping_add(black_box(atom.0).len());
            }
            black_box(total_bytes);
        });
}

#[divan::bench(args = BENCH_CASES)]
fn compare_index_u32(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    let allocator = Allocator::new();
    let arena = Bump::new();
    let mut pool = IndexStringPool::new(&arena, &allocator);
    let atoms = values
        .iter()
        .map(|value| pool.intern(value))
        .collect::<std::vec::Vec<_>>();

    bencher
        .counter(ItemsCount::new(atoms.len().saturating_sub(1)))
        .bench_local(|| compare_adjacent(black_box(&atoms)));
}

#[divan::bench(args = BENCH_CASES)]
fn compare_pointer(bencher: Bencher<'_, '_>, case: BenchCase) {
    let values = string_tokens(case.source);
    let allocator = Allocator::new();
    let arena = Bump::new();
    let mut pool = PointerStringPool::new(&arena, &allocator);
    let atoms = values
        .iter()
        .map(|value| pool.intern(value))
        .collect::<std::vec::Vec<_>>();

    bencher
        .counter(ItemsCount::new(atoms.len().saturating_sub(1)))
        .bench_local(|| compare_adjacent(black_box(&atoms)));
}

#[inline(never)]
fn compare_adjacent<T: Eq>(atoms: &[T]) {
    let mut equal = 0usize;
    for pair in atoms.windows(2) {
        equal += black_box(pair[0] == pair[1]) as usize;
    }
    black_box(equal);
}

fn string_tokens(source: &str) -> std::vec::Vec<&str> {
    let mut tokenizer = Tokenizer::new(source);
    let mut values = std::vec::Vec::new();
    while let Ok(token) = tokenizer.next() {
        if is_string_token(token.token) {
            values.push(&source[token.span.start as usize..token.span.end as usize]);
        }
    }
    values
}

fn is_string_token(token: Token) -> bool {
    matches!(
        token,
        Token::Ident
            | Token::AtKeyword
            | Token::Hash
            | Token::IDHash
            | Token::QuotedString
            | Token::UnquotedUrl
            | Token::Delim
            | Token::Dimension
            | Token::WhiteSpace
            | Token::Comment
            | Token::Function
            | Token::BadUrl
            | Token::BadString
    )
}

impl fmt::Debug for IndexAtom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Debug for PointerAtom<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

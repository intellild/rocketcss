//! Compares two designs for merging declaration blocks during minification.
//!
//! `rebuild_each_merge` models rebuilding a compact declaration object after
//! every merge. `block_links` keeps declaration blocks at stable addresses,
//! records the latest occurrence of each longhand in an IR map as a block
//! pointer plus an index, and marks replaced declarations with tombstones.
//! Codegen follows the block links and folds the surviving declarations into
//! the final output.
//!
//! The benchmark measures both minification alone and minification followed by
//! codegen to verify which design performs better after deferred work is
//! included, rather than comparing only the cost moved out of minification.

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::fmt;
use std::ptr::NonNull;

use divan::{Bencher, black_box, counter::ItemsCount};

const NONE: usize = usize::MAX;
const SIDES: usize = 4;

const CASES: &[MergeCase] = &[
    MergeCase::new(2, 1, 1),
    MergeCase::new(16, 1, 1),
    MergeCase::new(1, 8, 8),
    MergeCase::new(4, 8, 16),
    MergeCase::new(16, 8, 16),
    MergeCase::new(64, 8, 16),
    MergeCase::new(256, 8, 16),
    MergeCase::new(4, 8, 32),
    MergeCase::new(16, 8, 128),
    MergeCase::new(64, 8, 512),
    MergeCase::new(256, 8, 2048),
];

fn main() {
    divan::main();
}

#[derive(Clone, Copy)]
struct MergeCase {
    rules: usize,
    declarations_per_rule: usize,
    families: usize,
}

impl MergeCase {
    const fn new(rules: usize, declarations_per_rule: usize, families: usize) -> Self {
        Self {
            rules,
            declarations_per_rule,
            families,
        }
    }

    const fn declaration_count(self) -> usize {
        self.rules * self.declarations_per_rule
    }
}

impl fmt::Display for MergeCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}rules-{}decls-{}families",
            self.rules, self.declarations_per_rule, self.families
        )
    }
}

#[derive(Clone, Copy)]
struct SourceDeclaration {
    family: usize,
    side: usize,
    value: u64,
}

impl SourceDeclaration {
    fn longhand_key(self) -> usize {
        self.family * SIDES + self.side
    }
}

#[derive(Clone, Copy, Default)]
struct MergedValue {
    present: u8,
    sides: [u64; SIDES],
}

impl MergedValue {
    fn from_source(declaration: SourceDeclaration) -> Self {
        let mut value = Self::default();
        value.apply(declaration);
        value
    }

    fn apply(&mut self, declaration: SourceDeclaration) {
        self.present |= 1 << declaration.side;
        self.sides[declaration.side] = declaration.value;
    }
}

#[derive(Clone, Copy)]
struct MergedDeclaration {
    family: usize,
    value: MergedValue,
}

/// Models Lightning CSS's DeclarationHandler: every merged rule allocates a
/// new compact declaration vector. Replaced vectors stay alive until after the
/// timed region to model arena allocation.
#[divan::bench(args = CASES)]
fn rebuild_each_merge(bencher: Bencher<'_, '_>, case: MergeCase) {
    verify(case);

    bencher
        .counter(ItemsCount::new(case.declaration_count()))
        .with_inputs(|| RebuildInput::new(case))
        .bench_local_refs(RebuildInput::minify);
}

/// Minification only creates links between stable DeclarationBlock boxes and
/// tombstones overridden longhands. The declaration IR stores a block pointer
/// and an index, never a pointer into a declaration vector.
#[divan::bench(args = CASES)]
fn block_links(bencher: Bencher<'_, '_>, case: MergeCase) {
    verify(case);

    bencher
        .counter(ItemsCount::new(case.declaration_count()))
        .with_inputs(|| BlockLinkInput::new(case))
        .bench_local_refs(|input| {
            input.minify();
            black_box((input.live_declarations(), input.valid_rules()))
        });
}

#[divan::bench(args = CASES)]
fn rebuild_each_merge_and_emit(bencher: Bencher<'_, '_>, case: MergeCase) {
    verify(case);

    bencher
        .counter(ItemsCount::new(case.declaration_count()))
        .with_inputs(|| RebuildInput::new(case))
        .bench_local_refs(|input| {
            let output = input.minify();
            let emitted = output.emit();
            black_box(emitted);
            (output, emitted)
        });
}

/// Codegen starts at the final live block, follows `previous` pointers, and
/// folds declarations from the oldest block to the newest.
#[divan::bench(args = CASES)]
fn block_links_and_emit(bencher: Bencher<'_, '_>, case: MergeCase) {
    verify(case);

    bencher
        .counter(ItemsCount::new(case.declaration_count()))
        .with_inputs(|| BlockLinkInput::new(case))
        .bench_local_refs(|input| {
            input.minify();
            black_box(input.emit())
        });
}

struct RebuildInput {
    blocks: Vec<Vec<SourceDeclaration>>,
    last_declaration: Vec<usize>,
}

impl RebuildInput {
    fn new(case: MergeCase) -> Self {
        Self {
            blocks: source_blocks(case),
            last_declaration: vec![NONE; case.families],
        }
    }

    fn minify(&mut self) -> RebuildOutput {
        let mut declarations: Vec<MergedDeclaration> = Vec::new();
        let mut discarded = Vec::with_capacity(self.blocks.len());

        for block in &self.blocks {
            self.last_declaration.fill(NONE);
            let mut output = Vec::with_capacity(declarations.len() + block.len());

            for declaration in &declarations {
                self.last_declaration[declaration.family] = output.len();
                output.push(*declaration);
            }
            for &declaration in block {
                let previous = self.last_declaration[declaration.family];
                if previous == NONE {
                    self.last_declaration[declaration.family] = output.len();
                    output.push(MergedDeclaration {
                        family: declaration.family,
                        value: MergedValue::from_source(declaration),
                    });
                } else {
                    output[previous].value.apply(declaration);
                }
            }

            discarded.push(declarations);
            declarations = output;
        }

        RebuildOutput {
            declarations,
            discarded,
        }
    }
}

struct RebuildOutput {
    declarations: Vec<MergedDeclaration>,
    discarded: Vec<Vec<MergedDeclaration>>,
}

impl RebuildOutput {
    fn emit(&self) -> (usize, u64) {
        black_box(self.discarded.len());
        emit_declarations(
            self.declarations
                .iter()
                .map(|declaration| (declaration.family, declaration.value)),
        )
    }
}

// Each block needs a stable address because other blocks and the longhand
// table retain pointers to it.
#[allow(clippy::vec_box)]
struct BlockLinkInput {
    blocks: Vec<Box<DeclarationBlock>>,
    declarations_ir: Vec<Option<DeclarationLocation>>,
    last_rule: Option<NonNull<DeclarationBlock>>,
    codegen_chain: Vec<NonNull<DeclarationBlock>>,
    codegen_values: Vec<MergedValue>,
    codegen_present: Vec<bool>,
}

impl BlockLinkInput {
    fn new(case: MergeCase) -> Self {
        Self {
            blocks: source_blocks(case)
                .into_iter()
                .map(|declarations| Box::new(DeclarationBlock::new(declarations)))
                .collect(),
            declarations_ir: vec![None; case.families * SIDES],
            last_rule: None,
            codegen_chain: Vec::with_capacity(case.rules),
            codegen_values: vec![MergedValue::default(); case.families],
            codegen_present: vec![false; case.families],
        }
    }

    fn minify(&mut self) {
        for block_index in 0..self.blocks.len() {
            let block_pointer = NonNull::from(self.blocks[block_index].as_mut());
            let declaration_count = self.blocks[block_index].declarations.len();

            unsafe {
                (*block_pointer.as_ptr()).previous = self.last_rule;
                if let Some(mut previous) = self.last_rule {
                    previous.as_mut().rule_invalid = true;
                }
            }

            for declaration_index in 0..declaration_count {
                let declaration =
                    unsafe { (&(*block_pointer.as_ptr()).declarations)[declaration_index] };
                let key = declaration.longhand_key();

                if let Some(previous) = self.declarations_ir[key] {
                    unsafe {
                        let previous_block = &mut *previous.block.as_ptr();
                        debug_assert!(!previous_block.invalid.get(previous.index));
                        debug_assert_eq!(
                            previous_block.declarations[previous.index].longhand_key(),
                            key
                        );
                        previous_block.invalid.set(previous.index);
                        previous_block.live -= 1;
                    }
                }

                self.declarations_ir[key] = Some(DeclarationLocation {
                    block: block_pointer,
                    index: declaration_index,
                });
            }

            self.last_rule = Some(block_pointer);
        }
    }

    fn live_declarations(&self) -> usize {
        self.blocks.iter().map(|block| block.live).sum()
    }

    fn valid_rules(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| !block.rule_invalid)
            .count()
    }

    fn emit(&mut self) -> (usize, u64) {
        self.codegen_chain.clear();
        let mut block = self.last_rule;
        while let Some(block_pointer) = block {
            self.codegen_chain.push(block_pointer);
            block = unsafe { block_pointer.as_ref().previous };
        }

        self.codegen_values.fill(MergedValue::default());
        self.codegen_present.fill(false);

        for block_pointer in self.codegen_chain.iter().rev() {
            let block = unsafe { block_pointer.as_ref() };
            for (index, &declaration) in block.declarations.iter().enumerate() {
                if block.invalid.get(index) {
                    continue;
                }
                self.codegen_present[declaration.family] = true;
                self.codegen_values[declaration.family].apply(declaration);
            }
        }

        emit_declarations(
            self.codegen_values
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(family, value)| {
                    self.codegen_present[family].then_some((family, value))
                }),
        )
    }
}

#[derive(Clone, Copy)]
struct DeclarationLocation {
    block: NonNull<DeclarationBlock>,
    index: usize,
}

struct DeclarationBlock {
    declarations: Vec<SourceDeclaration>,
    invalid: InvalidBits,
    live: usize,
    previous: Option<NonNull<DeclarationBlock>>,
    rule_invalid: bool,
}

impl DeclarationBlock {
    fn new(declarations: Vec<SourceDeclaration>) -> Self {
        let live = declarations.len();
        Self {
            declarations,
            invalid: InvalidBits::with_len(live),
            live,
            previous: None,
            rule_invalid: false,
        }
    }
}

struct InvalidBits {
    len: usize,
    words: Vec<usize>,
}

impl InvalidBits {
    fn with_len(len: usize) -> Self {
        Self {
            len,
            words: vec![0; len.div_ceil(usize::BITS as usize)],
        }
    }

    fn set(&mut self, index: usize) {
        debug_assert!(index < self.len);
        let bits = usize::BITS as usize;
        self.words[index / bits] |= 1 << (index % bits);
    }

    fn get(&self, index: usize) -> bool {
        debug_assert!(index < self.len);
        let bits = usize::BITS as usize;
        self.words[index / bits] & (1 << (index % bits)) != 0
    }
}

fn emit_declarations(declarations: impl Iterator<Item = (usize, MergedValue)>) -> (usize, u64) {
    declarations.fold((0, 0u64), |(count, checksum), (family, value)| {
        let value_checksum = value
            .sides
            .into_iter()
            .enumerate()
            .filter(|(side, _)| value.present & (1 << side) != 0)
            .fold(0u64, |checksum, (side, value)| {
                checksum.wrapping_add(value.rotate_left(side as u32))
            });
        (
            count + 1,
            checksum
                .wrapping_add(value_checksum ^ (family as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)),
        )
    })
}

fn source_blocks(case: MergeCase) -> Vec<Vec<SourceDeclaration>> {
    (0..case.rules)
        .map(|rule| {
            (0..case.declarations_per_rule)
                .map(|offset| {
                    let index = rule * case.declarations_per_rule + offset;
                    SourceDeclaration {
                        family: (index * 17) % case.families,
                        side: (index / case.families) % SIDES,
                        value: splitmix64(index as u64),
                    }
                })
                .collect()
        })
        .collect()
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn verify(case: MergeCase) {
    let mut rebuilt = RebuildInput::new(case);
    let rebuilt = rebuilt.minify();
    let mut links = BlockLinkInput::new(case);
    links.minify();
    assert_eq!(rebuilt.emit(), links.emit());
}

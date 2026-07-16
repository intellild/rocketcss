# Adjacent Declaration Block Merge Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Merge physically adjacent, structurally identical style-rule selectors by treating their declaration blocks as one logical declaration sequence while preserving nested-rule order.

**Architecture:** Add a backward `Ref` link to `DeclarationBlock`, generalize the existing declaration IR from block-local indices to logical declaration locations, and add a stylesheet-list pass that forms eligible same-selector runs. Earlier rule owners become selector tombstones; codegen follows the tail block's backward chain and emits all live declarations in source order.

**Tech Stack:** Rust 2024, RocketCSS arena AST, `rocketcss_allocator::Ref`, generated visitor API, existing minifier IR, fixture test harness.

## Global Constraints

- Reuse the single-block declaration IR for multi-block sequences; do not add a separate cross-block deduplicator.
- Only merge physically adjacent style-rule siblings with structurally equal live selectors and equal vendor prefixes.
- Do not merge different selectors with equal declarations, partially overlapping selector lists, or non-adjacent rules.
- A preceding style rule with nested children is a forward merge barrier; a following style rule may be the live tail and retain its nested children.
- Never cross a `NestedDeclarations`, nested style, or nested at-rule ordering boundary.
- Visitors and structural declaration-block equality ignore the backward merge link.
- Enable the existing cssnano `discard-duplicates/declarations` and `discard-duplicates/partial` fixtures; keep Lightning CSS `merge-selectors` disabled.
- Add an enabled-by-default option and cover both its enabled and disabled paths.
- Use tombstones and stable references; do not move blocks or allocate replacement AST nodes.
- Do not rebase or amend commits.

---

### Task 1: Stable declaration links and chain-aware code generation

**Files:**
- Modify: `crates/allocator/src/reference.rs`
- Modify: `crates/ast/src/rules/stylesheet.rs`
- Modify: `crates/codegen/src/rules.rs`
- Test: `crates/allocator/src/reference.rs`
- Test: `crates/codegen/tests/to_css.rs`

**Interfaces:**
- Produces: `Ref::from_pinned_box(&Pin<Box<'a, T>>) -> Ref<'a, T>`
- Produces: `DeclarationBlock::previous_merged() -> Option<Ref<'a, DeclarationBlock<'a>>>`
- Produces: `DeclarationBlock::set_previous_merged(Pin<&mut Self>, Option<Ref<'a, DeclarationBlock<'a>>>)`
- Produces: chain-aware declaration serialization used by every `DeclarationBlock` and `StyleRule` codegen path.

- [ ] **Step 1: Add a failing codegen regression test that manually links two parsed blocks**

Add this test to `crates/codegen/tests/to_css.rs` using the file's existing imports and helpers:

```rust
#[test]
fn merged_declaration_blocks_serialize_from_chain_head() {
    let allocator = Allocator::new();
    let mut stylesheet = parse(
        "a{width:1px}a{height:2px}",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let [CssRule::Style(first), CssRule::Style(second)] = &mut stylesheet.rules[..] else {
        panic!("expected two style rules")
    };
    let previous = Ref::from_pinned_box(&first.declarations);
    second
        .declarations
        .as_mut()
        .set_previous_merged(Some(previous));
    for selector in &mut first.selectors {
        *selector = Selector::Tombstone;
    }

    assert_eq!(
        stylesheet
            .to_css_string(PrinterOptions { prettify: false })
            .unwrap(),
        "a{width:1px;height:2px}"
    );
}
```

Import `rocketcss_allocator::Ref`, `rocketcss_ast::{CssRule, Selector}`, and `rocketcss_parser::{ParserOptions, parse}` if they are not already imported.

- [ ] **Step 2: Run the test and verify RED**

Run:

```bash
cargo test -p rocketcss_codegen --test to_css merged_declaration_blocks_serialize_from_chain_head -- --exact
```

Expected: compilation fails because `Ref::from_pinned_box`, `DeclarationBlock::set_previous_merged`, and chain-aware output do not exist.

- [ ] **Step 3: Add an arena-lifetime constructor for pinned allocator boxes**

In `crates/allocator/src/reference.rs`, add:

```rust
#[inline]
pub fn from_pinned_box(value: &Pin<crate::boxed::Box<'a, T>>) -> Self {
    Self {
        pointer: NonNull::from(value.as_ref().get_ref()),
        marker: PhantomData,
    }
}
```

Extend the module test so the returned handle remains usable after the short borrow passed to `from_pinned_box` ends:

```rust
#[test]
fn borrows_a_pinned_arena_box_for_the_arena_lifetime() {
    let allocator = Allocator::new();
    let value = allocator.pinned(PinnedValue {
        value: 42,
        _pin: PhantomPinned,
    });
    let reference = Ref::from_pinned_box(&value);

    assert_eq!(reference.get().value, 42);
}
```

- [ ] **Step 4: Add the backward link without making it semantic AST state**

In `crates/ast/src/rules/stylesheet.rs`, remove `PartialEq` from the `DeclarationBlock` derive and add the skipped field:

```rust
#[derive(Debug, Visit)]
#[visit(pinned)]
pub struct DeclarationBlock<'a> {
    pub declarations: Vec<'a, Declaration<'a>>,
    #[visit(skip)]
    pub declarations_importance: BitVec<'a>,
    #[visit(skip)]
    previous_merged: Option<rocketcss_allocator::Ref<'a, DeclarationBlock<'a>>>,
    #[visit(skip)]
    _pin: PhantomPinned,
}
```

Initialize it and expose pinned mutation:

```rust
previous_merged: None,
```

```rust
#[inline]
pub fn previous_merged(
    &self,
) -> Option<rocketcss_allocator::Ref<'a, DeclarationBlock<'a>>> {
    self.previous_merged
}

#[inline]
pub fn set_previous_merged(
    mut self: Pin<&mut Self>,
    previous: Option<rocketcss_allocator::Ref<'a, DeclarationBlock<'a>>>,
) {
    // SAFETY: assigning a pointer-sized field does not move the pinned block.
    unsafe { self.as_mut().get_unchecked_mut() }.previous_merged = previous;
}
```

Implement `PartialEq` using only declarations and importance:

```rust
impl PartialEq for DeclarationBlock<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.declarations == other.declarations
            && self.declarations_importance == other.declarations_importance
    }
}
```

- [ ] **Step 5: Serialize the complete backward chain iteratively**

In `crates/codegen/src/rules.rs`, replace direct block serialization with a helper shaped as follows:

```rust
fn write_declaration_chain<PrinterT: PrinterTrait>(
    tail: &DeclarationBlock<'_>,
    dest: &mut PrinterT,
    last_semicolon: LastSemicolon,
) -> fmt::Result {
    let mut blocks = std::vec::Vec::new();
    let mut current = tail;
    blocks.push(current);
    while let Some(previous) = current.previous_merged() {
        current = previous.get().get_ref();
        blocks.push(current);
    }

    let mut live = blocks
        .into_iter()
        .rev()
        .filter(|block| !block.is_output_empty())
        .peekable();
    while let Some(block) = live.next() {
        let has_next_block = live.peek().is_some();
        write_declarations(
            block,
            dest,
            if has_next_block {
                LastSemicolon::Required
            } else {
                last_semicolon
            },
        )?;
        if has_next_block {
            dest.new_line()?;
        }
    }
    Ok(())
}
```

Make `DeclarationBlock::to_css`, `write_declaration_block`, and `StyleRule::to_css` call `write_declaration_chain`. In `StyleRule::to_css`, calculate the declaration/nested-rule separator from whether any block in the chain has live declarations.

Before whitespace handling in `write_rule_list`, skip a style rule whose selectors are all tombstones:

```rust
if matches!(rule, CssRule::Style(style)
    if style.selectors.iter().all(Selector::is_tombstone))
{
    continue;
}
```

- [ ] **Step 6: Run focused tests and verify GREEN**

Run:

```bash
cargo fmt --all
cargo test -p rocketcss_allocator reference
cargo test -p rocketcss_codegen --test to_css merged_declaration_blocks_serialize_from_chain_head -- --exact
cargo test -p rocketcss_ast
cargo test -p rocketcss_codegen
```

Expected: all commands pass and compact output contains one style rule with `width` before `height`.

- [ ] **Step 7: Commit the stable-link slice**

```bash
git add crates/allocator/src/reference.rs crates/ast/src/rules/stylesheet.rs crates/codegen/src/rules.rs crates/codegen/tests/to_css.rs
git commit -m "feat(ast): link merged declaration blocks"
```

---

### Task 2: Shared multi-block declaration IR and adjacent-rule pass

**Files:**
- Modify: `tests/src/minify.rs`
- Modify: `crates/minify/src/options.rs`
- Modify: `crates/minify/src/rules/stylesheet/declaration_block.rs`
- Create: `crates/minify/src/rules/stylesheet/adjacent.rs`
- Modify: `crates/minify/src/rules/stylesheet/mod.rs`
- Modify: `crates/minify/src/lib.rs`
- Test: `tests/fixtures/minify/cssnano/discard-duplicates/declarations/*`
- Test: `tests/fixtures/minify/cssnano/discard-duplicates/partial/*`
- Test: `crates/minify/src/lib.rs`

**Interfaces:**
- Consumes: `Ref::from_pinned_box`, `DeclarationBlock::set_previous_merged`, and chain-aware codegen from Task 1.
- Produces: packed `DeclarationLocation` values understood by the existing exact-declaration and box-family IR.
- Produces: `DeclarationBlockMinifier::minify_sequence(&mut [Ref<'ast, DeclarationBlock<'ast>>], &mut MinifyContext)`.
- Produces: `merge_adjacent_style_rules(&mut Vec<'ast, CssRule<'ast>>, &mut DeclarationBlockMinifier, &mut MinifyContext)`.

- [ ] **Step 1: Enable the existing cssnano fixtures**

In `tests/src/minify.rs`, remove only these entries from `unsupported_cases`:

```rust
"/cssnano/discard-duplicates/declarations/",
"/cssnano/discard-duplicates/partial/",
```

Keep this entry unchanged because it is outside scope:

```rust
"/lightningcss/rules/merge-selectors/",
```

- [ ] **Step 2: Add focused failing tests for one logical declaration sequence**

In the existing `#[cfg(test)]` module in `crates/minify/src/lib.rs`, add:

```rust
#[test]
fn merges_adjacent_equal_selector_declaration_blocks() {
    assert_eq!(
        run("h1{color:red;background:blue}h1{color:red}"),
        "h1{background:#00f;color:red}"
    );
    assert_eq!(
        run("a{width:1px}a{height:2px}a{opacity:.5}"),
        "a{width:1px;height:2px;opacity:.5}"
    );
}

#[test]
fn runs_box_ir_across_adjacent_blocks() {
    assert_eq!(
        run("a{margin-top:1px;margin-right:2px}a{margin-bottom:3px;margin-left:4px}"),
        "a{margin:1px 2px 3px 4px}"
    );
    assert_eq!(
        run("a{padding:1px}a{padding-left:2px}"),
        "a{padding:1px 1px 1px 2px}"
    );
}

#[test]
fn preserves_cross_block_fallbacks_and_importance() {
    assert_eq!(
        run("a{width:1px}a{width:2px}a{width:1px}"),
        "a{width:1px;width:2px;width:1px}"
    );
    assert_eq!(
        run("a{color:red!important}a{color:blue}"),
        "a{color:red !important;color:#00f}"
    );
}
```

- [ ] **Step 3: Run the existing and focused tests and verify RED**

Run:

```bash
cargo test -p rocketcss_tests --test fixtures minify::minifies_upstream_fixtures -- --exact --nocapture
cargo test -p rocketcss_minify merges_adjacent_equal_selector_declaration_blocks
cargo test -p rocketcss_minify runs_box_ir_across_adjacent_blocks
```

Expected: assertions fail because adjacent rules still serialize separately. The fixture failure must name one of the newly enabled cssnano paths.

- [ ] **Step 4: Add an enabled-by-default adjacent merge option**

In `crates/minify/src/options.rs`, add:

```rust
/// Merge physically adjacent style rules with equal selectors.
const MERGE_ADJACENT_RULES = 1 << 12;
```

Include `Options::MERGE_ADJACENT_RULES` in `MinifyOptions::default().flags`.

- [ ] **Step 5: Replace block-local IR indices with packed logical locations**

In `crates/minify/src/rules/stylesheet/declaration_block.rs`, replace `EMPTY_INDEX` and IR value types with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct DeclarationLocation(u64);

impl DeclarationLocation {
    const EMPTY: Self = Self(u64::MAX);

    #[inline]
    fn new(block: usize, declaration: usize) -> Self {
        Self(((block as u64) << 32) | declaration as u64)
    }

    #[inline]
    fn block(self) -> usize {
        (self.0 >> 32) as usize
    }

    #[inline]
    fn declaration(self) -> usize {
        self.0 as u32 as usize
    }
}
```

Change `DeclarationMap` values, `BoxFamilyIr::pending_longhands`, `sides`, and `shorthand` from `u32` to `DeclarationLocation`. Continue using the same packed declaration keys; only stored values change.

- [ ] **Step 6: Introduce one access abstraction for single and linked sequences**

Add a private sequence representation in `declaration_block.rs`:

```rust
enum DeclarationBlocks<'sequence, 'ast> {
    Single(&'sequence mut DeclarationBlock<'ast>),
    Linked(&'sequence mut [rocketcss_allocator::Ref<'ast, DeclarationBlock<'ast>>]),
}

struct DeclarationSequence<'sequence, 'ast> {
    blocks: DeclarationBlocks<'sequence, 'ast>,
}
```

Give it complete accessors used by every IR algorithm:

```rust
fn block(&self, index: usize) -> &DeclarationBlock<'ast>;
fn block_mut(&mut self, index: usize) -> &mut DeclarationBlock<'ast>;
fn declaration(&self, location: DeclarationLocation) -> &Declaration<'ast>;
fn declaration_mut(&mut self, location: DeclarationLocation) -> &mut Declaration<'ast>;
fn replace(&mut self, location: DeclarationLocation, value: Declaration<'ast>) -> Declaration<'ast>;
fn is_important(&self, location: DeclarationLocation) -> bool;
fn locations(&self) -> impl Iterator<Item = DeclarationLocation>;
```

For `Linked`, copy the selected `Ref`, use `get()` for immutable access, and use `get_mut()` plus `Pin::get_unchecked_mut` inside `block_mut`. Document the invariant that the slice contains unique blocks from one live sibling run.

Make `DeclarationBlockMinifier::minify` wrap one block in `DeclarationSequence::single`. Add:

```rust
pub(crate) fn minify_sequence(
    &mut self,
    blocks: &mut [rocketcss_allocator::Ref<'ast, DeclarationBlock<'ast>>],
    cx: &mut MinifyContext<'scratch>,
) {
    let mut sequence = DeclarationSequence::linked(blocks);
    self.ir.clear();
    deduplicate_declarations(&mut sequence, &mut self.ir, cx);
}
```

- [ ] **Step 7: Convert every declaration algorithm to sequence locations**

Change `deduplicate_declarations`, `deduplicate_exact_declaration`, `process_box_declaration`, `fold_box_side_override`, `merge_box_longhands`, `merge_typed_box_longhands`, `merge_unparsed_box_longhands`, `record_merged_longhands`, and `minify_unparsed_declaration` to accept `&mut DeclarationSequence` plus `DeclarationLocation` values.

Preserve current behavior by replacing direct indexing as follows:

```rust
let declaration = sequence.declaration(current);
let previous = declarations.insert_known(
    property_id,
    vendor_prefix,
    important,
    current,
);
```

```rust
if !sequence.declaration(previous).is_tombstone()
    && sequence
        .declaration(previous)
        .eq_ignoring_tombstones(sequence.declaration(current))
{
    sequence.replace(previous, Declaration::Tombstone);
    cx.record_declaration_removed();
}
```

For two-location shorthand folding, temporarily replace the later longhand with a tombstone, mutate the earlier shorthand, and restore the longhand if folding is rejected. For four-side merging, extract the four declarations by location, construct the shorthand in the latest location, and leave the other three locations as tombstones. This avoids taking overlapping mutable references across arena blocks.

- [ ] **Step 8: Add the adjacent sibling-list pass under the stylesheet module**

Create `crates/minify/src/rules/stylesheet/adjacent.rs` with:

```rust
pub(crate) fn merge_adjacent_style_rules<'ast, 'scratch>(
    rules: &mut rocketcss_allocator::vec::Vec<'ast, CssRule<'ast>>,
    minifier: &mut DeclarationBlockMinifier<'scratch, 'ast>,
    cx: &mut MinifyContext<'scratch>,
) where
    'ast: 'scratch,
{
    for rule in rules.iter_mut() {
        merge_children(rule, minifier, cx);
    }
    if !cx.is_enabled(Options::MERGE_ADJACENT_RULES, OptionsOp::Any) {
        return;
    }
    merge_current_list(rules, minifier, cx);
}
```

`merge_children` must recurse into the `rules` vectors owned by `Style`, `Nesting::style`, `Media`, `Supports`, `MozDocument`, `LayerBlock`, `Container`, `Scope`, and `StartingStyle` variants. Other variants have no sibling `CssRule` list.

Use live-selector structural equality:

```rust
fn equal_live_selectors(left: &SelectorList<'_>, right: &SelectorList<'_>) -> bool {
    left.iter()
        .filter(|selector| !selector.is_tombstone())
        .eq(right.iter().filter(|selector| !selector.is_tombstone()))
}
```

Build maximal physical runs. A pair is eligible only when the previous rule has no nested children, both owners have live selectors, selectors are equal, prefixes match, and neither current block already has a merge link. For each run:

```rust
let mut blocks = run
    .iter()
    .map(|rule| match rule {
        CssRule::Style(style) => Ref::from_pinned_box(&style.declarations),
        _ => unreachable!("eligible runs contain only style rules"),
    })
    .collect::<std::vec::Vec<_>>();
minifier.minify_sequence(&mut blocks, cx);
```

Then link every later block to the previous block and tombstone all selectors on every owner except the final live tail.

- [ ] **Step 9: Invoke the list-aware pass after node-local normalization**

Export the new function from `crates/minify/src/rules/stylesheet/mod.rs`. In `minify_style_sheet`, after `stylesheet.visit_mut(&mut minifier)`, call it on `stylesheet.rules` before restoring the context:

```rust
rules::merge_adjacent_style_rules(
    &mut stylesheet.rules,
    &mut minifier.declaration_blocks,
    &mut minifier.cx,
);
```

Update the existing `preserves_rule_structure` test so its adjacent identical rules expect the newly merged output, while its at-rule boundary remains intact.

- [ ] **Step 10: Run the RED tests and the affected crates until GREEN**

Run:

```bash
cargo fmt --all
cargo test -p rocketcss_tests --test fixtures minify::minifies_upstream_fixtures -- --exact --nocapture
cargo test -p rocketcss_minify merges_adjacent_equal_selector_declaration_blocks
cargo test -p rocketcss_minify runs_box_ir_across_adjacent_blocks
cargo test -p rocketcss_minify preserves_cross_block_fallbacks_and_importance
cargo test -p rocketcss_minify
```

Expected: both existing cssnano fixtures execute and pass; all focused tests pass; the Lightning CSS different-selector fixture remains reported as skipped.

- [ ] **Step 11: Commit the shared-IR slice**

```bash
git add tests/src/minify.rs crates/minify/src/options.rs crates/minify/src/rules/stylesheet/declaration_block.rs crates/minify/src/rules/stylesheet/adjacent.rs crates/minify/src/rules/stylesheet/mod.rs crates/minify/src/lib.rs
git commit -m "feat(minify): merge adjacent declaration blocks"
```

---

### Task 3: Nested barriers, configuration, and idempotence

**Files:**
- Modify: `crates/minify/src/lib.rs`
- Modify: `crates/minify/src/rules/stylesheet/adjacent.rs`
- Modify: `crates/codegen/tests/to_css.rs`

**Interfaces:**
- Consumes: adjacent-run pass and option from Task 2.
- Produces: verified nested-order boundary behavior, disabled-option behavior, and repeat-safe links.

- [ ] **Step 1: Add failing nested and adjacency boundary tests**

Add to `crates/minify/src/lib.rs`:

```rust
#[test]
fn respects_nested_content_as_a_forward_merge_barrier() {
    assert_eq!(
        run(".a{color:red;& .child{display:block}}.a{color:blue}"),
        ".a{color:red;& .child{display:block}}.a{color:#00f}"
    );
    assert_eq!(
        run(".a{color:red}.a{color:blue;& .child{display:block}}"),
        ".a{color:red;color:#00f;& .child{display:block}}"
    );
}

#[test]
fn merges_only_inside_the_current_sibling_scope() {
    assert_eq!(
        run("a{color:red}b{display:block}a{color:blue}"),
        "a{color:red}b{display:block}a{color:#00f}"
    );
    assert_eq!(
        run("@media print{a{color:red}a{background:blue}}"),
        "@media print{a{color:red;background:#00f}}"
    );
}
```

- [ ] **Step 2: Add the disabled-option and repeat-minify tests**

```rust
#[test]
fn adjacent_rule_merging_is_configurable() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::MERGE_ADJACENT_RULES);

    assert_eq!(
        run_with_options("a{color:red}a{background:blue}", options),
        "a{color:red}a{background:#00f}"
    );
}

#[test]
fn adjacent_rule_merging_is_idempotent() {
    let allocator = Allocator::new();
    let mut stylesheet = parse(
        "a{width:1px}a{height:2px}a{width:1px}",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();

    minify(&mut stylesheet, MinifyOptions::default());
    let once = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();
    let second_stats = minify(&mut stylesheet, MinifyOptions::default());
    let twice = stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap();

    assert_eq!(once, "a{height:2px;width:1px}");
    assert_eq!(twice, once);
    assert_eq!(second_stats.declarations_removed, 0);
}
```

- [ ] **Step 3: Run the tests and verify RED where boundaries are incomplete**

Run:

```bash
cargo test -p rocketcss_minify respects_nested_content_as_a_forward_merge_barrier
cargo test -p rocketcss_minify merges_only_inside_the_current_sibling_scope
cargo test -p rocketcss_minify adjacent_rule_merging_is_configurable
cargo test -p rocketcss_minify adjacent_rule_merging_is_idempotent
```

Expected: any missing nested barrier, option check, or relink guard causes its focused assertion to fail for the intended reason.

- [ ] **Step 4: Tighten run eligibility and idempotence guards**

In `adjacent.rs`, make the pair predicate explicitly enforce:

```rust
previous.rules.is_empty()
    && previous.vendor_prefix == current.vendor_prefix
    && previous.selectors.iter().any(|selector| !selector.is_tombstone())
    && current.selectors.iter().any(|selector| !selector.is_tombstone())
    && current.declarations.previous_merged().is_none()
    && equal_live_selectors(&previous.selectors, &current.selectors)
```

Do not skip over any non-style sibling or tombstoned owner while searching for a match. Ensure recursive processing does not compare selectors from different `rules` vectors.

- [ ] **Step 5: Cover pretty output without phantom separators**

Extend the Task 1 codegen test to serialize the linked stylesheet with `PrinterOptions { prettify: true }`. Assert that the trimmed output starts directly with the live selector and contains only one `a` rule:

```rust
let pretty = stylesheet
    .to_css_string(PrinterOptions { prettify: true })
    .unwrap();
assert_eq!(pretty.matches("a {").count(), 1);
assert!(pretty.trim_start().starts_with("a {"));
```

- [ ] **Step 6: Run affected suites and verify GREEN**

Run:

```bash
cargo fmt --all
cargo test -p rocketcss_minify
cargo test -p rocketcss_codegen
cargo test -p rocketcss_tests --test fixtures minify::minifies_upstream_fixtures -- --exact --nocapture
```

Expected: all tests pass, the existing cssnano fixtures are no longer printed as skipped, and no pretty-output test observes a phantom rule separator.

- [ ] **Step 7: Commit nested and repeat-safety coverage**

```bash
git add crates/minify/src/lib.rs crates/minify/src/rules/stylesheet/adjacent.rs crates/codegen/tests/to_css.rs
git commit -m "test(minify): cover nested declaration merging"
```

---

### Task 4: Documentation and full verification

**Files:**
- Modify: `crates/minify/README.md`
- Verify: all modified source and test files.

**Interfaces:**
- Consumes: completed feature behavior from Tasks 1-3.
- Produces: accurate minifier documentation and repository-wide verification evidence.

- [ ] **Step 1: Update the minifier README**

Replace the statement that the minifier does not analyze multiple declaration blocks with text that states:

```markdown
Within one declaration block, and across physically adjacent style rules with
structurally equal selectors, a shared single-pass declaration IR removes exact
duplicates, merges compatible physical margin/padding longhands, and folds
simple longhand overrides into an earlier shorthand. Adjacent blocks retain
their arena allocations and are serialized through a backward merge chain.
Nested content remains an ordering barrier.
```

- [ ] **Step 2: Format and inspect whitespace errors**

Run:

```bash
cargo fmt --all
git diff --check
```

Expected: both commands exit successfully with no output from `git diff --check`.

- [ ] **Step 3: Run all affected package tests**

Run:

```bash
cargo test -p rocketcss_allocator
cargo test -p rocketcss_ast
cargo test -p rocketcss_codegen
cargo test -p rocketcss_minify
cargo test -p rocketcss_tests --test fixtures
```

Expected: every command passes with zero failed tests. The fixture runner may still report unrelated unsupported fixture groups, but must not skip the two enabled cssnano discard-duplicate groups.

- [ ] **Step 4: Run workspace tests and Clippy**

Run:

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: both commands exit successfully with no failures or warnings.

- [ ] **Step 5: Review the final diff and option coverage**

Run:

```bash
git status --short
git diff --stat b6ebb1d..HEAD
git diff b6ebb1d..HEAD -- tests/src/minify.rs crates/minify/src/options.rs crates/minify/src/rules/stylesheet crates/ast/src/rules/stylesheet.rs crates/codegen/src/rules.rs
```

Confirm that only the two in-scope cssnano skip entries were removed, the Lightning CSS different-selector fixture is still skipped, every link is backward-only, and no code path crosses nested content.

- [ ] **Step 6: Commit documentation and formatting changes**

```bash
git add crates/minify/README.md crates/allocator/src/reference.rs crates/ast/src/rules/stylesheet.rs crates/codegen/src/rules.rs crates/codegen/tests/to_css.rs crates/minify/src tests/src/minify.rs
git commit -m "docs(minify): describe adjacent block merging"
```

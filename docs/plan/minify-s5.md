# Minify S5 implementation plan

## Goal

Complete the S5 terminal commit for cross-rule declaration minification. S5
must consume the stable S1-S4 result, materialize every deferred declaration
representation into the Radix AST, discard merge-only state, and hand an
ordinary self-contained AST to codegen.

The semantic contract remains the one in
[S5: terminal commit and cleanup](../cross-rule-declaration-merging/s5-ast-reification-commit.md).
This plan covers implementation work only; it does not redesign S1-S4.

## Current baseline

The S4 implementation already contains the minimum S5 path needed to commit
partially live `margin` and `padding` shorthands:

- `AstDeclarationPlan` snapshots owner, block revision, effect revision,
  importance, family, and live-effect mask in
  `crates/nano/src/cross_rule_declaration_merging/radix_state.rs`;
- `CrossRuleState::finish` validates those snapshots, preflights aggregate
  declaration growth, and applies every plan;
- `RadixCompilation::rewrite_declaration_with_sequence` in
  `crates/ast/src/rules/stylesheet/compilation/mutation.rs` preserves the
  authored origin ID and supports non-contiguous declaration chains; and
- `crates/nano/src/lib.rs` reruns block-local representation minification for
  the deduplicated dirty blocks returned by `finish`.

S5 therefore should harden and make this boundary explicit rather than add a
second reification path.

## Scope

S5 will:

1. prove that semantic stabilization is complete;
2. validate all S4 plans before the first AST mutation;
3. preflight the total declaration growth once;
4. commit plans at their authored origins without moving later declarations;
5. consume all cross-rule-only sidecars;
6. rerun the existing block-local representation pass only for dirty live
   blocks; and
7. establish structural validity and byte idempotence at the public minify
   boundary.

The first supported deferred representation remains typed physical
`margin`/`padding`. New effect families belong in S2/S4 first; S5 only gains a
new typed plan arm after S4 can prove and construct it.

## Non-goals

- No new liveness, selector-union, movement-safety, compatibility, or
  profitability decision in S5.
- No full rule, block, or declaration-store rebuild.
- No source-order reconstruction; Radix links are already authoritative.
- No tombstone or arena compaction until benchmarks demonstrate a net win.
- No second S1-S4 stabilization cycle after the first S5 mutation.
- No fallback to `Declaration::Unparsed` for a typed plan that cannot be
  committed.

## Target execution flow

```text
crates/nano/src/cross_rule_declaration_merging/radix_state.rs
S1-S3 queues empty + S4 dirty queue drained
       ↓
S5 fixed-point check
  queue/candidate coverage is complete
       ↓
S5 read-only preflight
  validate owner/revisions/origin membership/typed payload/live mask/importance
  reject duplicate origins
  sum checked declaration growth
       ↓
crates/ast/src/rules/stylesheet/compilation/mutation.rs
reserve/verify aggregate declaration capacity
       ↓
commit every prevalidated plan
  reuse origin ID → append replacement tail → repair links/count/last/revision
       ↓
consume CrossRuleState
  histories/IR/queues/revisions/plans become unreachable
       ↓
crates/nano/src/lib.rs
rerun DeclarationBlockMinifier for representation-dirty live blocks
       ↓
debug validate_ast() + return MinifyStats
```

## Implementation steps

### 1. Make the S5 boundary explicit

Keep scheduling in
`crates/nano/src/cross_rule_declaration_merging/radix_state.rs`, but separate
terminal work from the stabilization loop:

- add one `is_semantic_fixed_point`/`assert_semantic_fixed_point` helper that
  covers the S1, S2, S3, and S4 queues plus candidate storage;
- call S5 only after that helper succeeds;
- preserve the consuming `self` boundary so successful commit necessarily
  drops `histories`, `DeclarationIrStore`, queues, revisions, and plan storage;
- rename `finish` to a phase-specific name such as `commit_s5` if it stays in
  `radix_state.rs`; and
- if the terminal code grows, move plan validation and application into
  `crates/nano/src/cross_rule_declaration_merging/s5.rs`, leaving only the
  state-to-S5 handoff in `radix_state.rs`.

Do not introduce a parallel AST or a second set of property discriminants.
Plan dispatch stays typed through `AstDeclarationPlanKind` and `BoxFamily`.

### 2. Turn validation into a complete read-only preflight

Before mutating any payload or link, validate every plan against both the AST
and `DeclarationIrStore`:

- the owner block exists and is live;
- its revision equals `block_revision`;
- the origin exists in the owner's current declaration chain, not merely in
  the global declaration arena;
- origins are unique across the plan batch;
- the occurrence still names the same owner and `effect_revision`;
- the occurrence is still a partially live `BoxShorthand` of the planned
  family;
- the current live-effect mask exactly equals the planned mask and contains
  only `ALL_BOX_SIDES` bits;
- the AST payload is the matching typed `Declaration::Margin` or
  `Declaration::Padding` variant;
- the current importance equals the plan snapshot; and
- `count_ones(live_effects) - 1` and the aggregate additional count use checked
  arithmetic.

Add a reusable read-only declaration-chain validation helper beside
`rewrite_declaration_with_sequence` in
`crates/ast/src/rules/stylesheet/compilation/mutation.rs`. Both S5 preflight and
the public rewrite transaction should use the same topology rules so they
cannot drift.

Expected validation or capacity failure must occur before the first mutation.
After complete preflight, a commit failure is an internal invariant violation,
not a recoverable partial S5 result.

### 3. Commit the prevalidated declaration plans

Keep `RadixCompilation::rewrite_declaration_with_sequence` as the only
one-to-many mutation primitive:

- move the authored non-`Clone` shorthand payload only after all plans and
  aggregate capacity have passed preflight;
- reuse the origin ID for the first emitted longhand;
- emit physical sides in top/right/bottom/left order, filtered by the live
  mask;
- copy the authored importance bit to every replacement;
- connect the replacement tail to the origin's previous successor;
- update `first_declaration`, `last_declaration`, and `declaration_count` as
  required; and
- advance the owning block revision once per rewritten origin.

`materialize_box_longhands` in `crates/nano/src/rules/layout.rs` remains the
single typed conversion helper. S5 must not duplicate its declaration matching
or serialize values to strings.

The commit must work for:

- one, two, or three surviving sides;
- first, middle, and last declaration origins;
- multiple plans in one owner block;
- plans spread across multiple owner blocks; and
- declaration chains made non-contiguous by S1/S3.

### 4. Finish cleanup and local representation follow-up

Continue returning a deduplicated list of representation-dirty blocks from the
cross-rule phase. In `crates/nano/src/lib.rs`:

- ignore blocks retired before the follow-up;
- run `DeclarationBlockMinifier::minify_compilation_block` exactly once for
  each remaining dirty block;
- do not republish those blocks to cross-rule histories or enqueue new S1-S4
  work; and
- run the debug AST validator after the local follow-up, so validation covers
  the actual public minify output rather than only the intermediate S5 output.

The dirty list must include both S1 concatenation owners that may now form a
local shorthand and S5 owners whose one-to-many rewrite may be recombined.
Preserve deterministic first-dirty order; no hash-set iteration should decide
mutation order.

### 5. Add focused S5 tests

#### AST transaction tests

Extend `crates/ast/src/rules/stylesheet/compilation/tests.rs` with:

- read-only preflight for first, middle, and last origins;
- rejection of an origin from a different block;
- rejection of an invalid/truncated declaration chain;
- aggregate capacity failure leaving every payload, link, endpoint, count,
  importance bit, and revision unchanged;
- multiple sequential rewrites in one non-contiguous block; and
- preservation of the primary origin ID and correct final tail linkage.

Retain the existing non-`Clone` payload coverage.

#### Scheduler/S5 unit tests

Add tests near `radix_state.rs` (or in `s5.rs` after extraction) for:

- a no-plan fixed point;
- duplicate origins and stale owner/block/effect snapshots rejected before
  mutation;
- planned family, live mask, payload kind, and importance mismatches;
- two valid plans in one block and in separate blocks;
- batch preflight failure on the last plan proving the first plan was not
  committed; and
- consuming S5 state returning each dirty block once.

#### End-to-end minify tests

Extend `crates/nano/src/tests/box_model.rs` and, where topology is involved,
`crates/nano/src/tests/rule_merge.rs` with:

- partial shorthand reification for every live-side count and both box
  families;
- `!important` isolation and propagation;
- authored shorthand origins before, between, and after unrelated
  declarations;
- S1/S3-created non-contiguous chains;
- logical properties, `all`, CSS-wide values, variables, and unparsed values
  remaining barriers;
- local recombination after S1 and after S5;
- the disabled `MERGE_ADJACENT_RULES` path remaining unchanged; and
- parse → minify → codegen → parse → minify → codegen producing
  identical bytes.

Every structural test should also assert `Compilation::validate_ast()`.

### 6. Enable all currently supported CSSNano and Lightning CSS fixtures

Treat upstream-source synchronization and runnable coverage as separate facts.
This step targets the fixtures already copied under `tests/`; it must not claim
coverage merely because an upstream source file exists in a snapshot.

The audit baseline recorded on 2026-08-13 is:

| Source        | Static pairs | Executed | Skipped |
| ------------- | -----------: | -------: | ------: |
| CSSNano       |           53 |       28 |      25 |
| Lightning CSS |           32 |       14 |      18 |

CSSNano also has recorded dynamic coverage in
`tests/fixtures/minify-dynamic/cssnano/*.json`. Recalculate its executed and
skipped case counts during implementation; the older count in
`docs/cssnano-skipped-tests.md` must not be copied forward without rerunning
the harness.

#### Audit and enable CSSNano cases

Use both CSSNano fixture paths:

- static input/output pairs in `tests/fixtures/minify/cssnano/`; and
- recorded dynamic specs in `tests/fixtures/minify-dynamic/cssnano/`.

After S5 is implemented, run every locally skipped case through the normal
parse/minify/codegen pipeline, including cases currently hidden by a
whole-directory or whole-spec skip. In particular, audit the S1-S5-relevant
`discard-duplicates` and `merge-rules` cases first, then continue through every
remaining CSSNano skip rather than stopping after the expected S5 wins.

For each case:

1. remove the skip when RocketCSS now produces the normalized expected result;
2. replace a broad plugin/directory skip with precise per-case skips when only
   part of that group remains unsupported;
3. retain `upstreamSkip: true` cases as upstream-disabled rather than counting
   them as RocketCSS coverage;
4. preserve the recorded upstream input and expectation; do not rewrite an
   expectation merely to hide a full-pipeline behavior difference; and
5. keep every unsupported case in the corpus with a concrete reason naming
   the missing transform, parser feature, option, or intentional policy.

Update both skip functions:

- `still_requires_unsupported_transform` in `tests/src/minify.rs`; and
- `still_requires_unsupported_transform` in
  `tests/src/minify_dynamic.rs`.

Delete stale skip arms instead of adding an allowlist in front of them. The
default fixture tests must execute the newly supported cases without a special
environment variable or opt-in test target.

#### Audit and enable Lightning CSS cases

Run every skipped pair under `tests/fixtures/minify/lightningcss/` after S5,
not only the selector-merge fixtures expected to pass. Remove every skip whose
expected output is now supported, and narrow directory-level skips when a
directory mixes supported and unsupported behavior.

Prioritize `rules/merge-selectors` and other declaration/rule-merging cases
that map directly to S1-S5. Keep target-browser prefix synthesis, unsupported
math evaluation, invalid-value repair, keyframe merging, media/layer merging,
and other unrelated transforms explicitly skipped unless the existing
RocketCSS pipeline already produces the fixture's expected output.

Do not add Lightning-specific string dispatch to Nano. A fixture is enabled
only through the same typed AST and minify path used by ordinary RocketCSS
input.

#### Coverage accounting and regression guard

Run the fixture status command with the actual local upstream paths:

```sh
pnpm upstream-tests status \
  --lightningcss /data00/home/jinzhixin/rstack/lightningcss \
  --cssnano /data00/home/jinzhixin/rstack/cssnano
cargo test -p rocketcss_tests --test fixtures minifies_static_fixtures
cargo test -p rocketcss_tests --test fixtures minifies_dynamic_fixtures
```

Record the before/after executed and skipped counts in
`docs/cssnano-skipped-tests.md` and the fixture README. The enabled count must
increase monotonically unless a documented upstream fixture is removed. Add a
focused count or path assertion for newly enabled S5 groups so a future broad
skip cannot silently disable them again.

The current status also reports no checked-in upstream source snapshot and
drift against both local upstream repositories. If source snapshots are
refreshed to look for additional unported cases, do that with
`pnpm upstream-tests diff`/`sync` in a separate reviewed commit. Snapshot sync
does not replace the runnable-fixture audit above.

### 7. Verify correctness and performance

Run:

```sh
cargo fmt --all
cargo test -p rocketcss_ast
cargo test -p rocketcss_nano
cargo test -p rocketcss_tests --test fixtures
cargo test -p rocketcss_tests --test fixtures minifies_dynamic_fixtures
cargo clippy --workspace --all-targets --all-features -- -D warnings
git diff --check
```

Then run the existing minify benchmarks in `tasks/benchmark/benches/minify.rs`
against the pre-S5 baseline. Record parse, minify, codegen, peak memory, and
output-size changes. S5 should add no full-store scan beyond plan validation
and the explicitly dirty local blocks.

## Suggested commit sequence

1. **refactor(ast): share declaration rewrite preflight**
   - extract read-only chain validation and add AST transaction tests.
2. **feat(minify): complete S5 terminal validation and commit**
   - add the fixed-point gate, complete all-plan preflight, and commit typed
     plans through the existing transaction.
3. **test(minify): cover S5 cleanup and idempotence**
   - add scheduler and end-to-end regressions, then move debug validation after
     the local representation follow-up.
4. **test(minify): enable supported upstream minify fixtures**
   - audit every CSSNano static/dynamic skip and every Lightning CSS static
     skip, narrow unsupported matchers, and record the new coverage counts.
5. **perf(minify): validate S5 terminal overhead**
   - benchmark and only then consider sparse-state or compaction follow-ups.

Each commit should keep the AST valid and the test suite green; S5 should not
land in a state where codegen depends on merge-only sidecars.

## Definition of done

- S5 is entered only from a proven S1-S4 fixed point.
- Every failure reachable from external AST state happens before mutation.
- All valid S4 plans are committed at their authored origins with exact value
  and importance preservation.
- Cross-rule sidecars are unavailable after successful S5 commit.
- The local representation follow-up cannot create semantic scheduler work.
- `validate_ast()` succeeds on the final minified AST.
- Minifying generated output a second time is byte-idempotent.
- Every copied CSSNano and Lightning CSS fixture supported by the completed
  pipeline runs by default, with only precise unsupported cases skipped.
- CSSNano dynamic cases remain recorded even when unsupported, and the skip
  report reflects current executed/skipped counts.
- Enabled and disabled feature paths pass focused and workspace verification.
- Benchmarks show no unexplained full-AST copy, scan, or material regression.

## Implementation evidence

Implemented and audited on 2026-08-13 against baseline `a489ef3`. The existing
Divan minify benchmark was run from isolated release builds with identical
sampling settings.

| Measurement | Baseline median | S5 median | Change |
| --- | ---: | ---: | ---: |
| Bootstrap parse | 6.664 ms | 6.949 ms | +4.3% |
| Bootstrap minify | 1.905 ms | 1.953 ms | +2.5% |
| Bootstrap codegen | 788.4 µs | 789.0 µs | +0.1% |
| `box_s4_reification` minify | 2.221 µs | 2.269 µs | +2.2% |

The small minify increase is the expected complete read-only plan and
declaration-chain preflight. Parse and codegen have no changed execution path;
their observed difference is benchmark noise. A one-shot benchmark run over
the bootstrap and Tailwind inputs reported the same 153 MiB maximum resident
set for baseline and S5. Exact expected-output fixtures and byte-idempotence
tests report no output-size change.

Runnable upstream coverage after the exhaustive audit is 35/53 CSSNano static
pairs, 15/32 Lightning CSS static pairs, and 1239/2105 CSSNano dynamic cases.
The remaining dynamic count is 865 precise RocketCSS gaps plus one case marked
disabled by upstream.

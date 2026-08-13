# CSSNano fixture coverage

Audited on 2026-08-13 against the locally recorded CSSNano corpus. Upstream
source synchronization and runnable fixture coverage are separate: the local
source snapshot is currently empty, while all copied static and dynamic
fixtures are still executed or skipped explicitly by the Rust harness.

## Current coverage

| Corpus                                  | Total | Executed | Skipped |
| --------------------------------------- | ----: | -------: | ------: |
| Static `input.css` / `output.css` pairs |    53 |       35 |      18 |
| Dynamic recorded cases                  |  2105 |     1239 |     866 |

The dynamic skipped count contains one upstream-disabled case and 865
RocketCSS-specific unsupported cases. Every unsupported dynamic case carries a
`rocketcssSkip` reason beside its original input and expected output. This
replaces broad plugin-level matcher arms that previously hid supported
pass-through and already implemented cases.

Before this audit, the static status command reported 28 executed and 25
skipped CSSNano pairs. The previous dynamic report recorded 1287 executed and
818 skipped, but its broad matcher order was stale: an exhaustive run of all
recorded cases found 1239 cases that match today and 865 genuine RocketCSS
gaps, plus the single upstream-disabled case.

Newly enabled static coverage includes:

- `discard-duplicates/declarations` and `discard-duplicates/partial` through
  the ordinary S1-S5 declaration pipeline;
- `normalize-positions/center`;
- `normalize-repeat/repeat-x` and `normalize-repeat/repeat-y`;
- `normalize-timing/step-start`; and
- `normalize-url/unquoted`.

## Remaining skips

Static skips remain precise path entries in `tests/src/minify.rs`. They cover
unimplemented empty/overridden-rule removal, opaque gradient and shadow
grammars, typed percentage-zero semantics, transforms, position/repeat/URL
subcases that still differ, and related parser or policy gaps.

Dynamic skips live in
`tests/fixtures/minify-dynamic/cssnano/*.json`. The recorder preserves those
annotations by the case identity `(test, input, expected, passthrough,
upstreamSkip)`, so refreshing the upstream recordings cannot silently restore
broad skips or erase reviewed reasons.

The Rust harness asserts the current `(1239 executed, 866 skipped)` accounting.
Any new or reclassified case therefore requires an explicit audit rather than
silently changing coverage.

## Verification

```sh
pnpm upstream-tests status
cargo test -p rocketcss_tests --test fixtures minifies_static_fixtures
cargo test -p rocketcss_tests --test fixtures minifies_dynamic_fixtures
```

The status command currently also reports upstream snapshot drift. Refreshing
the byte-for-byte source snapshot is intentionally a separate change and does
not substitute for runnable fixture coverage.

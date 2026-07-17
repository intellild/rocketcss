---
name: rocketcss-best-practices
description: Review and implement RocketCSS changes with type-directed dispatch, shared matching abstractions, compact state, lossless serialization, and behavior-preserving verification. Use for RocketCSS feature work, refactors, bug fixes, and code review.
---

# RocketCSS Best Practices

## Goal

Keep RocketCSS changes compact, type-directed, and behavior-preserving. Reuse the repository's common abstractions before adding local helpers.

## Review Workflow

1. Inspect the AST type and existing shared macros/helpers before editing consumers.
2. Compare the branch against its base and identify newly duplicated state, string dispatch, and serialization paths.
3. Apply the rules below without broad unrelated refactors.
4. Add focused regression tests for enabled and disabled feature paths.
5. Run formatting, relevant tests, and Clippy in proportion to the change.

## Prefer Typed Dispatch

- Match known properties through `PropertyId` variants, not `property_id.name()` or repeated property-name strings.
- Keep property identity separate from typed value support. If a standard property is known but its value still uses `Declaration::Unparsed`, add a leaf `PropertyId` variant instead of classifying it as `Custom` or introducing a parallel property-ID enum.
- Reserve `PropertyId::Custom` for genuinely unknown names. Parse supported vendor-prefixed names into the existing prefix-aware `PropertyId` variants and serialize the stored prefix losslessly.
- Match other established AST enums directly whenever parsing has already classified the input.

Example shape:

```rust
let context = if order_values {
    match property_id {
        PropertyId::Animation(_) => PropertyContext::Animation,
        PropertyId::Border | PropertyId::BorderTop => PropertyContext::Border,
        PropertyId::Columns => PropertyContext::Columns,
        _ => property_context(property_id),
    }
} else {
    property_context(property_id)
};
```

Do not repeat `if order_values && ...` for every property. Put one large feature-gated block outside, then dispatch with `match` inside. The disabled branch must use one common non-ordering path.

## Reuse ASCII-Insensitive Matching

- Use the shared, general `match_ignore_ascii_case!` macro for free-text CSS identifiers and keywords.
- Pass all related literal arms to one macro invocation so expansion produces one `if`/`else if` chain.
- Do not create a property-specific case-insensitive macro.
- Do not replace typed enum dispatch with string matching merely to use the macro.
- Avoid multiple boolean invocations when one multi-arm invocation expresses the same classification.

```rust
match_ignore_ascii_case!(
    keyword,
    "normal" => FontWeight::Normal,
    "bold" => FontWeight::Bold,
    "bolder" | "lighter" => FontWeight::Relative,
    _ => FontWeight::Unknown,
)
```

## Pack Related Boolean State

- When an AST node gains multiple related booleans, store them in a private `u8` bitflags field rather than separate public `bool` fields.
- Keep construction explicit and expose semantic accessors such as `is_math_function()` and `is_color_function()`.
- Make parser, visitor, minifier, and codegen call accessors instead of reading representation details.
- Preserve existing public behavior and update struct literals through a constructor when possible.

## Serialize Colors as Packed Integers

- Convert RGB or RGBA channels to one `u32` before hexadecimal serialization.
- Call the hexadecimal serializer once with the packed value and final digit width.
- Preserve short-hex eligibility by checking paired nibbles before choosing three/four versus six/eight digits.
- Avoid serializing each `u8` channel independently when the output is one CSS hex token.

## Separate Integer and Float Serialization

- Keep `serialize_int` independent from `serialize_number`.
- Serialize integers with `itoa`; do not cast them through `f32` or route them through floating-point normalization.
- Reserve float shortening, exponent decisions, negative-zero handling, and decimal trimming for `serialize_number`.
- Add boundary tests that would expose precision loss if an integer accidentally took the floating-point path.

## Preserve Edge Behavior

- Test feature flags in both enabled and disabled states.
- Cover ASCII case variants while resolving names into `PropertyId`; downstream context selection should exercise enum dispatch.
- Retain vendor-prefixed behavior through prefix-aware known variants and retain genuinely unsupported property names through `Custom`.
- Check that refactors do not alter AST allocation, node identity, visitor flow, or serialized output except where intended.

## Verification

Run at minimum:

```sh
cargo fmt --all
cargo test -p <affected-crate>
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Run broader workspace tests when shared AST types, exported macros, or codegen primitives change. Inspect `git diff --check` and the final diff before handoff.

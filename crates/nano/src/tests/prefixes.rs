use super::*;

#[test]
#[ignore]
fn preserves_vendor_prefixes_in_supports_conditions() {
    assert_eq!(
        run(
            "@supports ((display:-webkit-box) and (-webkit-box-orient:vertical) and (-webkit-line-clamp:3)){.foo{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:3}}"
        ),
        "@supports ((display:-webkit-box) and (-webkit-box-orient:vertical) and (-webkit-line-clamp:3)){.foo{display:-webkit-box;-webkit-box-orient:vertical;-webkit-line-clamp:3}}"
    );
    assert_eq!(
        run(
            "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
        ),
        "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
    );
}

#[test]
#[ignore]
fn preserves_prefixed_and_standard_backdrop_filter_fallbacks() {
    assert_eq!(
        run(".a{backdrop-filter:blur(10px);-webkit-backdrop-filter:blur(10px)}"),
        ".a{backdrop-filter:blur(10px);-webkit-backdrop-filter:blur(10px)}"
    );
    assert_eq!(
        run(".b{-webkit-backdrop-filter:blur(10px);backdrop-filter:blur(10px)}"),
        ".b{-webkit-backdrop-filter:blur(10px);backdrop-filter:blur(10px)}"
    );
    assert_eq!(
        run("a{-webkit-text-size-adjust:none;text-size-adjust:none}"),
        "a{-webkit-text-size-adjust:none;text-size-adjust:none}"
    );
    assert_eq!(
        run("b{text-size-adjust:none;-webkit-text-size-adjust:none}"),
        "b{text-size-adjust:none;-webkit-text-size-adjust:none}"
    );
}

#[test]
#[ignore]
fn preserves_authored_text_decoration_prefixes_without_duplication() {
    assert_eq!(
        run("a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}"),
        "a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}"
    );
    assert_eq!(
        run("a{text-decoration:inherit;-webkit-text-decoration:inherit}"),
        "a{text-decoration:inherit;-webkit-text-decoration:inherit}"
    );
}

#[test]
#[ignore]
fn preserves_authored_legacy_grid_fallbacks_without_targets() {
    assert_eq!(
        run(
            ".grid{display:-ms-grid;display:grid;grid-auto-columns:1fr;grid-column-gap:16px;grid-row-gap:16px;-ms-grid-columns:1fr 1fr 1fr;grid-template-columns:1fr 1fr 1fr;-ms-grid-rows:auto;grid-template-rows:auto}"
        ),
        ".grid{display:-ms-grid;display:grid;grid-auto-columns:1fr;grid-column-gap:1pc;grid-row-gap:1pc;-ms-grid-columns:1fr 1fr 1fr;grid-template-columns:1fr 1fr 1fr;-ms-grid-rows:auto;grid-template-rows:auto}"
    );
}

#[test]
#[ignore]
fn preserves_logical_overflow_alongside_physical_fallbacks() {
    assert_eq!(
        run("a{overflow-inline:auto;overflow-block:scroll;overflow-x:hidden}"),
        "a{overflow-inline:auto;overflow-block:scroll;overflow-x:hidden}"
    );
}

#[test]
#[ignore]
fn preserves_dynamic_logical_values_and_user_select_fallbacks() {
    assert_eq!(
        run("a{margin-inline:var(--m);margin-inline:calc(var(--gap) + 1px)}"),
        "a{margin-inline:var(--m);margin-inline:calc(var(--gap) + 1px)}"
    );
    assert_eq!(
        run("a{-webkit-user-select:auto;user-select:all}"),
        "a{-webkit-user-select:auto;user-select:all}"
    );
}

#[test]
#[ignore]
fn preserves_prefixed_mask_image_variable_fallbacks_without_duplication() {
    const SOURCE: &str = ".foo{-webkit-mask-image:url(./foo.svg);mask-image:url(./foo.svg)}.bar{-webkit-mask-image:var(--foo);mask-image:var(--foo)}.fallback{-webkit-mask-image:var(--foo,url(./fallback.svg));mask-image:var(--foo,url(./fallback.svg))}";
    assert_eq!(run(SOURCE), SOURCE);
}

#[test]
#[ignore]
fn preserves_distinct_vendor_values_and_negated_supports_conditions() {
    const SOURCE: &str = "a{-webkit-appearance:none;appearance:textfield}b{appearance:textfield;-webkit-appearance:none}@supports not (backdrop-filter:none){c{-webkit-backdrop-filter:none;backdrop-filter:none}}";
    assert_eq!(run(SOURCE), SOURCE);
}

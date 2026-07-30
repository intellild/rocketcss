use super::*;

#[test]
#[ignore]
fn preserves_variables_in_the_all_property() {
    assert_eq!(
        run(".boop{margin:1px;all:var(--all,revert-layer);margin-left:2px}"),
        ".boop{margin:1px;all:var(--all,revert-layer);margin-left:2px}"
    );
}

#[test]
fn custom_property_values_are_not_minified() {
    assert_eq!(
        run("a{--color:rgb(0 0 0);--size:calc(3px * 2);--broken:10.px}"),
        "a{--color:rgb(0 0 0);--size:calc(3px * 2);--broken:10.px}"
    );
}

#[test]
fn opaque_invalid_and_unknown_values_are_not_minified() {
    assert_eq!(
        run("a{opacity:calc(.2 * 3);width:10.px;future:calc(3px * 2)}"),
        "a{opacity:calc(.2 * 3);width:10.px;future:calc(3px * 2)}"
    );
}

#[test]
fn skipped_declarations_do_not_participate_in_deduplication() {
    assert_eq!(
        run("a{--theme:1;--theme:1;width:10.px;width:10.px}"),
        "a{--theme:1;--theme:1;width:10.px;width:10.px}"
    );
}

#[test]
#[ignore]
fn minifies_supported_colors_in_custom_properties() {
    assert_eq!(
        run("a{--white:white;--hex:#FFFFFF;--dynamic:var(--color)}"),
        "a{--white:#fff;--hex:#fff;--dynamic:var(--color)}"
    );
}

#[test]
#[ignore]
fn preserves_whitespace_between_variables_and_adjacent_values() {
    assert_eq!(
        run("a{margin:var(--x) var(--y);padding:var(--x) 0}"),
        "a{margin:var(--x) var(--y);padding:var(--x) 0}"
    );
}

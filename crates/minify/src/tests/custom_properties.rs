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
fn custom_property_transforms_are_configurable() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::TRANSFORM_CUSTOM_PROPERTIES);
    assert_eq!(
        run_with_options("a{--color:rgb(0 0 0);--size:calc(3px * 2)}", options),
        "a{--color:rgb(0 0 0);--size:calc(3px * 2)}"
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

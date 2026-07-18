use super::*;

#[test]
#[ignore]
fn leaves_invalid_nonzero_unitless_lengths_unchanged() {
    assert_eq!(run("a{width:100}"), "a{width:100}");
}

#[test]
fn folds_calc_values_in_the_existing_function_node() {
    let mut options = MinifyOptions::default();
    options
        .flags
        .remove(Options::CONVERT_LENGTH_UNITS | Options::CONVERT_EXTENDED_LENGTH_UNITS);
    assert_eq!(
        run_with_options(
            "a{width:calc(3px * 2);height:calc(100px + 50px - 25px)}",
            options,
        ),
        "a{width:6px;height:125px}"
    );
    assert_eq!(run("a{width:calc(0px + 1em)}"), "a{width:calc(0px + 1em)}");
}

#[test]
#[ignore]
fn preserves_new_viewport_units_instead_of_approximating_them() {
    assert_eq!(
        run(
            "a{height:100dvh;min-height:100svh;max-height:100lvh;width:100dvw;min-width:100svw;max-width:100lvw}"
        ),
        "a{height:100dvh;min-height:100svh;max-height:100lvh;width:100dvw;min-width:100svw;max-width:100lvw}"
    );
}

#[test]
#[ignore]
fn folds_static_calc_in_custom_properties_but_preserves_dynamic_terms() {
    assert_eq!(
        run(":root{--static:calc(10px + 20px);--dynamic:calc(10px + var(--bar))}"),
        ":root{--static:30px;--dynamic:calc(10px + var(--bar))}"
    );
}

#[test]
#[ignore]
fn minifies_nested_calc_groups_without_panicking_or_losing_units() {
    let output = run("a{height:calc((100dvh - 10.5rem) + (4vh + 230px))}");
    for unit in ["100dvh", "10.5rem", "4vh", "230px"] {
        assert!(output.contains(unit), "missing {unit} in {output}");
    }
    assert_eq!(run(&output), output);
}

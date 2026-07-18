use super::*;

#[test]
fn preserves_blue_identifiers_in_untyped_values() {
    assert_eq!(
        run("a{--theme:blue;unknown:blue;background:blue;color:blue}"),
        "a{--theme:blue;unknown:blue;background:#00f;color:#00f}"
    );
}

#[test]
fn normalizes_numbers_colors_and_lengths() {
    assert_eq!(
        run("a{color:rgb(255 0 0);border-color:#ffffff;width:16px}"),
        "a{color:red;border-color:#fff;width:1pc}"
    );
    assert_eq!(
        run("a{transition-duration:500ms;transform:rotate(.25turn)}"),
        "a{transition-duration:.5s;transform:rotate(90deg)}"
    );
    assert_eq!(run("a{MARGIN:1px 1px 1px 1px}"), "a{margin:1px}");
}

#[test]
fn serializes_rgb_channels_as_integers() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::USE_HEX_ALPHA_COLORS);

    assert_eq!(
        run_with_options("a{color:rgba(1 2 3/.5)}", options),
        "a{color:rgba(1,2,3,.5)}"
    );
}

#[test]
#[ignore]
fn preserves_light_dark_and_color_scheme_without_lowering() {
    assert_eq!(
        run(
            ":root{--background:light-dark(white,black)}p{background:var(--background);color-scheme:dark}"
        ),
        ":root{--background:light-dark(#fff,#000)}p{background:var(--background);color-scheme:dark}"
    );
    assert_eq!(
        run(
            "a{border-bottom:1px solid light-dark(var(--light),var(--dark));border-color:light-dark(white,black)}"
        ),
        "a{border-bottom:1px solid light-dark(var(--light),var(--dark));border-color:light-dark(#fff,#000)}"
    );
    assert_eq!(
        run(
            ".dark{color-scheme:only dark}.light{color-scheme:only light}.alt{color-scheme:dark only}"
        ),
        ".dark{color-scheme:only dark}.light{color-scheme:only light}.alt{color-scheme:dark only}"
    );
    assert_eq!(
        run(":host{color-scheme:inherit}a{color-scheme:normal}"),
        ":host{color-scheme:inherit}a{color-scheme:normal}"
    );
}

#[test]
#[ignore]
fn minifies_color_keywords_in_variable_fallbacks_by_property_context() {
    assert_eq!(
        run("#foo{color:white}#bar{color:var(--c, white)}"),
        "#foo{color:#fff}#bar{color:var(--c,#fff)}"
    );
    assert_eq!(
        run("a{font-family:var(--family,white)}"),
        "a{font-family:var(--family,white)}"
    );
}

#[test]
#[ignore]
fn preserves_oklch_variables_when_fallback_generation_is_unavailable() {
    assert_eq!(
        run(
            ".text-red-200{--tw-text-opacity:1;color:oklch(92.19% .04 20/var(--tw-text-opacity))}"
        ),
        ".text-red-200{--tw-text-opacity:1;color:oklch(92.19% .04 20/var(--tw-text-opacity))}"
    );
    assert_eq!(
        run("a{color:oklch(var(--channels)/var(--alpha))}"),
        "a{color:oklch(var(--channels)/var(--alpha))}"
    );
}

#[test]
#[ignore]
fn preserves_powerless_color_channels_inside_color_mix() {
    assert_eq!(
        run("a{background:color-mix(in hsl,var(--primary) 40%,hsl(193 100% 100%) 60%)}"),
        "a{background:color-mix(in hsl,var(--primary) 40%,hsl(193 100% 100%) 60%)}"
    );
}

#[test]
#[ignore]
fn disabling_value_normalization_preserves_authored_color_functions() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::NORMALIZE_VALUES);
    assert_eq!(
        run_with_options("a{color:rgb(255 255 255);background:hsl(40 50% 50%)}", options),
        "a{color:rgb(255 255 255);background:hsl(40 50% 50%)}"
    );
}

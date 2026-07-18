use rocketcss_allocator::Allocator;
use rocketcss_codegen::{PrinterOptions, ToCss};
use rocketcss_minify::{MinifyOptions, Options, minify};
use rocketcss_parser::{ParserOptions, parse};

fn minify_css(source: &str, flags: Options) -> String {
    let allocator = Allocator::new();
    let mut stylesheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    minify(
        &mut stylesheet,
        MinifyOptions {
            flags,
            ..MinifyOptions::default()
        },
    );
    stylesheet
        .to_css_string(PrinterOptions { prettify: false })
        .unwrap()
}

fn default_flags() -> Options {
    MinifyOptions::default().flags
}

#[test]
fn keeps_explicit_denominator_one_by_default() {
    assert_eq!(
        minify_css(
            "@media (aspect-ratio: 1/1){.foo{color:red}}",
            default_flags()
        ),
        "@media (aspect-ratio:1/1){.foo{color:red}}"
    );
}

#[test]
fn reduces_ratio_values_but_keeps_explicit_denominator_by_default() {
    assert_eq!(
        minify_css(
            "@media (aspect-ratio: 2/2){.foo{color:red}}",
            default_flags()
        ),
        "@media (aspect-ratio:1/1){.foo{color:red}}"
    );
}

#[test]
fn drops_denominator_one_when_convert_ratios_is_enabled() {
    let flags = default_flags() | Options::CONVERT_RATIOS;
    assert_eq!(
        minify_css("@media (aspect-ratio: 1/1){.foo{color:red}}", flags),
        "@media (aspect-ratio:1){.foo{color:red}}"
    );
    assert_eq!(
        minify_css("@media (aspect-ratio: 2/2){.foo{color:red}}", flags),
        "@media (aspect-ratio:1){.foo{color:red}}"
    );
}

#[test]
fn never_invents_a_denominator() {
    for flags in [default_flags(), default_flags() | Options::CONVERT_RATIOS] {
        assert_eq!(
            minify_css("@media (aspect-ratio: 2){.foo{color:red}}", flags),
            "@media (aspect-ratio:2){.foo{color:red}}"
        );
    }
}

#[test]
fn keeps_ratio_when_scaled_values_stay_non_integral() {
    for flags in [default_flags(), default_flags() | Options::CONVERT_RATIOS] {
        assert_eq!(
            minify_css(
                "@media (aspect-ratio: 0.0000001/0.000001){.foo{color:red}}",
                flags
            ),
            "@media (aspect-ratio:1e-7/.000001){.foo{color:red}}"
        );
    }
}

#[test]
fn still_reduces_when_scaling_reaches_integers() {
    for flags in [default_flags(), default_flags() | Options::CONVERT_RATIOS] {
        assert_eq!(
            minify_css("@media (aspect-ratio: 0.1/0.3){.foo{color:red}}", flags),
            "@media (aspect-ratio:1/3){.foo{color:red}}"
        );
    }
}

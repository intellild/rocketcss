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
fn minifies_valid_custom_colors_and_preserves_invalid_color_functions() {
    assert_eq!(
        run("a{--valid:rgb(0 0 0);--mixed:rgb(50%,23,54);\
             --bad-commas:rgb(0,,0,0);--bad-slashes:rgb(0/0/0)}"),
        "a{--valid:#000;--mixed:rgb(50%,23,54);--bad-commas:rgb(0,,0,0);--bad-slashes:rgb(0/0/0)}"
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
fn compacts_comments_and_whitespace_in_one_pass() {
    assert_eq!(
        run(
            "a{--idents: foo/**/ /**/bar ;--punct:fn( /**/a/**/,/**/b/**/ );\
             --multiply:1 * (2);--comment-multiply:1/**/*/**/(2);\
             --fallback:var(--x, fallback)}"
        ),
        "a{--idents:foo bar;--punct:fn(a,b);--multiply:1 * (2);\
           --comment-multiply:1*(2);--fallback:var(--x,fallback)}"
    );
}

#[test]
fn preserves_required_whitespace_between_custom_property_components() {
    const SOURCE: &str = ":root{--neutral-h:0;--neutral-s:0%;\
        --neutrals-1000:var(--neutral-h) var(--neutral-s) 100%;\
        --bg-surface-overlay:var(--neutrals-1000)}\
        .test{color:hsl(var(--bg-surface-overlay))}";
    const EXPECTED: &str = ":root{--neutral-h:0;--neutral-s:0%;\
        --neutrals-1000:var(--neutral-h) var(--neutral-s) 100%;\
        --bg-surface-overlay:var(--neutrals-1000)}\
        .test{color:hsl(var(--bg-surface-overlay))}";

    assert_eq!(run(SOURCE), EXPECTED);
}

#[test]
fn separator_compaction_respects_independent_options() {
    let mut discard_only = MinifyOptions::default();
    discard_only.flags.remove(Options::NORMALIZE_WHITESPACE);
    assert_eq!(
        run_with_options(
            "a{--x:foo/**/bar;--y:foo /**/ bar;--multiply:1/**/*/**/(2)}",
            discard_only
        ),
        "a{--x:foo bar;--y:foo  bar;--multiply:1*(2)}"
    );

    let mut normalize_only = MinifyOptions::default();
    normalize_only.flags.remove(Options::DISCARD_COMMENTS);
    assert_eq!(
        custom_property_token_shape("a{--x:  foo  /**/  bar  }", normalize_only),
        "iwcwi"
    );

    let mut preserve_fallback_space = MinifyOptions::default();
    preserve_fallback_space
        .flags
        .insert(Options::PRESERVE_VARIABLE_FALLBACK_SPACE);
    assert_eq!(
        run_with_options(
            "a{--comment-only:var(--x,/**/fallback);--authored-space:var(--x,/**/ fallback)}",
            preserve_fallback_space
        ),
        "a{--comment-only:var(--x,fallback);--authored-space:var(--x, fallback)}"
    );
    preserve_fallback_space
        .flags
        .remove(Options::NORMALIZE_WHITESPACE);
    assert_eq!(
        run_with_options(
            "a{--comment-only:var(--x,/**/fallback);--authored-space:var(--x,/**/ fallback)}",
            preserve_fallback_space
        ),
        "a{--comment-only:var(--x,fallback);--authored-space:var(--x, fallback)}"
    );

    let mut preserve_both = MinifyOptions::default();
    preserve_both
        .flags
        .remove(Options::DISCARD_COMMENTS | Options::NORMALIZE_WHITESPACE);
    assert_eq!(
        custom_property_token_shape("a{--x:foo/**/  bar}", preserve_both),
        "icWi"
    );
}

fn custom_property_token_shape(source: &str, options: MinifyOptions) -> String {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet =
            parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        minify(&mut stylesheet, &mut token, options);
        let rocketcss_ast::Declaration::Custom(property) = first_property_declaration(&stylesheet)
        else {
            panic!("expected custom property")
        };
        let property = stylesheet.resolve_node(*property);
        stylesheet
            .vec_iter(property.value)
            .map(|value| match value {
                rocketcss_ast::TokenOrValue::Token(token) => match stylesheet.resolve_node(token) {
                    rocketcss_ast::Token::Ident(_) => 'i',
                    rocketcss_ast::Token::WhiteSpace(value) if stylesheet.str(value) == " " => 'w',
                    rocketcss_ast::Token::WhiteSpace(_) => 'W',
                    rocketcss_ast::Token::Comment(_) => 'c',
                    _ => 't',
                },
                _ => 'v',
            })
            .collect()
    })
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

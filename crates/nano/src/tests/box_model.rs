use super::*;

#[test]
fn minifies_box_longhands_through_single_pass_ir() {
    assert_eq!(
        run("a{margin-top:10px;margin-right:20px;margin-bottom:10px;margin-left:20px}"),
        "a{margin:10px 20px}"
    );
    assert_eq!(
        run("a{padding-left:4px;padding-top:1px;padding-bottom:3px;padding-right:2px}"),
        "a{padding:1px 2px 3px 4px}"
    );
    assert_eq!(
        run("a{margin-top:1px;margin-right:2px;margin:3px}"),
        "a{margin:3px}"
    );
    assert_eq!(
        run("a{padding:1px;padding-left:2px}"),
        "a{padding:1px 1px 1px 2px}"
    );
    assert_eq!(
        run("a{margin:1px 2px;margin-left:2px}"),
        "a{margin:1px 2px}"
    );
    assert_eq!(
        run(
            "a{margin-top:1px!important;margin-right:1px!important;margin-bottom:1px!important;margin-left:1px!important}"
        ),
        "a{margin:1px !important}"
    );
    assert_eq!(
        run("a{margin-bottom:unset;margin-top:unset;margin-left:unset;margin-right:unset}"),
        "a{margin:unset}"
    );
}

#[test]
fn box_ir_preserves_fallback_and_logical_property_barriers() {
    assert_eq!(
        run("a{margin:inherit;margin-left:1px}"),
        "a{margin:inherit;margin-left:1px}"
    );
    assert_eq!(
        run("a{margin:1px;margin-left:var(--space)}"),
        "a{margin:1px;margin-left:var(--space)}"
    );
    assert_eq!(
        run("a{margin:1px;margin-left:var(--space);margin-left:2px}"),
        "a{margin:1px;margin-left:var(--space);margin-left:2px}"
    );
    assert_eq!(
        run("a{padding:1px;padding-top:var(--space);padding-top:2px}"),
        "a{padding:1px;padding-top:var(--space);padding-top:2px}"
    );
    assert_eq!(
        run("a{margin:1px;margin-left:var(--space);margin-right:2px}"),
        "a{margin:1px;margin-left:var(--space);margin-right:2px}"
    );
    assert_eq!(
        run("a{margin-left:1px;margin:invalid}"),
        "a{margin-left:1px;margin:invalid}"
    );
    assert_eq!(
        run("a{padding-left:1px;padding:auto}"),
        "a{padding-left:1px;padding:auto}"
    );
    assert_eq!(
        run(
            "a{margin-top:1px;margin-inline-start:2px;margin-right:3px;margin-bottom:4px;margin-left:5px}"
        ),
        "a{margin-inline-start:2px;margin:1px 3px 4px 5px}"
    );
    assert_eq!(
        run("a{padding-top:1px!important;padding-right:1px;padding-bottom:1px;padding-left:1px}"),
        "a{padding-top:1px !important;padding-right:1px;padding-bottom:1px;padding-left:1px}"
    );
}

#[test]
fn keeps_existing_token_storage() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{border:0 0 7px 7px solid black}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let (buffer_before, token_before) = unparsed_value_storage(&stylesheet);

        minify(&mut stylesheet, &mut token, MinifyOptions::default());

        let (buffer_after, token_after) = unparsed_value_storage(&stylesheet);
        assert_eq!(buffer_after, buffer_before);
        assert_eq!(token_after, token_before);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{border:0 0 7px 7px solid black}"
        );
    });
}

#[test]
#[ignore = "cross-block relational effect reification requires S4"]
fn runs_box_ir_across_adjacent_blocks() {
    assert_eq!(
        run("a{margin-top:1px;margin-right:2px}a{margin-bottom:3px;margin-left:4px}"),
        "a{margin:1px 2px 3px 4px}"
    );
    assert_eq!(
        run("a{padding:1px}a{padding-left:2px}"),
        "a{padding:1px 1px 1px 2px}"
    );
}

#[test]
fn s2_preserves_the_position_of_a_longhand_folded_into_a_shorthand() {
    assert_eq!(
        run(".a{margin:1px}div{margin-left:3px}.a{margin-left:2px}"),
        ".a{margin:1px}div{margin-left:3px}.a{margin-left:2px}"
    );
}

#[test]
fn s2_preserves_the_positions_of_box_longhands_merged_into_a_shorthand() {
    assert_eq!(
        run(
            ".a{margin-top:1px}div{margin-top:9px}.a{margin-right:2px}.bar{x:1}.a{margin-bottom:3px}.baz{x:1}.a{margin-left:4px}"
        ),
        ".a{margin-top:1px}div{margin-top:9px}.a{margin-right:2px}.bar{x:1}.a{margin-bottom:3px}.baz{x:1}.a{margin-left:4px}"
    );
}

#[test]
fn s2_preserves_parent_declaration_positions_around_a_nested_rule() {
    assert_eq!(
        run(".p{margin:1px;&{margin-left:3px}margin-left:2px}"),
        ".p{margin:1px;&{margin-left:3px}margin-left:2px}"
    );
}

fn unparsed_value_storage<'a>(
    stylesheet: &StyleSheet<'a>,
) -> (*const TokenOrValue<'a>, *const Token<'a>) {
    let Declaration::Unparsed(property) = first_property_declaration(stylesheet) else {
        panic!("expected unparsed property")
    };
    let TokenOrValue::Token(token) = &property.value[0] else {
        panic!("expected token value")
    };
    (property.value.as_ptr(), &**token)
}

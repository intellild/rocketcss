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
fn cross_rule_box_ir_respects_options_importance_and_opaque_barriers() {
    let mut disabled = MinifyOptions::default();
    disabled.flags.remove(Options::MERGE_ADJACENT_RULES);
    assert_eq!(
        run_with_options("a{padding:1px}a{padding-left:2px}", disabled),
        "a{padding:1px}a{padding-left:2px}"
    );
    assert_eq!(
        run("a{margin:1px!important}.x{display:block}a{margin-left:2px}"),
        "a{margin:1px !important}.x{display:block}a{margin-left:2px}"
    );
    assert_eq!(
        run(
            "a{margin:1px}.x{display:block}a{margin-left:var(--x)}.y{display:block}a{margin-left:2px}"
        ),
        "a{margin:1px}.x{display:block}a{margin-left:var(--x)}.y{display:block}a{margin-left:2px}"
    );
}

#[test]
fn cross_rule_box_ir_treats_opaque_shorthands_as_materialization_barriers() {
    assert_eq!(
        run("a{margin:inherit}.x{display:block}a{margin-left:2px}"),
        "a{margin:inherit}.x{display:block}a{margin-left:2px}"
    );
    assert_eq!(
        run("a{padding:unset}.x{display:block}a{padding-top:2px}"),
        "a{padding:unset}.x{display:block}a{padding-top:2px}"
    );
    assert_eq!(
        run("a{margin:var(--m)}.x{display:block}a{margin-right:2px}"),
        "a{margin:var(--m)}.x{display:block}a{margin-right:2px}"
    );
    assert_eq!(
        run("a{padding:var(--p)}.x{display:block}a{padding-bottom:2px}"),
        "a{padding:var(--p)}.x{display:block}a{padding-bottom:2px}"
    );
}

#[test]
fn s2_preserves_the_position_of_a_longhand_folded_into_a_shorthand() {
    assert_eq!(
        run(".a{margin:1px}div{margin-left:3px}.a{margin-left:2px}"),
        ".a{margin-top:1px;margin-right:1px;margin-bottom:1px}div{margin-left:3px}.a{margin-left:2px}"
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

#[test]
fn s5_materializes_every_partial_box_live_side_count() {
    assert_eq!(
        run(
            ".a{margin:1px 2px 3px 4px}.x{display:block}.a{margin-right:5px;margin-bottom:6px;margin-left:7px}"
        ),
        ".a{margin-top:1px}.x{display:block}.a{margin-right:5px;margin-bottom:6px;margin-left:7px}"
    );
    assert_eq!(
        run(".a{margin:1px 2px 3px 4px}.x{display:block}.a{margin-bottom:6px;margin-left:7px}"),
        ".a{margin-top:1px;margin-right:2px}.x{display:block}.a{margin-bottom:6px;margin-left:7px}"
    );
    assert_eq!(
        run(".a{margin:1px 2px 3px 4px}.x{display:block}.a{margin-left:7px}"),
        ".a{margin-top:1px;margin-right:2px;margin-bottom:3px}.x{display:block}.a{margin-left:7px}"
    );
    assert_eq!(
        run(
            ".a{padding:1px 2px 3px 4px}.x{display:block}.a{padding-right:5px;padding-bottom:6px;padding-left:7px}"
        ),
        ".a{padding-top:1px}.x{display:block}.a{padding-right:5px;padding-bottom:6px;padding-left:7px}"
    );
    assert_eq!(
        run(".a{padding:1px 2px 3px 4px}.x{display:block}.a{padding-bottom:6px;padding-left:7px}"),
        ".a{padding-top:1px;padding-right:2px}.x{display:block}.a{padding-bottom:6px;padding-left:7px}"
    );
    assert_eq!(
        run(".a{padding:1px 2px 3px 4px}.x{display:block}.a{padding-left:7px}"),
        ".a{padding-top:1px;padding-right:2px;padding-bottom:3px}.x{display:block}.a{padding-left:7px}"
    );
}

#[test]
fn s5_preserves_origin_position_importance_and_noncontiguous_links() {
    assert_eq!(
        run(".a{color:red;margin:1px;background:blue}.x{display:block}.a{margin-left:2px}"),
        ".a{color:red;margin-top:1px;margin-right:1px;margin-bottom:1px;background:#00f}.x{display:block}.a{margin-left:2px}"
    );
    assert_eq!(
        run(".a{padding:1px!important}.x{display:block}.a{padding-left:2px!important}"),
        ".a{padding-top:1px !important;padding-right:1px !important;padding-bottom:1px !important}.x{display:block}.a{padding-left:2px !important}"
    );
    assert_eq!(
        run(
            "a{width:1px;color:red}b{color:red}a,b{padding:1px}.x{display:block}a,b{padding-left:2px}"
        ),
        "a{width:1px}a,b{color:red;padding-top:1px;padding-right:1px;padding-bottom:1px}.x{display:block}a,b{padding-left:2px}"
    );
}

#[test]
fn s5_output_is_byte_idempotent_after_reparse() {
    let source = ".a{margin:1px;color:red}.b{color:red}.a{margin-left:2px;padding:3px}.x{display:block}.a{padding-top:4px}";
    let once = run(source);
    let twice = run(&once);

    assert_eq!(twice, once);

    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut compilation =
            parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        minify(&mut compilation, &mut token, MinifyOptions::default());
        assert_eq!(compilation.validate_ast(), Ok(()));
    });
}

fn unparsed_value_storage<'a>(
    stylesheet: &Compilation<'a>,
) -> (*const TokenOrValue<'a>, *const Token<'a>) {
    let Declaration::Unparsed(property) = first_property_declaration(stylesheet) else {
        panic!("expected unparsed property")
    };
    let property = stylesheet.resolve_node(*property);
    let values = stylesheet.vec(property.value);
    let TokenOrValue::Token(token) = &values[0] else {
        panic!("expected token value")
    };
    (
        values.as_ptr(),
        stylesheet.resolve_node(*token) as *const Token<'a>,
    )
}

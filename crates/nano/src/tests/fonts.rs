use super::*;

#[test]
#[ignore]
fn deduplicates_equivalent_font_families() {
    assert_eq!(
        run("a{font-family:\"A\",Arial,a,sans-serif}"),
        "a{font-family:A,Arial,sans-serif}"
    );
    assert_eq!(
        run("a{font-family:\"serif\",serif}"),
        "a{font-family:\"serif\",serif}"
    );
    assert_eq!(
        run("a{font-family:A,A,serif,Helvetica}"),
        "a{font-family:A,serif}"
    );
    assert_eq!(
        run("a{font-family:monospace,monospace}"),
        "a{font-family:monospace}"
    );
    assert_eq!(
        run("a{font-family:A,var(--family),a,serif}"),
        "a{font-family:A,var(--family),a,serif}"
    );
    assert_eq!(
        run("a{font-family:A,serif,Helvetica;font-family:A,serif}"),
        "a{font-family:A,serif}"
    );
    assert_eq!(
        run("a{font-family:Inter,system-ui,sans-serif}"),
        "a{font-family:Inter,system-ui,sans-serif}"
    );

    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{font-family:A,var(--family),a,serif}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        minify(&mut stylesheet, &mut token, MinifyOptions::default());
        let Declaration::Unparsed(value) = first_property_declaration(&stylesheet) else {
            panic!("expected opaque font-family declaration")
        };
        assert_eq!(value.reason, UnparsedPropertyReason::OpaqueValue);
    });
}

#[test]
fn preserves_opaque_font_family_declarations() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            "a{font-family:var(--family);font-family:slab inherit}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let stats = minify(&mut stylesheet, &mut token, MinifyOptions::default());
        assert_eq!(stats.declarations_removed, 0);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{font-family:var(--family);font-family:slab inherit}"
        );
    });
}

#[test]
fn font_family_deduplication_is_configurable() {
    let mut options = MinifyOptions::default();
    options.flags.remove(Options::DEDUPLICATE_LISTS);

    assert_eq!(
        run_with_options("a{font-family:\"A\",Arial,a,sans-serif,Helvetica}", options),
        "a{font-family:A,Arial,a,sans-serif}"
    );

    let mut options = MinifyOptions::default();
    options.flags.remove(Options::NORMALIZE_VALUES);
    assert_eq!(
        run_with_options("a{font-family:A,A,serif,Helvetica}", options),
        "a{font-family:A,serif,Helvetica}"
    );
}

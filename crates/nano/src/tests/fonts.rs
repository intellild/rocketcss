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
        "a{font-family:A,var(--family),serif}"
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
        let Declaration::FontFamily(families) = first_property_declaration(&stylesheet) else {
            panic!("expected typed font-family declaration")
        };
        assert!(matches!(families[0], FontFamily::Custom("A")));
        assert!(matches!(families[1], FontFamily::Unparsed(_)));
        assert!(matches!(families[2], FontFamily::Tombstone));
        assert!(matches!(families[3], FontFamily::Serif));
    });
}

#[test]
fn removes_font_family_declarations_containing_only_tombstones() {
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
        assert!(
            stylesheet
                .declarations_in_block(first_declaration_block_id(&stylesheet))
                .unwrap()
                .all(|declaration| matches!(
                    declaration.payload(),
                    radix_ast::DeclarationPayload::Property(Declaration::Tombstone)
                ))
        );
        assert_eq!(stats.declarations_removed, 2);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{}"
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

use rocketcss_parser::prelude::*;

#[test]
fn parser_decodes_values_from_token_spans() {
    let allocator = Allocator::new();
    let mut input = ParserInput::new(
        r#"\66 oo "b\61 r" -1.5e2PX 2furlong 25% url(icon\2e svg)"#,
        &allocator,
    );
    let mut parser = Parser::new(&mut input);

    assert_eq!(parser.expect_ident(), Ok("foo"));
    assert_eq!(parser.expect_string(), Ok("bar"));
    assert!(matches!(
        parser.next(),
        Ok(ValueToken::Dimension {
            unit: Unit::Length(LengthUnit::Px),
            value,
        }) if *value == -150.0
    ));
    assert!(matches!(
        parser.next(),
        Ok(ValueToken::UnknownDimension { unit: "furlong", value }) if *value == 2.0
    ));
    assert_eq!(parser.expect_percentage(), Ok(0.25));
    assert_eq!(parser.expect_url(), Ok("icon.svg"));
    assert!(parser.is_exhausted());
}

#[test]
fn parser_backtracks_and_parses_nested_blocks() {
    let allocator = Allocator::new();
    let mut input = ParserInput::new("foo(1, [bar]) tail", &allocator);
    let mut parser = Parser::new(&mut input);

    let state = parser.state();
    assert_eq!(parser.expect_function(), Ok("foo"));
    let values = parser
        .parse_nested_block(|input| {
            let first = input.expect_number()?;
            input.expect_comma()?;
            input.expect_square_bracket_block()?;
            let inner = input.parse_nested_block(|input| {
                Ok::<_, rocketcss_parser::ParseError<'_, ()>>(input.expect_ident()?)
            })?;
            Ok::<_, rocketcss_parser::ParseError<'_, ()>>((first, inner))
        })
        .unwrap();
    assert_eq!(values, (1.0, "bar"));
    assert_eq!(parser.expect_ident(), Ok("tail"));

    parser.reset(&state);
    assert_eq!(parser.expect_function(), Ok("foo"));
}

#[test]
fn delimited_parse_does_not_stop_inside_nested_blocks() {
    let allocator = Allocator::new();
    let mut input = ParserInput::new("one(foo;bar);two", &allocator);
    let mut parser = Parser::new(&mut input);
    let raw = parser
        .parse_until_before(rocketcss_parser::Delimiter::Semicolon, |input| {
            let start = input.position();
            while input.next().is_ok() {}
            Ok::<_, rocketcss_parser::ParseError<'_, ()>>(input.slice_from(start))
        })
        .unwrap();

    assert_eq!(raw, "one(foo;bar)");
    parser.expect_semicolon().unwrap();
    assert_eq!(parser.expect_ident(), Ok("two"));
}

#[test]
fn parses_style_rule_selectors_and_declarations() {
    let allocator = Allocator::new();
    let source =
        "/*! license */ .Foo, #app > a:hover { color: red; opacity: .5 !important; --gap: 1rem; }";
    let sheet = parse(
        source,
        &allocator,
        ParserOptions {
            filename: "input.css",
            ..ParserOptions::default()
        },
    )
    .unwrap();

    assert_eq!(&*sheet.license_comments, ["! license "]);
    assert_eq!(&*sheet.sources, ["input.css"]);
    assert_eq!(sheet.rules.len(), 1);
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.span, Span::new(15, source.len() as u32));
    assert_eq!(rule.selectors.len(), 2);
    assert!(matches!(
        &rule.selectors[0][0],
        SelectorComponent::Class(name) if *name == "Foo"
    ));
    assert!(matches!(
        &rule.selectors[1][1],
        SelectorComponent::Combinator(Combinator::Child)
    ));
    assert!(matches!(
        &rule.selectors[1][3],
        SelectorComponent::PseudoClass(value) if matches!(**value, PseudoClass::Hover)
    ));

    assert_eq!(rule.declarations.declarations.len(), 3);
    assert_eq!(rule.declarations.declarations_importance.len(), 3);
    assert!(matches!(
        &rule.declarations.declarations[0],
        Declaration::Color(value)
            if matches!(**value, rocketcss_ast::CssColor::Known(KnownColor::Red))
    ));
    assert!(matches!(
        &rule.declarations.declarations[1],
        Declaration::Opacity(0.5)
    ));
    assert!(matches!(
        &rule.declarations.declarations[2],
        Declaration::Custom(value)
            if matches!(*value.name, CustomPropertyName::Custom("--gap"))
    ));
    assert!(!rule.declarations.is_important(0));
    assert!(rule.declarations.is_important(1));
    assert!(!rule.declarations.is_important(2));
}

#[test]
fn parses_named_colors_as_known_color_nodes() {
    let allocator = Allocator::new();
    let sheet = parse(
        "a { color: blue; background-color: lightgreen; background: blue }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };

    assert!(matches!(
        &rule.declarations.declarations[0],
        Declaration::Color(value)
            if matches!(**value, CssColor::Known(KnownColor::Blue))
    ));
    assert!(matches!(
        &rule.declarations.declarations[1],
        Declaration::BackgroundColor(value)
            if matches!(**value, CssColor::Known(KnownColor::Lightgreen))
    ));
    assert!(matches!(
        &rule.declarations.declarations[2],
        Declaration::Background(values)
            if matches!(
                &*values[0].color,
                CssColor::Known(KnownColor::Blue)
            )
    ));
}

#[test]
fn escaped_selector_and_function_values_are_decoded_in_ast() {
    let allocator = Allocator::new();
    let sheet = parse(
        r#".f\6f o { width: calc(100% - var(--gap)); }"#,
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert!(matches!(
        &rule.selectors[0][0],
        SelectorComponent::Class("foo")
    ));

    let Declaration::Unparsed(width) = &rule.declarations.declarations[0] else {
        panic!("expected unparsed width")
    };
    assert!(matches!(
        &width.value[0],
        TokenOrValue::Function(function)
            if function.name() == "calc"
                && function.arguments.iter().any(|value| matches!(
                    value,
                    TokenOrValue::Function(nested) if nested.name() == "var"
                ))
    ));
}

#[test]
fn parses_import_media_unknown_and_font_face_rules() {
    let allocator = Allocator::new();
    let source = r#"
        @import url("a.css") screen;
        @media only screen and (min-width: 10px) { .a { display: block } }
        @font-face { font-family: "Demo"; src: url(demo.woff2); }
        @unknown foo(1) { bar: baz }
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 4);

    let CssRule::Import(rule) = &sheet.rules[0] else {
        panic!("expected import")
    };
    assert_eq!(rule.url, "a.css");
    assert!(matches!(
        rule.media
            .as_ref()
            .map(|media| &media.media_queries[0].media_type),
        Some(MediaType::Screen)
    ));

    let CssRule::Media(rule) = &sheet.rules[1] else {
        panic!("expected media")
    };
    assert_eq!(rule.rules.len(), 1);
    assert!(matches!(
        rule.query.media_queries[0].media_type,
        MediaType::Screen
    ));
    assert!(rule.query.media_queries[0].condition.is_some());

    let CssRule::FontFace(rule) = &sheet.rules[2] else {
        panic!("expected font-face")
    };
    assert_eq!(rule.properties.len(), 2);
    assert!(matches!(
        &rule.properties[0],
        FontFaceProperty::Custom(value)
            if matches!(*value.name, CustomPropertyName::Unknown("font-family"))
    ));

    let CssRule::Unknown(rule) = &sheet.rules[3] else {
        panic!("expected unknown at-rule")
    };
    assert_eq!(rule.name, "unknown");
    assert!(rule.block.is_some());
}

#[test]
fn parses_typed_media_conditions_and_features() {
    let allocator = Allocator::new();
    let source = r#"
        @media (width >= 600px) and (orientation: landscape),
               not (hover),
               (400px < width <= 1000px),
               screen and (resolution: 2dppx),
               (max-width: env(--narrow, 10px)) {}
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    let CssRule::Media(rule) = &sheet.rules[0] else {
        panic!("expected media rule")
    };
    assert_eq!(rule.query.media_queries.len(), 5);

    assert!(matches!(
        rule.query.media_queries[0].condition.as_deref(),
        Some(MediaCondition::Operation {
            operator: Operator::And,
            conditions,
        }) if matches!(
            &conditions[0],
            MediaCondition::Feature(feature)
                if matches!(
                    &**feature,
                    QueryFeature::Range {
                        name: MediaFeatureName::Standard(MediaFeatureId::Width),
                        operator: MediaFeatureComparison::GreaterThanEqual,
                        value,
                    } if matches!(
                        value,
                        MediaFeatureValue::Length(Length::Value(length))
                            if length.value == 600.0 && length.unit == LengthUnit::Px
                    )
                )
        )
    ));
    assert!(matches!(
        rule.query.media_queries[1].condition.as_deref(),
        Some(MediaCondition::Not(condition))
            if matches!(
                &**condition,
                MediaCondition::Feature(feature)
                    if matches!(
                        &**feature,
                        QueryFeature::Boolean {
                            name: MediaFeatureName::Standard(MediaFeatureId::Hover)
                        }
                    )
            )
    ));
    assert!(matches!(
        rule.query.media_queries[2].condition.as_deref(),
        Some(MediaCondition::Feature(feature))
            if matches!(&**feature, QueryFeature::Interval {
                name: MediaFeatureName::Standard(MediaFeatureId::Width),
                start_operator: MediaFeatureComparison::LessThan,
                end_operator: MediaFeatureComparison::LessThanEqual,
                ..
            })
    ));
    assert!(matches!(
        rule.query.media_queries[3].media_type,
        MediaType::Screen
    ));
    assert!(matches!(
        rule.query.media_queries[3].condition.as_deref(),
        Some(MediaCondition::Feature(feature))
            if matches!(
                &**feature,
                QueryFeature::Plain {
                    name: MediaFeatureName::Standard(MediaFeatureId::Resolution),
                    value,
                } if matches!(value, MediaFeatureValue::Resolution(Resolution::Dppx(2.0)))
            )
    ));
    assert!(matches!(
        rule.query.media_queries[4].condition.as_deref(),
        Some(MediaCondition::Feature(feature))
            if matches!(
                &**feature,
                QueryFeature::Range {
                    name: MediaFeatureName::Standard(MediaFeatureId::Width),
                    operator: MediaFeatureComparison::LessThanEqual,
                    value,
                } if matches!(value, MediaFeatureValue::Env(_))
            )
    ));
}

#[test]
fn invalid_selector_reports_source_location() {
    let allocator = Allocator::new();
    let error = parse(
        "a, { color: red }",
        &allocator,
        ParserOptions {
            filename: "broken.css",
            ..ParserOptions::default()
        },
    )
    .unwrap_err();

    assert_eq!(error.filename, "broken.css");
    assert_eq!(error.location.line, 0);
    assert_eq!(error.location.column, 4);
    assert!(matches!(
        error.kind,
        rocketcss_parser::ParserError::InvalidSelector
    ));
}

#[test]
fn selector_error_recovery_preserves_a_pure_invalid_selector() {
    let allocator = Allocator::new();
    let sheet = parse(
        "(font-[family-name:var(--font-*)]) { color: red }",
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected recovered style rule")
    };
    assert!(matches!(
        &rule.selectors[0],
        Selector::Unparsed("(font-[family-name:var(--font-*)])")
    ));
    assert!(matches!(
        &rule.declarations.declarations[0],
        Declaration::Color(_)
    ));
}

#[test]
fn selector_error_recovery_continues_at_commas() {
    let allocator = Allocator::new();
    let sheet = parse(
        ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected recovered style rule")
    };
    assert_eq!(rule.selectors.len(), 3);
    assert!(matches!(&rule.selectors[0], Selector::Parsed(_)));
    assert!(matches!(
        &rule.selectors[1],
        Selector::Unparsed("(font-[family-name:var(--font-*)])")
    ));
    assert!(matches!(&rule.selectors[2], Selector::Parsed(_)));
}

#[test]
fn selector_error_recovery_consumes_multiple_invalid_tokens() {
    let allocator = Allocator::new();
    let sheet = parse(
        ".valid, .broken ?? trailing, #also-valid { color: red }",
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();

    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected recovered style rule")
    };
    assert_eq!(rule.selectors.len(), 3);
    assert!(matches!(&rule.selectors[0], Selector::Parsed(_)));
    assert!(matches!(
        &rule.selectors[1],
        Selector::Unparsed(".broken ?? trailing")
    ));
    assert!(matches!(&rule.selectors[2], Selector::Parsed(_)));
}

#[test]
fn invalid_selector_still_fails_without_error_recovery() {
    let allocator = Allocator::new();
    let error = parse(
        "(font-[family-name:var(--font-*)]) { color: red }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(error.kind, ParserError::InvalidSelector));
}

#[test]
fn parser_reports_unmatched_closing_token() {
    let allocator = Allocator::new();
    let mut input = ParserInput::new(")", &allocator);
    let mut parser = Parser::new(&mut input);
    let error = parser.expect_no_error_token().unwrap_err();
    assert!(matches!(
        error.kind,
        BasicParseErrorKind::UnexpectedToken(token)
            if token.span == Span::new(0, 1)
    ));
}

#[test]
fn lightningcss_parse_trait_parses_values_from_strings() {
    let allocator = Allocator::new();
    let selectors = rocketcss_ast::SelectorList::parse_string(".a:is(.b, #c)", &allocator).unwrap();
    assert_eq!(selectors.len(), 1);
    assert!(matches!(
        &selectors[0][1],
        SelectorComponent::Is(list) if list.len() == 2
    ));
}

#[test]
fn parses_namespace_deep_and_empty_where_selectors() {
    let allocator = Allocator::new();
    let selectors = SelectorList::parse_string(
        "|e, *|*, svg|circle, [svg|fill=red], .a /deep/ .b, foo:where()",
        &allocator,
    )
    .unwrap();

    assert!(matches!(
        &selectors[0][..],
        [
            SelectorComponent::ExplicitNoNamespace,
            SelectorComponent::LocalName { name: "e", .. }
        ]
    ));
    assert!(matches!(
        &selectors[1][..],
        [
            SelectorComponent::ExplicitAnyNamespace,
            SelectorComponent::ExplicitUniversalType
        ]
    ));
    assert!(matches!(
        &selectors[2][..],
        [
            SelectorComponent::Namespace { prefix: "svg", .. },
            SelectorComponent::LocalName { name: "circle", .. }
        ]
    ));
    assert!(matches!(
        &selectors[3][0],
        SelectorComponent::AttributeOther(attribute)
            if matches!(&attribute.namespace, Some(NamespaceConstraint::Specific { prefix: "svg", .. }))
    ));
    assert!(matches!(
        &selectors[4][..],
        [
            SelectorComponent::Class("a"),
            SelectorComponent::Combinator(Combinator::Deep),
            SelectorComponent::Class("b")
        ]
    ));
    assert!(matches!(
        &selectors[5][..],
        [SelectorComponent::LocalName { name: "foo", .. }, SelectorComponent::Where(list)] if list.is_empty()
    ));
}

#[test]
fn parses_selection_and_placeholder_vendor_prefixes_into_typed_selectors() {
    let allocator = Allocator::new();
    let selectors = SelectorList::parse_string(
        "::-MoZ-selection,::-webkit-placeholder,::placeholder",
        &allocator,
    )
    .unwrap();

    assert!(matches!(
        &selectors[0][0],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Selection(VendorPrefix::MOZ))
    ));
    assert!(matches!(
        &selectors[1][0],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Placeholder(VendorPrefix::WEBKIT))
    ));
    assert!(matches!(
        &selectors[2][0],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Placeholder(VendorPrefix::NONE))
    ));
}

#[test]
fn parses_timeline_range_keyframes_and_skips_invalid_selectors() {
    let allocator = Allocator::new();
    let sheet = parse(
        "@keyframes demo { entry 0% { opacity: 0 } entry to { opacity: .5 } exit 100% { opacity: 1 } }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Keyframes(rule) = &sheet.rules[0] else {
        panic!("expected keyframes rule")
    };

    assert_eq!(rule.keyframes.len(), 2);
    assert!(matches!(
        &rule.keyframes[0].selectors[0],
        KeyframeSelector::TimelineRangePercentage(value)
            if value.name == TimelineRangeName::Entry && value.percentage == 0.0
    ));
    assert!(matches!(
        &rule.keyframes[1].selectors[0],
        KeyframeSelector::TimelineRangePercentage(value)
            if value.name == TimelineRangeName::Exit && value.percentage == 1.0
    ));
}

#[test]
fn parses_lightningcss_rule_families() {
    let allocator = Allocator::new();
    let source = r#"
        @namespace svg url(http://www.w3.org/2000/svg);
        @layer reset, theme.base;
        @layer components { button { color: blue } }
        @custom-media --narrow (max-width: 30em);
        @keyframes fade { from { opacity: 0 } 50% { opacity: .5 } to { opacity: 1 } }
        @counter-style thumbs { system: cyclic; symbols: "👍"; }
        @viewport { width: device-width; }
        @position-try --fallback { inset: 1rem; }
        @container card (width > 30rem) { .item { display: grid } }
        @-moz-document url-prefix() { a { color: green } }
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 10);

    assert!(matches!(
        &sheet.rules[0],
        CssRule::Namespace(rule)
            if rule.prefix == Some("svg") && rule.url == "http://www.w3.org/2000/svg"
    ));
    assert!(matches!(
        &sheet.rules[1],
        CssRule::LayerStatement(rule)
            if rule.names.len() == 2 && rule.names[1].as_slice() == ["theme", "base"]
    ));
    assert!(matches!(
        &sheet.rules[2],
        CssRule::LayerBlock(rule)
            if rule.name.is_some() && rule.rules.len() == 1
    ));
    assert!(matches!(
        &sheet.rules[3],
        CssRule::CustomMedia(rule)
            if rule.name == "--narrow" && rule.query.media_queries.len() == 1
    ));
    assert!(matches!(
        &sheet.rules[4],
        CssRule::Keyframes(rule)
            if matches!(*rule.name, rocketcss_ast::KeyframesName::Ident("fade"))
                && rule.keyframes.len() == 3
                && matches!(rule.keyframes[1].selectors[0], rocketcss_ast::KeyframeSelector::Percentage(0.5))
    ));
    assert!(matches!(&sheet.rules[5], CssRule::CounterStyle(_)));
    assert!(matches!(&sheet.rules[6], CssRule::Viewport(_)));
    assert!(matches!(
        &sheet.rules[7],
        CssRule::PositionTry(rule) if rule.name == "--fallback"
    ));
    assert!(matches!(
        &sheet.rules[8],
        CssRule::Container(rule) if rule.name == Some("card") && rule.condition.is_some()
    ));
    assert!(matches!(
        &sheet.rules[9],
        CssRule::MozDocument(rule) if rule.rules.len() == 1
    ));
}

#[test]
fn parses_import_modifiers_scope_and_page() {
    let allocator = Allocator::new();
    let source = r#"
        @import "theme.css" layer(theme.base) supports(display: grid) print;
        @scope (.card) to (.boundary) { .title { color: red } }
        @page invoice:first { margin: 1cm; @top-center { content: "Invoice"; } }
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 3);

    let CssRule::Import(import) = &sheet.rules[0] else {
        panic!("expected import")
    };
    assert_eq!(import.layer.as_deref(), Some(&["theme", "base"][..]));
    assert!(import.supports.is_some());
    assert!(matches!(
        import
            .media
            .as_ref()
            .map(|media| &media.media_queries[0].media_type),
        Some(MediaType::Print)
    ));

    assert!(matches!(
        &sheet.rules[1],
        CssRule::Scope(rule)
            if rule.scope_start.is_some() && rule.scope_end.is_some() && rule.rules.len() == 1
    ));
    assert!(matches!(
        &sheet.rules[2],
        CssRule::Page(rule)
            if rule.selectors.len() == 1
                && rule.declarations.declarations.len() == 1
                && rule.rules.len() == 1
    ));
}

#[test]
fn enforces_import_and_namespace_order_like_lightningcss() {
    let allocator = Allocator::new();
    let import_error = parse(
        "a {} @import 'late.css';",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap_err();
    assert!(matches!(
        import_error.kind,
        rocketcss_parser::ParserError::UnexpectedImportRule
    ));

    let namespace_error = parse(
        "a {} @namespace svg 'urn:svg';",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap_err();
    assert!(matches!(
        namespace_error.kind,
        rocketcss_parser::ParserError::UnexpectedNamespaceRule
    ));

    let valid = parse(
        "@charset 'UTF-8'; @layer reset; @import 'theme.css'; @namespace svg 'urn:svg'; a {}",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    assert_eq!(valid.rules.len(), 5);
    assert!(matches!(
        &valid.rules[0],
        CssRule::Charset(rule)
            if rule.encoding == "UTF-8" && rule.span == Span::new(0, 17)
    ));

    let interrupted_import = parse(
        "@import \"a.css\";\n@layer reset,base;\n@import \"b.css\" layer(base);",
        &allocator,
        ParserOptions {
            filename: "layers.css",
            ..ParserOptions::default()
        },
    )
    .unwrap_err();
    assert!(matches!(
        interrupted_import.kind,
        rocketcss_parser::ParserError::UnexpectedImportRule
    ));
    assert_eq!(interrupted_import.filename, "layers.css");
    assert_eq!(interrupted_import.location.line, 2);
    assert!(
        interrupted_import
            .to_string()
            .contains("initial @layer statements")
    );

    let initial_layers = parse(
        "@layer reset,base;@import \"a.css\";@import \"b.css\";",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    assert_eq!(initial_layers.rules.len(), 3);
}

#[test]
fn parses_charset_as_a_typed_rule() {
    let allocator = Allocator::new();
    let sheet = parse(
        r#"@charset "UTF-\38 ";"#,
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();

    assert!(matches!(
        &sheet.rules[..],
        [CssRule::Charset(rule)]
            if rule.encoding == "UTF-8" && rule.span == Span::new(0, 20)
    ));

    assert!(parse("@charset UTF-8;", &allocator, ParserOptions::default(),).is_err());
}

#[test]
fn parses_declarations_inside_nested_group_rules() {
    let allocator = Allocator::new();
    let sheet = parse(
        ".card { @media (width > 30rem) { color: red; & .title { opacity: .8 } } }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    let CssRule::Media(media) = &style.rules[0] else {
        panic!("expected nested media")
    };
    assert!(matches!(
        &media.rules[0],
        CssRule::NestedDeclarations(rule)
            if rule.declarations.declarations.len() == 1
    ));
    assert!(matches!(&media.rules[1], CssRule::Style(_)));
}

#[test]
fn distinguishes_nested_pseudo_selectors_from_declarations() {
    let allocator = Allocator::new();
    let sheet = parse(
        ".card { color: red; button:hover { color: blue } }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };
    assert_eq!(style.declarations.declarations.len(), 1);
    assert_eq!(style.rules.len(), 1);
    assert!(matches!(&style.rules[0], CssRule::Style(_)));
}

#[test]
fn declaration_error_recovery_continues_at_semicolon() {
    let allocator = Allocator::new();
    let sheet = parse(
        "a { broken value; width: 10px; }",
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };
    assert_eq!(style.declarations.declarations.len(), 1);
    assert!(matches!(
        &style.declarations.declarations[0],
        Declaration::Width(_)
    ));
}

#[test]
#[ignore]
fn declaration_like_identifier_requires_explicit_error_recovery() {
    let allocator = Allocator::new();
    let source = r#"div {
        width: 100px;
        height: 100px;
        background: #dd6b4d;
        fhbj32brjb3;
    }"#;

    let error = parse(source, &allocator, ParserOptions::default()).unwrap_err();
    assert!(matches!(
        error.kind,
        rocketcss_parser::ParserError::InvalidDeclaration
    ));

    let sheet = parse(
        source,
        &allocator,
        ParserOptions {
            error_recovery: true,
            ..ParserOptions::default()
        },
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };

    assert_eq!(style.declarations.declarations.len(), 3);
    assert!(style.rules.is_empty());
    assert!(matches!(
        style.declarations.declarations[0],
        Declaration::Width(_)
    ));
    assert!(matches!(
        style.declarations.declarations[1],
        Declaration::Height(_)
    ));
    assert!(matches!(
        &style.declarations.declarations[2],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Background)
    ));
}

#[test]
fn parses_typed_core_property_values() {
    let allocator = Allocator::new();
    let sheet = parse(
        "a { color: #0f08; background-color: currentColor; display: inline-flex; visibility: hidden; width: 10rem; height: 25%; all: revert-layer; }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };
    let declarations = &style.declarations.declarations;
    assert!(matches!(
        &declarations[0],
        Declaration::Color(color)
            if matches!(**color, rocketcss_ast::CssColor::Rgba(rocketcss_ast::RGBA { red: 0, green: 255, blue: 0, alpha: 136 }))
    ));
    assert!(matches!(
        &declarations[1],
        Declaration::BackgroundColor(color)
            if matches!(**color, rocketcss_ast::CssColor::CurrentColor)
    ));
    assert!(matches!(&declarations[2], Declaration::Display(_)));
    assert!(matches!(
        &declarations[3],
        Declaration::Visibility(rocketcss_ast::Visibility::Hidden)
    ));
    assert!(matches!(&declarations[4], Declaration::Width(_)));
    assert!(matches!(&declarations[5], Declaration::Height(_)));
    assert!(matches!(
        &declarations[6],
        Declaration::All(CSSWideKeyword::RevertLayer)
    ));
}

#[test]
fn parses_font_family_into_typed_ast_nodes() {
    let allocator = Allocator::new();
    let sheet = parse(
        r#"a { font-family: "serif", SANS-SERIF, Fancy Font, "A", "slab inherit"; font-family: var(--family), sans-serif; font-family: slab inherit; }"#,
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };
    let declarations = &style.declarations.declarations;

    assert!(matches!(
        &declarations[0],
        Declaration::FontFamily(families)
            if matches!(families.as_slice(), [
                FontFamily::Custom("serif"),
                FontFamily::SansSerif,
                FontFamily::Custom("Fancy Font"),
                FontFamily::Custom("A"),
                FontFamily::Custom("slab inherit"),
            ])
    ));
    assert!(matches!(
        &declarations[1],
        Declaration::FontFamily(families)
            if matches!(families.as_slice(), [
                FontFamily::Unparsed(_),
                FontFamily::SansSerif,
            ])
    ));
    assert!(matches!(
        &declarations[2],
        Declaration::FontFamily(families)
            if matches!(families.as_slice(), [FontFamily::Unparsed(_)])
    ));
}

#[test]
fn parses_known_multicol_and_legacy_gap_ast_nodes() {
    let allocator = Allocator::new();
    let sheet = parse(
        "a { -webkit-column-rule: red solid 1px; columns: 3 10px; grid-column-gap: 10%; grid-row-gap: normal; columns: var(--count); column-width: INHERIT; columns: REVERT-LAYER; }",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style")
    };
    let declarations = &style.declarations.declarations;

    assert!(matches!(
        &declarations[0],
        Declaration::ColumnRule(value, prefix)
            if prefix.contains(VendorPrefix::WEBKIT)
                && matches!(value.style, Some(LineStyle::Solid))
                && value.width.is_some()
                && value.color.is_some()
    ));
    assert!(matches!(
        &declarations[1],
        Declaration::Columns(CSSWideOr::Value(value), prefix)
            if *prefix == VendorPrefix::NONE
                && matches!(value.count, ColumnCount::Integer(3))
                && matches!(&*value.width, ColumnWidth::Length(_))
    ));
    assert!(matches!(
        &declarations[2],
        Declaration::GridColumnGap(value)
            if matches!(&**value, GapValue::LengthPercentage(_))
    ));
    assert!(matches!(
        &declarations[3],
        Declaration::GridRowGap(value) if matches!(&**value, GapValue::Normal)
    ));
    assert!(matches!(
        &declarations[4],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Columns(VendorPrefix::NONE))
    ));
    assert!(matches!(
        &declarations[5],
        Declaration::ColumnWidth(CSSWideOr::CSSWide(CSSWideKeyword::Inherit), prefix)
            if *prefix == VendorPrefix::NONE
    ));
    assert!(matches!(
        &declarations[6],
        Declaration::Columns(CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer), prefix)
            if *prefix == VendorPrefix::NONE
    ));
}

#[test]
fn declaration_parsing_uses_property_ids_and_preserves_fallbacks() {
    let allocator = Allocator::new();
    let sheet = parse(
        r#"a {
            COLOR: red ! IMPORTANT;
            WIDTH: calc(100% - var(--gap)) !important;
            -WEBKIT-TRANSFORM: translateX(1px);
            future-property: fn(!important);
            --theme: fn(!important) !important;
            opacity: .5 !urgent;
            height: 10px;
        }"#,
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    let declarations = &style.declarations.declarations;

    assert_eq!(declarations.len(), 7);
    assert!(matches!(&declarations[0], Declaration::Color(_)));
    assert!(style.declarations.is_important(0));

    assert!(matches!(
        &declarations[1],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Width)
                && matches!(&value.value[0], TokenOrValue::Function(function)
                    if function.name().eq_ignore_ascii_case("calc"))
    ));
    assert!(style.declarations.is_important(1));
    assert!(matches!(
        &declarations[2],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Transform(prefix)
                if prefix.contains(VendorPrefix::WEBKIT))
    ));
    assert!(matches!(
        &declarations[3],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Custom("future-property"))
    ));
    assert!(matches!(
        &declarations[4],
        Declaration::Custom(value)
            if matches!(&*value.name, CustomPropertyName::Custom("--theme"))
                && value.value.iter().any(|token| matches!(token,
                    TokenOrValue::Function(function) if function.name() == "fn"))
    ));
    assert!(style.declarations.is_important(4));
    assert!(matches!(
        &declarations[5],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Opacity)
    ));
    assert!(!style.declarations.is_important(5));
    assert!(matches!(&declarations[6], Declaration::Height(_)));
}

#[test]
#[ignore = "the overlay property does not have typed metadata yet"]
fn recognizes_overlay_as_a_known_property() {
    let allocator = Allocator::new();
    let sheet = parse(
        "a{overlay:auto;overlay:var(--state)}",
        &allocator,
        ParserOptions::default(),
    )
    .unwrap();
    let CssRule::Style(style) = &sheet.rules[0] else {
        panic!("expected style rule")
    };

    assert!(style.declarations.declarations.iter().all(|declaration| {
        !matches!(
            declaration,
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Custom("overlay"))
        )
    }));
}

#[test]
fn parses_property_view_transition_palette_and_nest_rules() {
    let allocator = Allocator::new();
    let source = r#"
        @property --brand-color {
          syntax: "<color>";
          inherits: false;
          initial-value: red;
        }
        @view-transition { navigation: auto; types: forward backward; }
        @font-palette-values --dark { font-family: Demo; base-palette: 1; }
        @font-feature-values "Demo Sans" { @styleset { compact: 1 2; } }
        .card { @nest & > .title { color: blue; } }
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 5);
    assert!(matches!(
        &sheet.rules[0],
        CssRule::Property(rule)
            if rule.name == "--brand-color"
                && !rule.inherits
                && rule.initial_value.is_some()
                && matches!(*rule.syntax, rocketcss_ast::SyntaxString::Components(_))
    ));
    assert!(matches!(
        &sheet.rules[1],
        CssRule::ViewTransition(rule) if rule.properties.len() == 2
    ));
    assert!(matches!(
        &sheet.rules[2],
        CssRule::FontPaletteValues(rule)
            if rule.name == "--dark" && rule.properties.len() == 2
    ));
    assert!(matches!(
        &sheet.rules[3],
        CssRule::FontFeatureValues(rule)
            if rule.name.len() == 1
                && rule.name[0].0 == "Demo Sans"
                && rule.rules.len() == 1
                && rule.rules[0].declarations[0].values.as_slice() == [1, 2]
    ));
    let CssRule::Style(style) = &sheet.rules[4] else {
        panic!("expected style")
    };
    assert!(matches!(&style.rules[0], CssRule::Nesting(_)));
}

#[test]
#[ignore]
fn rejects_property_rules_nested_in_style_rules() {
    let allocator = Allocator::new();
    let error = parse(
        r#".example{@property --angle{syntax:"<angle>";inherits:true;initial-value:0turn}animation:spin 3s linear infinite}"#,
        &allocator,
        ParserOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error.kind,
        rocketcss_parser::ParserError::InvalidAtRule("property")
    ));
}

#[test]
#[ignore]
fn parses_property_initial_value_edge_cases_losslessly() {
    let allocator = Allocator::new();
    let source = r#"
        @property --omitted { syntax: "*"; inherits: false; }
        @property --empty { syntax: "*"; inherits: false; initial-value:; }
        @property --ordered {
          initial-value: 25px;
          inherits: true;
          syntax: "<length>";
        }
    "#;
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 3);

    let CssRule::Property(omitted) = &sheet.rules[0] else {
        panic!("expected omitted property registration")
    };
    assert!(omitted.initial_value.is_none());

    let CssRule::Property(empty) = &sheet.rules[1] else {
        panic!("expected empty property registration")
    };
    assert!(empty.initial_value.is_some());

    let CssRule::Property(ordered) = &sheet.rules[2] else {
        panic!("expected ordered property registration")
    };
    assert!(ordered.inherits);
    assert!(ordered.initial_value.is_some());
}

#[test]
fn extracts_source_directives_in_parser_layer() {
    let allocator = Allocator::new();
    let source =
        "a { color: red } /*# sourceURL=original.scss */ /*# sourceMappingURL=style.css.map */";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(&*sheet.source_map_urls, [Some("style.css.map")]);

    let mut input = ParserInput::new(source, &allocator);
    let mut parser = Parser::new(&mut input);
    while parser.next_including_whitespace_and_comments().is_ok() {}
    assert_eq!(parser.current_source_url(), Some("original.scss"));
    assert_eq!(parser.current_source_map_url(), Some("style.css.map"));
}

#[test]
#[ignore]
fn preserves_picker_pseudo_element_and_allows_chaining_pseudo_class() {
    let allocator = Allocator::new();
    let source = "select::picker(select):not(:popover-open) { color: red }";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 1);
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.selectors.len(), 1);

    let selector = &rule.selectors[0];
    assert_eq!(selector.len(), 3);

    assert!(matches!(
        &selector[0],
        SelectorComponent::LocalName { name: "select", .. }
    ));

    assert!(matches!(
        &selector[1],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::CustomFunction { name: "picker", .. })
    ));

    assert!(matches!(&selector[2], SelectorComponent::Negation(_)));

    assert_eq!(rule.declarations.declarations.len(), 1);
}

#[test]
#[ignore]
fn preserves_details_content_chained_with_before_pseudo_element() {
    let allocator = Allocator::new();
    let source = "::details-content::before { background-color: red }";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 1);
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.selectors.len(), 1);

    let selector = &rule.selectors[0];
    assert_eq!(selector.len(), 2);

    assert!(matches!(
        &selector[0],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Custom { name: "details-content" })
    ));

    assert!(matches!(
        &selector[1],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Before)
    ));

    let Declaration::BackgroundColor(_) = &rule.declarations.declarations[0] else {
        panic!("expected background-color declaration")
    };
}

#[test]
#[ignore]
fn preserves_has_slotted_pseudo_class() {
    let allocator = Allocator::new();
    let source = "slot:has-slotted { display: none }";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.selectors.len(), 1);

    let selector = &rule.selectors[0];
    assert_eq!(selector.len(), 2);

    assert!(matches!(
        &selector[0],
        SelectorComponent::LocalName { name: "slot", .. }
    ));

    assert!(matches!(
        &selector[1],
        SelectorComponent::PseudoClass(pc)
            if matches!(**pc, PseudoClass::Custom { name: "has-slotted" })
    ));
}

#[test]
#[ignore]
fn preserves_pseudo_element_arg_inside_has_selector() {
    let allocator = Allocator::new();
    let source = "video:not(:has(::backdrop)) { color: red }";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected style rule")
    };
    assert_eq!(rule.selectors.len(), 1);

    let selector = &rule.selectors[0];
    assert_eq!(selector.len(), 2);

    assert!(matches!(
        &selector[0],
        SelectorComponent::LocalName { name: "video", .. }
    ));

    assert!(matches!(&selector[1], SelectorComponent::Negation(_)));
}

#[test]
#[ignore]
fn preserves_scroll_button_and_scroll_marker_pseudo_elements() {
    let allocator = Allocator::new();
    let source = "::scroll-button { color: red } .carousel > *::scroll-marker { content: '' }";
    let sheet = parse(source, &allocator, ParserOptions::default()).unwrap();
    assert_eq!(sheet.rules.len(), 2);

    let CssRule::Style(rule) = &sheet.rules[0] else {
        panic!("expected scroll-button style rule")
    };
    assert!(matches!(
        &rule.selectors[0][0],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Custom { name: "scroll-button" })
    ));

    let CssRule::Style(rule) = &sheet.rules[1] else {
        panic!("expected scroll-marker style rule")
    };
    assert!(matches!(
        &rule.selectors[0][3],
        SelectorComponent::PseudoElement(element)
            if matches!(**element, PseudoElement::Custom { name: "scroll-marker" })
    ));
}

use rocketcss_ast::radix_ast::{CssRulePayload, DeclarationPayload, RuleId, RuleRecord};
use rocketcss_parser::parse;
use rocketcss_parser::prelude::*;

fn root_rule_ids(compilation: &Compilation<'_>) -> std::vec::Vec<RuleId> {
    compilation
        .rules_in_list(compilation.stylesheet().root_rules())
        .unwrap()
        .map(|(id, _)| id)
        .collect()
}

fn root_rule<'tree, 'ast>(
    compilation: &'tree Compilation<'ast>,
    index: usize,
) -> (RuleId, &'tree RuleRecord<CssRulePayload<'ast>>) {
    let id = root_rule_ids(compilation)[index];
    (id, compilation.rule(id).unwrap())
}

fn child_rule_ids(compilation: &Compilation<'_>, parent: RuleId) -> std::vec::Vec<RuleId> {
    let list = compilation.rule(parent).unwrap().child_list().unwrap();
    compilation
        .rules_in_list(list)
        .unwrap()
        .map(|(id, _)| id)
        .collect()
}

fn style_selectors<'tree, 'ast>(
    compilation: &'tree Compilation<'ast>,
    rule: RuleId,
) -> &'tree SelectorList<'ast> {
    let selector = match compilation.rule(rule).unwrap().payload() {
        CssRulePayload::Style(payload) => payload.selector_value,
        CssRulePayload::Nesting(payload) => payload.selector_value,
        _ => panic!("expected selector-owning rule"),
    };
    compilation.selector_value(selector).unwrap().selectors()
}

fn property_declarations<'tree, 'ast>(
    compilation: &'tree Compilation<'ast>,
    rule: RuleId,
) -> std::vec::Vec<(&'tree Declaration<'ast>, bool)> {
    let block = compilation
        .rule(rule)
        .and_then(|record| record.declaration_block())
        .expect("expected declaration-owning rule");
    compilation
        .declarations_in_block(block)
        .unwrap()
        .filter_map(|record| {
            record
                .payload()
                .as_property()
                .map(|declaration| (declaration, record.is_important()))
        })
        .collect()
}

fn declaration_values<'tree, 'ast>(
    compilation: &'tree Compilation<'ast>,
    rule: RuleId,
) -> std::vec::Vec<&'tree Declaration<'ast>> {
    property_declarations(compilation, rule)
        .into_iter()
        .map(|(declaration, _)| declaration)
        .collect()
}

fn expect_parse_error<'ast>(
    result: Result<Compilation<'ast>, rocketcss_parser::Error<'ast>>,
) -> rocketcss_parser::Error<'ast> {
    match result {
        Ok(_) => panic!("expected parsing to fail"),
        Err(error) => error,
    }
}

#[test]
fn parser_decodes_values_from_token_spans() {
    let allocator = Allocator::new();
    let mut parser = Compiler::new_with_source(
        r#"\66 oo "b\61 r" -1.5e2PX 2furlong 25% url(icon\2e svg)"#,
        &allocator,
    );

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
    let mut parser = Compiler::new_with_source("foo(1, [bar]) tail", &allocator);

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
    let mut parser = Compiler::new_with_source("one(foo;bar);two", &allocator);
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
    allocator.with_ghost(|mut token| {
        let source = "/*! license */ .Foo, #app > a:hover { color: red; opacity: .5 !important; --gap: 1rem; }";
        let mut compiler = Compiler::new(&allocator);
        let sheet = compiler
            .parse(
                source,
                &mut token,
                ParserOptions {
                    filename: "input.css",
                    ..ParserOptions::default()
                },
            )
            .unwrap();

        assert_eq!(sheet.license_comments(), ["! license "]);
        assert_eq!(compiler.source(), "input.css");
        assert_eq!(root_rule_ids(&sheet).len(), 1);
        let (rule_id, rule) = root_rule(&sheet, 0);
        let CssRulePayload::Style(payload) = rule.payload() else {
            panic!("expected style rule")
        };
        assert_eq!(payload.span, Span::new(15, source.len() as u32));
        let selectors = style_selectors(&sheet, rule_id);
        assert_eq!(selectors.len(), 2);
        assert!(matches!(
            &selectors[0][0],
            SelectorComponent::Class(name) if *name == "Foo"
        ));
        assert!(matches!(
            &selectors[1][1],
            SelectorComponent::Combinator(Combinator::Child)
        ));
        assert!(matches!(
            &selectors[1][3],
            SelectorComponent::PseudoClass(value) if matches!(**value, PseudoClass::Hover)
        ));

        let declarations = property_declarations(&sheet, rule_id);
        assert_eq!(declarations.len(), 3);
        assert!(matches!(
            declarations[0].0,
            Declaration::Color(value)
                if matches!(**value, rocketcss_ast::CssColor::Known(KnownColor::Red))
        ));
        assert!(matches!(declarations[1].0, Declaration::Opacity(0.5)));
        assert!(matches!(
            declarations[2].0,
            Declaration::Custom(value)
                if matches!(*value.name, CustomPropertyName::Custom("--gap"))
        ));
        assert_eq!(
            declarations
                .iter()
                .map(|(_, important)| *important)
                .collect::<std::vec::Vec<_>>(),
            [false, true, false]
        );
    })
}

#[test]
fn rgb_functions_are_reified_only_after_strict_validation() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a{--valid:rgb(0 0 0);--invalid:rgb(foo);\
             --bad-commas:rgb(0,,0,0);--bad-slashes:rgb(0/0/0);--raw:10.px}\
             b{color:rgb(0,,0,0);color:rgb(0/0/0)}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0)
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        assert!(matches!(
            declarations[0],
            Declaration::Custom(value)
                if matches!(
                    &value.value[..],
                    [TokenOrValue::Color(color)]
                        if matches!(
                            &**color,
                            CssColor::Function(function)
                                if function.kind() == KnownFunction::Rgb
                                    && function.is_valid_rgb()
                        )
                )
        ));
        for declaration in &declarations[1..4] {
            assert!(matches!(
                declaration,
                Declaration::Custom(value)
                    if matches!(
                        &value.value[..],
                        [TokenOrValue::Function(function)]
                            if function.kind() == KnownFunction::Rgb
                                && !function.is_valid_rgb()
                    )
            ));
        }
        assert!(matches!(
            declarations[4],
            Declaration::Custom(value)
                if value
                    .value
                    .iter()
                    .all(|value| matches!(value, TokenOrValue::Token(_)))
        ));

        for (declaration, _) in property_declarations(&sheet, root_rule(&sheet, 1).0) {
            assert!(matches!(
                declaration,
                Declaration::Unparsed(value)
                    if value.reason == UnparsedPropertyReason::OpaqueValue
                        && matches!(
                            &value.value[..],
                            [TokenOrValue::Function(function)]
                                if function.kind() == KnownFunction::Rgb
                                    && !function.is_valid_rgb()
                        )
            ));
        }
    })
}

#[test]
fn modern_rgb_accepts_mixed_and_missing_components() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a{--mixed:rgb(255 50% 0);--missing:rgb(none 50% 0/none);\
             --out-of-range:rgba(300 -10 0);--legacy-rgba:rgba(1,2,3);\
             color:rgb(255 50% 0);background-color:rgb(none 50% 0/none)}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0)
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        for declaration in &declarations[..4] {
            assert!(matches!(
                declaration,
                Declaration::Custom(value)
                    if matches!(
                        &value.value[..],
                        [TokenOrValue::Color(color)]
                            if matches!(
                                &**color,
                                CssColor::Function(function) if function.is_valid_rgb()
                            )
                    )
            ));
        }
        for declaration in &declarations[4..] {
            assert!(matches!(
                declaration,
                Declaration::Color(color) | Declaration::BackgroundColor(color)
                    if matches!(
                        &**color,
                        CssColor::Function(function) if function.is_valid_rgb()
                    )
            ));
        }
    })
}

#[test]
fn review_regressions_preserve_invalid_and_commented_declarations() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a{display:;display:none flow;display:table-cell flow;\
             transform:initial/**/;all:initial/**/;columns:initial/**/;\
             display:inline-block;display:-webkit-inline-box;display:-moz-inline-box}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0)
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        for declaration in &declarations[..3] {
            assert!(matches!(
                declaration,
                Declaration::Unparsed(value)
                    if value.reason == UnparsedPropertyReason::InvalidValue
            ));
        }
        for declaration in &declarations[3..6] {
            assert!(
                matches!(
                    declaration,
                    Declaration::Unparsed(value)
                        if value.reason == UnparsedPropertyReason::OpaqueValue
                            && value.value.iter().any(|value| matches!(
                                value,
                                TokenOrValue::Token(token)
                                    if matches!(**token, ValueToken::Comment(_))
                            ))
                ),
                "{declaration:?}"
            );
        }
        assert!(matches!(
            declarations[6],
            Declaration::Display(Display::Pair {
                outside: DisplayOutside::Inline,
                inside: DisplayInside::FlowRoot,
                is_list_item: false,
            })
        ));
        for (declaration, prefix) in [
            (declarations[7], VendorPrefix::WEBKIT),
            (declarations[8], VendorPrefix::MOZ),
        ] {
            assert!(matches!(
                declaration,
                Declaration::Display(Display::Pair {
                    outside: DisplayOutside::Inline,
                    inside: DisplayInside::Box { vendor_prefix },
                    is_list_item: false,
                }) if *vendor_prefix == prefix
            ));
        }
    })
}

#[test]
fn parses_named_colors_as_known_color_nodes() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { color: blue; background-color: lightgreen; background: blue }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let rule = root_rule(&sheet, 0).0;
        let declarations = property_declarations(&sheet, rule);

        assert!(matches!(
            declarations[0].0,
            Declaration::Color(value)
                if matches!(**value, CssColor::Known(KnownColor::Blue))
        ));
        assert!(matches!(
            declarations[1].0,
            Declaration::BackgroundColor(value)
                if matches!(**value, CssColor::Known(KnownColor::Lightgreen))
        ));
        assert!(matches!(
            declarations[2].0,
            Declaration::Background(values)
                if matches!(
                    &*values[0].color,
                    CssColor::Known(KnownColor::Blue)
                )
        ));
    })
}

#[test]
fn escaped_selector_and_function_values_are_decoded_in_ast() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            r#".f\6f o { width: calc(100% - var(--gap)); }"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let rule = root_rule(&sheet, 0).0;
        let selectors = style_selectors(&sheet, rule);
        assert!(matches!(
            &selectors[0][0],
            SelectorComponent::Class(name) if name == "foo"
        ));

        let declarations = property_declarations(&sheet, rule);
        let Declaration::Width(width) = declarations[0].0 else {
            panic!("expected typed width")
        };
        assert!(matches!(
            &**width,
            Size::MathFunction(function)
                if function.name() == "calc"
                    && function.arguments.iter().any(|value| matches!(
                        value,
                        TokenOrValue::Function(nested) if nested.name() == "var"
                    ))
        ));
    })
}

#[test]
fn compiler_interns_equal_selector_strings_to_one_atom() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut compiler = Compiler::new(&allocator);
        let sheet = compiler
            .parse(
                r#".foo,.f\6f o { color: red }"#,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        let SelectorComponent::Class(first) = &selectors[0][0] else {
            panic!("expected first class selector")
        };
        let SelectorComponent::Class(second) = &selectors[1][0] else {
            panic!("expected second class selector")
        };

        assert_eq!(first, second);
        assert!(std::ptr::eq(first.as_str(), second.as_str()));
    })
}

#[test]
fn scope_prelude_reuses_the_compiler_string_pool() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut compiler = Compiler::new(&allocator);
        let sheet = compiler
            .parse(
                "@scope (.shared){.shared{color:red}}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
        let (scope_id, scope) = root_rule(&sheet, 0);
        let CssRulePayload::Scope(scope) = scope.payload() else {
            panic!("expected scope rule")
        };
        let SelectorComponent::Class(scope_class) = &scope.scope_start.as_ref().unwrap()[0][0]
        else {
            panic!("expected scope class selector")
        };
        let scoped_rule = child_rule_ids(&sheet, scope_id)[0];
        let SelectorComponent::Class(rule_class) = &style_selectors(&sheet, scoped_rule)[0][0]
        else {
            panic!("expected style class selector")
        };

        assert_eq!(scope_class, rule_class);
        assert!(std::ptr::eq(scope_class.as_str(), rule_class.as_str()));
    })
}

#[test]
fn parses_import_media_unknown_and_font_face_rules() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = r#"
        @import url("a.css") screen;
        @media only screen and (min-width: 10px) { .a { display: block } }
        @font-face { font-family: "Demo"; src: url(demo.woff2); }
        @unknown foo(1) { bar: baz }
    "#;
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(root_rule_ids(&sheet).len(), 4);

        let CssRulePayload::Import(rule) = root_rule(&sheet, 0).1.payload() else {
            panic!("expected import")
        };
        assert_eq!(rule.url, "a.css");
        assert!(matches!(
            rule.media
                .as_ref()
                .map(|media| &media.media_queries[0].media_type),
            Some(MediaType::Screen)
        ));

        let (media_id, media) = root_rule(&sheet, 1);
        let CssRulePayload::Media(rule) = media.payload() else {
            panic!("expected media")
        };
        assert_eq!(child_rule_ids(&sheet, media_id).len(), 1);
        assert!(matches!(
            rule.query.media_queries[0].media_type,
            MediaType::Screen
        ));
        assert!(rule.query.media_queries[0].condition.is_some());

        let (font_face_id, font_face) = root_rule(&sheet, 2);
        let CssRulePayload::FontFace(_) = font_face.payload() else {
            panic!("expected font-face")
        };
        let font_face_block = sheet
            .rule(font_face_id)
            .unwrap()
            .declaration_block()
            .unwrap();
        let properties = sheet
            .declarations_in_block(font_face_block)
            .unwrap()
            .collect::<std::vec::Vec<_>>();
        assert_eq!(properties.len(), 2);
        assert!(matches!(
            properties[0].payload(),
            DeclarationPayload::FontFace(FontFaceProperty::Custom(value))
                if matches!(*value.name, CustomPropertyName::Unknown("font-family"))
        ));

        let CssRulePayload::Unknown(rule) = root_rule(&sheet, 3).1.payload() else {
            panic!("expected unknown at-rule")
        };
        assert_eq!(rule.name, "unknown");
        assert!(rule.block.is_some());
    })
}

#[test]
fn parses_typed_media_conditions_and_features() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = r#"
        @media (width >= 600px) and (orientation: landscape),
               not (hover),
               (400px < width <= 1000px),
               screen and (resolution: 2dppx),
               (max-width: env(--narrow, 10px)) {}
    "#;
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let CssRulePayload::Media(rule) = root_rule(&sheet, 0).1.payload() else {
            panic!("expected media rule")
        };
        assert_eq!(rule.query.media_queries.len(), 5);

        assert!(matches!(
            rule.query.media_queries[0].condition.as_ref(),
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
            rule.query.media_queries[1].condition.as_ref(),
            Some(MediaCondition::Not(condition))
                if matches!(
                    condition.as_ref(),
                    MediaCondition::Feature(feature)
                        if matches!(
                            feature.as_ref(),
                            QueryFeature::Boolean {
                                name: MediaFeatureName::Standard(MediaFeatureId::Hover)
                            }
                        )
                )
        ));
        assert!(matches!(
            rule.query.media_queries[2].condition.as_ref(),
            Some(MediaCondition::Feature(feature))
                if matches!(feature.as_ref(), QueryFeature::Interval {
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
            rule.query.media_queries[3].condition.as_ref(),
            Some(MediaCondition::Feature(feature))
                if matches!(
                    feature.as_ref(),
                    QueryFeature::Plain {
                        name: MediaFeatureName::Standard(MediaFeatureId::Resolution),
                        value,
                    } if matches!(value, MediaFeatureValue::Resolution(Resolution::Dppx(2.0)))
                )
        ));
        assert!(matches!(
            rule.query.media_queries[4].condition.as_ref(),
            Some(MediaCondition::Feature(feature))
                if matches!(
                    feature.as_ref(),
                    QueryFeature::Range {
                        name: MediaFeatureName::Standard(MediaFeatureId::Width),
                        operator: MediaFeatureComparison::LessThanEqual,
                        value,
                    } if matches!(value, MediaFeatureValue::Env(_))
                )
        ));
    })
}

#[test]
fn invalid_selector_reports_source_location() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let error = expect_parse_error(parse(
            "a, { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                filename: "broken.css",
                ..ParserOptions::default()
            },
        ));

        assert_eq!(error.filename, "broken.css");
        assert_eq!(error.location.line, 0);
        assert_eq!(error.location.column, 4);
        assert!(matches!(
            error.kind,
            rocketcss_parser::ParserError::InvalidSelector
        ));
    })
}

#[test]
fn selector_error_recovery_preserves_a_pure_invalid_selector() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "(font-[family-name:var(--font-*)]) { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let rule = root_rule(&sheet, 0).0;
        let selectors = style_selectors(&sheet, rule);
        assert!(matches!(
            &selectors[0],
            Selector::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(
            property_declarations(&sheet, rule)[0].0,
            Declaration::Color(_)
        ));
    })
}

#[test]
fn selector_error_recovery_continues_at_commas() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        assert_eq!(selectors.len(), 3);
        assert!(matches!(&selectors[0], Selector::Parsed(_)));
        assert!(matches!(
            &selectors[1],
            Selector::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(&selectors[2], Selector::Parsed(_)));
    })
}

#[test]
fn selector_error_recovery_consumes_multiple_invalid_tokens() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            ".valid, .broken ?? trailing, #also-valid { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        assert_eq!(selectors.len(), 3);
        assert!(matches!(&selectors[0], Selector::Parsed(_)));
        assert!(matches!(
            &selectors[1],
            Selector::Unparsed(raw) if raw == ".broken ?? trailing"
        ));
        assert!(matches!(&selectors[2], Selector::Parsed(_)));
    })
}

#[test]
fn invalid_selector_still_fails_without_error_recovery() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let error = expect_parse_error(parse(
            "(font-[family-name:var(--font-*)]) { color: red }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        ));

        assert!(matches!(error.kind, ParserError::InvalidSelector));
    })
}

#[test]
fn parser_reports_unmatched_closing_token() {
    let allocator = Allocator::new();
    let mut parser = Compiler::new_with_source(")", &allocator);
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
            SelectorComponent::LocalName { name, .. }
        ] if name == "e"
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
            SelectorComponent::Namespace { prefix, .. },
            SelectorComponent::LocalName { name, .. }
        ] if prefix == "svg" && name == "circle"
    ));
    assert!(matches!(
        &selectors[3][0],
        SelectorComponent::AttributeOther(attribute)
            if matches!(
                &attribute.namespace,
                Some(NamespaceConstraint::Specific { prefix, .. }) if prefix == "svg"
            )
    ));
    assert!(matches!(
        &selectors[4][..],
        [
            SelectorComponent::Class(left),
            SelectorComponent::Combinator(Combinator::Deep),
            SelectorComponent::Class(right)
        ] if left == "a" && right == "b"
    ));
    assert!(matches!(
        &selectors[5][..],
        [SelectorComponent::LocalName { name, .. }, SelectorComponent::Where(list)]
            if name == "foo" && list.is_empty()
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
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "@keyframes demo { entry 0% { opacity: 0 } entry to { opacity: .5 } exit 100% { opacity: 1 } }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let keyframes = root_rule(&sheet, 0).0;
        let frames = child_rule_ids(&sheet, keyframes);
        assert_eq!(frames.len(), 2);
        let CssRulePayload::Keyframe(first) = sheet.rule(frames[0]).unwrap().payload() else {
            panic!("expected keyframe")
        };
        assert!(matches!(
            &first.selectors[0],
            KeyframeSelector::TimelineRangePercentage(value)
                if value.name == TimelineRangeName::Entry && value.percentage == 0.0
        ));
        let CssRulePayload::Keyframe(second) = sheet.rule(frames[1]).unwrap().payload() else {
            panic!("expected keyframe")
        };
        assert!(matches!(
            &second.selectors[0],
            KeyframeSelector::TimelineRangePercentage(value)
                if value.name == TimelineRangeName::Exit && value.percentage == 1.0
        ));
    })
}

#[test]
fn parses_lightningcss_rule_families() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
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
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let roots = root_rule_ids(&sheet);
        assert_eq!(roots.len(), 10);

        assert!(matches!(
            sheet.rule(roots[0]).unwrap().payload(),
            CssRulePayload::Namespace(rule)
                if rule.prefix == Some("svg") && rule.url == "http://www.w3.org/2000/svg"
        ));
        assert!(matches!(
            sheet.rule(roots[1]).unwrap().payload(),
            CssRulePayload::LayerStatement(rule)
                if rule.names.len() == 2 && rule.names[1].as_slice() == ["theme", "base"]
        ));
        assert!(matches!(
            sheet.rule(roots[2]).unwrap().payload(),
            CssRulePayload::LayerBlock(rule) if rule.name.is_some()
        ));
        assert_eq!(child_rule_ids(&sheet, roots[2]).len(), 1);
        assert!(matches!(
            sheet.rule(roots[3]).unwrap().payload(),
            CssRulePayload::CustomMedia(rule)
                if rule.name == "--narrow" && rule.query.media_queries.len() == 1
        ));
        assert!(matches!(
            sheet.rule(roots[4]).unwrap().payload(),
            CssRulePayload::Keyframes(rule)
                if matches!(*rule.name, rocketcss_ast::KeyframesName::Ident("fade"))
        ));
        let frames = child_rule_ids(&sheet, roots[4]);
        assert_eq!(frames.len(), 3);
        assert!(matches!(
            sheet.rule(frames[1]).unwrap().payload(),
            CssRulePayload::Keyframe(rule)
                if matches!(rule.selectors[0], KeyframeSelector::Percentage(0.5))
        ));
        assert!(matches!(
            sheet.rule(roots[5]).unwrap().payload(),
            CssRulePayload::CounterStyle(_)
        ));
        assert!(matches!(
            sheet.rule(roots[6]).unwrap().payload(),
            CssRulePayload::Viewport(_)
        ));
        assert!(matches!(
            sheet.rule(roots[7]).unwrap().payload(),
            CssRulePayload::PositionTry(rule) if rule.name == "--fallback"
        ));
        assert!(matches!(
            sheet.rule(roots[8]).unwrap().payload(),
            CssRulePayload::Container(rule) if rule.name == Some("card") && rule.condition.is_some()
        ));
        assert!(matches!(
            sheet.rule(roots[9]).unwrap().payload(),
            CssRulePayload::MozDocument(_)
        ));
        assert_eq!(child_rule_ids(&sheet, roots[9]).len(), 1);
    })
}

#[test]
fn parses_import_modifiers_scope_and_page() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = r#"
        @import "theme.css" layer(theme.base) supports(display: grid) print;
        @scope (.card) to (.boundary) { .title { color: red } }
        @page invoice:first { margin: 1cm; @top-center { content: "Invoice"; } }
    "#;
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let roots = root_rule_ids(&sheet);
        assert_eq!(roots.len(), 3);

        let CssRulePayload::Import(import) = sheet.rule(roots[0]).unwrap().payload() else {
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
            sheet.rule(roots[1]).unwrap().payload(),
            CssRulePayload::Scope(rule)
                if rule.scope_start.is_some() && rule.scope_end.is_some()
        ));
        assert_eq!(child_rule_ids(&sheet, roots[1]).len(), 1);
        assert!(matches!(
            sheet.rule(roots[2]).unwrap().payload(),
            CssRulePayload::Page(rule) if rule.selectors.len() == 1
        ));
        assert_eq!(property_declarations(&sheet, roots[2]).len(), 1);
        assert_eq!(child_rule_ids(&sheet, roots[2]).len(), 1);
    })
}

#[test]
fn enforces_import_and_namespace_order_like_lightningcss() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let import_error = expect_parse_error(parse(
            "a {} @import 'late.css';",
            &allocator,
            &mut token,
            ParserOptions::default(),
        ));
        assert!(matches!(
            import_error.kind,
            rocketcss_parser::ParserError::UnexpectedImportRule
        ));

        let namespace_error = expect_parse_error(parse(
            "a {} @namespace svg 'urn:svg';",
            &allocator,
            &mut token,
            ParserOptions::default(),
        ));
        assert!(matches!(
            namespace_error.kind,
            rocketcss_parser::ParserError::UnexpectedNamespaceRule
        ));

        let valid = parse(
            "@charset 'UTF-8'; @layer reset; @import 'theme.css'; @namespace svg 'urn:svg'; a {}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert_eq!(root_rule_ids(&valid).len(), 5);
        assert!(matches!(
            root_rule(&valid, 0).1.payload(),
            CssRulePayload::Charset(rule)
                if rule.encoding == "UTF-8" && rule.span == Span::new(0, 17)
        ));

        let interrupted_import = expect_parse_error(parse(
            "@import \"a.css\";\n@layer reset,base;\n@import \"b.css\" layer(base);",
            &allocator,
            &mut token,
            ParserOptions {
                filename: "layers.css",
                ..ParserOptions::default()
            },
        ));
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert_eq!(root_rule_ids(&initial_layers).len(), 3);
    })
}

#[test]
fn parses_charset_as_a_typed_rule() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            r#"@charset "UTF-\38 ";"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        assert_eq!(root_rule_ids(&sheet).len(), 1);
        assert!(matches!(
            root_rule(&sheet, 0).1.payload(),
            CssRulePayload::Charset(rule)
                if rule.encoding == "UTF-8" && rule.span == Span::new(0, 20)
        ));

        assert!(
            parse(
                "@charset UTF-8;",
                &allocator,
                &mut token,
                ParserOptions::default(),
            )
            .is_err()
        );
    })
}

#[test]
fn parses_declarations_inside_nested_group_rules() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            ".card { @media (width > 30rem) { color: red; & .title { opacity: .8 } } }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let style = root_rule(&sheet, 0).0;
        let media = child_rule_ids(&sheet, style)[0];
        assert!(matches!(
            sheet.rule(media).unwrap().payload(),
            CssRulePayload::Media(_)
        ));
        let media_children = child_rule_ids(&sheet, media);
        assert!(matches!(
            sheet.rule(media_children[0]).unwrap().payload(),
            CssRulePayload::NestedDeclarations(_)
        ));
        assert_eq!(property_declarations(&sheet, media_children[0]).len(), 1);
        assert!(matches!(
            sheet.rule(media_children[1]).unwrap().payload(),
            CssRulePayload::Style(_)
        ));
    })
}

#[test]
fn distinguishes_nested_pseudo_selectors_from_declarations() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            ".card { color: red; button:hover { color: blue } }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let style = root_rule(&sheet, 0).0;
        assert_eq!(property_declarations(&sheet, style).len(), 1);
        let children = child_rule_ids(&sheet, style);
        assert_eq!(children.len(), 1);
        assert!(matches!(
            sheet.rule(children[0]).unwrap().payload(),
            CssRulePayload::Style(_)
        ));
    })
}

#[test]
fn declaration_error_recovery_continues_at_semicolon() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { broken value; width: 10px; }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0);
        assert_eq!(declarations.len(), 1);
        assert!(matches!(declarations[0].0, Declaration::Width(_)));
    })
}

#[test]
#[ignore]
fn declaration_like_identifier_requires_explicit_error_recovery() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = r#"div {
        width: 100px;
        height: 100px;
        background: #dd6b4d;
        fhbj32brjb3;
    }"#;

        let error = expect_parse_error(parse(
            source,
            &allocator,
            &mut token,
            ParserOptions::default(),
        ));
        assert!(matches!(
            error.kind,
            rocketcss_parser::ParserError::InvalidDeclaration
        ));

        let sheet = parse(
            source,
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let style = root_rule(&sheet, 0).0;
        let declarations = property_declarations(&sheet, style);
        assert_eq!(declarations.len(), 3);
        assert!(sheet.rule(style).unwrap().child_list().is_none());
        assert!(matches!(declarations[0].0, Declaration::Width(_)));
        assert!(matches!(declarations[1].0, Declaration::Height(_)));
        assert!(matches!(
            declarations[2].0,
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Background)
        ));
    })
}

#[test]
fn parses_typed_core_property_values() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
    let sheet = parse(
        "a { color: #0f08; background-color: currentColor; display: inline-flex; visibility: hidden; width: 10rem; height: 25%; all: revert-layer; }",
        &allocator,
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0).0);
    assert!(matches!(
        declarations[0],
        Declaration::Color(color)
            if matches!(**color, rocketcss_ast::CssColor::Rgba(rocketcss_ast::RGBA { red: 0, green: 255, blue: 0, alpha: 136 }))
    ));
    assert!(matches!(
        declarations[1],
        Declaration::BackgroundColor(color)
            if matches!(**color, rocketcss_ast::CssColor::CurrentColor)
    ));
    assert!(matches!(declarations[2], Declaration::Display(_)));
    assert!(matches!(
        declarations[3],
        Declaration::Visibility(rocketcss_ast::Visibility::Hidden)
    ));
    assert!(matches!(declarations[4], Declaration::Width(_)));
    assert!(matches!(declarations[5], Declaration::Height(_)));
    assert!(matches!(
        declarations[6],
        Declaration::All(CSSWideKeyword::RevertLayer)
    ));
    })
}

#[test]
fn parses_font_family_into_typed_ast_nodes() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
    let sheet = parse(
        r#"a { font-family: "serif", SANS-SERIF, Fancy Font, "A", "slab inherit"; font-family: var(--family), sans-serif; font-family: slab inherit; }"#,
        &allocator,
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0).0);

    assert!(matches!(
        declarations[0],
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
        declarations[1],
        Declaration::FontFamily(families)
            if matches!(families.as_slice(), [
                FontFamily::Unparsed(_),
                FontFamily::SansSerif,
            ])
    ));
    assert!(matches!(
        declarations[2],
        Declaration::FontFamily(families)
            if matches!(families.as_slice(), [FontFamily::Unparsed(_)])
    ));
    })
}

#[test]
fn parses_known_multicol_and_legacy_gap_ast_nodes() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
    let sheet = parse(
        "a { -webkit-column-rule: red solid 1px; columns: 3 10px; grid-column-gap: 10%; grid-row-gap: normal; columns: var(--count); column-width: INHERIT; columns: REVERT-LAYER; }",
        &allocator,
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0).0);

    assert!(matches!(
        declarations[0],
        Declaration::ColumnRule(value, prefix)
            if prefix.contains(VendorPrefix::WEBKIT)
                && matches!(value.style, Some(LineStyle::Solid))
                && value.width.is_some()
                && value.color.is_some()
    ));
    assert!(matches!(
        declarations[1],
        Declaration::Columns(CSSWideOr::Value(value), prefix)
            if *prefix == VendorPrefix::NONE
                && matches!(value.count, ColumnCount::Integer(3))
                && matches!(&value.width, ColumnWidth::Length(_))
    ));
    assert!(matches!(
        declarations[2],
        Declaration::GridColumnGap(value)
            if matches!(&**value, GapValue::LengthPercentage(_))
    ));
    assert!(matches!(
        declarations[3],
        Declaration::GridRowGap(value) if matches!(&**value, GapValue::Normal)
    ));
    assert!(matches!(
        declarations[4],
        Declaration::Unparsed(value)
            if matches!(&*value.property_id, PropertyId::Columns(VendorPrefix::NONE))
    ));
    assert!(matches!(
        declarations[5],
        Declaration::ColumnWidth(CSSWideOr::CSSWide(CSSWideKeyword::Inherit), prefix)
            if *prefix == VendorPrefix::NONE
    ));
    assert!(matches!(
        declarations[6],
        Declaration::Columns(CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer), prefix)
            if *prefix == VendorPrefix::NONE
    ));
    })
}

#[test]
fn declaration_parsing_uses_property_ids_and_preserves_fallbacks() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0);

        assert_eq!(declarations.len(), 7);
        assert!(matches!(declarations[0].0, Declaration::Color(_)));
        assert!(declarations[0].1);

        assert!(matches!(
            declarations[1].0,
            Declaration::Width(value)
                if matches!(&**value, Size::MathFunction(function)
                    if function.name().eq_ignore_ascii_case("calc"))
        ));
        assert!(declarations[1].1);
        assert!(matches!(
            declarations[2].0,
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Transform(prefix)
                    if prefix.contains(VendorPrefix::WEBKIT))
        ));
        assert!(matches!(
            declarations[3].0,
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Custom("future-property"))
        ));
        assert!(matches!(
            declarations[4].0,
            Declaration::Custom(value)
                if matches!(&*value.name, CustomPropertyName::Custom("--theme"))
                    && value.value.iter().any(|token| matches!(token,
                        TokenOrValue::Function(function) if function.name() == "fn"))
        ));
        assert!(declarations[4].1);
        assert!(matches!(
            declarations[5].0,
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Opacity)
        ));
        assert!(!declarations[5].1);
        assert!(matches!(declarations[6].0, Declaration::Height(_)));
    })
}

#[test]
fn declaration_ast_distinguishes_typed_opaque_invalid_and_unsupported_values() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            r#"a {
                width: initial;
                max-width: fit-content(10px);
                border-top-style: solid;
                animation-duration: 1s, 200ms;
                opacity: calc(.5);
                width: potato;
                transform: translateX(1px);
                future-property: fn(1);
            }"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0).0);

        assert!(matches!(
            declarations[0],
            Declaration::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(**property_id, PropertyId::Width)
        ));
        assert!(matches!(
            declarations[1],
            Declaration::MaxWidth(value)
                if matches!(**value, MaxSize::FitContentFunction(_))
        ));
        assert!(matches!(
            declarations[2],
            Declaration::BorderTopStyle(LineStyle::Solid)
        ));
        assert!(matches!(
            declarations[3],
            Declaration::AnimationDuration(values, prefix)
                if values.len() == 2 && *prefix == VendorPrefix::NONE
        ));
        assert!(matches!(
            declarations[4],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            declarations[5],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            declarations[6],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnsupportedGrammar
        ));
        assert!(matches!(
            declarations[7],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnknownProperty
        ));
    })
}

#[test]
fn css_wide_probe_preserves_typed_and_lossless_declaration_paths() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            r#"a {
                width: 1px;
                height: InHeRiT !important;
                max-width: unset extra;
                opacity: /**/ revert;
                --theme: initial;
                future-property: revert-layer;
                column-width: revert-layer;
                columns: initial/**/;
                all: UNSET;
            }"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0).0);

        assert!(matches!(declarations[0].0, Declaration::Width(_)));
        assert!(matches!(
            declarations[1].0,
            Declaration::CSSWide(property_id, CSSWideKeyword::Inherit)
                if matches!(**property_id, PropertyId::Height)
        ));
        assert!(declarations[1].1);
        assert!(matches!(
            declarations[2].0,
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            declarations[3].0,
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(declarations[4].0, Declaration::Custom(_)));
        assert!(matches!(
            declarations[5].0,
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnknownProperty
        ));
        assert!(matches!(
            declarations[6].0,
            Declaration::ColumnWidth(
                CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer),
                VendorPrefix::NONE
            )
        ));
        assert!(matches!(
            declarations[7].0,
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            declarations[8].0,
            Declaration::All(CSSWideKeyword::Unset)
        ));
    })
}

#[test]
fn css_wide_prescan_handles_escapes_and_an_omitted_final_semicolon() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            r#"a {
                color: \69nitial;
                min-width: revert-layer
            }"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0).0);

        assert!(matches!(
            declarations[0],
            Declaration::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(**property_id, PropertyId::Color)
        ));
        assert!(matches!(
            declarations[1],
            Declaration::CSSWide(property_id, CSSWideKeyword::RevertLayer)
                if matches!(**property_id, PropertyId::MinWidth)
        ));
    })
}

#[test]
#[ignore = "the overlay property does not have typed metadata yet"]
fn recognizes_overlay_as_a_known_property() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a{overlay:auto;overlay:var(--state)}",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert!(
            declaration_values(&sheet, root_rule(&sheet, 0).0)
                .iter()
                .all(|declaration| {
                    !matches!(
                        *declaration,
                        Declaration::Unparsed(value)
                            if matches!(&*value.property_id, PropertyId::Custom("overlay"))
                    )
                })
        );
    })
}

#[test]
fn parses_property_view_transition_palette_and_nest_rules() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
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
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let roots = root_rule_ids(&sheet);
        assert_eq!(roots.len(), 5);
        let CssRulePayload::Property(property) = sheet.rule(roots[0]).unwrap().payload() else {
            panic!("expected property rule")
        };
        assert_eq!(property.name, "--brand-color");
        assert!(property.initial_value.is_some());
        assert!(matches!(
            sheet.declaration(property.syntax.unwrap()).unwrap().payload(),
            DeclarationPayload::PropertyRule(
                rocketcss_ast::radix_ast::PropertyRuleDescriptor::Syntax(syntax)
            ) if matches!(&**syntax, SyntaxString::Components(_))
        ));
        assert!(matches!(
            sheet
                .declaration(property.inherits.unwrap())
                .unwrap()
                .payload(),
            DeclarationPayload::PropertyRule(
                rocketcss_ast::radix_ast::PropertyRuleDescriptor::Inherits(false)
            )
        ));
        assert_eq!(
            sheet
                .declarations_in_block(sheet.rule(roots[1]).unwrap().declaration_block().unwrap())
                .unwrap()
                .count(),
            2
        );
        assert!(matches!(
            sheet.rule(roots[2]).unwrap().payload(),
            CssRulePayload::FontPaletteValues(rule) if rule.name == "--dark"
        ));
        assert_eq!(
            sheet
                .declarations_in_block(sheet.rule(roots[2]).unwrap().declaration_block().unwrap())
                .unwrap()
                .count(),
            2
        );
        let CssRulePayload::FontFeatureValues(features) = sheet.rule(roots[3]).unwrap().payload()
        else {
            panic!("expected font-feature-values")
        };
        assert!(matches!(
            features.name.as_slice(),
            [FamilyName("Demo Sans")]
        ));
        let feature_rule = child_rule_ids(&sheet, roots[3])[0];
        let feature_block = sheet
            .rule(feature_rule)
            .unwrap()
            .declaration_block()
            .unwrap();
        assert!(matches!(
            sheet
                .declarations_in_block(feature_block)
                .unwrap()
                .next()
                .unwrap()
                .payload(),
            DeclarationPayload::FontFeature(declaration)
                if declaration.values.as_slice() == [1, 2]
        ));
        assert!(matches!(
            sheet
                .rule(child_rule_ids(&sheet, roots[4])[0])
                .unwrap()
                .payload(),
            CssRulePayload::Nesting(_)
        ));
    })
}

#[test]
#[ignore]
fn rejects_property_rules_nested_in_style_rules() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
    let error = expect_parse_error(parse(
        r#".example{@property --angle{syntax:"<angle>";inherits:true;initial-value:0turn}animation:spin 3s linear infinite}"#,
        &allocator,
        &mut token,
        ParserOptions::default(),
    ));

    assert!(matches!(
        error.kind,
        rocketcss_parser::ParserError::InvalidAtRule("property")
    ));
    })
}

#[test]
#[ignore]
fn parses_property_initial_value_edge_cases_losslessly() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = r#"
        @property --omitted { syntax: "*"; inherits: false; }
        @property --empty { syntax: "*"; inherits: false; initial-value:; }
        @property --ordered {
          initial-value: 25px;
          inherits: true;
          syntax: "<length>";
        }
    "#;
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(root_rule_ids(&sheet).len(), 3);

        let CssRulePayload::Property(omitted) = root_rule(&sheet, 0).1.payload() else {
            panic!("expected omitted property registration")
        };
        assert!(omitted.initial_value.is_none());

        let CssRulePayload::Property(empty) = root_rule(&sheet, 1).1.payload() else {
            panic!("expected empty property registration")
        };
        assert!(empty.initial_value.is_some());

        let CssRulePayload::Property(ordered) = root_rule(&sheet, 2).1.payload() else {
            panic!("expected ordered property registration")
        };
        assert!(matches!(
            sheet
                .declaration(ordered.inherits.unwrap())
                .unwrap()
                .payload(),
            DeclarationPayload::PropertyRule(
                rocketcss_ast::radix_ast::PropertyRuleDescriptor::Inherits(true)
            )
        ));
        assert!(ordered.initial_value.is_some());
    })
}

#[test]
fn extracts_source_directives_in_parser_layer() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source =
            "a { color: red } /*# sourceURL=original.scss */ /*# sourceMappingURL=style.css.map */";
        let mut compiler = Compiler::new(&allocator);
        let _sheet = compiler
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        assert_eq!(compiler.source_map_url(), Some("style.css.map"));

        let mut parser = Compiler::new_with_source(source, &allocator);
        while parser.next_including_whitespace_and_comments().is_ok() {}
        assert_eq!(parser.current_source_url(), Some("original.scss"));
        assert_eq!(parser.current_source_map_url(), Some("style.css.map"));
    })
}

#[test]
#[ignore]
fn preserves_picker_pseudo_element_and_allows_chaining_pseudo_class() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = "select::picker(select):not(:popover-open) { color: red }";
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(root_rule_ids(&sheet).len(), 1);
        let rule = root_rule(&sheet, 0).0;
        let selectors = style_selectors(&sheet, rule);
        assert_eq!(selectors.len(), 1);

        let selector = &selectors[0];
        assert_eq!(selector.len(), 3);

        assert!(matches!(
            &selector[0],
            SelectorComponent::LocalName { name, .. } if name == "select"
        ));

        assert!(matches!(
            &selector[1],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::CustomFunction { name, .. } if name == "picker"
                )
        ));

        assert!(matches!(&selector[2], SelectorComponent::Negation(_)));

        assert_eq!(property_declarations(&sheet, rule).len(), 1);
    })
}

#[test]
#[ignore]
fn preserves_details_content_chained_with_before_pseudo_element() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = "::details-content::before { background-color: red }";
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(root_rule_ids(&sheet).len(), 1);
        let rule = root_rule(&sheet, 0).0;
        let selectors = style_selectors(&sheet, rule);
        assert_eq!(selectors.len(), 1);

        let selector = &selectors[0];
        assert_eq!(selector.len(), 2);

        assert!(matches!(
            &selector[0],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::Custom { name } if name == "details-content"
                )
        ));

        assert!(matches!(
            &selector[1],
            SelectorComponent::PseudoElement(element)
                if matches!(**element, PseudoElement::Before)
        ));

        let Declaration::BackgroundColor(_) = property_declarations(&sheet, rule)[0].0 else {
            panic!("expected background-color declaration")
        };
    })
}

#[test]
#[ignore]
fn preserves_has_slotted_pseudo_class() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = "slot:has-slotted { display: none }";
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        assert_eq!(selectors.len(), 1);

        let selector = &selectors[0];
        assert_eq!(selector.len(), 2);

        assert!(matches!(
            &selector[0],
            SelectorComponent::LocalName { name, .. } if name == "slot"
        ));

        assert!(matches!(
            &selector[1],
            SelectorComponent::PseudoClass(pc)
                if matches!(
                    &**pc,
                    PseudoClass::Custom { name } if name == "has-slotted"
                )
        ));
    })
}

#[test]
#[ignore]
fn preserves_pseudo_element_arg_inside_has_selector() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = "video:not(:has(::backdrop)) { color: red }";
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        assert_eq!(selectors.len(), 1);

        let selector = &selectors[0];
        assert_eq!(selector.len(), 2);

        assert!(matches!(
            &selector[0],
            SelectorComponent::LocalName { name, .. } if name == "video"
        ));

        assert!(matches!(&selector[1], SelectorComponent::Negation(_)));
    })
}

#[test]
#[ignore]
fn preserves_scroll_button_and_scroll_marker_pseudo_elements() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let source = "::scroll-button { color: red } .carousel > *::scroll-marker { content: '' }";
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(root_rule_ids(&sheet).len(), 2);

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0).0);
        assert!(matches!(
            &selectors[0][0],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::Custom { name } if name == "scroll-button"
                )
        ));

        let selectors = style_selectors(&sheet, root_rule(&sheet, 1).0);
        assert!(matches!(
            &selectors[0][3],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::Custom { name } if name == "scroll-marker"
                )
        ));
    })
}

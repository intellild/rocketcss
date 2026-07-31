use rocketcss_parser::prelude::*;

fn declaration_source_order(source: &str) -> std::vec::Vec<String> {
    GhostToken::scope(|mut token| {
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        sheet.validate_flat_ir().unwrap();
        sheet
            .declaration_slots()
            .enumerate()
            .map(|(index, (id, declaration))| {
                assert_eq!(id.index(), index);
                declaration.name().to_owned()
            })
            .collect()
    })
}

#[test]
fn declaration_ids_follow_lexical_source_order_across_nested_rules() {
    assert_eq!(
        declaration_source_order(
            "a{--p0:0;--p1:1}b{--p2:2}\
             c{--p3:3;& d{--p4:4;--p5:5}--p6:6;\
             @media (width > 1px){--p7:7}--p8:8}",
        ),
        (0..=8)
            .map(|index| format!("--p{index}"))
            .collect::<std::vec::Vec<_>>()
    );
}

#[test]
fn declaration_ids_follow_source_order_in_non_style_blocks() {
    assert_eq!(
        declaration_source_order(
            "@counter-style x{--p0:0}\
             @keyframes x{from{--p1:1}to{--p2:2}}\
             @page{--p3:3;@top-left{--p4:4}--p5:5}\
             @supports(display:grid){a{--p6:6}}",
        ),
        (0..=6)
            .map(|index| format!("--p{index}"))
            .collect::<std::vec::Vec<_>>()
    );
}

#[test]
fn selector_occurrence_ids_follow_lexical_source_order() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            ".first,.second{} .first{} .third{}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let names = sheet
            .selector_slots()
            .enumerate()
            .map(|(index, (id, selector))| {
                assert_eq!(id.index(), index);
                let Selector::Parsed(components) = selector else {
                    panic!("expected parsed selector")
                };
                let [SelectorComponent::Class(name)] = components.as_slice() else {
                    panic!("expected one class component")
                };
                name.as_str()
            })
            .collect::<std::vec::Vec<_>>();
        assert_eq!(names, ["first", "second", "first", "third"]);

        let rules = sheet.root_rules().iter().collect::<std::vec::Vec<_>>();
        let CssRule::Style(first) = rules[0] else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.selector_range(first.selectors).offset(), 0);
        assert_eq!(sheet.selector_range(first.selectors).len(), 2);
    });
}

#[test]
fn nested_prelude_parsers_share_the_compilation_string_pool() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "@layer utilities{}@layer utilities{}@container card (width>1px){}\
             @container card (width>2px){}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let rules = sheet.root_rules().iter().collect::<std::vec::Vec<_>>();
        let (CssRule::LayerBlock(first), CssRule::LayerBlock(second)) = (rules[0], rules[1]) else {
            panic!("expected layer blocks")
        };
        assert_eq!(
            first.name.as_ref().unwrap()[0],
            second.name.as_ref().unwrap()[0]
        );

        let (CssRule::Container(first), CssRule::Container(second)) = (rules[2], rules[3]) else {
            panic!("expected container rules")
        };
        assert_eq!(first.name, second.name);
    });
}

#[test]
fn empty_leading_style_run_keeps_its_pre_child_cursor() {
    GhostToken::scope(|mut token| {
        let sheet = parse("a{& b{--p0:0}--p1:1}", &mut token, ParserOptions::default()).unwrap();
        let CssRule::Style(parent) = &sheet.root_rules()[0] else {
            panic!("expected parent style rule")
        };
        let leading = sheet.declaration_block(parent.declarations);
        assert!(leading.is_empty());
        assert_eq!(leading.ranges()[0].offset(), 0);
        sheet.validate_flat_ir().unwrap();

        let CssRule::Style(child) = &sheet.rule_list(parent.rules)[0] else {
            panic!("expected nested style rule")
        };
        assert_eq!(
            sheet.declaration_block(child.declarations).declarations[0].name(),
            "--p0"
        );
        let CssRule::NestedDeclarations(trailing) = &sheet.rule_list(parent.rules)[1] else {
            panic!("expected trailing nested declarations")
        };
        assert_eq!(
            sheet.declaration_block(trailing.declarations).declarations[0].name(),
            "--p1"
        );
    });
}

#[test]
fn flat_rule_topology_uses_preorder_ids_and_direct_sibling_links() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a{b{x:1}c{x:2}}d{x:3}@unknown foo{bar}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        sheet.validate_flat_ir().unwrap();

        let ids = sheet.rule_store().ids().collect::<std::vec::Vec<_>>();
        assert_eq!(
            ids.iter()
                .map(|id| id.index())
                .collect::<std::vec::Vec<_>>(),
            [0, 1, 2, 3, 4]
        );
        for id in &ids {
            assert_eq!(sheet.rule_store().payload_id(*id).index(), id.index());
        }

        let [parent, first_child, second_child, sibling, unknown] = ids.as_slice() else {
            panic!("expected five flat rules")
        };
        assert_eq!(
            sheet.rule_topology(*parent),
            RuleTopology {
                parent: None,
                list: sheet.rules,
                next_sibling: Some(*sibling),
                subtree_end: 3,
            }
        );
        let CssRule::Style(parent_rule) = sheet.rule(*parent) else {
            panic!("expected parent style rule")
        };
        assert_eq!(
            sheet.rule_topology(*first_child),
            RuleTopology {
                parent: Some(*parent),
                list: parent_rule.rules,
                next_sibling: Some(*second_child),
                subtree_end: 2,
            }
        );
        assert_eq!(
            sheet.rule_topology(*second_child),
            RuleTopology {
                parent: Some(*parent),
                list: parent_rule.rules,
                next_sibling: None,
                subtree_end: 3,
            }
        );
        assert_eq!(sheet.rule_topology(*sibling).next_sibling, Some(*unknown));
        assert_eq!(sheet.rule_topology(*sibling).subtree_end, 4);
        assert_eq!(sheet.rule_topology(*unknown).subtree_end, 5);
    });
}

#[test]
fn flat_rule_topology_handles_empty_lists_and_recovery() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "@media print{}a{}@broken ;b{}",
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        sheet.validate_flat_ir().unwrap();

        let root = sheet.rules(sheet.rules).collect::<std::vec::Vec<_>>();
        assert_eq!(root.len(), 4);
        let CssRule::Media(media) = root[0].1 else {
            panic!("expected media rule")
        };
        assert!(sheet.rule_list(media.rules).is_empty());
        for (expected, (id, _)) in root.iter().enumerate() {
            assert_eq!(id.index(), expected);
            assert_eq!(sheet.rule_topology(*id).subtree_end, expected as u32 + 1);
        }
    });
}

#[test]
fn parser_decodes_values_from_token_spans() {
    let mut parser =
        Compiler::new_with_source(r#"\66 oo "b\61 r" -1.5e2PX 2furlong 25% url(icon\2e svg)"#);

    assert_eq!(parser.expect_ident().as_deref(), Ok("foo"));
    assert_eq!(parser.expect_string().as_deref(), Ok("bar"));
    assert!(matches!(
        parser.next(),
        Ok(ValueToken::Dimension {
            unit: Unit::Length(LengthUnit::Px),
            value,
        }) if *value == -150.0
    ));
    assert!(matches!(
        parser.next(),
        Ok(ValueToken::UnknownDimension { unit, value }) if unit == "furlong" && *value == 2.0
    ));
    assert_eq!(parser.expect_percentage(), Ok(0.25));
    assert_eq!(parser.expect_url().as_deref(), Ok("icon.svg"));
    assert!(parser.is_exhausted());
}

#[test]
fn parser_backtracks_and_parses_nested_blocks() {
    let mut parser = Compiler::new_with_source("foo(1, [bar]) tail");

    let state = parser.state();
    assert_eq!(parser.expect_function().as_deref(), Ok("foo"));
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
    assert_eq!(values.0, 1.0);
    assert_eq!(values.1, "bar");
    assert_eq!(parser.expect_ident().as_deref(), Ok("tail"));

    parser.reset(&state);
    assert_eq!(parser.expect_function().as_deref(), Ok("foo"));
}

#[test]
fn delimited_parse_does_not_stop_inside_nested_blocks() {
    let mut parser = Compiler::new_with_source("one(foo;bar);two");
    let raw = parser
        .parse_until_before(rocketcss_parser::Delimiter::Semicolon, |input| {
            let start = input.position();
            while input.next().is_ok() {}
            Ok::<_, rocketcss_parser::ParseError<'_, ()>>(input.slice_from(start))
        })
        .unwrap();

    assert_eq!(raw, "one(foo;bar)");
    parser.expect_semicolon().unwrap();
    assert_eq!(parser.expect_ident().as_deref(), Ok("two"));
}

#[test]
fn parses_style_rule_selectors_and_declarations() {
    GhostToken::scope(|mut token| {
        let source = "/*! license */ .Foo, #app > a:hover { color: red; opacity: .5 !important; --gap: 1rem; }";
        let mut compiler = Compiler::new();
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

        assert_eq!(&*sheet.license_comments, ["! license "]);
        assert_eq!(compiler.source(), "input.css");
        assert_eq!(sheet.root_rules().len(), 1);
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert_eq!(rule.span, Span::new(15, source.len() as u32));
        assert_eq!(sheet.selectors(rule.selectors).len(), 2);
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0][0],
            SelectorComponent::Class(name) if *name == "Foo"
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[1][1],
            SelectorComponent::Combinator(Combinator::Child)
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[1][3],
            SelectorComponent::PseudoClass(value) if matches!(**value, PseudoClass::Hover)
        ));

        assert_eq!(
            sheet
                .declaration_block(rule.declarations)
                .declarations
                .len(),
            3
        );
        assert_eq!(
            sheet
                .declaration_block(rule.declarations)
                .declarations_importance
                .len(),
            3
        );
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[0],
            Declaration::Color(value)
                if matches!(**value, rocketcss_ast::CssColor::Known(KnownColor::Red))
        ));
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[1],
            Declaration::Opacity(0.5)
        ));
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[2],
            Declaration::Custom(value)
                if matches!(&*value.name, CustomPropertyName::Custom(name) if name == "--gap")
        ));
        assert!(!sheet.declaration_block(rule.declarations).is_important(0));
        assert!(sheet.declaration_block(rule.declarations).is_important(1));
        assert!(!sheet.declaration_block(rule.declarations).is_important(2));
    })
}

#[test]
fn rgb_functions_are_reified_only_after_strict_validation() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a{--valid:rgb(0 0 0);--invalid:rgb(foo);\
             --bad-commas:rgb(0,,0,0);--bad-slashes:rgb(0/0/0);--raw:10.px}\
             b{color:rgb(0,,0,0);color:rgb(0/0/0)}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = &sheet.declaration_block(rule.declarations).declarations;

        assert!(matches!(
            &declarations[0],
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
            &declarations[4],
            Declaration::Custom(value)
                if value
                    .value
                    .iter()
                    .all(|value| matches!(value, TokenOrValue::Token(_)))
        ));

        let CssRule::Style(rule) = &sheet.root_rules()[1] else {
            panic!("expected style rule")
        };
        for declaration in &sheet.declaration_block(rule.declarations).declarations {
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
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a{--mixed:rgb(255 50% 0);--missing:rgb(none 50% 0/none);\
             --out-of-range:rgba(300 -10 0);--legacy-rgba:rgba(1,2,3);\
             color:rgb(255 50% 0);background-color:rgb(none 50% 0/none)}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = &sheet.declaration_block(rule.declarations).declarations;

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
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a{display:;display:none flow;display:table-cell flow;\
             transform:initial/**/;all:initial/**/;columns:initial/**/;\
             display:inline-block;display:-webkit-inline-box;display:-moz-inline-box}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = &sheet.declaration_block(rule.declarations).declarations;

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
            &declarations[6],
            Declaration::Display(Display::Pair {
                outside: DisplayOutside::Inline,
                inside: DisplayInside::FlowRoot,
                is_list_item: false,
            })
        ));
        for (declaration, prefix) in [
            (&declarations[7], VendorPrefix::WEBKIT),
            (&declarations[8], VendorPrefix::MOZ),
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
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a { color: blue; background-color: lightgreen; background: blue }",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };

        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[0],
            Declaration::Color(value)
                if matches!(**value, CssColor::Known(KnownColor::Blue))
        ));
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[1],
            Declaration::BackgroundColor(value)
                if matches!(**value, CssColor::Known(KnownColor::Lightgreen))
        ));
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[2],
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
    GhostToken::scope(|mut token| {
        let sheet = parse(
            r#".f\6f o { width: calc(100% - var(--gap)); }"#,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0][0],
            SelectorComponent::Class(name) if name == "foo"
        ));

        let Declaration::Width(width) = &sheet.declaration_block(rule.declarations).declarations[0]
        else {
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
    GhostToken::scope(|mut token| {
        let mut compiler = Compiler::new();
        let mut sheet = compiler
            .parse(
                r#".foo,.f\6f o { color: red }"#,
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let SelectorComponent::Class(first) = &sheet.selectors(rule.selectors)[0][0] else {
            panic!("expected first class selector")
        };
        let SelectorComponent::Class(second) = &sheet.selectors(rule.selectors)[1][0] else {
            panic!("expected second class selector")
        };
        let first = first.clone();
        let second = second.clone();

        assert_eq!(first, second);
        assert!(std::ptr::eq(first.as_str(), second.as_str()));
        let interned_after_parse = sheet.intern("foo");
        assert_eq!(first, interned_after_parse);
    })
}

#[test]
fn scope_prelude_reuses_the_compiler_string_pool() {
    GhostToken::scope(|mut token| {
        let mut compiler = Compiler::new();
        let sheet = compiler
            .parse(
                "@scope (.shared){.shared{color:red}}",
                &mut token,
                ParserOptions::default(),
            )
            .unwrap();
        let CssRule::Scope(scope) = &sheet.root_rules()[0] else {
            panic!("expected scope rule")
        };
        let SelectorComponent::Class(scope_class) = &scope.scope_start.as_ref().unwrap()[0][0]
        else {
            panic!("expected scope class selector")
        };
        let CssRule::Style(rule) = &sheet.rule_list(scope.rules)[0] else {
            panic!("expected scoped style rule")
        };
        let SelectorComponent::Class(rule_class) = &sheet.selectors(rule.selectors)[0][0] else {
            panic!("expected style class selector")
        };

        assert_eq!(scope_class, rule_class);
        assert!(std::ptr::eq(scope_class.as_str(), rule_class.as_str()));
    })
}

#[test]
fn parses_import_media_unknown_and_font_face_rules() {
    GhostToken::scope(|mut token| {
        let source = r#"
        @import url("a.css") screen;
        @media only screen and (min-width: 10px) { .a { display: block } }
        @font-face { font-family: "Demo"; src: url(demo.woff2); }
        @unknown foo(1) { bar: baz }
    "#;
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 4);

        let CssRule::Import(rule) = &sheet.root_rules()[0] else {
            panic!("expected import")
        };
        assert_eq!(rule.url, "a.css");
        assert!(matches!(
            rule.media
                .as_ref()
                .map(|media| &media.media_queries[0].media_type),
            Some(MediaType::Screen)
        ));

        let CssRule::Media(rule) = &sheet.root_rules()[1] else {
            panic!("expected media")
        };
        assert_eq!(sheet.rule_list(rule.rules).len(), 1);
        assert!(matches!(
            rule.query.media_queries[0].media_type,
            MediaType::Screen
        ));
        assert!(rule.query.media_queries[0].condition.is_some());

        let CssRule::FontFace(rule) = &sheet.root_rules()[2] else {
            panic!("expected font-face")
        };
        assert_eq!(rule.properties.len(), 2);
        assert!(matches!(
            &rule.properties[0],
            FontFaceProperty::Custom(value)
                if matches!(&*value.name, CustomPropertyName::Unknown(name) if name == "font-family")
        ));

        let CssRule::Unknown(rule) = &sheet.root_rules()[3] else {
            panic!("expected unknown at-rule")
        };
        assert_eq!(rule.name, "unknown");
        assert!(rule.block.is_some());
    })
}

#[test]
fn parses_typed_media_conditions_and_features() {
    GhostToken::scope(|mut token| {
        let source = r#"
        @media (width >= 600px) and (orientation: landscape),
               not (hover),
               (400px < width <= 1000px),
               screen and (resolution: 2dppx),
               (max-width: env(--narrow, 10px)) {}
    "#;
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        let CssRule::Media(rule) = &sheet.root_rules()[0] else {
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
    GhostToken::scope(|mut token| {
        let error = parse(
            "a, { color: red }",
            &mut token,
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
    })
}

#[test]
fn selector_error_recovery_preserves_a_pure_invalid_selector() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "(font-[family-name:var(--font-*)]) { color: red }",
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected recovered style rule")
        };
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0],
            Selector::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(
            &sheet.declaration_block(rule.declarations).declarations[0],
            Declaration::Color(_)
        ));
    })
}

#[test]
fn selector_error_recovery_continues_at_commas() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected recovered style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 3);
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0],
            Selector::Parsed(_)
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[1],
            Selector::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[2],
            Selector::Parsed(_)
        ));
    })
}

#[test]
fn selector_error_recovery_consumes_multiple_invalid_tokens() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            ".valid, .broken ?? trailing, #also-valid { color: red }",
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected recovered style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 3);
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0],
            Selector::Parsed(_)
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[1],
            Selector::Unparsed(raw) if raw == ".broken ?? trailing"
        ));
        assert!(matches!(
            &sheet.selectors(rule.selectors)[2],
            Selector::Parsed(_)
        ));
    })
}

#[test]
fn invalid_selector_still_fails_without_error_recovery() {
    GhostToken::scope(|mut token| {
        let error = parse(
            "(font-[family-name:var(--font-*)]) { color: red }",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap_err();

        assert!(matches!(error.kind, ParserError::InvalidSelector));
    })
}

#[test]
fn parser_reports_unmatched_closing_token() {
    let mut parser = Compiler::new_with_source(")");
    let error = parser.expect_no_error_token().unwrap_err();
    assert!(matches!(
        error.kind,
        BasicParseErrorKind::UnexpectedToken(token)
            if token.span == Span::new(0, 1)
    ));
}

#[test]
fn lightningcss_parse_trait_parses_values_from_strings() {
    let selectors = rocketcss_ast::SelectorList::parse_string(".a:is(.b, #c)").unwrap();
    assert_eq!(selectors.len(), 1);
    assert!(matches!(
        &selectors[0][1],
        SelectorComponent::Is(list) if list.len() == 2
    ));
}

#[test]
fn parses_namespace_deep_and_empty_where_selectors() {
    let selectors = SelectorList::parse_string(
        "|e, *|*, svg|circle, [svg|fill=red], .a /deep/ .b, foo:where()",
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
    let selectors =
        SelectorList::parse_string("::-MoZ-selection,::-webkit-placeholder,::placeholder").unwrap();

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
    GhostToken::scope(|mut token| {
        let sheet = parse(
        "@keyframes demo { entry 0% { opacity: 0 } entry to { opacity: .5 } exit 100% { opacity: 1 } }",
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
        let CssRule::Keyframes(rule) = &sheet.root_rules()[0] else {
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
    })
}

#[test]
fn parses_lightningcss_rule_families() {
    GhostToken::scope(|mut token| {
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
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 10);

        assert!(matches!(
            &sheet.root_rules()[0],
            CssRule::Namespace(rule)
                if rule.prefix.as_ref().is_some_and(|prefix| prefix == "svg")
                    && rule.url == "http://www.w3.org/2000/svg"
        ));
        assert!(matches!(
            &sheet.root_rules()[1],
            CssRule::LayerStatement(rule)
                if rule.names.len() == 2
                    && rule.names[1].iter().map(Atom::as_str).eq(["theme", "base"])
        ));
        assert!(matches!(
            &sheet.root_rules()[2],
            CssRule::LayerBlock(rule)
                if rule.name.is_some() && sheet.rule_list(rule.rules).len() == 1
        ));
        assert!(matches!(
            &sheet.root_rules()[3],
            CssRule::CustomMedia(rule)
                if rule.name == "--narrow" && rule.query.media_queries.len() == 1
        ));
        assert!(matches!(
            &sheet.root_rules()[4],
            CssRule::Keyframes(rule)
                if matches!(&*rule.name, rocketcss_ast::KeyframesName::Ident(name) if name == "fade")
                    && rule.keyframes.len() == 3
                    && matches!(rule.keyframes[1].selectors[0], rocketcss_ast::KeyframeSelector::Percentage(0.5))
        ));
        assert!(matches!(&sheet.root_rules()[5], CssRule::CounterStyle(_)));
        assert!(matches!(&sheet.root_rules()[6], CssRule::Viewport(_)));
        assert!(matches!(
            &sheet.root_rules()[7],
            CssRule::PositionTry(rule) if rule.name == "--fallback"
        ));
        assert!(matches!(
            &sheet.root_rules()[8],
            CssRule::Container(rule)
                if rule.name.as_ref().is_some_and(|name| name == "card") && rule.condition.is_some()
        ));
        assert!(matches!(
            &sheet.root_rules()[9],
            CssRule::MozDocument(rule) if sheet.rule_list(rule.rules).len() == 1
        ));
    })
}

#[test]
fn parses_import_modifiers_scope_and_page() {
    GhostToken::scope(|mut token| {
        let source = r#"
        @import "theme.css" layer(theme.base) supports(display: grid) print;
        @scope (.card) to (.boundary) { .title { color: red } }
        @page invoice:first { margin: 1cm; @top-center { content: "Invoice"; } }
    "#;
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 3);

        let CssRule::Import(import) = &sheet.root_rules()[0] else {
            panic!("expected import")
        };
        assert!(
            import
                .layer
                .as_deref()
                .is_some_and(|layer| layer.iter().map(Atom::as_str).eq(["theme", "base"]))
        );
        assert!(import.supports.is_some());
        assert!(matches!(
            import
                .media
                .as_ref()
                .map(|media| &media.media_queries[0].media_type),
            Some(MediaType::Print)
        ));

        assert!(matches!(
            &sheet.root_rules()[1],
            CssRule::Scope(rule)
                if rule.scope_start.is_some()
                    && rule.scope_end.is_some()
                    && sheet.rule_list(rule.rules).len() == 1
        ));
        assert!(matches!(
            &sheet.root_rules()[2],
            CssRule::Page(rule)
                if rule.selectors.len() == 1
                    && sheet.declaration_block(rule.declarations).declarations.len() == 1
                    && rule.rules.len() == 1
        ));
    })
}

#[test]
fn enforces_import_and_namespace_order_like_lightningcss() {
    GhostToken::scope(|mut token| {
        let import_error = parse(
            "a {} @import 'late.css';",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap_err();
        assert!(matches!(
            import_error.kind,
            rocketcss_parser::ParserError::UnexpectedImportRule
        ));

        let namespace_error = parse(
            "a {} @namespace svg 'urn:svg';",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap_err();
        assert!(matches!(
            namespace_error.kind,
            rocketcss_parser::ParserError::UnexpectedNamespaceRule
        ));

        let valid = parse(
            "@charset 'UTF-8'; @layer reset; @import 'theme.css'; @namespace svg 'urn:svg'; a {}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert_eq!(valid.rule_list(valid.rules).len(), 5);
        assert!(matches!(
            &valid.rule_list(valid.rules)[0],
            CssRule::Charset(rule)
                if rule.encoding == "UTF-8" && rule.span == Span::new(0, 17)
        ));

        let interrupted_import = parse(
            "@import \"a.css\";\n@layer reset,base;\n@import \"b.css\" layer(base);",
            &mut token,
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        assert_eq!(initial_layers.rule_list(initial_layers.rules).len(), 3);
    })
}

#[test]
fn parses_charset_as_a_typed_rule() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            r#"@charset "UTF-\38 ";"#,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();

        assert!(matches!(
            &sheet.root_rules()[0],
            CssRule::Charset(rule)
                if sheet.root_rules().len() == 1
                    && rule.encoding == "UTF-8"
                    && rule.span == Span::new(0, 20)
        ));

        assert!(parse("@charset UTF-8;", &mut token, ParserOptions::default(),).is_err());
    })
}

#[test]
fn parses_declarations_inside_nested_group_rules() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            ".card { @media (width > 30rem) { color: red; & .title { opacity: .8 } } }",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let CssRule::Media(media) = &sheet.rule_list(style.rules)[0] else {
            panic!("expected nested media")
        };
        assert!(matches!(
            &sheet.rule_list(media.rules)[0],
            CssRule::NestedDeclarations(rule)
                if sheet.declaration_block(rule.declarations).declarations.len() == 1
        ));
        assert!(matches!(
            &sheet.rule_list(media.rules)[1],
            CssRule::Style(_)
        ));
    })
}

#[test]
fn distinguishes_nested_pseudo_selectors_from_declarations() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            ".card { color: red; button:hover { color: blue } }",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };
        assert_eq!(
            sheet
                .declaration_block(style.declarations)
                .declarations
                .len(),
            1
        );
        assert_eq!(sheet.rule_list(style.rules).len(), 1);
        assert!(matches!(
            &sheet.rule_list(style.rules)[0],
            CssRule::Style(_)
        ));
    })
}

#[test]
fn declaration_error_recovery_continues_at_semicolon() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a { broken value; width: 10px; }",
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };
        assert_eq!(
            sheet
                .declaration_block(style.declarations)
                .declarations
                .len(),
            1
        );
        assert!(matches!(
            &sheet.declaration_block(style.declarations).declarations[0],
            Declaration::Width(_)
        ));
    })
}

#[test]
#[ignore]
fn declaration_like_identifier_requires_explicit_error_recovery() {
    GhostToken::scope(|mut token| {
        let source = r#"div {
        width: 100px;
        height: 100px;
        background: #dd6b4d;
        fhbj32brjb3;
    }"#;

        let error = parse(source, &mut token, ParserOptions::default()).unwrap_err();
        assert!(matches!(
            error.kind,
            rocketcss_parser::ParserError::InvalidDeclaration
        ));

        let sheet = parse(
            source,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };

        assert_eq!(
            sheet
                .declaration_block(style.declarations)
                .declarations
                .len(),
            3
        );
        assert!(sheet.rule_list(style.rules).is_empty());
        assert!(matches!(
            sheet.declaration_block(style.declarations).declarations[0],
            Declaration::Width(_)
        ));
        assert!(matches!(
            sheet.declaration_block(style.declarations).declarations[1],
            Declaration::Height(_)
        ));
        assert!(matches!(
            &sheet.declaration_block(style.declarations).declarations[2],
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Background)
        ));
    })
}

#[test]
fn parses_typed_core_property_values() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
        "a { color: #0f08; background-color: currentColor; display: inline-flex; visibility: hidden; width: 10rem; height: 25%; all: revert-layer; }",
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };
        let declarations = &sheet.declaration_block(style.declarations).declarations;
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
    })
}

#[test]
fn parses_font_family_into_typed_ast_nodes() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
        r#"a { font-family: "serif", SANS-SERIF, Fancy Font, "A", "slab inherit"; font-family: var(--family), sans-serif; font-family: slab inherit; }"#,
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };
        let declarations = &sheet.declaration_block(style.declarations).declarations;

        assert!(matches!(
            &declarations[0],
            Declaration::FontFamily(families)
                if matches!(families.as_slice(), [
                    FontFamily::Custom(first),
                    FontFamily::SansSerif,
                    FontFamily::Custom(third),
                    FontFamily::Custom(fourth),
                    FontFamily::Custom(fifth),
                ] if first.as_str() == "serif"
                    && third.as_str() == "Fancy Font"
                    && fourth.as_str() == "A"
                    && fifth.as_str() == "slab inherit")
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
    })
}

#[test]
fn parses_known_multicol_and_legacy_gap_ast_nodes() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
        "a { -webkit-column-rule: red solid 1px; columns: 3 10px; grid-column-gap: 10%; grid-row-gap: normal; columns: var(--count); column-width: INHERIT; columns: REVERT-LAYER; }",
        &mut token,
        ParserOptions::default(),
    )
    .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style")
        };
        let declarations = &sheet.declaration_block(style.declarations).declarations;

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
                    && matches!(&value.width, ColumnWidth::Length(_))
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
    })
}

#[test]
fn declaration_parsing_uses_property_ids_and_preserves_fallbacks() {
    GhostToken::scope(|mut token| {
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = &sheet.declaration_block(style.declarations).declarations;

        assert_eq!(declarations.len(), 7);
        assert!(matches!(&declarations[0], Declaration::Color(_)));
        assert!(sheet.declaration_block(style.declarations).is_important(0));

        assert!(matches!(
            &declarations[1],
            Declaration::Width(value)
                if matches!(&**value, Size::MathFunction(function)
                    if function.name().eq_ignore_ascii_case("calc"))
        ));
        assert!(sheet.declaration_block(style.declarations).is_important(1));
        assert!(matches!(
            &declarations[2],
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Transform(prefix)
                    if prefix.contains(VendorPrefix::WEBKIT))
        ));
        assert!(matches!(
            &declarations[3],
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Custom(name) if name == "future-property")
        ));
        assert!(matches!(
            &declarations[4],
            Declaration::Custom(value)
                if matches!(&*value.name, CustomPropertyName::Custom(name) if name == "--theme")
                    && value.value.iter().any(|token| matches!(token,
                        TokenOrValue::Function(function) if function.name() == "fn"))
        ));
        assert!(sheet.declaration_block(style.declarations).is_important(4));
        assert!(matches!(
            &declarations[5],
            Declaration::Unparsed(value)
                if matches!(&*value.property_id, PropertyId::Opacity)
        ));
        assert!(!sheet.declaration_block(style.declarations).is_important(5));
        assert!(matches!(&declarations[6], Declaration::Height(_)));
    })
}

#[test]
fn declaration_ast_distinguishes_typed_opaque_invalid_and_unsupported_values() {
    GhostToken::scope(|mut token| {
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = &sheet.declaration_block(style.declarations).declarations;

        assert!(matches!(
            &declarations[0],
            Declaration::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(**property_id, PropertyId::Width)
        ));
        assert!(matches!(
            &declarations[1],
            Declaration::MaxWidth(value)
                if matches!(**value, MaxSize::FitContentFunction(_))
        ));
        assert!(matches!(
            &declarations[2],
            Declaration::BorderTopStyle(LineStyle::Solid)
        ));
        assert!(matches!(
            &declarations[3],
            Declaration::AnimationDuration(values, prefix)
                if values.len() == 2 && *prefix == VendorPrefix::NONE
        ));
        assert!(matches!(
            &declarations[4],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            &declarations[5],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            &declarations[6],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnsupportedGrammar
        ));
        assert!(matches!(
            &declarations[7],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnknownProperty
        ));
    })
}

#[test]
fn css_wide_probe_preserves_typed_and_lossless_declaration_paths() {
    GhostToken::scope(|mut token| {
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
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declaration_block = sheet.declaration_block(style.declarations);
        let declarations = &declaration_block.declarations;

        assert!(matches!(&declarations[0], Declaration::Width(_)));
        assert!(matches!(
            &declarations[1],
            Declaration::CSSWide(property_id, CSSWideKeyword::Inherit)
                if matches!(**property_id, PropertyId::Height)
        ));
        assert!(declaration_block.is_important(1));
        assert!(matches!(
            &declarations[2],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            &declarations[3],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(&declarations[4], Declaration::Custom(_)));
        assert!(matches!(
            &declarations[5],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::UnknownProperty
        ));
        assert!(matches!(
            &declarations[6],
            Declaration::ColumnWidth(
                CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer),
                VendorPrefix::NONE
            )
        ));
        assert!(matches!(
            &declarations[7],
            Declaration::Unparsed(value)
                if value.reason == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            &declarations[8],
            Declaration::All(CSSWideKeyword::Unset)
        ));
    })
}

#[test]
fn css_wide_prescan_handles_escapes_and_an_omitted_final_semicolon() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            r#"a {
                color: \69nitial;
                min-width: revert-layer
            }"#,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        let declarations = sheet.declaration_block(style.declarations);

        assert!(matches!(
            &declarations.declarations[0],
            Declaration::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(**property_id, PropertyId::Color)
        ));
        assert!(matches!(
            &declarations.declarations[1],
            Declaration::CSSWide(property_id, CSSWideKeyword::RevertLayer)
                if matches!(**property_id, PropertyId::MinWidth)
        ));
    })
}

#[test]
#[ignore = "the overlay property does not have typed metadata yet"]
fn recognizes_overlay_as_a_known_property() {
    GhostToken::scope(|mut token| {
        let sheet = parse(
            "a{overlay:auto;overlay:var(--state)}",
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let CssRule::Style(style) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };

        assert!(
            sheet
                .declaration_block(style.declarations)
                .declarations
                .iter()
                .all(|declaration| {
                    !matches!(
                        declaration,
                        Declaration::Unparsed(value)
                            if matches!(&*value.property_id, PropertyId::Custom(name) if name == "overlay")
                    )
                })
        );
    })
}

#[test]
fn parses_property_view_transition_palette_and_nest_rules() {
    GhostToken::scope(|mut token| {
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
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 5);
        assert!(matches!(
            &sheet.root_rules()[0],
            CssRule::Property(rule)
                if rule.name == "--brand-color"
                    && !rule.inherits
                    && rule.initial_value.is_some()
                    && matches!(*rule.syntax, rocketcss_ast::SyntaxString::Components(_))
        ));
        assert!(matches!(
            &sheet.root_rules()[1],
            CssRule::ViewTransition(rule) if rule.properties.len() == 2
        ));
        assert!(matches!(
            &sheet.root_rules()[2],
            CssRule::FontPaletteValues(rule)
                if rule.name == "--dark" && rule.properties.len() == 2
        ));
        assert!(matches!(
            &sheet.root_rules()[3],
            CssRule::FontFeatureValues(rule)
                if rule.name.len() == 1
                    && rule.name[0].0 == "Demo Sans"
                    && rule.rules.len() == 1
                    && rule.rules[0].declarations[0].values.as_slice() == [1, 2]
        ));
        let CssRule::Style(style) = &sheet.root_rules()[4] else {
            panic!("expected style")
        };
        assert!(matches!(
            &sheet.rule_list(style.rules)[0],
            CssRule::Nesting(_)
        ));
    })
}

#[test]
#[ignore]
fn rejects_property_rules_nested_in_style_rules() {
    GhostToken::scope(|mut token| {
        let error = parse(
        r#".example{@property --angle{syntax:"<angle>";inherits:true;initial-value:0turn}animation:spin 3s linear infinite}"#,
        &mut token,
        ParserOptions::default(),
    )
    .unwrap_err();

        assert!(matches!(
            error.kind,
            rocketcss_parser::ParserError::InvalidAtRule(ref name) if name == "property"
        ));
    })
}

#[test]
#[ignore]
fn parses_property_initial_value_edge_cases_losslessly() {
    GhostToken::scope(|mut token| {
        let source = r#"
        @property --omitted { syntax: "*"; inherits: false; }
        @property --empty { syntax: "*"; inherits: false; initial-value:; }
        @property --ordered {
          initial-value: 25px;
          inherits: true;
          syntax: "<length>";
        }
    "#;
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 3);

        let CssRule::Property(omitted) = &sheet.root_rules()[0] else {
            panic!("expected omitted property registration")
        };
        assert!(omitted.initial_value.is_none());

        let CssRule::Property(empty) = &sheet.root_rules()[1] else {
            panic!("expected empty property registration")
        };
        assert!(empty.initial_value.is_some());

        let CssRule::Property(ordered) = &sheet.root_rules()[2] else {
            panic!("expected ordered property registration")
        };
        assert!(ordered.inherits);
        assert!(ordered.initial_value.is_some());
    })
}

#[test]
fn extracts_source_directives_in_parser_layer() {
    GhostToken::scope(|mut token| {
        let source =
            "a { color: red } /*# sourceURL=original.scss */ /*# sourceMappingURL=style.css.map */";
        let mut compiler = Compiler::new();
        let _sheet = compiler
            .parse(source, &mut token, ParserOptions::default())
            .unwrap();
        assert_eq!(compiler.source_map_url(), Some("style.css.map"));

        let mut parser = Compiler::new_with_source(source);
        while parser.next_including_whitespace_and_comments().is_ok() {}
        assert_eq!(parser.current_source_url(), Some("original.scss"));
        assert_eq!(parser.current_source_map_url(), Some("style.css.map"));
    })
}

#[test]
#[ignore]
fn preserves_picker_pseudo_element_and_allows_chaining_pseudo_class() {
    GhostToken::scope(|mut token| {
        let source = "select::picker(select):not(:popover-open) { color: red }";
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 1);
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 1);

        let selector = &sheet.selectors(rule.selectors)[0];
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

        assert_eq!(
            sheet
                .declaration_block(rule.declarations)
                .declarations
                .len(),
            1
        );
    })
}

#[test]
#[ignore]
fn preserves_details_content_chained_with_before_pseudo_element() {
    GhostToken::scope(|mut token| {
        let source = "::details-content::before { background-color: red }";
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 1);
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 1);

        let selector = &sheet.selectors(rule.selectors)[0];
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

        let Declaration::BackgroundColor(_) =
            &sheet.declaration_block(rule.declarations).declarations[0]
        else {
            panic!("expected background-color declaration")
        };
    })
}

#[test]
#[ignore]
fn preserves_has_slotted_pseudo_class() {
    GhostToken::scope(|mut token| {
        let source = "slot:has-slotted { display: none }";
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 1);

        let selector = &sheet.selectors(rule.selectors)[0];
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
    GhostToken::scope(|mut token| {
        let source = "video:not(:has(::backdrop)) { color: red }";
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.selectors(rule.selectors).len(), 1);

        let selector = &sheet.selectors(rule.selectors)[0];
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
    GhostToken::scope(|mut token| {
        let source = "::scroll-button { color: red } .carousel > *::scroll-marker { content: '' }";
        let sheet = parse(source, &mut token, ParserOptions::default()).unwrap();
        assert_eq!(sheet.root_rules().len(), 2);

        let CssRule::Style(rule) = &sheet.root_rules()[0] else {
            panic!("expected scroll-button style rule")
        };
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0][0],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::Custom { name } if name == "scroll-button"
                )
        ));

        let CssRule::Style(rule) = &sheet.root_rules()[1] else {
            panic!("expected scroll-marker style rule")
        };
        assert!(matches!(
            &sheet.selectors(rule.selectors)[0][3],
            SelectorComponent::PseudoElement(element)
                if matches!(
                    &**element,
                    PseudoElement::Custom { name } if name == "scroll-marker"
                )
        ));
    })
}

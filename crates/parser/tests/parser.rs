use rocketcss_ast_next::{
    AbsoluteFontWeight, AlignItems, AlignSelf, AspectRatioData, BackgroundRepeatKeyword,
    BackgroundSizeData, BoxSizing, CSSWideKeyword, CSSWideOr, ColumnCount, ColumnWidthData,
    Combinator, Compilation, ContentDistribution, CssColorData, CssRule, CssRulePayloadData,
    CustomPropertyName, Declaration, DeclarationData, DeclarationPayloadData, DisplayData,
    DisplayInsideData, DisplayOutside, FillRule, FlexDirection, FlexWrap, FontFacePropertyData,
    FontFamilyData, FontWeight, FunctionFlags, GapValueData, GeometryBox, GradientData,
    GradientItemData, ImageData, JustifyContent, KeyframeSelectorData, KeyframesName, KnownColor,
    KnownFunction, LengthData, LengthUnit, LineStyle, MaskClip, MaskComposite, MaskMode, MaskType,
    MaxSizeData, MediaConditionData, MediaFeatureComparison, MediaFeatureId, MediaFeatureName,
    MediaFeatureValueData, MediaType, NamespaceConstraint, ObjectFit, Operator, OverflowData,
    OverflowKeyword, PropertyId, PseudoClassData, PseudoElementData, QueryFeatureData, RGBAData,
    RatioData, Resize, Resolution, ScrollbarColorData, Selector, SelectorComponentData,
    SelectorData, SelfPosition, Span, StrokeLinecap, SyntaxStringData, TextAlign,
    TextDecorationStyle, TextDirection, TextOverflow, TextSizeAdjust, TimelineRangeName,
    TokenOrValueData, TransformStyle, UnicodeBidi, UnparsedPropertyReason, UserSelect,
    VendorPrefix, Visibility, WhiteSpace, ZIndex,
};
use rocketcss_parser::parse;
use rocketcss_parser::prelude::*;

fn root_rule_ids<'ast>(compilation: &Compilation<'ast>) -> std::vec::Vec<CssRule<'ast>> {
    compilation.style_sheet().rules(compilation.ast()).to_vec()
}

fn root_rule<'ast>(compilation: &Compilation<'ast>, index: usize) -> CssRule<'ast> {
    root_rule_ids(compilation)[index]
}

fn child_rule_ids<'ast>(
    compilation: &Compilation<'ast>,
    parent: CssRule<'ast>,
) -> std::vec::Vec<CssRule<'ast>> {
    parent.children(compilation.ast()).to_vec()
}

fn style_selectors<'ast>(
    compilation: &Compilation<'ast>,
    rule: CssRule<'ast>,
) -> &'ast [Selector<'ast>] {
    let ast = compilation.ast();
    match rule.payload(ast).data(ast) {
        CssRulePayloadData::Style(payload) => payload.selectors(ast),
        CssRulePayloadData::Nesting(payload) => payload.selectors(ast),
        _ => panic!("expected selector-owning rule"),
    }
}

fn property_declarations<'ast>(
    compilation: &Compilation<'ast>,
    rule: CssRule<'ast>,
) -> std::vec::Vec<(Declaration<'ast>, bool)> {
    let ast = compilation.ast();
    let block = rule
        .declarations(ast)
        .expect("expected declaration-owning rule");
    block
        .declarations(ast)
        .iter()
        .zip(block.important(ast))
        .filter_map(|(payload, important)| {
            payload
                .as_property(ast)
                .map(|declaration| (declaration, *important))
        })
        .collect()
}

fn declaration_values<'ast>(
    compilation: &Compilation<'ast>,
    rule: CssRule<'ast>,
) -> std::vec::Vec<Declaration<'ast>> {
    property_declarations(compilation, rule)
        .into_iter()
        .map(|(declaration, _)| declaration)
        .collect()
}

fn parse_selectors<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> (Compiler<'i>, &'i [Selector<'i>]) {
    let mut compiler = Compiler::new_with_source(source, allocator);
    let selectors = <&[Selector]>::parse(&mut compiler).unwrap();
    (compiler, selectors)
}

fn component_data_vec<'ast>(
    ast: &rocketcss_ast_next::Ast<'ast>,
    selector: Selector<'ast>,
) -> std::vec::Vec<SelectorComponentData<'ast>> {
    components(ast, selector)
        .iter()
        .map(|component| component.data(ast))
        .collect()
}

fn components<'ast>(
    ast: &rocketcss_ast_next::Ast<'ast>,
    selector: Selector<'ast>,
) -> &'ast [rocketcss_ast_next::SelectorComponent<'ast>] {
    match selector.data(ast) {
        SelectorData::Parsed(components) => components,
        _ => panic!("expected a parsed selector"),
    }
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
            unit: rocketcss_parser::Unit::Length(rocketcss_parser::LengthUnit::Px),
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

        assert_eq!(sheet.style_sheet().license_comments(sheet.ast()), ["! license "]);
        assert_eq!(compiler.source(), "input.css");
        assert_eq!(root_rule_ids(&sheet).len(), 1);
        let rule = root_rule(&sheet, 0);
        let rule_id = rule;
        let CssRulePayloadData::Style(payload) = rule.payload(sheet.ast()).data(sheet.ast()) else {
            panic!("expected style rule")
        };
        assert_eq!(sheet.ast().span(payload), Span::new(15, source.len() as u32));
        let selectors = style_selectors(&sheet, rule_id);
        assert_eq!(selectors.len(), 2);
        assert!(matches!(
            components(sheet.ast(), selectors[0])[0].data(sheet.ast()),
            SelectorComponentData::Class(name) if name == "Foo"
        ));
        assert!(matches!(
            components(sheet.ast(), selectors[1])[1].data(sheet.ast()),
            SelectorComponentData::Combinator(Combinator::Child)
        ));
        assert!(matches!(
            components(sheet.ast(), selectors[1])[3].data(sheet.ast()),
            SelectorComponentData::PseudoClass(value) if matches!(value.data(sheet.ast()), PseudoClassData::Hover)
        ));

        let declarations = property_declarations(&sheet, rule_id);
        assert_eq!(declarations.len(), 3);
        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Color(value)
                if matches!(value.data(sheet.ast()), CssColorData::Known(KnownColor::Red))
        ));
        assert!(matches!(declarations[1].0.data(sheet.ast()), DeclarationData::Opacity(0.5)));
        assert!(matches!(
            declarations[2].0.data(sheet.ast()),
            DeclarationData::Custom(value)
                if matches!(value.name(sheet.ast()), CustomPropertyName::Custom("--gap"))
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0))
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        assert!(matches!(
            declarations[0].data(sheet.ast()),
            DeclarationData::Custom(value)
                if matches!(
                    value.value(sheet.ast()),
                    [token] if matches!(
                        token.data(sheet.ast()),
                        TokenOrValueData::Color(color)
                            if matches!(
                                color.data(sheet.ast()),
                                CssColorData::Function(function)
                                    if function.kind(sheet.ast()) == KnownFunction::Rgb
                                        && function.flags(sheet.ast()).contains(FunctionFlags::VALID_RGB)
                            )
                    )
                )
        ));
        for declaration in &declarations[1..4] {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Custom(value)
                    if matches!(
                        value.value(sheet.ast()),
                        [token] if matches!(
                            token.data(sheet.ast()),
                            TokenOrValueData::Function(function)
                                if function.kind(sheet.ast()) == KnownFunction::Rgb
                                    && !function.flags(sheet.ast()).contains(FunctionFlags::VALID_RGB)
                        )
                    )
            ));
        }
        assert!(matches!(
            declarations[4].data(sheet.ast()),
            DeclarationData::Custom(value)
                if value
                    .value(sheet.ast())
                    .iter()
                    .all(|value| matches!(value.data(sheet.ast()), TokenOrValueData::Token(_)))
        ));

        for (declaration, _) in property_declarations(&sheet, root_rule(&sheet, 1)) {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Unparsed(value)
                    if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
                        && matches!(
                            value.value(sheet.ast()),
                            [token] if matches!(
                                token.data(sheet.ast()),
                                TokenOrValueData::Function(function)
                                    if function.kind(sheet.ast()) == KnownFunction::Rgb
                                        && !function.flags(sheet.ast()).contains(FunctionFlags::VALID_RGB)
                            )
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0))
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        for declaration in &declarations[..4] {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Custom(value)
                    if matches!(
                        value.value(sheet.ast()),
                        [token] if matches!(
                            token.data(sheet.ast()),
                            TokenOrValueData::Color(color)
                                if matches!(
                                    color.data(sheet.ast()),
                                    CssColorData::Function(function)
                                        if function.flags(sheet.ast()).contains(FunctionFlags::VALID_RGB)
                                )
                        )
                    )
            ));
        }
        for declaration in &declarations[4..] {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Color(color) | DeclarationData::BackgroundColor(color)
                    if matches!(
                        color.data(sheet.ast()),
                        CssColorData::Function(function)
                            if function.flags(sheet.ast()).contains(FunctionFlags::VALID_RGB)
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0))
            .into_iter()
            .map(|(declaration, _)| declaration)
            .collect::<std::vec::Vec<_>>();

        for declaration in &declarations[..3] {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Unparsed(value)
                    if value.reason(sheet.ast()) == UnparsedPropertyReason::InvalidValue
            ));
        }
        for declaration in &declarations[3..6] {
            assert!(
                matches!(
                    declaration.data(sheet.ast()),
                    DeclarationData::Unparsed(value)
                        if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
                            && value.value(sheet.ast()).iter().any(|value| matches!(
                                value.data(sheet.ast()),
                                TokenOrValueData::Token(token)
                                    if matches!(token.data(sheet.ast()), rocketcss_ast_next::TokenData::Comment(_))
                            ))
                ),
                "{declaration:?}"
            );
        }
        assert!(matches!(
            declarations[6].data(sheet.ast()),
            DeclarationData::Display(display)
                if matches!(
                    display.data(sheet.ast()),
                    DisplayData::Pair {
                        outside: DisplayOutside::Inline,
                        inside,
                        is_list_item: false,
                    } if matches!(inside.data(sheet.ast()), DisplayInsideData::FlowRoot)
                )
        ));
        for (declaration, prefix) in [
            (declarations[7], VendorPrefix::WEBKIT),
            (declarations[8], VendorPrefix::MOZ),
        ] {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Display(display)
                    if matches!(
                        display.data(sheet.ast()),
                        DisplayData::Pair {
                            outside: DisplayOutside::Inline,
                            inside,
                            is_list_item: false,
                        } if matches!(
                            inside.data(sheet.ast()),
                            DisplayInsideData::Box { vendor_prefix } if vendor_prefix == prefix
                        )
                    )
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
        let rule = root_rule(&sheet, 0);
        let declarations = property_declarations(&sheet, rule);

        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Color(value)
                if matches!(value.data(sheet.ast()), CssColorData::Known(KnownColor::Blue))
        ));
        assert!(matches!(
            declarations[1].0.data(sheet.ast()),
            DeclarationData::BackgroundColor(value)
                if matches!(value.data(sheet.ast()), CssColorData::Known(KnownColor::Lightgreen))
        ));
        assert!(matches!(
            declarations[2].0.data(sheet.ast()),
            DeclarationData::Background(values)
                if matches!(
                    sheet.ast().data(values[0]).color.data(sheet.ast()),
                    CssColorData::Known(KnownColor::Blue)
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
        let rule = root_rule(&sheet, 0);
        let selectors = style_selectors(&sheet, rule);
        assert!(matches!(
            components(sheet.ast(), selectors[0])[0].data(sheet.ast()),
            SelectorComponentData::Class(name) if name == "foo"
        ));

        let declarations = property_declarations(&sheet, rule);
        let DeclarationData::Unparsed(width) = declarations[0].0.data(sheet.ast()) else {
            panic!("expected opaque width")
        };
        assert!(matches!(width.property_id(sheet.ast()), PropertyId::Width));
        assert_eq!(
            width.reason(sheet.ast()),
            UnparsedPropertyReason::OpaqueValue
        );
        assert!(width.value(sheet.ast()).iter().any(|value| matches!(
            value.data(sheet.ast()),
            TokenOrValueData::Function(function) if function.name(sheet.ast()) == "calc"
        )));
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
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        let SelectorComponentData::Class(first) =
            components(sheet.ast(), selectors[0])[0].data(sheet.ast())
        else {
            panic!("expected first class selector")
        };
        let SelectorComponentData::Class(second) =
            components(sheet.ast(), selectors[1])[0].data(sheet.ast())
        else {
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
        let scope = root_rule(&sheet, 0);
        let scope_id = scope;
        let CssRulePayloadData::Scope(scope) = scope.payload(sheet.ast()).data(sheet.ast()) else {
            panic!("expected scope rule")
        };
        let SelectorComponentData::Class(scope_class) =
            &components(sheet.ast(), scope.scope_start(sheet.ast()).unwrap()[0])[0]
                .data(sheet.ast())
        else {
            panic!("expected scope class selector")
        };
        let scoped_rule = child_rule_ids(&sheet, scope_id)[0];
        let SelectorComponentData::Class(rule_class) =
            &components(sheet.ast(), style_selectors(&sheet, scoped_rule)[0])[0].data(sheet.ast())
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

        let CssRulePayloadData::Import(rule) =
            root_rule(&sheet, 0).payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected import")
        };
        assert_eq!(rule.url(sheet.ast()), "a.css");
        assert!(matches!(
            rule.media(sheet.ast())
                .map(|media| media.media_queries(sheet.ast())[0].media_type(sheet.ast())),
            Some(MediaType::Screen)
        ));

        let media = root_rule(&sheet, 1);
        let media_id = media;
        let CssRulePayloadData::Media(rule) = media.payload(sheet.ast()).data(sheet.ast()) else {
            panic!("expected media")
        };
        assert_eq!(child_rule_ids(&sheet, media_id).len(), 1);
        assert!(matches!(
            rule.query(sheet.ast()).media_queries(sheet.ast())[0].media_type(sheet.ast()),
            MediaType::Screen
        ));
        assert!(
            rule.query(sheet.ast()).media_queries(sheet.ast())[0]
                .condition(sheet.ast())
                .is_some()
        );

        let font_face = root_rule(&sheet, 2);
        let font_face_id = font_face;
        let CssRulePayloadData::FontFace(_) = font_face.payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected font-face")
        };
        let font_face_block = font_face_id.declarations(sheet.ast()).unwrap();
        let properties = font_face_block.declarations(sheet.ast()).to_vec();
        assert_eq!(properties.len(), 2);
        let DeclarationPayloadData::FontFace(property) = properties[0].data(sheet.ast()) else {
            panic!("expected font-face descriptor")
        };
        let FontFacePropertyData::Custom(value) = property.data(sheet.ast()) else {
            panic!("expected custom font-face descriptor")
        };
        assert!(matches!(
            value.name(sheet.ast()),
            CustomPropertyName::Unknown("font-family")
        ));

        let CssRulePayloadData::Unknown(rule) =
            root_rule(&sheet, 3).payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected unknown at-rule")
        };
        assert_eq!(rule.name(sheet.ast()), "unknown");
        assert!(rule.block(sheet.ast()).is_some());
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
               (width <= 30em) {
            a { color: red }
        }
        "#;
        let sheet = parse(source, &allocator, &mut token, ParserOptions::default()).unwrap();
        let ast = sheet.ast();
        let CssRulePayloadData::Media(rule) = root_rule(&sheet, 0).payload(ast).data(ast) else {
            panic!("expected media rule")
        };
        let queries = rule.query(ast).media_queries(ast);
        assert_eq!(queries.len(), 5);

        let condition = queries[0].condition(ast).unwrap();
        let MediaConditionData::Operation {
            operator,
            conditions,
        } = condition.data(ast)
        else {
            panic!("expected operation")
        };
        assert_eq!(operator, Operator::And);
        let MediaConditionData::Feature(feature) = conditions[0].data(ast) else {
            panic!("expected feature")
        };
        let QueryFeatureData::Range {
            name,
            operator,
            value,
        } = feature.data(ast)
        else {
            panic!("expected range")
        };
        assert_eq!(name, MediaFeatureName::Standard(MediaFeatureId::Width));
        assert_eq!(operator, MediaFeatureComparison::GreaterThanEqual);
        let MediaFeatureValueData::Length(length) = value.data(ast) else {
            panic!("expected length")
        };
        let LengthData::Value(length) = length.data(ast) else {
            panic!("expected plain length")
        };
        assert_eq!(length.value(ast), 600.0);
        assert_eq!(length.unit(ast), LengthUnit::Px);

        let condition = queries[1].condition(ast).unwrap();
        let MediaConditionData::Not(inner) = condition.data(ast) else {
            panic!("expected not")
        };
        let MediaConditionData::Feature(feature) = inner.data(ast) else {
            panic!("expected feature")
        };
        assert!(matches!(
            feature.data(ast),
            QueryFeatureData::Boolean {
                name: MediaFeatureName::Standard(MediaFeatureId::Hover)
            }
        ));

        let condition = queries[2].condition(ast).unwrap();
        let MediaConditionData::Feature(feature) = condition.data(ast) else {
            panic!("expected feature")
        };
        assert!(matches!(
            feature.data(ast),
            QueryFeatureData::Interval {
                name: MediaFeatureName::Standard(MediaFeatureId::Width),
                start_operator: MediaFeatureComparison::LessThan,
                end_operator: MediaFeatureComparison::LessThanEqual,
                ..
            }
        ));

        assert_eq!(queries[3].media_type(ast), MediaType::Screen);
        let condition = queries[3].condition(ast).unwrap();
        let MediaConditionData::Feature(feature) = condition.data(ast) else {
            panic!("expected feature")
        };
        let QueryFeatureData::Plain { name, value } = feature.data(ast) else {
            panic!("expected plain feature")
        };
        assert_eq!(name, MediaFeatureName::Standard(MediaFeatureId::Resolution));
        assert!(matches!(
            value.data(ast),
            MediaFeatureValueData::Resolution(Resolution::Dppx(2.0))
        ));

        let condition = queries[4].condition(ast).unwrap();
        let MediaConditionData::Feature(feature) = condition.data(ast) else {
            panic!("expected feature")
        };
        assert!(matches!(
            feature.data(ast),
            QueryFeatureData::Range {
                name: MediaFeatureName::Standard(MediaFeatureId::Width),
                operator: MediaFeatureComparison::LessThanEqual,
                ..
            }
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

        let rule = root_rule(&sheet, 0);
        let selectors = style_selectors(&sheet, rule);
        assert!(matches!(selectors[0].data(sheet.ast()), SelectorData::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(
            property_declarations(&sheet, rule)[0].0.data(sheet.ast()),
            DeclarationData::Color(_)
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

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        assert_eq!(selectors.len(), 3);
        assert!(matches!(selectors[0].data(sheet.ast()), SelectorData::Parsed(_)));
        assert!(matches!(selectors[1].data(sheet.ast()), SelectorData::Unparsed(raw) if raw == "(font-[family-name:var(--font-*)])"
        ));
        assert!(matches!(selectors[2].data(sheet.ast()), SelectorData::Parsed(_)));
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

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        assert_eq!(selectors.len(), 3);
        assert!(matches!(selectors[0].data(sheet.ast()), SelectorData::Parsed(_)));
        assert!(matches!(selectors[1].data(sheet.ast()), SelectorData::Unparsed(raw) if raw == ".broken ?? trailing"
        ));
        assert!(matches!(selectors[2].data(sheet.ast()), SelectorData::Parsed(_)));
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
            if token.span == rocketcss_parser::Span::new(0, 1)
    ));
}

#[test]
fn parses_selectors_from_strings() {
    let allocator = Allocator::new();
    let (compiler, selectors) = parse_selectors(".a:is(.b, #c)", &allocator);
    let ast = compiler.ast();
    assert_eq!(selectors.len(), 1);
    assert!(matches!(
        components(ast, selectors[0])[1].data(ast),
        SelectorComponentData::Is(list) if list.len() == 2
    ));
}

#[test]
fn parses_namespace_deep_and_empty_where_selectors() {
    let allocator = Allocator::new();
    let (compiler, selectors) = parse_selectors(
        "|e, *|*, svg|circle, [svg|fill=red], .a /deep/ .b, foo:where()",
        &allocator,
    );
    let ast = compiler.ast();

    assert!(matches!(
        &component_data_vec(ast, selectors[0])[..],
        [
            SelectorComponentData::ExplicitNoNamespace,
            SelectorComponentData::LocalName { name, .. }
        ] if name == "e"
    ));
    assert!(matches!(
        &component_data_vec(ast, selectors[1])[..],
        [
            SelectorComponentData::ExplicitAnyNamespace,
            SelectorComponentData::ExplicitUniversalType
        ]
    ));
    assert!(matches!(
        &component_data_vec(ast, selectors[2])[..],
        [
            SelectorComponentData::Namespace { prefix, .. },
            SelectorComponentData::LocalName { name, .. }
        ] if prefix == "svg" && name == "circle"
    ));
    assert!(matches!(
        components(ast, selectors[3])[0].data(ast),
        SelectorComponentData::AttributeOther(attribute)
            if matches!(
                attribute.namespace(ast),
                Some(NamespaceConstraint::Specific { prefix, .. }) if prefix == "svg"
            )
    ));
    assert!(matches!(
        &component_data_vec(ast, selectors[4])[..],
        [
            SelectorComponentData::Class(left),
            SelectorComponentData::Combinator(Combinator::Deep),
            SelectorComponentData::Class(right)
        ] if left == "a" && right == "b"
    ));
    assert!(matches!(
        &component_data_vec(ast, selectors[5])[..],
        [SelectorComponentData::LocalName { name, .. }, SelectorComponentData::Where(list)]
            if name == "foo" && list.is_empty()
    ));
}

#[test]
fn parses_selection_and_placeholder_vendor_prefixes_into_typed_selectors() {
    let allocator = Allocator::new();
    let (compiler, selectors) = parse_selectors(
        "::-MoZ-selection,::-webkit-placeholder,::placeholder",
        &allocator,
    );
    let ast = compiler.ast();

    assert!(matches!(
        components(ast, selectors[0])[0].data(ast),
        SelectorComponentData::PseudoElement(element)
            if matches!(element.data(ast), PseudoElementData::Selection(VendorPrefix::MOZ))
    ));
    assert!(matches!(
        components(ast, selectors[1])[0].data(ast),
        SelectorComponentData::PseudoElement(element)
            if matches!(element.data(ast), PseudoElementData::Placeholder(VendorPrefix::WEBKIT))
    ));
    assert!(matches!(
        components(ast, selectors[2])[0].data(ast),
        SelectorComponentData::PseudoElement(element)
            if matches!(element.data(ast), PseudoElementData::Placeholder(VendorPrefix::NONE))
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
        let keyframes = root_rule(&sheet, 0);
        let frames = child_rule_ids(&sheet, keyframes);
        assert_eq!(frames.len(), 2);
        let CssRulePayloadData::Keyframe(first) = frames[0].payload(sheet.ast()).data(sheet.ast()) else {
            panic!("expected keyframe")
        };
        assert!(matches!(
            &first.selectors(sheet.ast())[0].data(sheet.ast()),
            KeyframeSelectorData::TimelineRangePercentage(value)
                if value.name(sheet.ast()) == TimelineRangeName::Entry && value.percentage(sheet.ast()) == 0.0
        ));
        let CssRulePayloadData::Keyframe(second) = frames[1].payload(sheet.ast()).data(sheet.ast()) else {
            panic!("expected keyframe")
        };
        assert!(matches!(
            &second.selectors(sheet.ast())[0].data(sheet.ast()),
            KeyframeSelectorData::TimelineRangePercentage(value)
                if value.name(sheet.ast()) == TimelineRangeName::Exit && value.percentage(sheet.ast()) == 1.0
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
            roots[0].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Namespace(rule)
                if rule.prefix(sheet.ast()) == Some("svg") && rule.url(sheet.ast()) == "http://www.w3.org/2000/svg"
        ));
        assert!(matches!(
            roots[1].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::LayerStatement(rule)
                if rule.names(sheet.ast()).len() == 2 && rule.names(sheet.ast())[1] == ["theme", "base"]
        ));
        assert!(matches!(
            roots[2].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::LayerBlock(rule) if rule.name(sheet.ast()).is_some()
        ));
        assert_eq!(child_rule_ids(&sheet, roots[2]).len(), 1);
        assert!(matches!(
            roots[3].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::CustomMedia(rule)
                if rule.name(sheet.ast()) == "--narrow" && rule.query(sheet.ast()).media_queries(sheet.ast()).len() == 1
        ));
        assert!(matches!(
            roots[4].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Keyframes(rule)
                if matches!(rule.name(sheet.ast()), KeyframesName::Ident("fade"))
        ));
        let frames = child_rule_ids(&sheet, roots[4]);
        assert_eq!(frames.len(), 3);
        assert!(matches!(
            frames[1].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Keyframe(rule)
                if matches!(rule.selectors(sheet.ast())[0].data(sheet.ast()), KeyframeSelectorData::Percentage(0.5))
        ));
        assert!(matches!(
            roots[5].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::CounterStyle(_)
        ));
        assert!(matches!(
            roots[6].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Viewport(_)
        ));
        assert!(matches!(
            roots[7].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::PositionTry(rule) if rule.name(sheet.ast()) == "--fallback"
        ));
        assert!(matches!(
            roots[8].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Container(rule) if rule.name(sheet.ast()) == Some("card") && rule.condition(sheet.ast()).is_some()
        ));
        assert!(matches!(
            roots[9].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::MozDocument(_)
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

        let CssRulePayloadData::Import(import) = roots[0].payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected import")
        };
        assert_eq!(import.layer(sheet.ast()), Some(&["theme", "base"][..]));
        assert!(import.supports(sheet.ast()).is_some());
        assert!(matches!(
            import
                .media(sheet.ast())
                .map(|media| media.media_queries(sheet.ast())[0].media_type(sheet.ast())),
            Some(MediaType::Print)
        ));

        assert!(matches!(
            roots[1].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Scope(rule)
                if rule.scope_start(sheet.ast()).is_some() && rule.scope_end(sheet.ast()).is_some()
        ));
        assert_eq!(child_rule_ids(&sheet, roots[1]).len(), 1);
        assert!(matches!(
            roots[2].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Page(rule) if rule.selectors(sheet.ast()).len() == 1
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
            root_rule(&valid, 0).payload(valid.ast()).data(valid.ast()),
            CssRulePayloadData::Charset(rule)
                if rule.encoding(valid.ast()) == "UTF-8" && valid.ast().span(rule) == Span::new(0, 17)
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
            root_rule(&sheet, 0).payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Charset(rule)
                if rule.encoding(sheet.ast()) == "UTF-8" && sheet.ast().span(rule) == Span::new(0, 20)
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
        let style = root_rule(&sheet, 0);
        let media = child_rule_ids(&sheet, style)[0];
        assert!(matches!(
            media.payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Media(_)
        ));
        let media_children = child_rule_ids(&sheet, media);
        assert!(matches!(
            media_children[0].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::NestedDeclarations(_)
        ));
        assert_eq!(property_declarations(&sheet, media_children[0]).len(), 1);
        assert!(matches!(
            media_children[1].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Style(_)
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
        let style = root_rule(&sheet, 0);
        assert_eq!(property_declarations(&sheet, style).len(), 1);
        let children = child_rule_ids(&sheet, style);
        assert_eq!(children.len(), 1);
        assert!(matches!(
            children[0].payload(sheet.ast()).data(sheet.ast()),
            CssRulePayloadData::Style(_)
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0));
        assert_eq!(declarations.len(), 1);
        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Width(_)
        ));
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
        let style = root_rule(&sheet, 0);
        let declarations = property_declarations(&sheet, style);
        assert_eq!(declarations.len(), 3);
        assert!(style.children(sheet.ast()).is_empty());
        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Width(_)
        ));
        assert!(matches!(
            declarations[1].0.data(sheet.ast()),
            DeclarationData::Height(_)
        ));
        assert!(matches!(
            declarations[2].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if matches!(value.property_id(sheet.ast()), PropertyId::Background)
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
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0));
    let ast = sheet.ast();
    assert!(matches!(
        declarations[0].data(ast),
        DeclarationData::Color(color)
            if matches!(
                color.data(ast),
                CssColorData::Rgba(rgba)
                    if ast.data(rgba) == RGBAData { red: 0, green: 255, blue: 0, alpha: 136 }
            )
    ));
    assert!(matches!(
        declarations[1].data(ast),
        DeclarationData::BackgroundColor(color)
            if matches!(color.data(ast), CssColorData::CurrentColor)
    ));
    assert!(matches!(declarations[2].data(ast), DeclarationData::Display(_)));
    assert!(matches!(
        declarations[3].data(ast),
        DeclarationData::Visibility(Visibility::Hidden)
    ));
    assert!(matches!(declarations[4].data(ast), DeclarationData::Width(_)));
    assert!(matches!(declarations[5].data(ast), DeclarationData::Height(_)));
    assert!(matches!(
        declarations[6].data(ast),
        DeclarationData::All(CSSWideKeyword::RevertLayer)
    ));
    })
}

#[test]
fn parses_typed_sizing_and_overflow_values() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { box-sizing: border-box; aspect-ratio: auto 2 / 3; overflow: hidden scroll; overflow-x: clip; overflow-y: auto; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        let ast = sheet.ast();
        assert!(matches!(
            declarations[0].data(ast),
            DeclarationData::BoxSizing(BoxSizing::BorderBox, VendorPrefix::NONE)
        ));
        assert!(matches!(
            declarations[1].data(ast),
            DeclarationData::AspectRatio(aspect)
                if matches!(
                    ast.data(aspect),
                    AspectRatioData {
                        auto: true,
                        ratio: Some(ratio),
                    } if matches!(
                        ast.data(ratio),
                        RatioData {
                            denominator: Some(3.0),
                            numerator: 2.0,
                        }
                    )
                )
        ));
        assert!(matches!(
            declarations[2].data(ast),
            DeclarationData::Overflow(overflow)
                if matches!(
                    ast.data(overflow),
                    OverflowData {
                        x: OverflowKeyword::Hidden,
                        y: OverflowKeyword::Scroll,
                    }
                )
        ));
        assert!(matches!(
            declarations[3].data(sheet.ast()),
            DeclarationData::OverflowX(OverflowKeyword::Clip)
        ));
        assert!(matches!(
            declarations[4].data(sheet.ast()),
            DeclarationData::OverflowY(OverflowKeyword::Auto)
        ));
    })
}

#[test]
fn rejects_auto_for_scroll_margin_and_padding() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { scroll-margin-block: auto; scroll-padding-inline: auto; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        for declaration in declarations {
            assert!(matches!(
                declaration.data(sheet.ast()),
                DeclarationData::Unparsed(value)
                    if value.reason(sheet.ast()) == UnparsedPropertyReason::InvalidValue
            ));
        }
    })
}

#[test]
fn parses_typed_box_border_and_mask_families() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { margin: 1px 2px; padding-inline: 3px 4px; inset: 5px auto; position: sticky; z-index: 2; border: 1px solid red; border-color: red blue; border-radius: 1px 2px / 3px 4px; mask-image: linear-gradient(red, blue), url(mask.svg); mask-composite: add, exclude; mask-type: alpha; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::Margin(_)));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::PaddingInline(_)));
        assert!(matches!(declarations[2].data(sheet.ast()), DeclarationData::Inset(_)));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::Position(_)));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::ZIndex(ZIndex::Integer(2))));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::Border(_)));
        assert!(matches!(declarations[6].data(sheet.ast()), DeclarationData::BorderColor(_)));
        assert!(matches!(declarations[7].data(sheet.ast()), DeclarationData::BorderRadius(_, _)));
        assert!(matches!(declarations[8].data(sheet.ast()), DeclarationData::MaskImage(_, _)));
        assert!(matches!(declarations[9].data(sheet.ast()), DeclarationData::MaskComposite(_)));
        assert!(matches!(declarations[10].data(sheet.ast()), DeclarationData::MaskType(MaskType::Alpha)));
    })
}

#[test]
fn parses_mask_shorthand_layers_without_losing_defaults() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { mask: url(one.svg) center / cover no-repeat padding-box content-box exclude alpha, linear-gradient(red, blue); }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));
        let DeclarationData::Mask(layers, VendorPrefix::NONE) = declarations[0].data(sheet.ast()) else {
            panic!("expected typed mask shorthand")
        };
        let ast = sheet.ast();
        assert_eq!(layers.len(), 2);
        let first = layers[0];
        assert!(matches!(first.image(ast).data(ast), ImageData::Url(_)));
        assert!(matches!(first.size(ast).data(ast), BackgroundSizeData::Cover));
        assert!(matches!(
            first.repeat(ast).x(ast),
            BackgroundRepeatKeyword::NoRepeat
        ));
        assert!(matches!(first.origin(ast), GeometryBox::PaddingBox));
        assert!(matches!(
            first.clip(ast),
            MaskClip::GeometryBox(GeometryBox::ContentBox)
        ));
        assert!(matches!(first.composite(ast), MaskComposite::Exclude));
        assert!(matches!(first.mode(ast), MaskMode::Alpha));
        assert!(matches!(layers[1].image(ast).data(ast), ImageData::Gradient(_)));
        assert!(matches!(layers[1].composite(ast), MaskComposite::Add));
        assert!(matches!(layers[1].mode(ast), MaskMode::MatchSource));
    })
}

#[test]
fn parses_mask_repeat_x_and_y_as_typed_layers() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { mask: url(one.svg) repeat-x, url(two.svg) repeat-y; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let DeclarationData::Mask(layers, VendorPrefix::NONE) =
            declaration_values(&sheet, root_rule(&sheet, 0))[0].data(sheet.ast())
        else {
            panic!("expected typed mask shorthand")
        };
        assert!(matches!(
            (
                layers[0].repeat(sheet.ast()).x(sheet.ast()),
                layers[0].repeat(sheet.ast()).y(sheet.ast())
            ),
            (
                BackgroundRepeatKeyword::Repeat,
                BackgroundRepeatKeyword::NoRepeat
            )
        ));
        assert!(matches!(
            (
                layers[1].repeat(sheet.ast()).x(sheet.ast()),
                layers[1].repeat(sheet.ast()).y(sheet.ast())
            ),
            (
                BackgroundRepeatKeyword::NoRepeat,
                BackgroundRepeatKeyword::Repeat
            )
        ));
    })
}

#[test]
fn parses_radial_mask_images_and_gradient_hints() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { mask-image: radial-gradient(circle at 25% 75%, transparent 8px, black 8px), repeating-radial-gradient(ellipse closest-side, red, blue 50%, 75%), conic-gradient(from 45deg at 25% 75%, red, blue 50%, 75%), image-set(url(one.png) 1x, \"two.png\" 2x); }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));
        let DeclarationData::MaskImage(images, _) = declarations[0].data(sheet.ast()) else {
            panic!("expected typed mask-image")
        };
        let ast = sheet.ast();
        let ImageData::Gradient(gradient) = images[0].data(ast) else {
            panic!("expected gradient image")
        };
        assert!(matches!(gradient.data(ast), GradientData::Radial { .. }));
        let ImageData::Gradient(gradient) = images[1].data(ast) else {
            panic!("expected gradient image")
        };
        let GradientData::RepeatingRadial { items, .. } = gradient.data(ast) else {
            panic!("expected repeating radial gradient")
        };
        assert!(items.iter().any(|item| matches!(item.data(ast), GradientItemData::Hint(_))));
        let ImageData::Gradient(gradient) = images[2].data(ast) else {
            panic!("expected gradient image")
        };
        assert!(matches!(gradient.data(ast), GradientData::Conic { .. }));
        let ImageData::ImageSet(image_set) = images[3].data(ast) else {
            panic!("expected image-set")
        };
        assert_eq!(image_set.options(ast).len(), 2);
    })
}

#[test]
fn parses_typed_flex_alignment_transform_and_font_values() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { flex-direction: column; flex-wrap: wrap; flex-flow: row wrap; flex: 1 0 10px; flex: 1 auto; flex-grow: 2; order: 3; align-items: center; justify-content: space-between; align-self: safe flex-end; transform: translateX(1px) rotateZ(.25turn); transform-style: preserve-3d; translate: 1px 2px; rotate: 90deg; scale: 2; font-weight: bold; font-size: 1rem; line-height: 1.5; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::FlexDirection(FlexDirection::Column, _)));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::FlexWrap(FlexWrap::Wrap, _)));
        assert!(matches!(declarations[2].data(sheet.ast()), DeclarationData::FlexFlow(_, _)));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::Flex(_, _)));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::Flex(_, _)));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::FlexGrow(2.0, _)));
        assert!(matches!(declarations[6].data(sheet.ast()), DeclarationData::Order(3.0, _)));
        assert!(matches!(declarations[7].data(sheet.ast()), DeclarationData::AlignItems(AlignItems::SelfPosition { value: SelfPosition::Center, .. }, _)));
        assert!(matches!(declarations[8].data(sheet.ast()), DeclarationData::JustifyContent(JustifyContent::ContentDistribution(ContentDistribution::SpaceBetween), _)));
        assert!(matches!(declarations[9].data(sheet.ast()), DeclarationData::AlignSelf(AlignSelf::SelfPosition { .. }, _)));
        assert!(matches!(declarations[10].data(sheet.ast()), DeclarationData::Transform(values, _) if values.len() == 2));
        assert!(matches!(declarations[11].data(sheet.ast()), DeclarationData::TransformStyle(TransformStyle::Preserve3d, _)));
        assert!(matches!(declarations[12].data(sheet.ast()), DeclarationData::Translate(_,)));
        assert!(matches!(declarations[13].data(sheet.ast()), DeclarationData::Rotate(_)));
        assert!(matches!(declarations[14].data(sheet.ast()), DeclarationData::Scale(_,)));
        assert!(matches!(declarations[15].data(sheet.ast()), DeclarationData::FontWeight(FontWeight::Absolute(AbsoluteFontWeight::Bold))));
        assert!(matches!(declarations[16].data(sheet.ast()), DeclarationData::FontSize(_)));
        assert!(matches!(declarations[17].data(sheet.ast()), DeclarationData::LineHeight(_)));
    })
}

#[test]
fn parses_single_and_two_value_layout_shorthands() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { place-content: center; place-self: auto; place-items: center end; gap: 1px; row-gap: normal; column-gap: 2%; border-spacing: 1px 2px; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::PlaceContent(_)));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::PlaceSelf(_)));
        assert!(matches!(declarations[2].data(sheet.ast()), DeclarationData::PlaceItems(_)));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::Gap(_)));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::RowGap(value) if matches!(value.data(sheet.ast()), GapValueData::Normal)));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::ColumnGap(_)));
        assert!(matches!(declarations[6].data(sheet.ast()), DeclarationData::BorderSpacing(_)));
    })
}

#[test]
fn parses_typed_svg_paint_and_ui_values() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { fill: currentColor; stroke: url(icon.svg) red; fill-rule: evenodd; stroke-linecap: round; stroke-width: 2px; stroke-dasharray: 1px, 2px; accent-color: #0f0; object-fit: cover; scrollbar-color: auto; user-select: none; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::Fill(_)));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::Stroke(_)));
        assert!(matches!(declarations[2].data(sheet.ast()), DeclarationData::FillRule(FillRule::Evenodd)));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::StrokeLinecap(StrokeLinecap::Round)));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::StrokeWidth(_)));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::StrokeDasharray(_)));
        assert!(matches!(declarations[6].data(sheet.ast()), DeclarationData::AccentColor(_)));
        assert!(matches!(declarations[7].data(sheet.ast()), DeclarationData::ObjectFit(ObjectFit::Cover)));
        assert!(matches!(declarations[8].data(sheet.ast()), DeclarationData::ScrollbarColor(color) if matches!(color.data(sheet.ast()), ScrollbarColorData::Auto)));
        assert!(matches!(declarations[9].data(sheet.ast()), DeclarationData::UserSelect(UserSelect::None, _)));
    })
}

#[test]
fn stylo_derived_property_grammar_keeps_vectors_and_substitutions_typed() {
    // Grammar sources: ../stylo/style/properties/longhands.toml and
    // ../stylo/style/properties/shorthands.toml. This compact matrix covers
    // object-fit/position, scrollbar-color's two-color form, mask vectors,
    // logical identity, and the arbitrary-substitution boundary.
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { object-fit: cover; object-position: 10% 20%; scrollbar-color: red blue; mask-image: url(a.svg), url(b.svg); mask-composite: add, exclude; margin-inline: 1px 2px; width: env(--size, 01.00px); }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::ObjectFit(ObjectFit::Cover)));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::ObjectPosition(_)));
        assert!(matches!(
            declarations[2].data(sheet.ast()),
            DeclarationData::ScrollbarColor(color) if matches!(color.data(sheet.ast()), ScrollbarColorData::Colors(_, _))
        ));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::MaskImage(images, _) if images.len() == 2));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::MaskComposite(values) if values.len() == 2));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::MarginInline(_)));
        assert!(matches!(
            declarations[6].data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
                    && matches!(value.property_id(sheet.ast()), PropertyId::Width)
        ));
    })
}

#[test]
fn parses_typed_text_longhands() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let sheet = parse(
            "a { text-transform: uppercase full-width; white-space: pre-wrap; text-overflow: ellipsis; text-align: justify; word-spacing: normal; letter-spacing: 1px; text-indent: 2em hanging each-line; text-decoration-line: underline line-through; text-decoration-style: wavy; text-decoration-thickness: from-font; text-size-adjust: 80%; direction: rtl; unicode-bidi: isolate; resize: both; }",
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(declarations[0].data(sheet.ast()), DeclarationData::TextTransform(value) if value.full_width(sheet.ast())));
        assert!(matches!(declarations[1].data(sheet.ast()), DeclarationData::WhiteSpace(WhiteSpace::PreWrap)));
        assert!(matches!(declarations[2].data(sheet.ast()), DeclarationData::TextOverflow(TextOverflow::Ellipsis, _)));
        assert!(matches!(declarations[3].data(sheet.ast()), DeclarationData::TextAlign(TextAlign::Justify)));
        assert!(matches!(declarations[4].data(sheet.ast()), DeclarationData::WordSpacing(_)));
        assert!(matches!(declarations[5].data(sheet.ast()), DeclarationData::LetterSpacing(_)));
        assert!(matches!(declarations[6].data(sheet.ast()), DeclarationData::TextIndent(value) if value.hanging(sheet.ast()) && value.each_line(sheet.ast())));
        assert!(matches!(declarations[7].data(sheet.ast()), DeclarationData::TextDecorationLine(_, _)));
        assert!(matches!(declarations[8].data(sheet.ast()), DeclarationData::TextDecorationStyle(TextDecorationStyle::Wavy, _)));
        assert!(matches!(declarations[9].data(sheet.ast()), DeclarationData::TextDecorationThickness(_)));
        assert!(matches!(declarations[10].data(sheet.ast()), DeclarationData::TextSizeAdjust(TextSizeAdjust::Percentage(value), _) if (value - 0.8).abs() < f32::EPSILON));
        assert!(matches!(declarations[11].data(sheet.ast()), DeclarationData::Direction(TextDirection::Rtl)));
        assert!(matches!(declarations[12].data(sheet.ast()), DeclarationData::UnicodeBidi(UnicodeBidi::Isolate)));
        assert!(matches!(declarations[13].data(sheet.ast()), DeclarationData::Resize(Resize::Both)));
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
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

    assert!(matches!(
        declarations[0].data(sheet.ast()),
        DeclarationData::FontFamily(families)
            if {
                let ast = sheet.ast();
                let values: std::vec::Vec<_> =
                    families.iter().map(|family| family.data(ast)).collect();
                matches!(&values[..], [
                    FontFamilyData::Custom("serif"),
                    FontFamilyData::SansSerif,
                    FontFamilyData::Custom("Fancy Font"),
                    FontFamilyData::Custom("A"),
                    FontFamilyData::Custom("slab inherit"),
                ])
            }
    ));
    assert!(matches!(
        declarations[1].data(sheet.ast()),
        DeclarationData::Unparsed(value)
            if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
                && matches!(value.property_id(sheet.ast()), PropertyId::FontFamily)
    ));
    assert!(matches!(
        declarations[2].data(sheet.ast()),
        DeclarationData::Unparsed(value)
            if value.reason(sheet.ast()) == UnparsedPropertyReason::InvalidValue
                && matches!(value.property_id(sheet.ast()), PropertyId::FontFamily)
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
    let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

    assert!(matches!(
        declarations[0].data(sheet.ast()),
        DeclarationData::ColumnRule(value, prefix)
            if prefix.bits & VendorPrefix::WEBKIT.bits != 0
                && matches!(value.style(sheet.ast()), Some(LineStyle::Solid))
                && value.width(sheet.ast()).is_some()
                && value.color(sheet.ast()).is_some()
    ));
    assert!(matches!(
        declarations[1].data(sheet.ast()),
        DeclarationData::Columns(CSSWideOr::Value(value), prefix)
            if prefix == VendorPrefix::NONE
                && matches!(value.count(sheet.ast()), ColumnCount::Integer(3))
                && matches!(value.width(sheet.ast()).data(sheet.ast()), ColumnWidthData::Length(_))
    ));
    assert!(matches!(
        declarations[2].data(sheet.ast()),
        DeclarationData::GridColumnGap(value)
            if matches!(value.data(sheet.ast()), GapValueData::LengthPercentage(_))
    ));
    assert!(matches!(
        declarations[3].data(sheet.ast()),
        DeclarationData::GridRowGap(value) if matches!(value.data(sheet.ast()), GapValueData::Normal)
    ));
    assert!(matches!(
        declarations[4].data(sheet.ast()),
        DeclarationData::Unparsed(value)
            if matches!(value.property_id(sheet.ast()), PropertyId::Columns(VendorPrefix::NONE))
    ));
    assert!(matches!(
        declarations[5].data(sheet.ast()),
        DeclarationData::ColumnWidth(CSSWideOr::CSSWide(CSSWideKeyword::Inherit), prefix)
            if prefix == VendorPrefix::NONE
    ));
    assert!(matches!(
        declarations[6].data(sheet.ast()),
        DeclarationData::Columns(CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer), prefix)
            if prefix == VendorPrefix::NONE
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0));

        assert_eq!(declarations.len(), 7);
        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Color(_)
        ));
        assert!(declarations[0].1);

        assert!(matches!(
            declarations[1].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
                    && matches!(value.property_id(sheet.ast()), PropertyId::Width)
        ));
        assert!(declarations[1].1);
        assert!(matches!(
            declarations[2].0.data(sheet.ast()),
            DeclarationData::Transform(values, prefix)
                if values.len() == 1 && prefix.bits & VendorPrefix::WEBKIT.bits != 0
        ));
        assert!(matches!(
            declarations[3].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if matches!(value.property_id(sheet.ast()), PropertyId::Custom("future-property"))
        ));
        assert!(matches!(
            declarations[4].0.data(sheet.ast()),
            DeclarationData::Custom(value)
                if matches!(value.name(sheet.ast()), CustomPropertyName::Custom("--theme"))
                    && value.value(sheet.ast()).iter().any(|token| matches!(token.data(sheet.ast()),
                        TokenOrValueData::Function(function) if function.name(sheet.ast()) == "fn"))
        ));
        assert!(declarations[4].1);
        assert!(matches!(
            declarations[5].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if matches!(value.property_id(sheet.ast()), PropertyId::Opacity)
        ));
        assert!(!declarations[5].1);
        assert!(matches!(
            declarations[6].0.data(sheet.ast()),
            DeclarationData::Height(_)
        ));
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
                box-shadow: 0 0 1px red;
                future-property: fn(1);
            }"#,
            &allocator,
            &mut token,
            ParserOptions::default(),
        )
        .unwrap();
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(
            declarations[0].data(sheet.ast()),
            DeclarationData::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(property_id, PropertyId::Width)
        ));
        assert!(matches!(
            declarations[1].data(sheet.ast()),
            DeclarationData::MaxWidth(value)
                if matches!(value.data(sheet.ast()), MaxSizeData::FitContentFunction(_))
        ));
        assert!(matches!(
            declarations[2].data(sheet.ast()),
            DeclarationData::BorderTopStyle(LineStyle::Solid)
        ));
        assert!(matches!(
            declarations[3].data(sheet.ast()),
            DeclarationData::AnimationDuration(values, prefix)
                if values.len() == 2 && prefix == VendorPrefix::NONE
        ));
        assert!(matches!(
            declarations[4].data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            declarations[5].data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            declarations[6].data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::UnsupportedGrammar
                    && matches!(value.property_id(sheet.ast()), PropertyId::BoxShadow(..))
        ));
        assert!(matches!(
            declarations[7].data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::UnknownProperty
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
        let declarations = property_declarations(&sheet, root_rule(&sheet, 0));

        assert!(matches!(
            declarations[0].0.data(sheet.ast()),
            DeclarationData::Width(_)
        ));
        assert!(matches!(
            declarations[1].0.data(sheet.ast()),
            DeclarationData::CSSWide(property_id, CSSWideKeyword::Inherit)
                if matches!(property_id, PropertyId::Height)
        ));
        assert!(declarations[1].1);
        assert!(matches!(
            declarations[2].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::InvalidValue
        ));
        assert!(matches!(
            declarations[3].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            declarations[4].0.data(sheet.ast()),
            DeclarationData::Custom(_)
        ));
        assert!(matches!(
            declarations[5].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::UnknownProperty
        ));
        assert!(matches!(
            declarations[6].0.data(sheet.ast()),
            DeclarationData::ColumnWidth(
                CSSWideOr::CSSWide(CSSWideKeyword::RevertLayer),
                VendorPrefix::NONE
            )
        ));
        assert!(matches!(
            declarations[7].0.data(sheet.ast()),
            DeclarationData::Unparsed(value)
                if value.reason(sheet.ast()) == UnparsedPropertyReason::OpaqueValue
        ));
        assert!(matches!(
            declarations[8].0.data(sheet.ast()),
            DeclarationData::All(CSSWideKeyword::Unset)
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
        let declarations = declaration_values(&sheet, root_rule(&sheet, 0));

        assert!(matches!(
            declarations[0].data(sheet.ast()),
            DeclarationData::CSSWide(property_id, CSSWideKeyword::Initial)
                if matches!(property_id, PropertyId::Color)
        ));
        assert!(matches!(
            declarations[1].data(sheet.ast()),
            DeclarationData::CSSWide(property_id, CSSWideKeyword::RevertLayer)
                if matches!(property_id, PropertyId::MinWidth)
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
            declaration_values(&sheet, root_rule(&sheet, 0))
                .iter()
                .all(|declaration| {
                    !matches!(
                        declaration.data(sheet.ast()),
                        DeclarationData::Unparsed(value)
                            if matches!(value.property_id(sheet.ast()), PropertyId::Custom("overlay"))
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
        let CssRulePayloadData::Property(property) =
            roots[0].payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected property rule")
        };
        assert_eq!(property.name(sheet.ast()), "--brand-color");
        assert!(property.initial_value(sheet.ast()).is_some());
        assert_eq!(property.inherits(sheet.ast()), Some(false));
        let syntax = property.syntax(sheet.ast()).expect("expected syntax");
        assert!(matches!(
            syntax.data(sheet.ast()),
            SyntaxStringData::Components(_)
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

        let CssRulePayloadData::Property(omitted) =
            root_rule(&sheet, 0).payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected omitted property registration")
        };
        assert!(omitted.initial_value(sheet.ast()).is_none());

        let CssRulePayloadData::Property(empty) =
            root_rule(&sheet, 1).payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected empty property registration")
        };
        assert!(empty.initial_value(sheet.ast()).is_some());

        let CssRulePayloadData::Property(ordered) =
            root_rule(&sheet, 2).payload(sheet.ast()).data(sheet.ast())
        else {
            panic!("expected ordered property registration")
        };
        assert_eq!(ordered.inherits(sheet.ast()), Some(true));
        assert!(ordered.initial_value(sheet.ast()).is_some());
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
        let rule = root_rule(&sheet, 0);
        let selectors = style_selectors(&sheet, rule);
        assert_eq!(selectors.len(), 1);

        let selector = selectors[0];
        assert_eq!(components(sheet.ast(), selector).len(), 3);

        assert!(matches!(
            components(sheet.ast(), selector)[0].data(sheet.ast()),
            SelectorComponentData::LocalName { name, .. } if name == "select"
        ));

        assert!(matches!(
            components(sheet.ast(), selector)[1].data(sheet.ast()),
            SelectorComponentData::PseudoElement(element)
                if matches!(
                    element.data(sheet.ast()),
                    PseudoElementData::CustomFunction { name, .. } if name == "picker"
                )
        ));
        assert!(matches!(
            components(sheet.ast(), selector)[2].data(sheet.ast()),
            SelectorComponentData::Negation(_)
        ));

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
        let rule = root_rule(&sheet, 0);
        let selectors = style_selectors(&sheet, rule);
        assert_eq!(selectors.len(), 1);

        let selector = selectors[0];
        assert_eq!(components(sheet.ast(), selector).len(), 2);

        assert!(matches!(
            components(sheet.ast(), selector)[0].data(sheet.ast()),
            SelectorComponentData::PseudoElement(element)
                if matches!(
                    element.data(sheet.ast()),
                    PseudoElementData::Custom { name } if name == "details-content"
                )
        ));

        assert!(matches!(
            components(sheet.ast(), selector)[1].data(sheet.ast()),
            SelectorComponentData::PseudoElement(element)
                if matches!(element.data(sheet.ast()), PseudoElementData::Before)
        ));

        let DeclarationData::BackgroundColor(_) =
            property_declarations(&sheet, rule)[0].0.data(sheet.ast())
        else {
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
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        assert_eq!(selectors.len(), 1);

        let selector = selectors[0];
        assert_eq!(components(sheet.ast(), selector).len(), 2);

        assert!(matches!(
            components(sheet.ast(), selector)[0].data(sheet.ast()),
            SelectorComponentData::LocalName { name, .. } if name == "slot"
        ));

        assert!(matches!(
            components(sheet.ast(), selector)[1].data(sheet.ast()),
            SelectorComponentData::PseudoClass(pc)
                if matches!(
                    pc.data(sheet.ast()),
                    PseudoClassData::Custom { name } if name == "has-slotted"
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
        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        assert_eq!(selectors.len(), 1);

        let selector = selectors[0];
        assert_eq!(components(sheet.ast(), selector).len(), 2);

        assert!(matches!(
            components(sheet.ast(), selector)[0].data(sheet.ast()),
            SelectorComponentData::LocalName { name, .. } if name == "video"
        ));

        assert!(matches!(
            components(sheet.ast(), selector)[1].data(sheet.ast()),
            SelectorComponentData::Negation(_)
        ));
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

        let selectors = style_selectors(&sheet, root_rule(&sheet, 0));
        assert!(matches!(
            components(sheet.ast(), selectors[0])[0].data(sheet.ast()),
            SelectorComponentData::PseudoElement(element)
                if matches!(
                    element.data(sheet.ast()),
                    PseudoElementData::Custom { name } if name == "scroll-button"
                )
        ));

        let selectors = style_selectors(&sheet, root_rule(&sheet, 1));
        assert!(matches!(
            components(sheet.ast(), selectors[0])[3].data(sheet.ast()),
            SelectorComponentData::PseudoElement(element)
                if matches!(
                    element.data(sheet.ast()),
                    PseudoElementData::Custom { name } if name == "scroll-marker"
                )
        ));
    })
}

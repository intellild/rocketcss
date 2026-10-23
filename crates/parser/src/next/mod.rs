//! Parsers being migrated to the flattened AST.

mod alignment;
mod animation;
mod background;
mod border;
mod box_model;
mod color;
mod css_rule;
mod flex;
mod font;
mod image;
mod length;
mod mask;
mod media;
mod multicol;
mod prelude;
mod properties;
mod rules;
mod selector;
mod svg;
mod text;
mod token;
mod transform;
mod ui;
mod values;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    fn parse_value<'i, T: Parse<'i>>(
        source: &'i str,
        allocator: &'i rocketcss_common::Allocator,
    ) -> (Compiler<'i>, T) {
        let mut compiler = Compiler::new_with_source(source, allocator);
        let value = T::parse(&mut compiler).unwrap();
        (compiler, value)
    }

    #[test]
    fn parses_length_into_value_nodes() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, length) = parse_value::<Length>("10px", &allocator);
        let ast = compiler.ast();
        assert!(matches!(length.data(ast), LengthData::Value(_)));
        let LengthData::Value(value) = length.data(ast) else {
            unreachable!()
        };
        assert_eq!(value.unit(ast), LengthUnit::Px);
        assert_eq!(value.value(ast), 10.0);
    }

    #[test]
    fn parses_hex_and_named_colors() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, color) = parse_value::<CssColor>("#fff", &allocator);
        let ast = compiler.ast();
        let CssColorData::Rgba(rgba) = color.data(ast) else {
            panic!("expected rgba color")
        };
        assert_eq!(rgba.red(ast), 255);
        assert_eq!(rgba.green(ast), 255);
        assert_eq!(rgba.blue(ast), 255);
        assert_eq!(rgba.alpha(ast), 255);

        let (compiler, color) = parse_value::<CssColor>("red", &allocator);
        let ast = compiler.ast();
        assert!(matches!(
            color.data(ast),
            CssColorData::Known(KnownColor::Red)
        ));
    }

    #[test]
    fn parses_function_color_and_validates_rgb() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, color) = parse_value::<CssColor>("rgb(1 2 3)", &allocator);
        let ast = compiler.ast();
        let CssColorData::Function(function) = color.data(ast) else {
            panic!("expected function color")
        };
        assert_eq!(function.kind(ast), KnownFunction::Rgb);
        assert!(function.flags(ast).contains(FunctionFlags::VALID_RGB));
        assert_eq!(function.arguments(ast).len(), 5);
    }

    #[test]
    fn parses_box_model_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, display) = parse_value::<Display>("block flex", &allocator);
        let ast = compiler.ast();
        let DisplayData::Pair {
            inside, outside, ..
        } = display.data(ast)
        else {
            panic!("expected display pair")
        };
        assert_eq!(outside, DisplayOutside::Block);
        assert_eq!(inside.kind(ast), DisplayInsideKind::Flex);

        let (compiler, padding) = parse_value::<Padding>("1px 2px", &allocator);
        let ast = compiler.ast();
        let top = padding.top(ast);
        assert!(!matches!(top.data(ast), LengthPercentageOrAutoData::Auto));
        assert_eq!(padding.bottom(ast), top);

        let (compiler, value) = parse_value::<LengthPercentageOrAuto>("auto", &allocator);
        let ast = compiler.ast();
        assert!(matches!(value.data(ast), LengthPercentageOrAutoData::Auto));
    }

    #[test]
    fn parses_border_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, width) = parse_value::<BorderWidth>("thin 2px", &allocator);
        let ast = compiler.ast();
        assert!(matches!(
            width.top(ast).data(ast),
            BorderSideWidthData::Thin
        ));
        let BorderSideWidthData::Length(length) = width.right(ast).data(ast) else {
            panic!("expected length width")
        };
        let LengthData::Value(value) = length.data(ast) else {
            panic!("expected plain length")
        };
        assert_eq!(value.value(ast), 2.0);
        assert_eq!(width.bottom(ast), width.top(ast));

        let (compiler, border) =
            parse_value::<GenericBorder<LineStyle>>("1px solid red", &allocator);
        let ast = compiler.ast();
        assert_eq!(border.style(ast), LineStyle::Solid);
        assert!(matches!(
            border.width(ast).data(ast),
            BorderSideWidthData::Length(_)
        ));
        assert!(matches!(
            border.color(ast).data(ast),
            CssColorData::Known(_)
        ));
    }

    #[test]
    fn parses_images_and_backgrounds() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, image) = parse_value::<Image>("linear-gradient(red, blue)", &allocator);
        let ast = compiler.ast();
        let ImageData::Gradient(gradient) = image.data(ast) else {
            panic!("expected gradient image")
        };
        let GradientData::Linear {
            direction, items, ..
        } = gradient.data(ast)
        else {
            panic!("expected linear gradient")
        };
        assert_eq!(
            direction,
            LineDirection::Vertical(VerticalPositionKeyword::Bottom)
        );
        assert_eq!(items.len(), 2);
        assert!(matches!(
            items[0].data(ast),
            GradientItemData::ColorStop { .. }
        ));

        let (compiler, image) = parse_value::<Image>("url(a.png)", &allocator);
        let ast = compiler.ast();
        let ImageData::Url(url) = image.data(ast) else {
            panic!("expected url image")
        };
        assert_eq!(url.url(ast), "a.png");

        let (compiler, repeat) = parse_value::<BackgroundRepeat>("repeat-x", &allocator);
        let ast = compiler.ast();
        assert_eq!(repeat.x(ast), BackgroundRepeatKeyword::Repeat);
        assert_eq!(repeat.y(ast), BackgroundRepeatKeyword::NoRepeat);

        let (compiler, size) = parse_value::<BackgroundSize>("1px 2px", &allocator);
        let ast = compiler.ast();
        let BackgroundSizeData::Explicit { height, width } = size.data(ast) else {
            panic!("expected explicit size")
        };
        assert!(!matches!(width.data(ast), LengthPercentageOrAutoData::Auto));
        assert!(matches!(
            height.data(ast),
            LengthPercentageOrAutoData::LengthPercentage(_)
        ));
    }

    #[test]
    fn parses_multicol_and_flex() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, columns) = parse_value::<Columns>("2 100px", &allocator);
        let ast = compiler.ast();
        assert_eq!(columns.count(ast), ColumnCount::Integer(2));
        let ColumnWidthData::Length(width) = columns.width(ast).data(ast) else {
            panic!("expected column length")
        };
        let LengthData::Value(value) = width.data(ast) else {
            panic!("expected plain length")
        };
        assert_eq!(value.value(ast), 100.0);

        let (compiler, gap) = parse_value::<Gap>("10px 20px", &allocator);
        let ast = compiler.ast();
        let GapValueData::LengthPercentage(row) = gap.row(ast).data(ast) else {
            panic!("expected gap length")
        };
        let DimensionPercentageData::Dimension(row) = row.data(ast) else {
            panic!("expected dimension")
        };
        assert_eq!(row.value(ast), 10.0);

        let (compiler, flex) = parse_value::<Flex>("1 1 auto", &allocator);
        let ast = compiler.ast();
        assert_eq!(flex.grow(ast), 1.0);
        assert_eq!(flex.shrink(ast), 1.0);
        assert!(matches!(
            flex.basis(ast).data(ast),
            LengthPercentageOrAutoData::Auto
        ));
    }

    #[test]
    fn parses_font_svg_and_ui_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, size) = parse_value::<FontSize>("12px", &allocator);
        let ast = compiler.ast();
        let FontSizeData::Length(value) = size.data(ast) else {
            panic!("expected length font size")
        };
        let DimensionPercentageData::Dimension(value) = value.data(ast) else {
            panic!("expected dimension")
        };
        assert_eq!(value.value(ast), 12.0);

        let (compiler, family) = parse_value::<FontFamily>("Arial", &allocator);
        let ast = compiler.ast();
        assert!(matches!(family.data(ast), FontFamilyData::Custom("Arial")));

        let (compiler, family) = parse_value::<FontFamily>("serif", &allocator);
        let ast = compiler.ast();
        assert!(matches!(family.data(ast), FontFamilyData::Serif));

        let (compiler, paint) = parse_value::<SVGPaint>("none", &allocator);
        let ast = compiler.ast();
        assert!(matches!(paint.data(ast), SVGPaintData::None));

        let (compiler, color) = parse_value::<ColorOrAuto>("auto", &allocator);
        let ast = compiler.ast();
        assert!(matches!(color.data(ast), ColorOrAutoData::Auto));

        let (compiler, line_height) = parse_value::<LineHeight>("1.5", &allocator);
        let ast = compiler.ast();
        assert!(matches!(line_height.data(ast), LineHeightData::Number(1.5)));
    }

    #[test]
    fn parses_text_and_mask_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, transform) =
            parse_value::<TextTransform>("uppercase full-width", &allocator);
        let ast = compiler.ast();
        assert_eq!(transform.case(ast), TextTransformCase::Uppercase);
        assert!(transform.full_width(ast));
        assert!(!transform.full_size_kana(ast));

        let (compiler, line) = parse_value::<TextDecorationLine>("underline overline", &allocator);
        let ast = compiler.ast();
        let TextDecorationLineData::Value(values) = line.data(ast) else {
            panic!("expected decoration values")
        };
        assert_eq!(
            values,
            &[
                OtherTextDecorationLine::Underline,
                OtherTextDecorationLine::Overline
            ]
        );

        let (compiler, indent) = parse_value::<TextIndent>("2em hanging", &allocator);
        let ast = compiler.ast();
        assert!(indent.hanging(ast));
        assert!(!indent.each_line(ast));

        let (compiler, mask) = parse_value::<Mask>("none", &allocator);
        let ast = compiler.ast();
        assert!(matches!(mask.image(ast).data(ast), ImageData::None));
        assert_eq!(mask.repeat(ast).x(ast), BackgroundRepeatKeyword::Repeat);
        assert_eq!(mask.mode(ast), MaskMode::MatchSource);
        assert_eq!(
            mask.clip(ast),
            MaskClip::GeometryBox(GeometryBox::BorderBox)
        );
    }

    #[test]
    fn parses_transform_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, transform) = parse_value::<Transform>("translate(10px, 20px)", &allocator);
        let ast = compiler.ast();
        let TransformData::Translate((x, y)) = transform.data(ast) else {
            panic!("expected translate")
        };
        let DimensionPercentageData::Dimension(x) = x.data(ast) else {
            panic!("expected x dimension")
        };
        assert_eq!(x.value(ast), 10.0);
        let DimensionPercentageData::Dimension(y) = y.data(ast) else {
            panic!("expected y dimension")
        };
        assert_eq!(y.value(ast), 20.0);

        let (compiler, transform) = parse_value::<Transform>("rotate(45deg)", &allocator);
        let ast = compiler.ast();
        assert!(matches!(
            transform.data(ast),
            TransformData::Rotate(Angle::Deg(45.0))
        ));

        let (compiler, transform) =
            parse_value::<Transform>("matrix(1, 0, 0, 1, 0, 0)", &allocator);
        let ast = compiler.ast();
        let TransformData::Matrix(matrix) = transform.data(ast) else {
            panic!("expected matrix")
        };
        assert_eq!(matrix.a(ast), 1.0);
        assert_eq!(matrix.d(ast), 1.0);

        let (compiler, scale) = parse_value::<Scale>("2", &allocator);
        let _ = compiler.ast();
        let Scale::Xyz { x, y, z } = scale else {
            panic!("expected scale xyz")
        };
        assert_eq!(x, NumberOrPercentage::Number(2.0));
        assert_eq!(y, NumberOrPercentage::Number(2.0));
        assert_eq!(z, NumberOrPercentage::Number(1.0));
    }

    fn parse_selector_list<'i>(
        source: &'i str,
        allocator: &'i rocketcss_common::Allocator,
    ) -> (Compiler<'i>, &'i [Selector<'i>]) {
        let mut compiler = Compiler::new_with_source(source, allocator);
        let selectors = <&[Selector]>::parse(&mut compiler).unwrap();
        assert!(compiler.is_exhausted());
        (compiler, selectors)
    }

    fn components<'i>(selector: Selector<'i>, ast: &Ast<'i>) -> &'i [SelectorComponent<'i>] {
        match selector.data(ast) {
            SelectorData::Parsed(components) => components,
            other => panic!("expected parsed selector, got {other:?}"),
        }
    }

    #[test]
    fn parses_simple_and_compound_selectors() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) = parse_selector_list("a.b#c", &allocator);
        let ast = compiler.ast();
        assert_eq!(selectors.len(), 1);
        let parts = components(selectors[0], ast);
        assert_eq!(parts.len(), 3);
        let SelectorComponentData::LocalName { name, .. } = parts[0].data(ast) else {
            panic!("expected local name")
        };
        assert_eq!(name, "a");
        let SelectorComponentData::Class(name) = parts[1].data(ast) else {
            panic!("expected class")
        };
        assert_eq!(name, "b");
        let SelectorComponentData::Id(name) = parts[2].data(ast) else {
            panic!("expected id")
        };
        assert_eq!(name, "c");
    }

    #[test]
    fn parses_combinators_and_nesting() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) = parse_selector_list("a > b + c ~ d e", &allocator);
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        let combinators: Vec<_> = parts
            .iter()
            .filter_map(|part| match part.data(ast) {
                SelectorComponentData::Combinator(combinator) => Some(combinator),
                _ => None,
            })
            .collect();
        assert_eq!(
            combinators,
            &[
                Combinator::Child,
                Combinator::NextSibling,
                Combinator::LaterSibling,
                Combinator::Descendant
            ]
        );

        let (compiler, selectors) = parse_selector_list("& .a", &allocator);
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        assert!(matches!(parts[0].data(ast), SelectorComponentData::Nesting));
        assert_eq!(
            parts[1].data(ast),
            SelectorComponentData::Combinator(Combinator::Descendant)
        );
    }

    #[test]
    fn parses_functional_pseudo_classes() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) = parse_selector_list(":not(.a, #b)", &allocator);
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        let SelectorComponentData::Negation(inner) = parts[0].data(ast) else {
            panic!("expected negation")
        };
        assert_eq!(inner.len(), 2);

        let (compiler, selectors) = parse_selector_list(":nth-child(2n+1 of .a)", &allocator);
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        let SelectorComponentData::NthOf { data, selectors } = parts[0].data(ast) else {
            panic!("expected nth-of")
        };
        assert_eq!(data.kind(ast), NthType::Child);
        assert_eq!((data.a(ast), data.b(ast)), (2, 1));
        assert_eq!(selectors.len(), 1);

        let (compiler, selectors) = parse_selector_list(":is(.a):where(.b):has(> .c)", &allocator);
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        assert!(matches!(parts[0].data(ast), SelectorComponentData::Is(_)));
        assert!(matches!(
            parts[1].data(ast),
            SelectorComponentData::Where(_)
        ));
        assert!(matches!(parts[2].data(ast), SelectorComponentData::Has(_)));
    }

    #[test]
    fn parses_namespace_and_type_selectors() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) = parse_selector_list("svg|a, *|b, |c, *", &allocator);
        let ast = compiler.ast();

        let parts = components(selectors[0], ast);
        let SelectorComponentData::Namespace { prefix, url } = parts[0].data(ast) else {
            panic!("expected namespace")
        };
        assert_eq!(prefix, "svg");
        assert_eq!(url, "");
        let SelectorComponentData::LocalName { name, .. } = parts[1].data(ast) else {
            panic!("expected local name")
        };
        assert_eq!(name, "a");

        let parts = components(selectors[1], ast);
        assert!(matches!(
            parts[0].data(ast),
            SelectorComponentData::ExplicitAnyNamespace
        ));
        let SelectorComponentData::LocalName { name, .. } = parts[1].data(ast) else {
            panic!("expected local name")
        };
        assert_eq!(name, "b");

        let parts = components(selectors[2], ast);
        assert!(matches!(
            parts[0].data(ast),
            SelectorComponentData::ExplicitNoNamespace
        ));
        let SelectorComponentData::LocalName { name, .. } = parts[1].data(ast) else {
            panic!("expected local name")
        };
        assert_eq!(name, "c");

        let parts = components(selectors[3], ast);
        assert!(matches!(
            parts[0].data(ast),
            SelectorComponentData::ExplicitUniversalType
        ));
    }

    #[test]
    fn parses_attribute_selectors() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) = parse_selector_list(
            "[a][b=c][d~=e][f|=g][h^=i][j$=k][l*=m][n=o i][p=Q s]",
            &allocator,
        );
        let ast = compiler.ast();
        let parts = components(selectors[0], ast);
        let SelectorComponentData::AttributeInNoNamespaceExists { local_name, .. } =
            parts[0].data(ast)
        else {
            panic!("expected attribute exists selector")
        };
        assert_eq!(local_name, "a");
        let expected = [
            AttrSelectorOperator::Equal,
            AttrSelectorOperator::Includes,
            AttrSelectorOperator::DashMatch,
            AttrSelectorOperator::Prefix,
            AttrSelectorOperator::Suffix,
            AttrSelectorOperator::Substring,
        ];
        for (part, operator) in parts[1..7].iter().zip(expected) {
            let SelectorComponentData::AttributeInNoNamespace {
                operator: actual, ..
            } = part.data(ast)
            else {
                panic!("expected attribute selector")
            };
            assert_eq!(actual, operator);
        }
        let SelectorComponentData::AttributeInNoNamespace {
            case_sensitivity, ..
        } = parts[7].data(ast)
        else {
            panic!("expected attribute selector")
        };
        assert_eq!(
            case_sensitivity,
            ParsedCaseSensitivity::AsciiCaseInsensitive
        );
        let SelectorComponentData::AttributeInNoNamespace {
            case_sensitivity, ..
        } = parts[8].data(ast)
        else {
            panic!("expected attribute selector")
        };
        assert_eq!(
            case_sensitivity,
            ParsedCaseSensitivity::ExplicitCaseSensitive
        );
    }

    #[test]
    fn parses_pseudo_elements_and_custom_functions() {
        let allocator = rocketcss_common::Allocator::new();
        let (compiler, selectors) =
            parse_selector_list("a::before, b::placeholder, c:unknown-fn(x)", &allocator);
        let ast = compiler.ast();

        let parts = components(selectors[0], ast);
        let SelectorComponentData::PseudoElement(element) = parts[1].data(ast) else {
            panic!("expected pseudo element")
        };
        assert_eq!(element.data(ast), PseudoElementData::Before);

        let parts = components(selectors[1], ast);
        let SelectorComponentData::PseudoElement(element) = parts[1].data(ast) else {
            panic!("expected pseudo element")
        };
        assert_eq!(
            element.data(ast),
            PseudoElementData::Placeholder(VendorPrefix::NONE)
        );

        let parts = components(selectors[2], ast);
        let SelectorComponentData::PseudoClass(pseudo) = parts[1].data(ast) else {
            panic!("expected pseudo class")
        };
        let PseudoClassData::CustomFunction { name, arguments } = pseudo.data(ast) else {
            panic!("expected custom function")
        };
        assert_eq!(name, "unknown-fn");
        assert_eq!(arguments.len(), 1);
    }

    #[test]
    fn parses_stylesheet_rule_tree() {
        let allocator = rocketcss_common::Allocator::new();
        rocketcss_common::GhostToken::scope(|mut token| {
            let source = "a, .b { color: red; margin: 1px } @media screen { c { display: block } }";
            let compilation = Compiler::new(&allocator)
                .parse(source, &mut token, ParserOptions::default())
                .unwrap();
            let ast = compilation.ast();
            let rules = compilation.style_sheet().rules(ast);
            assert_eq!(rules.len(), 2);

            let payload = rules[0].payload(ast);
            let declarations = rules[0].declarations(ast);
            let CssRulePayloadData::Style(style) = payload.data(ast) else {
                panic!("expected style rule")
            };
            assert_eq!(style.selectors(ast).len(), 2);
            let block = declarations.expect("style rule owns declarations");
            let declarations = block.declarations(ast);
            assert_eq!(declarations.len(), 2);
            let entry = declarations[0];
            let DeclarationPayloadData::Property(declaration) = entry.data(ast) else {
                panic!("expected property declaration")
            };
            assert!(!block.important(ast)[0]);
            assert!(matches!(declaration.data(ast), DeclarationData::Color(_)));

            let payload = rules[1].payload(ast);
            let CssRulePayloadData::Media(media) = payload.data(ast) else {
                panic!("expected media rule")
            };
            assert_eq!(media.query(ast).media_queries(ast).len(), 1);
            assert_eq!(rules[1].children(ast).len(), 1);
            let child = rules[1].children(ast)[0];
            assert!(matches!(
                child.payload(ast).data(ast),
                CssRulePayloadData::Style(_)
            ));
        });
    }

    #[test]
    fn parses_nested_rules_and_declaration_segments() {
        let allocator = rocketcss_common::Allocator::new();
        rocketcss_common::GhostToken::scope(|mut token| {
            let source = "a { color: red; b { color: blue } margin: 1px }";
            let compilation = Compiler::new(&allocator)
                .parse(source, &mut token, ParserOptions::default())
                .unwrap();
            let ast = compilation.ast();
            let rule = compilation.style_sheet().rules(ast)[0];
            assert_eq!(rule.declarations(ast).unwrap().declarations(ast).len(), 1);
            let children = rule.children(ast);
            assert_eq!(children.len(), 2);
            assert!(matches!(
                children[0].payload(ast).data(ast),
                CssRulePayloadData::Style(_)
            ));
            let CssRulePayloadData::NestedDeclarations(_) = children[1].payload(ast).data(ast)
            else {
                panic!("expected nested declarations segment")
            };
            assert_eq!(
                children[1]
                    .declarations(ast)
                    .unwrap()
                    .declarations(ast)
                    .len(),
                1
            );
        });
    }

    #[test]
    fn parses_keyframes_page_and_property_rules() {
        let allocator = rocketcss_common::Allocator::new();
        rocketcss_common::GhostToken::scope(|mut token| {
            let source = "@keyframes spin { from { opacity: 0 } to { opacity: 1 } } \
                @page :first { margin: 1cm; @top-center { content: \"x\" } } \
                @property --x { syntax: \"<length>\"; inherits: false; initial-value: 0px }";
            let compilation = Compiler::new(&allocator)
                .parse(source, &mut token, ParserOptions::default())
                .unwrap();
            let ast = compilation.ast();
            let rules = compilation.style_sheet().rules(ast);
            assert_eq!(rules.len(), 3);

            let CssRulePayloadData::Keyframes(keyframes) = rules[0].payload(ast).data(ast) else {
                panic!("expected keyframes")
            };
            assert_eq!(keyframes.name(ast), KeyframesName::Ident("spin"));
            assert_eq!(rules[0].children(ast).len(), 2);

            let CssRulePayloadData::Page(page) = rules[1].payload(ast).data(ast) else {
                panic!("expected page")
            };
            assert_eq!(page.selectors(ast).len(), 1);
            assert_eq!(page.selectors(ast)[0].pseudo_classes(ast).len(), 1);
            assert_eq!(rules[1].children(ast).len(), 1);
            assert!(matches!(
                rules[1].children(ast)[0].payload(ast).data(ast),
                CssRulePayloadData::PageMargin(_)
            ));

            let CssRulePayloadData::Property(property) = rules[2].payload(ast).data(ast) else {
                panic!("expected property rule")
            };
            assert_eq!(property.name(ast), "--x");
            assert_eq!(property.inherits(ast), Some(false));
            assert!(property.syntax(ast).is_some());
            assert!(property.initial_value(ast).is_some());
        });
    }

    #[test]
    fn parses_alignment_and_animation_values() {
        let allocator = rocketcss_common::Allocator::new();

        let (compiler, place) = parse_value::<PlaceContent>("center", &allocator);
        let ast = compiler.ast();
        assert!(matches!(
            place.align(ast),
            AlignContent::ContentPosition {
                value: ContentPosition::Center,
                ..
            }
        ));
        assert!(matches!(
            place.justify(ast),
            JustifyContent::ContentPosition {
                value: ContentPosition::Center,
                ..
            }
        ));

        let (compiler, animation) = parse_value::<Animation>("1s ease-in slide", &allocator);
        let ast = compiler.ast();
        let components = animation.components(ast);
        assert_eq!(components.len(), 3);
        assert!(matches!(
            components[0].data(ast),
            AnimationComponentData::Duration(Time::Seconds(1.0))
        ));
        assert!(matches!(
            components[1].data(ast),
            AnimationComponentData::TimingFunction(EasingFunction::EaseIn)
        ));
        assert!(matches!(
            components[2].data(ast),
            AnimationComponentData::Name(AnimationName::Ident("slide"))
        ));

        let (compiler, transition) = parse_value::<Transition>("opacity 1s linear 2s", &allocator);
        let ast = compiler.ast();
        assert_eq!(transition.property(ast), PropertyId::Opacity);
        assert_eq!(transition.duration(ast), Time::Seconds(1.0));
        assert_eq!(transition.delay(ast), Time::Seconds(2.0));
        assert_eq!(transition.timing_function(ast), EasingFunction::Linear);
    }
}

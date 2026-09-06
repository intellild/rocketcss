use rocketcss_codegen::{Printer, PrinterOptions, ToCss, ToCssContext};
use rocketcss_common::GhostToken;
use rocketcss_parser::parse;
use rocketcss_parser::prelude::*;

fn parse_stylesheet<'a, 'ghost>(
    source: &'a str,
    allocator: &'a Allocator,
    token: &mut GhostToken<'ghost>,
) -> AstContext<'a> {
    parse(source, allocator, token, ParserOptions::default()).unwrap()
}

#[test]
fn stored_is_wrapper_checks_component_tags() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.intern("a");
        let class = ast.alloc_node(SelectorComponent::Class(name), DUMMY_SP);
        let tag = ast.alloc_node(
            SelectorComponent::LocalName {
                name,
                lower_name: name,
            },
            DUMMY_SP,
        );
        let universal = ast.alloc_node(SelectorComponent::ExplicitUniversalType, DUMMY_SP);
        let combinator = ast.alloc_node(
            SelectorComponent::Combinator(Combinator::Descendant),
            DUMMY_SP,
        );
        let attribute = ast.alloc_node(
            SelectorComponent::AttributeInNoNamespace {
                local_name: name,
                operator: AttrSelectorOperator::Equal,
                value: name,
                case_sensitivity: ParsedCaseSensitivity::ExplicitCaseSensitive,
                never_matches: false,
            },
            DUMMY_SP,
        );
        for (id, expected) in [
            (class, false),
            (tag, true),
            (universal, true),
            (combinator, true),
            (attribute, false),
        ] {
            assert_eq!(ast.selector_component_is_combinator_or_type(id), expected);
        }
        for (components, expected) in [
            (std::vec![class], ".a"),
            (std::vec![tag, class], ":is(a.a)"),
            (std::vec![universal, class], ":is(*.a)"),
            (std::vec![class, combinator, class], ":is(.a .a)"),
            (std::vec![attribute, class], "[a=a s].a"),
        ] {
            let components = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
                components, &allocator,
            ));
            let child = ast.alloc_node(Selector::Parsed(components), DUMMY_SP);
            let selectors = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
                [child],
                &allocator,
            ));
            let node = ast.alloc_node(SelectorComponent::Is(selectors), DUMMY_SP);
            let checkpoint = ast.node_checkpoint();
            for prettify in [false, true] {
                let expected = if prettify {
                    expected.replace("=a ", "=\"a\" ")
                } else {
                    expected.to_owned()
                };
                let cx = ToCssContext::with_ast(&token, &ast);
                for _ in 0..3 {
                    assert_eq!(
                        node.to_css_string(PrinterOptions { prettify }, &cx)
                            .unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_transform_preserves_all_variants_and_tail_order() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let x = ast.alloc_node(LengthPercentage::Percentage(0.25), DUMMY_SP);
        let y = ast.alloc_node(LengthPercentage::Percentage(0.75), DUMMY_SP);
        let z = ast.alloc_node(
            Length::Value(LengthValue {
                value: 3.0,
                unit: LengthUnit::Px,
            }),
            DUMMY_SP,
        );
        let n = NumberOrPercentage::Number(2.0);
        let p = NumberOrPercentage::Percentage(0.5);
        let matrix = ast.alloc_node(
            MatrixForFloat {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0,
            },
            DUMMY_SP,
        );
        let matrix3d = ast.alloc_node(
            Matrix3DForFloat {
                m11: 1.0,
                m12: 2.0,
                m13: 3.0,
                m14: 4.0,
                m21: 5.0,
                m22: 6.0,
                m23: 7.0,
                m24: 8.0,
                m31: 9.0,
                m32: 10.0,
                m33: 11.0,
                m34: 12.0,
                m41: 13.0,
                m42: 14.0,
                m43: 15.0,
                m44: 16.0,
            },
            DUMMY_SP,
        );
        let node = ast.alloc_node(Transform::TranslateX(x), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (value, expected) in [
            (Transform::Translate((x, y)), "translate(25%,75%)"),
            (Transform::TranslateX(x), "translateX(25%)"),
            (Transform::TranslateY(y), "translateY(75%)"),
            (Transform::TranslateZ(z), "translateZ(3px)"),
            (
                Transform::Translate3d((x, y, z)),
                "translate3d(25%,75%,3px)",
            ),
            (Transform::Scale((n, p)), "scale(2,50%)"),
            (Transform::ScaleX(n), "scaleX(2)"),
            (Transform::ScaleY(p), "scaleY(50%)"),
            (Transform::ScaleZ(n), "scaleZ(2)"),
            (Transform::Scale3d((p, n, p)), "scale3d(50%,2,50%)"),
            (Transform::Rotate(Angle::Deg(10.0)), "rotate(10deg)"),
            (Transform::RotateX(Angle::Rad(2.0)), "rotateX(2rad)"),
            (Transform::RotateY(Angle::Grad(30.0)), "rotateY(30grad)"),
            (Transform::RotateZ(Angle::Turn(0.25)), "rotateZ(.25turn)"),
            (
                Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Deg(40.0))),
                "rotate3d(1,2,3,40deg)",
            ),
            (
                Transform::Skew((Angle::Deg(10.0), Angle::Rad(2.0))),
                "skew(10deg,2rad)",
            ),
            (Transform::SkewX(Angle::Grad(30.0)), "skewX(30grad)"),
            (Transform::SkewY(Angle::Turn(0.25)), "skewY(.25turn)"),
            (Transform::Perspective(z), "perspective(3px)"),
            (Transform::Matrix(matrix), "matrix(1,2,3,4,5,6)"),
            (
                Transform::Matrix3d(matrix3d),
                "matrix3d(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16)",
            ),
        ] {
            for _ in 0..3 {
                ast.mutate_node(node, |stored, _| *stored = value);
                let cx = ToCssContext::with_ast(&token, &ast);
                for prettify in [false, true] {
                    let expected = if prettify {
                        expected.replace(',', ", ")
                    } else {
                        expected.to_owned()
                    };
                    let options = PrinterOptions { prettify };
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(value.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_gradient_preserves_variants_prefixes_and_item_ranges() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let x = ast.alloc_node(PositionComponent::Center, DUMMY_SP);
        let y = ast.alloc_node(PositionComponent::Center, DUMMY_SP);
        let position = ast.alloc_node(Position { x, y }, DUMMY_SP);
        let circle = ast.alloc_node(Circle::Extent(ShapeExtent::FarthestCorner), DUMMY_SP);
        let shape = ast.alloc_node(EndingShape::Circle(circle), DUMMY_SP);
        let length = ast.alloc_node(
            DimensionPercentage::<LengthValue>::Percentage(0.25),
            DUMMY_SP,
        );
        let hint = ast.alloc_node(GradientItem::Hint(length), DUMMY_SP);
        let mut values = allocator.vec();
        values.extend([hint, hint]);
        let items = ast.alloc_vec(values);
        let angle = ast.alloc_node(DimensionPercentage::<Angle>::Percentage(0.75), DUMMY_SP);
        let hint = ast.alloc_node(GradientItem::Hint(angle), DUMMY_SP);
        let mut values = allocator.vec();
        values.extend([hint, hint]);
        let angle_items = ast.alloc_vec(values);
        let point = ast.alloc_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Center,
                y: WebKitGradientPointComponent::Center,
            },
            DUMMY_SP,
        );
        let stops = ast.alloc_vec(allocator.vec::<WebKitColorStop<'_>>());
        let webkit = ast.alloc_node(
            WebKitGradient::Linear {
                from: point,
                to: point,
                stops,
            },
            DUMMY_SP,
        );
        let vendor_prefix = VendorPrefix::WEBKIT;
        let direction = LineDirection::Horizontal(HorizontalPositionKeyword::Right);
        let node = ast.alloc_node(Gradient::WebKitGradient(webkit), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (value, expected) in [
            (
                Gradient::Linear {
                    direction: LineDirection::Vertical(VerticalPositionKeyword::Bottom),
                    items,
                    vendor_prefix: VendorPrefix::empty(),
                },
                "linear-gradient(25%,25%)",
            ),
            (
                Gradient::RepeatingLinear {
                    direction,
                    items,
                    vendor_prefix,
                },
                "-webkit-repeating-linear-gradient(to right,25%,25%)",
            ),
            (
                Gradient::Radial {
                    position,
                    shape,
                    items,
                    vendor_prefix,
                },
                "-webkit-radial-gradient(circle farthest-corner at center,25%,25%)",
            ),
            (
                Gradient::RepeatingRadial {
                    position,
                    shape,
                    items,
                    vendor_prefix,
                },
                "-webkit-repeating-radial-gradient(circle farthest-corner at center,25%,25%)",
            ),
            (
                Gradient::Conic {
                    angle: Angle::Turn(0.25),
                    position,
                    items: angle_items,
                },
                "conic-gradient(from .25turn at center,75%,75%)",
            ),
            (
                Gradient::RepeatingConic {
                    angle: Angle::Grad(42.0),
                    position,
                    items: angle_items,
                },
                "repeating-conic-gradient(from 42grad at center,75%,75%)",
            ),
            (
                Gradient::WebKitGradient(webkit),
                "-webkit-gradient(linear, center center,center center)",
            ),
        ] {
            ast.mutate_node(node, |stored, _| *stored = value);
            let cx = ToCssContext::with_ast(&token, &ast);
            for prettify in [false, true] {
                let expected = if prettify {
                    expected.replace(',', ", ").replace("linear,  ", "linear, ")
                } else {
                    expected.to_owned()
                };
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(value.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_webkit_gradient_preserves_radii_and_stop_order() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let from = ast.alloc_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Center,
                y: WebKitGradientPointComponent::Center,
            },
            DUMMY_SP,
        );
        let to = ast.alloc_node(
            WebKitGradientPoint {
                x: WebKitGradientPointComponent::Side(HorizontalPositionKeyword::Right),
                y: WebKitGradientPointComponent::Side(VerticalPositionKeyword::Bottom),
            },
            DUMMY_SP,
        );
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let mut values = allocator.vec();
        for position in [1.0, 0.25, 0.0, 0.25] {
            values.push(WebKitColorStop { color, position });
        }
        let stops = ast.alloc_vec(values);
        let empty = ast.alloc_vec(allocator.vec::<WebKitColorStop<'_>>());
        let node = ast.alloc_node(WebKitGradient::Linear { from, to, stops }, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (radii, range) in [
            (None, stops),
            (Some([2.0, 7.0]), stops),
            (Some([0.0, 3.5]), empty),
            (None, empty),
            (None, stops),
        ] {
            ast.mutate_node(node, |value, _| {
                *value = match radii {
                    None => WebKitGradient::Linear {
                        from,
                        to,
                        stops: range,
                    },
                    Some([start_radius, end_radius]) => WebKitGradient::Radial {
                        from,
                        to,
                        start_radius,
                        end_radius,
                        stops: range,
                    },
                }
            });
            let view = ast.webkit_gradient(node);
            assert_eq!(view.radii(), radii);
            assert_eq!(view.stops(), range);
            let owned = ast.resolve_node(node);
            let cx = ToCssContext::with_ast(&token, &ast);
            for prettify in [false, true] {
                let comma = if prettify { ", " } else { "," };
                let geometry = match radii {
                    None => format!("linear, center center{comma}right bottom"),
                    Some([2.0, 7.0]) => {
                        format!("radial, center center{comma}2{comma}right bottom{comma}7")
                    }
                    Some(_) => {
                        format!("radial, center center{comma}0{comma}right bottom{comma}3.5")
                    }
                };
                let suffix = if range == empty {
                    String::new()
                } else {
                    format!(
                        "{comma}to(currentColor){comma}color-stop(.25{comma}currentColor){comma}from(currentColor){comma}color-stop(.25{comma}currentColor)"
                    )
                };
                let expected = format!("-webkit-gradient({geometry}{suffix})");
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(owned.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_easing_preserves_variants_coordinates_and_steps() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let node = ast.alloc_node(EasingFunction::Linear, DUMMY_SP);
        let mut cases = vec![
            (EasingFunction::Linear, "linear"),
            (EasingFunction::Ease, "ease"),
            (EasingFunction::EaseIn, "ease-in"),
            (EasingFunction::EaseOut, "ease-out"),
            (EasingFunction::EaseInOut, "ease-in-out"),
            (EasingFunction::Frames(i32::MIN), "frames(-2147483648)"),
            (EasingFunction::Frames(i32::MAX), "frames(2147483647)"),
        ];
        for (position, one, many) in [
            (StepPosition::Start, "step-start", "steps(3,start)"),
            (StepPosition::End, "step-end", "steps(3)"),
            (
                StepPosition::JumpNone,
                "steps(1,jump-none)",
                "steps(3,jump-none)",
            ),
            (
                StepPosition::JumpBoth,
                "steps(1,jump-both)",
                "steps(3,jump-both)",
            ),
        ] {
            cases.push((EasingFunction::Steps { count: 1, position }, one));
            cases.push((EasingFunction::Steps { count: 3, position }, many));
        }
        for ([x1, y1, x2, y2], expected) in [
            ([0.0, 0.0, 1.0, 1.0], "linear"),
            ([0.25, 0.1, 0.25, 1.0], "ease"),
            ([0.42, 0.0, 1.0, 1.0], "ease-in"),
            ([0.0, 0.0, 0.58, 1.0], "ease-out"),
            ([0.42, 0.0, 0.58, 1.0], "ease-in-out"),
            ([0.1, 0.2, 0.3, 0.4], "cubic-bezier(.1,.2,.3,.4)"),
        ] {
            cases.push((EasingFunction::CubicBezier { x1, y1, x2, y2 }, expected));
        }
        cases.push((EasingFunction::Ease, "ease"));
        for (value, expected) in cases {
            ast.mutate_node(node, |stored, _| *stored = value);
            let checkpoint = ast.node_checkpoint();
            for _ in 0..3 {
                ast.mutate_node(node, |stored, _| *stored = value);
                let cx = ToCssContext::with_ast(&token, &ast);
                for prettify in [false, true] {
                    let expected = if prettify {
                        expected.replace(',', ", ")
                    } else {
                        expected.to_owned()
                    };
                    let options = PrinterOptions { prettify };
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(value.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_mask_preserves_defaults_geometry_and_keyword_updates() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let image = ast.alloc_node(Image::None, DUMMY_SP);
        let zero = ast.alloc_node(LengthPercentage::Zero, DUMMY_SP);
        let x = ast.alloc_node(PositionComponent::Length(zero), DUMMY_SP);
        let y = ast.alloc_node(PositionComponent::Length(zero), DUMMY_SP);
        let position = ast.alloc_node(Position { x, y }, DUMMY_SP);
        let auto = ast.alloc_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let size = ast.alloc_node(
            BackgroundSize::Explicit {
                width: auto,
                height: auto,
            },
            DUMMY_SP,
        );
        let repeat = BackgroundRepeat {
            x: BackgroundRepeatKeyword::Repeat,
            y: BackgroundRepeatKeyword::Repeat,
        };
        let node = ast.alloc_node(
            Mask {
                image,
                position,
                size,
                repeat,
                clip: MaskClip::GeometryBox(GeometryBox::BorderBox),
                origin: GeometryBox::BorderBox,
                composite: MaskComposite::Add,
                mode: MaskMode::MatchSource,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for prettify in [false, true] {
            let cx = ToCssContext::with_ast(&token, &ast);
            assert_eq!(
                node.to_css_string(PrinterOptions { prettify }, &cx)
                    .unwrap(),
                "none"
            );
        }
        ast.mutate_node(x, |value, _| {
            *value = PositionComponent::Side {
                side: HorizontalPositionKeyword::Right,
                offset: None,
            }
        });
        ast.mutate_node(y, |value, _| {
            *value = PositionComponent::Side {
                side: VerticalPositionKeyword::Bottom,
                offset: None,
            }
        });
        ast.mutate_node(size, |value, _| *value = BackgroundSize::Cover);
        for (mode, suffix) in [
            (MaskMode::MatchSource, ""),
            (MaskMode::Alpha, " alpha"),
            (MaskMode::Luminance, " luminance"),
        ] {
            ast.mutate_node(node, |value, _| {
                value.mode = mode;
                value.clip = MaskClip::NoClip;
                value.origin = GeometryBox::PaddingBox;
                value.composite = MaskComposite::Exclude;
                value.repeat = BackgroundRepeat {
                    x: BackgroundRepeatKeyword::NoRepeat,
                    y: BackgroundRepeatKeyword::NoRepeat,
                };
            });
            let owned = ast.resolve_node(node);
            let view = ast.mask(node);
            assert_eq!(view.image(), image);
            assert_eq!(view.position(), position);
            assert_eq!(view.size(), size);
            let keywords = view.keywords();
            assert_eq!(keywords.clip(), owned.clip);
            assert_eq!(keywords.origin(), owned.origin);
            assert_eq!(keywords.repeat(), owned.repeat);
            assert_eq!(keywords.composite(), owned.composite);
            assert_eq!(keywords.mode(), mode);
            let cx = ToCssContext::with_ast(&token, &ast);
            for (prettify, geometry) in [(false, "100% 100%/cover"), (true, "right bottom / cover")]
            {
                let expected =
                    format!("none {geometry} no-repeat padding-box no-clip exclude{suffix}");
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(owned.to_css_string(options, &cx).unwrap(), expected);
                }
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    });
}

#[test]
fn stored_border_images_preserve_fields_repeat_and_mask_modes() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let [a, b, c, d] =
            [1.0, 2.0, 3.0, 4.0].map(|n| ast.alloc_node(NumberOrPercentage::Number(n), DUMMY_SP));
        let offsets = ast.alloc_node(Rect(a, b, c, d), DUMMY_SP);
        let slice = ast.alloc_node(
            BorderImageSlice {
                fill: false,
                offsets,
            },
            DUMMY_SP,
        );
        let [a, b, c, d] =
            [5.0, 6.0, 7.0, 8.0].map(|n| ast.alloc_node(LengthOrNumber::Number(n), DUMMY_SP));
        let outset = ast.alloc_node(Rect(a, b, c, d), DUMMY_SP);
        let auto = ast.alloc_node(BorderImageSideWidth::Auto, DUMMY_SP);
        let number = ast.alloc_node(BorderImageSideWidth::Number(2.0), DUMMY_SP);
        let auto_width = ast.alloc_node(Rect(auto, auto, auto, auto), DUMMY_SP);
        let mixed_width = ast.alloc_node(Rect(auto, number, auto, number), DUMMY_SP);
        let source = ast.alloc_node(Image::None, DUMMY_SP);
        let node = ast.alloc_node(
            BorderImage {
                source,
                slice,
                width: auto_width,
                outset,
                repeat: BorderImageRepeat {
                    horizontal: BorderImageRepeatKeyword::Stretch,
                    vertical: BorderImageRepeatKeyword::Stretch,
                },
            },
            DUMMY_SP,
        );
        let mask = ast.alloc_node(
            MaskBorder {
                mode: MaskBorderMode::Alpha,
                source,
                slice,
                width: auto_width,
                outset,
                repeat: BorderImageRepeat {
                    horizontal: BorderImageRepeatKeyword::Stretch,
                    vertical: BorderImageRepeatKeyword::Stretch,
                },
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for (repeat, fill, width, expected) in [
            (
                BorderImageRepeatKeyword::Stretch,
                false,
                auto_width,
                "none 1 2 3 4 / auto / 5 6 7 8 stretch",
            ),
            (
                BorderImageRepeatKeyword::Round,
                true,
                mixed_width,
                "none 1 2 3 4 fill / auto 2 / 5 6 7 8 round stretch",
            ),
            (
                BorderImageRepeatKeyword::Space,
                false,
                auto_width,
                "none 1 2 3 4 / auto / 5 6 7 8 space stretch",
            ),
            (
                BorderImageRepeatKeyword::Repeat,
                true,
                mixed_width,
                "none 1 2 3 4 fill / auto 2 / 5 6 7 8 repeat stretch",
            ),
        ] {
            ast.mutate_node(node, |value, _| {
                value.repeat.horizontal = repeat;
                value.width = width;
            });
            ast.mutate_node(slice, |value, _| value.fill = fill);
            let owned = ast.resolve_node(node);
            let cx = ToCssContext::with_ast(&token, &ast);
            for prettify in [false, true] {
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(node.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(owned.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            for (mode, suffix) in [
                (MaskBorderMode::Alpha, "alpha"),
                (MaskBorderMode::Luminance, "luminance"),
            ] {
                ast.mutate_node(mask, |value, _| {
                    value.mode = mode;
                    value.repeat.horizontal = repeat;
                    value.width = width;
                });
                let owned_mask = ast.resolve_node(mask);
                let cx = ToCssContext::with_ast(&token, &ast);
                let expected = format!("{expected} {suffix}");
                for prettify in [false, true] {
                    let options = PrinterOptions { prettify };
                    for _ in 0..3 {
                        assert_eq!(mask.to_css_string(options, &cx).unwrap(), expected);
                        assert_eq!(owned_mask.to_css_string(options, &cx).unwrap(), expected);
                    }
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn stored_track_sizing_preserves_none_names_only_and_trailing_groups() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::with_source_in(&allocator, "edge edge", Default::default());
        let first = ast.string_pool().source_range(0, 4);
        let second = ast.string_pool().source_range(5, 9);
        assert_ne!(first, second);
        let tail = ast.add_str("tail");
        let begin = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [first, second],
            &allocator,
        ));
        let end = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in([tail], &allocator));
        let empty_names = ast.alloc_vec(allocator.vec::<AstStr<'_>>());
        let names = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [begin, empty_names, end],
            &allocator,
        ));
        let names_only = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [begin],
            &allocator,
        ));
        let no_names = ast.alloc_vec(allocator.vec::<AstVec<'_, AstStr<'_>>>());
        let items = [1.0, 2.0].map(|value| {
            let breadth = ast.alloc_node(TrackBreadth::Flex(value), DUMMY_SP);
            let track = ast.alloc_node(TrackSize::TrackBreadth(breadth), DUMMY_SP);
            ast.alloc_node(TrackListItem::TrackSize(track), DUMMY_SP)
        });
        let items = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(items, &allocator));
        let no_items = ast.alloc_vec(allocator.vec::<NodeId<'_, TrackListItem<'_>>>());
        let id = ast.alloc_node(TrackSizing::None, DUMMY_SP);
        let pool_len = ast.string_pool().extra_len();
        for (fields, expected) in [
            (None, "none"),
            (Some((items, names)), "[edge edge] 1fr 2fr [tail]"),
            (Some((no_items, names_only)), "[edge edge]"),
            (Some((items, no_names)), "1fr 2fr"),
            (Some((no_items, no_names)), ""),
            (None, "none"),
            (Some((items, names)), "[edge edge] 1fr 2fr [tail]"),
        ] {
            let make_value = || match fields {
                None => TrackSizing::None,
                Some((items, line_names)) => TrackSizing::TrackList { items, line_names },
            };
            ast.mutate_node(id, |value, _| *value = make_value());
            let checkpoint = ast.node_checkpoint();
            ast.mutate_node(id, |value, _| *value = make_value());
            match (ast.track_sizing(id), fields) {
                (None, None) => {}
                (Some(view), Some((items, names))) => {
                    assert_eq!(view.items(), items);
                    assert_eq!(view.line_names(), names);
                }
                _ => panic!("track sizing view changed its variant"),
            }
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), pool_len);
        }
    });
}

#[test]
fn stored_grid_fields_preserve_auto_tracks_flow_and_areas() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let no_names = ast.alloc_vec(allocator.vec::<AstVec<'_, AstStr<'_>>>());
        let pairs = [1.0, 2.0].map(|value| {
            let breadth = ast.alloc_node(TrackBreadth::Flex(value), DUMMY_SP);
            let track = ast.alloc_node(TrackSize::TrackBreadth(breadth), DUMMY_SP);
            let item = ast.alloc_node(TrackListItem::TrackSize(track), DUMMY_SP);
            let items = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in([item], &allocator));
            let sizing = ast.alloc_node(
                TrackSizing::TrackList {
                    items,
                    line_names: no_names,
                },
                DUMMY_SP,
            );
            let tracks = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
                [track],
                &allocator,
            ));
            (sizing, tracks)
        });
        let [(one, auto_one), (two, auto_two)] = pairs;
        let empty = ast.alloc_vec(allocator.vec::<NodeId<'_, TrackSize<'_>>>());
        let none = ast.alloc_node(GridTemplateAreas::None, DUMMY_SP);
        let name = ast.add_str("zone");
        let names = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [Some(name)],
            &allocator,
        ));
        let areas = ast.alloc_node(
            GridTemplateAreas::Areas {
                areas: names,
                columns: 1,
            },
            DUMMY_SP,
        );
        let id = ast.alloc_node(
            Grid {
                rows: one,
                columns: two,
                areas: none,
                auto_rows: empty,
                auto_columns: empty,
                auto_flow: GridAutoFlow {
                    direction: AutoFlowDirection::Row,
                    dense: false,
                },
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let pool_len = ast.string_pool().extra_len();
        for (direction, dense, auto_rows, auto_columns, areas, expected) in [
            (
                AutoFlowDirection::Row,
                false,
                empty,
                empty,
                none,
                "1fr / 2fr auto-flow row",
            ),
            (
                AutoFlowDirection::Column,
                true,
                auto_one,
                auto_two,
                areas,
                "1fr / 2fr auto-flow column dense 1fr / 2fr \"zone\"",
            ),
            (
                AutoFlowDirection::Row,
                true,
                empty,
                auto_one,
                none,
                "1fr / 2fr auto-flow row dense / 1fr",
            ),
            (
                AutoFlowDirection::Column,
                false,
                auto_two,
                empty,
                areas,
                "1fr / 2fr auto-flow column 2fr \"zone\"",
            ),
        ] {
            ast.mutate_node(id, |grid, _| {
                grid.auto_flow = GridAutoFlow { direction, dense };
                grid.auto_rows = auto_rows;
                grid.auto_columns = auto_columns;
                grid.areas = areas;
            });
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), pool_len);
        }
    });
}

#[test]
fn stored_track_repeat_preserves_nested_line_names_and_trailing_names() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::with_source_in(&allocator, "edge edge", Default::default());
        let first = ast.string_pool().source_range(0, 4);
        let second = ast.string_pool().source_range(5, 9);
        assert_ne!(first, second);
        let escaped = ast.add_str("a:b");
        let begin = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [first, second],
            &allocator,
        ));
        let end = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [escaped],
            &allocator,
        ));
        let empty_names = ast.alloc_vec(allocator.vec::<AstStr<'_>>());
        let names = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [begin, empty_names, end],
            &allocator,
        ));
        let sparse = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [empty_names, begin, empty_names],
            &allocator,
        ));
        let no_names = ast.alloc_vec(allocator.vec::<AstVec<'_, AstStr<'_>>>());
        let tracks = [1.0, 2.0].map(|value| {
            let breadth = ast.alloc_node(TrackBreadth::Flex(value), DUMMY_SP);
            ast.alloc_node(TrackSize::TrackBreadth(breadth), DUMMY_SP)
        });
        let tracks = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(tracks, &allocator));
        let no_tracks = ast.alloc_vec(allocator.vec::<NodeId<'_, TrackSize<'_>>>());
        let id = ast.alloc_node(
            TrackRepeat {
                count: RepeatCount::Number(2.0),
                line_names: names,
                track_sizes: tracks,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let pool_len = ast.string_pool().extra_len();
        for (count, line_names, track_sizes, compact, pretty) in [
            (
                RepeatCount::Number(2.0),
                names,
                tracks,
                r"repeat(2,[edge edge] 1fr 2fr [a\:b])",
                r"repeat(2, [edge edge] 1fr 2fr [a\:b])",
            ),
            (
                RepeatCount::AutoFill,
                sparse,
                tracks,
                "repeat(auto-fill,1fr [edge edge] 2fr)",
                "repeat(auto-fill, 1fr [edge edge] 2fr)",
            ),
            (
                RepeatCount::AutoFit,
                no_names,
                no_tracks,
                "repeat(auto-fit,)",
                "repeat(auto-fit, )",
            ),
        ] {
            ast.mutate_node(id, |value, _| {
                *value = TrackRepeat {
                    count,
                    line_names,
                    track_sizes,
                }
            });
            let cx = ToCssContext::with_ast(&token, &ast);
            for (prettify, expected) in [(false, compact), (true, pretty)] {
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), pool_len);
        }
    });
}

#[test]
fn stored_shadow_fields_preserve_order_color_and_inset() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let lengths = [1.0, 2.0, 3.0, 4.0].map(|value| {
            ast.alloc_node(
                Length::Value(LengthValue {
                    value,
                    unit: LengthUnit::Px,
                }),
                DUMMY_SP,
            )
        });
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let canvas = ast.alloc_node(CssColor::System(SystemColor::Canvas), DUMMY_SP);
        let [x_offset, y_offset, blur, spread] = lengths;
        let box_shadow = ast.alloc_node(
            BoxShadow {
                x_offset,
                y_offset,
                blur,
                spread,
                color,
                inset: false,
            },
            DUMMY_SP,
        );
        let text_shadow = ast.alloc_node(
            TextShadow {
                x_offset,
                y_offset,
                blur,
                spread,
                color,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for (offsets, color, inset, expected) in [
            (lengths, color, false, "1px 2px 3px 4px currentColor"),
            (
                [spread, blur, y_offset, x_offset],
                canvas,
                true,
                "4px 3px 2px 1px Canvas",
            ),
            (lengths, color, false, "1px 2px 3px 4px currentColor"),
        ] {
            let [x_offset, y_offset, blur, spread] = offsets;
            ast.mutate_node(box_shadow, |stored, _| {
                *stored = BoxShadow {
                    x_offset,
                    y_offset,
                    blur,
                    spread,
                    color,
                    inset,
                }
            });
            ast.mutate_node(text_shadow, |stored, _| {
                *stored = TextShadow {
                    x_offset,
                    y_offset,
                    blur,
                    spread,
                    color,
                }
            });
            assert_eq!(ast.box_shadow(box_shadow).offsets(), offsets);
            assert_eq!(ast.text_shadow(text_shadow).offsets(), offsets);
            assert_eq!(ast.box_shadow(box_shadow).inset(), inset);
            assert_eq!(ast.box_shadow(box_shadow).color(), color);
            assert_eq!(ast.text_shadow(text_shadow).color(), color);
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                let options = PrinterOptions { prettify };
                let expected_box = format!("{}{expected}", if inset { "inset " } else { "" });
                for _ in 0..3 {
                    assert_eq!(
                        box_shadow.to_css_string(options, &cx).unwrap(),
                        expected_box
                    );
                    assert_eq!(
                        ast.resolve_node(box_shadow)
                            .to_css_string(options, &cx)
                            .unwrap(),
                        expected_box
                    );
                    assert_eq!(text_shadow.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(text_shadow)
                            .to_css_string(options, &cx)
                            .unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), 0);
        }
    });
}

#[test]
fn stored_font_fields_preserve_order_stretch_and_family_ranges() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let serif = ast.alloc_node(FontFamily::Serif, DUMMY_SP);
        let mono = ast.alloc_node(FontFamily::Monospace, DUMMY_SP);
        let family = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [serif, mono],
            &allocator,
        ));
        let reversed = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [mono, serif],
            &allocator,
        ));
        let empty = ast.alloc_vec(allocator.vec::<NodeId<'_, FontFamily<'_>>>());
        let style = ast.alloc_node(FontStyle::Italic, DUMMY_SP);
        let weight = ast.alloc_node(FontWeight::Absolute(AbsoluteFontWeight::Bold), DUMMY_SP);
        let size = ast.alloc_node(FontSize::Absolute(AbsoluteFontSize::Medium), DUMMY_SP);
        let line_height = ast.alloc_node(LineHeight::Number(1.5), DUMMY_SP);
        let id = ast.alloc_node(
            Font {
                family,
                line_height,
                size,
                style,
                weight,
                stretch: FontStretch::Keyword(FontStretchKeyword::Condensed),
                variant_caps: FontVariantCaps::SmallCaps,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        for (families, stretch, stretch_text, family_text) in [
            (
                family,
                FontStretch::Keyword(FontStretchKeyword::Condensed),
                "condensed",
                "serif,monospace",
            ),
            (
                reversed,
                FontStretch::Percentage(0.75),
                "75%",
                "monospace,serif",
            ),
            (
                empty,
                FontStretch::Keyword(FontStretchKeyword::Normal),
                "normal",
                "",
            ),
            (
                family,
                FontStretch::Percentage(1.25),
                "125%",
                "serif,monospace",
            ),
        ] {
            ast.mutate_node(id, |font, _| {
                font.family = families;
                font.stretch = stretch;
            });
            let cx = ToCssContext::with_ast(&token, &ast);
            for prettify in [false, true] {
                let family_text = if prettify {
                    family_text.replace(',', ", ")
                } else {
                    family_text.to_owned()
                };
                let expected =
                    format!("italic small-caps bold {stretch_text} medium / 1.5 {family_text}");
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), 0);
        }
    });
}

#[test]
fn position_writers_preserve_explicit_offsets_and_center_omission() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let zero = ast.alloc_node(LengthPercentage::Zero, DUMMY_SP);
        for (x, y, expected) in [
            (
                PositionComponent::Center,
                PositionComponent::Center,
                "center",
            ),
            (
                PositionComponent::Length(zero),
                PositionComponent::Center,
                "0",
            ),
            (
                PositionComponent::Center,
                PositionComponent::Length(zero),
                "center 0",
            ),
            (
                PositionComponent::Side {
                    side: HorizontalPositionKeyword::Left,
                    offset: None,
                },
                PositionComponent::Center,
                "left",
            ),
            (
                PositionComponent::Side {
                    side: HorizontalPositionKeyword::Left,
                    offset: Some(zero),
                },
                PositionComponent::Center,
                "left 0 center",
            ),
            (
                PositionComponent::Center,
                PositionComponent::Side {
                    side: VerticalPositionKeyword::Top,
                    offset: Some(zero),
                },
                "center top 0",
            ),
        ] {
            let x = ast.alloc_node(x, DUMMY_SP);
            let y = ast.alloc_node(y, DUMMY_SP);
            let position = ast.alloc_node(Position { x, y }, DUMMY_SP);
            let background = ast.alloc_node(BackgroundPosition { x, y }, DUMMY_SP);
            let checkpoint = ast.node_checkpoint();
            let cx = ToCssContext::with_ast(&token, &ast);
            for prettify in [false, true] {
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(position.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(background.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn background_color_shortcut_preserves_each_guard_and_full_output() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let image = ast.alloc_node(Image::None, DUMMY_SP);
        let zero = ast.alloc_node(LengthPercentage::Zero, DUMMY_SP);
        let x = ast.alloc_node(
            PositionComponent::<HorizontalPositionKeyword>::Length(zero),
            DUMMY_SP,
        );
        let y = ast.alloc_node(
            PositionComponent::<VerticalPositionKeyword>::Length(zero),
            DUMMY_SP,
        );
        let position = ast.alloc_node(BackgroundPosition { x, y }, DUMMY_SP);
        let auto = ast.alloc_node(LengthPercentageOrAuto::Auto, DUMMY_SP);
        let size = ast.alloc_node(
            BackgroundSize::Explicit {
                width: auto,
                height: auto,
            },
            DUMMY_SP,
        );
        let cover = ast.alloc_node(BackgroundSize::Cover, DUMMY_SP);
        let center_x = ast.alloc_node(
            PositionComponent::<HorizontalPositionKeyword>::Center,
            DUMMY_SP,
        );
        let centered = ast.alloc_node(BackgroundPosition { x: center_x, y }, DUMMY_SP);
        let url_text = ast.add_str("a.png");
        let url = ast.alloc_node(Url { url: url_text }, DUMMY_SP);
        let url_image = ast.alloc_node(Image::Url(url), DUMMY_SP);
        let base = || Background {
            color,
            image,
            position,
            size,
            attachment: BackgroundAttachment::Scroll,
            clip: BackgroundClip::BorderBox,
            origin: BackgroundOrigin::PaddingBox,
            repeat: BackgroundRepeat {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
        };
        let id = ast.alloc_node(base(), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (case, expected) in [
            (0, "currentColor"),
            (
                1,
                "none 0 0 / auto auto repeat-x scroll padding-box border-box currentColor",
            ),
            (
                2,
                "none 0 0 / auto auto repeat fixed padding-box border-box currentColor",
            ),
            (
                3,
                "none 0 0 / auto auto repeat scroll content-box border-box currentColor",
            ),
            (
                4,
                "none 0 0 / auto auto repeat scroll padding-box currentColor",
            ),
            (
                5,
                "none 0 0 / cover repeat scroll padding-box border-box currentColor",
            ),
            (
                6,
                "none center 0 / auto auto repeat scroll padding-box border-box currentColor",
            ),
            (
                7,
                "url(a.png) 0 0 / auto auto repeat scroll padding-box border-box currentColor",
            ),
        ] {
            let mut value = base();
            match case {
                0 => {}
                1 => value.repeat.y = BackgroundRepeatKeyword::NoRepeat,
                2 => value.attachment = BackgroundAttachment::Fixed,
                3 => value.origin = BackgroundOrigin::ContentBox,
                4 => value.clip = BackgroundClip::PaddingBox,
                5 => value.size = cover,
                6 => value.position = centered,
                7 => value.image = url_image,
                _ => unreachable!(),
            }
            ast.mutate_node(id, |stored, _| *stored = value);
            let owned = ast.resolve_node(id);
            let view = ast.background(id);
            assert_eq!(view.image(), owned.image);
            assert_eq!(view.color(), owned.color);
            assert_eq!(view.position(), owned.position);
            assert_eq!(view.size(), owned.size);
            let keywords = view.keywords();
            assert_eq!(keywords.repeat(), owned.repeat);
            assert_eq!(keywords.attachment(), owned.attachment);
            assert_eq!(keywords.origin(), owned.origin);
            assert_eq!(keywords.clip(), owned.clip);
            for prettify in [false, true] {
                let expected = if prettify {
                    expected.replace("url(a.png)", "url(\"a.png\")")
                } else {
                    expected.to_owned()
                };
                let cx = ToCssContext::with_ast(&token, &ast);
                for _ in 0..2 {
                    assert_eq!(
                        owned
                            .to_css_string(PrinterOptions { prettify }, &cx)
                            .unwrap(),
                        expected
                    );
                    assert_eq!(
                        id.to_css_string(PrinterOptions { prettify }, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    });
}

#[test]
fn animation_streaming_preserves_keyword_collision_order() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let linear = ast.alloc_node(EasingFunction::Linear, DUMMY_SP);
        for (text, matching) in [
            ("linear", AnimationComponent::TimingFunction(linear)),
            (
                "infinite",
                AnimationComponent::IterationCount(AnimationIterationCount::Infinite),
            ),
            (
                "reverse",
                AnimationComponent::Direction(AnimationDirection::Reverse),
            ),
            (
                "both",
                AnimationComponent::FillMode(AnimationFillMode::Both),
            ),
            (
                "paused",
                AnimationComponent::PlayState(AnimationPlayState::Paused),
            ),
        ] {
            let range = ast.add_str(text);
            let name =
                AnimationComponent::Name(ast.alloc_node(AnimationName::String(range), DUMMY_SP));
            let duration = AnimationComponent::Duration(Time::Seconds(1.0));
            let (other, other_text) = if text == "reverse" {
                (AnimationComponent::TimingFunction(linear), "linear")
            } else {
                (
                    AnimationComponent::Direction(AnimationDirection::Reverse),
                    "reverse",
                )
            };
            for (components, expected) in [
                (std::vec![other, name], format!("{other_text} \"{text}\"")),
                (
                    std::vec![name, duration, matching],
                    format!("\"{text}\" 1s {text}"),
                ),
                (
                    std::vec![matching, duration, name],
                    format!("{text} 1s {text}"),
                ),
                (
                    std::vec![name, matching, name],
                    format!("\"{text}\" {text} {text}"),
                ),
                (std::vec![duration, name], format!("1s \"{text}\"")),
            ] {
                let components = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
                    components, &allocator,
                ));
                let animation = Animation { components };
                let checkpoint = ast.node_checkpoint();
                let pool_len = ast.string_pool().extra_len();
                for prettify in [false, true] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    for _ in 0..3 {
                        assert_eq!(
                            animation
                                .to_css_string(PrinterOptions { prettify }, &cx)
                                .unwrap(),
                            expected
                        );
                    }
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), pool_len);
            }
        }
    });
}

#[test]
fn stored_transitions_preserve_time_and_easing_output() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let property = ast.alloc_node(PropertyId::Opacity, DUMMY_SP);
        let timing_function = ast.alloc_node(EasingFunction::Ease, DUMMY_SP);
        let id = ast.alloc_node(
            Transition {
                property,
                timing_function,
                duration: Time::Seconds(1.0),
                delay: Time::Seconds(0.0),
            },
            DUMMY_SP,
        );
        for (easing, compact_easing, pretty_easing) in [
            (EasingFunction::Ease, "", ""),
            (EasingFunction::Linear, " linear", " linear"),
            (
                EasingFunction::CubicBezier {
                    x1: 0.1,
                    y1: 0.2,
                    x2: 0.3,
                    y2: 0.4,
                },
                " cubic-bezier(.1,.2,.3,.4)",
                " cubic-bezier(.1, .2, .3, .4)",
            ),
            (
                EasingFunction::CubicBezier {
                    x1: 0.25,
                    y1: 0.1,
                    x2: 0.25,
                    y2: 1.0,
                },
                " ease",
                " ease",
            ),
            (
                EasingFunction::Steps {
                    count: 1,
                    position: StepPosition::Start,
                },
                " step-start",
                " step-start",
            ),
        ] {
            ast.mutate_node(timing_function, |stored, _| *stored = easing);
            for (duration, delay, duration_text, delay_text) in [
                (Time::Seconds(1.0), Time::Seconds(0.0), "1s", ""),
                (Time::Milliseconds(1.0), Time::Milliseconds(-0.0), "1ms", ""),
                (
                    Time::Milliseconds(1000.0),
                    Time::Seconds(-0.5),
                    "1s",
                    " -.5s",
                ),
                (Time::Seconds(0.5), Time::Milliseconds(1.0), ".5s", " 1ms"),
            ] {
                ast.mutate_node(id, |stored, _| {
                    stored.duration = duration;
                    stored.delay = delay;
                });
                let checkpoint = ast.node_checkpoint();
                for (prettify, easing_text) in [(false, compact_easing), (true, pretty_easing)] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    let options = PrinterOptions { prettify };
                    let expected = format!("opacity {duration_text}{easing_text}{delay_text}");
                    for _ in 0..3 {
                        assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                        assert_eq!(
                            ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                            expected
                        );
                    }
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), 0);
            }
        }
    });
}

#[test]
fn stored_query_features_preserve_names_predicates_and_operators() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let custom = ast.add_str("--example");
        let unknown = ast.add_str("unusual");
        let start = ast.alloc_node(MediaFeatureValue::Integer(1), DUMMY_SP);
        let end = ast.alloc_node(MediaFeatureValue::Integer(2), DUMMY_SP);
        macro_rules! check {
            ($feature:expr, $keyword:expr) => {{
                let id = ast.alloc_node(
                    QueryFeature::Boolean {
                        name: MediaFeatureName::Standard($feature),
                    },
                    DUMMY_SP,
                );
                for (name, text) in [
                    (MediaFeatureName::Standard($feature), $keyword),
                    (MediaFeatureName::Custom(custom), "--example"),
                    (MediaFeatureName::Unknown(unknown), "unusual"),
                ] {
                    for (operator, symbol) in [
                        (MediaFeatureComparison::Equal, "="),
                        (MediaFeatureComparison::GreaterThan, ">"),
                        (MediaFeatureComparison::GreaterThanEqual, ">="),
                        (MediaFeatureComparison::LessThan, "<"),
                        (MediaFeatureComparison::LessThanEqual, "<="),
                    ] {
                        for (end_operator, end_symbol) in [
                            (MediaFeatureComparison::Equal, "="),
                            (MediaFeatureComparison::GreaterThan, ">"),
                            (MediaFeatureComparison::GreaterThanEqual, ">="),
                            (MediaFeatureComparison::LessThan, "<"),
                            (MediaFeatureComparison::LessThanEqual, "<="),
                        ] {
                            for (value, compact, pretty) in [
                                (
                                    QueryFeature::Boolean { name },
                                    format!("({text})"),
                                    format!("({text})"),
                                ),
                                (
                                    QueryFeature::Plain { name, value: start },
                                    format!("({text}:1)"),
                                    format!("({text}: 1)"),
                                ),
                                (
                                    QueryFeature::Range {
                                        name,
                                        operator,
                                        value: end,
                                    },
                                    format!("({text}{symbol}2)"),
                                    format!("({text} {symbol} 2)"),
                                ),
                                (
                                    QueryFeature::Interval {
                                        name,
                                        start,
                                        start_operator: operator,
                                        end,
                                        end_operator,
                                    },
                                    format!("(1{symbol}{text}{end_symbol}2)"),
                                    format!("(1 {symbol} {text} {end_symbol} 2)"),
                                ),
                            ] {
                                ast.mutate_node(id, |stored, _| *stored = value);
                                let checkpoint = ast.node_checkpoint();
                                ast.mutate_node(id, |stored, _| *stored = value);
                                for (prettify, expected) in [(false, &compact), (true, &pretty)] {
                                    let cx = ToCssContext::with_ast(&token, &ast);
                                    let options = PrinterOptions { prettify };
                                    for _ in 0..2 {
                                        assert_eq!(
                                            &id.to_css_string(options, &cx).unwrap(),
                                            expected
                                        );
                                        assert_eq!(
                                            &ast.resolve_node(id)
                                                .to_css_string(options, &cx)
                                                .unwrap(),
                                            expected
                                        );
                                    }
                                }
                                assert_eq!(ast.node_checkpoint(), checkpoint);
                            }
                        }
                    }
                }
            }};
        }
        let pool_len = ast.string_pool().extra_len();
        check!(MediaFeatureId::Width, "width");
        check!(ContainerSizeFeatureId::InlineSize, "inline-size");
        check!(ScrollStateFeatureId::Scrollable, "scrollable");
        assert_eq!(ast.string_pool().extra_len(), pool_len);
    });
}

#[test]
fn stored_unresolved_colors_preserve_lists_after_variant_changes() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let mut lists = std::vec::Vec::new();
        for number in [0.5, 1.0, 2.0] {
            let value = ast.alloc_node(rocketcss_ast::Token::Number(number), DUMMY_SP);
            lists.push(ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
                [TokenOrValue::Token(value)],
                &allocator,
            )));
        }
        let [alpha, light, dark] = lists.try_into().unwrap();
        let empty = ast.alloc_vec(allocator.vec::<TokenOrValue<'_>>());
        let id = ast.alloc_node(UnresolvedColor::LightDark { light, dark }, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (value, compact, pretty) in [
            (
                UnresolvedColor::Rgb {
                    r: 1.0,
                    g: 2.0,
                    b: 3.0,
                    alpha,
                },
                "rgb(1 2 3 / .5)",
                "rgb(1 2 3 / .5)",
            ),
            (
                UnresolvedColor::LightDark { light, dark },
                "light-dark(1,2)",
                "light-dark(1, 2)",
            ),
            (
                UnresolvedColor::Hsl {
                    h: 25.0,
                    s: 0.01,
                    l: 0.02,
                    alpha,
                },
                "hsl(25 1% 2% / .5)",
                "hsl(25 1% 2% / .5)",
            ),
            (
                UnresolvedColor::Rgb {
                    r: 1.0,
                    g: 2.0,
                    b: 3.0,
                    alpha: empty,
                },
                "rgb(1 2 3 / )",
                "rgb(1 2 3 / )",
            ),
            (
                UnresolvedColor::LightDark {
                    light: dark,
                    dark: light,
                },
                "light-dark(2,1)",
                "light-dark(2, 1)",
            ),
        ] {
            ast.mutate_node(id, |stored, _| *stored = value);
            for (prettify, expected) in [(false, compact), (true, pretty)] {
                let cx = ToCssContext::with_ast(&token, &ast);
                let options = PrinterOptions { prettify };
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                        expected
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), 0);
        }
    });
}

#[test]
fn stored_lab_and_float_colors_preserve_output_after_variant_changes() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        macro_rules! check {
            ($cases:expr) => {{
                let mut previous = None;
                for (value, expected) in $cases {
                    let id = if let Some(id) = previous {
                        ast.mutate_node(id, |stored, _| *stored = value);
                        id
                    } else {
                        ast.alloc_node(value, DUMMY_SP)
                    };
                    previous = Some(id);
                    let checkpoint = ast.node_checkpoint();
                    for prettify in [false, true] {
                        let cx = ToCssContext::with_ast(&token, &ast);
                        let options = PrinterOptions { prettify };
                        for _ in 0..3 {
                            assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
                            assert_eq!(
                                ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                                expected
                            );
                        }
                    }
                    assert_eq!(ast.node_checkpoint(), checkpoint);
                    assert_eq!(ast.string_pool().extra_len(), 0);
                }
            }};
        }
        check!([
            (
                LABColor::Lab {
                    l: 25.0,
                    a: 1.0,
                    b: 2.0,
                    alpha: 0.5
                },
                "lab(25% 1 2 / .5)"
            ),
            (
                LABColor::Lch {
                    l: 25.0,
                    c: 1.0,
                    h: 2.0,
                    alpha: 0.5
                },
                "lch(25% 1 2 / .5)"
            ),
            (
                LABColor::Oklab {
                    l: 0.25,
                    a: 1.0,
                    b: 2.0,
                    alpha: 0.5
                },
                "oklab(25% 1 2 / .5)"
            ),
            (
                LABColor::Oklch {
                    l: 0.25,
                    c: 1.0,
                    h: 2.0,
                    alpha: 1.0
                },
                "oklch(25% 1 2)"
            ),
        ]);
        check!([
            (
                FloatColor::Rgb {
                    r: 0.25,
                    g: 1.0,
                    b: 2.0,
                    alpha: 0.5
                },
                "rgb(25% 1 2 / .5)"
            ),
            (
                FloatColor::Hsl {
                    h: 25.0,
                    s: 0.01,
                    l: 0.02,
                    alpha: 0.5
                },
                "hsl(25 1% 2% / .5)"
            ),
            (
                FloatColor::Hwb {
                    h: 25.0,
                    w: 0.01,
                    b: 0.02,
                    alpha: 1.0
                },
                "hwb(25 1% 2%)"
            ),
        ]);
    });
}

#[test]
fn stored_predefined_colors_preserve_spaces_and_component_order() {
    type MakeColor = fn(f32) -> PredefinedColor;
    let cases: [(&str, MakeColor); 8] = [
        ("srgb", |alpha| PredefinedColor::Srgb {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("srgb-linear", |alpha| PredefinedColor::SrgbLinear {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("display-p3", |alpha| PredefinedColor::DisplayP3 {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("a98-rgb", |alpha| PredefinedColor::A98Rgb {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("prophoto-rgb", |alpha| PredefinedColor::ProphotoRgb {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("rec2020", |alpha| PredefinedColor::Rec2020 {
            alpha,
            r: 1.0,
            g: 2.0,
            b: 3.0,
        }),
        ("xyz-d50", |alpha| PredefinedColor::XyzD50 {
            alpha,
            x: 1.0,
            y: 2.0,
            z: 3.0,
        }),
        ("xyz-d65", |alpha| PredefinedColor::XyzD65 {
            alpha,
            x: 1.0,
            y: 2.0,
            z: 3.0,
        }),
    ];
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let id = ast.alloc_node(cases[0].1(1.0), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for (space, make) in cases {
            for alpha in [0.5, 1.0] {
                ast.mutate_node(id, |value, _| *value = make(alpha));
                let view = ast.predefined_color(id);
                assert_eq!(view.space_name(), space);
                assert_eq!(view.components(), ([1.0, 2.0, 3.0], alpha));
                let expected = if alpha == 1.0 {
                    format!("color({space} 1 2 3)")
                } else {
                    format!("color({space} 1 2 3 / .5)")
                };
                for prettify in [false, true] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    let options = PrinterOptions { prettify };
                    for _ in 0..3 {
                        let stored = id.to_css_string(options, &cx).unwrap();
                        assert_eq!(
                            stored,
                            ast.resolve_node(id).to_css_string(options, &cx).unwrap()
                        );
                        assert_eq!(stored, expected);
                    }
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), 0);
                assert_eq!(ast.string_pool().len(), 0);
            }
        }
    });
}

#[test]
fn stored_environment_variables_preserve_indices_and_optional_fallbacks() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let unknown = ast.add_str("unusual");
        let ident = ast.add_str("--custom");
        let custom = ast.alloc_node(DashedIdentReference { ident, from: None }, DUMMY_SP);
        let indices = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in([1, 2], &allocator));
        let empty = ast.alloc_vec(allocator.vec::<TokenOrValue<'_>>());
        let token_value = ast.alloc_node(rocketcss_ast::Token::Number(3.0), DUMMY_SP);
        let fallback = ast.alloc_vec(rocketcss_common::vec::Vec::from_iter_in(
            [TokenOrValue::Token(token_value)],
            &allocator,
        ));
        let id = ast.alloc_node(
            EnvironmentVariable {
                name: EnvironmentVariableName::Unknown(unknown),
                indices,
                fallback: None,
            },
            DUMMY_SP,
        );
        for name in [
            EnvironmentVariableName::Unknown(unknown),
            EnvironmentVariableName::UA(UAEnvironmentVariable::SafeAreaInsetTop),
            EnvironmentVariableName::Custom(custom),
        ] {
            for fallback in [None, Some(empty), Some(fallback), None] {
                ast.mutate_node(id, |value, _| {
                    value.name = name;
                    value.fallback = fallback;
                });
                let checkpoint = ast.node_checkpoint();
                let bytes = ast.string_pool().extra_len();
                let interned = ast.string_pool().len();
                let view = ast.environment_variable(id);
                assert_eq!(view.name(), name);
                assert_eq!(view.indices(), indices);
                assert_eq!(view.fallback(), fallback);
                for prettify in [false, true] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    let options = PrinterOptions { prettify };
                    let owned = ast.resolve_node(id).to_css_string(options, &cx).unwrap();
                    assert!(owned.contains(" 1 2"));
                    assert_eq!(owned.contains(','), fallback.is_some());
                    for _ in 0..3 {
                        assert_eq!(id.to_css_string(options, &cx).unwrap(), owned);
                    }
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), bytes);
                assert_eq!(ast.string_pool().len(), interned);
            }
        }
    });
}

#[test]
fn stored_image_set_options_preserve_optional_type_after_mutation() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let image = ast.alloc_node(Image::None, DUMMY_SP);
        let mime = ast.add_str("image/avif");
        let id = ast.alloc_node(
            ImageSetOption {
                image,
                resolution: Resolution::Dpi(192.0),
                file_type: None,
            },
            DUMMY_SP,
        );
        for file_type in [None, Some(AstStr::EMPTY), Some(mime), None, Some(mime)] {
            ast.mutate_node(id, |value, _| value.file_type = file_type);
            let checkpoint = ast.node_checkpoint();
            let bytes = ast.string_pool().extra_len();
            let interned = ast.string_pool().len();
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                let options = PrinterOptions { prettify };
                let owned = ast.resolve_node(id).to_css_string(options, &cx).unwrap();
                for _ in 0..3 {
                    assert_eq!(id.to_css_string(options, &cx).unwrap(), owned);
                }
                match file_type {
                    None => assert!(!owned.contains("type(")),
                    Some(value) if value.is_empty() => assert!(owned.ends_with("type(\"\")")),
                    Some(_) => assert!(owned.ends_with("type(\"image/avif\")")),
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
            assert_eq!(ast.string_pool().len(), interned);
        }
    });
}

#[test]
fn stored_selector_component_names_preserve_authored_syntax_and_mutations() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.intern("DATA-X");
        let lower = ast.intern("data-x");
        let url = ast.intern("https://example.test/ns");
        let escaped = ast.intern("a:b");
        let component = ast.alloc_node(SelectorComponent::Class(name), DUMMY_SP);
        let mut components = allocator.vec();
        components.push(component);
        let components = ast.alloc_vec(components);
        let selector = ast.alloc_node(Selector::Parsed(components), DUMMY_SP);
        let child_component = ast.alloc_node(SelectorComponent::Class(lower), DUMMY_SP);
        let mut child_components = allocator.vec();
        child_components.push(child_component);
        let child_components = ast.alloc_vec(child_components);
        let child = ast.alloc_node(Selector::Parsed(child_components), DUMMY_SP);
        let mut children = allocator.vec();
        children.push(child);
        let children = ast.alloc_vec(children);
        let mut parts = allocator.vec();
        parts.extend([name, lower, name]);
        let parts = ast.alloc_vec(parts);
        let pseudo = ast.alloc_node(PseudoClass::Hover, DUMMY_SP);
        let element = ast.alloc_node(PseudoElement::Before, DUMMY_SP);
        let nth = NthSelectorData {
            kind: NthType::Child,
            is_function: true,
            a: 2,
            b: 1,
        };
        let attribute = ast.alloc_node(
            AttrSelector {
                local_name: name,
                local_name_lower: lower,
                namespace: None,
                operation: AttrOperation::Exists,
                never_matches: false,
            },
            DUMMY_SP,
        );
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        for (value, expected) in [
            (SelectorComponent::ExplicitNoNamespace, "|"),
            (SelectorComponent::AttributeOther(attribute), "[DATA-X]"),
            (SelectorComponent::DefaultNamespace(url), ""),
            (SelectorComponent::ExplicitUniversalType, "*"),
            (SelectorComponent::Root, ":root"),
            (SelectorComponent::Empty, ":empty"),
            (SelectorComponent::Scope, ":scope"),
            (SelectorComponent::Nesting, "&"),
            (
                SelectorComponent::AttributeInNoNamespace {
                    local_name: name,
                    operator: AttrSelectorOperator::Equal,
                    value: lower,
                    case_sensitivity: ParsedCaseSensitivity::ExplicitCaseSensitive,
                    never_matches: true,
                },
                "[DATA-X=data-x s]",
            ),
            (SelectorComponent::Negation(children), ":not(.data-x)"),
            (SelectorComponent::Where(children), ":where(.data-x)"),
            (SelectorComponent::Is(children), ".data-x"),
            (SelectorComponent::Has(children), ":has(.data-x)"),
            (
                SelectorComponent::Any {
                    vendor_prefix: VendorPrefix::WEBKIT,
                    selectors: children,
                },
                ":-webkit-any(.data-x)",
            ),
            (SelectorComponent::Host(None), ":host"),
            (SelectorComponent::Host(Some(child)), ":host(.data-x)"),
            (SelectorComponent::Slotted(child), "::slotted(.data-x)"),
            (
                SelectorComponent::Part(parts),
                "::part(DATA-X data-x DATA-X)",
            ),
            (SelectorComponent::PseudoClass(pseudo), ":hover"),
            (SelectorComponent::PseudoElement(element), ":before"),
            (SelectorComponent::Nth(nth), ":nth-child(odd)"),
            (
                SelectorComponent::NthOf {
                    data: nth,
                    selectors: children,
                },
                ":nth-child(odd of .data-x)",
            ),
            (
                SelectorComponent::Namespace { prefix: name, url },
                "DATA-X|",
            ),
            (
                SelectorComponent::LocalName {
                    name,
                    lower_name: lower,
                },
                "DATA-X",
            ),
            (
                SelectorComponent::AttributeInNoNamespaceExists {
                    local_name: name,
                    local_name_lower: lower,
                },
                "[DATA-X]",
            ),
            (SelectorComponent::Class(name), ".DATA-X"),
            (SelectorComponent::Id(name), "#DATA-X"),
            (SelectorComponent::Class(escaped), ".a\\:b"),
            (SelectorComponent::Id(escaped), "#a\\:b"),
            (SelectorComponent::ExplicitAnyNamespace, "*|"),
            (
                SelectorComponent::LocalName {
                    name: lower,
                    lower_name: lower,
                },
                "data-x",
            ),
        ] {
            let expected_value = value.clone();
            ast.mutate_node(component, |node, _| *node = value);
            assert_eq!(ast.resolve_node(component), expected_value);
            let checkpoint = ast.node_checkpoint();
            for prettify in [false, true] {
                let expected = if prettify {
                    expected.replace("=data-x", "=\"data-x\"")
                } else {
                    expected.to_owned()
                };
                let options = PrinterOptions { prettify };
                let cx = ToCssContext::with_ast(&token, &ast);
                for _ in 0..3 {
                    assert_eq!(component.to_css_string(options, &cx).unwrap(), expected);
                    assert_eq!(
                        ast.resolve_node(component)
                            .to_css_string(options, &cx)
                            .unwrap(),
                        expected
                    );
                    assert_eq!(selector.to_css_string(options, &cx).unwrap(), expected);
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
            assert_eq!(ast.string_pool().len(), interned);
        }
    });
}

#[test]
fn stored_matrices_preserve_component_order_after_mutation() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let matrix = ast.alloc_node(
            MatrixForFloat {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0,
            },
            DUMMY_SP,
        );
        let matrix_3d = ast.alloc_node(
            Matrix3DForFloat {
                m11: 1.0,
                m12: 2.0,
                m13: 3.0,
                m14: 4.0,
                m21: 5.0,
                m22: 6.0,
                m23: 7.0,
                m24: 8.0,
                m31: 9.0,
                m32: 10.0,
                m33: 11.0,
                m34: 12.0,
                m41: 13.0,
                m42: 14.0,
                m43: 15.0,
                m44: 16.0,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for last in [6.0, -0.0, -2.5] {
            ast.mutate_node(matrix, |value, _| value.f = last);
            ast.mutate_node(matrix_3d, |value, _| value.m44 = last);
            for prettify in [false, true] {
                let options = PrinterOptions { prettify };
                let cx = ToCssContext::with_ast(&token, &ast);
                for _ in 0..3 {
                    assert_eq!(
                        matrix.to_css_string(options, &cx).unwrap(),
                        ast.resolve_node(matrix)
                            .to_css_string(options, &cx)
                            .unwrap()
                    );
                    assert_eq!(
                        matrix_3d.to_css_string(options, &cx).unwrap(),
                        ast.resolve_node(matrix_3d)
                            .to_css_string(options, &cx)
                            .unwrap()
                    );
                }
                if !prettify && last == 6.0 {
                    assert_eq!(
                        matrix.to_css_string(options, &cx).unwrap(),
                        "matrix(1,2,3,4,5,6)"
                    );
                    assert_eq!(
                        matrix_3d.to_css_string(options, &cx).unwrap(),
                        "matrix3d(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,6)"
                    );
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
        }
    });
}

#[test]
fn stored_attribute_syntax_preserves_authored_name_and_mutations() {
    use rocketcss_ast::{
        AttrOperation, AttrSelector, AttrSelectorOperator, NamespaceConstraint,
        ParsedCaseSensitivity,
    };
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.intern("DATA-X");
        let lower = ast.intern("data-x");
        let value = ast.intern("hello world");
        let id = ast.alloc_node(
            AttrSelector {
                local_name: name,
                local_name_lower: lower,
                namespace: Some(NamespaceConstraint::Any),
                operation: AttrOperation::Exists,
                never_matches: false,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for (operation, expected) in [
            (AttrOperation::Exists, "[*|DATA-X]"),
            (
                AttrOperation::WithValue {
                    operator: AttrSelectorOperator::Equal,
                    expected_value: value,
                    case_sensitivity: ParsedCaseSensitivity::AsciiCaseInsensitive,
                },
                r"[*|DATA-X=hello\ world i]",
            ),
            (AttrOperation::Exists, "[*|DATA-X]"),
        ] {
            ast.mutate_node(id, |node, _| node.operation = operation);
            let cx = ToCssContext::with_ast(&token, &ast);
            let options = PrinterOptions { prettify: false };
            assert_eq!(id.to_css_string(options, &cx).unwrap(), expected);
            assert_eq!(
                ast.resolve_node(id).to_css_string(options, &cx).unwrap(),
                expected
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn stored_function_fields_preserve_fallbacks_and_replacement_transitions() {
    use rocketcss_ast::{Function, FunctionReplacement, Token, TokenOrValue};
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.add_str("--x");
        let ident = ast.alloc_node(Token::Ident(name), DUMMY_SP);
        let comma = ast.alloc_node(Token::Comma, DUMMY_SP);
        let mut values = allocator.vec();
        values.push(TokenOrValue::Token(ident));
        values.push(TokenOrValue::Token(comma));
        let arguments = ast.alloc_vec(values);
        for (name, expected) in [
            ("var", "var(--x, )"),
            ("env", "env(--x, )"),
            ("constant", "constant(--x, )"),
            ("FuN", "FuN(--x,)"),
        ] {
            let function = Function::new(name, arguments, &mut ast);
            let id = ast.alloc_node(function, DUMMY_SP);
            let checkpoint = ast.node_checkpoint();
            let pool_bytes = ast.string_pool().extra_len();
            for (replacement, output) in [
                (None, expected),
                (Some(FunctionReplacement::Number(3.0)), "3"),
                (None, expected),
                (
                    Some(FunctionReplacement::Rgb {
                        red: 255,
                        green: 0,
                        blue: 0,
                    }),
                    "red",
                ),
                (None, expected),
            ] {
                ast.mutate_node(id, |value, _| value.replacement = replacement);
                let view = ast.function(id);
                assert_eq!(view.replacement(), replacement);
                assert_eq!(view.arguments(), arguments);
                assert_eq!(ast.str(view.name()), name);
                assert_eq!(
                    id.to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::with_ast(&token, &ast)
                    )
                    .unwrap(),
                    output,
                );
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), pool_bytes);
        }
    });
}

#[test]
fn stored_unparsed_fields_preserve_raw_empty_and_token_fallback() {
    use rocketcss_ast::{
        AstStr, Function, FunctionReplacement, PropertyId, Token, TokenOrValue, UnparsedProperty,
        UnparsedPropertyReason,
    };
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.add_str("future-prop");
        let property_id = ast.alloc_node(PropertyId::Custom(name), DUMMY_SP);
        let ident = ast.add_str("--x");
        let ident = ast.alloc_node(Token::Ident(ident), DUMMY_SP);
        let comma = ast.alloc_node(Token::Comma, DUMMY_SP);
        let mut arguments = allocator.vec();
        arguments.push(TokenOrValue::Token(ident));
        arguments.push(TokenOrValue::Token(comma));
        let arguments = ast.alloc_vec(arguments);
        let mut function = Function::new("var", arguments, &mut ast);
        // Opaque token fallback must ignore the cached replacement.
        function.replacement = Some(FunctionReplacement::Number(3.0));
        let function = ast.alloc_node(function, DUMMY_SP);
        let mut values = allocator.vec();
        values.push(TokenOrValue::Function(function));
        let values = ast.alloc_vec(values);
        let raw = ast.add_str("Fn(01.00PX,/*x*/'Y')");
        let property = ast.alloc_node(
            UnparsedProperty {
                property_id,
                reason: UnparsedPropertyReason::UnknownProperty,
                raw_value: Some(raw),
                value: values,
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let pool_bytes = ast.string_pool().extra_len();
        for (raw_value, expected) in [
            (Some(raw), "Fn(01.00PX,/*x*/'Y')"),
            (Some(AstStr::EMPTY), ""),
            (None, "var(--x, )"),
            (Some(raw), "Fn(01.00PX,/*x*/'Y')"),
        ] {
            ast.mutate_node(property, |value, _| value.raw_value = raw_value);
            let view = ast.unparsed_property(property);
            assert_eq!(view.raw_value(), raw_value);
            assert_eq!(view.property_id(), property_id);
            assert_eq!(view.value(), values);
            assert_eq!(
                property
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::with_ast(&token, &ast)
                    )
                    .unwrap(),
                expected
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), pool_bytes);
    });
}

#[test]
fn native_text_and_appearance_ranges_preserve_content_without_growth() {
    use rocketcss_ast::{Appearance, TextEmphasisFillMode, TextEmphasisShape, TextEmphasisStyle};
    GhostToken::scope(|token| {
        assert_eq!(std::mem::size_of::<TextEmphasisStyle<'_>>(), 12);
        assert_eq!(std::mem::size_of::<Appearance<'_>>(), 12);
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first = ast.add_str("重点");
        let second = ast.add_str("重点");
        assert_ne!(first, second);
        let style = ast.alloc_node(TextEmphasisStyle::String(first), DUMMY_SP);
        let equal = ast.alloc_node(TextEmphasisStyle::String(second), DUMMY_SP);
        assert!(ast.nodes_eq(style, equal));
        let appearance = ast.alloc_node(Appearance::NonStandard(first), DUMMY_SP);
        let equal_appearance = ast.alloc_node(Appearance::NonStandard(second), DUMMY_SP);
        assert!(ast.nodes_eq(appearance, equal_appearance));
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            let cx = ToCssContext::with_ast(&token, &ast);
            assert_eq!(
                style
                    .to_css_string(PrinterOptions { prettify: false }, &cx)
                    .unwrap(),
                "\"重点\""
            );
            assert_eq!(
                appearance
                    .to_css_string(PrinterOptions { prettify: false }, &cx)
                    .unwrap(),
                "重点"
            );
        }
        for fill in [TextEmphasisFillMode::Filled, TextEmphasisFillMode::Open] {
            for shape in [
                None,
                Some(TextEmphasisShape::Dot),
                Some(TextEmphasisShape::Circle),
                Some(TextEmphasisShape::DoubleCircle),
                Some(TextEmphasisShape::Triangle),
                Some(TextEmphasisShape::Sesame),
            ] {
                let value = TextEmphasisStyle::Keyword { fill, shape };
                ast.mutate_node(style, |node, _| *node = value);
                assert_eq!(ast.resolve_node(style), value);
            }
        }
        for value in [
            Appearance::Auto,
            Appearance::Textfield,
            Appearance::None,
            Appearance::NonStandard(second),
        ] {
            ast.mutate_node(appearance, |node, _| *node = value);
            assert_eq!(ast.resolve_node(appearance), value);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn grid_line_range_codegen_preserves_indices_and_name_syntax() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.add_str("区域");
        let node = ast.alloc_node(GridLine::Auto, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for (value, expected) in [
            (GridLine::Auto, "auto"),
            (GridLine::Area { name }, "区域"),
            (
                GridLine::Line {
                    index: -2,
                    name: Some(name),
                },
                "-2 区域",
            ),
            (
                GridLine::Line {
                    index: 0,
                    name: Some(name),
                },
                "区域",
            ),
            (
                GridLine::Span {
                    index: 3,
                    name: None,
                },
                "span 3",
            ),
            (
                GridLine::Span {
                    index: 0,
                    name: Some(name),
                },
                "span 区域",
            ),
        ] {
            ast.mutate_node(node, |stored, _| *stored = value);
            assert_eq!(
                node.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::with_ast(&token, &ast)
                )
                .unwrap(),
                expected
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

fn first_rule_id<'ast>(compilation: &AstContext<'ast>) -> ConcreteRuleId<'ast> {
    compilation
        .rules_in_list(compilation.stylesheet().root_rules())
        .unwrap()
        .next()
        .expect("expected a root rule")
        .0
}

fn first_block_id<'ast>(compilation: &AstContext<'ast>) -> ConcreteDeclarationBlockId<'ast> {
    compilation
        .rule(first_rule_id(compilation))
        .and_then(|rule| rule.declaration_block())
        .expect("expected a declaration block")
}

fn property_declarations<'tree, 'ast>(
    compilation: &'tree AstContext<'ast>,
    block: ConcreteDeclarationBlockId<'ast>,
) -> std::vec::Vec<(&'tree Declaration<'ast>, bool)> {
    compilation
        .declarations_in_block(block)
        .unwrap()
        .filter_map(|record| match record.payload() {
            DeclarationPayload::Property(declaration) => Some((declaration, record.is_important())),
            _ => None,
        })
        .collect()
}
#[test]
#[ignore = "nested custom page regions are not represented in the AST yet"]
fn preserves_unknown_nested_page_regions() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@page{@footnote{float:bottom}@prince-overlay{content:\"continued\"}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
fn printer_remains_send_for_a_send_writer() {
    fn assert_send<T: Send>(_: T) {}

    let mut output = String::new();
    assert_send(Printer::new(&mut output, PrinterOptions::default()));
}

#[test]
fn preserves_text_decoration_line_order_before_minification() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{text-decoration-line:overline underline underline}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
fn preserves_known_color_keywords_before_minification() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{color:lightgreen;background-color:grey}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
fn preserves_animation_component_order_before_minification() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{animation:fade 1s ease}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
fn preserves_ratio_denominator_presence_before_minification() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{aspect-ratio:1;aspect-ratio:1/1}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let declarations = property_declarations(&stylesheet, first_block_id(&stylesheet));

        assert!(matches!(
            declarations[0].0,
            Declaration::AspectRatio(AspectRatio {
                ratio: Some(Ratio {
                    denominator: None,
                    numerator: 1.0,
                }),
                ..
            })
        ));
        assert!(matches!(
            declarations[1].0,
            Declaration::AspectRatio(AspectRatio {
                ratio: Some(Ratio {
                    denominator: Some(1.0),
                    numerator: 1.0,
                }),
                ..
            })
        ));
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
fn preserves_comments_in_css_wide_fallbacks_when_prettifying() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a{transform:initial/**/;all:initial/**/;columns:initial/**/}",
            &allocator,
            &mut token,
        );
        for (declaration, _) in property_declarations(&stylesheet, first_block_id(&stylesheet)) {
            assert!(
                declaration
                    .to_css_string(
                        PrinterOptions::default(),
                        &ToCssContext::with_ast(&token, &stylesheet),
                    )
                    .unwrap()
                    .contains("/**/")
            );
        }
    })
}

#[test]
fn ports_lightningcss_public_to_css_api_cases() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(".foo { color: red }", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            ".foo {\n  color: red;\n}\n"
        );

        let declarations = property_declarations(&stylesheet, first_block_id(&stylesheet));
        assert_eq!(
            declarations[0]
                .0
                .to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &stylesheet),
                )
                .unwrap(),
            "color: red"
        );
        let stylesheet = parse_stylesheet(
            "@media print{.a{color:red}.b{display:block}}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            "@media print{.a{color:red}.b{display:block}}"
        );
    })
}

#[test]
fn serializes_mask_shorthand_without_emitting_default_components() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a { mask: url(one.svg) center / cover no-repeat padding-box content-box exclude alpha, linear-gradient(red, blue); }",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token),
                )
                .unwrap(),
            "a{mask:url(one.svg) center/cover no-repeat padding-box content-box exclude alpha,linear-gradient(red,blue)}"
        );
    })
}

#[test]
fn stylesheet_implements_to_css() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".foo { color: green }\n.bar { color: red; background: pink }\n@media print { .baz { color: green } }",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            concat!(
                ".foo {\n",
                "  color: green;\n",
                "}\n\n",
                ".bar {\n",
                "  color: red;\n",
                "  background: pink;\n",
                "}\n\n",
                "@media print {\n",
                "  .baz {\n",
                "    color: green;\n",
                "  }\n",
                "}\n"
            )
        );
    })
}

#[test]
#[ignore]
fn supports_conditions_preserve_source_order_deterministically() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@supports ((foo: bar) or (color: red)) { .a { color: green } }";
        const EXPECTED: &str = "@supports ((foo: bar) or (color: red)){.a{color:green}}";

        for _ in 0..32 {
            let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
            let CssRulePayload::Supports(rule) = stylesheet
                .rule(first_rule_id(&stylesheet))
                .unwrap()
                .payload()
            else {
                panic!("expected a supports rule")
            };

            assert!(
                matches!(rule.condition, SupportsCondition::Unknown(value) if stylesheet.str(value) == "((foo: bar) or (color: red))")
            );
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                EXPECTED
            );
        }
    })
}

#[test]
#[ignore]
fn preserves_nonstandard_yahoo_media_query_prelude() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "@media screen yahoo { .a { color: red } }",
            &allocator,
            &mut token,
        );
        let CssRulePayload::Media(rule) = stylesheet
            .rule(first_rule_id(&stylesheet))
            .unwrap()
            .payload()
        else {
            panic!("expected media rule")
        };
        let query = stylesheet.resolve_node(stylesheet.vec_snapshot(rule.query.media_queries)[0]);
        assert!(matches!(query.media_type, MediaType::All));
        assert!(query.qualifier.is_none());
        assert!(matches!(
            stylesheet.resolve_node(query.condition.unwrap()),
            MediaCondition::Unknown(_)
        ));
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "@media screen yahoo{.a{color:red}}"
        );
    })
}

#[test]
#[ignore]
fn preserves_nonstandard_important_at_rule_as_unknown_syntax() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "@important{.card{color:red}.a{color:black}}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "@important{.card{color:red}.a{color:black}}"
        );
    })
}

#[test]
#[ignore]
fn pseudo_classes_are_debuggable_and_serializable() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        for source in [
            ".foo:hover{color:red}",
            ".foo:disabled{color:red}",
            ".foo:first-child{color:red}",
        ] {
            let stylesheet = parse_stylesheet(source, &allocator, &mut token);
            let CssRulePayload::Style(style) = stylesheet
                .rule(first_rule_id(&stylesheet))
                .unwrap()
                .payload()
            else {
                panic!("expected style rule")
            };
            assert!(format!("{style:#?}").contains("StyleRule"));
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                source
            );
        }
    })
}

#[test]
#[ignore]
fn preserves_keyframe_names_in_custom_properties_without_module_linking() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".root{--animation-name:fade-in}@keyframes fade-in{from{opacity:0}to{opacity:1}}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            ".root{--animation-name:fade-in}@keyframes fade-in{from{opacity:0}to{opacity:1}}"
        );
    })
}

#[test]
#[ignore]
fn preserves_css_modules_import_syntax_without_compiling_it() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "@value button from \"./button.module.css\";:import(\"./button.module.css\"){button:button}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "@value button from \"./button.module.css\";:import(\"./button.module.css\"){button:button}"
        );
    })
}

#[test]
#[ignore = "CSS Modules file aliases are preserved but not resolved yet"]
fn preserves_css_modules_file_alias_syntax() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@alias \"../../../../style/theme/colors.module.css\" as colors;.foobar{color:var(--primary from colors)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore]
fn preserves_nested_layer_structure_until_lifting_is_implemented() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".foo{@layer utilities{color:red}}.baz{@layer components{color:red}}.bar{@layer utilities{color:red}}",
            &allocator,
            &mut token,
        );
        for (_, rule) in stylesheet
            .rules_in_list(stylesheet.stylesheet().root_rules())
            .unwrap()
        {
            let CssRulePayload::Style(_) = rule.payload() else {
                panic!("expected style rule")
            };
            let layer_list = rule.child_list().expect("expected nested layer list");
            let (_, layer) = stylesheet
                .rules_in_list(layer_list)
                .unwrap()
                .next()
                .unwrap();
            let CssRulePayload::LayerBlock(_) = layer.payload() else {
                panic!("expected nested layer block")
            };
            let layer_children = layer.child_list().expect("expected layer contents");
            assert!(matches!(
                stylesheet
                    .rules_in_list(layer_children)
                    .unwrap()
                    .next()
                    .unwrap()
                    .1
                    .payload(),
                CssRulePayload::NestedDeclarations(_)
            ));
        }
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            ".foo{@layer utilities{color:red}}.baz{@layer components{color:red}}.bar{@layer utilities{color:red}}"
        );
    })
}

#[test]
#[ignore]
fn box_sizing_css_wide_keywords_round_trip_as_known_unparsed_values() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a{box-sizing:initial;box-sizing:inherit;box-sizing:unset;box-sizing:revert;box-sizing:revert-layer}",
            &allocator,
            &mut token,
        );
        let declarations = property_declarations(&stylesheet, first_block_id(&stylesheet));

        assert_eq!(declarations.len(), 5);
        assert!(declarations.iter().all(|(declaration, _)| matches!(
            declaration,
            Declaration::Unparsed(value)
                if {
                    let value = stylesheet.resolve_node(*value);
                    matches!(
                        stylesheet.resolve_node(value.property_id),
                        PropertyId::BoxSizing(VendorPrefix::NONE)
                    )
                }
        )));
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{box-sizing:initial;box-sizing:inherit;box-sizing:unset;box-sizing:revert;box-sizing:revert-layer}"
        );
    })
}

#[test]
fn compact_stylesheet_omits_optional_whitespace() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(".foo { color: #ff00ff }", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            ".foo{color:#f0f}"
        );
    })
}

#[test]
fn recovered_unparsed_selectors_round_trip_before_minification() {
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let mut stylesheet = parse(
            ".valid, (font-[family-name:var(--font-*)]), #also-valid { color: red }",
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        // Recovery publishes a raw selector alongside parsed selectors. Grow
        // the pool afterwards, then verify both remain stable across codegen.
        stylesheet.add_str(&"decoded-é".repeat(8192));
        let checkpoint = stylesheet.node_checkpoint();
        let bytes = stylesheet.string_pool().extra_len();
        for _ in 0..3 {
            assert_eq!(
                stylesheet
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::new(&token)
                    )
                    .unwrap(),
                ".valid,(font-[family-name:var(--font-*)]),#also-valid{color:red}"
            );
        }
        assert_eq!(stylesheet.node_checkpoint(), checkpoint);
        assert_eq!(stylesheet.string_pool().extra_len(), bytes);
    });
}

#[test]
#[ignore = "invalid declarations need a lossless raw AST representation"]
fn error_recovery_preserves_tailwind_wildcard_custom_properties() {
    const SOURCE: &str = ":root{--color-*:initial;color:red}";
    let allocator = Allocator::new();
    allocator.with_ghost(|mut token| {
        let stylesheet = parse(
            SOURCE,
            &allocator,
            &mut token,
            ParserOptions {
                error_recovery: true,
                ..ParserOptions::default()
            },
        )
        .unwrap();

        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    });
}

#[test]
fn font_family_lists_skip_tombstones_without_extra_commas() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.add_str("A");
        let mut families = allocator.vec();
        families.push(FontFamily::Tombstone);
        families.push(FontFamily::Custom(name));
        families.push(FontFamily::Tombstone);
        families.push(FontFamily::Serif);
        families.push(FontFamily::Tombstone);
        let mut family_ids = allocator.vec();
        for family in families {
            family_ids.push(ast.alloc_node(family, DUMMY_SP));
        }
        let families = ast.alloc_vec(family_ids);

        assert_eq!(
            families
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::with_ast(&token, &ast),
                )
                .unwrap(),
            "A,serif"
        );
    });
}

#[test]
fn serializes_typed_multicol_and_legacy_gap_properties() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a { -webkit-column-rule: red solid 1px; columns: 3 10px; grid-column-gap: 10%; grid-row-gap: normal }",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{-webkit-column-rule:1px solid red;columns:10px 3;grid-column-gap:10%;grid-row-gap:normal}"
        );
    })
}

#[test]
fn serializes_charset_rules() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "@charset 'UTF-8'; @import 'theme.css'; .foo { color: green }",
            &allocator,
            &mut token,
        );

        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            concat!(
                "@charset \"UTF-8\";\n",
                "@import \"theme.css\";\n\n",
                ".foo {\n",
                "  color: green;\n",
                "}\n"
            )
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "@charset \"UTF-8\";@import \"theme.css\";.foo{color:green}"
        );
    })
}

#[test]
fn function_codegen_uses_known_identity_and_preserves_original_name() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet =
            parse_stylesheet("a{color:VAR(--x,);width:CuStOm(1)}", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{color:VAR(--x,);width:CuStOm(1)}"
        );
    })
}

#[test]
fn unparsed_values_preserve_authored_spelling_for_every_reason() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let source = concat!(
            "a{",
            "unknown-prop: 01.2300e+2 \"x\" /*keep*/ custom( 1 , 2 );",
            "box-shadow: 01.2300px 0 0 \"shadow\";",
            "-webkit-box-shadow: 01.2300px 0 0 \"vendor\";",
            "width: calc( 100% - var(--gap) ) !important;",
            "display: TABLE-CELL flow",
            "}"
        );
        let stylesheet = parse_stylesheet(source, &allocator, &mut token);
        let declarations = property_declarations(&stylesheet, first_block_id(&stylesheet));
        let reasons = declarations
            .iter()
            .map(|(declaration, _)| match declaration {
                Declaration::Unparsed(value) => {
                    let value = stylesheet.resolve_node(*value);
                    assert!(value.raw_value.is_some());
                    value.reason
                }
                _ => panic!("expected all declarations to use the fallback AST"),
            })
            .collect::<std::vec::Vec<_>>();
        assert_eq!(
            reasons,
            [
                UnparsedPropertyReason::UnknownProperty,
                UnparsedPropertyReason::UnsupportedGrammar,
                UnparsedPropertyReason::UnsupportedGrammar,
                UnparsedPropertyReason::OpaqueValue,
                UnparsedPropertyReason::InvalidValue,
            ]
        );
        assert!(matches!(
            declarations[2].0,
            Declaration::Unparsed(value)
                if stylesheet
                    .resolve_node(stylesheet.resolve_node(*value).property_id)
                    .vendor_prefix()
                    == VendorPrefix::WEBKIT
        ));

        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            concat!(
                "a{",
                "unknown-prop:01.2300e+2 \"x\" /*keep*/ custom( 1 , 2 );",
                "box-shadow:01.2300px 0 0 \"shadow\";",
                "-webkit-box-shadow:01.2300px 0 0 \"vendor\";",
                "width:calc( 100% - var(--gap) ) !important;",
                "display:TABLE-CELL flow",
                "}"
            )
        );
    })
}

#[test]
fn serializes_packed_rgb_and_rgba_hex_values() {
    GhostToken::scope(|token| {
        for (color, expected) in [
            (
                RGBA {
                    red: 0xaa,
                    green: 0xbb,
                    blue: 0xcc,
                    alpha: 0xff,
                },
                "#abc",
            ),
            (
                RGBA {
                    red: 0x12,
                    green: 0x34,
                    blue: 0x56,
                    alpha: 0xff,
                },
                "#123456",
            ),
            (
                RGBA {
                    red: 0xaa,
                    green: 0xbb,
                    blue: 0xcc,
                    alpha: 0xdd,
                },
                "#abcd",
            ),
            (
                RGBA {
                    red: 0x12,
                    green: 0x34,
                    blue: 0x56,
                    alpha: 0x78,
                },
                "#12345678",
            ),
        ] {
            assert_eq!(
                color
                    .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                    .unwrap(),
                expected
            );
        }
    });
}

#[test]
fn serializes_typed_and_unknown_dimension_units() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let unit = ast.add_str("furlong");
        assert_eq!(
            Token::Dimension {
                value: 2.0,
                unit: Unit::Length(LengthUnit::Px),
            }
            .to_css_string(
                PrinterOptions::default(),
                &ToCssContext::with_ast(&token, &ast)
            )
            .unwrap(),
            "2px"
        );
        assert_eq!(
            Token::UnknownDimension { value: 2.0, unit }
                .to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast)
                )
                .unwrap(),
            "2furlong"
        );
    });
}

#[test]
fn declaration_block_preserves_importance_bits() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".foo { color: red !important; opacity: .5 }",
            &allocator,
            &mut token,
        );
        let declarations = property_declarations(&stylesheet, first_block_id(&stylesheet));
        assert_eq!(declarations.len(), 2);
        assert!(declarations[0].1);
        assert!(!declarations[1].1);
        assert_eq!(
            stylesheet
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            ".foo {\n  color: red !important;\n  opacity: .5;\n}\n"
        );
    })
}

#[test]
fn ports_lightningcss_typed_value_serialization_cases() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        assert_eq!(
            Time::Milliseconds(100.0)
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            ".1s"
        );
        assert_eq!(
            EasingFunction::CubicBezier {
                x1: 0.42,
                y1: 0.0,
                x2: 1.0,
                y2: 1.0,
            }
            .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
            .unwrap(),
            "ease-in"
        );
        assert_eq!(
            UnicodeRange {
                start: 0x400,
                end: 0x4ff,
            }
            .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
            .unwrap(),
            "U+4??"
        );
        assert_eq!(
            FontFormat::Woff
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            "\"woff\""
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("Fancy Font Name");
                FamilyName(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "Fancy Font Name"
        );
        assert_eq!(
            FontFamily::SansSerif
                .to_css_string(PrinterOptions::default(), &ToCssContext::new(&token))
                .unwrap(),
            "sans-serif"
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("serif");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "\"serif\""
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("Fancy Font");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "Fancy Font"
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("A  B");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "\"A  B\""
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("1");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "\"1\""
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("slab serif");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "\"slab serif\""
        );
        assert_eq!(
            {
                let mut ast = AstContext::new_in(&allocator);
                let name = ast.add_str("slab inherit");
                FontFamily::Custom(name).to_css_string(
                    PrinterOptions::default(),
                    &ToCssContext::with_ast(&token, &ast),
                )
            }
            .unwrap(),
            "\"slab inherit\""
        );
    });
}

#[test]
#[ignore = "pseudo-elements inside :is() need lossless diagnostics"]
fn preserves_pseudo_elements_inside_is() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(".foo:is(::before){color:green}", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            ".foo:is(:before){color:green}"
        );
    })
}

#[test]
#[ignore = "CSS Modules composition is not implemented"]
fn preserves_composes_inside_layers_until_module_compilation() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".default{color:red}.button{composes:default}@layer components{.foo{composes:bar from \"./other.module.css\"}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "CSS Modules grid symbol transforms are not implemented"]
fn preserves_dynamic_grid_symbols_until_module_compilation() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".test{grid-template:\"test\" var(--foo);grid-template:\"test\" 1fr}.item{grid-area:test}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "CSS Modules dashed-ident resolution is not implemented"]
fn preserves_imported_dashed_idents_in_nested_values_and_rules() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".x{background-color:rgb(var(--blue from \"./colors.module.css\"));&.info{border-color:var(--border);color:var(--red from \"./colors.module.css\")}}@media (min-width:10px){.x{color:var(--red from \"./colors.module.css\")}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "module-qualified custom-property definitions are not represented"]
fn preserves_module_qualified_custom_property_definitions() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".other-button{composes:button from \"./button.module.css\";--accent from \"./button.module.css\":blue}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "CSS custom functions and mixins are preserved but not implemented"]
fn preserves_css_custom_functions_and_mixins_draft() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@function --negative(--value <length>) returns <length>{result:calc(-1 * var(--value))}.foo{margin:--negative(1px)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "custom at-rule visitor expansion is not implemented"]
fn expands_mixins_at_the_apply_position_without_reordering_declarations() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@mixin card{background:var(--bg-card);border-radius:var(--border-radius-md);padding:var(--spacing-5)}.quote{@apply card;transition:background var(--duration);margin-block-end:0;border-top-left-radius:0;border-bottom-left-radius:0;border-left-width:5px;border-left-color:var(--color-gray-400)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            ".quote{background:var(--bg-card);border-radius:var(--border-radius-md);padding:var(--spacing-5);transition:background var(--duration);margin-block-end:0;border-top-left-radius:0;border-bottom-left-radius:0;border-left-width:5px;border-left-color:var(--color-gray-400)}"
        );
    })
}

#[test]
#[ignore = "CSS Modules scoped keyframe names are not represented"]
fn preserves_global_keyframe_names_until_module_compilation() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@keyframes :global(jump){0%{transform:translateY(0)}50%{transform:translateY(-10px)}100%{transform:translateY(0)}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore = "target-aware light-dark lowering is not implemented"]
fn preserves_light_dark_when_a_child_changes_color_scheme() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ":root{--background:light-dark(white,black);--text:light-dark(black,white)}p{color:var(--text);background:var(--background);color-scheme:dark}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, SOURCE);
        assert!(!output.contains("--lightningcss-light"));
        assert!(!output.contains("--lightningcss-dark"));
    })
}

#[test]
#[ignore = "pseudo-element nesting validation and lowering are not implemented"]
fn preserves_nested_pseudo_element_rules_without_invalid_flattening() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".input::placeholder{&:not(.noAdaptiveTypography){font-size:inherit}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, SOURCE);
        assert!(!output.contains(".input::placeholder:not("));
    })
}

#[test]
#[ignore = "target-aware vendor prefix generation is not implemented"]
fn does_not_duplicate_authored_text_decoration_when_prefixing_for_targets() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str =
            "a{color:inherit;-webkit-text-decoration:inherit;text-decoration:inherit}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, SOURCE);
        assert_eq!(output.matches("-webkit-text-decoration:inherit").count(), 1);
    })
}

#[test]
#[ignore = "CSS Modules scoped selector compilation and cross-rule merging are not implemented"]
fn combines_resolved_local_and_global_css_module_selectors() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str =
            ".a{color:red}.b{color:red}:global(.c){color:red}:global(.d){color:red}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output.matches("{color:red}").count(), 1);
        assert!(!output.contains(":global"));
    })
}

#[test]
#[ignore = "target-aware supports fallback generation is not implemented"]
fn preserves_root_and_host_when_generating_supports_fallbacks() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ":root,:host{--theme:color(display-p3 1 0 0)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(
            output,
            "@supports (color:color(display-p3 0 0 0)){:root,:host{--theme:color(display-p3 1 0 0)}}"
        );
    })
}

#[test]
#[ignore = "target-driven user-select prefix generation is not implemented"]
fn generates_user_select_prefix_for_safari_targets() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{user-select:all}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{-webkit-user-select:all;user-select:all}"
        );
    })
}

#[test]
#[ignore = "target-driven logical property lowering is not implemented"]
fn does_not_partially_lower_dynamic_logical_shorthands() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{margin-inline:var(--m);color:red}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore]
fn preserves_svg_data_urls_with_opposite_quote_styles() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = r#".a{background:url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg"></svg>')}.b{background:url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg'></svg>")}"#;
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output.matches("data:image/svg+xml").count(), 2);
        assert!(output.contains("xmlns"));
        let _ = parse_stylesheet(&output, &allocator, &mut token);
    })
}

#[test]
#[ignore]
fn preserves_unescaped_exponent_like_unknown_units() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = r"a{height:0e;height:0E;height:0\65}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert!(output.contains("height:0e"));
        assert!(output.contains("height:0E"));
        assert!(!output.contains(r"0\65"));
    })
}

#[test]
#[ignore]
fn retains_more_than_six_significant_digits_when_serializing_numbers() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a{line-height:1.3333333333;width:33.333333%}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{line-height:1.3333334;width:33.333332%}"
        );
    })
}

#[test]
#[ignore = "custom-media expansion after stylesheet replacement is not implemented"]
fn expands_custom_media_after_a_stylesheet_replacement() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str =
            "@custom-media --narrow (max-width:30em);@media (--narrow){.a{color:red}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "@media (max-width:30em){.a{color:red}}"
        );
    })
}

#[test]
#[ignore = "iOS-target text-size-adjust prefix generation is not implemented"]
fn generates_text_size_adjust_prefix_for_ios_safari() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet("a{text-size-adjust:none}", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{-webkit-text-size-adjust:none;text-size-adjust:none}"
        );
    })
}

#[test]
#[ignore = "browser-target diagnostics for unlowerable selectors are not implemented"]
fn preserves_where_specificity_when_a_legacy_target_requires_a_diagnostic() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ":where(.button,#danger){color:red}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, SOURCE);
        assert!(!output.contains(":is("));
    })
}

#[test]
#[ignore]
fn preserves_property_rules_inside_layer_blocks() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "@layer base{@property --radialprogress{syntax:\"<percentage>\";inherits:true;initial-value:0%}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let layer = stylesheet.rule(first_rule_id(&stylesheet)).unwrap();
        let CssRulePayload::LayerBlock(_) = layer.payload() else {
            panic!("expected layer block")
        };
        assert!(matches!(
            stylesheet
                .rules_in_list(layer.child_list().unwrap())
                .unwrap()
                .next()
                .unwrap()
                .1
                .payload(),
            CssRulePayload::Property(_)
        ));
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore]
fn preserves_numeric_oklch_property_initial_values() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str =
            "@property --accent{syntax:\"<color>\";inherits:false;initial-value:oklch(.5 0 0)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            SOURCE
        );
    })
}

#[test]
#[ignore]
fn preserves_attr_type_angle_brackets_without_inserted_whitespace() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = "a{max-width:attr(data-max-width type(<length>)|fit-content)}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert!(output.contains("type(<length>)"));
        assert!(!output.contains("< length>"));
        let _ = parse_stylesheet(&output, &allocator, &mut token);
    })
}

#[test]
#[ignore = "target-aware nesting lowering is not implemented"]
fn avoids_invalid_is_wrapping_for_nested_pseudo_element_media_rules() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".foo::after,.bar::after{@media screen{color:red}}",
            &allocator,
            &mut token,
        );
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, "@media screen{.foo:after,.bar:after{color:red}}");
        assert!(!output.contains(":is("));
    })
}

#[test]
#[ignore = "target-aware vendor prefix generation is not implemented"]
fn retains_authored_vendor_values_when_generating_missing_prefixes() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "a{-webkit-appearance:none;appearance:textfield}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{-webkit-appearance:none;-moz-appearance:textfield;appearance:textfield}"
        );
    })
}

#[test]
#[ignore]
fn preserves_three_length_text_shadows_without_inserting_a_spread() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            ".foo{text-shadow:0 .02rem 0 rgba(0,0,0,.05)}",
            &allocator,
            &mut token,
        );
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert!(output.contains("text-shadow:0 .02rem 0 rgba(0,0,0,.05)"));
        assert!(!output.contains("text-shadow:0 .02rem 0 0"));
        let _ = parse_stylesheet(&output, &allocator, &mut token);
    })
}

#[test]
#[ignore]
fn preserves_unknown_media_calc_symbols_and_rule_bodies() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "@media (min-width:calc(baseUnit * 1)){.className{color:red}}",
            &allocator,
            &mut token,
        );
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert!(output.contains("baseUnit * 1"));
        assert!(output.contains(".className{color:red}"));
        let reparsed = parse_stylesheet(&output, &allocator, &mut token);
        assert_eq!(
            reparsed
                .rules_in_list(reparsed.stylesheet().root_rules())
                .unwrap()
                .count(),
            1
        );
    })
}

#[test]
#[ignore = "target-aware nesting lowering is not implemented"]
fn preserves_pseudo_elements_when_lowering_nested_parent_selectors() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet("#b::after{&{color:green}}", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "#b::after{color:green}"
        );
    })
}

#[test]
#[ignore = "pseudo-element chaining validation and source spelling preservation are not implemented"]
fn preserves_valid_before_and_after_marker_chains() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(
            "li::before::marker,li::after::marker{content:\"\"}",
            &allocator,
            &mut token,
        );
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "li::before::marker,li::after::marker{content:\"\"}"
        );
    })
}

#[test]
#[ignore = "browser-target selector lowering is not implemented"]
fn avoids_legacy_any_fallbacks_when_targets_support_selector_list_not() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet(":not(a,block){color:red}", &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, ":not(a,block){color:red}");
        assert!(!output.contains("-webkit-any"));
        assert!(!output.contains("-moz-any"));
    })
}

#[test]
#[ignore]
fn printer_options_are_copy_clone_and_debuggable() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<PrinterOptions>();
    let options = PrinterOptions { prettify: false };
    let copied = options;
    assert_eq!(options, copied);
    assert_eq!(format!("{options:?}"), "PrinterOptions { prettify: false }");
}

#[test]
#[ignore = "an explicit quirks-mode color parser is not implemented"]
fn normalizes_legacy_bare_hex_colors_only_in_quirks_mode() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        let stylesheet = parse_stylesheet("a{background-color:333333}", &allocator, &mut token);
        assert_eq!(
            stylesheet
                .to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::new(&token)
                )
                .unwrap(),
            "a{background-color:#333}"
        );
    })
}

#[test]
#[ignore = "target-aware logical-property lowering is not implemented"]
fn avoids_specificity_increases_when_lowering_logical_margins() {
    GhostToken::scope(|mut token| {
        let allocator = Allocator::new();
        const SOURCE: &str = ".ms-0{margin-inline-start:0}@media(min-width:1536px){.two-xl\\:mx-auto{margin-inline:auto}}";
        let stylesheet = parse_stylesheet(SOURCE, &allocator, &mut token);
        let output = stylesheet
            .to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::new(&token),
            )
            .unwrap();
        assert_eq!(output, SOURCE);
        assert!(!output.contains(":lang("));
    })
}

#[test]
fn grid_nested_line_name_ranges_codegen_without_pool_growth() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first = ast.add_str("区域");
        let second = ast.add_str("区域");
        assert_ne!(first, second);
        let mut names = allocator.vec();
        names.extend([first, second]);
        let names = ast.alloc_vec(names);
        let mut groups = allocator.vec();
        groups.extend([names, names]);
        let line_names = ast.alloc_vec(groups);
        let breadth = ast.alloc_node(TrackBreadth::Flex(2.0), DUMMY_SP);
        let size = ast.alloc_node(TrackSize::TrackBreadth(breadth), DUMMY_SP);
        let mut sizes = allocator.vec();
        sizes.push(size);
        let sizes = ast.alloc_vec(sizes);
        let repeat = ast.alloc_node(
            TrackRepeat {
                count: RepeatCount::Number(2.0),
                line_names,
                track_sizes: sizes,
            },
            DUMMY_SP,
        );
        let item = ast.alloc_node(TrackListItem::TrackRepeat(repeat), DUMMY_SP);
        let mut items = allocator.vec();
        items.push(item);
        let items = ast.alloc_vec(items);
        let tracks = ast.alloc_node(TrackSizing::TrackList { items, line_names }, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            ast.mutate_node(tracks, |_, _| {});
            assert_eq!(
                tracks
                    .to_css_string(
                        PrinterOptions { prettify: false },
                        &ToCssContext::with_ast(&token, &ast)
                    )
                    .unwrap(),
                "[区域 区域] repeat(2,[区域 区域] 2fr [区域 区域]) [区域 区域]"
            );
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn grid_area_ranges_preserve_empty_and_missing_cells() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let name = ast.add_str("区域");
        let mut cells = allocator.vec();
        cells.extend([Some(name), None, Some(AstStr::EMPTY), Some(name), None]);
        let areas = ast.alloc_vec(cells);
        let node = ast.alloc_node(GridTemplateAreas::Areas { areas, columns: 2 }, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            ast.mutate_node(node, |_, _| {});
            assert_eq!(
                node.to_css_string(
                    PrinterOptions { prettify: false },
                    &ToCssContext::with_ast(&token, &ast)
                )
                .unwrap(),
                "\"区域 .\" \" 区域\" \".\""
            );
        }
        ast.mutate_node(node, |value, _| {
            *value = GridTemplateAreas::Areas { areas, columns: 0 }
        });
        assert_eq!(
            node.to_css_string(
                PrinterOptions { prettify: false },
                &ToCssContext::with_ast(&token, &ast)
            )
            .unwrap(),
            ""
        );
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn ordinary_list_container_and_timeline_names_use_ranges_without_growth() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first = ast.add_str("区域");
        let second = ast.add_str("区域");
        assert_ne!(first, second);
        let symbol = ast.alloc_node(Symbol::String(first), DUMMY_SP);
        let equal_symbol = ast.alloc_node(Symbol::String(second), DUMMY_SP);
        assert!(ast.nodes_eq(symbol, equal_symbol));
        let empty = ast.alloc_node(Symbol::String(AstStr::EMPTY), DUMMY_SP);
        let mut symbols = allocator.vec();
        symbols.extend([symbol, equal_symbol, empty]);
        let symbols = ast.alloc_vec(symbols);
        let counter = ast.alloc_node(
            CounterStyle::Symbols {
                symbols,
                system: SymbolsType::Cyclic,
            },
            DUMMY_SP,
        );
        let named_counter = ast.alloc_node(CounterStyle::Name(first), DUMMY_SP);
        let equal_counter = ast.alloc_node(CounterStyle::Name(second), DUMMY_SP);
        assert!(ast.nodes_eq(named_counter, equal_counter));
        let list = ast.alloc_node(ListStyleType::String(first), DUMMY_SP);
        let equal_list = ast.alloc_node(ListStyleType::String(second), DUMMY_SP);
        assert!(ast.nodes_eq(list, equal_list));
        let timeline_name = ast.add_str("--区域");
        let other_timeline_name = ast.add_str("--区域");
        let timeline = ast.alloc_node(AnimationTimeline::DashedIdent(timeline_name), DUMMY_SP);
        let equal_timeline = ast.alloc_node(
            AnimationTimeline::DashedIdent(other_timeline_name),
            DUMMY_SP,
        );
        assert!(ast.nodes_eq(timeline, equal_timeline));
        let mut names = allocator.vec();
        names.extend([first, second]);
        let names = ast.alloc_vec(names);
        let container = ast.alloc_node(ContainerNameList::Names(names), DUMMY_SP);
        let bytes_before_clone = ast.string_pool().extra_len();
        let cloned_counter = ast.clone_node(counter);
        let CounterStyle::Symbols {
            symbols: cloned_symbols,
            ..
        } = ast.resolve_node(cloned_counter)
        else {
            panic!()
        };
        assert_ne!(cloned_symbols, symbols);
        let cloned_symbol = ast.vec_get(cloned_symbols, 0).unwrap();
        assert_ne!(cloned_symbol, symbol);
        ast.mutate_node(cloned_symbol, |value, _| {
            *value = Symbol::String(AstStr::EMPTY)
        });
        assert_eq!(ast.resolve_node(symbol), Symbol::String(first));
        assert_eq!(ast.string_pool().extra_len(), bytes_before_clone);
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            ast.mutate_node(counter, |_, _| {});
            ast.mutate_node(symbol, |_, _| {});
            ast.mutate_node(timeline, |_, _| {});
            ast.mutate_node(list, |_, _| {});
            ast.mutate_node(container, |_, _| {});
            let cx = ToCssContext::with_ast(&token, &ast);
            let options = PrinterOptions { prettify: false };
            assert_eq!(
                counter.to_css_string(options, &cx).unwrap(),
                "symbols(cyclic \"区域\" \"区域\" \"\")"
            );
            assert_eq!(named_counter.to_css_string(options, &cx).unwrap(), "区域");
            assert_eq!(list.to_css_string(options, &cx).unwrap(), "\"区域\"");
            assert_eq!(timeline.to_css_string(options, &cx).unwrap(), "--区域");
            assert_eq!(container.to_css_string(options, &cx).unwrap(), "区域 区域");
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn composes_and_view_transition_lists_preserve_duplicate_ranges() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first = ast.add_str("区域");
        let second = ast.add_str("区域");
        assert_ne!(first, second);
        let mut names = allocator.vec();
        names.extend([first, second]);
        let names = ast.alloc_vec(names);
        let from = ast.alloc_node(Specifier::Global, DUMMY_SP);
        let composes = ast.alloc_node(
            Composes {
                names,
                from: Some(from),
            },
            DUMMY_SP,
        );
        let name = ast.alloc_node(ViewTransitionPartName::All, DUMMY_SP);
        let selector = ast.alloc_node(
            ViewTransitionPartSelector {
                classes: names,
                name: Some(name),
            },
            DUMMY_SP,
        );
        let checkpoint = ast.node_checkpoint();
        let bytes = ast.string_pool().extra_len();
        for _ in 0..3 {
            ast.mutate_node(composes, |_, _| {});
            ast.mutate_node(selector, |_, _| {});
            let cx = ToCssContext::with_ast(&token, &ast);
            assert_eq!(
                composes
                    .to_css_string(PrinterOptions { prettify: false }, &cx)
                    .unwrap(),
                "区域 区域 from global"
            );
            assert_eq!(
                selector
                    .to_css_string(PrinterOptions { prettify: false }, &cx)
                    .unwrap(),
                "*.区域.区域"
            );
        }
        ast.mutate_node(composes, |value, _| value.from = None);
        ast.mutate_node(selector, |value, _| value.name = None);
        let cx = ToCssContext::with_ast(&token, &ast);
        assert_eq!(
            composes
                .to_css_string(PrinterOptions { prettify: false }, &cx)
                .unwrap(),
            "区域 区域"
        );
        assert_eq!(
            selector
                .to_css_string(PrinterOptions { prettify: false }, &cx)
                .unwrap(),
            ".区域.区域"
        );
        assert_eq!(ast.node_checkpoint(), checkpoint);
        assert_eq!(ast.string_pool().extra_len(), bytes);
    });
}

#[test]
fn supports_not_reuses_values_without_changing_parentheses() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let first_text = ast.add_str("(color:red)");
        let second_text = ast.add_str("(display:grid)");
        let first = ast.alloc_node(SupportsCondition::Unknown(first_text), DUMMY_SP);
        let second = ast.alloc_node(SupportsCondition::Unknown(second_text), DUMMY_SP);
        let values = ast.alloc_vec({
            let mut list = allocator.vec();
            list.extend([first, second]);
            list
        });
        let and = ast.alloc_node(SupportsCondition::And(values), DUMMY_SP);
        let or = ast.alloc_node(SupportsCondition::Or(values), DUMMY_SP);
        for (child, expected) in [
            (first, "not (color:red)"),
            (and, "not ((color:red) and (display:grid))"),
            (or, "not ((color:red) or (display:grid))"),
        ] {
            let id = ast.alloc_node(SupportsCondition::Not(child), DUMMY_SP);
            let checkpoint = ast.node_checkpoint();
            let bytes = ast.string_pool().extra_len();
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                assert_eq!(
                    id.to_css_string(PrinterOptions { prettify }, &cx).unwrap(),
                    expected
                );
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
        }
    });
}

#[test]
fn unknown_media_ranges_stream_leading_whitespace_and_qualifiers() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let space = ast.add_str(" ");
        let name = ast.add_str("custom");
        let items = [
            Token::WhiteSpace(space),
            Token::WhiteSpace(space),
            Token::ParenthesisBlock,
            Token::Ident(name),
            Token::CloseParenthesis,
            Token::WhiteSpace(space),
        ]
        .map(|value| TokenOrValue::Token(ast.alloc_node(value, DUMMY_SP)));
        let mut values = allocator.vec();
        values.extend(items);
        let range = ast.alloc_vec(values);
        let condition = ast.alloc_node(MediaCondition::Unknown(range), DUMMY_SP);
        for (qualifier, media_type, expected) in [
            (None, MediaType::All, "(custom) "),
            (Some(Qualifier::Not), MediaType::All, "not (custom) "),
            (Some(Qualifier::Only), MediaType::All, "only all (custom) "),
            (
                Some(Qualifier::Not),
                MediaType::Screen,
                "not screen (custom) ",
            ),
            (None, MediaType::Screen, "screen (custom) "),
        ] {
            let id = ast.alloc_node(
                MediaQuery {
                    qualifier,
                    media_type,
                    condition: Some(condition),
                },
                DUMMY_SP,
            );
            let checkpoint = ast.node_checkpoint();
            let bytes = ast.string_pool().extra_len();
            for prettify in [false, true] {
                let cx = ToCssContext::with_ast(&token, &ast);
                assert_eq!(
                    id.to_css_string(PrinterOptions { prettify }, &cx).unwrap(),
                    expected
                );
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
            assert_eq!(ast.string_pool().extra_len(), bytes);
        }
        for empty in [true, false] {
            let mut values = allocator.vec();
            if !empty {
                values.extend(ast.vec_iter(range).take(2));
            }
            let range = ast.alloc_vec(values);
            ast.mutate_node(condition, |value, _| {
                *value = MediaCondition::Unknown(range)
            });
            for (qualifier, expected) in [(None, ""), (Some(Qualifier::Not), "not all ")] {
                let query = MediaQuery {
                    qualifier,
                    media_type: MediaType::All,
                    condition: Some(condition),
                };
                for prettify in [false, true] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    assert_eq!(
                        query
                            .to_css_string(PrinterOptions { prettify }, &cx)
                            .unwrap(),
                        expected
                    );
                }
            }
        }
    });
}

#[test]
fn animation_name_keyword_matching_preserves_case_and_quotes() {
    GhostToken::scope(|token| {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        for keyword in [
            "none",
            "initial",
            "inherit",
            "unset",
            "default",
            "revert",
            "revert-layer",
        ] {
            for text in [
                keyword.to_owned(),
                keyword.to_ascii_uppercase(),
                format!("{}{}", keyword[..1].to_ascii_uppercase(), &keyword[1..]),
            ] {
                let range = ast.add_str(&text);
                let quoted = ast.alloc_node(AnimationName::String(range), DUMMY_SP);
                let ident = ast.alloc_node(AnimationName::Ident(range), DUMMY_SP);
                let checkpoint = ast.node_checkpoint();
                let bytes = ast.string_pool().extra_len();
                for prettify in [false, true] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    let options = PrinterOptions { prettify };
                    assert_eq!(
                        quoted.to_css_string(options, &cx).unwrap(),
                        format!("\"{text}\"")
                    );
                    assert_eq!(ident.to_css_string(options, &cx).unwrap(), text);
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), bytes);
            }
        }
        for text in ["fade-in", "NONE-x", "xINITIAL", "İNİTİAL", "révert"] {
            let range = ast.add_str(text);
            let value = ast.alloc_node(AnimationName::String(range), DUMMY_SP);
            let cx = ToCssContext::with_ast(&token, &ast);
            assert_eq!(
                value
                    .to_css_string(PrinterOptions { prettify: false }, &cx)
                    .unwrap(),
                text
            );
        }
    });
}

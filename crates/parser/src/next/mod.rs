//! Parsers being migrated to the flattened AST.

mod alignment;
mod animation;
mod background;
mod border;
mod box_model;
mod color;
mod flex;
mod font;
mod image;
mod multicol;
mod ui;
mod svg;
mod length;
mod text;
mod transform;
mod mask;
mod prelude;
mod token;
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
        assert!(!matches!(
            top.data(ast),
            LengthPercentageOrAutoData::Auto
        ));
        assert_eq!(padding.bottom(ast), top);

        let (compiler, value) = parse_value::<LengthPercentageOrAuto>("auto", &allocator);
        let ast = compiler.ast();
        assert!(matches!(
            value.data(ast),
            LengthPercentageOrAutoData::Auto
        ));
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
        assert!(matches!(border.color(ast).data(ast), CssColorData::Known(_)));
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
        assert!(!matches!(
            width.data(ast),
            LengthPercentageOrAutoData::Auto
        ));
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
        assert!(matches!(
            family.data(ast),
            FontFamilyData::Custom("Arial")
        ));

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

        let (compiler, transform) = parse_value::<TextTransform>("uppercase full-width", &allocator);
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
        assert_eq!(mask.clip(ast), MaskClip::GeometryBox(GeometryBox::BorderBox));
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

        let (compiler, transform) = parse_value::<Transform>("matrix(1, 0, 0, 1, 0, 0)", &allocator);
        let ast = compiler.ast();
        let TransformData::Matrix(matrix) = transform.data(ast) else {
            panic!("expected matrix")
        };
        assert_eq!(matrix.a(ast), 1.0);
        assert_eq!(matrix.d(ast), 1.0);

        let (compiler, scale) = parse_value::<Scale>("2", &allocator);
        let ast = compiler.ast();
        let Scale::Xyz { x, y, z } = scale else {
            panic!("expected scale xyz")
        };
        assert_eq!(x, NumberOrPercentage::Number(2.0));
        assert_eq!(y, NumberOrPercentage::Number(2.0));
        assert_eq!(z, NumberOrPercentage::Number(1.0));
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

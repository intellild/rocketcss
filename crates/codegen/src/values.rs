use crate::prelude::*;

pub(crate) mod font;

macro_rules! keyword_values {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, '_, '_, 'ghost>) -> fmt::Result {
                    dest.write_str(self.as_css_str().expect("keyword enum has only static variants"))
                }
            }
        )+
    };
}

keyword_values! {
    HorizontalPositionKeyword,
    VerticalPositionKeyword,
    ShapeExtent,
    BackgroundRepeatKeyword,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    ObjectFit,
    DisplayKeyword,
    DisplayOutside,
    Visibility,
    BoxSizing,
    OverflowKeyword,
    TextOverflow,
    LineStyle,
    BorderImageRepeatKeyword,
    FlexDirection,
    FlexWrap,
    BaselinePosition,
    ContentDistribution,
    OverflowPosition,
    ContentPosition,
    SelfPosition,
    LegacyJustify,
    BoxOrient,
    BoxDirection,
    BoxAlign,
    BoxPack,
    BoxLines,
    FlexPack,
    FlexItemAlign,
    FlexLinePack,
    AutoFlowDirection,
    AbsoluteFontSize,
    RelativeFontSize,
    FontStretchKeyword,
    FontVariantCaps,
    VerticalAlignKeyword,
    StepPosition,
    AnimationDirection,
    AnimationPlayState,
    AnimationFillMode,
    AnimationComposition,
    ScrollAxis,
    Scroller,
    TimelineRangeName,
    TransformStyle,
    TransformBox,
    BackfaceVisibility,
    TextTransformCase,
    WhiteSpace,
    WordBreak,
    LineBreak,
    Hyphens,
    OverflowWrap,
    TextAlign,
    TextAlignLast,
    TextJustify,
    ExclusiveTextDecorationLine,
    OtherTextDecorationLine,
    TextDecorationStyle,
    TextDecorationSkipInk,
    TextEmphasisFillMode,
    TextEmphasisShape,
    TextEmphasisPositionHorizontal,
    TextEmphasisPositionVertical,
    TextDirection,
    UnicodeBidi,
    BoxDecorationBreak,
    Resize,
    CursorKeyword,
    CaretShape,
    UserSelect,
    PointerEvents,
    Float,
    Clear,
    TouchAction,
    ScrollBehavior,
    SymbolsType,
    PredefinedCounterStyle,
    ListStylePosition,
    MarkerSide,
    FillRule,
    StrokeLinecap,
    StrokeLinejoin,
    ColorInterpolation,
    ColorRendering,
    ShapeRendering,
    TextRendering,
    ImageRendering,
    GeometryBox,
    MaskMode,
    MaskComposite,
    MaskType,
    MaskBorderMode,
    WebKitMaskComposite,
    WebKitMaskSourceType,
    ContainerType,
    PrintColorAdjust,
    CSSWideKeyword,
}

impl<'ast, 'ghost> ToCss<'ghost> for ScrollbarColor<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(cx.ast()) {
            ScrollbarColorData::Auto => dest.write_str("auto"),
            ScrollbarColorData::Colors(first, second) => {
                first.to_css(dest, cx)?;
                dest.write_char(' ')?;
                second.to_css(dest, cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Content<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = cx.ast().data(*self);
        crate::token::write_token_list(data.value, dest, cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Image<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ImageData::None => dest.write_str("none"),
            ImageData::Url(value) => value.to_css(dest, _cx),
            ImageData::Gradient(value) => value.to_css(dest, _cx),
            ImageData::ImageSet(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_gradient_items<'ast, 'ghost, PrinterT: PrinterTrait, D: ToCss<'ghost> + Copy>(
    items: &[GradientItem<'ast, D>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        item.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for Gradient<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            GradientData::Linear {
                direction,
                items,
                vendor_prefix,
            }
            | GradientData::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str(
                    if matches!(self.data(_cx.ast()), GradientData::RepeatingLinear { .. }) {
                        "repeating-linear-gradient("
                    } else {
                        "linear-gradient("
                    },
                )?;
                if !matches!(
                    direction,
                    LineDirection::Vertical(VerticalPositionKeyword::Bottom)
                ) {
                    direction.to_css(dest, _cx)?;
                    dest.delim(Delimiter::Comma)?;
                }
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            GradientData::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            }
            | GradientData::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str(
                    if matches!(self.data(_cx.ast()), GradientData::RepeatingRadial { .. }) {
                        "repeating-radial-gradient("
                    } else {
                        "radial-gradient("
                    },
                )?;
                shape.to_css(dest, _cx)?;
                dest.write_str(" at ")?;
                position.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            GradientData::Conic {
                angle,
                items,
                position,
            }
            | GradientData::RepeatingConic {
                angle,
                items,
                position,
            } => {
                dest.write_str(
                    if matches!(self.data(_cx.ast()), GradientData::RepeatingConic { .. }) {
                        "repeating-conic-gradient(from "
                    } else {
                        "conic-gradient(from "
                    },
                )?;
                angle.to_css(dest, _cx)?;
                dest.write_str(" at ")?;
                position.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            GradientData::WebKitGradient(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for WebKitGradient<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("-webkit-gradient(")?;
        match self.data(_cx.ast()) {
            WebKitGradientData::Linear { from, to, stops } => {
                dest.write_str("linear, ")?;
                from.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest, _cx)?;
                for stop in stops {
                    dest.delim(Delimiter::Comma)?;
                    stop.to_css(dest, _cx)?;
                }
            }
            WebKitGradientData::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => {
                dest.write_str("radial, ")?;
                from.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(start_radius, dest)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(end_radius, dest)?;
                for stop in stops {
                    dest.delim(Delimiter::Comma)?;
                    stop.to_css(dest, _cx)?;
                }
            }
        }
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for LineDirection {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Angle(value) => value.to_css(dest, _cx),
            Self::Horizontal(value) => {
                dest.write_str("to ")?;
                value.to_css(dest, _cx)
            }
            Self::Vertical(value) => {
                dest.write_str("to ")?;
                value.to_css(dest, _cx)
            }
            Self::Corner {
                horizontal,
                vertical,
            } => {
                dest.write_str("to ")?;
                horizontal.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                vertical.to_css(dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost, D: ToCss<'ghost> + Copy> ToCss<'ghost> for GradientItem<'ast, D> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            GradientItemData::ColorStop { color, position } => {
                color.to_css(dest, _cx)?;
                if let Some(position) = position {
                    dest.write_char(' ')?;
                    position.to_css(dest, _cx)?;
                }
                Ok(())
            }
            GradientItemData::Hint(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost, D: ToCss<'ghost> + Copy> ToCss<'ghost> for DimensionPercentage<'ast, D> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            DimensionPercentageData::Dimension(value) => value.to_css(dest, _cx),
            DimensionPercentageData::Percentage(value) => {
                serialize_number(value * 100.0, dest)?;
                dest.write_char('%')
            }
            DimensionPercentageData::Zero => dest.write_char('0'),
            DimensionPercentageData::Calc(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost, S: ToCss<'ghost> + Copy> ToCss<'ghost> for PositionComponent<'ast, S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            PositionComponentData::Center => dest.write_str("center"),
            PositionComponentData::Length(value) => value.to_css(dest, _cx),
            PositionComponentData::Side { offset, side } => {
                side.to_css(dest, _cx)?;
                if let Some(offset) = offset {
                    dest.write_char(' ')?;
                    offset.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for EndingShape<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            EndingShapeData::Ellipse(value) => value.to_css(dest, _cx),
            EndingShapeData::Circle(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Ellipse<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("ellipse")?;
        match self.data(_cx.ast()) {
            EllipseData::Size { x, y } => {
                dest.write_char(' ')?;
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)
            }
            EllipseData::Extent(value) => {
                dest.write_char(' ')?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Circle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("circle")?;
        dest.write_char(' ')?;
        match self.data(_cx.ast()) {
            CircleData::Radius(value) => value.to_css(dest, _cx),
            CircleData::Extent(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost, S: ToCss<'ghost>> ToCss<'ghost> for WebKitGradientPointComponent<S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Center => dest.write_str("center"),
            Self::Number(value) => value.to_css(dest, _cx),
            Self::Side(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for NumberOrPercentage {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BackgroundSize<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            BackgroundSizeData::Explicit { height, width } => {
                width.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                height.to_css(dest, _cx)
            }
            BackgroundSizeData::Cover => dest.write_str("cover"),
            BackgroundSizeData::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for LengthPercentageOrAuto<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            LengthPercentageOrAutoData::Auto => dest.write_str("auto"),
            LengthPercentageOrAutoData::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Display<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            DisplayData::Keyword(value) => value.to_css(dest, _cx),
            DisplayData::Pair {
                inside,
                is_list_item,
                outside,
            } => {
                if is_list_item
                    && matches!(outside, DisplayOutside::Block)
                    && matches!(inside.data(_cx.ast()), DisplayInsideData::Flow)
                {
                    return dest.write_str("list-item");
                }
                match (outside, inside.data(_cx.ast())) {
                    (DisplayOutside::Block, DisplayInsideData::Flow) => dest.write_str("block")?,
                    (DisplayOutside::Inline, DisplayInsideData::Flow) => {
                        dest.write_str("inline")?
                    }
                    (DisplayOutside::Block, DisplayInsideData::FlowRoot) => {
                        dest.write_str("flow-root")?
                    }
                    (DisplayOutside::Inline, DisplayInsideData::FlowRoot) => {
                        dest.write_str("inline-block")?
                    }
                    (DisplayOutside::Block, DisplayInsideData::Flex { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Inline, DisplayInsideData::Flex { vendor_prefix }) => {
                        dest.write_str("inline-")?;
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Block, DisplayInsideData::Box { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("box")?;
                    }
                    (DisplayOutside::Inline, DisplayInsideData::Box { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("inline-box")?;
                    }
                    (DisplayOutside::Block, DisplayInsideData::Grid) => dest.write_str("grid")?,
                    (DisplayOutside::Inline, DisplayInsideData::Grid) => {
                        dest.write_str("inline-grid")?
                    }
                    (DisplayOutside::Inline, DisplayInsideData::Ruby) => dest.write_str("ruby")?,
                    (DisplayOutside::Block, DisplayInsideData::Table) => dest.write_str("table")?,
                    (DisplayOutside::Inline, DisplayInsideData::Table) => {
                        dest.write_str("inline-table")?
                    }
                    _ => {
                        outside.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                        inside.to_css(dest, _cx)?;
                    }
                }
                if is_list_item {
                    dest.write_str(" list-item")?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for DisplayInside<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            DisplayInsideData::Flow => dest.write_str("flow"),
            DisplayInsideData::FlowRoot => dest.write_str("flow-root"),
            DisplayInsideData::Table => dest.write_str("table"),
            DisplayInsideData::Flex { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("flex")
            }
            DisplayInsideData::Box { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("box")
            }
            DisplayInsideData::Grid => dest.write_str("grid"),
            DisplayInsideData::Ruby => dest.write_str("ruby"),
        }
    }
}

fn write_prefixed_keyword<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    value: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    prefix.to_css(dest, cx)?;
    dest.write_str(value)
}

impl<'ast, 'ghost> ToCss<'ghost> for Size<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SizeData::Auto => dest.write_str("auto"),
            SizeData::LengthPercentage(value) => value.to_css(dest, _cx),
            SizeData::MathFunction(value) => value.to_css(dest, _cx),
            SizeData::MinContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "min-content", dest, _cx)
            }
            SizeData::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "max-content", dest, _cx)
            }
            SizeData::FitContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "fit-content", dest, _cx)
            }
            SizeData::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            SizeData::Stretch { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "stretch", dest, _cx)
            }
            SizeData::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for MaxSize<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            MaxSizeData::None => dest.write_str("none"),
            MaxSizeData::LengthPercentage(value) => value.to_css(dest, _cx),
            MaxSizeData::MathFunction(value) => value.to_css(dest, _cx),
            MaxSizeData::MinContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "min-content", dest, _cx)
            }
            MaxSizeData::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "max-content", dest, _cx)
            }
            MaxSizeData::FitContent { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "fit-content", dest, _cx)
            }
            MaxSizeData::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            MaxSizeData::Stretch { vendor_prefix } => {
                write_prefixed_keyword(&vendor_prefix, "stretch", dest, _cx)
            }
            MaxSizeData::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for PositionProperty {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Static => dest.write_str("static"),
            Self::Relative => dest.write_str("relative"),
            Self::Absolute => dest.write_str("absolute"),
            Self::Sticky(prefix) => write_prefixed_keyword(prefix, "sticky", dest, _cx),
            Self::Fixed => dest.write_str("fixed"),
        }
    }
}

impl<'ast, 'ghost, T: ToCss<'ghost> + PartialEq + Copy> ToCss<'ghost> for Size2D<'ast, T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.field_0.to_css(dest, _cx)?;
        if data.field_0 != data.field_1 {
            dest.write_char(' ')?;
            data.field_1.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost, T: ToCss<'ghost> + PartialEq + Copy> ToCss<'ghost> for Rect<'ast, T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        data.field_0.to_css(dest, _cx)?;
        if data.field_0 == data.field_1
            && data.field_0 == data.field_2
            && data.field_0 == data.field_3
        {
            return Ok(());
        }
        dest.write_char(' ')?;
        data.field_1.to_css(dest, _cx)?;
        if data.field_0 == data.field_2 && data.field_1 == data.field_3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        data.field_2.to_css(dest, _cx)?;
        if data.field_1 != data.field_3 {
            dest.write_char(' ')?;
            data.field_3.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderSideWidth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            BorderSideWidthData::Thin => dest.write_str("thin"),
            BorderSideWidthData::Medium => dest.write_str("medium"),
            BorderSideWidthData::Thick => dest.write_str("thick"),
            BorderSideWidthData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for LengthOrNumber<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            LengthOrNumberData::Number(value) => serialize_number(value, dest),
            LengthOrNumberData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BorderImageSideWidth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            BorderImageSideWidthData::Number(value) => serialize_number(value, dest),
            BorderImageSideWidthData::LengthPercentage(value) => value.to_css(dest, _cx),
            BorderImageSideWidthData::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for OutlineStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            OutlineStyleData::Auto => dest.write_str("auto"),
            OutlineStyleData::LineStyle(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_overflow_position<'ghost, PrinterT: PrinterTrait>(
    overflow: &Option<OverflowPosition>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    if let Some(overflow) = overflow {
        overflow.to_css(dest, cx)?;
        dest.write_char(' ')?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for AlignContent {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::ContentDistribution(value) => value.to_css(dest, _cx),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for JustifyContent {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::ContentDistribution(value) => value.to_css(dest, _cx),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
        }
    }
}

macro_rules! self_alignment {
    ($ty:ty, $dest:ident, $($extra:pat => $body:expr),* $(,)?) => {
        impl<'ghost> ToCss<'ghost> for $ty {
            fn to_css<PrinterT: PrinterTrait>(
                &self,
                $dest: &mut PrinterT,
                _cx: &ToCssContext<'_, '_, '_, 'ghost>,
            ) -> fmt::Result {
                match self {
                    Self::Normal => $dest.write_str("normal"),
                    Self::Stretch => $dest.write_str("stretch"),
                    Self::BaselinePosition(value) => {
                        value.to_css($dest, _cx)?;
                        $dest.write_str(" baseline")
                    }
                    Self::SelfPosition { overflow, value } => {
                        write_overflow_position(overflow, $dest, _cx)?;
                        value.to_css($dest, _cx)
                    }
                    $($extra => $body,)*
                }
            }
        }
    };
}

self_alignment!(AlignSelf, dest, Self::Auto => dest.write_str("auto"));
self_alignment!(AlignItems, dest,);

impl<'ghost> ToCss<'ghost> for JustifySelf {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for JustifyItems {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest, _cx)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest, _cx)?;
                value.to_css(dest, _cx)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest, _cx)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
            Self::Legacy(value) => {
                dest.write_str("legacy ")?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for GapValue<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            GapValueData::Normal => dest.write_str("normal"),
            GapValueData::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

pub(crate) fn write_line_names<PrinterT: PrinterTrait>(
    names: &[&str],
    dest: &mut PrinterT,
) -> fmt::Result {
    if names.is_empty() {
        return Ok(());
    }
    dest.write_char('[')?;
    for (index, name) in names.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        serialize_identifier(name, dest)?;
    }
    dest.write_char(']')
}

impl<'ast, 'ghost> ToCss<'ghost> for TrackSizing<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TrackSizingData::None => dest.write_str("none"),
            TrackSizingData::TrackList { items, line_names } => {
                let mut wrote_value = false;
                for (index, item) in items.iter().enumerate() {
                    if let Some(names) = line_names.get(index)
                        && !names.is_empty()
                    {
                        if wrote_value {
                            dest.write_char(' ')?;
                        }
                        write_line_names(names, dest)?;
                        wrote_value = true;
                    }
                    if wrote_value {
                        dest.write_char(' ')?;
                    }
                    item.to_css(dest, _cx)?;
                    wrote_value = true;
                }
                if let Some(names) = line_names.get(items.len())
                    && !names.is_empty()
                {
                    if wrote_value {
                        dest.write_char(' ')?;
                    }
                    write_line_names(names, dest)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TrackListItem<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TrackListItemData::TrackSize(value) => value.to_css(dest, _cx),
            TrackListItemData::TrackRepeat(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TrackSize<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TrackSizeData::TrackBreadth(value) => value.to_css(dest, _cx),
            TrackSizeData::MinMax { max, min } => {
                dest.write_str("minmax(")?;
                min.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            TrackSizeData::FitContent(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TrackBreadth<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TrackBreadthData::Length(value) => value.to_css(dest, _cx),
            TrackBreadthData::Flex(value) => {
                serialize_number(value, dest)?;
                dest.write_str("fr")
            }
            TrackBreadthData::MinContent => dest.write_str("min-content"),
            TrackBreadthData::MaxContent => dest.write_str("max-content"),
            TrackBreadthData::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for RepeatCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::AutoFill => dest.write_str("auto-fill"),
            Self::AutoFit => dest.write_str("auto-fit"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for GridTemplateAreas<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            GridTemplateAreasData::None => dest.write_str("none"),
            GridTemplateAreasData::Areas { areas, columns } => {
                let columns = columns as usize;
                if columns == 0 {
                    return Ok(());
                }
                for (row, values) in areas.chunks(columns).enumerate() {
                    if row > 0 {
                        dest.write_char(' ')?;
                    }
                    let mut output = String::new();
                    for (column, value) in values.iter().enumerate() {
                        if column > 0 {
                            output.push(' ');
                        }
                        output.push_str(value.unwrap_or("."));
                    }
                    serialize_string(&output, dest)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for GridLine<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            GridLineData::Auto => dest.write_str("auto"),
            GridLineData::Area { name } => serialize_identifier(name, dest),
            GridLineData::Line { index, name } => {
                if index != 0 {
                    serialize_int(index, dest)?;
                    if name.is_some() {
                        dest.write_char(' ')?;
                    }
                }
                if let Some(name) = name {
                    serialize_identifier(name, dest)?;
                }
                Ok(())
            }
            GridLineData::Span { index, name } => {
                dest.write_str("span")?;
                if index != 0 {
                    dest.write_char(' ')?;
                    serialize_int(index, dest)?;
                }
                if let Some(name) = name {
                    dest.write_char(' ')?;
                    serialize_identifier(name, dest)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontWeight {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Absolute(value) => value.to_css(dest, _cx),
            Self::Bolder => dest.write_str("bolder"),
            Self::Lighter => dest.write_str("lighter"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AbsoluteFontWeight {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Weight(value) => serialize_number(*value, dest),
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FontSize<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FontSizeData::Length(value) => value.to_css(dest, _cx),
            FontSizeData::Absolute(value) => value.to_css(dest, _cx),
            FontSizeData::Relative(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontStretch {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(value) => {
                dest.write_str("oblique")?;
                dest.write_char(' ')?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for LineHeight<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            LineHeightData::Normal => dest.write_str("normal"),
            LineHeightData::Number(value) => serialize_number(value, dest),
            LineHeightData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for VerticalAlign<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            VerticalAlignData::Keyword(value) => value.to_css(dest, _cx),
            VerticalAlignData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for EasingFunction {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Linear => dest.write_str("linear"),
            Self::Ease => dest.write_str("ease"),
            Self::EaseIn => dest.write_str("ease-in"),
            Self::EaseOut => dest.write_str("ease-out"),
            Self::EaseInOut => dest.write_str("ease-in-out"),
            Self::CubicBezier { x1, x2, y1, y2 } => {
                if (*x1, *y1, *x2, *y2) == (0.0, 0.0, 1.0, 1.0) {
                    return dest.write_str("linear");
                }
                if (*x1, *y1, *x2, *y2) == (0.25, 0.1, 0.25, 1.0) {
                    return dest.write_str("ease");
                }
                if (*x1, *y1, *x2, *y2) == (0.42, 0.0, 1.0, 1.0) {
                    return dest.write_str("ease-in");
                }
                if (*x1, *y1, *x2, *y2) == (0.0, 0.0, 0.58, 1.0) {
                    return dest.write_str("ease-out");
                }
                if (*x1, *y1, *x2, *y2) == (0.42, 0.0, 0.58, 1.0) {
                    return dest.write_str("ease-in-out");
                }
                dest.write_str("cubic-bezier(")?;
                for (index, value) in [x1, y1, x2, y2].into_iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_number(*value, dest)?;
                }
                dest.write_char(')')
            }
            Self::Frames(count) => {
                dest.write_str("frames(")?;
                serialize_int(*count, dest)?;
                dest.write_char(')')
            }
            Self::Steps { count, position } => {
                if *count == 1 {
                    match position {
                        StepPosition::Start => return dest.write_str("step-start"),
                        StepPosition::End => return dest.write_str("step-end"),
                        _ => {}
                    }
                }
                dest.write_str("steps(")?;
                serialize_int(*count, dest)?;
                if !matches!(position, StepPosition::End) {
                    dest.delim(Delimiter::Comma)?;
                    position.to_css(dest, _cx)?;
                }
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationIterationCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Infinite => dest.write_str("infinite"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for AnimationTimeline<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            AnimationTimelineData::Auto => dest.write_str("auto"),
            AnimationTimelineData::None => dest.write_str("none"),
            AnimationTimelineData::DashedIdent(value) => {
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            AnimationTimelineData::Scroll(value) => {
                dest.write_str("scroll(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            AnimationTimelineData::View(value) => {
                dest.write_str("view(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for AnimationAttachmentRange<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            AnimationAttachmentRangeData::Normal => dest.write_str("normal"),
            AnimationAttachmentRangeData::LengthPercentage(value) => value.to_css(dest, _cx),
            AnimationAttachmentRangeData::TimelineRange { name, offset } => {
                name.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                offset.to_css(dest, _cx)
            }
        }
    }
}

fn write_function_values<PrinterT: PrinterTrait, F>(
    name: &str,
    dest: &mut PrinterT,
    callback: F,
) -> fmt::Result
where
    F: FnOnce(&mut PrinterT) -> fmt::Result,
{
    dest.write_str(name)?;
    dest.write_char('(')?;
    callback(dest)?;
    dest.write_char(')')
}

fn write_comma_values<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[&T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for Transform<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TransformData::Translate((x, y)) => write_function_values("translate", dest, |dest| {
                write_comma_values(&[&x, &y], dest, _cx)
            }),
            TransformData::TranslateX(value) => {
                write_function_values("translateX", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::TranslateY(value) => {
                write_function_values("translateY", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::TranslateZ(value) => {
                write_function_values("translateZ", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::Translate3d((x, y, z)) => {
                write_function_values("translate3d", dest, |dest| {
                    x.to_css(dest, _cx)?;
                    dest.delim(Delimiter::Comma)?;
                    y.to_css(dest, _cx)?;
                    dest.delim(Delimiter::Comma)?;
                    z.to_css(dest, _cx)
                })
            }
            TransformData::Scale((x, y)) => write_function_values("scale", dest, |dest| {
                write_comma_values(&[&x, &y], dest, _cx)
            }),
            TransformData::ScaleX(value) => {
                write_function_values("scaleX", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::ScaleY(value) => {
                write_function_values("scaleY", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::ScaleZ(value) => {
                write_function_values("scaleZ", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::Scale3d((x, y, z)) => write_function_values("scale3d", dest, |dest| {
                x.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                z.to_css(dest, _cx)
            }),
            TransformData::Rotate(value) => {
                write_function_values("rotate", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::RotateX(value) => {
                write_function_values("rotateX", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::RotateY(value) => {
                write_function_values("rotateY", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::RotateZ(value) => {
                write_function_values("rotateZ", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::Rotate3d((x, y, z, angle)) => {
                write_function_values("rotate3d", dest, |dest| {
                    for value in [x, y, z] {
                        serialize_number(value, dest)?;
                        dest.delim(Delimiter::Comma)?;
                    }
                    angle.to_css(dest, _cx)
                })
            }
            TransformData::Skew((x, y)) => write_function_values("skew", dest, |dest| {
                write_comma_values(&[&x, &y], dest, _cx)
            }),
            TransformData::SkewX(value) => {
                write_function_values("skewX", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::SkewY(value) => {
                write_function_values("skewY", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::Perspective(value) => {
                write_function_values("perspective", dest, |dest| value.to_css(dest, _cx))
            }
            TransformData::Matrix(value) => value.to_css(dest, _cx),
            TransformData::Matrix3d(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Perspective<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            PerspectiveData::None => dest.write_str("none"),
            PerspectiveData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Translate<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TranslateData::None => dest.write_str("none"),
            TranslateData::Xyz { x, y, z } => {
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                z.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Scale {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Xyz { x, y, z } => {
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                z.to_css(dest, _cx)
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Spacing<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SpacingData::Normal => dest.write_str("normal"),
            SpacingData::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextDecorationLine<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TextDecorationLineData::ExclusiveTextDecorationLine(value) => value.to_css(dest, _cx),
            TextDecorationLineData::Value(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextDecorationThickness<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TextDecorationThicknessData::Auto => dest.write_str("auto"),
            TextDecorationThicknessData::FromFont => dest.write_str("from-font"),
            TextDecorationThicknessData::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for TextEmphasisStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TextEmphasisStyleData::None => dest.write_str("none"),
            TextEmphasisStyleData::Keyword { fill, shape } => {
                fill.to_css(dest, _cx)?;
                if let Some(shape) = shape {
                    dest.write_char(' ')?;
                    shape.to_css(dest, _cx)?;
                }
                Ok(())
            }
            TextEmphasisStyleData::String(value) => serialize_string(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextSizeAdjust {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::None => dest.write_str("none"),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ColorOrAuto<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ColorOrAutoData::Auto => dest.write_str("auto"),
            ColorOrAutoData::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Appearance<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::NonStandard(value) => dest.write_str(value),
            value => dest.write_str(
                value
                    .as_css_str()
                    .expect("non-standard appearance handled separately"),
            ),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ListStyleType<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ListStyleTypeData::None => dest.write_str("none"),
            ListStyleTypeData::String(value) => serialize_string(value, dest),
            ListStyleTypeData::CounterStyle(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for CounterStyle<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            CounterStyleData::Predefined(value) => value.to_css(dest, _cx),
            CounterStyleData::Name(value) => serialize_identifier(value, dest),
            CounterStyleData::Symbols { symbols, system } => {
                dest.write_str("symbols(")?;
                if !matches!(system, SymbolsType::Symbolic) {
                    system.to_css(dest, _cx)?;
                    dest.write_char(' ')?;
                }
                for (index, symbol) in symbols.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    symbol.to_css(dest, _cx)?;
                }
                dest.write_char(')')
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Symbol<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SymbolData::String(value) => serialize_string(value, dest),
            SymbolData::Image(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SVGPaint<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SVGPaintData::Url { fallback, url } => {
                url.to_css(dest, _cx)?;
                if let Some(fallback) = fallback {
                    dest.write_char(' ')?;
                    fallback.to_css(dest, _cx)?;
                }
                Ok(())
            }
            SVGPaintData::Color(value) => value.to_css(dest, _cx),
            SVGPaintData::ContextFill => dest.write_str("context-fill"),
            SVGPaintData::ContextStroke => dest.write_str("context-stroke"),
            SVGPaintData::None => dest.write_str("none"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for SVGPaintFallback<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SVGPaintFallbackData::None => dest.write_str("none"),
            SVGPaintFallbackData::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for StrokeDasharray<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            StrokeDasharrayData::None => dest.write_str("none"),
            StrokeDasharrayData::Values(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Marker<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            MarkerData::None => dest.write_str("none"),
            MarkerData::Url(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ClipPath<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ClipPathData::None => dest.write_str("none"),
            ClipPathData::Url(value) => value.to_css(dest, _cx),
            ClipPathData::Shape {
                reference_box,
                shape,
            } => {
                shape.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                reference_box.to_css(dest, _cx)
            }
            ClipPathData::Box(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for BasicShape<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            BasicShapeData::Inset(value) => value.to_css(dest, _cx),
            BasicShapeData::Circle(value) => value.to_css(dest, _cx),
            BasicShapeData::Ellipse(value) => value.to_css(dest, _cx),
            BasicShapeData::Polygon(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ShapeRadius<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ShapeRadiusData::LengthPercentage(value) => value.to_css(dest, _cx),
            ShapeRadiusData::ClosestSide => dest.write_str("closest-side"),
            ShapeRadiusData::FarthestSide => dest.write_str("farthest-side"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MaskClip {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::GeometryBox(value) => value.to_css(dest, _cx),
            Self::NoClip => dest.write_str("no-clip"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for FilterList<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FilterListData::None => dest.write_str("none"),
            FilterListData::Filters(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Filter<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FilterData::Blur(value) => {
                write_function_values("blur", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Brightness(value) => {
                write_function_values("brightness", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Contrast(value) => {
                write_function_values("contrast", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Grayscale(value) => {
                write_function_values("grayscale", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::HueRotate(value) => {
                write_function_values("hue-rotate", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Invert(value) => {
                write_function_values("invert", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Opacity(value) => {
                write_function_values("opacity", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Saturate(value) => {
                write_function_values("saturate", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Sepia(value) => {
                write_function_values("sepia", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::DropShadow(value) => {
                write_function_values("drop-shadow", dest, |dest| value.to_css(dest, _cx))
            }
            FilterData::Url(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ZIndex {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ContainerNameList<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            ContainerNameListData::None => dest.write_str("none"),
            ContainerNameListData::Names(values) => write_ident_list(values, dest),
        }
    }
}

fn write_ident_list<PrinterT: PrinterTrait>(values: &[&str], dest: &mut PrinterT) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        serialize_identifier(value, dest)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for ViewTransitionName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Auto => dest.write_str("auto"),
            Self::Custom(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for NoneOrCustomIdentList<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            NoneOrCustomIdentListData::None => dest.write_str("none"),
            NoneOrCustomIdentListData::Idents(values) => write_ident_list(values, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ViewTransitionGroup<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Contain => dest.write_str("contain"),
            Self::Nearest => dest.write_str("nearest"),
            Self::Custom(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for CustomPropertyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let value = match self {
            Self::Custom(value) | Self::Unknown(value) => value,
        };
        dest.write_str("--")?;
        serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
    }
}

use crate::prelude::*;

pub(crate) mod font;

macro_rules! keyword_values {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<'ghost> ToCss<'ghost> for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT, _cx: &ToCssContext<'_, 'ghost>) -> fmt::Result {
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

impl<'ghost> ToCss<'ghost> for Image<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Gradient(value) => value.to_css(dest, _cx),
            Self::ImageSet(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_gradient_items<'ghost, PrinterT: PrinterTrait, D: ToCss<'ghost>>(
    items: &[GradientItem<'_, D>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, 'ghost>,
) -> fmt::Result {
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        item.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Gradient<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Linear {
                direction,
                items,
                vendor_prefix,
            }
            | Self::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str(if matches!(self, Self::RepeatingLinear { .. }) {
                    "repeating-linear-gradient("
                } else {
                    "linear-gradient("
                })?;
                direction.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            Self::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            }
            | Self::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str(if matches!(self, Self::RepeatingRadial { .. }) {
                    "repeating-radial-gradient("
                } else {
                    "radial-gradient("
                })?;
                shape.to_css(dest, _cx)?;
                dest.write_str(" at ")?;
                position.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            Self::Conic {
                angle,
                items,
                position,
            }
            | Self::RepeatingConic {
                angle,
                items,
                position,
            } => {
                dest.write_str(if matches!(self, Self::RepeatingConic { .. }) {
                    "repeating-conic-gradient(from "
                } else {
                    "conic-gradient(from "
                })?;
                angle.to_css(dest, _cx)?;
                dest.write_str(" at ")?;
                position.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest, _cx)?;
                dest.write_char(')')
            }
            Self::WebKitGradient(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for WebKitGradient<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("-webkit-gradient(")?;
        match self {
            Self::Linear { from, to, stops } => {
                dest.write_str("linear, ")?;
                from.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest, _cx)?;
                for stop in stops {
                    dest.delim(Delimiter::Comma)?;
                    stop.to_css(dest, _cx)?;
                }
            }
            Self::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => {
                dest.write_str("radial, ")?;
                from.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(*start_radius, dest)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(*end_radius, dest)?;
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
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost, D: ToCss<'ghost>> ToCss<'ghost> for GradientItem<'_, D> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::ColorStop { color, position } => {
                color.to_css(dest, _cx)?;
                if let Some(position) = position {
                    dest.write_char(' ')?;
                    position.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Hint(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost, D: ToCss<'ghost>> ToCss<'ghost> for DimensionPercentage<'_, D> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Dimension(value) => value.to_css(dest, _cx),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::Zero => dest.write_char('0'),
            Self::Calc(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost, S: ToCss<'ghost>> ToCss<'ghost> for PositionComponent<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Center => dest.write_str("center"),
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Side { offset, side } => {
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

impl<'ghost> ToCss<'ghost> for EndingShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Ellipse(value) => value.to_css(dest, _cx),
            Self::Circle(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Ellipse<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("ellipse")?;
        match self {
            Self::Size { x, y } => {
                dest.write_char(' ')?;
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)
            }
            Self::Extent(value) => {
                dest.write_char(' ')?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Circle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("circle")?;
        dest.write_char(' ')?;
        match self {
            Self::Radius(value) => value.to_css(dest, _cx),
            Self::Extent(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost, S: ToCss<'ghost>> ToCss<'ghost> for WebKitGradientPointComponent<S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for BackgroundSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Explicit { height, width } => {
                width.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                height.to_css(dest, _cx)
            }
            Self::Cover => dest.write_str("cover"),
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for LengthPercentageOrAuto<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Display {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Pair {
                inside,
                is_list_item,
                outside,
            } => {
                if *is_list_item
                    && matches!(outside, DisplayOutside::Block)
                    && matches!(inside, DisplayInside::Flow)
                {
                    return dest.write_str("list-item");
                }
                match (outside, inside) {
                    (DisplayOutside::Block, DisplayInside::Flow) => dest.write_str("block")?,
                    (DisplayOutside::Inline, DisplayInside::Flow) => dest.write_str("inline")?,
                    (DisplayOutside::Block, DisplayInside::FlowRoot) => {
                        dest.write_str("flow-root")?
                    }
                    (DisplayOutside::Block, DisplayInside::Flex { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Inline, DisplayInside::Flex { vendor_prefix }) => {
                        dest.write_str("inline-")?;
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Block, DisplayInside::Grid) => dest.write_str("grid")?,
                    (DisplayOutside::Inline, DisplayInside::Grid) => {
                        dest.write_str("inline-grid")?
                    }
                    (DisplayOutside::Block, DisplayInside::Table) => dest.write_str("table")?,
                    (DisplayOutside::Inline, DisplayInside::Table) => {
                        dest.write_str("inline-table")?
                    }
                    _ => {
                        outside.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                        inside.to_css(dest, _cx)?;
                    }
                }
                if *is_list_item {
                    dest.write_str(" list-item")?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for DisplayInside {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Flow => dest.write_str("flow"),
            Self::FlowRoot => dest.write_str("flow-root"),
            Self::Table => dest.write_str("table"),
            Self::Flex { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("flex")
            }
            Self::Box { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("box")
            }
            Self::Grid => dest.write_str("grid"),
            Self::Ruby => dest.write_str("ruby"),
        }
    }
}

fn write_prefixed_keyword<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    value: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, 'ghost>,
) -> fmt::Result {
    prefix.to_css(dest, cx)?;
    dest.write_str(value)
}

impl<'ghost> ToCss<'ghost> for Size<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest, _cx)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest, _cx)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest, _cx)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest, _cx)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MaxSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest, _cx)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest, _cx)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest, _cx)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest, _cx)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for PositionProperty {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost, T: ToCss<'ghost> + PartialEq> ToCss<'ghost> for Size2D<'_, T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        self.0.to_css(dest, _cx)?;
        if self.0 != self.1 {
            dest.write_char(' ')?;
            self.1.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost, T: ToCss<'ghost> + PartialEq> ToCss<'ghost> for Rect<'_, T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        self.0.to_css(dest, _cx)?;
        if self.0 == self.1 && self.0 == self.2 && self.0 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.1.to_css(dest, _cx)?;
        if self.0 == self.2 && self.1 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.2.to_css(dest, _cx)?;
        if self.1 != self.3 {
            dest.write_char(' ')?;
            self.3.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BorderSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Thin => dest.write_str("thin"),
            Self::Medium => dest.write_str("medium"),
            Self::Thick => dest.write_str("thick"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for LengthOrNumber<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for OutlineStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LineStyle(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_overflow_position<'ghost, PrinterT: PrinterTrait>(
    overflow: &Option<OverflowPosition>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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
                _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for GapValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
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

impl<'ghost> ToCss<'ghost> for TrackSizing<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::TrackList { items, line_names } => {
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

impl<'ghost> ToCss<'ghost> for TrackListItem<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::TrackSize(value) => value.to_css(dest, _cx),
            Self::TrackRepeat(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TrackSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::TrackBreadth(value) => value.to_css(dest, _cx),
            Self::MinMax { max, min } => {
                dest.write_str("minmax(")?;
                min.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::FitContent(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for TrackBreadth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Flex(value) => serialize_dimension(*value, &Unit::Flex, dest, _cx),
            Self::MinContent => dest.write_str("min-content"),
            Self::MaxContent => dest.write_str("max-content"),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for RepeatCount {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::AutoFill => dest.write_str("auto-fill"),
            Self::AutoFit => dest.write_str("auto-fit"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for GridTemplateAreas<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Areas { areas, columns } => {
                let columns = *columns as usize;
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

impl<'ghost> ToCss<'ghost> for GridLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Area { name } => serialize_identifier(name, dest),
            Self::Line { index, name } => {
                if *index != 0 {
                    serialize_int(*index, dest)?;
                    if name.is_some() {
                        dest.write_char(' ')?;
                    }
                }
                if let Some(name) = name {
                    serialize_identifier(name, dest)?;
                }
                Ok(())
            }
            Self::Span { index, name } => {
                dest.write_str("span")?;
                if *index != 0 {
                    dest.write_char(' ')?;
                    serialize_int(*index, dest)?;
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
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Weight(value) => serialize_number(*value, dest),
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Absolute(value) => value.to_css(dest, _cx),
            Self::Relative(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontStretch {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for LineHeight<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for VerticalAlign<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for EasingFunction {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Infinite => dest.write_str("infinite"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::None => dest.write_str("none"),
            Self::DashedIdent(value) => {
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Scroll(value) => {
                dest.write_str("scroll(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::View(value) => {
                dest.write_str("view(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationAttachmentRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::TimelineRange { name, offset } => {
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
    cx: &ToCssContext<'_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Transform<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Translate((x, y)) => write_function_values("translate", dest, |dest| {
                write_comma_values(&[x, y], dest, _cx)
            }),
            Self::TranslateX(value) => {
                write_function_values("translateX", dest, |dest| value.to_css(dest, _cx))
            }
            Self::TranslateY(value) => {
                write_function_values("translateY", dest, |dest| value.to_css(dest, _cx))
            }
            Self::TranslateZ(value) => {
                write_function_values("translateZ", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Translate3d((x, y, z)) => write_function_values("translate3d", dest, |dest| {
                x.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                z.to_css(dest, _cx)
            }),
            Self::Scale((x, y)) => {
                write_function_values("scale", dest, |dest| write_comma_values(&[x, y], dest, _cx))
            }
            Self::ScaleX(value) => {
                write_function_values("scaleX", dest, |dest| value.to_css(dest, _cx))
            }
            Self::ScaleY(value) => {
                write_function_values("scaleY", dest, |dest| value.to_css(dest, _cx))
            }
            Self::ScaleZ(value) => {
                write_function_values("scaleZ", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Scale3d((x, y, z)) => write_function_values("scale3d", dest, |dest| {
                x.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                z.to_css(dest, _cx)
            }),
            Self::Rotate(value) => {
                write_function_values("rotate", dest, |dest| value.to_css(dest, _cx))
            }
            Self::RotateX(value) => {
                write_function_values("rotateX", dest, |dest| value.to_css(dest, _cx))
            }
            Self::RotateY(value) => {
                write_function_values("rotateY", dest, |dest| value.to_css(dest, _cx))
            }
            Self::RotateZ(value) => {
                write_function_values("rotateZ", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Rotate3d((x, y, z, angle)) => write_function_values("rotate3d", dest, |dest| {
                for value in [x, y, z] {
                    serialize_number(*value, dest)?;
                    dest.delim(Delimiter::Comma)?;
                }
                angle.to_css(dest, _cx)
            }),
            Self::Skew((x, y)) => {
                write_function_values("skew", dest, |dest| write_comma_values(&[x, y], dest, _cx))
            }
            Self::SkewX(value) => {
                write_function_values("skewX", dest, |dest| value.to_css(dest, _cx))
            }
            Self::SkewY(value) => {
                write_function_values("skewY", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Perspective(value) => {
                write_function_values("perspective", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Matrix(value) => value.to_css(dest, _cx),
            Self::Matrix3d(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Perspective<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Translate<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for Scale {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for Spacing<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextDecorationLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::ExclusiveTextDecorationLine(value) => value.to_css(dest, _cx),
            Self::Value(values) => {
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

impl<'ghost> ToCss<'ghost> for TextDecorationThickness<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::FromFont => dest.write_str("from-font"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasisStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Keyword { fill, shape } => {
                fill.to_css(dest, _cx)?;
                if let Some(shape) = shape {
                    dest.write_char(' ')?;
                    shape.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::String(value) => serialize_string(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextSizeAdjust {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for ColorOrAuto<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Appearance<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for ListStyleType<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::String(value) => serialize_string(value, dest),
            Self::CounterStyle(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for CounterStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Predefined(value) => value.to_css(dest, _cx),
            Self::Name(value) => serialize_identifier(value, dest),
            Self::Symbols { symbols, system } => {
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

impl<'ghost> ToCss<'ghost> for Symbol<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::String(value) => serialize_string(value, dest),
            Self::Image(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for SVGPaint<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Url { fallback, url } => {
                url.to_css(dest, _cx)?;
                if let Some(fallback) = fallback {
                    dest.write_char(' ')?;
                    fallback.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Color(value) => value.to_css(dest, _cx),
            Self::ContextFill => dest.write_str("context-fill"),
            Self::ContextStroke => dest.write_str("context-stroke"),
            Self::None => dest.write_str("none"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for SVGPaintFallback<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for StrokeDasharray<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Values(values) => {
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

impl<'ghost> ToCss<'ghost> for Marker<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ClipPath<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Shape {
                reference_box,
                shape,
            } => {
                shape.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                reference_box.to_css(dest, _cx)
            }
            Self::Box(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for BasicShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Inset(value) => value.to_css(dest, _cx),
            Self::Circle(value) => value.to_css(dest, _cx),
            Self::Ellipse(value) => value.to_css(dest, _cx),
            Self::Polygon(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ShapeRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::ClosestSide => dest.write_str("closest-side"),
            Self::FarthestSide => dest.write_str("farthest-side"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MaskClip {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::GeometryBox(value) => value.to_css(dest, _cx),
            Self::NoClip => dest.write_str("no-clip"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FilterList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Filters(values) => {
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

impl<'ghost> ToCss<'ghost> for Filter<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Blur(value) => {
                write_function_values("blur", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Brightness(value) => {
                write_function_values("brightness", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Contrast(value) => {
                write_function_values("contrast", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Grayscale(value) => {
                write_function_values("grayscale", dest, |dest| value.to_css(dest, _cx))
            }
            Self::HueRotate(value) => {
                write_function_values("hue-rotate", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Invert(value) => {
                write_function_values("invert", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Opacity(value) => {
                write_function_values("opacity", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Saturate(value) => {
                write_function_values("saturate", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Sepia(value) => {
                write_function_values("sepia", dest, |dest| value.to_css(dest, _cx))
            }
            Self::DropShadow(value) => {
                write_function_values("drop-shadow", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Url(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ZIndex {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ContainerNameList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Names(values) => write_ident_list(values, dest),
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
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Auto => dest.write_str("auto"),
            Self::Custom(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for NoneOrCustomIdentList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Idents(values) => write_ident_list(values, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ViewTransitionGroup<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
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
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        let value = match self {
            Self::Custom(value) | Self::Unknown(value) => value,
        };
        dest.write_str("--")?;
        serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
    }
}

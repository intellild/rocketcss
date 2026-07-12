use crate::prelude::*;

macro_rules! keyword_values {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ToCss for $ty {
                fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
                    serialize_debug_keyword(self, dest)
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
    GenericFontFamily,
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

impl ToCss for Image<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest),
            Self::Gradient(value) => value.to_css(dest),
            Self::ImageSet(value) => value.to_css(dest),
        }
    }
}

fn write_gradient_items<PrinterT: PrinterTrait, D: ToCss>(
    items: &[GradientItem<'_, D>],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        item.to_css(dest)?;
    }
    Ok(())
}

impl ToCss for Gradient<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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
                vendor_prefix.to_css(dest)?;
                dest.write_str(if matches!(self, Self::RepeatingLinear { .. }) {
                    "repeating-linear-gradient("
                } else {
                    "linear-gradient("
                })?;
                direction.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest)?;
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
                vendor_prefix.to_css(dest)?;
                dest.write_str(if matches!(self, Self::RepeatingRadial { .. }) {
                    "repeating-radial-gradient("
                } else {
                    "radial-gradient("
                })?;
                shape.to_css(dest)?;
                dest.write_str(" at ")?;
                position.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest)?;
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
                angle.to_css(dest)?;
                dest.write_str(" at ")?;
                position.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                write_gradient_items(items, dest)?;
                dest.write_char(')')
            }
            Self::WebKitGradient(value) => value.to_css(dest),
        }
    }
}

impl ToCss for WebKitGradient<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("-webkit-gradient(")?;
        match self {
            Self::Linear { from, to, stops } => {
                dest.write_str("linear, ")?;
                from.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest)?;
                for stop in stops {
                    dest.delim(Delimiter::Comma)?;
                    stop.to_css(dest)?;
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
                from.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(*start_radius, dest)?;
                dest.delim(Delimiter::Comma)?;
                to.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                serialize_number(*end_radius, dest)?;
                for stop in stops {
                    dest.delim(Delimiter::Comma)?;
                    stop.to_css(dest)?;
                }
            }
        }
        dest.write_char(')')
    }
}

impl ToCss for LineDirection<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Angle(value) => value.to_css(dest),
            Self::Horizontal(value) => {
                dest.write_str("to ")?;
                value.to_css(dest)
            }
            Self::Vertical(value) => {
                dest.write_str("to ")?;
                value.to_css(dest)
            }
            Self::Corner {
                horizontal,
                vertical,
            } => {
                dest.write_str("to ")?;
                horizontal.to_css(dest)?;
                dest.write_char(' ')?;
                vertical.to_css(dest)
            }
        }
    }
}

impl<D: ToCss> ToCss for GradientItem<'_, D> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::ColorStop { color, position } => {
                color.to_css(dest)?;
                if let Some(position) = position {
                    dest.write_char(' ')?;
                    position.to_css(dest)?;
                }
                Ok(())
            }
            Self::Hint(value) => value.to_css(dest),
        }
    }
}

impl<D: ToCss> ToCss for DimensionPercentage<'_, D> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Dimension(value) => value.to_css(dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::Zero => dest.write_char('0'),
            Self::Calc(value) => value.to_css(dest),
        }
    }
}

impl<S: ToCss> ToCss for PositionComponent<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Center => dest.write_str("center"),
            Self::Length(value) => value.to_css(dest),
            Self::Side { offset, side } => {
                side.to_css(dest)?;
                if let Some(offset) = offset {
                    dest.write_char(' ')?;
                    offset.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for EndingShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Ellipse(value) => value.to_css(dest),
            Self::Circle(value) => value.to_css(dest),
        }
    }
}

impl ToCss for Ellipse<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("ellipse")?;
        match self {
            Self::Size { x, y } => {
                dest.write_char(' ')?;
                x.to_css(dest)?;
                dest.write_char(' ')?;
                y.to_css(dest)
            }
            Self::Extent(value) => {
                dest.write_char(' ')?;
                value.to_css(dest)
            }
        }
    }
}

impl ToCss for Circle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("circle")?;
        dest.write_char(' ')?;
        match self {
            Self::Radius(value) => value.to_css(dest),
            Self::Extent(value) => value.to_css(dest),
        }
    }
}

impl<S: ToCss> ToCss for WebKitGradientPointComponent<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Center => dest.write_str("center"),
            Self::Number(value) => value.to_css(dest),
            Self::Side(value) => value.to_css(dest),
        }
    }
}

impl ToCss for NumberOrPercentage {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl ToCss for BackgroundSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Explicit { height, width } => {
                width.to_css(dest)?;
                dest.write_char(' ')?;
                height.to_css(dest)
            }
            Self::Cover => dest.write_str("cover"),
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl ToCss for LengthPercentageOrAuto<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest),
        }
    }
}

impl ToCss for Display<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest),
            Self::Pair {
                inside,
                is_list_item,
                outside,
            } => {
                if *is_list_item
                    && matches!(outside, DisplayOutside::Block)
                    && matches!(&**inside, DisplayInside::Flow)
                {
                    return dest.write_str("list-item");
                }
                match (outside, &**inside) {
                    (DisplayOutside::Block, DisplayInside::Flow) => dest.write_str("block")?,
                    (DisplayOutside::Inline, DisplayInside::Flow) => dest.write_str("inline")?,
                    (DisplayOutside::Block, DisplayInside::FlowRoot) => {
                        dest.write_str("flow-root")?
                    }
                    (DisplayOutside::Block, DisplayInside::Flex { vendor_prefix }) => {
                        vendor_prefix.to_css(dest)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Inline, DisplayInside::Flex { vendor_prefix }) => {
                        dest.write_str("inline-")?;
                        vendor_prefix.to_css(dest)?;
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
                        outside.to_css(dest)?;
                        dest.write_char(' ')?;
                        inside.to_css(dest)?;
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

impl ToCss for DisplayInside {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Flow => dest.write_str("flow"),
            Self::FlowRoot => dest.write_str("flow-root"),
            Self::Table => dest.write_str("table"),
            Self::Flex { vendor_prefix } => {
                vendor_prefix.to_css(dest)?;
                dest.write_str("flex")
            }
            Self::Box { vendor_prefix } => {
                vendor_prefix.to_css(dest)?;
                dest.write_str("box")
            }
            Self::Grid => dest.write_str("grid"),
            Self::Ruby => dest.write_str("ruby"),
        }
    }
}

fn write_prefixed_keyword<PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    prefix.to_css(dest)?;
    dest.write_str(value)
}

impl ToCss for Size<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl ToCss for MaxSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl ToCss for PositionProperty {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Static => dest.write_str("static"),
            Self::Relative => dest.write_str("relative"),
            Self::Absolute => dest.write_str("absolute"),
            Self::Sticky(prefix) => write_prefixed_keyword(prefix, "sticky", dest),
            Self::Fixed => dest.write_str("fixed"),
        }
    }
}

impl<T: ToCss + PartialEq> ToCss for Size2D<'_, T> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 != self.1 {
            dest.write_char(' ')?;
            self.1.to_css(dest)?;
        }
        Ok(())
    }
}

impl<T: ToCss + PartialEq> ToCss for Rect<'_, T> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        self.0.to_css(dest)?;
        if self.0 == self.1 && self.0 == self.2 && self.0 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.1.to_css(dest)?;
        if self.0 == self.2 && self.1 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.2.to_css(dest)?;
        if self.1 != self.3 {
            dest.write_char(' ')?;
            self.3.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for BorderSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Thin => dest.write_str("thin"),
            Self::Medium => dest.write_str("medium"),
            Self::Thick => dest.write_str("thick"),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for LengthOrNumber<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for BorderImageSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl ToCss for OutlineStyle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LineStyle(value) => value.to_css(dest),
        }
    }
}

fn write_overflow_position<PrinterT: PrinterTrait>(
    overflow: &Option<OverflowPosition>,
    dest: &mut PrinterT,
) -> fmt::Result {
    if let Some(overflow) = overflow {
        overflow.to_css(dest)?;
        dest.write_char(' ')?;
    }
    Ok(())
}

impl ToCss for AlignContent {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::BaselinePosition(value) => {
                value.to_css(dest)?;
                dest.write_str(" baseline")
            }
            Self::ContentDistribution(value) => value.to_css(dest),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest)?;
                value.to_css(dest)
            }
        }
    }
}

impl ToCss for JustifyContent {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::ContentDistribution(value) => value.to_css(dest),
            Self::ContentPosition { overflow, value } => {
                write_overflow_position(overflow, dest)?;
                value.to_css(dest)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest)?;
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
        impl ToCss for $ty {
            fn to_css<PrinterT: PrinterTrait>(&self, $dest: &mut PrinterT) -> fmt::Result {
                match self {
                    Self::Normal => $dest.write_str("normal"),
                    Self::Stretch => $dest.write_str("stretch"),
                    Self::BaselinePosition(value) => {
                        value.to_css($dest)?;
                        $dest.write_str(" baseline")
                    }
                    Self::SelfPosition { overflow, value } => {
                        write_overflow_position(overflow, $dest)?;
                        value.to_css($dest)
                    }
                    $($extra => $body,)*
                }
            }
        }
    };
}

self_alignment!(AlignSelf, dest, Self::Auto => dest.write_str("auto"));
self_alignment!(AlignItems, dest,);

impl ToCss for JustifySelf {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest)?;
                value.to_css(dest)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
        }
    }
}

impl ToCss for JustifyItems {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Stretch => dest.write_str("stretch"),
            Self::BaselinePosition(value) => {
                value.to_css(dest)?;
                dest.write_str(" baseline")
            }
            Self::SelfPosition { overflow, value } => {
                write_overflow_position(overflow, dest)?;
                value.to_css(dest)
            }
            Self::Left { overflow } | Self::Right { overflow } => {
                write_overflow_position(overflow, dest)?;
                dest.write_str(if matches!(self, Self::Left { .. }) {
                    "left"
                } else {
                    "right"
                })
            }
            Self::Legacy(value) => {
                dest.write_str("legacy ")?;
                value.to_css(dest)
            }
        }
    }
}

impl ToCss for GapValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest),
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

impl ToCss for TrackSizing<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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
                    item.to_css(dest)?;
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

impl ToCss for TrackListItem<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::TrackSize(value) => value.to_css(dest),
            Self::TrackRepeat(value) => value.to_css(dest),
        }
    }
}

impl ToCss for TrackSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::TrackBreadth(value) => value.to_css(dest),
            Self::MinMax { max, min } => {
                dest.write_str("minmax(")?;
                min.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest)?;
                dest.write_char(')')
            }
            Self::FitContent(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for TrackBreadth<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest),
            Self::Flex(value) => serialize_dimension(*value, &Unit::Flex, dest),
            Self::MinContent => dest.write_str("min-content"),
            Self::MaxContent => dest.write_str("max-content"),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl ToCss for RepeatCount {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::AutoFill => dest.write_str("auto-fill"),
            Self::AutoFit => dest.write_str("auto-fit"),
        }
    }
}

impl ToCss for GridTemplateAreas<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for GridLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Area { name } => serialize_identifier(name, dest),
            Self::Line { index, name } => {
                if *index != 0 {
                    serialize_integer(*index, dest)?;
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
                    serialize_integer(*index, dest)?;
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

impl ToCss for FontWeight<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Absolute(value) => value.to_css(dest),
            Self::Bolder => dest.write_str("bolder"),
            Self::Lighter => dest.write_str("lighter"),
        }
    }
}

impl ToCss for AbsoluteFontWeight {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Weight(value) => serialize_number(*value, dest),
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
        }
    }
}

impl ToCss for FontSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest),
            Self::Absolute(value) => value.to_css(dest),
            Self::Relative(value) => value.to_css(dest),
        }
    }
}

impl ToCss for FontStretch {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl ToCss for FontFamily<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Generic(value) => value.to_css(dest),
            Self::FamilyName(value) => value.to_css(dest),
        }
    }
}

impl ToCss for FontStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(value) => {
                dest.write_str("oblique")?;
                dest.write_char(' ')?;
                value.to_css(dest)
            }
        }
    }
}

impl ToCss for LineHeight<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => match &**value {
                DimensionPercentage::Dimension(value) if value.value == 0.0 => {
                    serialize_dimension(value.value, &value.unit, dest)
                }
                _ => value.to_css(dest),
            },
        }
    }
}

impl ToCss for VerticalAlign<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for EasingFunction {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Linear => dest.write_str("linear"),
            Self::Ease => dest.write_str("ease"),
            Self::EaseIn => dest.write_str("ease-in"),
            Self::EaseOut => dest.write_str("ease-out"),
            Self::EaseInOut => dest.write_str("ease-in-out"),
            Self::CubicBezier { x1, x2, y1, y2 } => {
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
                dest.write_str("steps(")?;
                serialize_integer(*count, dest)?;
                dest.delim(Delimiter::Comma)?;
                position.to_css(dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for AnimationIterationCount {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Infinite => dest.write_str("infinite"),
        }
    }
}

impl ToCss for AnimationTimeline<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::None => dest.write_str("none"),
            Self::DashedIdent(value) => {
                dest.write_str("--")?;
                serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
            }
            Self::Scroll(value) => {
                dest.write_str("scroll(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
            Self::View(value) => {
                dest.write_str("view(")?;
                value.to_css(dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for AnimationAttachmentRange<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::TimelineRange { name, offset } => {
                name.to_css(dest)?;
                dest.write_char(' ')?;
                offset.to_css(dest)
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

fn write_comma_values<PrinterT: PrinterTrait, T: ToCss>(
    values: &[&T],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

impl ToCss for Transform<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Translate((x, y)) => {
                write_function_values("translate", dest, |dest| write_comma_values(&[x, y], dest))
            }
            Self::TranslateX(value) => {
                write_function_values("translateX", dest, |dest| value.to_css(dest))
            }
            Self::TranslateY(value) => {
                write_function_values("translateY", dest, |dest| value.to_css(dest))
            }
            Self::TranslateZ(value) => {
                write_function_values("translateZ", dest, |dest| value.to_css(dest))
            }
            Self::Translate3d((x, y, z)) => write_function_values("translate3d", dest, |dest| {
                x.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                z.to_css(dest)
            }),
            Self::Scale((x, y)) => {
                write_function_values("scale", dest, |dest| write_comma_values(&[x, y], dest))
            }
            Self::ScaleX(value) => write_function_values("scaleX", dest, |dest| value.to_css(dest)),
            Self::ScaleY(value) => write_function_values("scaleY", dest, |dest| value.to_css(dest)),
            Self::ScaleZ(value) => write_function_values("scaleZ", dest, |dest| value.to_css(dest)),
            Self::Scale3d((x, y, z)) => write_function_values("scale3d", dest, |dest| {
                x.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                z.to_css(dest)
            }),
            Self::Rotate(value) => write_function_values("rotate", dest, |dest| value.to_css(dest)),
            Self::RotateX(value) => {
                write_function_values("rotateX", dest, |dest| value.to_css(dest))
            }
            Self::RotateY(value) => {
                write_function_values("rotateY", dest, |dest| value.to_css(dest))
            }
            Self::RotateZ(value) => {
                write_function_values("rotateZ", dest, |dest| value.to_css(dest))
            }
            Self::Rotate3d((x, y, z, angle)) => write_function_values("rotate3d", dest, |dest| {
                for value in [x, y, z] {
                    serialize_number(*value, dest)?;
                    dest.delim(Delimiter::Comma)?;
                }
                angle.to_css(dest)
            }),
            Self::Skew((x, y)) => {
                write_function_values("skew", dest, |dest| write_comma_values(&[x, y], dest))
            }
            Self::SkewX(value) => write_function_values("skewX", dest, |dest| value.to_css(dest)),
            Self::SkewY(value) => write_function_values("skewY", dest, |dest| value.to_css(dest)),
            Self::Perspective(value) => {
                write_function_values("perspective", dest, |dest| value.to_css(dest))
            }
            Self::Matrix(value) => value.to_css(dest),
            Self::Matrix3d(value) => value.to_css(dest),
        }
    }
}

impl ToCss for Perspective<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for Translate<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Xyz { x, y, z } => {
                x.to_css(dest)?;
                dest.write_char(' ')?;
                y.to_css(dest)?;
                dest.write_char(' ')?;
                z.to_css(dest)
            }
        }
    }
}

impl ToCss for Scale<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Xyz { x, y, z } => {
                x.to_css(dest)?;
                dest.write_char(' ')?;
                y.to_css(dest)?;
                dest.write_char(' ')?;
                z.to_css(dest)
            }
        }
    }
}

impl ToCss for Spacing<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Length(value) => value.to_css(dest),
        }
    }
}

impl ToCss for TextDecorationLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::ExclusiveTextDecorationLine(value) => value.to_css(dest),
            Self::Value(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for TextDecorationThickness<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::FromFont => dest.write_str("from-font"),
            Self::LengthPercentage(value) => value.to_css(dest),
        }
    }
}

impl ToCss for TextEmphasisStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Keyword { fill, shape } => {
                fill.to_css(dest)?;
                if let Some(shape) = shape {
                    dest.write_char(' ')?;
                    shape.to_css(dest)?;
                }
                Ok(())
            }
            Self::String(value) => serialize_string(value, dest),
        }
    }
}

impl ToCss for TextSizeAdjust {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for ColorOrAuto<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Color(value) => value.to_css(dest),
        }
    }
}

impl ToCss for Appearance<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::NonStandard(value) => dest.write_str(value),
            value => serialize_debug_keyword(value, dest),
        }
    }
}

impl ToCss for ListStyleType<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::String(value) => serialize_string(value, dest),
            Self::CounterStyle(value) => value.to_css(dest),
        }
    }
}

impl ToCss for CounterStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Predefined(value) => value.to_css(dest),
            Self::Name(value) => serialize_identifier(value, dest),
            Self::Symbols { symbols, system } => {
                dest.write_str("symbols(")?;
                if !matches!(system, SymbolsType::Symbolic) {
                    system.to_css(dest)?;
                    dest.write_char(' ')?;
                }
                for (index, symbol) in symbols.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    symbol.to_css(dest)?;
                }
                dest.write_char(')')
            }
        }
    }
}

impl ToCss for Symbol<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::String(value) => serialize_string(value, dest),
            Self::Image(value) => value.to_css(dest),
        }
    }
}

impl ToCss for SVGPaint<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Url { fallback, url } => {
                url.to_css(dest)?;
                if let Some(fallback) = fallback {
                    dest.write_char(' ')?;
                    fallback.to_css(dest)?;
                }
                Ok(())
            }
            Self::Color(value) => value.to_css(dest),
            Self::ContextFill => dest.write_str("context-fill"),
            Self::ContextStroke => dest.write_str("context-stroke"),
            Self::None => dest.write_str("none"),
        }
    }
}

impl ToCss for SVGPaintFallback<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Color(value) => value.to_css(dest),
        }
    }
}

impl ToCss for StrokeDasharray<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Values(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for Marker<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest),
        }
    }
}

impl ToCss for ClipPath<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest),
            Self::Shape {
                reference_box,
                shape,
            } => {
                shape.to_css(dest)?;
                dest.write_char(' ')?;
                reference_box.to_css(dest)
            }
            Self::Box(value) => value.to_css(dest),
        }
    }
}

impl ToCss for BasicShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Inset(value) => value.to_css(dest),
            Self::Circle(value) => value.to_css(dest),
            Self::Ellipse(value) => value.to_css(dest),
            Self::Polygon(value) => value.to_css(dest),
        }
    }
}

impl ToCss for ShapeRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::ClosestSide => dest.write_str("closest-side"),
            Self::FarthestSide => dest.write_str("farthest-side"),
        }
    }
}

impl ToCss for MaskClip {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::GeometryBox(value) => value.to_css(dest),
            Self::NoClip => dest.write_str("no-clip"),
        }
    }
}

impl ToCss for FilterList<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Filters(values) => {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest)?;
                }
                Ok(())
            }
        }
    }
}

impl ToCss for Filter<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Blur(value) => write_function_values("blur", dest, |dest| value.to_css(dest)),
            Self::Brightness(value) => {
                write_function_values("brightness", dest, |dest| value.to_css(dest))
            }
            Self::Contrast(value) => {
                write_function_values("contrast", dest, |dest| value.to_css(dest))
            }
            Self::Grayscale(value) => {
                write_function_values("grayscale", dest, |dest| value.to_css(dest))
            }
            Self::HueRotate(value) => {
                write_function_values("hue-rotate", dest, |dest| value.to_css(dest))
            }
            Self::Invert(value) => write_function_values("invert", dest, |dest| value.to_css(dest)),
            Self::Opacity(value) => {
                write_function_values("opacity", dest, |dest| value.to_css(dest))
            }
            Self::Saturate(value) => {
                write_function_values("saturate", dest, |dest| value.to_css(dest))
            }
            Self::Sepia(value) => write_function_values("sepia", dest, |dest| value.to_css(dest)),
            Self::DropShadow(value) => {
                write_function_values("drop-shadow", dest, |dest| value.to_css(dest))
            }
            Self::Url(value) => value.to_css(dest),
        }
    }
}

impl ToCss for ZIndex {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_integer(*value, dest),
        }
    }
}

impl ToCss for ContainerNameList<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

impl ToCss for ViewTransitionName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Auto => dest.write_str("auto"),
            Self::Custom(value) => serialize_identifier(value, dest),
        }
    }
}

impl ToCss for NoneOrCustomIdentList<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Idents(values) => write_ident_list(values, dest),
        }
    }
}

impl ToCss for ViewTransitionGroup<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Contain => dest.write_str("contain"),
            Self::Nearest => dest.write_str("nearest"),
            Self::Custom(value) => serialize_identifier(value, dest),
        }
    }
}

impl ToCss for CustomPropertyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let value = match self {
            Self::Custom(value) | Self::Unknown(value) => value,
        };
        dest.write_str("--")?;
        serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
    }
}

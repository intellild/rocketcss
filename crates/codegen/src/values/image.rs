use super::*;

keyword_values! {
    HorizontalPositionKeyword,
    VerticalPositionKeyword,
    ShapeExtent,
    BackgroundRepeatKeyword,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,
    ObjectFit,
}

impl<'ghost> ToCss<'ghost> for Image<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Gradient(value) => value.to_css(dest, _cx),
            Self::ImageSet(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_gradient_items<'ast, 'ghost, PrinterT, D, I>(
    items: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    D: ToCss<'ghost> + DimensionValue + 'ast,
    I: IntoIterator<Item = NodeId<'ast, GradientItem<'ast, D>>>,
{
    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        item.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Gradient<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        match cx.ast_context().gradient(id) {
            GradientRead::Linear {
                repeating,
                vendor_prefix,
                direction,
                items,
            } => write_linear_gradient(
                repeating,
                vendor_prefix,
                direction,
                || items.items(),
                dest,
                cx,
            ),
            GradientRead::Radial {
                repeating,
                vendor_prefix,
                position,
                shape,
                items,
            } => write_radial_gradient(
                repeating,
                vendor_prefix,
                position,
                shape,
                || items.items(),
                dest,
                cx,
            ),
            GradientRead::Conic {
                repeating,
                angle,
                position,
                items,
            } => write_conic_gradient(repeating, angle, position, || items.items(), dest, cx),
            GradientRead::WebKitGradient(value) => value.to_css(dest, cx),
        }
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match *self {
            Self::Linear {
                direction,
                items,
                vendor_prefix,
            } => write_linear_gradient(false, vendor_prefix, direction, || items, dest, cx),
            Self::RepeatingLinear {
                direction,
                items,
                vendor_prefix,
            } => write_linear_gradient(true, vendor_prefix, direction, || items, dest, cx),
            Self::Radial {
                items,
                position,
                shape,
                vendor_prefix,
            } => write_radial_gradient(false, vendor_prefix, position, shape, || items, dest, cx),
            Self::RepeatingRadial {
                items,
                position,
                shape,
                vendor_prefix,
            } => write_radial_gradient(true, vendor_prefix, position, shape, || items, dest, cx),
            Self::Conic {
                angle,
                items,
                position,
            } => write_conic_gradient(false, angle, position, || items, dest, cx),
            Self::RepeatingConic {
                angle,
                items,
                position,
            } => write_conic_gradient(true, angle, position, || items, dest, cx),
            Self::WebKitGradient(value) => value.to_css(dest, cx),
        }
    }
}
fn write_linear_gradient<'id, 'ghost, PrinterT: PrinterTrait>(
    repeating: bool,
    vendor_prefix: VendorPrefix,
    direction: LineDirection,
    items: impl FnOnce() -> AstVec<'id, NodeId<'id, GradientItem<'id, LengthValue>>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    vendor_prefix.to_css(dest, cx)?;
    dest.write_str(if repeating {
        "repeating-linear-gradient("
    } else {
        "linear-gradient("
    })?;
    if !matches!(
        direction,
        LineDirection::Vertical(VerticalPositionKeyword::Bottom)
    ) {
        direction.to_css(dest, cx)?;
        dest.delim(Delimiter::Comma)?;
    }
    write_gradient_items(cx.ast_context().vec_iter(items()), dest, cx)?;
    dest.write_char(')')
}

fn write_radial_gradient<'id, 'ghost, PrinterT: PrinterTrait>(
    repeating: bool,
    vendor_prefix: VendorPrefix,
    position: NodeId<'id, Position<'id>>,
    shape: NodeId<'id, EndingShape<'id>>,
    items: impl FnOnce() -> AstVec<'id, NodeId<'id, GradientItem<'id, LengthValue>>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    vendor_prefix.to_css(dest, cx)?;
    dest.write_str(if repeating {
        "repeating-radial-gradient("
    } else {
        "radial-gradient("
    })?;
    shape.to_css(dest, cx)?;
    dest.write_str(" at ")?;
    position.to_css(dest, cx)?;
    dest.delim(Delimiter::Comma)?;
    write_gradient_items(cx.ast_context().vec_iter(items()), dest, cx)?;
    dest.write_char(')')
}

fn write_conic_gradient<'id, 'ghost, PrinterT: PrinterTrait>(
    repeating: bool,
    angle: Angle,
    position: NodeId<'id, Position<'id>>,
    items: impl FnOnce() -> AstVec<'id, NodeId<'id, GradientItem<'id, Angle>>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str(if repeating {
        "repeating-conic-gradient(from "
    } else {
        "conic-gradient(from "
    })?;
    angle.to_css(dest, cx)?;
    dest.write_str(" at ")?;
    position.to_css(dest, cx)?;
    dest.delim(Delimiter::Comma)?;
    write_gradient_items(cx.ast_context().vec_iter(items()), dest, cx)?;
    dest.write_char(')')
}

impl<'ghost> ToCss<'ghost> for WebKitGradient<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().webkit_gradient(id);
        write_webkit_gradient(
            value.from(),
            value.to(),
            value.radii(),
            || value.stops(),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match *self {
            Self::Linear { from, to, stops } => {
                write_webkit_gradient(from, to, None, || stops, dest, cx)
            }
            Self::Radial {
                from,
                start_radius,
                to,
                end_radius,
                stops,
            } => write_webkit_gradient(
                from,
                to,
                Some([start_radius, end_radius]),
                || stops,
                dest,
                cx,
            ),
        }
    }
}
fn write_webkit_gradient<'id, 'ghost, PrinterT: PrinterTrait>(
    from: NodeId<'id, WebKitGradientPoint>,
    to: NodeId<'id, WebKitGradientPoint>,
    radii: Option<[f32; 2]>,
    stops: impl FnOnce() -> AstVec<'id, WebKitColorStop<'id>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("-webkit-gradient(")?;
    dest.write_str(if radii.is_some() {
        "radial, "
    } else {
        "linear, "
    })?;
    from.to_css(dest, cx)?;
    dest.delim(Delimiter::Comma)?;
    if let Some([start, _]) = radii {
        serialize_number(start, dest)?;
        dest.delim(Delimiter::Comma)?;
    }
    to.to_css(dest, cx)?;
    if let Some([_, end]) = radii {
        dest.delim(Delimiter::Comma)?;
        serialize_number(end, dest)?;
    }
    for stop in cx.ast_context().vec_iter(stops()) {
        dest.delim(Delimiter::Comma)?;
        stop.to_css(dest, cx)?;
    }
    dest.write_char(')')
}

impl<'ghost> ToCss<'ghost> for LineDirection {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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

impl<'ast, 'ghost, D> ToCss<'ghost> for GradientItem<'ast, D>
where
    D: ToCss<'ghost> + DimensionValue,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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

impl<'ast, 'ghost, D> ToCss<'ghost> for DimensionPercentage<'ast, D>
where
    D: ToCss<'ghost> + DimensionValue,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

use super::*;

impl<'ghost> ToCss<'ghost> for Position<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_position_components(self.x, self.y, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for WebKitGradientPoint {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.y.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for WebKitColorStop<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.position == 0.0 {
            dest.write_str("from(")?;
            self.color.to_css(dest, _cx)?;
        } else if self.position == 1.0 {
            dest.write_str("to(")?;
            self.color.to_css(dest, _cx)?;
        } else {
            dest.write_str("color-stop(")?;
            serialize_number(self.position, dest)?;
            dest.delim(Delimiter::Comma)?;
            self.color.to_css(dest, _cx)?;
        }
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for ImageSet<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.vendor_prefix.to_css(dest, _cx)?;
        dest.write_str("image-set(")?;
        write_comma_separated(_cx.ast_context().vec_iter(self.options), dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for ImageSetOption<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().image_set_option(id);
        write_image_set_option(
            value.image(),
            value.resolution(),
            || value.file_type(),
            dest,
            cx,
        )
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_image_set_option(self.image, self.resolution, || self.file_type, dest, cx)
    }
}

fn write_image_set_option<'ast, 'ghost, PrinterT: PrinterTrait>(
    image: NodeId<'ast, Image<'ast>>,
    resolution: Resolution,
    file_type: impl FnOnce() -> Option<AstStr<'ast>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    image.to_css(dest, cx)?;
    dest.write_char(' ')?;
    resolution.to_css(dest, cx)?;
    if let Some(file_type) = file_type() {
        dest.write_str(" type(")?;
        serialize_string(cx.ast_context().str(file_type), dest)?;
        dest.write_char(')')?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for BackgroundPosition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_position_components(self.x, self.y, dest, _cx)
    }
}

fn write_position_components<'ast, 'ghost, PrinterT: PrinterTrait>(
    x: NodeId<'ast, PositionComponent<'ast, HorizontalPositionKeyword>>,
    y: NodeId<'ast, PositionComponent<'ast, VerticalPositionKeyword>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast_context();
    let x = ast.resolve_node(x);
    x.to_css(dest, cx)?;
    let y = ast.resolve_node(y);
    if !matches!(y, PositionComponent::Center)
        || matches!(
            x,
            PositionComponent::Side {
                offset: Some(_),
                ..
            }
        )
    {
        dest.write_char(' ')?;
        y.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for BackgroundRepeat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match (&self.x, &self.y) {
            (BackgroundRepeatKeyword::Repeat, BackgroundRepeatKeyword::NoRepeat) => {
                dest.write_str("repeat-x")
            }
            (BackgroundRepeatKeyword::NoRepeat, BackgroundRepeatKeyword::Repeat) => {
                dest.write_str("repeat-y")
            }
            _ => write_pair(&self.x, &self.y, dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Background<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().background(id);
        let keywords = value.keywords();
        write_background(
            value.image(),
            value.color(),
            value.position(),
            value.size(),
            (
                keywords.repeat(),
                keywords.attachment(),
                keywords.origin(),
                keywords.clip(),
            ),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_background(
            self.image,
            self.color,
            self.position,
            self.size,
            (self.repeat, self.attachment, self.origin, self.clip),
            dest,
            cx,
        )
    }
}
fn write_background<'id, 'ghost, PrinterT: PrinterTrait>(
    image: NodeId<'id, Image<'id>>,
    color: NodeId<'id, CssColor<'id>>,
    position: NodeId<'id, BackgroundPosition<'id>>,
    size: NodeId<'id, BackgroundSize<'id>>,
    (repeat, attachment, origin, clip): (
        BackgroundRepeat,
        BackgroundAttachment,
        BackgroundOrigin,
        BackgroundClip,
    ),
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = _cx.ast_context();
    let image = ast.resolve_node(image);
    // Reject non-default enum fields before following position/size handles.
    if matches!(
        &repeat,
        BackgroundRepeat {
            x: BackgroundRepeatKeyword::Repeat,
            y: BackgroundRepeatKeyword::Repeat,
        }
    ) && attachment == BackgroundAttachment::Scroll
        && origin == BackgroundOrigin::PaddingBox
        && clip == BackgroundClip::BorderBox
        && matches!(image, Image::None)
        && is_zero_background_position(&position, _cx)
        && matches!(
            ast.resolve_node(size),
            BackgroundSize::Explicit { height, width }
                if matches!(ast.resolve_node(height), LengthPercentageOrAuto::Auto)
                    && matches!(ast.resolve_node(width), LengthPercentageOrAuto::Auto)
        )
    {
        return color.to_css(dest, _cx);
    }

    image.to_css(dest, _cx)?;
    dest.write_char(' ')?;
    position.to_css(dest, _cx)?;
    dest.write_str(" / ")?;
    size.to_css(dest, _cx)?;
    dest.write_char(' ')?;
    repeat.to_css(dest, _cx)?;
    dest.write_char(' ')?;
    attachment.to_css(dest, _cx)?;
    dest.write_char(' ')?;
    origin.to_css(dest, _cx)?;
    if clip
        != match &origin {
            BackgroundOrigin::BorderBox => BackgroundClip::BorderBox,
            BackgroundOrigin::PaddingBox => BackgroundClip::PaddingBox,
            BackgroundOrigin::ContentBox => BackgroundClip::ContentBox,
        }
    {
        dest.write_char(' ')?;
        clip.to_css(dest, _cx)?;
    }
    dest.write_char(' ')?;
    color.to_css(dest, _cx)
}

fn is_zero_position_components<'ast, Sx, Sy>(
    x: &NodeId<'ast, PositionComponent<'ast, Sx>>,
    y: &NodeId<'ast, PositionComponent<'ast, Sy>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool
where
    PositionComponent<'ast, Sx>: AstNodeStorage<'ast>,
    PositionComponent<'ast, Sy>: AstNodeStorage<'ast>,
{
    fn is_zero(
        component: &PositionComponent<'_, impl Sized>,
        cx: &ToCssContext<'_, '_, '_>,
    ) -> bool {
        matches!(
            component,
            PositionComponent::Length(value)
                if matches!(
                    cx.ast_context().resolve_node(*value),
                    DimensionPercentage::Percentage(0.0) | DimensionPercentage::Zero
                )
        )
    }

    let ast = cx.ast_context();
    is_zero(&ast.resolve_node(*x), cx) && is_zero(&ast.resolve_node(*y), cx)
}

fn is_zero_background_position(
    position: &NodeId<'_, BackgroundPosition<'_>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    let position = cx.ast_context().resolve_node(*position);
    is_zero_position_components(&position.x, &position.y, cx)
}

pub(super) fn is_zero_position(
    position: &NodeId<'_, Position<'_>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    let position = cx.ast_context().resolve_node(*position);
    is_zero_position_components(&position.x, &position.y, cx)
}

impl<'ghost> ToCss<'ghost> for BoxShadow<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let shadow = cx.ast_context().box_shadow(id);
        write_shadow(shadow.offsets(), shadow.color(), shadow.inset(), dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_shadow(
            [self.x_offset, self.y_offset, self.blur, self.spread],
            self.color,
            self.inset,
            dest,
            cx,
        )
    }
}

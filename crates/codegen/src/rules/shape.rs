use super::background::is_zero_position;
use super::*;

impl<'ghost> ToCss<'ghost> for InsetRect<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("inset(")?;
        self.rect.to_css(dest, _cx)?;
        dest.write_str(" round ")?;
        self.radius.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for CircleShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("circle(")?;
        self.radius.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for EllipseShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("ellipse(")?;
        self.radius_x.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.radius_y.to_css(dest, _cx)?;
        dest.write_str(" at ")?;
        self.position.to_css(dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Polygon<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("polygon(")?;
        self.fill_rule.to_css(dest, _cx)?;
        dest.delim(Delimiter::Comma)?;
        write_comma_separated(_cx.ast_context().vec_iter(self.points), dest, _cx)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Point<'_> {
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

impl<'ghost> ToCss<'ghost> for Mask<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().mask(id);
        write_mask_geometry(value.image(), value.position(), value.size(), dest, cx)?;
        let keywords = value.keywords();
        write_mask_keywords(
            keywords.repeat(),
            keywords.clip(),
            keywords.origin(),
            keywords.composite(),
            keywords.mode(),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_mask_geometry(self.image, self.position, self.size, dest, cx)?;
        write_mask_keywords(
            self.repeat,
            self.clip,
            self.origin,
            self.composite,
            self.mode,
            dest,
            cx,
        )
    }
}

fn write_mask_geometry<'id, 'ghost, PrinterT: PrinterTrait>(
    image: NodeId<'id, Image<'id>>,
    position: NodeId<'id, Position<'id>>,
    size: NodeId<'id, BackgroundSize<'id>>,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = _cx.ast_context();
    image.to_css(dest, _cx)?;

    let size = ast.resolve_node(size);
    let default_size = matches!(
        size,
        BackgroundSize::Explicit { height, width }
            if matches!(ast.resolve_node(height), LengthPercentageOrAuto::Auto)
                && matches!(ast.resolve_node(width), LengthPercentageOrAuto::Auto)
    );
    if !is_zero_position(&position, _cx) || !default_size {
        dest.write_char(' ')?;
        write_mask_position(&ast.resolve_node(position), dest, _cx)?;
        if !default_size {
            if dest.prettify() {
                dest.write_str(" / ")?;
            } else {
                dest.write_char('/')?;
            }
            write_mask_size(&size, dest, _cx)?;
        }
    }

    Ok(())
}

fn write_mask_keywords<'ghost, PrinterT: PrinterTrait>(
    repeat: BackgroundRepeat,
    clip: MaskClip,
    origin: GeometryBox,
    composite: MaskComposite,
    mode: MaskMode,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if repeat
        != (BackgroundRepeat {
            x: BackgroundRepeatKeyword::Repeat,
            y: BackgroundRepeatKeyword::Repeat,
        })
    {
        dest.write_char(' ')?;
        repeat.to_css(dest, _cx)?;
    }

    let clip_matches_origin = matches!(
        (&clip, &origin),
        (MaskClip::GeometryBox(clip), origin) if clip == origin
    );
    if origin != GeometryBox::BorderBox || !clip_matches_origin {
        dest.write_char(' ')?;
        origin.to_css(dest, _cx)?;
        if !clip_matches_origin {
            dest.write_char(' ')?;
            clip.to_css(dest, _cx)?;
        }
    }

    if composite != MaskComposite::Add {
        dest.write_char(' ')?;
        composite.to_css(dest, _cx)?;
    }
    if mode != MaskMode::MatchSource {
        dest.write_char(' ')?;
        mode.to_css(dest, _cx)?;
    }
    Ok(())
}

fn write_mask_position<'ghost, PrinterT: PrinterTrait>(
    value: &Position<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast_context();
    let x = ast.resolve_node(value.x);
    let y = ast.resolve_node(value.y);
    write_mask_horizontal_position(&x, dest, cx)?;
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
        write_mask_vertical_position(&y, dest, cx)?;
    }
    Ok(())
}

fn write_mask_size<'ghost, PrinterT: PrinterTrait>(
    value: &BackgroundSize<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if let BackgroundSize::Explicit { height, width } = value
        && matches!(
            cx.ast_context().resolve_node(*height),
            LengthPercentageOrAuto::Auto
        )
    {
        return width.to_css(dest, cx);
    }
    value.to_css(dest, cx)
}

fn write_mask_horizontal_position<'ghost, PrinterT: PrinterTrait>(
    value: &PositionComponent<'_, HorizontalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponent::Side { offset: None, side } = value
    {
        return dest.write_str(match side {
            HorizontalPositionKeyword::Left => "0",
            HorizontalPositionKeyword::Right => "100%",
        });
    }
    value.to_css(dest, cx)
}

fn write_mask_vertical_position<'ghost, PrinterT: PrinterTrait>(
    value: &PositionComponent<'_, VerticalPositionKeyword>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    if !dest.prettify()
        && let PositionComponent::Side { offset: None, side } = value
    {
        return dest.write_str(match side {
            VerticalPositionKeyword::Top => "0",
            VerticalPositionKeyword::Bottom => "100%",
        });
    }
    value.to_css(dest, cx)
}

impl<'ghost> ToCss<'ghost> for MaskBorder<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let value = cx.ast_context().mask_border(id);
        let (source, width) = value.source_and_width();
        super::border::write_border_image(
            source,
            value.slice(),
            width,
            value.outset(),
            value.repeat(),
            dest,
            cx,
        )?;
        dest.write_char(' ')?;
        value.mode().to_css(dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        super::border::write_border_image(
            self.source,
            self.slice,
            self.width,
            self.outset,
            self.repeat,
            dest,
            cx,
        )?;
        dest.write_char(' ')?;
        self.mode.to_css(dest, cx)
    }
}

impl<'ghost> ToCss<'ghost> for DropShadow<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_space_separated(&[&self.x_offset, &self.y_offset, &self.blur], dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

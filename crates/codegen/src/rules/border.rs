use super::*;

physical_four! { BorderWidth<'_>; }

impl<'ghost> ToCss<'ghost> for BorderColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_four_colors(&self.top, &self.right, &self.bottom, &self.left, dest, cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_four(&self.top, &self.right, &self.bottom, &self.left, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let ast = _cx.ast_context();
        let top_left = ast.resolve_node(self.top_left);
        let top_right = ast.resolve_node(self.top_right);
        let bottom_right = ast.resolve_node(self.bottom_right);
        let bottom_left = ast.resolve_node(self.bottom_left);
        write_four_nodes(
            &top_left.0,
            &top_right.0,
            &bottom_right.0,
            &bottom_left.0,
            dest,
            _cx,
        )?;
        if !css_values_are_equal(
            &ast.resolve_node(top_left.0),
            &ast.resolve_node(top_left.1),
            _cx,
        ) || !css_values_are_equal(
            &ast.resolve_node(top_right.0),
            &ast.resolve_node(top_right.1),
            _cx,
        ) || !css_values_are_equal(
            &ast.resolve_node(bottom_right.0),
            &ast.resolve_node(bottom_right.1),
            _cx,
        ) || !css_values_are_equal(
            &ast.resolve_node(bottom_left.0),
            &ast.resolve_node(bottom_left.1),
            _cx,
        ) {
            dest.write_str(" / ")?;
            write_four_nodes(
                &top_left.1,
                &top_right.1,
                &bottom_right.1,
                &bottom_left.1,
                dest,
                _cx,
            )?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageRepeat {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.horizontal, &self.vertical, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageSlice<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.offsets.to_css(dest, _cx)?;
        if self.fill {
            dest.write_str(" fill")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for BorderImage<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let image = cx.ast_context().border_image(id);
        let (source, width) = image.source_and_width();
        write_border_image(
            source,
            image.slice(),
            width,
            image.outset(),
            image.repeat(),
            dest,
            cx,
        )
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_border_image(
            self.source,
            self.slice,
            self.width,
            self.outset,
            self.repeat,
            dest,
            cx,
        )
    }
}

pub(super) fn write_border_image<'id, 'ghost, PrinterT: PrinterTrait>(
    source: NodeId<'id, Image<'id>>,
    slice: NodeId<'id, BorderImageSlice<'id>>,
    width: NodeId<'id, Rect<'id, BorderImageSideWidth<'id>>>,
    outset: NodeId<'id, Rect<'id, LengthOrNumber<'id>>>,
    repeat: BorderImageRepeat,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    source.to_css(dest, cx)?;
    dest.write_char(' ')?;
    slice.to_css(dest, cx)?;
    dest.write_str(" / ")?;
    width.to_css(dest, cx)?;
    dest.write_str(" / ")?;
    outset.to_css(dest, cx)?;
    dest.write_char(' ')?;
    repeat.to_css(dest, cx)
}

impl<'ghost> ToCss<'ghost> for BorderBlockColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_color_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderBlockStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderBlockWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_node_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_color_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for BorderInlineWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_node_pair(&self.start, &self.end, dest, _cx)
    }
}

impl<'ghost, S: ToCss<'ghost>> ToCss<'ghost> for GenericBorder<'_, S> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.width.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

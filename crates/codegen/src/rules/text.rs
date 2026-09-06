use super::*;

impl<'ghost> ToCss<'ghost> for TextTransform {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.case.to_css(dest, _cx)?;
        if self.full_width {
            dest.write_str(" full-width")?;
        }
        if self.full_size_kana {
            dest.write_str(" full-size-kana")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for TextIndent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.value.to_css(dest, _cx)?;
        if self.hanging {
            dest.write_str(" hanging")?;
        }
        if self.each_line {
            dest.write_str(" each-line")?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for TextDecoration<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.line.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.thickness.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasis<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasisPosition {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.vertical.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.horizontal.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for TextShadow<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let shadow = cx.ast_context().text_shadow(id);
        write_shadow(shadow.offsets(), shadow.color(), false, dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_shadow(
            [self.x_offset, self.y_offset, self.blur, self.spread],
            self.color,
            false,
            dest,
            cx,
        )
    }
}

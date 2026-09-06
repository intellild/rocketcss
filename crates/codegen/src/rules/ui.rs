use super::*;

impl<'ghost> ToCss<'ghost> for Cursor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for image in _cx.ast_context().vec_iter(self.images) {
            image.to_css(dest, _cx)?;
            dest.delim(Delimiter::Comma)?;
        }
        self.keyword.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for CursorImage<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.url.to_css(dest, _cx)?;
        if let Some((x, y)) = self.hotspot {
            dest.write_char(' ')?;
            serialize_number(x, dest)?;
            dest.write_char(' ')?;
            serialize_number(y, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for Caret<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.color.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.shape.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ListStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.position.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.image.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.list_style_type.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Composes<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for (index, name) in _cx.ast_context().vec_iter(self.names).enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_identifier(_cx.ast_context().str(name), dest)?;
        }
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for ColorScheme {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.only {
            dest.write_str("only ")?;
        }
        match (self.light, self.dark) {
            (true, true) => dest.write_str("light dark"),
            (true, false) => dest.write_str("light"),
            (false, true) => dest.write_str("dark"),
            (false, false) => dest.write_str("normal"),
        }
    }
}

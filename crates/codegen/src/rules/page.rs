use super::*;

impl<'ghost> ToCss<'ghost> for PageMarginBox {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page margin boxes are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for PagePseudoClass {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("page pseudo classes are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for PageSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(name) = self.name {
            serialize_identifier(_cx.ast_context().str(name), dest)?;
        }
        for pseudo_class in _cx.ast_context().vec_iter(self.pseudo_classes) {
            dest.write_char(':')?;
            pseudo_class.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

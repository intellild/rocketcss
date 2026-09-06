use super::*;

impl<'ghost> ToCss<'ghost> for CharsetRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@charset ")?;
        serialize_string(_cx.ast_context().str(self.encoding), dest)?;
        dest.write_char(';')
    }
}

impl<'ghost> ToCss<'ghost> for NamespaceRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("@namespace ")?;
        if let Some(prefix) = self.prefix {
            serialize_identifier(_cx.ast_context().str(prefix), dest)?;
            dest.write_char(' ')?;
        }
        serialize_string(_cx.ast_context().str(self.url), dest)?;
        dest.write_char(';')
    }
}

impl<'ghost> ToCss<'ghost> for CustomMediaRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let name = _cx.ast_context().str(self.name);
        dest.write_str("@custom-media ")?;
        dest.write_str("--")?;
        serialize_name(name.strip_prefix("--").unwrap_or(name), dest)?;
        dest.write_char(' ')?;
        self.query.to_css(dest, _cx)?;
        dest.write_char(';')
    }
}

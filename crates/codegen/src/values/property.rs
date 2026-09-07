use super::*;

keyword_values! {
    CSSWideKeyword,
}

impl<'ghost> ToCss<'ghost> for CustomPropertyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let value = match self {
            Self::Custom(value) | Self::Unknown(value) => _cx.ast_context().str(*value),
        };
        dest.write_str("--")?;
        serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
    }
}

impl<'ghost, T: ToCss<'ghost>> ToCss<'ghost> for CSSWideOr<T> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Value(value) => value.to_css(dest, _cx),
            Self::CSSWide(keyword) => keyword.to_css(dest, _cx),
        }
    }
}

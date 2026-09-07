use super::*;

keyword_values! {
    LineStyle,
    BorderImageRepeatKeyword,
}

impl<'ghost> ToCss<'ghost> for BorderSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Thin => dest.write_str("thin"),
            Self::Medium => dest.write_str("medium"),
            Self::Thick => dest.write_str("thick"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for LengthOrNumber<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for BorderImageSideWidth<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Number(value) => serialize_number(*value, dest),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for OutlineStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LineStyle(value) => value.to_css(dest, _cx),
        }
    }
}

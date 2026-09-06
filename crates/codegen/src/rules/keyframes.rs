use super::*;

impl<'ghost> ToCss<'ghost> for KeyframeSelector {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::From => dest.write_str("from"),
            Self::To => dest.write_str("to"),
            Self::TimelineRangePercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for KeyframesName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Ident(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
            Self::Custom(value) => serialize_string(_cx.ast_context().str(*value), dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TimelineRangePercentage {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.name.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        serialize_number(self.percentage * 100.0, dest)?;
        dest.write_char('%')
    }
}

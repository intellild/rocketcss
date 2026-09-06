use super::*;

keyword_values! {
    SymbolsType,
    PredefinedCounterStyle,
    ListStylePosition,
    MarkerSide,
}

impl<'ghost> ToCss<'ghost> for ListStyleType<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::String(value) => serialize_string(_cx.ast_context().str(*value), dest),
            Self::CounterStyle(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for CounterStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Predefined(value) => value.to_css(dest, _cx),
            Self::Name(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
            Self::Symbols { symbols, system } => {
                dest.write_str("symbols(")?;
                if !matches!(system, SymbolsType::Symbolic) {
                    system.to_css(dest, _cx)?;
                    dest.write_char(' ')?;
                }
                for (index, symbol) in _cx.ast_context().vec_iter(*symbols).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    symbol.to_css(dest, _cx)?;
                }
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Symbol<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::String(value) => serialize_string(_cx.ast_context().str(*value), dest),
            Self::Image(value) => value.to_css(dest, _cx),
        }
    }
}

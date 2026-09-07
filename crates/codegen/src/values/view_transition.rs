use super::*;

impl<'ghost> ToCss<'ghost> for ViewTransitionName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Auto => dest.write_str("auto"),
            Self::Custom(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for NoneOrCustomIdentList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Idents(values) => write_ident_list(
                _cx.ast_context()
                    .vec_iter(*values)
                    .map(|value| _cx.ast_context().str(value)),
                dest,
            ),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ViewTransitionGroup<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Contain => dest.write_str("contain"),
            Self::Nearest => dest.write_str("nearest"),
            Self::Custom(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
        }
    }
}

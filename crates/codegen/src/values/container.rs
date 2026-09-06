use super::*;

keyword_values! {
    ContainerType,
}

impl<'ghost> ToCss<'ghost> for ContainerNameList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Names(values) => write_ident_list(
                _cx.ast_context()
                    .vec_iter(*values)
                    .map(|name| _cx.ast_context().str(name)),
                dest,
            ),
        }
    }
}

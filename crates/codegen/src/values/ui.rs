use super::*;

keyword_values! {
    Resize,
    CursorKeyword,
    CaretShape,
    UserSelect,
    PointerEvents,
    Float,
    Clear,
    TouchAction,
    ScrollBehavior,
    PrintColorAdjust,
}

impl<'ghost> ToCss<'ghost> for ScrollbarColor<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Colors(first, second) => {
                first.to_css(dest, cx)?;
                dest.write_char(' ')?;
                second.to_css(dest, cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for ColorOrAuto<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Color(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Appearance<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::NonStandard(value) => dest.write_str(_cx.ast_context().str(*value)),
            value => dest.write_str(
                value
                    .as_css_str()
                    .expect("non-standard appearance handled separately"),
            ),
        }
    }
}

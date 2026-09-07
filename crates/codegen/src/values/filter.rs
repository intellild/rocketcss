use super::*;

impl<'ghost> ToCss<'ghost> for FilterList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Filters(values) => {
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Filter<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Blur(value) => {
                write_function_values("blur", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Brightness(value) => {
                write_function_values("brightness", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Contrast(value) => {
                write_function_values("contrast", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Grayscale(value) => {
                write_function_values("grayscale", dest, |dest| value.to_css(dest, _cx))
            }
            Self::HueRotate(value) => {
                write_function_values("hue-rotate", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Invert(value) => {
                write_function_values("invert", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Opacity(value) => {
                write_function_values("opacity", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Saturate(value) => {
                write_function_values("saturate", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Sepia(value) => {
                write_function_values("sepia", dest, |dest| value.to_css(dest, _cx))
            }
            Self::DropShadow(value) => {
                write_function_values("drop-shadow", dest, |dest| value.to_css(dest, _cx))
            }
            Self::Url(value) => value.to_css(dest, _cx),
        }
    }
}

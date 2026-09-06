use super::*;

keyword_values! {
    MaskMode,
    MaskComposite,
    MaskType,
    MaskBorderMode,
    WebKitMaskComposite,
    WebKitMaskSourceType,
}

impl<'ghost> ToCss<'ghost> for MaskClip {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::GeometryBox(value) => value.to_css(dest, _cx),
            Self::NoClip => dest.write_str("no-clip"),
        }
    }
}

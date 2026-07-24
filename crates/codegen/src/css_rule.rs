use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for CssRule<'_, 'ghost> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Media(value) => value.to_css(dest, _cx),
            Self::Import(value) => value.to_css(dest, _cx),
            Self::Style(value) => value.as_ref().get_ref().to_css(dest, _cx),
            Self::Keyframes(value) => value.to_css(dest, _cx),
            Self::FontFace(value) => value.to_css(dest, _cx),
            Self::FontPaletteValues(value) => value.to_css(dest, _cx),
            Self::FontFeatureValues(value) => value.to_css(dest, _cx),
            Self::Page(value) => value.to_css(dest, _cx),
            Self::Supports(value) => value.to_css(dest, _cx),
            Self::CounterStyle(value) => value.to_css(dest, _cx),
            Self::Charset(value) => value.to_css(dest, _cx),
            Self::Namespace(value) => value.to_css(dest, _cx),
            Self::MozDocument(value) => value.to_css(dest, _cx),
            Self::Nesting(value) => value.to_css(dest, _cx),
            Self::NestedDeclarations(value) => value.to_css(dest, _cx),
            Self::Viewport(value) => value.to_css(dest, _cx),
            Self::CustomMedia(value) => value.to_css(dest, _cx),
            Self::LayerStatement(value) => value.to_css(dest, _cx),
            Self::LayerBlock(value) => value.to_css(dest, _cx),
            Self::Property(value) => value.to_css(dest, _cx),
            Self::Container(value) => value.to_css(dest, _cx),
            Self::Scope(value) => value.to_css(dest, _cx),
            Self::StartingStyle(value) => value.to_css(dest, _cx),
            Self::ViewTransition(value) => value.to_css(dest, _cx),
            Self::PositionTry(value) => value.to_css(dest, _cx),
            Self::Custom(_) => Ok(()),
            Self::Unknown(value) => value.to_css(dest, _cx),
        }
    }
}

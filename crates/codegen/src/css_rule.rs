use crate::prelude::*;

impl<'ghost> ToCssWithGhost<'ghost> for CssRule<'_, 'ghost> {
    fn to_css_with_ghost<PrinterT: PrinterTrait>(
        &self,
        token: &GhostToken<'ghost>,
        dest: &mut PrinterT,
    ) -> fmt::Result {
        match self {
            Self::Media(value) => value.to_css_with_ghost(token, dest),
            Self::Import(value) => value.to_css(dest),
            Self::Style(value) => value.get(token).get_ref().to_css_with_ghost(token, dest),
            Self::Keyframes(value) => value.to_css_with_ghost(token, dest),
            Self::FontFace(value) => value.to_css(dest),
            Self::FontPaletteValues(value) => value.to_css(dest),
            Self::FontFeatureValues(value) => value.to_css(dest),
            Self::Page(value) => value.to_css_with_ghost(token, dest),
            Self::Supports(value) => value.to_css_with_ghost(token, dest),
            Self::CounterStyle(value) => value.to_css_with_ghost(token, dest),
            Self::Charset(value) => value.to_css(dest),
            Self::Namespace(value) => value.to_css(dest),
            Self::MozDocument(value) => value.to_css_with_ghost(token, dest),
            Self::Nesting(value) => value.to_css_with_ghost(token, dest),
            Self::NestedDeclarations(value) => value.to_css_with_ghost(token, dest),
            Self::Viewport(value) => value.to_css_with_ghost(token, dest),
            Self::CustomMedia(value) => value.to_css(dest),
            Self::LayerStatement(value) => value.to_css(dest),
            Self::LayerBlock(value) => value.to_css_with_ghost(token, dest),
            Self::Property(value) => value.to_css(dest),
            Self::Container(value) => value.to_css_with_ghost(token, dest),
            Self::Scope(value) => value.to_css_with_ghost(token, dest),
            Self::StartingStyle(value) => value.to_css_with_ghost(token, dest),
            Self::ViewTransition(value) => value.to_css(dest),
            Self::PositionTry(value) => value.to_css_with_ghost(token, dest),
            Self::Custom(_) => Ok(()),
            Self::Unknown(value) => value.to_css(dest),
        }
    }
}

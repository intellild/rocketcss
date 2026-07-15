use crate::prelude::*;

impl ToCss for CssRule<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Media(value) => value.to_css(dest),
            Self::Import(value) => value.to_css(dest),
            Self::Style(value) => value.to_css(dest),
            Self::Keyframes(value) => value.to_css(dest),
            Self::FontFace(value) => value.to_css(dest),
            Self::FontPaletteValues(value) => value.to_css(dest),
            Self::FontFeatureValues(value) => value.to_css(dest),
            Self::Page(value) => value.to_css(dest),
            Self::Supports(value) => value.to_css(dest),
            Self::CounterStyle(value) => value.to_css(dest),
            Self::Charset(value) => value.to_css(dest),
            Self::Namespace(value) => value.to_css(dest),
            Self::MozDocument(value) => value.to_css(dest),
            Self::Nesting(value) => value.to_css(dest),
            Self::NestedDeclarations(value) => value.to_css(dest),
            Self::Viewport(value) => value.to_css(dest),
            Self::CustomMedia(value) => value.to_css(dest),
            Self::LayerStatement(value) => value.to_css(dest),
            Self::LayerBlock(value) => value.to_css(dest),
            Self::Property(value) => value.to_css(dest),
            Self::Container(value) => value.to_css(dest),
            Self::Scope(value) => value.to_css(dest),
            Self::StartingStyle(value) => value.to_css(dest),
            Self::ViewTransition(value) => value.to_css(dest),
            Self::PositionTry(value) => value.to_css(dest),
            Self::Custom(_) => Ok(()),
            Self::Unknown(value) => value.to_css(dest),
        }
    }
}

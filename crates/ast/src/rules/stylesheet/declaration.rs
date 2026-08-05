use crate::{
    Box, CustomProperty, Declaration, FontFaceProperty, FontFeatureDeclaration,
    FontPaletteValuesProperty, ParsedComponent, SyntaxString, ViewTransitionProperty,
};

/// One typed descriptor occurrence inside `@property`.
#[derive(Debug, PartialEq)]
pub enum PropertyRuleDescriptor<'ast> {
    Syntax(Box<'ast, SyntaxString<'ast>>),
    Inherits(bool),
    InitialValue(Box<'ast, ParsedComponent<'ast>>),
    Unknown(Box<'ast, CustomProperty<'ast>>),
}

/// One heterogeneous occurrence in the global semantic declaration arena.
#[derive(Debug, PartialEq)]
pub enum CssDeclaration<'ast> {
    Property(Declaration<'ast>),
    FontFace(FontFaceProperty<'ast>),
    FontPaletteValues(FontPaletteValuesProperty<'ast>),
    ViewTransition(ViewTransitionProperty<'ast>),
    FontFeature(FontFeatureDeclaration<'ast>),
    PropertyRule(PropertyRuleDescriptor<'ast>),
}

impl<'ast> CssDeclaration<'ast> {
    #[inline]
    pub const fn as_property(&self) -> Option<&Declaration<'ast>> {
        match self {
            Self::Property(declaration) => Some(declaration),
            Self::FontFace(_)
            | Self::FontPaletteValues(_)
            | Self::ViewTransition(_)
            | Self::FontFeature(_)
            | Self::PropertyRule(_) => None,
        }
    }
}

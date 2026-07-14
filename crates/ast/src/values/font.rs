use crate::*;

#[derive(Debug, PartialEq)]
pub enum FontWeight<'a> {
    Absolute(Box<'a, AbsoluteFontWeight>),
    Bolder,
    Lighter,
}

#[derive(Debug, PartialEq)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[derive(Debug, PartialEq)]
pub enum FontSize<'a> {
    Length(Box<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum AbsoluteFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FontStretchKeyword {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Debug, PartialEq)]
pub enum FontFamily<'a> {
    Generic(GenericFontFamily),
    FamilyName(FamilyName<'a>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum GenericFontFamily {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Initial,
    Inherit,
    Unset,
    Default,
    Revert,
    RevertLayer,
}

#[derive(Debug, PartialEq)]
pub enum FontStyle<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Angle>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[derive(Debug, PartialEq)]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}

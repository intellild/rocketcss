use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FontWeight<'a> {
    Absolute(Box<'a, AbsoluteFontWeight>),
    Bolder,
    Lighter,
}

#[derive(Debug, PartialEq, Visit)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontSize<'a> {
    Length(Box<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFamily<'a> {
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
    Custom(&'a str),
}

impl<'a> FontFamily<'a> {
    pub fn from_name(name: &'a str) -> Self {
        match_ignore_ascii_case!(
            name,
            "serif" => Self::Serif,
            "sans-serif" => Self::SansSerif,
            "cursive" => Self::Cursive,
            "fantasy" => Self::Fantasy,
            "monospace" => Self::Monospace,
            "system-ui" => Self::SystemUi,
            "emoji" => Self::Emoji,
            "math" => Self::Math,
            "fangsong" => Self::Fangsong,
            "ui-serif" => Self::UiSerif,
            "ui-sans-serif" => Self::UiSansSerif,
            "ui-monospace" => Self::UiMonospace,
            "ui-rounded" => Self::UiRounded,
            "initial" => Self::Initial,
            "inherit" => Self::Inherit,
            "unset" => Self::Unset,
            "default" => Self::Default,
            "revert" => Self::Revert,
            "revert-layer" => Self::RevertLayer,
            _ => Self::Custom(name),
        )
    }

    #[inline]
    pub const fn is_generic(&self) -> bool {
        matches!(
            self,
            Self::Serif
                | Self::SansSerif
                | Self::Cursive
                | Self::Fantasy
                | Self::Monospace
                | Self::SystemUi
                | Self::Emoji
                | Self::Math
                | Self::Fangsong
                | Self::UiSerif
                | Self::UiSansSerif
                | Self::UiMonospace
                | Self::UiRounded
        )
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontStyle<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Angle>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

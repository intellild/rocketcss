use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
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
pub enum FontSize {
    Length(std::boxed::Box<LengthPercentage>),
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
    Unparsed(std::vec::Vec<TokenOrValue<'a>>),
    /// Tombstone for a family entry removed by an in-place transform.
    #[css_keyword("")]
    Tombstone,
    Custom(Atom<'a>),
}

impl<'a> FontFamily<'a> {
    pub fn from_name(name: Atom<'a>) -> Self {
        let text = name.as_str();
        match_ignore_ascii_case!(
            text,
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

    #[inline]
    pub fn is_generic_name(name: &str) -> bool {
        match_ignore_ascii_case!(
            name,
            "serif" | "sans-serif" | "cursive" | "fantasy" | "monospace" | "system-ui"
                | "emoji" | "math" | "fangsong" | "ui-serif" | "ui-sans-serif"
                | "ui-monospace" | "ui-rounded" => true,
            _ => false,
        )
    }

    #[inline]
    pub fn is_known_name(name: &str) -> bool {
        Self::is_generic_name(name)
            || match_ignore_ascii_case!(
                name,
                "initial" | "inherit" | "unset" | "default" | "revert" | "revert-layer" => true,
                _ => false,
            )
    }

    #[inline]
    pub const fn is_tombstone(&self) -> bool {
        matches!(self, Self::Tombstone)
    }
}

impl EqIgnoringTombstones for FontFamily<'_> {
    #[inline]
    fn eq_ignoring_tombstones(&self, other: &Self) -> bool {
        self == other
    }
}

impl<'a> EqIgnoringTombstones for std::vec::Vec<FontFamily<'a>> {
    fn eq_ignoring_tombstones(&self, other: &Self) -> bool {
        let mut left = self.iter().filter(|family| !family.is_tombstone());
        let mut right = other.iter().filter(|family| !family.is_tombstone());
        loop {
            match (left.next(), right.next()) {
                (None, None) => return true,
                (Some(left), Some(right)) if left.eq_ignoring_tombstones(right) => {}
                _ => return false,
            }
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
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
pub enum LineHeight {
    Normal,
    Number(f32),
    Length(std::boxed::Box<LengthPercentage>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum VerticalAlign {
    Keyword(VerticalAlignKeyword),
    Length(std::boxed::Box<LengthPercentage>),
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

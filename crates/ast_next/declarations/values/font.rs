#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[ast]
pub enum FontSize<'a> {
    Length(Box<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[ast]
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
    Unparsed(Vec<'a, TokenOrValue<'a>>),
    /// Tombstone for a family entry removed by an in-place transform.
    #[css_keyword("")]
    Tombstone,
    Custom(&'a str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[ast]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[ast]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(Box<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

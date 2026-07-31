use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FontFaceProperty<'a> {
    Source(std::vec::Vec<Source<'a>>),
    FontFamily(std::boxed::Box<FontFamily<'a>>),
    FontStyle(std::boxed::Box<FontFaceStyle>),
    FontWeight(std::boxed::Box<Size2D<FontWeight>>),
    FontStretch(std::boxed::Box<Size2D<FontStretch>>),
    UnicodeRange(std::vec::Vec<UnicodeRange>),
    Custom(std::boxed::Box<CustomProperty<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Source<'a> {
    Url(std::boxed::Box<UrlSource<'a>>),
    Local(std::boxed::Box<FontFamily<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFormat<'a> {
    Woff,
    Woff2,
    Truetype,
    Opentype,
    EmbeddedOpentype,
    Collection,
    Svg,
    String(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontTechnology {
    FeaturesOpentype,
    FeaturesAat,
    FeaturesGraphite,
    ColorColrv0,
    ColorColrv1,
    ColorSvg,
    ColorSbix,
    ColorCbdt,
    Variations,
    Palettes,
    Incremental,
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontFaceStyle {
    Normal,
    Italic,
    Oblique(std::boxed::Box<Size2D<Angle>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(std::boxed::Box<FontFamily<'a>>),
    BasePalette(std::boxed::Box<BasePalette>),
    OverrideColors(std::vec::Vec<OverrideColors<'a>>),
    Custom(std::boxed::Box<CustomProperty<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFeatureSubruleType {
    Stylistic,
    HistoricalForms,
    Styleset,
    CharacterVariant,
    Swash,
    Ornaments,
    Annotation,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Font<'a> {
    pub family: std::vec::Vec<FontFamily<'a>>,
    pub line_height: std::boxed::Box<LineHeight>,
    pub size: std::boxed::Box<FontSize>,
    pub stretch: FontStretch,
    pub style: std::boxed::Box<FontStyle>,
    pub variant_caps: FontVariantCaps,
    pub weight: std::boxed::Box<FontWeight>,
}
#[derive(Debug, PartialEq, Visit)]
pub struct FontFaceRule<'a> {
    pub span: Span,
    pub properties: std::vec::Vec<FontFaceProperty<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UrlSource<'a> {
    pub format: Option<std::boxed::Box<FontFormat<'a>>>,
    pub tech: std::vec::Vec<FontTechnology>,
    pub url: std::boxed::Box<Url<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontPaletteValuesRule<'a> {
    pub span: Span,
    pub name: Atom<'a>,
    pub properties: std::vec::Vec<FontPaletteValuesProperty<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct OverrideColors<'a> {
    pub color: std::boxed::Box<CssColor<'a>>,
    pub index: u16,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontFeatureValuesRule<'a> {
    pub span: Span,
    pub name: std::vec::Vec<FamilyName<'a>>,
    pub rules: std::vec::Vec<FontFeatureSubrule<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontFeatureSubrule<'a> {
    pub declarations: std::vec::Vec<FontFeatureDeclaration<'a>>,
    pub span: Span,
    pub name: FontFeatureSubruleType,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontFeatureDeclaration<'a> {
    pub name: Atom<'a>,
    pub values: std::vec::Vec<i32>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FamilyName<'a>(pub Atom<'a>);

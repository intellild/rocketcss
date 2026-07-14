use crate::*;

#[derive(Debug, PartialEq)]
pub enum FontFaceProperty<'a> {
    Source(Vec<'a, Source<'a>>),
    FontFamily(Box<'a, FontFamily<'a>>),
    FontStyle(Box<'a, FontFaceStyle<'a>>),
    FontWeight(Box<'a, Size2D<'a, FontWeight<'a>>>),
    FontStretch(Box<'a, Size2D<'a, FontStretch>>),
    UnicodeRange(Vec<'a, UnicodeRange>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum Source<'a> {
    Url(Box<'a, UrlSource<'a>>),
    Local(Box<'a, FontFamily<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq)]
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

#[derive(CssKeyword, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum FontFaceStyle<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Size2D<'a, Angle>>),
}

#[derive(Debug, PartialEq)]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(Box<'a, FontFamily<'a>>),
    BasePalette(Box<'a, BasePalette>),
    OverrideColors(Vec<'a, OverrideColors<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}

#[derive(CssKeyword, Debug, PartialEq)]
pub enum FontFeatureSubruleType {
    Stylistic,
    HistoricalForms,
    Styleset,
    CharacterVariant,
    Swash,
    Ornaments,
    Annotation,
}

#[derive(Debug, PartialEq)]
pub struct Font<'a> {
    pub family: Vec<'a, FontFamily<'a>>,
    pub line_height: Box<'a, LineHeight<'a>>,
    pub size: Box<'a, FontSize<'a>>,
    pub stretch: Box<'a, FontStretch>,
    pub style: Box<'a, FontStyle<'a>>,
    pub variant_caps: FontVariantCaps,
    pub weight: Box<'a, FontWeight<'a>>,
}
#[derive(Debug, PartialEq)]
pub struct FontFaceRule<'a> {
    pub span: Span,
    pub properties: Vec<'a, FontFaceProperty<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UrlSource<'a> {
    pub format: Option<Box<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: Box<'a, Url<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

#[derive(Debug, PartialEq)]
pub struct FontPaletteValuesRule<'a> {
    pub span: Span,
    pub name: &'a str,
    pub properties: Vec<'a, FontPaletteValuesProperty<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct OverrideColors<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub index: u16,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureValuesRule<'a> {
    pub span: Span,
    pub name: Vec<'a, FamilyName<'a>>,
    pub rules: Vec<'a, FontFeatureSubrule<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureSubrule<'a> {
    pub declarations: Vec<'a, FontFeatureDeclaration<'a>>,
    pub span: Span,
    pub name: FontFeatureSubruleType,
}

#[derive(Debug, PartialEq)]
pub struct FontFeatureDeclaration<'a> {
    pub name: &'a str,
    pub values: Vec<'a, i32>,
}

#[derive(Debug, PartialEq)]
pub struct FamilyName<'a>(pub &'a str);

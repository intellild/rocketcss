use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FontFaceProperty<'a> {
    Source(Vec<'a, Source<'a>>),
    FontFamily(NodeId<'a, FontFamily<'a>>),
    FontStyle(NodeId<'a, FontFaceStyle<'a>>),
    FontWeight(NodeId<'a, Size2D<'a, FontWeight>>),
    FontStretch(NodeId<'a, Size2D<'a, FontStretch>>),
    UnicodeRange(Vec<'a, UnicodeRange>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum Source<'a> {
    Url(NodeId<'a, UrlSource<'a>>),
    Local(NodeId<'a, FontFamily<'a>>),
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
pub enum FontFaceStyle<'a> {
    Normal,
    Italic,
    Oblique(NodeId<'a, Size2D<'a, Angle>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(NodeId<'a, FontFamily<'a>>),
    BasePalette(NodeId<'a, BasePalette>),
    OverrideColors(Vec<'a, OverrideColors<'a>>),
    Custom(NodeId<'a, CustomProperty<'a>>),
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
    pub family: Vec<'a, FontFamily<'a>>,
    pub line_height: NodeId<'a, LineHeight<'a>>,
    pub size: NodeId<'a, FontSize<'a>>,
    pub stretch: FontStretch,
    pub style: NodeId<'a, FontStyle>,
    pub variant_caps: FontVariantCaps,
    pub weight: NodeId<'a, FontWeight>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UrlSource<'a> {
    pub format: Option<NodeId<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: NodeId<'a, Url<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct OverrideColors<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub index: u16,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontFeatureDeclaration<'a> {
    pub name: &'a str,
    pub values: Vec<'a, i32>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FamilyName<'a>(pub &'a str);

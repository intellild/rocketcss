#[ast]
pub enum FontFaceProperty<'a> {
    Source(Vec<'a, Source<'a>>),
    FontFamily(Box<'a, FontFamily<'a>>),
    FontStyle(Box<'a, FontFaceStyle<'a>>),
    FontWeight(Box<'a, Size2D<'a, FontWeight>>),
    FontStretch(Box<'a, Size2D<'a, FontStretch>>),
    UnicodeRange(Vec<'a, UnicodeRange>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[ast]
pub enum Source<'a> {
    Url(Box<'a, UrlSource<'a>>),
    Local(Box<'a, FontFamily<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy)]
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

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[ast]
pub enum FontFaceStyle<'a> {
    Normal,
    Italic,
    Oblique(Box<'a, Size2D<'a, Angle>>),
}

#[ast]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(Box<'a, FontFamily<'a>>),
    BasePalette(Box<'a, BasePalette>),
    OverrideColors(Vec<'a, OverrideColors<'a>>),
    Custom(Box<'a, CustomProperty<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FontFeatureSubruleType {
    Stylistic,
    HistoricalForms,
    Styleset,
    CharacterVariant,
    Swash,
    Ornaments,
    Annotation,
}

#[ast]
pub struct Font<'a> {
    pub family: Vec<'a, FontFamily<'a>>,
    pub line_height: Box<'a, LineHeight<'a>>,
    pub size: Box<'a, FontSize<'a>>,
    pub stretch: FontStretch,
    pub style: Box<'a, FontStyle>,
    pub variant_caps: FontVariantCaps,
    pub weight: Box<'a, FontWeight>,
}

#[ast]
pub struct UrlSource<'a> {
    pub format: Option<Box<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: Box<'a, Url<'a>>,
}

#[ast]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

#[ast]
pub struct OverrideColors<'a> {
    pub color: Box<'a, CssColor<'a>>,
    pub index: u16,
}

#[ast]
pub struct FontFeatureDeclaration<'a> {
    pub name: &'a str,
    pub values: Vec<'a, i32>,
}

#[ast]
pub struct FamilyName<'a>(pub &'a str);

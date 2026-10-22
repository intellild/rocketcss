use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFaceProperty<'ast>(pub(crate) NodeId<'ast, FontFacePropertyData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontFacePropertyData<'ast> {
    Source(&'ast [Source<'ast>]),
    FontFamily(FontFamily<'ast>),
    FontStyle(FontFaceStyle<'ast>),
    FontWeight(Size2D<'ast, FontWeight>),
    FontStretch(Size2D<'ast, FontStretch>),
    UnicodeRange(&'ast [UnicodeRange<'ast>]),
    Custom(CustomProperty<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFacePropertyKind {
    Source,
    FontFamily,
    FontStyle,
    FontWeight,
    FontStretch,
    UnicodeRange,
    Custom,
}
impl<'ast> NodeStorage<'ast> for FontFaceProperty<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FACE_PROPERTY;
    type Data = FontFacePropertyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFacePropertyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFacePropertyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFaceProperty<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FontFacePropertyData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FontFacePropertyKind {
        match ast.data(self) {
            FontFacePropertyData::Source { .. } => FontFacePropertyKind::Source,
            FontFacePropertyData::FontFamily { .. } => FontFacePropertyKind::FontFamily,
            FontFacePropertyData::FontStyle { .. } => FontFacePropertyKind::FontStyle,
            FontFacePropertyData::FontWeight { .. } => FontFacePropertyKind::FontWeight,
            FontFacePropertyData::FontStretch { .. } => FontFacePropertyKind::FontStretch,
            FontFacePropertyData::UnicodeRange { .. } => FontFacePropertyKind::UnicodeRange,
            FontFacePropertyData::Custom { .. } => FontFacePropertyKind::Custom,
        }
    }
}
impl<'ast> GetSpan<'ast> for FontFaceProperty<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFaceProperty<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_face_property(
        &mut self,
        data: FontFacePropertyData<'ast>,
        span: Span,
    ) -> FontFaceProperty<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Source<'ast>(pub(crate) NodeId<'ast, SourceData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceData<'ast> {
    Url(UrlSource<'ast>),
    Local(FontFamily<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Url,
    Local,
}
impl<'ast> NodeStorage<'ast> for Source<'ast> {
    const KIND: NodeKind = super::kind::KIND_SOURCE;
    type Data = SourceData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SourceData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SourceData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Source<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SourceData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SourceKind {
        match ast.data(self) {
            SourceData::Url { .. } => SourceKind::Url,
            SourceData::Local { .. } => SourceKind::Local,
        }
    }
}
impl<'ast> GetSpan<'ast> for Source<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Source<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_source(&mut self, data: SourceData<'ast>, span: Span) -> Source<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy)]
pub enum FontFormat<'ast> {
    Woff,
    Woff2,
    Truetype,
    Opentype,
    EmbeddedOpentype,
    Collection,
    Svg,
    String(&'ast str),
}
use crate::*;
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFaceStyle<'ast>(pub(crate) NodeId<'ast, FontFaceStyleData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontFaceStyleData<'ast> {
    Normal,
    Italic,
    Oblique(Size2D<'ast, Angle>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFaceStyleKind {
    Normal,
    Italic,
    Oblique,
}
impl<'ast> NodeStorage<'ast> for FontFaceStyle<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FACE_STYLE;
    type Data = FontFaceStyleData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFaceStyleData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFaceStyleData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFaceStyle<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FontFaceStyleData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FontFaceStyleKind {
        match ast.data(self) {
            FontFaceStyleData::Normal { .. } => FontFaceStyleKind::Normal,
            FontFaceStyleData::Italic { .. } => FontFaceStyleKind::Italic,
            FontFaceStyleData::Oblique { .. } => FontFaceStyleKind::Oblique,
        }
    }
}
impl<'ast> GetSpan<'ast> for FontFaceStyle<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFaceStyle<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_face_style(
        &mut self,
        data: FontFaceStyleData<'ast>,
        span: Span,
    ) -> FontFaceStyle<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontPaletteValuesProperty<'ast>(
    pub(crate) NodeId<'ast, FontPaletteValuesPropertyData<'ast>>,
);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontPaletteValuesPropertyData<'ast> {
    FontFamily(FontFamily<'ast>),
    BasePalette(BasePalette),
    OverrideColors(&'ast [OverrideColors<'ast>]),
    Custom(CustomProperty<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontPaletteValuesPropertyKind {
    FontFamily,
    BasePalette,
    OverrideColors,
    Custom,
}
impl<'ast> NodeStorage<'ast> for FontPaletteValuesProperty<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_PALETTE_VALUES_PROPERTY;
    type Data = FontPaletteValuesPropertyData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontPaletteValuesPropertyData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontPaletteValuesPropertyData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontPaletteValuesProperty<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> FontPaletteValuesPropertyData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> FontPaletteValuesPropertyKind {
        match ast.data(self) {
            FontPaletteValuesPropertyData::FontFamily { .. } => {
                FontPaletteValuesPropertyKind::FontFamily
            }
            FontPaletteValuesPropertyData::BasePalette { .. } => {
                FontPaletteValuesPropertyKind::BasePalette
            }
            FontPaletteValuesPropertyData::OverrideColors { .. } => {
                FontPaletteValuesPropertyKind::OverrideColors
            }
            FontPaletteValuesPropertyData::Custom { .. } => FontPaletteValuesPropertyKind::Custom,
        }
    }
}
impl<'ast> GetSpan<'ast> for FontPaletteValuesProperty<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontPaletteValuesProperty<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_palette_values_property(
        &mut self,
        data: FontPaletteValuesPropertyData<'ast>,
        span: Span,
    ) -> FontPaletteValuesProperty<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}
use crate::*;
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Font<'ast>(pub(crate) NodeId<'ast, FontData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontData<'ast> {
    pub family: &'ast [FontFamily<'ast>],
    pub line_height: LineHeight<'ast>,
    pub size: FontSize<'ast>,
    pub stretch: FontStretch,
    pub style: FontStyle,
    pub variant_caps: FontVariantCaps,
    pub weight: FontWeight,
}
impl<'ast> NodeStorage<'ast> for Font<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT;
    type Data = FontData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Font<'ast> {
    #[inline]
    pub fn family(self, ast: &Ast<'ast>) -> &'ast [FontFamily<'ast>] {
        ast.data(self).family
    }
    #[inline]
    pub fn set_family(self, ast: &mut Ast<'ast>, value: &'ast [FontFamily<'ast>]) {
        let mut data = ast.data(self);
        data.family = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn line_height(self, ast: &Ast<'ast>) -> LineHeight<'ast> {
        ast.data(self).line_height
    }
    #[inline]
    pub fn set_line_height(self, ast: &mut Ast<'ast>, value: LineHeight<'ast>) {
        let mut data = ast.data(self);
        data.line_height = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn size(self, ast: &Ast<'ast>) -> FontSize<'ast> {
        ast.data(self).size
    }
    #[inline]
    pub fn set_size(self, ast: &mut Ast<'ast>, value: FontSize<'ast>) {
        let mut data = ast.data(self);
        data.size = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn stretch(self, ast: &Ast<'ast>) -> FontStretch {
        ast.data(self).stretch
    }
    #[inline]
    pub fn set_stretch(self, ast: &mut Ast<'ast>, value: FontStretch) {
        let mut data = ast.data(self);
        data.stretch = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn style(self, ast: &Ast<'ast>) -> FontStyle {
        ast.data(self).style
    }
    #[inline]
    pub fn set_style(self, ast: &mut Ast<'ast>, value: FontStyle) {
        let mut data = ast.data(self);
        data.style = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn variant_caps(self, ast: &Ast<'ast>) -> FontVariantCaps {
        ast.data(self).variant_caps
    }
    #[inline]
    pub fn set_variant_caps(self, ast: &mut Ast<'ast>, value: FontVariantCaps) {
        let mut data = ast.data(self);
        data.variant_caps = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn weight(self, ast: &Ast<'ast>) -> FontWeight {
        ast.data(self).weight
    }
    #[inline]
    pub fn set_weight(self, ast: &mut Ast<'ast>, value: FontWeight) {
        let mut data = ast.data(self);
        data.weight = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Font<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Font<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font(&mut self, data: FontData<'ast>, span: Span) -> Font<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UrlSource<'ast>(pub(crate) NodeId<'ast, UrlSourceData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UrlSourceData<'ast> {
    pub format: Option<FontFormat<'ast>>,
    pub tech: &'ast [FontTechnology],
    pub url: Url<'ast>,
}
impl<'ast> NodeStorage<'ast> for UrlSource<'ast> {
    const KIND: NodeKind = super::kind::KIND_URL_SOURCE;
    type Data = UrlSourceData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UrlSourceData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UrlSourceData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> UrlSource<'ast> {
    #[inline]
    pub fn format(self, ast: &Ast<'ast>) -> Option<FontFormat<'ast>> {
        ast.data(self).format
    }
    #[inline]
    pub fn set_format(self, ast: &mut Ast<'ast>, value: Option<FontFormat<'ast>>) {
        let mut data = ast.data(self);
        data.format = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn tech(self, ast: &Ast<'ast>) -> &'ast [FontTechnology] {
        ast.data(self).tech
    }
    #[inline]
    pub fn set_tech(self, ast: &mut Ast<'ast>, value: &'ast [FontTechnology]) {
        let mut data = ast.data(self);
        data.tech = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn url(self, ast: &Ast<'ast>) -> Url<'ast> {
        ast.data(self).url
    }
    #[inline]
    pub fn set_url(self, ast: &mut Ast<'ast>, value: Url<'ast>) {
        let mut data = ast.data(self);
        data.url = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for UrlSource<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for UrlSource<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_url_source(&mut self, data: UrlSourceData<'ast>, span: Span) -> UrlSource<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnicodeRange<'ast>(pub(crate) NodeId<'ast, UnicodeRangeData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnicodeRangeData {
    pub end: u32,
    pub start: u32,
}
impl<'ast> NodeStorage<'ast> for UnicodeRange<'ast> {
    const KIND: NodeKind = super::kind::KIND_UNICODE_RANGE;
    type Data = UnicodeRangeData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UnicodeRangeData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UnicodeRangeData>) -> Self {
        Self(id)
    }
}
impl<'ast> UnicodeRange<'ast> {
    #[inline]
    pub fn end(self, ast: &Ast<'ast>) -> u32 {
        ast.data(self).end
    }
    #[inline]
    pub fn set_end(self, ast: &mut Ast<'ast>, value: u32) {
        let mut data = ast.data(self);
        data.end = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn start(self, ast: &Ast<'ast>) -> u32 {
        ast.data(self).start
    }
    #[inline]
    pub fn set_start(self, ast: &mut Ast<'ast>, value: u32) {
        let mut data = ast.data(self);
        data.start = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for UnicodeRange<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for UnicodeRange<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_unicode_range(
        &mut self,
        data: UnicodeRangeData,
        span: Span,
    ) -> UnicodeRange<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OverrideColors<'ast>(pub(crate) NodeId<'ast, OverrideColorsData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverrideColorsData<'ast> {
    pub color: CssColor<'ast>,
    pub index: u16,
}
impl<'ast> NodeStorage<'ast> for OverrideColors<'ast> {
    const KIND: NodeKind = super::kind::KIND_OVERRIDE_COLORS;
    type Data = OverrideColorsData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, OverrideColorsData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, OverrideColorsData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> OverrideColors<'ast> {
    #[inline]
    pub fn color(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).color
    }
    #[inline]
    pub fn set_color(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.color = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn index(self, ast: &Ast<'ast>) -> u16 {
        ast.data(self).index
    }
    #[inline]
    pub fn set_index(self, ast: &mut Ast<'ast>, value: u16) {
        let mut data = ast.data(self);
        data.index = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for OverrideColors<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for OverrideColors<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_override_colors(
        &mut self,
        data: OverrideColorsData<'ast>,
        span: Span,
    ) -> OverrideColors<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FontFeatureDeclaration<'ast>(pub(crate) NodeId<'ast, FontFeatureDeclarationData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontFeatureDeclarationData<'ast> {
    pub name: &'ast str,
    pub values: &'ast [i32],
}
impl<'ast> NodeStorage<'ast> for FontFeatureDeclaration<'ast> {
    const KIND: NodeKind = super::kind::KIND_FONT_FEATURE_DECLARATION;
    type Data = FontFeatureDeclarationData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FontFeatureDeclarationData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FontFeatureDeclarationData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FontFeatureDeclaration<'ast> {
    #[inline]
    pub fn name(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).name
    }
    #[inline]
    pub fn set_name(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn values(self, ast: &Ast<'ast>) -> &'ast [i32] {
        ast.data(self).values
    }
    #[inline]
    pub fn set_values(self, ast: &mut Ast<'ast>, value: &'ast [i32]) {
        let mut data = ast.data(self);
        data.values = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for FontFeatureDeclaration<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FontFeatureDeclaration<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_font_feature_declaration(
        &mut self,
        data: FontFeatureDeclarationData<'ast>,
        span: Span,
    ) -> FontFeatureDeclaration<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FamilyName<'ast>(pub(crate) NodeId<'ast, FamilyNameData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FamilyNameData<'ast> {
    pub field_0: &'ast str,
}
impl<'ast> NodeStorage<'ast> for FamilyName<'ast> {
    const KIND: NodeKind = super::kind::KIND_FAMILY_NAME;
    type Data = FamilyNameData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, FamilyNameData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, FamilyNameData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> FamilyName<'ast> {
    #[inline]
    pub fn field_0(self, ast: &Ast<'ast>) -> &'ast str {
        ast.data(self).field_0
    }
    #[inline]
    pub fn set_field_0(self, ast: &mut Ast<'ast>, value: &'ast str) {
        let mut data = ast.data(self);
        data.field_0 = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for FamilyName<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for FamilyName<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_family_name(
        &mut self,
        data: FamilyNameData<'ast>,
        span: Span,
    ) -> FamilyName<'ast> {
        self.alloc(data, span)
    }
}

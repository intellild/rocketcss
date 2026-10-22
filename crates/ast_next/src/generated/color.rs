use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CssColor<'ast>(pub(crate) NodeId<'ast, CssColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CssColorData<'ast> {
    CurrentColor,
    Known(KnownColor),
    Rgba(RGBA<'ast>),
    Function(Function<'ast>),
    Lab(LABColor),
    Predefined(PredefinedColor),
    Float(FloatColor),
    LightDark(LightDark<'ast>),
    System(SystemColor),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CssColorKind {
    CurrentColor,
    Known,
    Rgba,
    Function,
    Lab,
    Predefined,
    Float,
    LightDark,
    System,
}
impl<'ast> NodeStorage<'ast> for CssColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_CSS_COLOR;
    type Data = CssColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, CssColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, CssColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> CssColor<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> CssColorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> CssColorKind {
        match ast.data(self) {
            CssColorData::CurrentColor { .. } => CssColorKind::CurrentColor,
            CssColorData::Known { .. } => CssColorKind::Known,
            CssColorData::Rgba { .. } => CssColorKind::Rgba,
            CssColorData::Function { .. } => CssColorKind::Function,
            CssColorData::Lab { .. } => CssColorKind::Lab,
            CssColorData::Predefined { .. } => CssColorKind::Predefined,
            CssColorData::Float { .. } => CssColorKind::Float,
            CssColorData::LightDark { .. } => CssColorKind::LightDark,
            CssColorData::System { .. } => CssColorKind::System,
        }
    }
}
impl<'ast> GetSpan<'ast> for CssColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for CssColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_css_color(&mut self, data: CssColorData<'ast>, span: Span) -> CssColor<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RGBA<'ast>(pub(crate) NodeId<'ast, RGBAData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBAData {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl<'ast> NodeStorage<'ast> for RGBA<'ast> {
    const KIND: NodeKind = super::kind::KIND_RGBA;
    type Data = RGBAData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, RGBAData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, RGBAData>) -> Self {
        Self(id)
    }
}
impl<'ast> RGBA<'ast> {
    #[inline]
    pub fn red(self, ast: &Ast<'ast>) -> u8 {
        ast.data(self).red
    }
    #[inline]
    pub fn set_red(self, ast: &mut Ast<'ast>, value: u8) {
        let mut data = ast.data(self);
        data.red = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn green(self, ast: &Ast<'ast>) -> u8 {
        ast.data(self).green
    }
    #[inline]
    pub fn set_green(self, ast: &mut Ast<'ast>, value: u8) {
        let mut data = ast.data(self);
        data.green = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn blue(self, ast: &Ast<'ast>) -> u8 {
        ast.data(self).blue
    }
    #[inline]
    pub fn set_blue(self, ast: &mut Ast<'ast>, value: u8) {
        let mut data = ast.data(self);
        data.blue = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn alpha(self, ast: &Ast<'ast>) -> u8 {
        ast.data(self).alpha
    }
    #[inline]
    pub fn set_alpha(self, ast: &mut Ast<'ast>, value: u8) {
        let mut data = ast.data(self);
        data.alpha = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for RGBA<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for RGBA<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_rgba(&mut self, data: RGBAData, span: Span) -> RGBA<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LABColor {
    Lab { a: f32, alpha: f32, b: f32, l: f32 },
    Lch { alpha: f32, c: f32, h: f32, l: f32 },
    Oklab { a: f32, alpha: f32, b: f32, l: f32 },
    Oklch { alpha: f32, c: f32, h: f32, l: f32 },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PredefinedColor {
    Srgb { alpha: f32, b: f32, g: f32, r: f32 },
    SrgbLinear { alpha: f32, b: f32, g: f32, r: f32 },
    DisplayP3 { alpha: f32, b: f32, g: f32, r: f32 },
    A98Rgb { alpha: f32, b: f32, g: f32, r: f32 },
    ProphotoRgb { alpha: f32, b: f32, g: f32, r: f32 },
    Rec2020 { alpha: f32, b: f32, g: f32, r: f32 },
    XyzD50 { alpha: f32, x: f32, y: f32, z: f32 },
    XyzD65 { alpha: f32, x: f32, y: f32, z: f32 },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FloatColor {
    Rgb { alpha: f32, b: f32, g: f32, r: f32 },
    Hsl { alpha: f32, h: f32, l: f32, s: f32 },
    Hwb { alpha: f32, b: f32, h: f32, w: f32 },
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LightDark<'ast>(pub(crate) NodeId<'ast, LightDarkData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightDarkData<'ast> {
    pub dark: CssColor<'ast>,
    pub light: CssColor<'ast>,
}
impl<'ast> NodeStorage<'ast> for LightDark<'ast> {
    const KIND: NodeKind = super::kind::KIND_LIGHT_DARK;
    type Data = LightDarkData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, LightDarkData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, LightDarkData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> LightDark<'ast> {
    #[inline]
    pub fn dark(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).dark
    }
    #[inline]
    pub fn set_dark(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.dark = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn light(self, ast: &Ast<'ast>) -> CssColor<'ast> {
        ast.data(self).light
    }
    #[inline]
    pub fn set_light(self, ast: &mut Ast<'ast>, value: CssColor<'ast>) {
        let mut data = ast.data(self);
        data.light = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for LightDark<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for LightDark<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_light_dark(&mut self, data: LightDarkData<'ast>, span: Span) -> LightDark<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemColor {
    Accentcolor,
    Accentcolortext,
    Activetext,
    Buttonborder,
    Buttonface,
    Buttontext,
    Canvas,
    Canvastext,
    Field,
    Fieldtext,
    Graytext,
    Highlight,
    Highlighttext,
    Linktext,
    Mark,
    Marktext,
    Selecteditem,
    Selecteditemtext,
    Visitedtext,
    Activeborder,
    Activecaption,
    Appworkspace,
    Background,
    Buttonhighlight,
    Buttonshadow,
    Captiontext,
    Inactiveborder,
    Inactivecaption,
    Inactivecaptiontext,
    Infobackground,
    Infotext,
    Menu,
    Menutext,
    Scrollbar,
    Threeddarkshadow,
    Threedface,
    Threedhighlight,
    Threedlightshadow,
    Threedshadow,
    Window,
    Windowframe,
    Windowtext,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnresolvedColor<'ast>(pub(crate) NodeId<'ast, UnresolvedColorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnresolvedColorData<'ast> {
    Rgb {
        alpha: &'ast [TokenOrValue<'ast>],
        b: f32,
        g: f32,
        r: f32,
    },
    Hsl {
        alpha: &'ast [TokenOrValue<'ast>],
        h: f32,
        l: f32,
        s: f32,
    },
    LightDark {
        dark: &'ast [TokenOrValue<'ast>],
        light: &'ast [TokenOrValue<'ast>],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnresolvedColorKind {
    Rgb,
    Hsl,
    LightDark,
}
impl<'ast> NodeStorage<'ast> for UnresolvedColor<'ast> {
    const KIND: NodeKind = super::kind::KIND_UNRESOLVED_COLOR;
    type Data = UnresolvedColorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, UnresolvedColorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, UnresolvedColorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> UnresolvedColor<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> UnresolvedColorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> UnresolvedColorKind {
        match ast.data(self) {
            UnresolvedColorData::Rgb { .. } => UnresolvedColorKind::Rgb,
            UnresolvedColorData::Hsl { .. } => UnresolvedColorKind::Hsl,
            UnresolvedColorData::LightDark { .. } => UnresolvedColorKind::LightDark,
        }
    }
}
impl<'ast> GetSpan<'ast> for UnresolvedColor<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for UnresolvedColor<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_unresolved_color(
        &mut self,
        data: UnresolvedColorData<'ast>,
        span: Span,
    ) -> UnresolvedColor<'ast> {
        self.alloc(data, span)
    }
}

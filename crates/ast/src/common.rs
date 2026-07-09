use super::*;

use rs_css_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum CssRule<'a> {
    Media {
        value: Box<'a, MediaRule<'a>>,
    },
    Import {
        value: Box<'a, ImportRule<'a>>,
    },
    Style {
        value: Box<'a, StyleRule<'a>>,
    },
    Keyframes {
        value: Box<'a, KeyframesRule<'a>>,
    },
    FontFace {
        value: Box<'a, FontFaceRule<'a>>,
    },
    FontPaletteValues {
        value: Box<'a, FontPaletteValuesRule<'a>>,
    },
    FontFeatureValues {
        value: Box<'a, FontFeatureValuesRule<'a>>,
    },
    Page {
        value: Box<'a, PageRule<'a>>,
    },
    Supports {
        value: Box<'a, SupportsRule<'a>>,
    },
    CounterStyle {
        value: Box<'a, CounterStyleRule<'a>>,
    },
    Namespace {
        value: Box<'a, NamespaceRule<'a>>,
    },
    MozDocument {
        value: Box<'a, MozDocumentRule<'a>>,
    },
    Nesting {
        value: Box<'a, NestingRule<'a>>,
    },
    NestedDeclarations {
        value: Box<'a, NestedDeclarationsRule<'a>>,
    },
    Viewport {
        value: Box<'a, ViewportRule<'a>>,
    },
    CustomMedia {
        value: Box<'a, CustomMediaRule<'a>>,
    },
    LayerStatement {
        value: Box<'a, LayerStatementRule<'a>>,
    },
    LayerBlock {
        value: Box<'a, LayerBlockRule<'a>>,
    },
    Property {
        value: Box<'a, PropertyRule<'a>>,
    },
    Container {
        value: Box<'a, ContainerRule<'a>>,
    },
    Scope {
        value: Box<'a, ScopeRule<'a>>,
    },
    StartingStyle {
        value: Box<'a, StartingStyleRule<'a>>,
    },
    ViewTransition {
        value: Box<'a, ViewTransitionRule<'a>>,
    },
    Ignored,
    Unknown {
        value: Box<'a, UnknownAtRule<'a>>,
    },
    Custom {
        value: Box<'a, DefaultAtRule>,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaCondition<'a> {
    Feature {
        value: Box<'a, QueryFeatureFor_MediaFeatureId<'a>>,
    },
    Not {
        value: Box<'a, MediaCondition<'a>>,
    },
    Operation {
        conditions: Vec<'a, Box<'a, MediaCondition<'a>>>,
        operator: Box<'a, Operator>,
    },
    Unknown {
        value: Vec<'a, Box<'a, TokenOrValue<'a>>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum QueryFeatureFor_MediaFeatureId<'a> {
    Plain {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
    },
    Range {
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        operator: Box<'a, MediaFeatureComparison>,
        value: Box<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: Box<'a, MediaFeatureValue<'a>>,
        end_operator: Box<'a, MediaFeatureComparison>,
        name: Box<'a, MediaFeatureNameFor_MediaFeatureId<'a>>,
        start: Box<'a, MediaFeatureValue<'a>>,
        start_operator: Box<'a, MediaFeatureComparison>,
    },
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureNameFor_MediaFeatureId<'a> {
    MediaFeatureId(Box<'a, MediaFeatureId>),
    CssString(&'a str),
    CssString2(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureId {
    Width,
    Height,
    AspectRatio,
    Orientation,
    OverflowBlock,
    OverflowInline,
    HorizontalViewportSegments,
    VerticalViewportSegments,
    DisplayMode,
    Resolution,
    Scan,
    Grid,
    Update,
    EnvironmentBlending,
    Color,
    ColorIndex,
    Monochrome,
    ColorGamut,
    DynamicRange,
    InvertedColors,
    Pointer,
    Hover,
    AnyPointer,
    AnyHover,
    NavControls,
    VideoColorGamut,
    VideoDynamicRange,
    Scripting,
    PrefersReducedMotion,
    PrefersReducedTransparency,
    PrefersContrast,
    ForcedColors,
    PrefersColorScheme,
    PrefersReducedData,
    DeviceWidth,
    DeviceHeight,
    DeviceAspectRatio,
    WebkitDevicePixelRatio,
    MozDevicePixelRatio,
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureValue<'a> {
    Length {
        value: Box<'a, Length<'a>>,
    },
    Number {
        value: f64,
    },
    Integer {
        value: f64,
    },
    Boolean {
        value: bool,
    },
    Resolution {
        value: Box<'a, Resolution>,
    },
    Ratio {
        value: Box<'a, Ratio>,
    },
    Ident {
        value: &'a str,
    },
    Env {
        value: Box<'a, EnvironmentVariable<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Length<'a> {
    Value { value: Box<'a, LengthValue<'a>> },
    Calc { value: Box<'a, CalcFor_Length<'a>> },
}

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Px,
    In,
    Cm,
    Mm,
    Q,
    Pt,
    Pc,
    Em,
    Rem,
    Ex,
    Rex,
    Ch,
    Rch,
    Cap,
    Rcap,
    Ic,
    Ric,
    Lh,
    Rlh,
    Vw,
    Lvw,
    Svw,
    Dvw,
    Cqw,
    Vh,
    Lvh,
    Svh,
    Dvh,
    Cqh,
    Vi,
    Svi,
    Lvi,
    Dvi,
    Cqi,
    Vb,
    Svb,
    Lvb,
    Dvb,
    Cqb,
    Vmin,
    Svmin,
    Lvmin,
    Dvmin,
    Cqmin,
    Vmax,
    Svmax,
    Lvmax,
    Dvmax,
    Cqmax,
}

#[derive(Debug, PartialEq)]
pub enum CalcFor_Length<'a> {
    Value {
        value: Box<'a, Length<'a>>,
    },
    Number {
        value: f64,
    },
    Sum {
        value: (Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>),
    },
    Product {
        value: (f64, Box<'a, CalcFor_Length<'a>>),
    },
    Function {
        value: Box<'a, MathFunctionFor_Length<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum MathFunctionFor_Length<'a> {
    Calc {
        value: Box<'a, CalcFor_Length<'a>>,
    },
    Min {
        value: Vec<'a, Box<'a, CalcFor_Length<'a>>>,
    },
    Max {
        value: Vec<'a, Box<'a, CalcFor_Length<'a>>>,
    },
    Clamp {
        value: (
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
        ),
    },
    Round {
        value: (
            Box<'a, RoundingStrategy>,
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
        ),
    },
    Rem {
        value: (Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>),
    },
    Mod {
        value: (Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>),
    },
    Abs {
        value: Box<'a, CalcFor_Length<'a>>,
    },
    Sign {
        value: Box<'a, CalcFor_Length<'a>>,
    },
    Hypot {
        value: Vec<'a, Box<'a, CalcFor_Length<'a>>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum RoundingStrategy {
    Nearest,
    Up,
    Down,
    ToZero,
}

#[derive(Debug, PartialEq)]
pub enum Resolution {
    Dpi { value: f64 },
    Dpcm { value: f64 },
    Dppx { value: f64 },
}

#[derive(Debug, PartialEq)]
pub struct Ratio(pub f64, pub f64);

#[derive(Debug, PartialEq)]
pub enum TokenOrValue<'a> {
    Token {
        value: Box<'a, Token<'a>>,
    },
    Color {
        value: Box<'a, CssColor<'a>>,
    },
    UnresolvedColor {
        value: Box<'a, UnresolvedColor<'a>>,
    },
    Url {
        value: Box<'a, Url<'a>>,
    },
    Var {
        value: Box<'a, Variable<'a>>,
    },
    Env {
        value: Box<'a, EnvironmentVariable<'a>>,
    },
    Function {
        value: Box<'a, Function<'a>>,
    },
    Length {
        value: Box<'a, LengthValue<'a>>,
    },
    Angle {
        value: Box<'a, Angle>,
    },
    Time {
        value: Box<'a, Time>,
    },
    Resolution {
        value: Box<'a, Resolution>,
    },
    DashedIdent {
        value: &'a str,
    },
    AnimationName {
        value: Box<'a, AnimationName<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Ident { value: &'a str },
    AtKeyword { value: &'a str },
    Hash { value: &'a str },
    IdHash { value: &'a str },
    String { value: &'a str },
    UnquotedUrl { value: &'a str },
    Delim { value: &'a str },
    Number { value: f64 },
    Percentage { value: f64 },
    Dimension { unit: &'a str, value: f64 },
    WhiteSpace { value: &'a str },
    Comment { value: &'a str },
    Colon,
    Semicolon,
    Comma,
    IncludeMatch,
    DashMatch,
    PrefixMatch,
    SuffixMatch,
    SubstringMatch,
    Cdo,
    Cdc,
    Function { value: &'a str },
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl { value: &'a str },
    BadString { value: &'a str },
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

#[derive(Debug, PartialEq)]
pub enum CssColor<'a> {
    CurrentColor(Box<'a, CurrentColor>),
    RGBColor(Box<'a, RGBColor>),
    LABColor(Box<'a, LABColor>),
    PredefinedColor(Box<'a, PredefinedColor>),
    FloatColor(Box<'a, FloatColor>),
    LightDark(Box<'a, LightDark<'a>>),
    SystemColor(Box<'a, SystemColor>),
}

#[derive(Debug, PartialEq)]
pub struct CurrentColor;

#[derive(Debug, PartialEq)]
pub struct RGBColor {
    pub alpha: f64,
    pub b: f64,
    pub g: f64,
    pub r: f64,
}

#[derive(Debug, PartialEq)]
pub enum LABColor {
    Lab { a: f64, alpha: f64, b: f64, l: f64 },
    Lch { alpha: f64, c: f64, h: f64, l: f64 },
    Oklab { a: f64, alpha: f64, b: f64, l: f64 },
    Oklch { alpha: f64, c: f64, h: f64, l: f64 },
}

#[derive(Debug, PartialEq)]
pub enum PredefinedColor {
    Srgb { alpha: f64, b: f64, g: f64, r: f64 },
    SrgbLinear { alpha: f64, b: f64, g: f64, r: f64 },
    DisplayP3 { alpha: f64, b: f64, g: f64, r: f64 },
    A98Rgb { alpha: f64, b: f64, g: f64, r: f64 },
    ProphotoRgb { alpha: f64, b: f64, g: f64, r: f64 },
    Rec2020 { alpha: f64, b: f64, g: f64, r: f64 },
    XyzD50 { alpha: f64, x: f64, y: f64, z: f64 },
    XyzD65 { alpha: f64, x: f64, y: f64, z: f64 },
}

#[derive(Debug, PartialEq)]
pub enum FloatColor {
    Rgb { alpha: f64, b: f64, g: f64, r: f64 },
    Hsl { alpha: f64, h: f64, l: f64, s: f64 },
    Hwb { alpha: f64, b: f64, h: f64, w: f64 },
}

#[derive(Debug, PartialEq)]
pub struct LightDark<'a> {
    pub dark: Box<'a, CssColor<'a>>,
    pub light: Box<'a, CssColor<'a>>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum UnresolvedColor<'a> {
    Rgb {
        alpha: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        b: f64,
        g: f64,
        r: f64,
    },
    Hsl {
        alpha: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        h: f64,
        l: f64,
        s: f64,
    },
    LightDark {
        dark: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        light: Vec<'a, Box<'a, TokenOrValue<'a>>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Specifier<'a> {
    Global,
    File { value: &'a str },
    SourceIndex { value: f64 },
}

#[derive(Debug, PartialEq)]
pub enum Angle {
    Deg { value: f64 },
    Rad { value: f64 },
    Grad { value: f64 },
    Turn { value: f64 },
}

#[derive(Debug, PartialEq)]
pub enum Time {
    Seconds { value: f64 },
    Milliseconds { value: f64 },
}

#[derive(Debug, PartialEq)]
pub enum AnimationName<'a> {
    None,
    Ident { value: &'a str },
    String { value: &'a str },
}

#[derive(Debug, PartialEq)]
pub enum EnvironmentVariableName<'a> {
    Ua {
        value: Box<'a, UAEnvironmentVariable>,
    },
    Custom {
        from: Option<Box<'a, Specifier<'a>>>,
        ident: &'a str,
    },
    Unknown {
        value: &'a str,
    },
}

#[derive(Debug, PartialEq)]
pub enum UAEnvironmentVariable {
    SafeAreaInsetTop,
    SafeAreaInsetRight,
    SafeAreaInsetBottom,
    SafeAreaInsetLeft,
    ViewportSegmentWidth,
    ViewportSegmentHeight,
    ViewportSegmentTop,
    ViewportSegmentLeft,
    ViewportSegmentBottom,
    ViewportSegmentRight,
}

#[derive(Debug, PartialEq)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
}

pub type MediaType<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum SupportsCondition<'a> {
    Not {
        value: Box<'a, SupportsCondition<'a>>,
    },
    And {
        value: Vec<'a, Box<'a, SupportsCondition<'a>>>,
    },
    Or {
        value: Vec<'a, Box<'a, SupportsCondition<'a>>>,
    },
    Declaration {
        property_id: Box<'a, PropertyId<'a>>,
        value: &'a str,
    },
    Selector {
        value: &'a str,
    },
    Unknown {
        value: &'a str,
    },
}

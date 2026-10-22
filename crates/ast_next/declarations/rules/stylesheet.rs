#[ast]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, Box<'a, MediaQuery<'a>>>,
}

#[ast]
pub struct MediaQuery<'a> {
    pub condition: Option<MediaCondition<'a>>,
    pub media_type: MediaType<'a>,
    pub qualifier: Option<Qualifier>,
}

#[ast]
pub struct LengthValue {
    pub unit: LengthUnit,
    pub value: f32,
}

#[ast]
pub struct EnvironmentVariable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub indices: Vec<'a, i32>,
    pub name: EnvironmentVariableName<'a>,
}

#[ast]
pub struct Url<'a> {
    pub url: &'a str,
}

#[ast]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: Box<'a, DashedIdentReference<'a>>,
}

#[ast]
pub struct DashedIdentReference<'a> {
    pub from: Option<Specifier<'a>>,
    pub ident: &'a str,
}

#[ast]
pub struct Function<'a> {
    pub arguments: Vec<'a, TokenOrValue<'a>>,
    flags: FunctionFlags,
    kind: KnownFunction,
    name: &'a str,
    /// A simple value serialized from this existing function node.
    pub replacement: Option<FunctionReplacement>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A function name recognized by RocketCSS.
///
/// The original function name remains on [`Function`] so parsing and code
/// generation stay lossless. This enum gives downstream passes a shared,
/// ASCII case-insensitive identity without repeating string matching.
#[repr(u8)]
pub enum KnownFunction {
    Abs,
    Calc,
    Clamp,
    Color,
    ColorMix,
    Constant,
    ConicGradient,
    CubicBezier,
    Env,
    Frames,
    Hsl,
    Hsla,
    Hwb,
    Hypot,
    Lab,
    Lch,
    Linear,
    LinearGradient,
    Local,
    Matrix,
    Matrix3d,
    Max,
    Min,
    Mod,
    RadialGradient,
    Rem,
    RepeatingConicGradient,
    RepeatingLinearGradient,
    RepeatingRadialGradient,
    Rgb,
    Rgba,
    Rotate,
    RotateX,
    RotateY,
    Rotate3d,
    RotateZ,
    Round,
    Scale,
    ScaleX,
    ScaleY,
    ScaleZ,
    Scale3d,
    Sign,
    Steps,
    Translate,
    TranslateY,
    TranslateZ,
    Translate3d,
    Url,
    Var,
    #[default]
    Unknown,
}

#[ast]
pub enum FunctionReplacement {
    GrayAlpha {
        alpha: f32,
        lightness: f32,
    },
    Number(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    Percentage(f32),
    Rgb {
        blue: u8,
        green: u8,
        red: u8,
    },
    Rgba {
        alpha: f32,
        blue: u8,
        green: u8,
        red: u8,
        use_hex: bool,
    },
}

#[ast]
pub struct ImportRule<'a> {
    pub layer: Option<Vec<'a, &'a str>>,
    pub media: Option<Box<'a, MediaList<'a>>>,
    pub supports: Option<Box<'a, SupportsCondition<'a>>>,
    pub url: &'a str,
}

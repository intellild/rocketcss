use crate::*;

use bitflags::bitflags;
use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug, Default, PartialEq, Visit)]
pub struct DefaultAtRule;

#[derive(Debug, PartialEq, Visit)]
pub struct StyleSheet<'a> {
    pub license_comments: Vec<'a, &'a str>,
    pub rules: Vec<'a, CssRule<'a>>,
    pub source_map_urls: Vec<'a, Option<&'a str>>,
    pub sources: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaRule<'a> {
    pub span: Span,
    pub query: MediaList<'a>,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, MediaQuery<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaQuery<'a> {
    pub condition: Option<Box<'a, MediaCondition<'a>>>,
    pub media_type: MediaType<'a>,
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LengthValue {
    pub unit: LengthUnit,
    pub value: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct EnvironmentVariable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub indices: Vec<'a, i32>,
    pub name: Box<'a, EnvironmentVariableName<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Url<'a> {
    pub span: Span,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: Box<'a, DashedIdentReference<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DashedIdentReference<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub ident: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Function<'a> {
    pub arguments: Vec<'a, TokenOrValue<'a>>,
    #[visit(skip)]
    flags: FunctionFlags,
    #[visit(skip)]
    kind: KnownFunction,
    name: &'a str,
    /// A simple value serialized from this existing function node.
    pub replacement: Option<FunctionReplacement>,
}

/// A function name recognized by RocketCSS.
///
/// The original function name remains on [`Function`] so parsing and code
/// generation stay lossless. This enum gives downstream passes a shared,
/// ASCII case-insensitive identity without repeating string matching.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

impl KnownFunction {
    /// Resolves a function name using CSS ASCII case-insensitive matching.
    pub fn from_name(name: &str) -> Self {
        Self::classify(name).0
    }

    fn classify(name: &str) -> (Self, bool) {
        let kind = Self::from_unprefixed_name(name);
        if kind != Self::Unknown {
            return (kind, false);
        }

        let unprefixed_name = name
            .strip_prefix('-')
            .and_then(|name| name.split_once('-').map(|(_, name)| name));
        let Some(unprefixed_name) = unprefixed_name else {
            return (Self::Unknown, false);
        };
        let kind = Self::from_unprefixed_name(unprefixed_name);
        if kind.is_math() || kind.is_gradient() {
            (kind, true)
        } else {
            (Self::Unknown, false)
        }
    }

    fn from_unprefixed_name(name: &str) -> Self {
        match_ignore_ascii_case!(
            name,
            "abs" => Self::Abs,
            "calc" => Self::Calc,
            "clamp" => Self::Clamp,
            "color" => Self::Color,
            "color-mix" => Self::ColorMix,
            "constant" => Self::Constant,
            "conic-gradient" => Self::ConicGradient,
            "cubic-bezier" => Self::CubicBezier,
            "env" => Self::Env,
            "frames" => Self::Frames,
            "hsl" => Self::Hsl,
            "hsla" => Self::Hsla,
            "hwb" => Self::Hwb,
            "hypot" => Self::Hypot,
            "lab" => Self::Lab,
            "lch" => Self::Lch,
            "linear" => Self::Linear,
            "linear-gradient" => Self::LinearGradient,
            "local" => Self::Local,
            "matrix" => Self::Matrix,
            "matrix3d" => Self::Matrix3d,
            "max" => Self::Max,
            "min" => Self::Min,
            "mod" => Self::Mod,
            "radial-gradient" => Self::RadialGradient,
            "rem" => Self::Rem,
            "repeating-conic-gradient" => Self::RepeatingConicGradient,
            "repeating-linear-gradient" => Self::RepeatingLinearGradient,
            "repeating-radial-gradient" => Self::RepeatingRadialGradient,
            "rgb" => Self::Rgb,
            "rgba" => Self::Rgba,
            "rotate" => Self::Rotate,
            "rotatex" => Self::RotateX,
            "rotatey" => Self::RotateY,
            "rotate3d" => Self::Rotate3d,
            "rotatez" => Self::RotateZ,
            "round" => Self::Round,
            "scale" => Self::Scale,
            "scalex" => Self::ScaleX,
            "scaley" => Self::ScaleY,
            "scalez" => Self::ScaleZ,
            "scale3d" => Self::Scale3d,
            "sign" => Self::Sign,
            "steps" => Self::Steps,
            "translate" => Self::Translate,
            "translatey" => Self::TranslateY,
            "translatez" => Self::TranslateZ,
            "translate3d" => Self::Translate3d,
            "url" => Self::Url,
            "var" => Self::Var,
            _ => Self::Unknown,
        )
    }

    /// Returns whether this function participates in math value parsing.
    pub const fn is_math(self) -> bool {
        matches!(
            self,
            Self::Abs
                | Self::Calc
                | Self::Clamp
                | Self::Hypot
                | Self::Max
                | Self::Min
                | Self::Mod
                | Self::Rem
                | Self::Round
                | Self::Sign
        )
    }

    /// Returns whether this function is accepted as a basic calculated value.
    pub const fn is_math_value(self) -> bool {
        matches!(self, Self::Calc | Self::Min | Self::Max | Self::Clamp)
    }

    /// Returns whether this is a gradient function.
    pub const fn is_gradient(self) -> bool {
        matches!(
            self,
            Self::LinearGradient
                | Self::RepeatingLinearGradient
                | Self::RadialGradient
                | Self::RepeatingRadialGradient
                | Self::ConicGradient
                | Self::RepeatingConicGradient
        )
    }

    /// Returns whether this function resolves a variable or environment value.
    pub const fn is_variable(self) -> bool {
        matches!(self, Self::Var | Self::Env | Self::Constant)
    }

    /// Returns whether this is a color function handled by the minifier.
    pub const fn is_color(self) -> bool {
        matches!(
            self,
            Self::Rgb
                | Self::Rgba
                | Self::Hsl
                | Self::Hsla
                | Self::Hwb
                | Self::Lab
                | Self::Lch
                | Self::Color
        )
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct FunctionFlags: u8 {
        /// This node was reduced to an identifier during minification.
        ///
        /// Keeping the replacement in the existing function allocation avoids
        /// allocating a new token solely to change the surrounding enum variant.
        const IS_IDENTIFIER = 1 << 0;
        /// Emit a quoted `url()` argument directly when it is safe to unquote.
        const UNQUOTED_URL = 1 << 1;
        /// The known identity was resolved after removing a vendor prefix.
        const VENDOR_PREFIXED = 1 << 2;
    }
}

impl<'a> Function<'a> {
    /// Creates a function with no minifier serialization state.
    #[inline]
    pub fn new(name: &'a str, arguments: Vec<'a, TokenOrValue<'a>>) -> Self {
        let (kind, vendor_prefixed) = KnownFunction::classify(name);
        let mut flags = FunctionFlags::empty();
        flags.set(FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
        Self {
            arguments,
            flags,
            kind,
            name,
            replacement: None,
        }
    }

    /// Returns the original function name.
    #[inline]
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// Returns the shared identity for a recognized function name.
    #[inline]
    pub const fn kind(&self) -> KnownFunction {
        self.kind
    }

    /// Updates the lossless function name and its recognized identity together.
    #[inline]
    pub fn set_name(&mut self, name: &'a str) {
        let (kind, vendor_prefixed) = KnownFunction::classify(name);
        self.name = name;
        self.kind = kind;
        self.flags
            .set(FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
    }

    /// Returns whether the known identity came from a vendor-prefixed name.
    #[inline]
    pub const fn is_vendor_prefixed(&self) -> bool {
        self.flags.contains(FunctionFlags::VENDOR_PREFIXED)
    }

    /// Returns whether this function serializes as an identifier.
    #[inline]
    pub const fn is_identifier(&self) -> bool {
        self.flags.contains(FunctionFlags::IS_IDENTIFIER)
    }

    /// Controls whether this function serializes as an identifier.
    #[inline]
    pub fn set_identifier(&mut self, is_identifier: bool) {
        self.flags.set(FunctionFlags::IS_IDENTIFIER, is_identifier);
    }

    /// Returns whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub const fn is_unquoted_url(&self) -> bool {
        self.flags.contains(FunctionFlags::UNQUOTED_URL)
    }

    /// Controls whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub fn set_unquoted_url(&mut self, unquoted_url: bool) {
        self.flags.set(FunctionFlags::UNQUOTED_URL, unquoted_url);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
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

#[derive(Debug, PartialEq, Visit)]
pub struct ImportRule<'a> {
    pub layer: Option<Vec<'a, &'a str>>,
    pub span: Span,
    pub media: Option<Box<'a, MediaList<'a>>>,
    pub supports: Option<Box<'a, SupportsCondition<'a>>>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StyleRule<'a> {
    pub declarations: Pin<Box<'a, DeclarationBlock<'a>>>,
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
    pub selectors: Box<'a, SelectorList<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
#[visit(pinned)]
pub struct DeclarationBlock<'a> {
    pub declarations: Vec<'a, Declaration<'a>>,
    #[visit(skip)]
    pub declarations_importance: BitVec<'a>,
    #[visit(skip)]
    _pin: PhantomPinned,
}

impl<'a> DeclarationBlock<'a> {
    #[inline]
    pub fn new(allocator: &'a rocketcss_allocator::Allocator) -> Self {
        Self {
            declarations: allocator.vec(),
            declarations_importance: BitVec::new(allocator),
            _pin: PhantomPinned,
        }
    }

    #[inline]
    pub fn push(&mut self, declaration: Declaration<'a>, important: bool) {
        self.declarations.push(declaration);
        self.declarations_importance.push(important);
    }

    #[inline]
    pub fn push_pinned(mut self: Pin<&mut Self>, declaration: Declaration<'a>, important: bool) {
        // SAFETY: pushing into the declaration vector does not move the block.
        unsafe { self.as_mut().get_unchecked_mut() }.push(declaration, important);
    }

    #[inline]
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_important(&self, index: usize) -> bool {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations_importance.is_set(index)
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&Declaration<'a>, bool)> {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations
            .iter()
            .zip(self.declarations_importance.iter())
    }
}

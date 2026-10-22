//! Hand-written helpers for plain AST values.

use crate::{Function, FunctionData, KnownFunction, RGBAData, TokenOrValue, VendorPrefix};

impl VendorPrefix {
    pub const NONE: Self = Self { bits: 1 };
    pub const WEBKIT: Self = Self { bits: 1 << 1 };
    pub const MOZ: Self = Self { bits: 1 << 2 };
    pub const MS: Self = Self { bits: 1 << 3 };
    pub const O: Self = Self { bits: 1 << 4 };

    #[inline]
    pub const fn is_none(self) -> bool {
        self.bits == Self::NONE.bits
    }

    /// Splits a vendor-prefixed property or rule name.
    pub fn split_from_name(name: &str) -> Option<(Self, &str)> {
        let rest = name.strip_prefix('-')?;
        let (prefix, rest) = rest.split_once('-')?;
        let prefix = if prefix.eq_ignore_ascii_case("webkit") {
            Self::WEBKIT
        } else if prefix.eq_ignore_ascii_case("moz") {
            Self::MOZ
        } else if prefix.eq_ignore_ascii_case("ms") {
            Self::MS
        } else if prefix.eq_ignore_ascii_case("o") {
            Self::O
        } else {
            return None;
        };
        Some((prefix, rest))
    }
}

impl Default for VendorPrefix {
    #[inline]
    fn default() -> Self {
        Self::NONE
    }
}

impl crate::FunctionFlags {
    pub const IS_IDENTIFIER: u8 = 1 << 0;
    pub const UNQUOTED_URL: u8 = 1 << 1;
    pub const VENDOR_PREFIXED: u8 = 1 << 2;
    pub const VALID_RGB: u8 = 1 << 3;

    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    #[inline]
    pub const fn contains(self, flag: u8) -> bool {
        self.bits & flag != 0
    }

    #[inline]
    pub fn set(&mut self, flag: u8, enabled: bool) {
        if enabled {
            self.bits |= flag;
        } else {
            self.bits &= !flag;
        }
    }
}

impl KnownFunction {
    /// Resolves a function name using CSS ASCII case-insensitive matching.
    pub fn from_name(name: &str) -> Self {
        Self::classify(name).0
    }

    pub fn classify(name: &str) -> (Self, bool) {
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
        crate::match_ignore_ascii_case!(
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

    pub const fn is_math_value(self) -> bool {
        matches!(self, Self::Calc | Self::Min | Self::Max | Self::Clamp)
    }

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

    pub const fn is_variable(self) -> bool {
        matches!(self, Self::Var | Self::Env | Self::Constant)
    }

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

impl<'ast> Function<'ast> {
    /// Creates a function with no minifier serialization state.
    pub fn new(
        name: &'ast str,
        arguments: &'ast [TokenOrValue<'ast>],
        ast: &mut crate::Ast<'ast>,
        span: crate::Span,
    ) -> Self {
        let (kind, vendor_prefixed) = KnownFunction::classify(name);
        let mut flags = crate::FunctionFlags::empty();
        flags.set(crate::FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
        ast.alloc_function(
            FunctionData {
                arguments,
                flags,
                kind,
                name,
                replacement: None,
            },
            span,
        )
    }
}

impl crate::KnownColor {
    pub fn from_name(name: &str) -> Option<Self> {
        crate::known_colors::from_name(name)
    }

    pub fn name(self) -> &'static str {
        crate::known_colors::name(self)
    }
}

impl<'ast> crate::FontFamily<'ast> {
    pub fn is_generic_name(name: &str) -> bool {
        crate::match_ignore_ascii_case!(
            name,
            "serif" | "sans-serif" | "cursive" | "fantasy" | "monospace"
                | "system-ui" | "emoji" | "math" | "fangsong" | "ui-serif"
                | "ui-sans-serif" | "ui-monospace" | "ui-rounded" | "initial"
                | "inherit" | "unset" | "default" | "revert" | "revert-layer" => true,
            _ => false,
        )
    }

    pub fn from_name(
        name: &'ast str,
        ast: &mut crate::Ast<'ast>,
        span: crate::Span,
    ) -> Self {
        let data = crate::match_ignore_ascii_case!(
            name,
            "serif" => crate::FontFamilyData::Serif,
            "sans-serif" => crate::FontFamilyData::SansSerif,
            "cursive" => crate::FontFamilyData::Cursive,
            "fantasy" => crate::FontFamilyData::Fantasy,
            "monospace" => crate::FontFamilyData::Monospace,
            "system-ui" => crate::FontFamilyData::SystemUi,
            "emoji" => crate::FontFamilyData::Emoji,
            "math" => crate::FontFamilyData::Math,
            "fangsong" => crate::FontFamilyData::Fangsong,
            "ui-serif" => crate::FontFamilyData::UiSerif,
            "ui-sans-serif" => crate::FontFamilyData::UiSansSerif,
            "ui-monospace" => crate::FontFamilyData::UiMonospace,
            "ui-rounded" => crate::FontFamilyData::UiRounded,
            "initial" => crate::FontFamilyData::Initial,
            "inherit" => crate::FontFamilyData::Inherit,
            "unset" => crate::FontFamilyData::Unset,
            "default" => crate::FontFamilyData::Default,
            "revert" => crate::FontFamilyData::Revert,
            "revert-layer" => crate::FontFamilyData::RevertLayer,
            _ => crate::FontFamilyData::Custom(name),
        );
        ast.alloc_font_family(data, span)
    }
}

/// A color channel tuple returned by [`crate::KnownColor::rgba`].
#[inline]
pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> RGBAData {
    RGBAData {
        red,
        green,
        blue,
        alpha,
    }
}

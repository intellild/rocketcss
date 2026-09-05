use crate::*;

use bitflags::bitflags;

mod compilation;
pub use compilation::*;

#[derive(Debug, PartialEq, Visit)]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, NodeId<'a, MediaQuery<'a>>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaQuery<'a> {
    pub condition: Option<MediaCondition<'a>>,
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
    pub name: EnvironmentVariableName<'a>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Url<'a> {
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: NodeId<'a, DashedIdentReference<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DashedIdentReference<'a> {
    pub from: Option<Specifier<'a>>,
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

// Fixed payload layout for `Function`:
//
// bytes 0..4   arguments range start
// bytes 4..8   arguments range end
// byte 8       FunctionFlags
// bytes 9..12  reserved
// bytes 12..16 first extra slot
//
// extra + 0    compact string ID for the lossless function name
// extra + 1..3 fixed-width optional FunctionReplacement
impl<'ast> AstNodeStorage<'ast> for Function<'ast> {
    const KIND: NodeKind = NodeKind::new(2);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let start = u32::from_le_bytes(
            bytes[..4]
                .try_into()
                .expect("Function range start is four bytes"),
        ) as usize;
        let end = u32::from_le_bytes(
            bytes[4..8]
                .try_into()
                .expect("Function range end is four bytes"),
        ) as usize;
        let extra = payload.extra_start();
        let name = context.resolve_string(context.extra_slot(extra).as_u64());
        Self {
            arguments: context.encoded_vec_range(start, end),
            flags: FunctionFlags::from_bits_retain(bytes[8]),
            kind: KnownFunction::from_name(name),
            name,
            replacement: decode_function_replacement(
                context.extra_slot(extra + 1),
                context.extra_slot(extra + 2),
            ),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_function(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_function(self, Some(current.extra_start()), context)
    }
}

fn encode_function<'ast>(
    function: Function<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let start = u32::try_from(function.arguments.start_index())
        .expect("Function argument range start exceeds four bytes");
    let end = u32::try_from(function.arguments.end_index())
        .expect("Function argument range end exceeds four bytes");
    let name = ExtraData::from_u64(context.store_string(function.name) as u64);
    let (replacement_low, replacement_high) = encode_function_replacement(function.replacement);
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, name);
            context.set_extra_slot(extra + 1, replacement_low);
            context.set_extra_slot(extra + 2, replacement_high);
            extra
        }
        None => context.alloc_extra_slots([name, replacement_low, replacement_high]),
    };

    let mut inline = [0; NodePayload::PARTIAL_INLINE_BYTES];
    inline[..4].copy_from_slice(&start.to_le_bytes());
    inline[4..8].copy_from_slice(&end.to_le_bytes());
    inline[8] = function.flags.bits();
    NodePayload::with_extra(&inline, extra)
}

fn encode_function_replacement(replacement: Option<FunctionReplacement>) -> (ExtraData, ExtraData) {
    let mut bytes = [0; 16];
    match replacement {
        None => {}
        Some(FunctionReplacement::GrayAlpha { alpha, lightness }) => {
            bytes[0] = 1;
            bytes[4..8].copy_from_slice(&alpha.to_bits().to_le_bytes());
            bytes[8..12].copy_from_slice(&lightness.to_bits().to_le_bytes());
        }
        Some(FunctionReplacement::Number(value)) => {
            bytes[0] = 2;
            bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
        }
        Some(FunctionReplacement::Dimension { unit, value }) => {
            bytes[0] = 3;
            bytes[1] = crate::token::encode_unit(unit);
            bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
        }
        Some(FunctionReplacement::Percentage(value)) => {
            bytes[0] = 4;
            bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
        }
        Some(FunctionReplacement::Rgb { blue, green, red }) => {
            bytes[0] = 5;
            bytes[1] = red;
            bytes[2] = green;
            bytes[3] = blue;
        }
        Some(FunctionReplacement::Rgba {
            alpha,
            blue,
            green,
            red,
            use_hex,
        }) => {
            bytes[0] = 6;
            bytes[1] = red;
            bytes[2] = green;
            bytes[3] = blue;
            bytes[4..8].copy_from_slice(&alpha.to_bits().to_le_bytes());
            bytes[8] = use_hex as u8;
        }
    }
    (
        ExtraData::from_bytes(&bytes[..8]),
        ExtraData::from_bytes(&bytes[8..]),
    )
}

fn decode_function_replacement(low: ExtraData, high: ExtraData) -> Option<FunctionReplacement> {
    let mut bytes = [0; 16];
    bytes[..8].copy_from_slice(&low.bytes());
    bytes[8..].copy_from_slice(&high.bytes());
    let value = f32::from_bits(u32::from_le_bytes(
        bytes[4..8]
            .try_into()
            .expect("Function replacement value is four bytes"),
    ));
    match bytes[0] {
        0 => None,
        1 => Some(FunctionReplacement::GrayAlpha {
            alpha: value,
            lightness: f32::from_bits(u32::from_le_bytes(
                bytes[8..12]
                    .try_into()
                    .expect("Function lightness is four bytes"),
            )),
        }),
        2 => Some(FunctionReplacement::Number(value)),
        3 => Some(FunctionReplacement::Dimension {
            unit: crate::token::decode_unit(bytes[1]),
            value,
        }),
        4 => Some(FunctionReplacement::Percentage(value)),
        5 => Some(FunctionReplacement::Rgb {
            blue: bytes[3],
            green: bytes[2],
            red: bytes[1],
        }),
        6 => Some(FunctionReplacement::Rgba {
            alpha: value,
            blue: bytes[3],
            green: bytes[2],
            red: bytes[1],
            use_hex: match bytes[8] {
                0 => false,
                1 => true,
                _ => panic!("invalid encoded FunctionReplacement::Rgba flag"),
            },
        }),
        _ => panic!("invalid encoded FunctionReplacement variant"),
    }
}

/// A function name recognized by RocketCSS.
///
/// The original function name remains on [`Function`] so parsing and code
/// generation stay lossless. This enum gives downstream passes a shared,
/// ASCII case-insensitive identity without repeating string matching.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Visit)]
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
        /// The parser proved that this `rgb()` or `rgba()` token list is a
        /// statically valid form supported by the color minifier.
        const VALID_RGB = 1 << 3;
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
        self.flags.remove(FunctionFlags::VALID_RGB);
    }

    /// Returns whether the known identity came from a vendor-prefixed name.
    #[inline]
    pub const fn is_vendor_prefixed(&self) -> bool {
        self.flags.contains(FunctionFlags::VENDOR_PREFIXED)
    }

    /// Returns whether this `rgb()` or `rgba()` token list was validated by
    /// the parser and can be consumed by the color minifier.
    #[inline]
    pub const fn is_valid_rgb(&self) -> bool {
        self.flags.contains(FunctionFlags::VALID_RGB)
    }

    /// Records the parser's validation result for an `rgb()` or `rgba()`
    /// function without changing its lossless token representation.
    #[inline]
    pub fn set_valid_rgb(&mut self, valid: bool) {
        self.flags.set(FunctionFlags::VALID_RGB, valid);
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
    pub media: Option<NodeId<'a, MediaList<'a>>>,
    pub supports: Option<NodeId<'a, SupportsCondition<'a>>>,
    pub url: &'a str,
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{AstContext, DUMMY_SP, Function, FunctionReplacement, KnownFunction, Unit};

    #[test]
    fn function_codec_uses_fixed_overflow_slots_and_compact_string_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let arguments = context.alloc_vec(allocator.vec());
        let mut function = Function::new("-webkit-calc", arguments);
        function.set_identifier(true);
        function.set_valid_rgb(true);
        function.replacement = Some(FunctionReplacement::Dimension {
            unit: Unit::Length(crate::LengthUnit::Cqmax),
            value: 12.5,
        });
        let id = context.alloc_encoded_node(function, DUMMY_SP);

        let decoded = context.encoded_node(id);
        assert_eq!(decoded.arguments, arguments);
        assert_eq!(decoded.name(), "-webkit-calc");
        assert_eq!(decoded.kind(), KnownFunction::Calc);
        assert!(decoded.is_vendor_prefixed());
        assert!(decoded.is_identifier());
        assert!(decoded.is_valid_rgb());
        assert_eq!(
            decoded.replacement,
            Some(FunctionReplacement::Dimension {
                unit: Unit::Length(crate::LengthUnit::Cqmax),
                value: 12.5,
            })
        );
        assert_eq!(context.encoded_extra_len(), 3);

        context.mutate_encoded_node(id, |function, _| {
            function.set_name("rgb");
            function.set_identifier(false);
            function.replacement = Some(FunctionReplacement::Rgba {
                alpha: 0.5,
                blue: 3,
                green: 2,
                red: 1,
                use_hex: true,
            });
        });

        let decoded = context.encoded_node(id);
        assert_eq!(decoded.name(), "rgb");
        assert_eq!(decoded.kind(), KnownFunction::Rgb);
        assert!(!decoded.is_vendor_prefixed());
        assert!(!decoded.is_identifier());
        assert_eq!(
            decoded.replacement,
            Some(FunctionReplacement::Rgba {
                alpha: 0.5,
                blue: 3,
                green: 2,
                red: 1,
                use_hex: true,
            })
        );
        assert_eq!(
            context.encoded_extra_len(),
            3,
            "same-kind mutation must reuse fixed overflow slots"
        );
    }
}

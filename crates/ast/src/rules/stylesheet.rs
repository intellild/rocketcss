use crate::*;

use bitflags::bitflags;

mod compilation;
pub use compilation::*;

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, NodeId<'a, MediaQuery<'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct MediaQuery<'a> {
    pub condition: Option<NodeId<'a, MediaCondition<'a>>>,
    pub media_type: MediaType<'a>,
    pub qualifier: Option<Qualifier>,
}

impl_inline_node!(MediaList<'ast>, 0x001a_000a);

impl<'ast> AstNodeClone<'ast> for MediaList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            media_queries: context.clone_encoded_vec(self.media_queries),
        }
    }
}

// repr(u8) places the qualifier beside the tag before aligned handles/ranges.
// This compresses the logical query without serializing primitive fields.
#[repr(u8)]
#[derive(Clone, Copy)]
enum MediaQuerySlot<'a> {
    All {
        qualifier: Option<Qualifier>,
        condition: Option<NodeId<'a, MediaCondition<'a>>>,
    },
    Print {
        qualifier: Option<Qualifier>,
        condition: Option<NodeId<'a, MediaCondition<'a>>>,
    },
    Screen {
        qualifier: Option<Qualifier>,
        condition: Option<NodeId<'a, MediaCondition<'a>>>,
    },
    Custom {
        qualifier: Option<Qualifier>,
        condition: Option<NodeId<'a, MediaCondition<'a>>>,
        name: AstStr<'a>,
    },
}
// SAFETY: this KIND always stores and reads MediaQuerySlot.
unsafe impl<'ast> AstNodeStorage<'ast> for MediaQuery<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_000b);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.condition == other.condition
            && self.qualifier == other.qualifier
            && match (self.media_type, other.media_type) {
                (MediaType::Custom(a), MediaType::Custom(b)) => context.str(a) == context.str(b),
                (a, b) => a == b,
            }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        match unsafe { payload.read_value::<MediaQuerySlot<'ast>>() } {
            MediaQuerySlot::All {
                qualifier,
                condition,
            } => Self {
                qualifier,
                condition,
                media_type: MediaType::All,
            },
            MediaQuerySlot::Print {
                qualifier,
                condition,
            } => Self {
                qualifier,
                condition,
                media_type: MediaType::Print,
            },
            MediaQuerySlot::Screen {
                qualifier,
                condition,
            } => Self {
                qualifier,
                condition,
                media_type: MediaType::Screen,
            },
            MediaQuerySlot::Custom {
                qualifier,
                condition,
                name,
            } => Self {
                qualifier,
                condition,
                media_type: MediaType::Custom(name),
            },
        }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        self.into_payload()
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.into_payload()
    }
}
impl MediaQuery<'_> {
    fn into_payload(self) -> NodePayload {
        let Self {
            condition,
            qualifier,
            media_type,
        } = self;
        NodePayload::from_value(match media_type {
            MediaType::All => MediaQuerySlot::All {
                qualifier,
                condition,
            },
            MediaType::Print => MediaQuerySlot::Print {
                qualifier,
                condition,
            },
            MediaType::Screen => MediaQuerySlot::Screen {
                qualifier,
                condition,
            },
            MediaType::Custom(name) => MediaQuerySlot::Custom {
                qualifier,
                condition,
                name,
            },
        })
    }
}

impl<'ast> AstNodeClone<'ast> for MediaQuery<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            condition: self
                .condition
                .map(|condition| context.clone_encoded_node(condition)),
            media_type: self.media_type,
            qualifier: self.qualifier,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
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

#[repr(C)]
#[derive(Clone, Copy)]
struct EnvironmentVariableHeader<'a> {
    name: EnvironmentVariableName<'a>,
    extra: u32,
}

pub use environment_access::EnvironmentVariableRead;

// Transient field views are excluded from persistent AST visitor generation.
mod environment_access {
    use super::*;

    pub struct EnvironmentVariableRead<'context, 'storage, 'ast> {
        context: &'context AstContext<'storage>,
        header: EnvironmentVariableHeader<'ast>,
    }

    impl<'ast> EnvironmentVariableRead<'_, '_, 'ast> {
        pub fn name(&self) -> EnvironmentVariableName<'ast> {
            self.header.name
        }

        pub fn indices(&self) -> Vec<'ast, i32> {
            // SAFETY: the first overflow slot stores the native indices range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }

        pub fn fallback(&self) -> Option<Vec<'ast, TokenOrValue<'ast>>> {
            // SAFETY: the second slot uses the matching optional-range layout.
            unsafe {
                Option::<Vec<'ast, TokenOrValue<'ast>>>::decode_extra(
                    self.context.extra_slot(self.header.extra as usize + 1),
                )
            }
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn environment_variable<'id>(
            &self,
            id: NodeId<'id, EnvironmentVariable<'id>>,
        ) -> EnvironmentVariableRead<'_, 'storage, 'id> {
            // SAFETY: the typed node validates the kind before reading its header.
            EnvironmentVariableRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: KIND stores EnvironmentVariableHeader. Its two overflow slots hold
// a native indices range and an Option<AstVec> written by encode_extra.
unsafe impl<'ast> AstNodeStorage<'ast> for EnvironmentVariable<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0004_0005);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: EnvironmentVariableHeader<'ast> = unsafe { payload.read_value() };
        let extra = header.extra as usize;
        Self {
            name: header.name,
            indices: unsafe { context.extra_slot(extra).read_value() },
            fallback: unsafe {
                Option::<Vec<'ast, TokenOrValue<'ast>>>::decode_extra(context.extra_slot(extra + 1))
            },
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_environment_variable(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: EnvironmentVariableHeader<'ast> = unsafe { current.read_value() };
        encode_environment_variable(self, Some(header.extra as usize), context)
    }

    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.fallback == other.fallback
            && self.indices == other.indices
            && match (&self.name, &other.name) {
                (
                    EnvironmentVariableName::Unknown(left),
                    EnvironmentVariableName::Unknown(right),
                ) => left == right || context.str(*left) == context.str(*right),
                _ => self.name == other.name,
            }
    }
}

impl<'ast> AstNodeClone<'ast> for EnvironmentVariable<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fallback: self
                .fallback
                .map(|fallback| context.clone_encoded_vec(fallback)),
            indices: context.clone_encoded_vec(self.indices),
            name: match self.name {
                EnvironmentVariableName::UA(value) => EnvironmentVariableName::UA(value),
                EnvironmentVariableName::Custom(value) => {
                    EnvironmentVariableName::Custom(context.clone_encoded_node(value))
                }
                EnvironmentVariableName::Unknown(value) => EnvironmentVariableName::Unknown(value),
            },
        }
    }
}

fn encode_environment_variable<'ast>(
    value: EnvironmentVariable<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let indices = ExtraData::from_value(value.indices);
    let fallback = value.fallback.encode_extra();
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, indices);
            context.set_extra_slot(extra + 1, fallback);
            extra
        }
        None => context.alloc_extra_slots([indices, fallback]),
    };
    NodePayload::from_value(EnvironmentVariableHeader {
        name: value.name,
        extra: u32::try_from(extra).expect("EnvironmentVariable overflow index exceeds u32"),
    })
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct Url<'a> {
    pub url: AstStr<'a>,
}

unsafe impl<'ast> AstNodeStorage<'ast> for Url<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0004_0002);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.url == other.url || context.str(self.url) == context.str(other.url)
    }

    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }

    #[inline]
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }

    #[inline]
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for Url<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: NodeId<'a, DashedIdentReference<'a>>,
}

impl_inline_node!(Variable<'ast>, 0x0004_0003);

impl<'ast> AstNodeClone<'ast> for Variable<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            fallback: self
                .fallback
                .map(|fallback| context.clone_encoded_vec(fallback)),
            name: context.clone_encoded_node(self.name),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct DashedIdentReference<'a> {
    pub from: Option<NodeId<'a, Specifier<'a>>>,
    pub ident: AstStr<'a>,
}

unsafe impl<'ast> AstNodeStorage<'ast> for DashedIdentReference<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0004_0004);
    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    #[inline]
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    #[inline]
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        (self.ident == other.ident || context.str(self.ident) == context.str(other.ident))
            && match (self.from, other.from) {
                (None, None) => true,
                (Some(left), Some(right)) => context.nodes_eq(left, right),
                _ => false,
            }
    }
}

impl<'ast> AstNodeClone<'ast> for DashedIdentReference<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            ident: self.ident,
            from: self.from.map(|from| context.clone_encoded_node(from)),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct Function<'a> {
    pub arguments: Vec<'a, TokenOrValue<'a>>,
    #[visit(skip)]
    flags: FunctionFlags,
    #[visit(skip)]
    kind: KnownFunction,
    name: AstStr<'a>,
    /// A simple value serialized from this existing function node.
    pub replacement: Option<FunctionReplacement>,
}

// The native header fits one payload. The lossless name occupies one extra
// slot and the optional replacement occupies two opaque extra slots.
#[derive(Clone, Copy)]
#[repr(C)]
struct FunctionHeader<'a> {
    arguments: Vec<'a, TokenOrValue<'a>>,
    extra: u32,
    flags: FunctionFlags,
    kind: KnownFunction,
    has_replacement: bool,
}

pub use access::FunctionRef;

// Access views are storage infrastructure, not persistent visitor nodes.
mod access {
    use super::*;

    /// Read-only access to a stored function without reconstructing its overflow.
    pub struct FunctionRef<'context, 'storage, 'ast> {
        context: &'context AstContext<'storage>,
        header: FunctionHeader<'ast>,
    }

    impl<'ast> AstContext<'ast> {
        #[inline]
        pub fn function<'id>(&self, id: NodeId<'_, Function<'id>>) -> FunctionRef<'_, 'ast, 'id> {
            // SAFETY: the checked node kind stores this header. Its ranges are
            // branded with the ID's lifetime; it contains no references.
            let header = unsafe { self.node_payload(id).read_value() };
            FunctionRef {
                context: self,
                header,
            }
        }
    }

    impl<'ast> FunctionRef<'_, '_, 'ast> {
        #[inline]
        pub fn name(&self) -> AstStr<'ast> {
            // SAFETY: the first overflow slot is written as an AstStr.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }

        #[inline]
        pub fn arguments(&self) -> Vec<'ast, TokenOrValue<'ast>> {
            self.header.arguments
        }

        #[inline]
        pub fn kind(&self) -> KnownFunction {
            self.header.kind
        }

        #[inline]
        pub fn is_identifier(&self) -> bool {
            self.header.flags.contains(FunctionFlags::IS_IDENTIFIER)
        }

        #[inline]
        pub fn is_unquoted_url(&self) -> bool {
            self.header.flags.contains(FunctionFlags::UNQUOTED_URL)
        }

        #[inline]
        pub fn replacement(&self) -> Option<FunctionReplacement> {
            if !self.header.has_replacement {
                return None;
            }
            let extra = self.header.extra as usize;
            // SAFETY: these two slots contain the native optional replacement.
            unsafe {
                ExtraData::read_value_array([
                    self.context.extra_slot(extra + 1),
                    self.context.extra_slot(extra + 2),
                ])
            }
        }
    }
}

unsafe impl<'ast> AstNodeStorage<'ast> for Function<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0004_0001);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        // SAFETY: this node kind is always written as FunctionHeader.
        let header: FunctionHeader<'ast> = unsafe { payload.read_value() };
        let extra = header.extra as usize;
        Self {
            arguments: header.arguments,
            flags: header.flags,
            kind: header.kind,
            name: unsafe { context.extra_slot(extra).read_value() },
            replacement: unsafe {
                ExtraData::read_value_array([
                    context.extra_slot(extra + 1),
                    context.extra_slot(extra + 2),
                ])
            },
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_function(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: FunctionHeader<'ast> = unsafe { current.read_value() };
        encode_function(self, Some(header.extra as usize), context)
    }

    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.arguments == other.arguments
            && self.flags == other.flags
            && self.kind == other.kind
            && self.replacement == other.replacement
            && (self.name == other.name || context.str(self.name) == context.str(other.name))
    }
}

impl<'ast> AstNodeClone<'ast> for Function<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            arguments: context.clone_encoded_vec(self.arguments),
            flags: self.flags,
            kind: self.kind,
            name: self.name,
            replacement: self.replacement,
        }
    }
}

fn encode_function<'ast>(
    function: Function<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let name = ExtraData::from_value(function.name);
    let [replacement_low, replacement_high] = ExtraData::from_value_array(function.replacement);
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, name);
            context.set_extra_slot(extra + 1, replacement_low);
            context.set_extra_slot(extra + 2, replacement_high);
            extra
        }
        None => context.alloc_extra_slots([name, replacement_low, replacement_high]),
    };
    NodePayload::from_value(FunctionHeader {
        arguments: function.arguments,
        extra: u32::try_from(extra).expect("Function overflow index exceeds u32"),
        flags: function.flags,
        kind: function.kind,
        has_replacement: function.replacement.is_some(),
    })
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
    pub fn new(
        name: &str,
        arguments: Vec<'a, TokenOrValue<'a>>,
        context: &mut AstContext<'a>,
    ) -> Self {
        let (kind, vendor_prefixed) = KnownFunction::classify(name);
        let mut flags = FunctionFlags::empty();
        flags.set(FunctionFlags::VENDOR_PREFIXED, vendor_prefixed);
        Self {
            arguments,
            flags,
            kind,
            name: context.add_str(name),
            replacement: None,
        }
    }

    /// Returns the original function name.
    #[inline]
    pub const fn name(&self) -> AstStr<'a> {
        self.name
    }

    /// Returns the shared identity for a recognized function name.
    #[inline]
    pub const fn kind(&self) -> KnownFunction {
        self.kind
    }

    /// Updates the lossless function name and its recognized identity together.
    #[inline]
    pub fn set_name(&mut self, name: &str, context: &mut AstContext<'a>) {
        let (kind, vendor_prefixed) = KnownFunction::classify(name);
        if context.str(self.name) != name {
            self.name = context.add_str(name);
        }
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
    pub layer: Option<Vec<'a, AstStr<'a>>>,
    pub media: Option<NodeId<'a, MediaList<'a>>>,
    pub supports: Option<NodeId<'a, SupportsCondition<'a>>>,
    pub url: AstStr<'a>,
}

#[cfg(test)]
mod storage_tests {
    use super::{MediaList, MediaQuery, MediaQuerySlot, MediaType, Qualifier};
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, DUMMY_SP, DashedIdentReference, EnvironmentVariable, EnvironmentVariableName,
        Function, FunctionReplacement, KnownFunction, Specifier, Token, TokenOrValue,
        UAEnvironmentVariable, Unit, Url, Variable,
    };

    #[test]
    fn media_query_native_slot_preserves_qualifiers_and_custom_text() {
        assert_eq!(std::mem::size_of::<MediaQuerySlot<'_>>(), 16);
        assert_eq!(std::mem::size_of::<MediaList<'_>>(), 8);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("FutureMedia");
        let second = context.add_str("FutureMedia");
        assert_ne!(first, second);
        let query = MediaQuery {
            condition: None,
            media_type: MediaType::Custom(first),
            qualifier: None,
        };
        let node = context.alloc_encoded_node(query, DUMMY_SP);
        let equal = context.alloc_encoded_node(
            MediaQuery {
                media_type: MediaType::Custom(second),
                ..query
            },
            DUMMY_SP,
        );
        assert!(context.nodes_eq(node, equal));
        let child = context.alloc_encoded_node(
            crate::MediaCondition::Unknown(context.encoded_vec_range(0, 0)),
            DUMMY_SP,
        );
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for condition in [None, Some(child)] {
            for qualifier in [None, Some(Qualifier::Only), Some(Qualifier::Not)] {
                for media_type in [
                    MediaType::All,
                    MediaType::Print,
                    MediaType::Screen,
                    MediaType::Custom(first),
                    MediaType::Custom(second),
                ] {
                    let value = MediaQuery {
                        condition,
                        qualifier,
                        media_type,
                    };
                    context.mutate_encoded_node(node, |stored, _| *stored = value);
                    assert_eq!(context.encoded_node(node), value);
                }
            }
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn function_storage_preserves_native_state_and_reuses_overflow() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let arguments = context.alloc_vec(allocator.vec());
        let mut function = Function::new("-webkit-calc", arguments, &mut context);
        function.set_identifier(true);
        function.set_valid_rgb(true);
        function.replacement = Some(FunctionReplacement::Dimension {
            unit: Unit::Length(crate::LengthUnit::Cqmax),
            value: 12.5,
        });
        let id = context.alloc_encoded_node(function, DUMMY_SP);

        let decoded = context.encoded_node(id);
        assert_eq!(decoded.arguments, arguments);
        assert_eq!(context.str(decoded.name()), "-webkit-calc");
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

        context.mutate_encoded_node(id, |function, context| {
            function.set_name("rgb", context);
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
        assert_eq!(context.str(decoded.name()), "rgb");
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

    #[test]
    fn function_replacements_preserve_all_variants_bits_and_flags() {
        use FunctionReplacement as R;
        fn snapshot(value: Option<R>) -> (u8, [u32; 5], Option<Unit>) {
            match value {
                None => (0, [0; 5], None),
                Some(R::Number(value)) => (1, [value.to_bits(), 0, 0, 0, 0], None),
                Some(R::Percentage(value)) => (2, [value.to_bits(), 0, 0, 0, 0], None),
                Some(R::Dimension { unit, value }) => {
                    (3, [value.to_bits(), 0, 0, 0, 0], Some(unit))
                }
                Some(R::GrayAlpha { alpha, lightness }) => {
                    (4, [alpha.to_bits(), lightness.to_bits(), 0, 0, 0], None)
                }
                Some(R::Rgb { red, green, blue }) => {
                    (5, [red.into(), green.into(), blue.into(), 0, 0], None)
                }
                Some(R::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                    use_hex,
                }) => (
                    6,
                    [
                        red.into(),
                        green.into(),
                        blue.into(),
                        alpha.to_bits(),
                        use_hex.into(),
                    ],
                    None,
                ),
            }
        }
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let arguments = context.alloc_encoded_vec(std::iter::empty());
        let function = Function::new("FuN", arguments, &mut context);
        let name = function.name();
        let id = context.alloc_node(function, DUMMY_SP);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            let f = f32::from_bits(bits);
            for flags in 0..8 {
                for replacement in [
                    None,
                    Some(R::Number(f)),
                    Some(R::Percentage(f)),
                    Some(R::Dimension {
                        unit: Unit::Length(crate::LengthUnit::Cqmax),
                        value: f,
                    }),
                    Some(R::GrayAlpha {
                        alpha: f,
                        lightness: 0.25,
                    }),
                    Some(R::GrayAlpha {
                        alpha: 0.75,
                        lightness: f,
                    }),
                    Some(R::Rgb {
                        red: 0,
                        green: 127,
                        blue: 255,
                    }),
                    Some(R::Rgba {
                        red: 255,
                        green: 127,
                        blue: 0,
                        alpha: f,
                        use_hex: false,
                    }),
                    Some(R::Rgba {
                        red: 1,
                        green: 2,
                        blue: 3,
                        alpha: f,
                        use_hex: true,
                    }),
                    None,
                ]
                .into_iter()
                .chain(
                    [
                        Unit::Deg,
                        Unit::Rad,
                        Unit::Grad,
                        Unit::Turn,
                        Unit::Seconds,
                        Unit::Milliseconds,
                        Unit::Hertz,
                        Unit::Kilohertz,
                        Unit::Dpi,
                        Unit::Dpcm,
                        Unit::Dppx,
                        Unit::ResolutionX,
                        Unit::Flex,
                    ]
                    .map(|unit| Some(R::Dimension { unit, value: f })),
                ) {
                    context.mutate_node(id, |value, _| {
                        value.set_identifier(flags & 1 != 0);
                        value.set_valid_rgb(flags & 2 != 0);
                        value.set_unquoted_url(flags & 4 != 0);
                        value.replacement = replacement;
                    });
                    let value = context.resolve_node(id);
                    assert_eq!(snapshot(value.replacement), snapshot(replacement));
                    assert_eq!(value.is_identifier(), flags & 1 != 0);
                    assert_eq!(value.is_valid_rgb(), flags & 2 != 0);
                    assert_eq!(value.is_unquoted_url(), flags & 4 != 0);
                    assert_eq!(value.name(), name);
                    assert_eq!(value.arguments, arguments);
                    let view = context.function(id);
                    assert_eq!(snapshot(view.replacement()), snapshot(replacement));
                    assert_eq!(view.is_identifier(), flags & 1 != 0);
                    assert_eq!(view.is_unquoted_url(), flags & 4 != 0);
                    assert_eq!(view.kind(), value.kind());
                    assert_eq!(view.name(), name);
                    assert_eq!(view.arguments(), arguments);
                    assert_eq!(context.node_checkpoint(), checkpoint);
                    assert_eq!(context.string_pool().extra_len(), bytes);
                }
            }
        }
    }

    #[test]
    fn function_names_are_non_interned_and_unchanged_writes_do_not_allocate() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let arguments = context.alloc_vec(allocator.vec());
        let first = Function::new("FuN", arguments, &mut context);
        let second = Function::new("FuN", arguments, &mut context);
        assert_ne!(first.name(), second.name());
        assert_eq!(context.string_pool().len(), 0);
        let first = context.alloc_encoded_node(first, DUMMY_SP);
        let second = context.alloc_encoded_node(second, DUMMY_SP);
        assert!(context.nodes_eq(first, second));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..100 {
            assert_eq!(context.str(context.encoded_node(first).name()), "FuN");
            context.mutate_encoded_node(first, |function, context| {
                function.set_name("FuN", context);
            });
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn url_range_reads_and_unchanged_writes_do_not_allocate() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("asset.svg#icon");
        let url = context.alloc_encoded_node(Url { url: text }, DUMMY_SP);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..100 {
            assert_eq!(context.str(context.encoded_node(url).url), "asset.svg#icon");
            context.mutate_encoded_node(url, |_, _| {});
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        assert_eq!(std::mem::size_of::<Url<'_>>(), 8);
    }

    #[test]
    fn url_and_variable_codecs_keep_strings_ranges_and_owned_node_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("asset.svg#icon");
        let url = context.alloc_encoded_node(Url { url: text }, DUMMY_SP);
        let cloned_url = context.clone_encoded_node(url);
        assert_ne!(url, cloned_url);
        assert_eq!(context.encoded_node(cloned_url), Url { url: text });

        let file = context.add_str("theme.css");
        let file = context.alloc_encoded_node(Specifier::File(file), DUMMY_SP);
        let ident = context.add_str("--accent");
        let name = context.alloc_encoded_node(
            DashedIdentReference {
                from: Some(file),
                ident,
            },
            DUMMY_SP,
        );
        let fallback_ident = context.add_str("--fallback");
        let fallback_ident = context.alloc_encoded_node(
            crate::DashedIdent {
                value: fallback_ident,
            },
            DUMMY_SP,
        );
        let cloned_name = context.clone_encoded_node(name);
        let cloned_file = context.encoded_node(cloned_name).from.unwrap();
        assert_ne!(cloned_file, file);
        assert!(context.nodes_eq(cloned_name, name));
        assert!(context.nodes_eq(cloned_file, file));
        let comma = context.alloc_encoded_node(Token::Comma, DUMMY_SP);
        let fallback = context.alloc_encoded_vec(
            [
                TokenOrValue::Token(comma),
                TokenOrValue::DashedIdent(fallback_ident),
            ]
            .into_iter(),
        );
        let variable = context.alloc_encoded_node(
            Variable {
                fallback: Some(fallback),
                name,
            },
            DUMMY_SP,
        );
        let decoded = context.encoded_node(variable);
        assert_eq!(decoded.fallback, Some(fallback));
        assert_eq!(
            context.encoded_node(decoded.name),
            DashedIdentReference {
                from: Some(file),
                ident,
            }
        );
    }

    #[test]
    fn variable_and_environment_storage_distinguish_absent_and_empty_fallback() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let ident = context.add_str("--name");
        let name = context.alloc_encoded_node(DashedIdentReference { ident, from: None }, DUMMY_SP);
        let empty = context.alloc_encoded_vec(std::iter::empty::<TokenOrValue<'_>>());
        let variable = context.alloc_encoded_node(
            Variable {
                name,
                fallback: None,
            },
            DUMMY_SP,
        );
        let indices = context.alloc_encoded_vec(std::iter::empty::<i32>());
        let env = context.alloc_encoded_node(
            EnvironmentVariable {
                name: EnvironmentVariableName::Unknown(ident),
                indices,
                fallback: None,
            },
            DUMMY_SP,
        );
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for fallback in [Some(empty), None, Some(empty)] {
            context.mutate_encoded_node(variable, |node, _| node.fallback = fallback);
            context.mutate_encoded_node(env, |node, _| node.fallback = fallback);
            assert_eq!(context.encoded_node(variable).fallback, fallback);
            assert_eq!(context.encoded_node(env).fallback, fallback);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn environment_variable_codec_round_trips_fixed_overflow_fields() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let ident = context.add_str("--fallback");
        let ident = context.alloc_encoded_node(crate::DashedIdent { value: ident }, DUMMY_SP);
        let fallback = context.alloc_encoded_vec([TokenOrValue::DashedIdent(ident)].into_iter());
        let indices = context.alloc_encoded_vec([2_i32, 4].into_iter());
        let before = context.encoded_extra_len();
        let id = context.alloc_encoded_node(
            EnvironmentVariable {
                fallback: Some(fallback),
                indices,
                name: EnvironmentVariableName::UA(UAEnvironmentVariable::ViewportSegmentWidth),
            },
            DUMMY_SP,
        );

        assert_eq!(context.encoded_extra_len(), before + 2);
        let decoded = context.encoded_node(id);
        assert_eq!(decoded.fallback, Some(fallback));
        assert_eq!(decoded.indices, indices);
        assert_eq!(
            decoded.name,
            EnvironmentVariableName::UA(UAEnvironmentVariable::ViewportSegmentWidth)
        );

        let unknown = context.add_str("viewport-custom");
        context.mutate_encoded_node(id, |value, _| {
            value.fallback = None;
            value.name = EnvironmentVariableName::Unknown(unknown);
        });
        let decoded = context.encoded_node(id);
        assert_eq!(decoded.fallback, None);
        assert_eq!(decoded.name, EnvironmentVariableName::Unknown(unknown));
        assert_eq!(context.encoded_extra_len(), before + 2);
        let empty_fallback = context.alloc_encoded_vec(std::iter::empty());
        let boundary_indices =
            context.alloc_encoded_vec([i32::MIN, 0, i32::MAX, i32::MIN].into_iter());
        let empty_indices = context.alloc_encoded_vec(std::iter::empty());
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for (indices, expected) in [
            (indices, &[2, 4][..]),
            (empty_indices, &[][..]),
            (boundary_indices, &[i32::MIN, 0, i32::MAX, i32::MIN][..]),
        ] {
            for fallback in [None, Some(empty_fallback), Some(fallback), None] {
                context.mutate_node(id, |value, _| {
                    value.indices = indices;
                    value.fallback = fallback;
                });
                let actual = context.resolve_node(id);
                assert_eq!(actual.indices, indices);
                assert_eq!(actual.fallback, fallback);
                assert_eq!(actual.name, EnvironmentVariableName::Unknown(unknown));
                let view = context.environment_variable(id);
                assert_eq!(view.indices(), indices);
                assert_eq!(view.fallback(), fallback);
                assert_eq!(view.name(), EnvironmentVariableName::Unknown(unknown));
                assert!(
                    context
                        .vec_iter(view.indices())
                        .eq(expected.iter().copied())
                );
                assert_eq!(context.node_checkpoint(), checkpoint);
                assert_eq!(context.string_pool().extra_len(), bytes);
            }
        }
    }
}

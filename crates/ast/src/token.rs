use super::*;

use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Visit)]
pub enum TokenOrValue<'a> {
    Token(NodeId<'a, Token<'a>>),
    Color(NodeId<'a, CssColor<'a>>),
    UnresolvedColor(NodeId<'a, UnresolvedColor<'a>>),
    Url(NodeId<'a, Url<'a>>),
    Var(NodeId<'a, Variable<'a>>),
    Env(NodeId<'a, EnvironmentVariable<'a>>),
    Function(NodeId<'a, Function<'a>>),
    Length(LengthValue),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    DashedIdent(NodeId<'a, DashedIdent<'a>>),
    AnimationName(NodeId<'a, AnimationName<'a>>),
}

// Scalar fields are flattened beside the enum tag so the actual list value
// fits one eight-byte slot. This performs real layout compression; node IDs,
// units and floats retain their native representations. Conversion allocates
// nothing: even the oversized string alternative is prepared before publication.
#[repr(u8)]
#[derive(Clone, Copy)]
enum TokenOrValueSlot<'a> {
    Token(NodeId<'a, Token<'a>>),
    Color(NodeId<'a, CssColor<'a>>),
    UnresolvedColor(NodeId<'a, UnresolvedColor<'a>>),
    Url(NodeId<'a, Url<'a>>),
    Var(NodeId<'a, Variable<'a>>),
    Env(NodeId<'a, EnvironmentVariable<'a>>),
    Function(NodeId<'a, Function<'a>>),
    DashedIdent(NodeId<'a, DashedIdent<'a>>),
    AnimationName(NodeId<'a, AnimationName<'a>>),
    Length { unit: LengthUnit, value: f32 },
    AngleDeg(f32),
    AngleRad(f32),
    AngleGrad(f32),
    AngleTurn(f32),
    TimeSeconds(f32),
    TimeMilliseconds(f32),
    ResolutionDpi(f32),
    ResolutionDpcm(f32),
    ResolutionDppx(f32),
}

unsafe impl<'ast> ExtraDataCompact<'ast> for TokenOrValue<'ast> {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        let value = match self {
            Self::Token(id) => TokenOrValueSlot::Token(id),
            Self::Color(id) => TokenOrValueSlot::Color(id),
            Self::UnresolvedColor(id) => TokenOrValueSlot::UnresolvedColor(id),
            Self::Url(id) => TokenOrValueSlot::Url(id),
            Self::Var(id) => TokenOrValueSlot::Var(id),
            Self::Env(id) => TokenOrValueSlot::Env(id),
            Self::Function(id) => TokenOrValueSlot::Function(id),
            Self::DashedIdent(id) => TokenOrValueSlot::DashedIdent(id),
            Self::AnimationName(id) => TokenOrValueSlot::AnimationName(id),
            Self::Length(LengthValue { unit, value }) => TokenOrValueSlot::Length { unit, value },
            Self::Angle(Angle::Deg(value)) => TokenOrValueSlot::AngleDeg(value),
            Self::Angle(Angle::Rad(value)) => TokenOrValueSlot::AngleRad(value),
            Self::Angle(Angle::Grad(value)) => TokenOrValueSlot::AngleGrad(value),
            Self::Angle(Angle::Turn(value)) => TokenOrValueSlot::AngleTurn(value),
            Self::Time(Time::Seconds(value)) => TokenOrValueSlot::TimeSeconds(value),
            Self::Time(Time::Milliseconds(value)) => TokenOrValueSlot::TimeMilliseconds(value),
            Self::Resolution(Resolution::Dpi(value)) => TokenOrValueSlot::ResolutionDpi(value),
            Self::Resolution(Resolution::Dpcm(value)) => TokenOrValueSlot::ResolutionDpcm(value),
            Self::Resolution(Resolution::Dppx(value)) => TokenOrValueSlot::ResolutionDppx(value),
        };
        ExtraData::from_value(value)
    }

    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        // SAFETY: every element is published as the same native compact enum.
        match unsafe { data.read_value::<TokenOrValueSlot<'ast>>() } {
            TokenOrValueSlot::Token(id) => Self::Token(id),
            TokenOrValueSlot::Color(id) => Self::Color(id),
            TokenOrValueSlot::UnresolvedColor(id) => Self::UnresolvedColor(id),
            TokenOrValueSlot::Url(id) => Self::Url(id),
            TokenOrValueSlot::Var(id) => Self::Var(id),
            TokenOrValueSlot::Env(id) => Self::Env(id),
            TokenOrValueSlot::Function(id) => Self::Function(id),
            TokenOrValueSlot::DashedIdent(id) => Self::DashedIdent(id),
            TokenOrValueSlot::AnimationName(id) => Self::AnimationName(id),
            TokenOrValueSlot::Length { unit, value } => Self::Length(LengthValue { unit, value }),
            TokenOrValueSlot::AngleDeg(value) => Self::Angle(Angle::Deg(value)),
            TokenOrValueSlot::AngleRad(value) => Self::Angle(Angle::Rad(value)),
            TokenOrValueSlot::AngleGrad(value) => Self::Angle(Angle::Grad(value)),
            TokenOrValueSlot::AngleTurn(value) => Self::Angle(Angle::Turn(value)),
            TokenOrValueSlot::TimeSeconds(value) => Self::Time(Time::Seconds(value)),
            TokenOrValueSlot::TimeMilliseconds(value) => Self::Time(Time::Milliseconds(value)),
            TokenOrValueSlot::ResolutionDpi(value) => Self::Resolution(Resolution::Dpi(value)),
            TokenOrValueSlot::ResolutionDpcm(value) => Self::Resolution(Resolution::Dpcm(value)),
            TokenOrValueSlot::ResolutionDppx(value) => Self::Resolution(Resolution::Dppx(value)),
        }
    }
}

/// An ordinary, non-interned dashed identifier stored before list publication.
#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub struct DashedIdent<'a> {
    pub value: AstStr<'a>,
}

unsafe impl<'ast> AstNodeStorage<'ast> for DashedIdent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0004);

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
        self.value == other.value || context.str(self.value) == context.str(other.value)
    }
}

impl<'ast> AstNodeClone<'ast> for DashedIdent<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

impl<'ast> ExtraDataClone<'ast> for TokenOrValue<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Token(value) => Self::Token(context.clone_encoded_node(value)),
            Self::Color(value) => Self::Color(context.clone_encoded_node(value)),
            Self::UnresolvedColor(value) => {
                Self::UnresolvedColor(context.clone_encoded_node(value))
            }
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Var(value) => Self::Var(context.clone_encoded_node(value)),
            Self::Env(value) => Self::Env(context.clone_encoded_node(value)),
            Self::Function(value) => Self::Function(context.clone_encoded_node(value)),
            Self::Length(value) => Self::Length(value),
            Self::Angle(value) => Self::Angle(value),
            Self::Time(value) => Self::Time(value),
            Self::Resolution(value) => Self::Resolution(value),
            Self::DashedIdent(value) => Self::DashedIdent(context.clone_encoded_node(value)),
            Self::AnimationName(value) => Self::AnimationName(context.clone_encoded_node(value)),
        }
    }
}

impl Eq for TokenOrValue<'_> {}

impl Hash for TokenOrValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        if let Self::Token(token) = self {
            token.hash(state);
        }
        // Values in selectors are rare, and fully hashing them would pull
        // floating-point hashing through much of the AST. Equal values still
        // share this hash; collisions are resolved by structural equality.
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub enum Unit {
    Length(LengthUnit),
    Deg,
    Rad,
    Grad,
    Turn,
    Seconds,
    Milliseconds,
    Hertz,
    Kilohertz,
    Dpi,
    Dpcm,
    Dppx,
    ResolutionX,
    Flex,
}

impl Unit {
    pub const fn length(self) -> Option<LengthUnit> {
        match self {
            Self::Length(unit) => Some(unit),
            _ => None,
        }
    }

    pub const fn is_length(self) -> bool {
        matches!(self, Self::Length(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Token<'a> {
    Ident(AstStr<'a>),
    AtKeyword(AstStr<'a>),
    Hash(AstStr<'a>),
    IdHash(AstStr<'a>),
    /// A hexadecimal color hash normalized during minification.
    MinifiedHash(AstStr<'a>),
    String(AstStr<'a>),
    /// A quoted font family that can be serialized as identifiers in place.
    UnquotedFont(AstStr<'a>),
    UnquotedUrl(AstStr<'a>),
    Delim(AstStr<'a>),
    Number(f32),
    Percentage(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    UnknownDimension {
        unit: AstStr<'a>,
        value: f32,
    },
    WhiteSpace(AstStr<'a>),
    Comment(AstStr<'a>),
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
    Function(AstStr<'a>),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(AstStr<'a>),
    BadString(AstStr<'a>),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

// SAFETY: KIND identifies native Token storage, published and read as the same Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for Token<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0001);

    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Ident(left), Self::Ident(right))
            | (Self::AtKeyword(left), Self::AtKeyword(right))
            | (Self::Hash(left), Self::Hash(right))
            | (Self::IdHash(left), Self::IdHash(right))
            | (Self::MinifiedHash(left), Self::MinifiedHash(right))
            | (Self::String(left), Self::String(right))
            | (Self::UnquotedFont(left), Self::UnquotedFont(right))
            | (Self::UnquotedUrl(left), Self::UnquotedUrl(right))
            | (Self::Delim(left), Self::Delim(right))
            | (Self::WhiteSpace(left), Self::WhiteSpace(right))
            | (Self::Comment(left), Self::Comment(right))
            | (Self::Function(left), Self::Function(right))
            | (Self::BadUrl(left), Self::BadUrl(right))
            | (Self::BadString(left), Self::BadString(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            (
                Self::UnknownDimension {
                    unit: left,
                    value: left_value,
                },
                Self::UnknownDimension {
                    unit: right,
                    value: right_value,
                },
            ) => {
                left_value == right_value
                    && (left == right || context.str(*left) == context.str(*right))
            }
            _ => self == other,
        }
    }

    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        // SAFETY: the typed node context checked this token's KIND.
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

impl<'ast> AstNodeClone<'ast> for Token<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

impl Eq for Token<'_> {}

impl Hash for Token<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Ident(value)
            | Self::AtKeyword(value)
            | Self::Hash(value)
            | Self::IdHash(value)
            | Self::MinifiedHash(value)
            | Self::String(value)
            | Self::UnquotedFont(value)
            | Self::UnquotedUrl(value)
            | Self::Delim(value)
            | Self::WhiteSpace(value)
            | Self::Comment(value)
            | Self::Function(value)
            | Self::BadUrl(value)
            | Self::BadString(value) => value.hash(state),
            Self::Number(value) | Self::Percentage(value) => hash_float(*value, state),
            Self::Dimension { unit, value } => {
                unit.hash(state);
                hash_float(*value, state);
            }
            Self::UnknownDimension { unit, value } => {
                unit.hash(state);
                hash_float(*value, state);
            }
            Self::Colon
            | Self::Semicolon
            | Self::Comma
            | Self::IncludeMatch
            | Self::DashMatch
            | Self::PrefixMatch
            | Self::SuffixMatch
            | Self::SubstringMatch
            | Self::Cdo
            | Self::Cdc
            | Self::ParenthesisBlock
            | Self::SquareBracketBlock
            | Self::CurlyBracketBlock
            | Self::CloseParenthesis
            | Self::CloseSquareBracket
            | Self::CloseCurlyBracket => {}
        }
    }
}

#[inline]
fn hash_float<H: Hasher>(value: f32, state: &mut H) {
    // PartialEq considers both signed zero representations equal.
    if value == 0.0 {
        0_u32.hash(state);
    } else {
        value.to_bits().hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Specifier<'a> {
    Global,
    File(AstStr<'a>),
    SourceIndex(u32),
}

unsafe impl<'ast> AstNodeStorage<'ast> for Specifier<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0003);
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
        match (self, other) {
            (Self::File(left), Self::File(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            _ => self == other,
        }
    }
}

impl<'ast> AstNodeClone<'ast> for Specifier<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum AnimationName<'a> {
    None,
    Ident(AstStr<'a>),
    String(AstStr<'a>),
}

unsafe impl<'ast> AstNodeStorage<'ast> for AnimationName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0002);
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
        match (self, other) {
            (Self::Ident(left), Self::Ident(right)) | (Self::String(left), Self::String(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            _ => self == other,
        }
    }
}

impl<'ast> AstNodeClone<'ast> for AnimationName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum EnvironmentVariableName<'a> {
    UA(UAEnvironmentVariable),
    Custom(NodeId<'a, DashedIdentReference<'a>>),
    Unknown(AstStr<'a>),
}

#[derive(Clone, Copy, CssKeyword, Debug, PartialEq, Visit)]
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

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        Angle, AnimationName, AstContext, DUMMY_SP, LengthUnit, LengthValue, Token, TokenOrValue,
        Unit,
    };

    #[test]
    fn token_node_equality_compares_text_at_distinct_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("same");
        let second = context.add_str("same");
        assert_ne!(first, second);
        let left = context.alloc_node(Token::Ident(first), DUMMY_SP);
        let right = context.alloc_node(Token::Ident(second), DUMMY_SP);
        assert!(context.nodes_eq(left, right));
        let different_kind = context.alloc_node(Token::String(second), DUMMY_SP);
        assert!(!context.nodes_eq(left, different_kind));
    }

    #[test]
    fn token_codec_round_trips_string_and_dimension_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let unit = context.add_str("furlong");
        let id = context.alloc_encoded_node(Token::UnknownDimension { unit, value: 2.5 }, DUMMY_SP);
        assert_eq!(
            context.encoded_node(id),
            Token::UnknownDimension { unit, value: 2.5 }
        );

        context.mutate_encoded_node(id, |token, _| {
            *token = Token::Dimension {
                unit: Unit::Length(LengthUnit::Rlh),
                value: 4.0,
            };
        });
        assert_eq!(
            context.encoded_node(id),
            Token::Dimension {
                unit: Unit::Length(LengthUnit::Rlh),
                value: 4.0,
            }
        );

        let cloned = context.clone_encoded_node(id);
        assert_ne!(id, cloned);
        assert_eq!(context.encoded_node_span(cloned), DUMMY_SP);
        assert_eq!(
            context.encoded_node(cloned),
            Token::Dimension {
                unit: Unit::Length(LengthUnit::Rlh),
                value: 4.0,
            }
        );
    }

    #[test]
    fn compact_token_values_preserve_native_scalars_without_allocation() {
        use crate::{ExtraDataCompact, Resolution, Time};
        let allocator = Allocator::new();
        let context = AstContext::new_in(&allocator);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0042,
        ] {
            let value = f32::from_bits(bits);
            let cases = [
                TokenOrValue::Length(LengthValue {
                    unit: LengthUnit::Rlh,
                    value,
                }),
                TokenOrValue::Angle(Angle::Deg(value)),
                TokenOrValue::Angle(Angle::Rad(value)),
                TokenOrValue::Angle(Angle::Grad(value)),
                TokenOrValue::Angle(Angle::Turn(value)),
                TokenOrValue::Time(Time::Seconds(value)),
                TokenOrValue::Time(Time::Milliseconds(value)),
                TokenOrValue::Resolution(Resolution::Dpi(value)),
                TokenOrValue::Resolution(Resolution::Dpcm(value)),
                TokenOrValue::Resolution(Resolution::Dppx(value)),
            ];
            for unit in [
                LengthUnit::Px,
                LengthUnit::In,
                LengthUnit::Cm,
                LengthUnit::Mm,
                LengthUnit::Q,
                LengthUnit::Pt,
                LengthUnit::Pc,
                LengthUnit::Em,
                LengthUnit::Rem,
                LengthUnit::Ex,
                LengthUnit::Rex,
                LengthUnit::Ch,
                LengthUnit::Rch,
                LengthUnit::Cap,
                LengthUnit::Rcap,
                LengthUnit::Ic,
                LengthUnit::Ric,
                LengthUnit::Lh,
                LengthUnit::Rlh,
                LengthUnit::Vw,
                LengthUnit::Lvw,
                LengthUnit::Svw,
                LengthUnit::Dvw,
                LengthUnit::Cqw,
                LengthUnit::Vh,
                LengthUnit::Lvh,
                LengthUnit::Svh,
                LengthUnit::Dvh,
                LengthUnit::Cqh,
                LengthUnit::Vi,
                LengthUnit::Svi,
                LengthUnit::Lvi,
                LengthUnit::Dvi,
                LengthUnit::Cqi,
                LengthUnit::Vb,
                LengthUnit::Svb,
                LengthUnit::Lvb,
                LengthUnit::Dvb,
                LengthUnit::Cqb,
                LengthUnit::Vmin,
                LengthUnit::Svmin,
                LengthUnit::Lvmin,
                LengthUnit::Dvmin,
                LengthUnit::Cqmin,
                LengthUnit::Vmax,
                LengthUnit::Svmax,
                LengthUnit::Lvmax,
                LengthUnit::Dvmax,
                LengthUnit::Cqmax,
            ] {
                let slot = TokenOrValue::Length(LengthValue { unit, value }).encode_extra();
                // SAFETY: the slot was just initialized as TokenOrValue.
                let actual = unsafe { TokenOrValue::decode_extra(slot) };
                let TokenOrValue::Length(actual) = actual else {
                    panic!("length tag changed")
                };
                assert_eq!(actual.unit, unit);
                assert_eq!(actual.value.to_bits(), bits);
            }
            for expected in cases {
                let description = format!("{expected:?}");
                let slot = expected.encode_extra();
                // SAFETY: this slot was written as TokenOrValue immediately above.
                let actual = unsafe { TokenOrValue::decode_extra(slot) };
                assert_eq!(format!("{actual:?}"), description);
                let actual = match actual {
                    TokenOrValue::Length(LengthValue {
                        unit: LengthUnit::Rlh,
                        value,
                    })
                    | TokenOrValue::Angle(
                        Angle::Deg(value)
                        | Angle::Rad(value)
                        | Angle::Grad(value)
                        | Angle::Turn(value),
                    )
                    | TokenOrValue::Time(Time::Seconds(value) | Time::Milliseconds(value))
                    | TokenOrValue::Resolution(
                        Resolution::Dpi(value) | Resolution::Dpcm(value) | Resolution::Dppx(value),
                    ) => value,
                    _ => panic!("unexpected scalar variant"),
                };
                assert_eq!(actual.to_bits(), bits);
            }
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        assert_eq!(std::mem::size_of::<super::TokenOrValueSlot<'_>>(), 8);
        assert_eq!(std::mem::size_of::<crate::DashedIdent<'_>>(), 8);
    }

    #[test]
    fn compact_token_handles_publish_and_replace_without_allocating_children() {
        use crate::*;
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let text = ast.add_str("--name");
        let token = ast.alloc_node(Token::Ident(text), DUMMY_SP);
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let unresolved = ast.alloc_node(
            UnresolvedColor::LightDark {
                light: AstVec::empty(),
                dark: AstVec::empty(),
            },
            DUMMY_SP,
        );
        let url = ast.alloc_node(Url { url: text }, DUMMY_SP);
        let name = ast.alloc_node(
            DashedIdentReference {
                ident: text,
                from: None,
            },
            DUMMY_SP,
        );
        let variable = ast.alloc_node(
            Variable {
                name,
                fallback: None,
            },
            DUMMY_SP,
        );
        let env = ast.alloc_node(
            EnvironmentVariable {
                name: EnvironmentVariableName::UA(UAEnvironmentVariable::SafeAreaInsetTop),
                indices: AstVec::empty(),
                fallback: None,
            },
            DUMMY_SP,
        );
        let function = Function::new("fn", AstVec::empty(), &mut ast);
        let function = ast.alloc_node(function, DUMMY_SP);
        let dashed = ast.alloc_node(DashedIdent { value: text }, DUMMY_SP);
        let animation = ast.alloc_node(AnimationName::Ident(text), DUMMY_SP);
        let make = || {
            [
                TokenOrValue::Token(token),
                TokenOrValue::Color(color),
                TokenOrValue::UnresolvedColor(unresolved),
                TokenOrValue::Url(url),
                TokenOrValue::Var(variable),
                TokenOrValue::Env(env),
                TokenOrValue::Function(function),
                TokenOrValue::DashedIdent(dashed),
                TokenOrValue::AnimationName(animation),
            ]
        };
        let nodes = ast.encoded_node_len();
        let extra = ast.encoded_extra_len();
        let bytes = ast.string_pool().extra_len();
        let interned = ast.string_pool().len();
        let range = ast.alloc_encoded_vec(make().into_iter());
        assert_eq!(ast.encoded_node_len(), nodes);
        assert_eq!(ast.encoded_extra_len(), extra + make().len());
        let checkpoint = ast.node_checkpoint();
        for (index, expected) in make().into_iter().enumerate() {
            assert_eq!(ast.vec_get(range, index).unwrap(), expected);
        }
        for (value, expected) in make().into_iter().zip(make()).rev() {
            ast.vec_set(range, 0, value);
            assert_eq!(ast.vec_get(range, 0).unwrap(), expected);
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
        assert_eq!(ast.string_pool().extra_len(), bytes);
        assert_eq!(ast.string_pool().len(), interned);
    }

    #[test]
    fn token_or_value_occupies_one_shared_extra_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.add_str("name");
        let token = context.alloc_encoded_node(Token::Ident(name), DUMMY_SP);
        let custom = context.add_str("--custom");
        let custom = context.alloc_encoded_node(crate::DashedIdent { value: custom }, DUMMY_SP);
        let before = (context.encoded_node_len(), context.encoded_extra_len());
        let values = context.alloc_encoded_vec(
            [
                TokenOrValue::Token(token),
                TokenOrValue::Length(LengthValue {
                    unit: LengthUnit::Px,
                    value: 3.0,
                }),
                TokenOrValue::Angle(Angle::Turn(0.5)),
                TokenOrValue::DashedIdent(custom),
            ]
            .into_iter(),
        );
        let after = context.node_checkpoint();
        assert_eq!(context.encoded_node_len(), before.0);
        assert_eq!(context.encoded_extra_len(), before.1 + 4);

        let TokenOrValue::Token(decoded_token) = context.encoded_vec_get(values, 0).unwrap() else {
            panic!("expected token identity");
        };
        assert_eq!(context.encoded_node(decoded_token), Token::Ident(name));
        assert_eq!(
            context.encoded_vec_get(values, 1),
            Some(TokenOrValue::Length(LengthValue {
                unit: LengthUnit::Px,
                value: 3.0,
            }))
        );
        assert_eq!(
            context.encoded_vec_get(values, 2),
            Some(TokenOrValue::Angle(Angle::Turn(0.5)))
        );
        assert_eq!(
            context.encoded_vec_get(values, 3),
            Some(TokenOrValue::DashedIdent(custom))
        );
        assert_eq!(values.len(), 4);
        assert_eq!(context.node_checkpoint(), after);
    }

    #[test]
    fn animation_name_ranges_are_stored_in_nodes_and_list_handles() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let text = context.add_str("slide-in");
        let id = context.alloc_encoded_node(AnimationName::String(text), DUMMY_SP);
        let cloned = context.clone_encoded_node(id);
        assert_ne!(id, cloned);
        assert_eq!(context.encoded_node(cloned), AnimationName::String(text));

        let text = context.add_str("fade");
        let name = context.alloc_encoded_node(AnimationName::Ident(text), DUMMY_SP);
        let names = context.alloc_encoded_vec([name].into_iter());
        assert_eq!(
            context.encoded_node(context.encoded_vec_get(names, 0).unwrap()),
            AnimationName::Ident(text)
        );
    }
}

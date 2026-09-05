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
    DashedIdent(&'a str),
    AnimationName(NodeId<'a, AnimationName<'a>>),
}

// One `TokenOrValue` consumes one shared ExtraData slot:
//
// byte 0      variant
// byte 1      nested scalar/unit variant when needed
// bytes 2..4  reserved
// bytes 4..8  NodeId, compact string ID, or f32 bits
impl<'ast> ExtraDataCompact<'ast> for TokenOrValue<'ast> {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        let data = match self {
            Self::Token(id) => {
                bytes[0] = 0;
                node_index(id)
            }
            Self::Color(id) => {
                bytes[0] = 1;
                node_index(id)
            }
            Self::UnresolvedColor(id) => {
                bytes[0] = 2;
                node_index(id)
            }
            Self::Url(id) => {
                bytes[0] = 3;
                node_index(id)
            }
            Self::Var(id) => {
                bytes[0] = 4;
                node_index(id)
            }
            Self::Env(id) => {
                bytes[0] = 5;
                node_index(id)
            }
            Self::Function(id) => {
                bytes[0] = 6;
                node_index(id)
            }
            Self::Length(value) => {
                bytes[0] = 7;
                bytes[1] = crate::length::encode_length_unit(value.unit);
                value.value.to_bits()
            }
            Self::Angle(value) => {
                bytes[0] = 8;
                let (kind, value) = encode_angle(value);
                bytes[1] = kind;
                value.to_bits()
            }
            Self::Time(value) => {
                bytes[0] = 9;
                let (kind, value) = encode_time(value);
                bytes[1] = kind;
                value.to_bits()
            }
            Self::Resolution(value) => {
                bytes[0] = 10;
                let (kind, value) = encode_resolution(value);
                bytes[1] = kind;
                value.to_bits()
            }
            Self::DashedIdent(value) => {
                bytes[0] = 11;
                context.store_string(value)
            }
            Self::AnimationName(id) => {
                bytes[0] = 12;
                node_index(id)
            }
        };
        bytes[4..].copy_from_slice(&data.to_le_bytes());
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        let value = u32::from_le_bytes(
            bytes[4..]
                .try_into()
                .expect("TokenOrValue data is four bytes"),
        );
        match bytes[0] {
            0 => Self::Token(context.encoded_node_id_at(value as usize)),
            1 => Self::Color(context.encoded_node_id_at(value as usize)),
            2 => Self::UnresolvedColor(context.encoded_node_id_at(value as usize)),
            3 => Self::Url(context.encoded_node_id_at(value as usize)),
            4 => Self::Var(context.encoded_node_id_at(value as usize)),
            5 => Self::Env(context.encoded_node_id_at(value as usize)),
            6 => Self::Function(context.encoded_node_id_at(value as usize)),
            7 => Self::Length(LengthValue {
                unit: crate::length::decode_length_unit(bytes[1]),
                value: f32::from_bits(value),
            }),
            8 => Self::Angle(decode_angle(bytes[1], f32::from_bits(value))),
            9 => Self::Time(decode_time(bytes[1], f32::from_bits(value))),
            10 => Self::Resolution(decode_resolution(bytes[1], f32::from_bits(value))),
            11 => Self::DashedIdent(context.resolve_string(value as u64)),
            12 => Self::AnimationName(context.encoded_node_id_at(value as usize)),
            _ => panic!("invalid encoded TokenOrValue variant"),
        }
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
            Self::DashedIdent(value) => Self::DashedIdent(value),
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

pub(crate) fn encode_unit(unit: Unit) -> u8 {
    match unit {
        Unit::Length(unit) => crate::length::encode_length_unit(unit),
        Unit::Deg => 49,
        Unit::Rad => 50,
        Unit::Grad => 51,
        Unit::Turn => 52,
        Unit::Seconds => 53,
        Unit::Milliseconds => 54,
        Unit::Hertz => 55,
        Unit::Kilohertz => 56,
        Unit::Dpi => 57,
        Unit::Dpcm => 58,
        Unit::Dppx => 59,
        Unit::ResolutionX => 60,
        Unit::Flex => 61,
    }
}

pub(crate) fn decode_unit(unit: u8) -> Unit {
    match unit {
        0..=48 => Unit::Length(crate::length::decode_length_unit(unit)),
        49 => Unit::Deg,
        50 => Unit::Rad,
        51 => Unit::Grad,
        52 => Unit::Turn,
        53 => Unit::Seconds,
        54 => Unit::Milliseconds,
        55 => Unit::Hertz,
        56 => Unit::Kilohertz,
        57 => Unit::Dpi,
        58 => Unit::Dpcm,
        59 => Unit::Dppx,
        60 => Unit::ResolutionX,
        61 => Unit::Flex,
        _ => panic!("invalid encoded Unit"),
    }
}

#[derive(Clone, Debug, PartialEq, Visit)]
pub enum Token<'a> {
    Ident(&'a str),
    AtKeyword(&'a str),
    Hash(&'a str),
    IdHash(&'a str),
    /// A hexadecimal color hash normalized during minification.
    MinifiedHash(&'a str),
    String(&'a str),
    /// A quoted font family that can be serialized as identifiers in place.
    UnquotedFont(&'a str),
    UnquotedUrl(&'a str),
    Delim(&'a str),
    Number(f32),
    Percentage(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    UnknownDimension {
        unit: &'a str,
        value: f32,
    },
    WhiteSpace(&'a str),
    Comment(&'a str),
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
    Function(&'a str),
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl(&'a str),
    BadString(&'a str),
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

// Fixed payload layout for `Token`:
//
// byte 0       variant
// byte 1       Unit for Dimension
// bytes 2..4   reserved
// bytes 4..8   compact string ID or f32 bits
// bytes 8..12  second compact string ID for UnknownDimension
// bytes 12..16 reserved
impl<'ast> AstNodeStorage<'ast> for Token<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let data = u32::from_le_bytes(
            bytes[4..8]
                .try_into()
                .expect("Token primary data is four bytes"),
        );
        let string = || context.resolve_string(data as u64);
        match bytes[0] {
            0 => Self::Ident(string()),
            1 => Self::AtKeyword(string()),
            2 => Self::Hash(string()),
            3 => Self::IdHash(string()),
            4 => Self::MinifiedHash(string()),
            5 => Self::String(string()),
            6 => Self::UnquotedFont(string()),
            7 => Self::UnquotedUrl(string()),
            8 => Self::Delim(string()),
            9 => Self::Number(f32::from_bits(data)),
            10 => Self::Percentage(f32::from_bits(data)),
            11 => Self::Dimension {
                unit: decode_unit(bytes[1]),
                value: f32::from_bits(data),
            },
            12 => Self::UnknownDimension {
                unit: context.resolve_string(u32::from_le_bytes(
                    bytes[8..12]
                        .try_into()
                        .expect("Token secondary string ID is four bytes"),
                ) as u64),
                value: f32::from_bits(data),
            },
            13 => Self::WhiteSpace(string()),
            14 => Self::Comment(string()),
            15 => Self::Colon,
            16 => Self::Semicolon,
            17 => Self::Comma,
            18 => Self::IncludeMatch,
            19 => Self::DashMatch,
            20 => Self::PrefixMatch,
            21 => Self::SuffixMatch,
            22 => Self::SubstringMatch,
            23 => Self::Cdo,
            24 => Self::Cdc,
            25 => Self::Function(string()),
            26 => Self::ParenthesisBlock,
            27 => Self::SquareBracketBlock,
            28 => Self::CurlyBracketBlock,
            29 => Self::BadUrl(string()),
            30 => Self::BadString(string()),
            31 => Self::CloseParenthesis,
            32 => Self::CloseSquareBracket,
            33 => Self::CloseCurlyBracket,
            _ => panic!("invalid encoded Token variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_token(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_token(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for Token<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_token<'ast>(token: Token<'ast>, context: &mut AstContext<'ast>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    let data = match token {
        Token::Ident(value) => encode_token_string(0, value, &mut bytes, context),
        Token::AtKeyword(value) => encode_token_string(1, value, &mut bytes, context),
        Token::Hash(value) => encode_token_string(2, value, &mut bytes, context),
        Token::IdHash(value) => encode_token_string(3, value, &mut bytes, context),
        Token::MinifiedHash(value) => encode_token_string(4, value, &mut bytes, context),
        Token::String(value) => encode_token_string(5, value, &mut bytes, context),
        Token::UnquotedFont(value) => encode_token_string(6, value, &mut bytes, context),
        Token::UnquotedUrl(value) => encode_token_string(7, value, &mut bytes, context),
        Token::Delim(value) => encode_token_string(8, value, &mut bytes, context),
        Token::Number(value) => {
            bytes[0] = 9;
            value.to_bits()
        }
        Token::Percentage(value) => {
            bytes[0] = 10;
            value.to_bits()
        }
        Token::Dimension { unit, value } => {
            bytes[0] = 11;
            bytes[1] = encode_unit(unit);
            value.to_bits()
        }
        Token::UnknownDimension { unit, value } => {
            bytes[0] = 12;
            bytes[8..12].copy_from_slice(&context.store_string(unit).to_le_bytes());
            value.to_bits()
        }
        Token::WhiteSpace(value) => encode_token_string(13, value, &mut bytes, context),
        Token::Comment(value) => encode_token_string(14, value, &mut bytes, context),
        Token::Colon => encode_empty_token(15, &mut bytes),
        Token::Semicolon => encode_empty_token(16, &mut bytes),
        Token::Comma => encode_empty_token(17, &mut bytes),
        Token::IncludeMatch => encode_empty_token(18, &mut bytes),
        Token::DashMatch => encode_empty_token(19, &mut bytes),
        Token::PrefixMatch => encode_empty_token(20, &mut bytes),
        Token::SuffixMatch => encode_empty_token(21, &mut bytes),
        Token::SubstringMatch => encode_empty_token(22, &mut bytes),
        Token::Cdo => encode_empty_token(23, &mut bytes),
        Token::Cdc => encode_empty_token(24, &mut bytes),
        Token::Function(value) => encode_token_string(25, value, &mut bytes, context),
        Token::ParenthesisBlock => encode_empty_token(26, &mut bytes),
        Token::SquareBracketBlock => encode_empty_token(27, &mut bytes),
        Token::CurlyBracketBlock => encode_empty_token(28, &mut bytes),
        Token::BadUrl(value) => encode_token_string(29, value, &mut bytes, context),
        Token::BadString(value) => encode_token_string(30, value, &mut bytes, context),
        Token::CloseParenthesis => encode_empty_token(31, &mut bytes),
        Token::CloseSquareBracket => encode_empty_token(32, &mut bytes),
        Token::CloseCurlyBracket => encode_empty_token(33, &mut bytes),
    };
    bytes[4..8].copy_from_slice(&data.to_le_bytes());
    NodePayload::inline(&bytes)
}

fn encode_token_string<'ast>(
    tag: u8,
    value: &'ast str,
    bytes: &mut [u8; NodePayload::INLINE_BYTES],
    context: &mut AstContext<'ast>,
) -> u32 {
    bytes[0] = tag;
    context.store_string(value)
}

fn encode_empty_token(tag: u8, bytes: &mut [u8; NodePayload::INLINE_BYTES]) -> u32 {
    bytes[0] = tag;
    0
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

pub(crate) fn encode_angle(angle: Angle) -> (u8, f32) {
    match angle {
        Angle::Deg(value) => (0, value),
        Angle::Rad(value) => (1, value),
        Angle::Grad(value) => (2, value),
        Angle::Turn(value) => (3, value),
    }
}

pub(crate) fn decode_angle(kind: u8, value: f32) -> Angle {
    match kind {
        0 => Angle::Deg(value),
        1 => Angle::Rad(value),
        2 => Angle::Grad(value),
        3 => Angle::Turn(value),
        _ => panic!("invalid encoded Angle"),
    }
}

pub(crate) fn encode_time(time: Time) -> (u8, f32) {
    match time {
        Time::Seconds(value) => (0, value),
        Time::Milliseconds(value) => (1, value),
    }
}

pub(crate) fn decode_time(kind: u8, value: f32) -> Time {
    match kind {
        0 => Time::Seconds(value),
        1 => Time::Milliseconds(value),
        _ => panic!("invalid encoded Time"),
    }
}

pub(crate) fn encode_resolution(resolution: Resolution) -> (u8, f32) {
    match resolution {
        Resolution::Dpi(value) => (0, value),
        Resolution::Dpcm(value) => (1, value),
        Resolution::Dppx(value) => (2, value),
    }
}

pub(crate) fn decode_resolution(kind: u8, value: f32) -> Resolution {
    match kind {
        0 => Resolution::Dpi(value),
        1 => Resolution::Dpcm(value),
        2 => Resolution::Dppx(value),
        _ => panic!("invalid encoded Resolution"),
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

#[derive(Debug, PartialEq, Visit)]
pub enum Specifier<'a> {
    Global,
    File(&'a str),
    SourceIndex(u32),
}

#[derive(Debug, PartialEq, Visit)]
pub enum AnimationName<'a> {
    None,
    Ident(&'a str),
    String(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for AnimationName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0005_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        decode_animation_name(&payload.bytes(), context)
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_animation_name_node(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_animation_name_node(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for AnimationName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

impl<'ast> ExtraDataCompact<'ast> for AnimationName<'ast> {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        let payload = encode_animation_name_node(self, context);
        ExtraData::from_bytes(&payload.bytes()[..ExtraData::BYTES])
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        decode_animation_name(&data.bytes(), context)
    }
}

impl<'ast> ExtraDataClone<'ast> for AnimationName<'ast> {
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_animation_name_node<'ast>(
    value: AnimationName<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        AnimationName::None => bytes[0] = 0,
        AnimationName::Ident(value) => {
            bytes[0] = 1;
            bytes[4..8].copy_from_slice(&context.store_string(value).to_le_bytes());
        }
        AnimationName::String(value) => {
            bytes[0] = 2;
            bytes[4..8].copy_from_slice(&context.store_string(value).to_le_bytes());
        }
    }
    NodePayload::inline(&bytes)
}

fn decode_animation_name<'ast>(bytes: &[u8], context: &AstContext<'ast>) -> AnimationName<'ast> {
    let string =
        || context.resolve_string(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as u64);
    match bytes[0] {
        0 => AnimationName::None,
        1 => AnimationName::Ident(string()),
        2 => AnimationName::String(string()),
        _ => panic!("invalid encoded AnimationName variant"),
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum EnvironmentVariableName<'a> {
    UA(UAEnvironmentVariable),
    Custom(NodeId<'a, DashedIdentReference<'a>>),
    Unknown(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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
    fn token_codec_round_trips_string_and_dimension_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(
            Token::UnknownDimension {
                unit: "furlong",
                value: 2.5,
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(id),
            Token::UnknownDimension {
                unit: "furlong",
                value: 2.5,
            }
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
    fn token_or_value_occupies_one_shared_extra_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let token = context.alloc_encoded_node(Token::Ident("name"), DUMMY_SP);
        let values = context.alloc_encoded_vec(
            [
                TokenOrValue::Token(token),
                TokenOrValue::Length(LengthValue {
                    unit: LengthUnit::Px,
                    value: 3.0,
                }),
                TokenOrValue::Angle(Angle::Turn(0.5)),
                TokenOrValue::DashedIdent("--custom"),
            ]
            .into_iter(),
        );

        let TokenOrValue::Token(decoded_token) = context.encoded_vec_get(values, 0).unwrap() else {
            panic!("expected token identity");
        };
        assert_eq!(context.encoded_node(decoded_token), Token::Ident("name"));
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
            Some(TokenOrValue::DashedIdent("--custom"))
        );
        assert_eq!(values.len(), 4);
    }

    #[test]
    fn animation_name_uses_the_same_compact_layout_as_node_and_list_storage() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(AnimationName::String("slide-in"), DUMMY_SP);
        let cloned = context.clone_encoded_node(id);
        assert_ne!(id, cloned);
        assert_eq!(
            context.encoded_node(cloned),
            AnimationName::String("slide-in")
        );

        let names = context.alloc_encoded_vec([AnimationName::Ident("fade")].into_iter());
        assert_eq!(
            context.encoded_vec_get(names, 0),
            Some(AnimationName::Ident("fade"))
        );
    }
}

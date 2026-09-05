use crate::*;

use crate::{
    AstNodeClone, AstNodeStorage, ExtraData, ExtraDataClone, ExtraDataCompact, NodeKind,
    NodePayload,
};

#[derive(Debug, PartialEq, Visit)]
pub enum ParsedComponent<'a> {
    Length(NodeId<'a, Length<'a>>),
    Number(f32),
    Percentage(f32),
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
    String(&'a str),
    Color(NodeId<'a, CssColor<'a>>),
    Image(NodeId<'a, Image<'a>>),
    Url(NodeId<'a, Url<'a>>),
    Integer(i32),
    Angle(Angle),
    Time(Time),
    Resolution(Resolution),
    TransformFunction(NodeId<'a, Transform<'a>>),
    TransformList(Vec<'a, NodeId<'a, Transform<'a>>>),
    CustomIdent(&'a str),
    Literal(&'a str),
    Repeated {
        components: Vec<'a, NodeId<'a, ParsedComponent<'a>>>,
        multiplier: Multiplier,
    },
    TokenList(Vec<'a, TokenOrValue<'a>>),
}

// byte 0       variant
// byte 1       scalar unit or multiplier
// bytes 2..4   reserved
// bytes 4..8   scalar bits, compact string ID, child ID, or range start
// bytes 8..12  range end
// bytes 12..16 reserved
impl<'ast> AstNodeStorage<'ast> for ParsedComponent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let id = read_u32(&bytes, 4) as usize;
        let scalar = f32::from_bits(read_u32(&bytes, 4));
        match bytes[0] {
            0 => Self::Length(context.encoded_node_id_at(id)),
            1 => Self::Number(scalar),
            2 => Self::Percentage(scalar),
            3 => Self::LengthPercentage(context.encoded_node_id_at(id)),
            4 => Self::String(context.resolve_string(id as u64)),
            5 => Self::Color(context.encoded_node_id_at(id)),
            6 => Self::Image(context.encoded_node_id_at(id)),
            7 => Self::Url(context.encoded_node_id_at(id)),
            8 => Self::Integer(read_u32(&bytes, 4) as i32),
            9 => Self::Angle(crate::token::decode_angle(bytes[1], scalar)),
            10 => Self::Time(crate::token::decode_time(bytes[1], scalar)),
            11 => Self::Resolution(crate::token::decode_resolution(bytes[1], scalar)),
            12 => Self::TransformFunction(context.encoded_node_id_at(id)),
            13 => Self::TransformList(decode_range(&bytes, 4, context)),
            14 => Self::CustomIdent(context.resolve_string(id as u64)),
            15 => Self::Literal(context.resolve_string(id as u64)),
            16 => Self::Repeated {
                components: decode_range(&bytes, 4, context),
                multiplier: decode_multiplier(bytes[1]),
            },
            17 => Self::TokenList(decode_range(&bytes, 4, context)),
            _ => panic!("invalid encoded ParsedComponent variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_parsed_component(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_parsed_component(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for ParsedComponent<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Number(value) => Self::Number(value),
            Self::Percentage(value) => Self::Percentage(value),
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            Self::String(value) => Self::String(value),
            Self::Color(value) => Self::Color(context.clone_encoded_node(value)),
            Self::Image(value) => Self::Image(context.clone_encoded_node(value)),
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Integer(value) => Self::Integer(value),
            Self::Angle(value) => Self::Angle(value),
            Self::Time(value) => Self::Time(value),
            Self::Resolution(value) => Self::Resolution(value),
            Self::TransformFunction(value) => {
                Self::TransformFunction(context.clone_encoded_node(value))
            }
            Self::TransformList(values) => Self::TransformList(context.clone_encoded_vec(values)),
            Self::CustomIdent(value) => Self::CustomIdent(value),
            Self::Literal(value) => Self::Literal(value),
            Self::Repeated {
                components,
                multiplier,
            } => Self::Repeated {
                components: context.clone_encoded_vec(components),
                multiplier,
            },
            Self::TokenList(values) => Self::TokenList(context.clone_encoded_vec(values)),
        }
    }
}

fn encode_parsed_component<'ast>(
    value: ParsedComponent<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        ParsedComponent::Length(value) => write_tagged_id(&mut bytes, 0, value),
        ParsedComponent::Number(value) => write_tagged_float(&mut bytes, 1, value),
        ParsedComponent::Percentage(value) => write_tagged_float(&mut bytes, 2, value),
        ParsedComponent::LengthPercentage(value) => write_tagged_id(&mut bytes, 3, value),
        ParsedComponent::String(value) => write_tagged_string(&mut bytes, 4, value, context),
        ParsedComponent::Color(value) => write_tagged_id(&mut bytes, 5, value),
        ParsedComponent::Image(value) => write_tagged_id(&mut bytes, 6, value),
        ParsedComponent::Url(value) => write_tagged_id(&mut bytes, 7, value),
        ParsedComponent::Integer(value) => {
            bytes[0] = 8;
            write_u32(&mut bytes, 4, value as u32);
        }
        ParsedComponent::Angle(value) => {
            let (kind, value) = crate::token::encode_angle(value);
            write_tagged_scalar(&mut bytes, 9, kind, value);
        }
        ParsedComponent::Time(value) => {
            let (kind, value) = crate::token::encode_time(value);
            write_tagged_scalar(&mut bytes, 10, kind, value);
        }
        ParsedComponent::Resolution(value) => {
            let (kind, value) = crate::token::encode_resolution(value);
            write_tagged_scalar(&mut bytes, 11, kind, value);
        }
        ParsedComponent::TransformFunction(value) => write_tagged_id(&mut bytes, 12, value),
        ParsedComponent::TransformList(values) => write_tagged_range(&mut bytes, 13, values),
        ParsedComponent::CustomIdent(value) => write_tagged_string(&mut bytes, 14, value, context),
        ParsedComponent::Literal(value) => write_tagged_string(&mut bytes, 15, value, context),
        ParsedComponent::Repeated {
            components,
            multiplier,
        } => {
            write_tagged_range(&mut bytes, 16, components);
            bytes[1] = encode_multiplier(multiplier);
        }
        ParsedComponent::TokenList(values) => write_tagged_range(&mut bytes, 17, values),
    }
    NodePayload::inline(&bytes)
}

fn write_tagged_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_tagged_float(bytes: &mut [u8], tag: u8, value: f32) {
    bytes[0] = tag;
    write_u32(bytes, 4, value.to_bits());
}

fn write_tagged_scalar(bytes: &mut [u8], tag: u8, kind: u8, value: f32) {
    write_tagged_float(bytes, tag, value);
    bytes[1] = kind;
}

fn write_tagged_string<'ast>(
    bytes: &mut [u8],
    tag: u8,
    value: &'ast str,
    context: &mut AstContext<'ast>,
) {
    bytes[0] = tag;
    write_u32(bytes, 4, context.store_string(value));
}

fn write_tagged_range<T>(bytes: &mut [u8], tag: u8, values: Vec<'_, T>) {
    bytes[0] = tag;
    write_range(bytes, 4, values);
}

#[derive(Debug, PartialEq, Visit)]
pub enum Multiplier {
    None,
    Space,
    Comma,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SyntaxString<'a> {
    Components(Vec<'a, SyntaxComponent<'a>>),
    Universal,
}

impl<'ast> AstNodeStorage<'ast> for SyntaxString<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Components(decode_range(&bytes, 4, context)),
            1 => Self::Universal,
            _ => panic!("invalid encoded SyntaxString variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Components(values) => {
                bytes[0] = 0;
                write_range(&mut bytes, 4, values);
            }
            Self::Universal => bytes[0] = 1,
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for SyntaxString<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Components(values) => Self::Components(context.clone_encoded_vec(values)),
            Self::Universal => Self::Universal,
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum SyntaxComponentKind<'a> {
    Length,
    Number,
    Percentage,
    LengthPercentage,
    String,
    Color,
    Image,
    Url,
    Integer,
    Angle,
    Time,
    Resolution,
    TransformFunction,
    TransformList,
    CustomIdent,
    Literal(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for SyntaxComponentKind<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        decode_syntax_component_kind(bytes[0], read_u32(&bytes, 4), context)
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let (kind, value) = encode_syntax_component_kind(self, context);
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        bytes[0] = kind;
        write_u32(&mut bytes, 4, value);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for SyntaxComponentKind<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnparsedProperty<'a> {
    pub property_id: NodeId<'a, PropertyId<'a>>,
    #[visit(skip)]
    pub reason: UnparsedPropertyReason,
    /// The authored value after removing declaration-level whitespace and
    /// `!important`. This keeps fallback serialization independent from the
    /// lossy numeric and function normalization used by typed tokens.
    #[visit(skip)]
    pub raw_value: Option<&'a str>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

// bytes 0..4 property ID, byte 4 reason, byte 5 raw-value presence,
// bytes 8..12 raw-value string ID, bytes 12..16 first extra slot;
// extra + 0 stores the token range.
impl<'ast> AstNodeStorage<'ast> for UnparsedProperty<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            property_id: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            reason: decode_unparsed_reason(bytes[4]),
            raw_value: match bytes[5] {
                0 => None,
                1 => Some(context.resolve_string(read_u32(&bytes, 8) as u64)),
                _ => panic!("invalid encoded raw-value presence"),
            },
            value: ExtraDataCompact::decode_extra(
                context.extra_slot(payload.extra_start()),
                context,
            ),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_unparsed_property(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_unparsed_property(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for UnparsedProperty<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            property_id: context.clone_encoded_node(self.property_id),
            reason: self.reason,
            raw_value: self.raw_value,
            value: context.clone_encoded_vec(self.value),
        }
    }
}

fn encode_unparsed_property<'ast>(
    value: UnparsedProperty<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_u32(
        &mut bytes,
        0,
        u32::try_from(value.property_id.index()).expect("AST node ID exceeds four bytes"),
    );
    bytes[4] = encode_unparsed_reason(value.reason);
    if let Some(raw_value) = value.raw_value {
        bytes[5] = 1;
        write_u32(&mut bytes, 8, context.store_string(raw_value));
    }
    let range = value.value.encode_extra(context);
    let extra = match existing_extra {
        Some(extra) => {
            context.set_extra_slot(extra, range);
            extra
        }
        None => context.alloc_extra_slots([range]),
    };
    NodePayload::with_extra(&bytes, extra)
}

/// Why a declaration could not be represented by its typed value AST.
///
/// Keeping this decision in the parsed tree lets transforms distinguish
/// unsupported grammar from values whose syntax or semantics are opaque.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum UnparsedPropertyReason {
    /// RocketCSS recognizes the property, but does not implement its grammar yet.
    UnsupportedGrammar,
    /// The property name is unknown, so its value grammar is also unknown.
    UnknownProperty,
    /// A supported grammar contains a function or comment that cannot be
    /// validated without preserving its original token representation.
    OpaqueValue,
    /// The implemented grammar rejected an otherwise tokenizable value.
    InvalidValue,
}

#[derive(Debug, PartialEq, Visit)]
pub struct CustomProperty<'a> {
    pub name: NodeId<'a, CustomPropertyName<'a>>,
    pub value: Vec<'a, TokenOrValue<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for CustomProperty<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0017_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        Self {
            name: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            value: decode_range(&bytes, 4, context),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        write_u32(
            &mut bytes,
            0,
            u32::try_from(self.name.index()).expect("AST node ID exceeds four bytes"),
        );
        write_range(&mut bytes, 4, self.value);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CustomProperty<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            name: context.clone_encoded_node(self.name),
            value: context.clone_encoded_vec(self.value),
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct SyntaxComponent<'a> {
    pub kind: NodeId<'a, SyntaxComponentKind<'a>>,
    pub multiplier: Multiplier,
}

impl<'ast> ExtraDataCompact<'ast> for SyntaxComponent<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        write_u32(
            &mut bytes,
            0,
            u32::try_from(self.kind.index()).expect("AST node ID exceeds four bytes"),
        );
        bytes[4] = encode_multiplier(self.multiplier);
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        Self {
            kind: context.encoded_node_id_at(read_u32(&bytes, 0) as usize),
            multiplier: decode_multiplier(bytes[4]),
        }
    }
}

impl<'ast> ExtraDataClone<'ast> for SyntaxComponent<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            kind: context.clone_encoded_node(self.kind),
            multiplier: self.multiplier,
        }
    }
}

fn encode_syntax_component_kind<'ast>(
    value: SyntaxComponentKind<'ast>,
    context: &mut AstContext<'ast>,
) -> (u8, u32) {
    match value {
        SyntaxComponentKind::Length => (0, 0),
        SyntaxComponentKind::Number => (1, 0),
        SyntaxComponentKind::Percentage => (2, 0),
        SyntaxComponentKind::LengthPercentage => (3, 0),
        SyntaxComponentKind::String => (4, 0),
        SyntaxComponentKind::Color => (5, 0),
        SyntaxComponentKind::Image => (6, 0),
        SyntaxComponentKind::Url => (7, 0),
        SyntaxComponentKind::Integer => (8, 0),
        SyntaxComponentKind::Angle => (9, 0),
        SyntaxComponentKind::Time => (10, 0),
        SyntaxComponentKind::Resolution => (11, 0),
        SyntaxComponentKind::TransformFunction => (12, 0),
        SyntaxComponentKind::TransformList => (13, 0),
        SyntaxComponentKind::CustomIdent => (14, 0),
        SyntaxComponentKind::Literal(value) => (15, context.store_string(value)),
    }
}

fn decode_syntax_component_kind<'ast>(
    kind: u8,
    value: u32,
    context: &AstContext<'ast>,
) -> SyntaxComponentKind<'ast> {
    match kind {
        0 => SyntaxComponentKind::Length,
        1 => SyntaxComponentKind::Number,
        2 => SyntaxComponentKind::Percentage,
        3 => SyntaxComponentKind::LengthPercentage,
        4 => SyntaxComponentKind::String,
        5 => SyntaxComponentKind::Color,
        6 => SyntaxComponentKind::Image,
        7 => SyntaxComponentKind::Url,
        8 => SyntaxComponentKind::Integer,
        9 => SyntaxComponentKind::Angle,
        10 => SyntaxComponentKind::Time,
        11 => SyntaxComponentKind::Resolution,
        12 => SyntaxComponentKind::TransformFunction,
        13 => SyntaxComponentKind::TransformList,
        14 => SyntaxComponentKind::CustomIdent,
        15 => SyntaxComponentKind::Literal(context.resolve_string(value as u64)),
        _ => panic!("invalid encoded SyntaxComponentKind"),
    }
}

fn encode_multiplier(value: Multiplier) -> u8 {
    match value {
        Multiplier::None => 0,
        Multiplier::Space => 1,
        Multiplier::Comma => 2,
    }
}

fn decode_multiplier(value: u8) -> Multiplier {
    match value {
        0 => Multiplier::None,
        1 => Multiplier::Space,
        2 => Multiplier::Comma,
        _ => panic!("invalid encoded Multiplier"),
    }
}

fn encode_unparsed_reason(value: UnparsedPropertyReason) -> u8 {
    match value {
        UnparsedPropertyReason::UnsupportedGrammar => 0,
        UnparsedPropertyReason::UnknownProperty => 1,
        UnparsedPropertyReason::OpaqueValue => 2,
        UnparsedPropertyReason::InvalidValue => 3,
    }
}

fn decode_unparsed_reason(value: u8) -> UnparsedPropertyReason {
    match value {
        0 => UnparsedPropertyReason::UnsupportedGrammar,
        1 => UnparsedPropertyReason::UnknownProperty,
        2 => UnparsedPropertyReason::OpaqueValue,
        3 => UnparsedPropertyReason::InvalidValue,
        _ => panic!("invalid encoded UnparsedPropertyReason"),
    }
}

fn write_range<T>(bytes: &mut [u8], offset: usize, range: Vec<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(range.start_index()).expect("AST range exceeds four bytes"),
    );
    write_u32(
        bytes,
        offset + 4,
        u32::try_from(range.end_index()).expect("AST range exceeds four bytes"),
    );
}

fn decode_range<'ast, T>(bytes: &[u8], offset: usize, context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(
        read_u32(bytes, offset) as usize,
        read_u32(bytes, offset + 4) as usize,
    )
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, CustomProperty, CustomPropertyName, DUMMY_SP, KeyframesName,
        NoneOrCustomIdentList, ParsedComponent, PropertyId, SyntaxComponent, SyntaxComponentKind,
        SyntaxString, TokenOrValue, UnparsedProperty, UnparsedPropertyReason,
    };

    use super::Multiplier;

    #[test]
    fn property_metadata_codecs_round_trip_and_deep_clone_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let literal = context.alloc_encoded_node(SyntaxComponentKind::Literal("|"), DUMMY_SP);
        let components = context.alloc_encoded_vec(
            [SyntaxComponent {
                kind: literal,
                multiplier: Multiplier::Comma,
            }]
            .into_iter(),
        );
        let syntax = context.alloc_encoded_node(SyntaxString::Components(components), DUMMY_SP);
        let cloned = context.clone_encoded_node(syntax);
        let SyntaxString::Components(cloned_components) = context.encoded_node(cloned) else {
            panic!("expected component syntax")
        };
        assert_ne!(components, cloned_components);
        let component = context
            .encoded_vec_get(cloned_components, 0)
            .expect("cloned syntax component");
        assert_ne!(component.kind, literal);
        assert_eq!(
            context.encoded_node(component.kind),
            SyntaxComponentKind::Literal("|")
        );
    }

    #[test]
    fn parsed_component_codec_deep_clones_recursive_component_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let literal = context.alloc_encoded_node(ParsedComponent::Literal("/"), DUMMY_SP);
        let components = context.alloc_encoded_vec([literal].into_iter());
        let repeated = context.alloc_encoded_node(
            ParsedComponent::Repeated {
                components,
                multiplier: Multiplier::Space,
            },
            DUMMY_SP,
        );

        let cloned = context.clone_encoded_node(repeated);
        let ParsedComponent::Repeated {
            components: cloned_components,
            multiplier,
        } = context.encoded_node(cloned)
        else {
            panic!("expected repeated component")
        };
        assert_eq!(multiplier, Multiplier::Space);
        assert_ne!(cloned_components, components);
        let cloned_literal = context
            .encoded_vec_get(cloned_components, 0)
            .expect("cloned parsed component");
        assert_ne!(cloned_literal, literal);
        assert_eq!(
            context.encoded_node(cloned_literal),
            ParsedComponent::Literal("/")
        );
    }

    #[test]
    fn unparsed_property_reuses_its_fixed_overflow_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let property_id = context.alloc_encoded_node(PropertyId::Custom("future-prop"), DUMMY_SP);
        let value =
            context.alloc_encoded_vec([TokenOrValue::DashedIdent("--fallback")].into_iter());
        let before = context.encoded_extra_len();
        let property = context.alloc_encoded_node(
            UnparsedProperty {
                property_id,
                reason: UnparsedPropertyReason::UnknownProperty,
                raw_value: Some(" var(--fallback) "),
                value,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(context.encoded_node(property).value, value);

        context.mutate_encoded_node(property, |value, _| {
            value.reason = UnparsedPropertyReason::OpaqueValue;
            value.raw_value = None;
        });
        assert_eq!(context.encoded_extra_len(), before + 1);
        let decoded = context.encoded_node(property);
        assert_eq!(decoded.reason, UnparsedPropertyReason::OpaqueValue);
        assert_eq!(decoded.raw_value, None);

        let cloned = context.clone_encoded_node(property);
        let cloned = context.encoded_node(cloned);
        assert_ne!(cloned.property_id, property_id);
        assert_ne!(cloned.value, value);
        assert_eq!(
            context.encoded_vec_get(cloned.value, 0),
            Some(TokenOrValue::DashedIdent("--fallback"))
        );
    }

    #[test]
    fn compact_name_and_custom_property_codecs_keep_typed_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.alloc_encoded_node(CustomPropertyName::Custom("--theme"), DUMMY_SP);
        let value = context.alloc_encoded_vec([TokenOrValue::DashedIdent("--base")].into_iter());
        let property = context.alloc_encoded_node(CustomProperty { name, value }, DUMMY_SP);
        let cloned_property = context.clone_encoded_node(property);
        let cloned = context.encoded_node(cloned_property);
        assert_ne!(cloned.name, name);
        assert_ne!(cloned.value, value);
        assert_eq!(
            context.encoded_node(cloned.name),
            CustomPropertyName::Custom("--theme")
        );

        let keyframes = context.alloc_encoded_node(KeyframesName::Custom("--motion"), DUMMY_SP);
        assert_eq!(
            context.encoded_node(keyframes),
            KeyframesName::Custom("--motion")
        );
        let idents = context.alloc_encoded_vec(["one", "two"].into_iter());
        let names = context.alloc_encoded_node(NoneOrCustomIdentList::Idents(idents), DUMMY_SP);
        let cloned_names = context.clone_encoded_node(names);
        let NoneOrCustomIdentList::Idents(cloned_idents) = context.encoded_node(cloned_names)
        else {
            panic!("expected custom identifier list")
        };
        assert_ne!(cloned_idents, idents);
        assert_eq!(context.encoded_vec_get(cloned_idents, 1), Some("two"));
    }
}

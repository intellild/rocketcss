use super::*;

#[derive(Debug, PartialEq, Visit)]
pub enum MediaCondition<'a> {
    Feature(NodeId<'a, MediaFeature<'a>>),
    Not(NodeId<'a, MediaCondition<'a>>),
    Operation {
        conditions: Vec<'a, NodeId<'a, MediaCondition<'a>>>,
        operator: Operator,
    },
    Unknown(Vec<'a, TokenOrValue<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum QueryFeature<'a, FeatureId> {
    Plain {
        name: MediaFeatureName<'a, FeatureId>,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    Boolean {
        name: MediaFeatureName<'a, FeatureId>,
    },
    Range {
        name: MediaFeatureName<'a, FeatureId>,
        operator: MediaFeatureComparison,
        value: NodeId<'a, MediaFeatureValue<'a>>,
    },
    Interval {
        end: NodeId<'a, MediaFeatureValue<'a>>,
        end_operator: MediaFeatureComparison,
        name: MediaFeatureName<'a, FeatureId>,
        start: NodeId<'a, MediaFeatureValue<'a>>,
        start_operator: MediaFeatureComparison,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureName<'a, FeatureId> {
    Standard(FeatureId),
    Custom(&'a str),
    Unknown(&'a str),
}

pub type MediaFeature<'a> = QueryFeature<'a, MediaFeatureId>;

#[repr(u8)]
#[derive(CssKeyword, Debug, PartialEq, Visit)]
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
    #[css_keyword("-webkit-device-pixel-ratio")]
    WebkitDevicePixelRatio,
    #[css_keyword("-moz-device-pixel-ratio")]
    MozDevicePixelRatio,
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureValue<'a> {
    Length(NodeId<'a, Length<'a>>),
    Number(f32),
    Integer(i32),
    Boolean(bool),
    Resolution(Resolution),
    Ratio(Ratio),
    Ident(&'a str),
    Env(NodeId<'a, EnvironmentVariable<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum MediaFeatureComparison {
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Operator {
    And,
    Or,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MediaType<'a> {
    All,
    Print,
    Screen,
    Custom(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Qualifier {
    Only,
    Not,
}

#[derive(Debug, PartialEq, Visit)]
pub enum SupportsCondition<'a> {
    Not(NodeId<'a, SupportsCondition<'a>>),
    And(Vec<'a, NodeId<'a, SupportsCondition<'a>>>),
    Or(Vec<'a, NodeId<'a, SupportsCondition<'a>>>),
    Declaration {
        property_id: NodeId<'a, PropertyId<'a>>,
        value: &'a str,
    },
    Selector(&'a str),
    Unknown(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for MediaCondition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Feature(read_node_id(&bytes, context)),
            1 => Self::Not(read_node_id(&bytes, context)),
            2 => Self::Operation {
                conditions: read_range(&bytes, context),
                operator: decode_operator(bytes[1]),
            },
            3 => Self::Unknown(read_range(&bytes, context)),
            _ => panic!("invalid encoded MediaCondition variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_media_condition(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_media_condition(self)
    }
}

impl<'ast> AstNodeClone<'ast> for MediaCondition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Feature(value) => Self::Feature(context.clone_encoded_node(value)),
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::Operation {
                conditions,
                operator,
            } => Self::Operation {
                conditions: context.clone_encoded_vec(conditions),
                operator,
            },
            Self::Unknown(values) => Self::Unknown(context.clone_encoded_vec(values)),
        }
    }
}

fn encode_media_condition(value: MediaCondition<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        MediaCondition::Feature(value) => write_node_id(&mut bytes, 0, value),
        MediaCondition::Not(value) => write_node_id(&mut bytes, 1, value),
        MediaCondition::Operation {
            conditions,
            operator,
        } => {
            write_range(&mut bytes, 2, conditions);
            bytes[1] = encode_operator(operator);
        }
        MediaCondition::Unknown(values) => write_range(&mut bytes, 3, values),
    }
    NodePayload::inline(&bytes)
}

pub(crate) trait QueryFeatureIdCodec: Sized {
    const KIND: NodeKind;

    fn encode(self) -> u8;

    fn decode(value: u8) -> Self;
}

impl QueryFeatureIdCodec for MediaFeatureId {
    const KIND: NodeKind = NodeKind::new(0x001a_0002);

    fn encode(self) -> u8 {
        self as u8
    }

    fn decode(value: u8) -> Self {
        match value {
            0 => Self::Width,
            1 => Self::Height,
            2 => Self::AspectRatio,
            3 => Self::Orientation,
            4 => Self::OverflowBlock,
            5 => Self::OverflowInline,
            6 => Self::HorizontalViewportSegments,
            7 => Self::VerticalViewportSegments,
            8 => Self::DisplayMode,
            9 => Self::Resolution,
            10 => Self::Scan,
            11 => Self::Grid,
            12 => Self::Update,
            13 => Self::EnvironmentBlending,
            14 => Self::Color,
            15 => Self::ColorIndex,
            16 => Self::Monochrome,
            17 => Self::ColorGamut,
            18 => Self::DynamicRange,
            19 => Self::InvertedColors,
            20 => Self::Pointer,
            21 => Self::Hover,
            22 => Self::AnyPointer,
            23 => Self::AnyHover,
            24 => Self::NavControls,
            25 => Self::VideoColorGamut,
            26 => Self::VideoDynamicRange,
            27 => Self::Scripting,
            28 => Self::PrefersReducedMotion,
            29 => Self::PrefersReducedTransparency,
            30 => Self::PrefersContrast,
            31 => Self::ForcedColors,
            32 => Self::PrefersColorScheme,
            33 => Self::PrefersReducedData,
            34 => Self::DeviceWidth,
            35 => Self::DeviceHeight,
            36 => Self::DeviceAspectRatio,
            37 => Self::WebkitDevicePixelRatio,
            38 => Self::MozDevicePixelRatio,
            _ => panic!("invalid encoded MediaFeatureId"),
        }
    }
}

impl<'ast, FeatureId: QueryFeatureIdCodec> AstNodeStorage<'ast> for QueryFeature<'ast, FeatureId> {
    const KIND: NodeKind = FeatureId::KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let name = decode_feature_name(&bytes, context);
        match bytes[0] {
            0 => Self::Plain {
                name,
                value: read_node_id_at(&bytes, 8, context),
            },
            1 => Self::Boolean { name },
            2 => Self::Range {
                name,
                operator: decode_comparison(bytes[2]),
                value: read_node_id_at(&bytes, 8, context),
            },
            3 => Self::Interval {
                end: read_node_id_at(&bytes, 12, context),
                end_operator: decode_comparison(bytes[3]),
                name,
                start: read_node_id_at(&bytes, 8, context),
                start_operator: decode_comparison(bytes[2]),
            },
            _ => panic!("invalid encoded QueryFeature variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_query_feature(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_query_feature(self, context)
    }
}

impl<'ast, FeatureId: QueryFeatureIdCodec> AstNodeClone<'ast> for QueryFeature<'ast, FeatureId> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Plain { name, value } => Self::Plain {
                name,
                value: context.clone_encoded_node(value),
            },
            Self::Boolean { name } => Self::Boolean { name },
            Self::Range {
                name,
                operator,
                value,
            } => Self::Range {
                name,
                operator,
                value: context.clone_encoded_node(value),
            },
            Self::Interval {
                end,
                end_operator,
                name,
                start,
                start_operator,
            } => Self::Interval {
                end: context.clone_encoded_node(end),
                end_operator,
                name,
                start: context.clone_encoded_node(start),
                start_operator,
            },
        }
    }
}

fn encode_query_feature<'ast, FeatureId: QueryFeatureIdCodec>(
    value: QueryFeature<'ast, FeatureId>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        QueryFeature::Plain { name, value } => {
            bytes[0] = 0;
            encode_feature_name(name, &mut bytes, context);
            write_id_at(&mut bytes, 8, value);
        }
        QueryFeature::Boolean { name } => {
            bytes[0] = 1;
            encode_feature_name(name, &mut bytes, context);
        }
        QueryFeature::Range {
            name,
            operator,
            value,
        } => {
            bytes[0] = 2;
            bytes[2] = encode_comparison(operator);
            encode_feature_name(name, &mut bytes, context);
            write_id_at(&mut bytes, 8, value);
        }
        QueryFeature::Interval {
            end,
            end_operator,
            name,
            start,
            start_operator,
        } => {
            bytes[0] = 3;
            bytes[2] = encode_comparison(start_operator);
            bytes[3] = encode_comparison(end_operator);
            encode_feature_name(name, &mut bytes, context);
            write_id_at(&mut bytes, 8, start);
            write_id_at(&mut bytes, 12, end);
        }
    }
    NodePayload::inline(&bytes)
}

fn encode_feature_name<'ast, FeatureId: QueryFeatureIdCodec>(
    name: MediaFeatureName<'ast, FeatureId>,
    bytes: &mut [u8],
    context: &mut AstContext<'ast>,
) {
    let (kind, data) = match name {
        MediaFeatureName::Standard(value) => (0, value.encode() as u32),
        MediaFeatureName::Custom(value) => (1, context.store_string(value)),
        MediaFeatureName::Unknown(value) => (2, context.store_string(value)),
    };
    bytes[1] = kind;
    write_u32(bytes, 4, data);
}

fn decode_feature_name<'ast, FeatureId: QueryFeatureIdCodec>(
    bytes: &[u8],
    context: &AstContext<'ast>,
) -> MediaFeatureName<'ast, FeatureId> {
    let data = read_u32(bytes, 4);
    match bytes[1] {
        0 => MediaFeatureName::Standard(FeatureId::decode(data as u8)),
        1 => MediaFeatureName::Custom(context.resolve_string(data as u64)),
        2 => MediaFeatureName::Unknown(context.resolve_string(data as u64)),
        _ => panic!("invalid encoded MediaFeatureName variant"),
    }
}

impl<'ast> AstNodeStorage<'ast> for MediaFeatureValue<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let data = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::Length(context.encoded_node_id_at(data as usize)),
            1 => Self::Number(f32::from_bits(data)),
            2 => Self::Integer(data as i32),
            3 => Self::Boolean(match data {
                0 => false,
                1 => true,
                _ => panic!("invalid encoded MediaFeatureValue boolean"),
            }),
            4 => Self::Resolution(crate::token::decode_resolution(
                bytes[1],
                f32::from_bits(data),
            )),
            5 => Self::Ratio(Ratio {
                denominator: match bytes[1] {
                    0 => None,
                    1 => Some(f32::from_bits(read_u32(&bytes, 8))),
                    _ => panic!("invalid encoded Ratio denominator flag"),
                },
                numerator: f32::from_bits(data),
            }),
            6 => Self::Ident(context.resolve_string(data as u64)),
            7 => Self::Env(context.encoded_node_id_at(data as usize)),
            _ => panic!("invalid encoded MediaFeatureValue variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_media_feature_value(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_media_feature_value(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for MediaFeatureValue<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Env(value) => Self::Env(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

fn encode_media_feature_value<'ast>(
    value: MediaFeatureValue<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        MediaFeatureValue::Length(value) => write_node_id(&mut bytes, 0, value),
        MediaFeatureValue::Number(value) => write_tagged_u32(&mut bytes, 1, value.to_bits()),
        MediaFeatureValue::Integer(value) => write_tagged_u32(&mut bytes, 2, value as u32),
        MediaFeatureValue::Boolean(value) => write_tagged_u32(&mut bytes, 3, value as u32),
        MediaFeatureValue::Resolution(value) => {
            let (kind, value) = crate::token::encode_resolution(value);
            write_tagged_u32(&mut bytes, 4, value.to_bits());
            bytes[1] = kind;
        }
        MediaFeatureValue::Ratio(value) => {
            write_tagged_u32(&mut bytes, 5, value.numerator.to_bits());
            if let Some(denominator) = value.denominator {
                bytes[1] = 1;
                write_u32(&mut bytes, 8, denominator.to_bits());
            }
        }
        MediaFeatureValue::Ident(value) => {
            write_tagged_u32(&mut bytes, 6, context.store_string(value));
        }
        MediaFeatureValue::Env(value) => write_node_id(&mut bytes, 7, value),
    }
    NodePayload::inline(&bytes)
}

impl<'ast> AstNodeStorage<'ast> for SupportsCondition<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001a_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Not(read_node_id(&bytes, context)),
            1 => Self::And(read_range(&bytes, context)),
            2 => Self::Or(read_range(&bytes, context)),
            3 => Self::Declaration {
                property_id: read_node_id(&bytes, context),
                value: context.resolve_string(read_u32(&bytes, 8) as u64),
            },
            4 => Self::Selector(context.resolve_string(read_u32(&bytes, 4) as u64)),
            5 => Self::Unknown(context.resolve_string(read_u32(&bytes, 4) as u64)),
            _ => panic!("invalid encoded SupportsCondition variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_supports_condition(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_supports_condition(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for SupportsCondition<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Not(value) => Self::Not(context.clone_encoded_node(value)),
            Self::And(values) => Self::And(context.clone_encoded_vec(values)),
            Self::Or(values) => Self::Or(context.clone_encoded_vec(values)),
            Self::Declaration { property_id, value } => Self::Declaration {
                property_id: context.clone_encoded_node(property_id),
                value,
            },
            Self::Selector(value) => Self::Selector(value),
            Self::Unknown(value) => Self::Unknown(value),
        }
    }
}

fn encode_supports_condition<'ast>(
    value: SupportsCondition<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        SupportsCondition::Not(value) => write_node_id(&mut bytes, 0, value),
        SupportsCondition::And(values) => write_range(&mut bytes, 1, values),
        SupportsCondition::Or(values) => write_range(&mut bytes, 2, values),
        SupportsCondition::Declaration { property_id, value } => {
            write_node_id(&mut bytes, 3, property_id);
            write_u32(&mut bytes, 8, context.store_string(value));
        }
        SupportsCondition::Selector(value) => {
            write_tagged_u32(&mut bytes, 4, context.store_string(value));
        }
        SupportsCondition::Unknown(value) => {
            write_tagged_u32(&mut bytes, 5, context.store_string(value));
        }
    }
    NodePayload::inline(&bytes)
}

fn encode_comparison(value: MediaFeatureComparison) -> u8 {
    match value {
        MediaFeatureComparison::Equal => 0,
        MediaFeatureComparison::GreaterThan => 1,
        MediaFeatureComparison::GreaterThanEqual => 2,
        MediaFeatureComparison::LessThan => 3,
        MediaFeatureComparison::LessThanEqual => 4,
    }
}

fn decode_comparison(value: u8) -> MediaFeatureComparison {
    match value {
        0 => MediaFeatureComparison::Equal,
        1 => MediaFeatureComparison::GreaterThan,
        2 => MediaFeatureComparison::GreaterThanEqual,
        3 => MediaFeatureComparison::LessThan,
        4 => MediaFeatureComparison::LessThanEqual,
        _ => panic!("invalid encoded MediaFeatureComparison"),
    }
}

fn encode_operator(value: Operator) -> u8 {
    match value {
        Operator::And => 0,
        Operator::Or => 1,
    }
}

fn decode_operator(value: u8) -> Operator {
    match value {
        0 => Operator::And,
        1 => Operator::Or,
        _ => panic!("invalid encoded Operator"),
    }
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_id_at(bytes, 4, value);
}

fn write_id_at<T>(bytes: &mut [u8], offset: usize, value: NodeId<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn read_node_id<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> NodeId<'ast, T> {
    read_node_id_at(bytes, 4, context)
}

fn read_node_id_at<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn write_range<T>(bytes: &mut [u8], tag: u8, value: Vec<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(value.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        8,
        u32::try_from(value.end_index()).expect("AST range end exceeds four bytes"),
    );
}

fn read_range<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(read_u32(bytes, 4) as usize, read_u32(bytes, 8) as usize)
}

fn write_tagged_u32(bytes: &mut [u8], tag: u8, value: u32) {
    bytes[0] = tag;
    write_u32(bytes, 4, value);
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{AstContext, DUMMY_SP, Length, LengthUnit, LengthValue};

    use super::*;

    #[test]
    fn media_query_codec_deep_clones_condition_tree() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 640.0,
            }),
            DUMMY_SP,
        );
        let value = context.alloc_encoded_node(MediaFeatureValue::Length(length), DUMMY_SP);
        let feature = context.alloc_encoded_node(
            QueryFeature::Range {
                name: MediaFeatureName::Standard(MediaFeatureId::Width),
                operator: MediaFeatureComparison::GreaterThanEqual,
                value,
            },
            DUMMY_SP,
        );
        let child = context.alloc_encoded_node(MediaCondition::Feature(feature), DUMMY_SP);
        let conditions = context.alloc_encoded_vec([child].into_iter());
        let condition = context.alloc_encoded_node(
            MediaCondition::Operation {
                conditions,
                operator: Operator::And,
            },
            DUMMY_SP,
        );
        let query = context.alloc_encoded_node(
            crate::MediaQuery {
                condition: Some(condition),
                media_type: MediaType::Screen,
                qualifier: Some(Qualifier::Only),
            },
            DUMMY_SP,
        );

        let cloned = context.clone_encoded_node(query);
        let cloned_condition = context
            .encoded_node(cloned)
            .condition
            .expect("cloned media condition");
        assert_ne!(cloned_condition, condition);
        let MediaCondition::Operation {
            conditions: cloned_conditions,
            operator: Operator::And,
        } = context.encoded_node(cloned_condition)
        else {
            panic!("expected cloned media operation")
        };
        assert_ne!(cloned_conditions, conditions);
        assert_ne!(context.encoded_vec_get(cloned_conditions, 0), Some(child));
    }

    #[test]
    fn supports_condition_codec_clones_node_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let child =
            context.alloc_encoded_node(SupportsCondition::Unknown("(display:grid)"), DUMMY_SP);
        let values = context.alloc_encoded_vec([child].into_iter());
        let condition = context.alloc_encoded_node(SupportsCondition::And(values), DUMMY_SP);
        let cloned = context.clone_encoded_node(condition);
        let SupportsCondition::And(cloned_values) = context.encoded_node(cloned) else {
            panic!("expected cloned supports operation")
        };
        assert_ne!(cloned_values, values);
        assert_ne!(context.encoded_vec_get(cloned_values, 0), Some(child));
    }
}

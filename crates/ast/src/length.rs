use super::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum Length<'a> {
    Value(LengthValue),
    Calc(NodeId<'a, Calc<'a, Length<'a>>>),
}

// Fixed payload layout for `Length`:
//
// byte 0      variant (0 = value, 1 = calc)
// byte 1      LengthUnit for the value variant
// bytes 2..4  reserved
// bytes 4..8  f32 bits or Calc NodeId index
// bytes 8..16 reserved
impl<'ast> AstNodeStorage<'ast> for Length<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0001_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let data = u32::from_le_bytes(bytes[4..8].try_into().expect("Length data is four bytes"));
        match bytes[0] {
            0 => Self::Value(LengthValue {
                unit: decode_length_unit(bytes[1]),
                value: f32::from_bits(data),
            }),
            1 => Self::Calc(context.encoded_node_id_at(data as usize)),
            _ => panic!("invalid encoded Length variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_length(self)
    }

    fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        encode_length(self)
    }
}

impl<'ast> AstNodeClone<'ast> for Length<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Value(value) => Self::Value(value),
            Self::Calc(value) => Self::Calc(context.clone_encoded_node(value)),
        }
    }
}

#[allow(dead_code)]
fn encode_length(value: Length<'_>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    let data = match value {
        Length::Value(value) => {
            bytes[0] = 0;
            bytes[1] = encode_length_unit(value.unit);
            value.value.to_bits()
        }
        Length::Calc(value) => {
            bytes[0] = 1;
            u32::try_from(value.index()).expect("AST node ID exceeds four bytes")
        }
    };
    bytes[4..8].copy_from_slice(&data.to_le_bytes());
    NodePayload::inline(&bytes)
}

#[allow(dead_code)]
pub(crate) fn encode_length_unit(unit: LengthUnit) -> u8 {
    match unit {
        LengthUnit::Px => 0,
        LengthUnit::In => 1,
        LengthUnit::Cm => 2,
        LengthUnit::Mm => 3,
        LengthUnit::Q => 4,
        LengthUnit::Pt => 5,
        LengthUnit::Pc => 6,
        LengthUnit::Em => 7,
        LengthUnit::Rem => 8,
        LengthUnit::Ex => 9,
        LengthUnit::Rex => 10,
        LengthUnit::Ch => 11,
        LengthUnit::Rch => 12,
        LengthUnit::Cap => 13,
        LengthUnit::Rcap => 14,
        LengthUnit::Ic => 15,
        LengthUnit::Ric => 16,
        LengthUnit::Lh => 17,
        LengthUnit::Rlh => 18,
        LengthUnit::Vw => 19,
        LengthUnit::Lvw => 20,
        LengthUnit::Svw => 21,
        LengthUnit::Dvw => 22,
        LengthUnit::Cqw => 23,
        LengthUnit::Vh => 24,
        LengthUnit::Lvh => 25,
        LengthUnit::Svh => 26,
        LengthUnit::Dvh => 27,
        LengthUnit::Cqh => 28,
        LengthUnit::Vi => 29,
        LengthUnit::Svi => 30,
        LengthUnit::Lvi => 31,
        LengthUnit::Dvi => 32,
        LengthUnit::Cqi => 33,
        LengthUnit::Vb => 34,
        LengthUnit::Svb => 35,
        LengthUnit::Lvb => 36,
        LengthUnit::Dvb => 37,
        LengthUnit::Cqb => 38,
        LengthUnit::Vmin => 39,
        LengthUnit::Svmin => 40,
        LengthUnit::Lvmin => 41,
        LengthUnit::Dvmin => 42,
        LengthUnit::Cqmin => 43,
        LengthUnit::Vmax => 44,
        LengthUnit::Svmax => 45,
        LengthUnit::Lvmax => 46,
        LengthUnit::Dvmax => 47,
        LengthUnit::Cqmax => 48,
    }
}

#[allow(dead_code)]
pub(crate) fn decode_length_unit(unit: u8) -> LengthUnit {
    match unit {
        0 => LengthUnit::Px,
        1 => LengthUnit::In,
        2 => LengthUnit::Cm,
        3 => LengthUnit::Mm,
        4 => LengthUnit::Q,
        5 => LengthUnit::Pt,
        6 => LengthUnit::Pc,
        7 => LengthUnit::Em,
        8 => LengthUnit::Rem,
        9 => LengthUnit::Ex,
        10 => LengthUnit::Rex,
        11 => LengthUnit::Ch,
        12 => LengthUnit::Rch,
        13 => LengthUnit::Cap,
        14 => LengthUnit::Rcap,
        15 => LengthUnit::Ic,
        16 => LengthUnit::Ric,
        17 => LengthUnit::Lh,
        18 => LengthUnit::Rlh,
        19 => LengthUnit::Vw,
        20 => LengthUnit::Lvw,
        21 => LengthUnit::Svw,
        22 => LengthUnit::Dvw,
        23 => LengthUnit::Cqw,
        24 => LengthUnit::Vh,
        25 => LengthUnit::Lvh,
        26 => LengthUnit::Svh,
        27 => LengthUnit::Dvh,
        28 => LengthUnit::Cqh,
        29 => LengthUnit::Vi,
        30 => LengthUnit::Svi,
        31 => LengthUnit::Lvi,
        32 => LengthUnit::Dvi,
        33 => LengthUnit::Cqi,
        34 => LengthUnit::Vb,
        35 => LengthUnit::Svb,
        36 => LengthUnit::Lvb,
        37 => LengthUnit::Dvb,
        38 => LengthUnit::Cqb,
        39 => LengthUnit::Vmin,
        40 => LengthUnit::Svmin,
        41 => LengthUnit::Lvmin,
        42 => LengthUnit::Dvmin,
        43 => LengthUnit::Cqmin,
        44 => LengthUnit::Vmax,
        45 => LengthUnit::Svmax,
        46 => LengthUnit::Lvmax,
        47 => LengthUnit::Dvmax,
        48 => LengthUnit::Cqmax,
        _ => panic!("invalid encoded LengthUnit"),
    }
}

#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
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

#[derive(Debug, PartialEq, Visit)]
pub enum Calc<'a, V> {
    Value(NodeId<'a, V>),
    Number(f32),
    Sum((NodeId<'a, Calc<'a, V>>, NodeId<'a, Calc<'a, V>>)),
    Product((f32, NodeId<'a, Calc<'a, V>>)),
    Function(NodeId<'a, MathFunction<'a, V>>),
}

#[derive(Debug, PartialEq, Visit)]
#[allow(clippy::type_complexity)]
pub enum MathFunction<'a, V> {
    Calc(NodeId<'a, Calc<'a, V>>),
    Min(Vec<'a, NodeId<'a, Calc<'a, V>>>),
    Max(Vec<'a, NodeId<'a, Calc<'a, V>>>),
    Clamp(
        (
            NodeId<'a, Calc<'a, V>>,
            NodeId<'a, Calc<'a, V>>,
            NodeId<'a, Calc<'a, V>>,
        ),
    ),
    Round(
        (
            RoundingStrategy,
            NodeId<'a, Calc<'a, V>>,
            NodeId<'a, Calc<'a, V>>,
        ),
    ),
    Rem((NodeId<'a, Calc<'a, V>>, NodeId<'a, Calc<'a, V>>)),
    Mod((NodeId<'a, Calc<'a, V>>, NodeId<'a, Calc<'a, V>>)),
    Abs(NodeId<'a, Calc<'a, V>>),
    Sign(NodeId<'a, Calc<'a, V>>),
    Hypot(Vec<'a, NodeId<'a, Calc<'a, V>>>),
}

pub(crate) trait CalcValueCodec {
    const CALC_KIND: NodeKind;
    const MATH_FUNCTION_KIND: NodeKind;
}

impl CalcValueCodec for Length<'_> {
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0001);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0001);
}

// byte 0       variant
// bytes 1..4   reserved
// bytes 4..8   value/left/factor/function
// bytes 8..12  right/product value
// bytes 12..16 reserved
impl<'ast, V: CalcValueCodec> AstNodeStorage<'ast> for Calc<'ast, V> {
    const KIND: NodeKind = V::CALC_KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Value(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            1 => Self::Number(f32::from_bits(read_u32(&bytes, 4))),
            2 => Self::Sum((
                context.encoded_node_id_at(read_u32(&bytes, 4) as usize),
                context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            )),
            3 => Self::Product((
                f32::from_bits(read_u32(&bytes, 4)),
                context.encoded_node_id_at(read_u32(&bytes, 8) as usize),
            )),
            4 => Self::Function(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded Calc variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_calc(self)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast, V> AstNodeClone<'ast> for Calc<'ast, V>
where
    V: CalcValueCodec + AstNodeClone<'ast>,
{
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Value(value) => Self::Value(context.clone_encoded_node(value)),
            Self::Number(value) => Self::Number(value),
            Self::Sum((left, right)) => Self::Sum((
                context.clone_encoded_node(left),
                context.clone_encoded_node(right),
            )),
            Self::Product((factor, value)) => {
                Self::Product((factor, context.clone_encoded_node(value)))
            }
            Self::Function(value) => Self::Function(context.clone_encoded_node(value)),
        }
    }
}

fn encode_calc<V: CalcValueCodec>(value: Calc<'_, V>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Calc::Value(value) => {
            bytes[0] = 0;
            write_node_id(&mut bytes, 4, value);
        }
        Calc::Number(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, value.to_bits());
        }
        Calc::Sum((left, right)) => {
            bytes[0] = 2;
            write_node_id(&mut bytes, 4, left);
            write_node_id(&mut bytes, 8, right);
        }
        Calc::Product((factor, value)) => {
            bytes[0] = 3;
            write_u32(&mut bytes, 4, factor.to_bits());
            write_node_id(&mut bytes, 8, value);
        }
        Calc::Function(value) => {
            bytes[0] = 4;
            write_node_id(&mut bytes, 4, value);
        }
    }
    NodePayload::inline(&bytes)
}

// byte 0       variant
// byte 1       rounding strategy when applicable
// bytes 2..4   reserved
// bytes 4..16  up to three child IDs or one range
impl<'ast, V: CalcValueCodec> AstNodeStorage<'ast> for MathFunction<'ast, V> {
    const KIND: NodeKind = V::MATH_FUNCTION_KIND;

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let first = || context.encoded_node_id_at(read_u32(&bytes, 4) as usize);
        let second = || context.encoded_node_id_at(read_u32(&bytes, 8) as usize);
        match bytes[0] {
            0 => Self::Calc(first()),
            1 => Self::Min(decode_range(&bytes, context)),
            2 => Self::Max(decode_range(&bytes, context)),
            3 => Self::Clamp((
                first(),
                second(),
                context.encoded_node_id_at(read_u32(&bytes, 12) as usize),
            )),
            4 => Self::Round((decode_rounding_strategy(bytes[1]), first(), second())),
            5 => Self::Rem((first(), second())),
            6 => Self::Mod((first(), second())),
            7 => Self::Abs(first()),
            8 => Self::Sign(first()),
            9 => Self::Hypot(decode_range(&bytes, context)),
            _ => panic!("invalid encoded MathFunction variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        encode_math_function(self)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast, V> AstNodeClone<'ast> for MathFunction<'ast, V>
where
    V: CalcValueCodec + AstNodeClone<'ast>,
{
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Calc(value) => Self::Calc(context.clone_encoded_node(value)),
            Self::Min(values) => Self::Min(context.clone_encoded_vec(values)),
            Self::Max(values) => Self::Max(context.clone_encoded_vec(values)),
            Self::Clamp((min, value, max)) => Self::Clamp((
                context.clone_encoded_node(min),
                context.clone_encoded_node(value),
                context.clone_encoded_node(max),
            )),
            Self::Round((strategy, value, interval)) => Self::Round((
                strategy,
                context.clone_encoded_node(value),
                context.clone_encoded_node(interval),
            )),
            Self::Rem((left, right)) => Self::Rem((
                context.clone_encoded_node(left),
                context.clone_encoded_node(right),
            )),
            Self::Mod((left, right)) => Self::Mod((
                context.clone_encoded_node(left),
                context.clone_encoded_node(right),
            )),
            Self::Abs(value) => Self::Abs(context.clone_encoded_node(value)),
            Self::Sign(value) => Self::Sign(context.clone_encoded_node(value)),
            Self::Hypot(values) => Self::Hypot(context.clone_encoded_vec(values)),
        }
    }
}

fn encode_math_function<V: CalcValueCodec>(value: MathFunction<'_, V>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        MathFunction::Calc(value) => write_tagged_node_id(&mut bytes, 0, value),
        MathFunction::Min(values) => write_tagged_range(&mut bytes, 1, values),
        MathFunction::Max(values) => write_tagged_range(&mut bytes, 2, values),
        MathFunction::Clamp((min, value, max)) => {
            bytes[0] = 3;
            write_node_id(&mut bytes, 4, min);
            write_node_id(&mut bytes, 8, value);
            write_node_id(&mut bytes, 12, max);
        }
        MathFunction::Round((strategy, value, interval)) => {
            bytes[0] = 4;
            bytes[1] = encode_rounding_strategy(strategy);
            write_node_id(&mut bytes, 4, value);
            write_node_id(&mut bytes, 8, interval);
        }
        MathFunction::Rem((left, right)) => {
            bytes[0] = 5;
            write_node_id(&mut bytes, 4, left);
            write_node_id(&mut bytes, 8, right);
        }
        MathFunction::Mod((left, right)) => {
            bytes[0] = 6;
            write_node_id(&mut bytes, 4, left);
            write_node_id(&mut bytes, 8, right);
        }
        MathFunction::Abs(value) => write_tagged_node_id(&mut bytes, 7, value),
        MathFunction::Sign(value) => write_tagged_node_id(&mut bytes, 8, value),
        MathFunction::Hypot(values) => write_tagged_range(&mut bytes, 9, values),
    }
    NodePayload::inline(&bytes)
}

fn write_tagged_node_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_node_id(bytes, 4, value);
}

fn write_node_id<T>(bytes: &mut [u8], offset: usize, value: NodeId<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_tagged_range<T>(bytes: &mut [u8], tag: u8, values: Vec<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(values.start_index()).expect("AST range exceeds four bytes"),
    );
    write_u32(
        bytes,
        8,
        u32::try_from(values.end_index()).expect("AST range exceeds four bytes"),
    );
}

fn decode_range<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(read_u32(bytes, 4) as usize, read_u32(bytes, 8) as usize)
}

fn encode_rounding_strategy(value: RoundingStrategy) -> u8 {
    match value {
        RoundingStrategy::Nearest => 0,
        RoundingStrategy::Up => 1,
        RoundingStrategy::Down => 2,
        RoundingStrategy::ToZero => 3,
    }
}

fn decode_rounding_strategy(value: u8) -> RoundingStrategy {
    match value {
        0 => RoundingStrategy::Nearest,
        1 => RoundingStrategy::Up,
        2 => RoundingStrategy::Down,
        3 => RoundingStrategy::ToZero,
        _ => panic!("invalid encoded RoundingStrategy"),
    }
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum RoundingStrategy {
    Nearest,
    Up,
    Down,
    ToZero,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Resolution {
    Dpi(f32),
    Dpcm(f32),
    Dppx(f32),
}

/// A CSS `<ratio>` value.
///
/// The optional denominator records whether the source wrote the `/ <number>` part, so
/// serialization reproduces the original expression instead of normalizing by
/// value.
#[derive(Debug, PartialEq, Visit)]
pub struct Ratio {
    pub denominator: Option<f32>,
    pub numerator: f32,
}

impl Ratio {
    /// `denominator` is `None` when the source omitted `/ <number>`.
    pub fn new(numerator: f32, denominator: Option<f32>) -> Self {
        Self {
            denominator,
            numerator,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum Angle {
    Deg(f32),
    Rad(f32),
    Grad(f32),
    Turn(f32),
}

// Fixed payload layout for `Angle`:
//
// byte 0      angle unit
// bytes 1..4  reserved
// bytes 4..8  f32 bits
// bytes 8..16 reserved
impl AstNodeStorage<'_> for Angle {
    const KIND: NodeKind = NodeKind::new(0x0001_0002);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        crate::token::decode_angle(
            bytes[0],
            f32::from_bits(u32::from_le_bytes(bytes[4..8].try_into().unwrap())),
        )
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_angle_node(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_angle_node(self)
    }
}

impl AstNodeClone<'_> for Angle {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_angle_node(value: Angle) -> NodePayload {
    let (kind, value) = crate::token::encode_angle(value);
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    bytes[0] = kind;
    bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum Time {
    Seconds(f32),
    Milliseconds(f32),
}

impl ExtraDataCompact<'_> for Time {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        let (kind, value) = crate::token::encode_time(self);
        let mut bytes = [0; ExtraData::BYTES];
        bytes[0] = kind;
        bytes[4..8].copy_from_slice(&value.to_bits().to_le_bytes());
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        let bytes = data.bytes();
        crate::token::decode_time(
            bytes[0],
            f32::from_bits(u32::from_le_bytes(bytes[4..8].try_into().unwrap())),
        )
    }
}

impl ExtraDataClone<'_> for Time {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[cfg(test)]
mod storage_tests {
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use rocketcss_common::Allocator;

    use crate::{AstContext, Calc, DUMMY_SP, Length, LengthUnit, LengthValue, MathFunction, Span};

    #[test]
    fn calc_and_math_function_codecs_deep_clone_recursive_nodes_and_ranges() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let length = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Rem,
                value: 2.0,
            }),
            DUMMY_SP,
        );
        let value = context.alloc_encoded_node(Calc::Value(length), DUMMY_SP);
        let number = context.alloc_encoded_node(Calc::<Length<'_>>::Number(3.0), DUMMY_SP);
        let values = context.alloc_encoded_vec([value, number].into_iter());
        let function = context.alloc_encoded_node(MathFunction::Min(values), DUMMY_SP);
        let calc = context.alloc_encoded_node(Calc::Function(function), DUMMY_SP);
        let root = context.alloc_encoded_node(Length::Calc(calc), DUMMY_SP);

        let cloned_root = context.clone_encoded_node(root);
        let Length::Calc(cloned_calc) = context.encoded_node(cloned_root) else {
            panic!("expected calc length")
        };
        assert_ne!(cloned_calc, calc);
        let Calc::Function(cloned_function) = context.encoded_node(cloned_calc) else {
            panic!("expected math function")
        };
        assert_ne!(cloned_function, function);
        let MathFunction::Min(cloned_values) = context.encoded_node(cloned_function) else {
            panic!("expected min function")
        };
        assert_ne!(cloned_values, values);
        assert_ne!(context.encoded_vec_get(cloned_values, 0), Some(value));
    }

    #[test]
    fn length_codec_round_trips_and_mutates_one_dense_identity() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Rem,
                value: 1.25,
            }),
            DUMMY_SP,
        );

        assert_eq!(
            context.encoded_node(id),
            Length::Value(LengthValue {
                unit: LengthUnit::Rem,
                value: 1.25,
            })
        );
        let span = Span::new(4, 9);
        context.set_encoded_node_span(id, span);
        assert_eq!(context.encoded_node_span(id), span);

        context.mutate_encoded_node(id, |length, _| {
            *length = Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 2.0,
            });
        });
        assert_eq!(
            context.encoded_node(id),
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 2.0,
            })
        );
    }

    #[test]
    fn length_codec_republishes_after_a_mutation_panic() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let id = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Em,
                value: 1.0,
            }),
            DUMMY_SP,
        );

        let result = catch_unwind(AssertUnwindSafe(|| {
            context.mutate_encoded_node(id, |length, _| {
                *length = Length::Value(LengthValue {
                    unit: LengthUnit::Vh,
                    value: 3.0,
                });
                panic!("stop after mutation");
            });
        }));
        assert!(result.is_err());
        assert_eq!(
            context.encoded_node(id),
            Length::Value(LengthValue {
                unit: LengthUnit::Vh,
                value: 3.0,
            })
        );
    }
}

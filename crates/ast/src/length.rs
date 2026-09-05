use super::*;

use crate::{AstNodeStorage, NodeKind, NodePayload};

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
    Min(Vec<'a, Calc<'a, V>>),
    Max(Vec<'a, Calc<'a, V>>),
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
    Hypot(Vec<'a, Calc<'a, V>>),
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

    use crate::{AstContext, DUMMY_SP, Length, LengthUnit, LengthValue, Span};

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

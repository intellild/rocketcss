use super::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Length<'a> {
    Value(LengthValue),
    Calc(NodeId<'a, Calc<'a, Length<'a>>>),
}

impl_inline_node!(Length<'ast>, 0x0001_0001);

impl<'ast> AstNodeClone<'ast> for Length<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Value(value) => Self::Value(value),
            Self::Calc(value) => Self::Calc(context.clone_encoded_node(value)),
        }
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
pub enum Calc<'a, V: CalcValueCodec + AstNodeStorage<'a>> {
    Value(NodeId<'a, V>),
    Number(f32),
    Sum((NodeId<'a, Calc<'a, V>>, NodeId<'a, Calc<'a, V>>)),
    Product((f32, NodeId<'a, Calc<'a, V>>)),
    Function(NodeId<'a, MathFunction<'a, V>>),
}

#[derive(Debug, PartialEq, Visit)]
#[allow(clippy::type_complexity)]
pub enum MathFunction<'a, V: CalcValueCodec + AstNodeStorage<'a>> {
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

#[doc(hidden)]
pub trait CalcValueCodec {
    const CALC_KIND: NodeKind;
    const MATH_FUNCTION_KIND: NodeKind;
}

impl CalcValueCodec for Length<'_> {
    const CALC_KIND: NodeKind = NodeKind::new(0x0018_0001);
    const MATH_FUNCTION_KIND: NodeKind = NodeKind::new(0x0019_0001);
}

impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> Copy for Calc<'ast, V> {}
impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> Clone for Calc<'ast, V> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: the supported value type selects a distinct kind and stores this native enum.
unsafe impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> AstNodeStorage<'ast> for Calc<'ast, V> {
    const KIND: NodeKind = V::CALC_KIND;
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
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

impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> Copy for MathFunction<'ast, V> {}
impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> Clone for MathFunction<'ast, V> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: each value type has a distinct math-function kind and stores the native enum.
unsafe impl<'ast, V: CalcValueCodec + AstNodeStorage<'ast>> AstNodeStorage<'ast>
    for MathFunction<'ast, V>
{
    const KIND: NodeKind = V::MATH_FUNCTION_KIND;
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
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

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum RoundingStrategy {
    Nearest,
    Up,
    Down,
    ToZero,
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
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
#[derive(Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Angle {
    Deg(f32),
    Rad(f32),
    Grad(f32),
    Turn(f32),
}

impl_inline_node!(Angle, 0x0001_0002);

impl AstNodeClone<'_> for Angle {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum Time {
    Seconds(f32),
    Milliseconds(f32),
}

unsafe impl ExtraDataCompact<'_> for Time {
    #[inline]
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    #[inline]
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
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
    fn native_math_functions_preserve_variants_and_rounding_strategies() {
        use crate::RoundingStrategy;
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let a = ast.alloc_node(Calc::<Length<'_>>::Number(1.0), DUMMY_SP);
        let b = ast.alloc_node(Calc::<Length<'_>>::Number(2.0), DUMMY_SP);
        let c = ast.alloc_node(Calc::<Length<'_>>::Number(3.0), DUMMY_SP);
        let values = ast.alloc_encoded_vec([c, a, b].into_iter());
        let function = ast.alloc_node(MathFunction::Calc(a), DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for expected in [
            MathFunction::Calc(a),
            MathFunction::Min(values),
            MathFunction::Max(values),
            MathFunction::Clamp((a, b, c)),
            MathFunction::Rem((a, b)),
            MathFunction::Mod((b, a)),
            MathFunction::Abs(c),
            MathFunction::Sign(a),
            MathFunction::Hypot(values),
        ] {
            ast.mutate_node(function, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(function), expected);
        }
        for strategy in [
            RoundingStrategy::Nearest,
            RoundingStrategy::Up,
            RoundingStrategy::Down,
            RoundingStrategy::ToZero,
        ] {
            let expected = MathFunction::Round((strategy, a, b));
            ast.mutate_node(function, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(function), expected);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

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

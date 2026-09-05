use super::*;

use rocketcss_common::vec::Vec;

#[derive(Debug, PartialEq, Visit)]
pub enum Length<'a> {
    Value(LengthValue),
    Calc(NodeId<'a, Calc<'a, Length<'a>>>),
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

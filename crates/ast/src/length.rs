use super::*;

#[derive(Debug, PartialEq, Visit)]
pub enum Length {
    Value(LengthValue),
    Calc(std::boxed::Box<Calc<Length>>),
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
pub enum Calc<V> {
    Value(std::boxed::Box<V>),
    Number(f32),
    Sum((std::boxed::Box<Calc<V>>, std::boxed::Box<Calc<V>>)),
    Product((f32, std::boxed::Box<Calc<V>>)),
    Function(std::boxed::Box<MathFunction<V>>),
}

#[derive(Debug, PartialEq, Visit)]
#[allow(clippy::type_complexity)]
pub enum MathFunction<V> {
    Calc(std::boxed::Box<Calc<V>>),
    Min(std::vec::Vec<Calc<V>>),
    Max(std::vec::Vec<Calc<V>>),
    Clamp(
        (
            std::boxed::Box<Calc<V>>,
            std::boxed::Box<Calc<V>>,
            std::boxed::Box<Calc<V>>,
        ),
    ),
    Round(
        (
            RoundingStrategy,
            std::boxed::Box<Calc<V>>,
            std::boxed::Box<Calc<V>>,
        ),
    ),
    Rem((std::boxed::Box<Calc<V>>, std::boxed::Box<Calc<V>>)),
    Mod((std::boxed::Box<Calc<V>>, std::boxed::Box<Calc<V>>)),
    Abs(std::boxed::Box<Calc<V>>),
    Sign(std::boxed::Box<Calc<V>>),
    Hypot(std::vec::Vec<Calc<V>>),
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
/// The variants record whether the source wrote the `/ <number>` part, so
/// serialization reproduces the original expression instead of normalizing by
/// value.
#[derive(Debug, PartialEq, Visit)]
pub enum Ratio {
    /// `<number>` with the denominator omitted.
    Number(f32),
    /// `<number> / <number>` with an explicit denominator.
    Fraction(f32, f32),
}

impl Ratio {
    /// `denominator` is `None` when the source omitted `/ <number>`.
    pub fn new(numerator: f32, denominator: Option<f32>) -> Self {
        match denominator {
            Some(denominator) => Self::Fraction(numerator, denominator),
            None => Self::Number(numerator),
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

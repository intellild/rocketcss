use super::*;

use rs_css_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum Length<'a> {
    Value(Box<'a, LengthValue>),
    Calc(Box<'a, Calc<'a, Length<'a>>>),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Calc<'a, V> {
    Value(Box<'a, V>),
    Number(f32),
    Sum((Box<'a, Calc<'a, V>>, Box<'a, Calc<'a, V>>)),
    Product((f32, Box<'a, Calc<'a, V>>)),
    Function(Box<'a, MathFunction<'a, V>>),
}

#[derive(Debug, PartialEq)]
#[allow(clippy::type_complexity)]
pub enum MathFunction<'a, V> {
    Calc(Box<'a, Calc<'a, V>>),
    Min(Vec<'a, Calc<'a, V>>),
    Max(Vec<'a, Calc<'a, V>>),
    Clamp(
        (
            Box<'a, Calc<'a, V>>,
            Box<'a, Calc<'a, V>>,
            Box<'a, Calc<'a, V>>,
        ),
    ),
    Round((RoundingStrategy, Box<'a, Calc<'a, V>>, Box<'a, Calc<'a, V>>)),
    Rem((Box<'a, Calc<'a, V>>, Box<'a, Calc<'a, V>>)),
    Mod((Box<'a, Calc<'a, V>>, Box<'a, Calc<'a, V>>)),
    Abs(Box<'a, Calc<'a, V>>),
    Sign(Box<'a, Calc<'a, V>>),
    Hypot(Vec<'a, Calc<'a, V>>),
}

#[derive(Debug, PartialEq)]
pub enum RoundingStrategy {
    Nearest,
    Up,
    Down,
    ToZero,
}

#[derive(Debug, PartialEq)]
pub enum Resolution {
    Dpi(f32),
    Dpcm(f32),
    Dppx(f32),
}

#[derive(Debug, PartialEq)]
pub struct Ratio(pub f32, pub f32);

#[derive(Debug, PartialEq)]
pub enum Angle {
    Deg(f32),
    Rad(f32),
    Grad(f32),
    Turn(f32),
}

#[derive(Debug, PartialEq)]
pub enum Time {
    Seconds(f32),
    Milliseconds(f32),
}

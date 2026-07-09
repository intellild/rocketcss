use super::*;

use rs_css_allocator::{boxed::Box, vec::Vec};

#[derive(Debug, PartialEq)]
pub enum Length<'a> {
    Value(Box<'a, LengthValue<'a>>),
    Calc(Box<'a, CalcFor_Length<'a>>),
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
pub enum CalcFor_Length<'a> {
    Value(Box<'a, Length<'a>>),
    Number(f64),
    Sum((Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>)),
    Product((f64, Box<'a, CalcFor_Length<'a>>)),
    Function(Box<'a, MathFunctionFor_Length<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum MathFunctionFor_Length<'a> {
    Calc(Box<'a, CalcFor_Length<'a>>),
    Min(Vec<'a, Box<'a, CalcFor_Length<'a>>>),
    Max(Vec<'a, Box<'a, CalcFor_Length<'a>>>),
    Clamp(
        (
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
        ),
    ),
    Round(
        (
            Box<'a, RoundingStrategy>,
            Box<'a, CalcFor_Length<'a>>,
            Box<'a, CalcFor_Length<'a>>,
        ),
    ),
    Rem((Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>)),
    Mod((Box<'a, CalcFor_Length<'a>>, Box<'a, CalcFor_Length<'a>>)),
    Abs(Box<'a, CalcFor_Length<'a>>),
    Sign(Box<'a, CalcFor_Length<'a>>),
    Hypot(Vec<'a, Box<'a, CalcFor_Length<'a>>>),
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
    Dpi(f64),
    Dpcm(f64),
    Dppx(f64),
}

#[derive(Debug, PartialEq)]
pub struct Ratio(pub f64, pub f64);

#[derive(Debug, PartialEq)]
pub enum Angle {
    Deg(f64),
    Rad(f64),
    Grad(f64),
    Turn(f64),
}

#[derive(Debug, PartialEq)]
pub enum Time {
    Seconds(f64),
    Milliseconds(f64),
}

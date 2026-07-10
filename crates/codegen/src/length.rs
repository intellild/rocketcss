use crate::prelude::*;

impl ToCss for LengthValue {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.value == 0.0 && !dest.in_calc() {
            return dest.write_char('0');
        }
        serialize_dimension(self.value, length_unit_str(&self.unit), dest)
    }
}

fn length_unit_str(unit: &LengthUnit) -> &'static str {
    match unit {
        LengthUnit::Px => "px",
        LengthUnit::In => "in",
        LengthUnit::Cm => "cm",
        LengthUnit::Mm => "mm",
        LengthUnit::Q => "q",
        LengthUnit::Pt => "pt",
        LengthUnit::Pc => "pc",
        LengthUnit::Em => "em",
        LengthUnit::Rem => "rem",
        LengthUnit::Ex => "ex",
        LengthUnit::Rex => "rex",
        LengthUnit::Ch => "ch",
        LengthUnit::Rch => "rch",
        LengthUnit::Cap => "cap",
        LengthUnit::Rcap => "rcap",
        LengthUnit::Ic => "ic",
        LengthUnit::Ric => "ric",
        LengthUnit::Lh => "lh",
        LengthUnit::Rlh => "rlh",
        LengthUnit::Vw => "vw",
        LengthUnit::Lvw => "lvw",
        LengthUnit::Svw => "svw",
        LengthUnit::Dvw => "dvw",
        LengthUnit::Cqw => "cqw",
        LengthUnit::Vh => "vh",
        LengthUnit::Lvh => "lvh",
        LengthUnit::Svh => "svh",
        LengthUnit::Dvh => "dvh",
        LengthUnit::Cqh => "cqh",
        LengthUnit::Vi => "vi",
        LengthUnit::Svi => "svi",
        LengthUnit::Lvi => "lvi",
        LengthUnit::Dvi => "dvi",
        LengthUnit::Cqi => "cqi",
        LengthUnit::Vb => "vb",
        LengthUnit::Svb => "svb",
        LengthUnit::Lvb => "lvb",
        LengthUnit::Dvb => "dvb",
        LengthUnit::Cqb => "cqb",
        LengthUnit::Vmin => "vmin",
        LengthUnit::Svmin => "svmin",
        LengthUnit::Lvmin => "lvmin",
        LengthUnit::Dvmin => "dvmin",
        LengthUnit::Cqmin => "cqmin",
        LengthUnit::Vmax => "vmax",
        LengthUnit::Svmax => "svmax",
        LengthUnit::Lvmax => "lvmax",
        LengthUnit::Dvmax => "dvmax",
        LengthUnit::Cqmax => "cqmax",
    }
}

impl ToCss for LengthUnit {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(length_unit_str(self))
    }
}

impl ToCss for Length<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Value(value) => value.to_css(dest),
            Self::Calc(calc) => calc.to_css(dest),
        }
    }
}

impl<V: ToCss> ToCss for Calc<'_, V> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.with_calc(|dest| match self {
            Self::Value(value) => value.to_css(dest),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Sum((left, right)) => {
                left.to_css(dest)?;
                dest.write_str(" + ")?;
                right.to_css(dest)
            }
            Self::Product((factor, value)) => {
                serialize_number(*factor, dest)?;
                dest.write_str(" * ")?;
                value.to_css(dest)
            }
            Self::Function(function) => function.to_css(dest),
        })
    }
}

fn write_function<PrinterT: PrinterTrait, F>(
    name: &str,
    dest: &mut PrinterT,
    callback: F,
) -> fmt::Result
where
    F: FnOnce(&mut PrinterT) -> fmt::Result,
{
    dest.write_str(name)?;
    dest.write_char('(')?;
    callback(dest)?;
    dest.write_char(')')
}

fn write_calc_list<PrinterT: PrinterTrait, V: ToCss>(
    values: &[Calc<'_, V>],
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(',', false)?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

impl<V: ToCss> ToCss for MathFunction<'_, V> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Calc(value) => write_function("calc", dest, |dest| value.to_css(dest)),
            Self::Min(values) => write_function("min", dest, |dest| write_calc_list(values, dest)),
            Self::Max(values) => write_function("max", dest, |dest| write_calc_list(values, dest)),
            Self::Clamp((min, value, max)) => write_function("clamp", dest, |dest| {
                min.to_css(dest)?;
                dest.delim(',', false)?;
                value.to_css(dest)?;
                dest.delim(',', false)?;
                max.to_css(dest)
            }),
            Self::Round((strategy, value, interval)) => write_function("round", dest, |dest| {
                if !matches!(strategy, RoundingStrategy::Nearest) {
                    strategy.to_css(dest)?;
                    dest.delim(',', false)?;
                }
                value.to_css(dest)?;
                dest.delim(',', false)?;
                interval.to_css(dest)
            }),
            Self::Rem((left, right)) => write_function("rem", dest, |dest| {
                left.to_css(dest)?;
                dest.delim(',', false)?;
                right.to_css(dest)
            }),
            Self::Mod((left, right)) => write_function("mod", dest, |dest| {
                left.to_css(dest)?;
                dest.delim(',', false)?;
                right.to_css(dest)
            }),
            Self::Abs(value) => write_function("abs", dest, |dest| value.to_css(dest)),
            Self::Sign(value) => write_function("sign", dest, |dest| value.to_css(dest)),
            Self::Hypot(values) => {
                write_function("hypot", dest, |dest| write_calc_list(values, dest))
            }
        }
    }
}

impl ToCss for RoundingStrategy {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::Nearest => "nearest",
            Self::Up => "up",
            Self::Down => "down",
            Self::ToZero => "to-zero",
        })
    }
}

impl ToCss for Resolution {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let (value, unit) = match self {
            Self::Dpi(value) => (*value, "dpi"),
            Self::Dpcm(value) => (*value, "dpcm"),
            Self::Dppx(value) => (*value, "dppx"),
        };
        serialize_dimension(value, unit, dest)
    }
}

impl ToCss for Ratio {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_number(self.0, dest)?;
        if self.1 != 1.0 {
            dest.write_char('/')?;
            serialize_number(self.1, dest)?;
        }
        Ok(())
    }
}

impl ToCss for Angle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let (value, unit) = match self {
            Self::Deg(value) => (*value, "deg"),
            Self::Rad(value) => (*value, "rad"),
            Self::Grad(value) => (*value, "grad"),
            Self::Turn(value) => (*value, "turn"),
        };
        serialize_dimension(value, unit, dest)
    }
}

impl ToCss for Time {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Seconds(value) => serialize_dimension(*value, "s", dest),
            Self::Milliseconds(value) => {
                let mut milliseconds = String::new();
                serialize_dimension(
                    *value,
                    "ms",
                    &mut Printer::new(&mut milliseconds, dest.options()),
                )?;
                let mut seconds = String::new();
                serialize_dimension(
                    *value / 1000.0,
                    "s",
                    &mut Printer::new(&mut seconds, dest.options()),
                )?;
                dest.write_str(if seconds.len() < milliseconds.len() {
                    &seconds
                } else {
                    &milliseconds
                })
            }
        }
    }
}

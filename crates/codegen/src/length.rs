use crate::prelude::*;

impl ToCss for LengthValue {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if self.value == 0.0 && !dest.in_calc() {
            return dest.write_char('0');
        }
        serialize_dimension(self.value, &self.unit, dest)
    }
}

impl ToCss for LengthUnit {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("length units are static strings"))
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
            dest.delim(Delimiter::Comma)?;
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
                dest.delim(Delimiter::Comma)?;
                value.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest)
            }),
            Self::Round((strategy, value, interval)) => write_function("round", dest, |dest| {
                if !matches!(strategy, RoundingStrategy::Nearest) {
                    strategy.to_css(dest)?;
                    dest.delim(Delimiter::Comma)?;
                }
                value.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                interval.to_css(dest)
            }),
            Self::Rem((left, right)) => write_function("rem", dest, |dest| {
                left.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
                right.to_css(dest)
            }),
            Self::Mod((left, right)) => write_function("mod", dest, |dest| {
                left.to_css(dest)?;
                dest.delim(Delimiter::Comma)?;
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
        dest.write_str(
            self.as_css_str()
                .expect("rounding strategies are static keywords"),
        )
    }
}

impl ToCss for Resolution {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let (value, unit) = match self {
            Self::Dpi(value) => (*value, Unit::Dpi),
            Self::Dpcm(value) => (*value, Unit::Dpcm),
            Self::Dppx(value) => (*value, Unit::Dppx),
        };
        serialize_dimension(value, &unit, dest)
    }
}

impl ToCss for Ratio {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_number(self.numerator, dest)?;
        if self.explicit_denominator {
            dest.write_char('/')?;
            serialize_number(self.denominator, dest)?;
        }
        Ok(())
    }
}

impl ToCss for Angle {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let (value, unit) = match self {
            Self::Deg(value) => (*value, Unit::Deg),
            Self::Rad(value) => (*value, Unit::Rad),
            Self::Grad(value) => (*value, Unit::Grad),
            Self::Turn(value) => (*value, Unit::Turn),
        };
        serialize_dimension(value, &unit, dest)
    }
}

impl ToCss for Time {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Seconds(value) => serialize_dimension(*value, &Unit::Seconds, dest),
            Self::Milliseconds(value) => {
                let mut milliseconds = String::new();
                serialize_dimension(
                    *value,
                    &Unit::Milliseconds,
                    &mut Printer::new(&mut milliseconds, dest.options()),
                )?;
                let mut seconds = String::new();
                serialize_dimension(
                    *value / 1000.0,
                    &Unit::Seconds,
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

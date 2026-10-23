use crate::prelude::*;

impl<'ast, 'ghost> ToCss<'ghost> for LengthValue<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if data.value == 0.0 && !dest.in_calc() {
            return dest.write_char('0');
        }
        serialize_dimension(data.value, &data.unit, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for LengthUnit {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("length units are static strings"))
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Length<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            LengthData::Value(value) => value.to_css(dest, _cx),
            LengthData::Calc(calc) => calc.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost, V: ToCss<'ghost> + Copy> ToCss<'ghost> for Calc<'ast, V> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.with_calc(|dest| match self.data(_cx.ast()) {
            CalcData::Value(value) => value.to_css(dest, _cx),
            CalcData::Number(value) => serialize_number(value, dest),
            CalcData::Sum((left, right)) => {
                left.to_css(dest, _cx)?;
                dest.write_str(" + ")?;
                right.to_css(dest, _cx)
            }
            CalcData::Product((factor, value)) => {
                serialize_number(factor, dest)?;
                dest.write_str(" * ")?;
                value.to_css(dest, _cx)
            }
            CalcData::Function(function) => function.to_css(dest, _cx),
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

fn write_calc_list<'ast, 'ghost, PrinterT: PrinterTrait, V: ToCss<'ghost> + Copy>(
    values: &[Calc<'ast, V>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost, V: ToCss<'ghost> + Copy> ToCss<'ghost> for MathFunction<'ast, V> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            MathFunctionData::Calc(value) => {
                write_function("calc", dest, |dest| value.to_css(dest, _cx))
            }
            MathFunctionData::Min(values) => {
                write_function("min", dest, |dest| write_calc_list(values, dest, _cx))
            }
            MathFunctionData::Max(values) => {
                write_function("max", dest, |dest| write_calc_list(values, dest, _cx))
            }
            MathFunctionData::Clamp((min, value, max)) => write_function("clamp", dest, |dest| {
                min.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                value.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest, _cx)
            }),
            MathFunctionData::Round((strategy, value, interval)) => {
                write_function("round", dest, |dest| {
                    if !matches!(strategy, RoundingStrategy::Nearest) {
                        strategy.to_css(dest, _cx)?;
                        dest.delim(Delimiter::Comma)?;
                    }
                    value.to_css(dest, _cx)?;
                    dest.delim(Delimiter::Comma)?;
                    interval.to_css(dest, _cx)
                })
            }
            MathFunctionData::Rem((left, right)) => write_function("rem", dest, |dest| {
                left.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                right.to_css(dest, _cx)
            }),
            MathFunctionData::Mod((left, right)) => write_function("mod", dest, |dest| {
                left.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                right.to_css(dest, _cx)
            }),
            MathFunctionData::Abs(value) => {
                write_function("abs", dest, |dest| value.to_css(dest, _cx))
            }
            MathFunctionData::Sign(value) => {
                write_function("sign", dest, |dest| value.to_css(dest, _cx))
            }
            MathFunctionData::Hypot(values) => {
                write_function("hypot", dest, |dest| write_calc_list(values, dest, _cx))
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for RoundingStrategy {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("rounding strategies are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for Resolution {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let (value, unit) = match self {
            Self::Dpi(value) => (*value, "dpi"),
            Self::Dpcm(value) => (*value, "dpcm"),
            Self::Dppx(value) => (*value, "dppx"),
        };
        serialize_number(value, dest)?;
        dest.write_str(unit)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Ratio<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        serialize_number(data.numerator, dest)?;
        if let Some(denominator) = data.denominator {
            dest.write_char('/')?;
            serialize_number(denominator, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for Angle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let (value, unit) = match self {
            Self::Deg(value) => (*value, "deg"),
            Self::Rad(value) => (*value, "rad"),
            Self::Grad(value) => (*value, "grad"),
            Self::Turn(value) => (*value, "turn"),
        };
        serialize_number(value, dest)?;
        dest.write_str(unit)
    }
}

impl<'ghost> ToCss<'ghost> for Time {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Seconds(value) => {
                serialize_number(*value, dest)?;
                dest.write_char('s')
            }
            Self::Milliseconds(value) => {
                let mut milliseconds = String::new();
                serialize_number(*value, &mut Printer::new(&mut milliseconds, dest.options()))?;
                milliseconds.push_str("ms");
                let mut seconds = String::new();
                serialize_number(
                    *value / 1000.0,
                    &mut Printer::new(&mut seconds, dest.options()),
                )?;
                seconds.push('s');
                dest.write_str(if seconds.len() < milliseconds.len() {
                    &seconds
                } else {
                    &milliseconds
                })
            }
        }
    }
}

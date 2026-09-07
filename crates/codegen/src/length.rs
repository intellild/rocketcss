use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for LengthUnit {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("length units are static strings"))
    }
}

impl<'ghost> ToCss<'ghost> for Length<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Value(value) => value.to_css(dest, _cx),
            Self::Calc(calc) => calc.to_css(dest, _cx),
        }
    }
}

impl<'ast, 'ghost, V> ToCss<'ghost> for Calc<'ast, V>
where
    V: ToCss<'ghost> + CalcValueCodec + AstNodeStorage<'ast> + 'ast,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.with_calc(|dest| match self {
            Self::Value(value) => value.to_css(dest, _cx),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Sum((left, right)) => {
                left.to_css(dest, _cx)?;
                dest.write_str(" + ")?;
                right.to_css(dest, _cx)
            }
            Self::Product((factor, value)) => {
                serialize_number(*factor, dest)?;
                dest.write_str(" * ")?;
                value.to_css(dest, _cx)
            }
            Self::Function(function) => function.to_css(dest, _cx),
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

fn write_calc_list<'ast, 'ghost, PrinterT, V, I>(
    values: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    V: ToCss<'ghost> + CalcValueCodec + AstNodeStorage<'ast> + 'ast,
    I: IntoIterator<Item = NodeId<'ast, Calc<'ast, V>>>,
{
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ast, 'ghost, V> ToCss<'ghost> for MathFunction<'ast, V>
where
    V: ToCss<'ghost> + CalcValueCodec + AstNodeStorage<'ast>,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Calc(value) => write_function("calc", dest, |dest| value.to_css(dest, _cx)),
            Self::Min(values) => write_function("min", dest, |dest| {
                write_calc_list(_cx.ast_context().vec_iter(*values), dest, _cx)
            }),
            Self::Max(values) => write_function("max", dest, |dest| {
                write_calc_list(_cx.ast_context().vec_iter(*values), dest, _cx)
            }),
            Self::Clamp((min, value, max)) => write_function("clamp", dest, |dest| {
                min.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                value.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                max.to_css(dest, _cx)
            }),
            Self::Round((strategy, value, interval)) => write_function("round", dest, |dest| {
                if !matches!(strategy, RoundingStrategy::Nearest) {
                    strategy.to_css(dest, _cx)?;
                    dest.delim(Delimiter::Comma)?;
                }
                value.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                interval.to_css(dest, _cx)
            }),
            Self::Rem((left, right)) => write_function("rem", dest, |dest| {
                left.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                right.to_css(dest, _cx)
            }),
            Self::Mod((left, right)) => write_function("mod", dest, |dest| {
                left.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                right.to_css(dest, _cx)
            }),
            Self::Abs(value) => write_function("abs", dest, |dest| value.to_css(dest, _cx)),
            Self::Sign(value) => write_function("sign", dest, |dest| value.to_css(dest, _cx)),
            Self::Hypot(values) => write_function("hypot", dest, |dest| {
                write_calc_list(_cx.ast_context().vec_iter(*values), dest, _cx)
            }),
        }
    }
}

impl<'ghost> ToCss<'ghost> for RoundingStrategy {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let (value, unit) = match self {
            Self::Dpi(value) => (*value, Unit::Dpi),
            Self::Dpcm(value) => (*value, Unit::Dpcm),
            Self::Dppx(value) => (*value, Unit::Dppx),
        };
        serialize_dimension(value, &unit, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Ratio {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_number(self.numerator, dest)?;
        if let Some(denominator) = self.denominator {
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let (value, unit) = match self {
            Self::Deg(value) => (*value, Unit::Deg),
            Self::Rad(value) => (*value, Unit::Rad),
            Self::Grad(value) => (*value, Unit::Grad),
            Self::Turn(value) => (*value, Unit::Turn),
        };
        serialize_dimension(value, &unit, dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Time {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Seconds(value) => serialize_dimension(*value, &Unit::Seconds, dest, _cx),
            Self::Milliseconds(value) => {
                let mut milliseconds = String::new();
                serialize_dimension(
                    *value,
                    &Unit::Milliseconds,
                    &mut Printer::new(&mut milliseconds, dest.options()),
                    _cx,
                )?;
                let mut seconds = String::new();
                serialize_dimension(
                    *value / 1000.0,
                    &Unit::Seconds,
                    &mut Printer::new(&mut seconds, dest.options()),
                    _cx,
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

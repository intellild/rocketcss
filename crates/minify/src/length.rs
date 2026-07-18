use rocketcss_ast::{Angle, LengthUnit, LengthValue, Ratio, Resolution, Time, Unit};

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for LengthValue {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if self.unit == LengthUnit::Px
            && let Some(precision) = cx.options().length_precision
        {
            let factor = 10_f32.powi(i32::from(precision));
            let rounded = (self.value * factor).round() / factor;
            if self.value != rounded {
                self.value = rounded;
                cx.record_value_normalized();
            }
            return;
        }
        if !cx.is_enabled(
            Options::NORMALIZE_VALUES | Options::CONVERT_LENGTH_UNITS,
            OptionsOp::And,
        ) || self.value == 0.0
        {
            return;
        }
        let Some(px) = to_px(self.value, self.unit) else {
            return;
        };
        let original_unit = self.unit;

        let candidates = [
            (px, LengthUnit::Px),
            (px / 96.0, LengthUnit::In),
            (px * 72.0 / 96.0, LengthUnit::Pt),
            (px / 16.0, LengthUnit::Pc),
            (px * 2.54 / 96.0, LengthUnit::Cm),
            (px * 25.4 / 96.0, LengthUnit::Mm),
            (px * 101.6 / 96.0, LengthUnit::Q),
        ];
        let original_len = dimension_len(self.value, Unit::Length(self.unit));
        if original_len <= 2 {
            return;
        }

        let mut best = None;
        let mut best_len = original_len;
        for (candidate_value, candidate_unit) in candidates {
            if !candidate_value.is_finite()
                || candidate_unit == original_unit
                || (cx.is_enabled(Options::CONVERT_EXTENDED_LENGTH_UNITS, OptionsOp::None)
                    && matches!(
                        candidate_unit,
                        LengthUnit::Cm | LengthUnit::Mm | LengthUnit::Q
                    ))
                || 1 + length_unit_len(candidate_unit) >= best_len
            {
                continue;
            }

            let candidate_value = clean_float(candidate_value);
            let candidate_len = dimension_len(candidate_value, Unit::Length(candidate_unit));
            if candidate_len < best_len {
                best = Some((candidate_value, candidate_unit));
                best_len = candidate_len;
            }
        }

        if let Some((candidate_value, candidate_unit)) = best {
            self.value = candidate_value;
            self.unit = candidate_unit;
            cx.record_value_normalized();
        }
    }
}

impl Minify for Angle {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }

        let degrees = match *self {
            Angle::Deg(number) => number,
            Angle::Rad(number) => number.to_degrees(),
            Angle::Grad(number) => number * 0.9,
            Angle::Turn(number) => number * 360.0,
        };
        if degrees == 0.0 {
            return;
        }
        let candidates = [
            Angle::Deg(clean_float(degrees)),
            Angle::Turn(clean_float(degrees / 360.0)),
            Angle::Grad(clean_float(degrees / 0.9)),
            Angle::Rad(clean_float(degrees.to_radians())),
        ];
        let original_len = dimension_len(angle_number(self), angle_unit(self));
        let original_unit = angle_unit(self);
        let mut best = None;
        let mut best_len = original_len;
        for candidate in candidates {
            let candidate_unit = angle_unit(&candidate);
            if candidate_unit == original_unit || 1 + unit_len(candidate_unit) >= best_len {
                continue;
            }

            let candidate_len = dimension_len(angle_number(&candidate), candidate_unit);
            if candidate_len < best_len {
                best = Some(candidate);
                best_len = candidate_len;
            }
        }

        if let Some(candidate) = best {
            *self = candidate;
            cx.record_value_normalized();
        }
    }
}

impl Minify for Time {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }

        let milliseconds = match *self {
            Time::Seconds(number) => number * 1000.0,
            Time::Milliseconds(number) => number,
        };
        let seconds = clean_float(milliseconds / 1000.0);
        let milliseconds = clean_float(milliseconds);
        let seconds_len = dimension_len(seconds, Unit::Seconds);
        let milliseconds_len = dimension_len(milliseconds, Unit::Milliseconds);
        let candidate = if seconds_len < milliseconds_len {
            Time::Seconds(seconds)
        } else {
            Time::Milliseconds(milliseconds)
        };
        if *self != candidate {
            *self = candidate;
            cx.record_value_normalized();
        }
    }
}

impl Minify for Resolution {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }

        let dpi = match *self {
            Resolution::Dpi(number) => number,
            Resolution::Dpcm(number) => number * 2.54,
            Resolution::Dppx(number) => number * 96.0,
        };
        let candidates = [
            Resolution::Dpi(clean_float(dpi)),
            Resolution::Dpcm(clean_float(dpi / 2.54)),
            Resolution::Dppx(clean_float(dpi / 96.0)),
        ];
        let candidate = candidates
            .into_iter()
            .min_by_key(resolution_len)
            .expect("resolution candidates are non-empty");
        if *self != candidate {
            *self = candidate;
            cx.record_value_normalized();
        }
    }
}

impl Minify for Ratio {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any) {
            reduce_ratio(self, cx);
        }
        if self.explicit_denominator
            && self.denominator == 1.0
            && cx.is_enabled(Options::CONVERT_RATIOS, OptionsOp::And)
        {
            self.explicit_denominator = false;
            cx.record_value_normalized();
        }
    }
}

fn reduce_ratio(ratio: &mut Ratio, cx: &mut MinifyContext) {
    if ratio.numerator <= 0.0 || ratio.denominator <= 0.0 {
        return;
    }
    let mut scale = 1_u64;
    while scale < 1_000_000
        && (!is_near_integer(ratio.numerator * scale as f32)
            || !is_near_integer(ratio.denominator * scale as f32))
    {
        scale *= 10;
    }
    let left = (ratio.numerator * scale as f32).round() as u64;
    let right = (ratio.denominator * scale as f32).round() as u64;
    let divisor = gcd(left, right);
    if divisor == 0 {
        return;
    }
    let reduced_left = (left / divisor) as f32;
    let reduced_right = (right / divisor) as f32;
    if reduced_left == ratio.numerator && reduced_right == ratio.denominator {
        return;
    }
    ratio.numerator = reduced_left;
    ratio.denominator = reduced_right;
    cx.record_value_normalized();
}

pub(crate) fn minify_dimension(
    value: f32,
    unit: Unit,
    cx: &mut MinifyContext,
) -> Option<(f32, Unit)> {
    if let Unit::Length(length_unit) = unit
        && to_px(value, length_unit).is_some()
    {
        let mut length = LengthValue {
            value,
            unit: length_unit,
        };
        length.minify(cx);
        return Some((length.value, Unit::Length(length.unit)));
    }
    if matches!(unit, Unit::Milliseconds | Unit::Seconds) {
        let mut time = match unit {
            Unit::Milliseconds => Time::Milliseconds(value),
            Unit::Seconds => Time::Seconds(value),
            _ => unreachable!("time units were checked above"),
        };
        time.minify(cx);
        return Some(match time {
            Time::Seconds(number) => (number, Unit::Seconds),
            Time::Milliseconds(number) => (number, Unit::Milliseconds),
        });
    }
    if matches!(unit, Unit::Deg | Unit::Rad | Unit::Grad | Unit::Turn) {
        let mut angle = match unit {
            Unit::Deg => Angle::Deg(value),
            Unit::Rad => Angle::Rad(value),
            Unit::Grad => Angle::Grad(value),
            Unit::Turn => Angle::Turn(value),
            _ => unreachable!("angle units were checked above"),
        };
        angle.minify(cx);
        return Some(match angle {
            Angle::Deg(number) => (number, Unit::Deg),
            Angle::Rad(number) => (number, Unit::Rad),
            Angle::Grad(number) => (number, Unit::Grad),
            Angle::Turn(number) => (number, Unit::Turn),
        });
    }
    None
}

fn to_px(value: f32, unit: LengthUnit) -> Option<f32> {
    Some(match unit {
        LengthUnit::Px => value,
        LengthUnit::In => value * 96.0,
        LengthUnit::Cm => value * 96.0 / 2.54,
        LengthUnit::Mm => value * 96.0 / 25.4,
        LengthUnit::Q => value * 96.0 / 101.6,
        LengthUnit::Pt => value * 96.0 / 72.0,
        LengthUnit::Pc => value * 16.0,
        _ => return None,
    })
}

fn angle_number(value: &Angle) -> f32 {
    match value {
        Angle::Deg(number) | Angle::Rad(number) | Angle::Grad(number) | Angle::Turn(number) => {
            *number
        }
    }
}

fn angle_unit(value: &Angle) -> Unit {
    match value {
        Angle::Deg(_) => Unit::Deg,
        Angle::Rad(_) => Unit::Rad,
        Angle::Grad(_) => Unit::Grad,
        Angle::Turn(_) => Unit::Turn,
    }
}

fn resolution_len(value: &Resolution) -> usize {
    match value {
        Resolution::Dpi(number) => dimension_len(*number, Unit::Dpi),
        Resolution::Dpcm(number) => dimension_len(*number, Unit::Dpcm),
        Resolution::Dppx(number) => dimension_len(*number, Unit::Dppx),
    }
}

fn dimension_len(value: f32, unit: Unit) -> usize {
    number_len(value) + unit_len(unit)
}

const fn unit_len(unit: Unit) -> usize {
    match unit {
        Unit::Length(unit) => length_unit_len(unit),
        Unit::Seconds | Unit::ResolutionX => 1,
        Unit::Milliseconds | Unit::Hertz | Unit::Flex => 2,
        Unit::Deg | Unit::Rad | Unit::Kilohertz | Unit::Dpi => 3,
        Unit::Grad | Unit::Turn | Unit::Dpcm | Unit::Dppx => 4,
    }
}

const fn length_unit_len(unit: LengthUnit) -> usize {
    match unit {
        LengthUnit::Q => 1,
        LengthUnit::Px
        | LengthUnit::In
        | LengthUnit::Cm
        | LengthUnit::Mm
        | LengthUnit::Pt
        | LengthUnit::Pc
        | LengthUnit::Em
        | LengthUnit::Ex
        | LengthUnit::Ch
        | LengthUnit::Ic
        | LengthUnit::Lh
        | LengthUnit::Vw
        | LengthUnit::Vh
        | LengthUnit::Vi
        | LengthUnit::Vb => 2,
        LengthUnit::Rem
        | LengthUnit::Rex
        | LengthUnit::Rch
        | LengthUnit::Cap
        | LengthUnit::Ric
        | LengthUnit::Rlh
        | LengthUnit::Lvw
        | LengthUnit::Svw
        | LengthUnit::Dvw
        | LengthUnit::Cqw
        | LengthUnit::Lvh
        | LengthUnit::Svh
        | LengthUnit::Dvh
        | LengthUnit::Cqh
        | LengthUnit::Svi
        | LengthUnit::Lvi
        | LengthUnit::Dvi
        | LengthUnit::Cqi
        | LengthUnit::Svb
        | LengthUnit::Lvb
        | LengthUnit::Dvb
        | LengthUnit::Cqb => 3,
        LengthUnit::Rcap | LengthUnit::Vmin | LengthUnit::Vmax => 4,
        LengthUnit::Svmin
        | LengthUnit::Lvmin
        | LengthUnit::Dvmin
        | LengthUnit::Cqmin
        | LengthUnit::Svmax
        | LengthUnit::Lvmax
        | LengthUnit::Dvmax
        | LengthUnit::Cqmax => 5,
    }
}

fn number_len(value: f32) -> usize {
    let value = clean_float(value);
    let mut buffer = zmij::Buffer::new();
    let output = buffer.format(value);
    let output = output.strip_suffix(".0").unwrap_or(output);
    if value != 0.0 && value.abs() < 1.0 {
        output.len().saturating_sub(1)
    } else {
        output.len()
    }
}

fn clean_float(value: f32) -> f32 {
    let rounded = value.round();
    if (value - rounded).abs() <= f32::EPSILON * value.abs().max(1.0) * 2.0 {
        rounded
    } else {
        (value * 1_000_000.0).round() / 1_000_000.0
    }
}

fn is_near_integer(value: f32) -> bool {
    (value - value.round()).abs() <= f32::EPSILON * value.abs().max(1.0)
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

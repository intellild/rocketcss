use rs_css_ast::{Angle, LengthUnit, LengthValue, Resolution, Time, Unit};

pub(crate) fn minify_length(value: &mut LengthValue) -> bool {
    if value.value == 0.0 {
        return false;
    }
    let original_value = value.value;
    let Some(px) = to_px(value.value, value.unit) else {
        return false;
    };
    let original_unit = value.unit;

    let candidates = [
        (px, LengthUnit::Px),
        (px / 96.0, LengthUnit::In),
        (px * 2.54 / 96.0, LengthUnit::Cm),
        (px * 25.4 / 96.0, LengthUnit::Mm),
        (px * 101.6 / 96.0, LengthUnit::Q),
        (px * 72.0 / 96.0, LengthUnit::Pt),
        (px / 16.0, LengthUnit::Pc),
    ];
    let original_len = dimension_len(value.value, Unit::Length(value.unit));
    let Some((candidate_value, candidate_unit)) = candidates
        .into_iter()
        .filter(|(number, _)| number.is_finite())
        .min_by_key(|(number, unit)| dimension_len(*number, Unit::Length(*unit)))
    else {
        return false;
    };

    if dimension_len(candidate_value, Unit::Length(candidate_unit)) < original_len {
        value.value = clean_float(candidate_value);
        value.unit = candidate_unit;
    }
    value.value != original_value || value.unit != original_unit
}

pub(crate) fn minify_angle(value: &mut Angle) -> bool {
    let degrees = match *value {
        Angle::Deg(number) => number,
        Angle::Rad(number) => number.to_degrees(),
        Angle::Grad(number) => number * 0.9,
        Angle::Turn(number) => number * 360.0,
    };
    let candidates = [
        Angle::Deg(clean_float(degrees)),
        Angle::Turn(clean_float(degrees / 360.0)),
        Angle::Grad(clean_float(degrees / 0.9)),
        Angle::Rad(clean_float(degrees.to_radians())),
    ];
    let original_len = dimension_len(angle_number(value), angle_unit(value));
    let candidate = candidates
        .into_iter()
        .min_by_key(|candidate| dimension_len(angle_number(candidate), angle_unit(candidate)))
        .expect("angle candidates are non-empty");
    let candidate_len = dimension_len(angle_number(&candidate), angle_unit(&candidate));
    if candidate_len < original_len {
        *value = candidate;
        true
    } else {
        false
    }
}

pub(crate) fn minify_time(value: &mut Time) -> bool {
    let milliseconds = match *value {
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
    if &candidate != value {
        *value = candidate;
        true
    } else {
        false
    }
}

pub(crate) fn minify_resolution(value: &mut Resolution) -> bool {
    let dpi = match *value {
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
    if &candidate != value {
        *value = candidate;
        true
    } else {
        false
    }
}

pub(crate) fn minify_dimension(value: f32, unit: Unit) -> Option<(f32, Unit)> {
    if let Unit::Length(length_unit) = unit
        && to_px(value, length_unit).is_some()
    {
        let mut length = LengthValue {
            value,
            unit: length_unit,
        };
        minify_length(&mut length);
        return Some((length.value, Unit::Length(length.unit)));
    }
    if matches!(unit, Unit::Milliseconds | Unit::Seconds) {
        let mut time = match unit {
            Unit::Milliseconds => Time::Milliseconds(value),
            Unit::Seconds => Time::Seconds(value),
            _ => unreachable!("time units were checked above"),
        };
        minify_time(&mut time);
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
        minify_angle(&mut angle);
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
    let output = value.to_string();
    if value != 0.0 && value.abs() < 1.0 {
        output.len().saturating_sub(1)
    } else {
        output.len()
    }
}

fn clean_float(value: f32) -> f32 {
    let rounded = value.round();
    if (value - rounded).abs() < 0.000_01 {
        rounded
    } else {
        (value * 1_000_000.0).round() / 1_000_000.0
    }
}

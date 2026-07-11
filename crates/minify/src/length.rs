use rs_css_ast::{Angle, LengthUnit, LengthValue, Resolution, Time};

pub(crate) fn minify_length(value: &mut LengthValue) -> bool {
    if value.value == 0.0 {
        return false;
    }
    let original_value = value.value;
    let Some(px) = to_px(value.value, &value.unit) else {
        return false;
    };
    let original_unit = length_unit_name(&value.unit);

    let candidates = [
        (px, LengthUnit::Px),
        (px / 96.0, LengthUnit::In),
        (px * 2.54 / 96.0, LengthUnit::Cm),
        (px * 25.4 / 96.0, LengthUnit::Mm),
        (px * 101.6 / 96.0, LengthUnit::Q),
        (px * 72.0 / 96.0, LengthUnit::Pt),
        (px / 16.0, LengthUnit::Pc),
    ];
    let original_len = dimension_len(value.value, length_unit_name(&value.unit));
    let Some((candidate_value, candidate_unit)) = candidates
        .into_iter()
        .filter(|(number, _)| number.is_finite())
        .min_by_key(|(number, unit)| dimension_len(*number, length_unit_name(unit)))
    else {
        return false;
    };

    if dimension_len(candidate_value, length_unit_name(&candidate_unit)) < original_len {
        value.value = clean_float(candidate_value);
        value.unit = candidate_unit;
    }
    value.value != original_value || length_unit_name(&value.unit) != original_unit
}

pub(crate) fn minify_angle(value: &mut Angle) -> bool {
    let degrees = match *value {
        Angle::Deg(number) => number,
        Angle::Rad(number) => number.to_degrees(),
        Angle::Grad(number) => number * 0.9,
        Angle::Turn(number) => number * 360.0,
    };
    let candidates = [
        (Angle::Deg(clean_float(degrees)), "deg"),
        (Angle::Turn(clean_float(degrees / 360.0)), "turn"),
        (Angle::Grad(clean_float(degrees / 0.9)), "grad"),
        (Angle::Rad(clean_float(degrees.to_radians())), "rad"),
    ];
    let original_len = match *value {
        Angle::Deg(number) => dimension_len(number, "deg"),
        Angle::Rad(number) => dimension_len(number, "rad"),
        Angle::Grad(number) => dimension_len(number, "grad"),
        Angle::Turn(number) => dimension_len(number, "turn"),
    };
    let (candidate, _) = candidates
        .into_iter()
        .min_by_key(|(candidate, unit)| dimension_len(angle_number(candidate), unit))
        .expect("angle candidates are non-empty");
    let candidate_len = match candidate {
        Angle::Deg(number) => dimension_len(number, "deg"),
        Angle::Rad(number) => dimension_len(number, "rad"),
        Angle::Grad(number) => dimension_len(number, "grad"),
        Angle::Turn(number) => dimension_len(number, "turn"),
    };
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
    let seconds_len = dimension_len(seconds, "s");
    let milliseconds_len = dimension_len(milliseconds, "ms");
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

pub(crate) fn minify_dimension(value: f32, unit: &str) -> Option<(f32, &'static str)> {
    if let Some(length_unit) = parse_absolute_length_unit(unit) {
        let mut length = LengthValue {
            value,
            unit: length_unit,
        };
        minify_length(&mut length);
        return Some((length.value, length_unit_name(&length.unit)));
    }
    if unit.eq_ignore_ascii_case("ms") || unit.eq_ignore_ascii_case("s") {
        let mut time = if unit.eq_ignore_ascii_case("ms") {
            Time::Milliseconds(value)
        } else {
            Time::Seconds(value)
        };
        minify_time(&mut time);
        return Some(match time {
            Time::Seconds(number) => (number, "s"),
            Time::Milliseconds(number) => (number, "ms"),
        });
    }
    if let Some(mut angle) = parse_angle(value, unit) {
        minify_angle(&mut angle);
        return Some(match angle {
            Angle::Deg(number) => (number, "deg"),
            Angle::Rad(number) => (number, "rad"),
            Angle::Grad(number) => (number, "grad"),
            Angle::Turn(number) => (number, "turn"),
        });
    }
    None
}

pub(crate) fn is_length_unit(unit: &str) -> bool {
    [
        "px", "in", "cm", "mm", "q", "pt", "pc", "em", "rem", "ex", "rex", "ch", "rch", "cap",
        "rcap", "ic", "ric", "lh", "rlh", "vw", "vh", "vi", "vb", "vmin", "vmax", "svw", "svh",
        "lvw", "lvh", "dvw", "dvh", "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax",
    ]
    .into_iter()
    .any(|candidate| unit.eq_ignore_ascii_case(candidate))
}

fn parse_absolute_length_unit(unit: &str) -> Option<LengthUnit> {
    if unit.eq_ignore_ascii_case("px") {
        Some(LengthUnit::Px)
    } else if unit.eq_ignore_ascii_case("in") {
        Some(LengthUnit::In)
    } else if unit.eq_ignore_ascii_case("cm") {
        Some(LengthUnit::Cm)
    } else if unit.eq_ignore_ascii_case("mm") {
        Some(LengthUnit::Mm)
    } else if unit.eq_ignore_ascii_case("q") {
        Some(LengthUnit::Q)
    } else if unit.eq_ignore_ascii_case("pt") {
        Some(LengthUnit::Pt)
    } else if unit.eq_ignore_ascii_case("pc") {
        Some(LengthUnit::Pc)
    } else {
        None
    }
}

fn parse_angle(value: f32, unit: &str) -> Option<Angle> {
    if unit.eq_ignore_ascii_case("deg") {
        Some(Angle::Deg(value))
    } else if unit.eq_ignore_ascii_case("rad") {
        Some(Angle::Rad(value))
    } else if unit.eq_ignore_ascii_case("grad") {
        Some(Angle::Grad(value))
    } else if unit.eq_ignore_ascii_case("turn") {
        Some(Angle::Turn(value))
    } else {
        None
    }
}

fn to_px(value: f32, unit: &LengthUnit) -> Option<f32> {
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

fn length_unit_name(unit: &LengthUnit) -> &'static str {
    match unit {
        LengthUnit::Px => "px",
        LengthUnit::In => "in",
        LengthUnit::Cm => "cm",
        LengthUnit::Mm => "mm",
        LengthUnit::Q => "q",
        LengthUnit::Pt => "pt",
        LengthUnit::Pc => "pc",
        _ => unreachable!("only absolute length units are normalized"),
    }
}

fn angle_number(value: &Angle) -> f32 {
    match value {
        Angle::Deg(number) | Angle::Rad(number) | Angle::Grad(number) | Angle::Turn(number) => {
            *number
        }
    }
}

fn resolution_len(value: &Resolution) -> usize {
    match value {
        Resolution::Dpi(number) => dimension_len(*number, "dpi"),
        Resolution::Dpcm(number) => dimension_len(*number, "dpcm"),
        Resolution::Dppx(number) => dimension_len(*number, "dppx"),
    }
}

fn dimension_len(value: f32, unit: &str) -> usize {
    number_len(value) + unit.len()
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

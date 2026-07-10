use rs_css_ast::Ratio;

pub(crate) fn minify_ratio(value: &mut Ratio) -> bool {
    if value.0 <= 0.0 || value.1 <= 0.0 {
        return false;
    }
    let (left, right) = (value.0.round(), value.1.round());
    if (value.0 - left).abs() > f32::EPSILON || (value.1 - right).abs() > f32::EPSILON {
        return false;
    }
    let divisor = gcd(left as u32, right as u32);
    if divisor <= 1 {
        return false;
    }
    value.0 /= divisor as f32;
    value.1 /= divisor as f32;
    true
}

fn gcd(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

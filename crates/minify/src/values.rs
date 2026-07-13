use rocketcss_ast::Ratio;

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl Minify for Ratio {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None)
            || self.0 <= 0.0
            || self.1 <= 0.0
        {
            return;
        }
        let mut scale = 1_u64;
        while scale < 1_000_000
            && (!is_near_integer(self.0 * scale as f32) || !is_near_integer(self.1 * scale as f32))
        {
            scale *= 10;
        }
        let left = (self.0 * scale as f32).round() as u64;
        let right = (self.1 * scale as f32).round() as u64;
        let divisor = gcd(left, right);
        if divisor == 0 {
            return;
        }
        let reduced_left = (left / divisor) as f32;
        let reduced_right = (right / divisor) as f32;
        if reduced_left == self.0 && reduced_right == self.1 {
            return;
        }
        self.0 = reduced_left;
        self.1 = reduced_right;
        cx.record_value_normalized();
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

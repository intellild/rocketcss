use rocketcss_ast::Ratio;

use crate::{Minify, MinifyContext};

impl<'a> Minify<'a> for Ratio {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        if !context.options().normalize_values || self.0 <= 0.0 || self.1 <= 0.0 {
            return;
        }
        let (left, right) = (self.0.round(), self.1.round());
        if (self.0 - left).abs() > f32::EPSILON || (self.1 - right).abs() > f32::EPSILON {
            return;
        }
        let divisor = gcd(left as u32, right as u32);
        if divisor <= 1 {
            return;
        }
        self.0 /= divisor as f32;
        self.1 /= divisor as f32;
        context.record_value_normalized();
    }
}

fn gcd(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

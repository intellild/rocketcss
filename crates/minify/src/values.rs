use rocketcss_ast::{Ratio, StyleSheet, ZIndex};
use rocketcss_visitor::VisitMut;

use crate::{Minify, MinifyContext, Options, OptionsOp};

pub(crate) fn reduce_z_indices<'a>(stylesheet: &mut StyleSheet<'a>, cx: &mut MinifyContext) {
    if cx.is_enabled(Options::REDUCE_Z_INDICES, OptionsOp::None) {
        return;
    }

    let mut collector = ZIndexCollector::default();
    collector.visit_style_sheet(stylesheet);
    if collector.has_negative || collector.values.is_empty() {
        return;
    }

    collector.values.sort_unstable();
    collector.values.dedup();
    ZIndexRewriter {
        values: &collector.values,
        start: cx.options().z_index_start,
        cx,
    }
    .visit_style_sheet(stylesheet);
}

#[derive(Default)]
struct ZIndexCollector {
    values: std::vec::Vec<i32>,
    has_negative: bool,
}

impl<'a> VisitMut<'a> for ZIndexCollector {
    fn visit_z_index(&mut self, node: &mut ZIndex) {
        let ZIndex::Integer(value) = node else {
            return;
        };
        if *value < 0 {
            self.has_negative = true;
        } else if *value > 0 {
            self.values.push(*value);
        }
    }
}

struct ZIndexRewriter<'values, 'cx> {
    values: &'values [i32],
    start: i32,
    cx: &'cx mut MinifyContext,
}

impl<'a> VisitMut<'a> for ZIndexRewriter<'_, '_> {
    fn visit_z_index(&mut self, node: &mut ZIndex) {
        let ZIndex::Integer(value) = node else {
            return;
        };
        let Ok(index) = self.values.binary_search(value) else {
            return;
        };
        let Ok(offset) = i32::try_from(index) else {
            debug_assert!(
                false,
                "z-index IR cannot contain more than i32::MAX entries"
            );
            return;
        };
        let Some(rebased) = self.start.checked_add(offset) else {
            debug_assert!(false, "rebased z-index must fit in i32");
            return;
        };
        if *value != rebased {
            *value = rebased;
            self.cx.record_value_normalized();
        }
    }
}

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

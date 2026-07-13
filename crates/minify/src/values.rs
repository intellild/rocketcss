use rocketcss_ast::{Ratio, StyleSheet, ZIndex};
use rocketcss_visitor::VisitMut;

use crate::{Minify, MinifyContext, Options};

pub(crate) fn reduce_z_indices<'a>(stylesheet: &mut StyleSheet<'a>, context: &mut MinifyContext) {
    if !context.options().is_enabled(Options::REDUCE_Z_INDICES) {
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
        start: context.options().z_index_start,
        context,
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

struct ZIndexRewriter<'values, 'context> {
    values: &'values [i32],
    start: i32,
    context: &'context mut MinifyContext,
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
            self.context.record_value_normalized();
        }
    }
}

impl Minify for Ratio {
    fn minify(&mut self, context: &mut MinifyContext) {
        if !context.options().is_enabled(Options::NORMALIZE_VALUES)
            || self.0 <= 0.0
            || self.1 <= 0.0
        {
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

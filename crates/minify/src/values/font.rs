use rocketcss_allocator::vec::Vec;
use rocketcss_ast::FontFamily;

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl<'a> Minify for Vec<'a, FontFamily<'a>> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::Any)
            && let Some(generic) = self.iter().position(is_terminal_generic)
            && self[..generic].iter().any(|family| !family.is_tombstone())
            && self[generic + 1..]
                .iter()
                .any(|family| !family.is_tombstone())
        {
            for family in &mut self[generic + 1..] {
                if !family.is_tombstone() {
                    *family = FontFamily::Tombstone;
                    cx.record_value_normalized();
                }
            }
        }

        if cx.is_enabled(Options::DEDUPLICATE_LISTS, OptionsOp::None) {
            return;
        }

        for current in 1..self.len() {
            if self[current].is_tombstone() {
                continue;
            }
            let duplicate = self[..current]
                .iter()
                .filter(|previous| !previous.is_tombstone())
                .any(|previous| equivalent(previous, &self[current]));
            if duplicate {
                self[current] = FontFamily::Tombstone;
                cx.record_value_normalized();
            }
        }
    }
}

fn is_terminal_generic(family: &FontFamily<'_>) -> bool {
    matches!(
        family,
        FontFamily::Serif
            | FontFamily::SansSerif
            | FontFamily::Cursive
            | FontFamily::Fantasy
            | FontFamily::Monospace
    )
}

fn equivalent(left: &FontFamily<'_>, right: &FontFamily<'_>) -> bool {
    match (left, right) {
        (FontFamily::Custom(left), FontFamily::Custom(right)) => left.eq_ignore_ascii_case(right),
        _ => left == right,
    }
}

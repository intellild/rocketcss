use rocketcss_allocator::vec::Vec;
use rocketcss_ast::FontFamily;

use crate::{Minify, MinifyContext, Options, OptionsOp};

impl<'a> Minify for Vec<'a, FontFamily<'a>> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }

        if let Some(generic) = self.iter().position(FontFamily::is_generic)
            && generic > 0
            && generic + 1 < self.len()
        {
            self.truncate(generic + 1);
            cx.record_value_normalized();
            return;
        }

        let mut current = 1;
        while current < self.len() {
            let duplicate = !self[current].is_generic()
                && self[..current]
                    .iter()
                    .any(|previous| equivalent(previous, &self[current]));
            if duplicate {
                self.remove(current);
                cx.record_value_normalized();
            } else {
                current += 1;
            }
        }
    }
}

fn equivalent(left: &FontFamily<'_>, right: &FontFamily<'_>) -> bool {
    match (left, right) {
        (FontFamily::Custom(left), FontFamily::Custom(right)) => left.eq_ignore_ascii_case(right),
        _ => left == right,
    }
}

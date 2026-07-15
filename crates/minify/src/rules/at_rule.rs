use rocketcss_ast::UnknownAtRule;

use crate::{Minify, MinifyContext};

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}

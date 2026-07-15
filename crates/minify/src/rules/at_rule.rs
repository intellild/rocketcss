use rocketcss_ast::UnknownAtRule;

use crate::{Minify, MinifyContext};

impl Minify for UnknownAtRule<'_> {
    fn minify<'alloc>(&mut self, cx: &mut MinifyContext<'alloc>)
    where
        Self: 'alloc,
    {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}

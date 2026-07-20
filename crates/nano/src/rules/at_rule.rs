use rocketcss_ast::UnknownAtRule;

use crate::{Minify, MinifyContext};

impl Minify for UnknownAtRule<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}

use rocketcss_ast::UnparsedProperty;

use crate::{Minify, MinifyContext};

impl Minify for UnparsedProperty<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        self.value.minify(cx);
    }
}

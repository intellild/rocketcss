use rocketcss_ast::{CustomProperty, UnparsedProperty};

use crate::{Minify, MinifyContext};

impl Minify for UnparsedProperty<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        self.value.minify(cx);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        self.value.minify(cx);
    }
}

use rocketcss_ast::{CustomProperty, UnparsedProperty};

use crate::{Minify, MinifyContext};

impl Minify for UnparsedProperty<'_> {
    fn minify<'alloc>(&mut self, cx: &mut MinifyContext<'alloc>)
    where
        Self: 'alloc,
    {
        self.value.minify(cx);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify<'alloc>(&mut self, cx: &mut MinifyContext<'alloc>)
    where
        Self: 'alloc,
    {
        self.value.minify(cx);
    }
}

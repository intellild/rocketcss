use rocketcss_ast::{CustomProperty, UnparsedProperty};

use crate::{Minify, MinifyContext};

impl Minify for UnparsedProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.value.minify(cx);
    }
}

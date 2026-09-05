use rocketcss_ast::UnparsedProperty;

use crate::{Minify, MinifyContext};

impl Minify for UnparsedProperty<'_> {
    fn minify<'cx>(&mut self, _cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        // Unparsed values are intentionally opaque to nano. Codegen remains
        // lossless and emits the original token sequence.
    }
}

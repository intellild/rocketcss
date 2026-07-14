use super::*;

impl Minify for StyleSheet<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        crate::minify_style_sheet(self, cx);
    }
}

impl Minify for KeyframeSelector<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if cx.is_enabled(Options::NORMALIZE_VALUES, OptionsOp::None) {
            return;
        }
        let changed = match self {
            KeyframeSelector::From => {
                *self = KeyframeSelector::Percentage(0.0);
                true
            }
            KeyframeSelector::Percentage(value) if *value == 1.0 => {
                *self = KeyframeSelector::To;
                true
            }
            _ => false,
        };
        if changed {
            cx.record_value_normalized();
        }
    }
}

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

impl Minify for Variable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, cx: &mut MinifyContext) {
        self.prelude.minify(cx);
        if let Some(block) = &mut self.block {
            block.minify(cx);
        }
    }
}

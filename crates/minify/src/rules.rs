use rocketcss_ast::{
    CustomProperty, EnvironmentVariable, Function, KeyframeSelector, StyleSheet, UnknownAtRule,
    UnparsedProperty, Variable,
};

use crate::{Minify, MinifyContext};

impl Minify for StyleSheet<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        crate::minify_style_sheet(self, context);
    }
}

impl Minify for KeyframeSelector<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if !context.options().normalize_values {
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
            context.record_value_normalized();
        }
    }
}

impl Minify for UnparsedProperty<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.value.minify(context);
    }
}

impl Minify for CustomProperty<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.value.minify(context);
    }
}

impl Minify for Function<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.arguments.minify(context);
    }
}

impl Minify for Variable<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl Minify for UnknownAtRule<'_> {
    fn minify(&mut self, context: &mut MinifyContext) {
        self.prelude.minify(context);
        if let Some(block) = &mut self.block {
            block.minify(context);
        }
    }
}

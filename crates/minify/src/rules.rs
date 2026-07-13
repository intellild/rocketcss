use rocketcss_ast::{
    CustomProperty, EnvironmentVariable, Function, KeyframeSelector, StyleSheet, UnknownAtRule,
    UnparsedProperty, Variable,
};

use crate::{Minify, MinifyContext};

impl<'a> Minify<'a> for StyleSheet<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        crate::minify_style_sheet(self, context);
    }
}

impl<'a> Minify<'a> for KeyframeSelector<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
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

impl<'a> Minify<'a> for UnparsedProperty<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        self.value.minify(context);
    }
}

impl<'a> Minify<'a> for CustomProperty<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        self.value.minify(context);
    }
}

impl<'a> Minify<'a> for Function<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        self.arguments.minify(context);
    }
}

impl<'a> Minify<'a> for Variable<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl<'a> Minify<'a> for EnvironmentVariable<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(context);
        }
    }
}

impl<'a> Minify<'a> for UnknownAtRule<'a> {
    fn minify(&mut self, context: &mut MinifyContext<'a>) {
        self.prelude.minify(context);
        if let Some(block) = &mut self.block {
            block.minify(context);
        }
    }
}

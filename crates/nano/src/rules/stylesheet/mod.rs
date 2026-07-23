mod adjacent;
mod calc;
mod color;
mod declaration_block;
mod function;
mod gradient;
mod timing;
mod transform;
mod url;

use rocketcss_allocator::vec::Vec;
use rocketcss_ast::{
    EnvironmentVariable, Function, FunctionReplacement, KnownFunction, LengthUnit, Token,
    TokenOrValue, Unit, Variable, match_ignore_ascii_case,
};

use crate::{Minify, MinifyContext, Options, OptionsOp, context::ValueContextFlags};

pub(crate) use adjacent::merge_adjacent_style_rules;
pub(crate) use declaration_block::DeclarationBlockMinifier;
pub(crate) use url::normalize_url_text;

fn token_or_value_contains_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) => true,
        TokenOrValue::Function(function) => {
            function.kind() == KnownFunction::Var
                || function
                    .arguments
                    .iter()
                    .any(token_or_value_contains_variable)
        }
        _ => false,
    }
}

fn token_number(value: &TokenOrValue<'_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Number(value) => Some(value),
        Token::Dimension { value, .. } | Token::UnknownDimension { value, .. } => Some(value),
        _ => None,
    }
}

fn number_at(values: &[TokenOrValue<'_>], index: usize) -> Option<f32> {
    values.get(index).and_then(token_number)
}

fn token_ident<'a>(value: &'a TokenOrValue<'a>) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match **token {
        Token::Ident(value) => Some(value),
        _ => None,
    }
}

fn is_comma(value: &TokenOrValue<'_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::Comma))
}

impl Minify for Variable<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

impl Minify for EnvironmentVariable<'_> {
    fn minify<'cx>(&mut self, cx: &mut MinifyContext<'cx>)
    where
        Self: 'cx,
    {
        if let Some(fallback) = &mut self.fallback {
            fallback.minify(cx);
        }
    }
}

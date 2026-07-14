mod calc;
mod color;
mod declaration_block;
mod function;
mod gradient;
mod misc;
mod timing;
mod transform;
mod url;

use rocketcss_allocator::prelude::{Allocator, HashMap, Vec};
use rocketcss_ast::{
    CustomProperty, Declaration, DeclarationBlock, EnvironmentVariable, Function,
    FunctionReplacement, KeyframeSelector, KnownFunction, LengthUnit, LengthValue, Margin, Padding,
    PropertyId, StyleSheet, Token, TokenOrValue, Unit, UnknownAtRule, UnparsedProperty, Variable,
    VendorPrefix, match_ignore_ascii_case,
};

use crate::{Minify, MinifyContext, Options, OptionsOp, context::ValueContextFlags};

pub(crate) use url::normalize_url_text;

fn token_or_value_contains_variable(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
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

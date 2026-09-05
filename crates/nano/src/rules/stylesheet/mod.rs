mod calc;
mod color;
mod declaration_block;
mod function;
mod gradient;
mod timing;
mod transform;
mod url;

use rocketcss_ast::{
    Function, FunctionReplacement, KnownFunction, LengthUnit, Token, TokenOrValue, Unit,
    VisitMutContext, match_ignore_ascii_case,
};
use rocketcss_common::vec::Vec;

use crate::{MinifyContext, Options, OptionsOp, context::ValueContextFlags};

pub(crate) use declaration_block::DeclarationBlockMinifier;
pub(crate) use function::minify_function;
pub(crate) use url::normalize_url_text;

fn token_or_value_contains_variable(
    value: &TokenOrValue<'_>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> bool {
    match value {
        TokenOrValue::Var(_) => true,
        TokenOrValue::Function(function) => {
            let function = ast.ast_context().resolve_node(*function);
            function.kind() == KnownFunction::Var
                || ast
                    .ast_context()
                    .vec(function.arguments)
                    .iter()
                    .any(|value| token_or_value_contains_variable(value, ast))
        }
        _ => false,
    }
}

fn token_number(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> Option<f32> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Number(value) => Some(*value),
        Token::Dimension { value, .. } | Token::UnknownDimension { value, .. } => Some(*value),
        _ => None,
    }
}

fn number_at(
    values: &[TokenOrValue<'_>],
    index: usize,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<f32> {
    values.get(index).and_then(|value| token_number(value, ast))
}

fn token_ident<'a>(
    value: &'a TokenOrValue<'a>,
    ast: &VisitMutContext<'_, '_, '_>,
) -> Option<&'a str> {
    let TokenOrValue::Token(token) = value else {
        return None;
    };
    match ast.ast_context().resolve_node(*token) {
        Token::Ident(value) => Some(*value),
        _ => None,
    }
}

fn is_comma(value: &TokenOrValue<'_>, ast: &VisitMutContext<'_, '_, '_>) -> bool {
    matches!(value, TokenOrValue::Token(token) if matches!(ast.ast_context().resolve_node(*token), Token::Comma))
}

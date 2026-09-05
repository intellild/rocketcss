use super::{
    color::{parse_hex_color, validate_rgb_function},
    stylesheet::check_depth,
};
use crate::prelude::*;

mod alignment;
mod animation;
mod background;
mod border;
mod box_model;
mod flex;
mod font;
mod image;
mod mask;
mod multicol;
mod svg;
mod text;
mod transform;
mod ui;

pub(super) use animation::{
    parse_animation_list, parse_comma_separated, parse_transition_property_list,
    value_contains_comment,
};
pub(super) use font::parse_font_family_list;
pub(super) use transform::parse_transform_list;

pub(super) fn single_token<'context, 'i>(
    ast: &'context AstContext<'i>,
    value: &[TokenOrValue<'i>],
) -> Option<&'context ValueToken<'i>> {
    if let [TokenOrValue::Token(token)] = value {
        Some(ast.node(*token))
    } else {
        None
    }
}

pub(super) fn token_values_contain_opaque<'i>(
    ast: &AstContext<'i>,
    values: &[TokenOrValue<'i>],
) -> bool {
    values.iter().any(|value| match value {
        TokenOrValue::Token(token) => matches!(ast.node(*token), ValueToken::Comment(_)),
        TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
        TokenOrValue::Function(function) => {
            let function = ast.node(*function);
            function.kind().is_variable()
                || ast.vec(function.arguments).iter().any(|argument| {
                    token_values_contain_opaque(ast, std::slice::from_ref(argument))
                })
        }
        _ => false,
    })
}

pub(super) fn collect_tokens<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, allocator, depth, false)
}

pub(super) fn collect_custom_property_tokens<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    collect_tokens_impl(input, allocator, depth, true)
}

fn collect_tokens_impl<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    depth: usize,
    parse_embedded_values: bool,
) -> Result<Vec<'i, TokenOrValue<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut tokens = allocator.vec();

    loop {
        let state = input.state();
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        let token_span = input.current_token_span().unwrap_or_default();

        match token {
            ValueToken::Function(name) => {
                let arguments = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, allocator, depth + 1, parse_embedded_values)
                })?;
                let arguments = store_vec(arguments, input);
                let mut function = Function::new(name, arguments);
                validate_rgb_function(input.ast_context(), &mut function);
                let kind = function.kind();
                let valid_rgb = function.is_valid_rgb();
                let function = store_node(function, input);
                if parse_embedded_values
                    && kind.is_color()
                    && (!matches!(kind, KnownFunction::Rgb | KnownFunction::Rgba) || valid_rgb)
                {
                    let color = input
                        .ast_context_mut()
                        .alloc_node(CssColor::Function(function), token_span);
                    tokens.push(TokenOrValue::Color(color));
                } else {
                    tokens.push(TokenOrValue::Function(function));
                }
            }
            ValueToken::Hash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    let color = input
                        .ast_context_mut()
                        .alloc_node(CssColor::Rgba(color), token_span);
                    tokens.push(TokenOrValue::Color(color));
                } else {
                    tokens.push(TokenOrValue::Token(store_node(
                        ValueToken::Hash(value),
                        input,
                    )));
                }
            }
            ValueToken::IdHash(value) if parse_embedded_values => {
                if let Some(color) = parse_hex_color(value) {
                    let color = input
                        .ast_context_mut()
                        .alloc_node(CssColor::Rgba(color), token_span);
                    tokens.push(TokenOrValue::Color(color));
                } else {
                    tokens.push(TokenOrValue::Token(store_node(
                        ValueToken::IdHash(value),
                        input,
                    )));
                }
            }
            ValueToken::UnquotedUrl(url) => {
                let span = input.current_token_span().unwrap_or_default();
                let url = input.ast_context_mut().alloc_node(Url { url }, span);
                tokens.push(TokenOrValue::Url(url));
            }
            ValueToken::Ident(name) if name.starts_with("--") => {
                tokens.push(TokenOrValue::DashedIdent(name));
            }
            opening @ (ValueToken::ParenthesisBlock
            | ValueToken::SquareBracketBlock
            | ValueToken::CurlyBracketBlock) => {
                let closing = match opening {
                    ValueToken::ParenthesisBlock => ValueToken::CloseParenthesis,
                    ValueToken::SquareBracketBlock => ValueToken::CloseSquareBracket,
                    ValueToken::CurlyBracketBlock => ValueToken::CloseCurlyBracket,
                    _ => unreachable!(),
                };
                tokens.push(TokenOrValue::Token(store_node(opening, input)));
                let nested = input.parse_nested_block(|input| {
                    collect_tokens_impl(input, allocator, depth + 1, parse_embedded_values)
                })?;
                tokens.extend(nested);
                tokens.push(TokenOrValue::Token(store_node(closing, input)));
            }
            ValueToken::BadUrl(_)
            | ValueToken::BadString(_)
            | ValueToken::CloseParenthesis
            | ValueToken::CloseSquareBracket
            | ValueToken::CloseCurlyBracket => {
                let token = input.current_token().unwrap_or_else(|| {
                    crate::TokenAndSpan::new(crate::Token::BadString, Span::default())
                });
                input.reset(&state);
                return Err(input.new_custom_error(ParserError::UnexpectedToken(token)));
            }
            token => tokens.push(TokenOrValue::Token(store_node(token, input))),
        }
    }

    Ok(tokens)
}

pub(super) fn remove_important<'i>(
    ast: &AstContext<'i>,
    value: &mut Vec<'i, TokenOrValue<'i>>,
) -> bool {
    let Some(important_index) = previous_non_whitespace(ast, value, value.len()) else {
        return false;
    };
    if !token_ident(ast, &value[important_index])
        .is_some_and(|name| name.eq_ignore_ascii_case("important"))
    {
        trim_trailing_whitespace(ast, value);
        return false;
    }
    let Some(bang_index) = previous_non_whitespace(ast, value, important_index) else {
        trim_trailing_whitespace(ast, value);
        return false;
    };
    if !matches!(&value[bang_index], TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::Delim("!")))
    {
        trim_trailing_whitespace(ast, value);
        return false;
    }
    value.remove(important_index);
    value.remove(bang_index);
    trim_trailing_whitespace(ast, value);
    true
}

pub(super) fn previous_non_whitespace<'i>(
    ast: &AstContext<'i>,
    value: &[TokenOrValue<'i>],
    before: usize,
) -> Option<usize> {
    (0..before).rev().find(|index| {
        !matches!(&value[*index], TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::WhiteSpace(_) | ValueToken::Comment(_)))
    })
}

pub(super) fn trim_trailing_whitespace<'i>(
    ast: &AstContext<'i>,
    value: &mut Vec<'i, TokenOrValue<'i>>,
) {
    while matches!(value.last(), Some(TokenOrValue::Token(token)) if matches!(ast.node(*token), ValueToken::WhiteSpace(_)))
    {
        value.pop();
    }
}

pub(super) fn trim_leading_whitespace<'i>(
    ast: &AstContext<'i>,
    value: &mut Vec<'i, TokenOrValue<'i>>,
) {
    while matches!(value.first(), Some(TokenOrValue::Token(token)) if matches!(ast.node(*token), ValueToken::WhiteSpace(_)))
    {
        value.remove(0);
    }
}

pub(super) fn token_ident<'i>(ast: &AstContext<'i>, value: &TokenOrValue<'i>) -> Option<&'i str> {
    match value {
        TokenOrValue::Token(token) => match ast.node(*token) {
            ValueToken::Ident(name) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn css_wide_keyword(value: &str) -> Option<CSSWideKeyword> {
    match_ignore_ascii_case!(
        value,
        "initial" => Some(CSSWideKeyword::Initial),
        "inherit" => Some(CSSWideKeyword::Inherit),
        "unset" => Some(CSSWideKeyword::Unset),
        "revert" => Some(CSSWideKeyword::Revert),
        "revert-layer" => Some(CSSWideKeyword::RevertLayer),
        _ => None,
    )
}

pub(super) fn matches_ignore_case(value: &str, expected: &[&str]) -> bool {
    expected.iter().any(|item| value.eq_ignore_ascii_case(item))
}

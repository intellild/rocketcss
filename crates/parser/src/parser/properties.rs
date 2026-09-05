use super::values::{
    collect_custom_property_tokens, collect_tokens, css_wide_keyword, parse_animation_list,
    parse_comma_separated, parse_font_family_list, parse_transform_list,
    parse_transition_property_list, remove_important, trim_leading_whitespace,
    value_contains_comment,
};
use crate::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy)]
pub(super) enum CssWideValueHint<'i> {
    NotCssWide,
    Candidate(&'i str),
}

pub(super) fn parse_declaration_with_css_wide_hint<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
    css_wide_hint: CssWideValueHint<'i>,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let property_id = PropertyId::from_name(name);

    let replay_enabled = !name.starts_with("--")
        && (matches!(css_wide_hint, CssWideValueHint::Candidate(_))
            || property_id.parser_strategy() != PropertyParserStrategy::Unsupported);
    if !replay_enabled {
        return parse_declaration_fallback(input, allocator, name, depth, property_id, false);
    }

    input.with_declaration_token_replay(|input| {
        parse_known_declaration_with_fallback(input, allocator, name, depth, css_wide_hint)
    })
}

fn parse_known_declaration_with_fallback<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
    css_wide_hint: CssWideValueHint<'i>,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let property_id = PropertyId::from_name(name);
    let start = input.state();

    let wide_keyword = match (property_id.known_id(), css_wide_hint) {
        (Some(_), CssWideValueHint::Candidate(ident)) => {
            if let Some(keyword) = css_wide_keyword(ident) {
                let parsed_ident = input.expect_ident()?;
                debug_assert_eq!(parsed_ident, ident);
                Some(keyword)
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(keyword) = wide_keyword {
        if let Some(important) = parse_declaration_end(input)
            && !input.saw_comments_since(&start)
        {
            let _ = input.try_parse(Compiler::expect_semicolon);
            let declaration = match property_id {
                PropertyId::All => Declaration::All(keyword),
                PropertyId::ColumnWidth(prefix) => {
                    Declaration::ColumnWidth(CSSWideOr::CSSWide(keyword), prefix)
                }
                PropertyId::ColumnCount(prefix) => {
                    Declaration::ColumnCount(CSSWideOr::CSSWide(keyword), prefix)
                }
                PropertyId::Columns(prefix) => {
                    Declaration::Columns(CSSWideOr::CSSWide(keyword), prefix)
                }
                _ => Declaration::CSSWide(store_node(property_id, input), keyword),
            };
            return Ok((declaration, important));
        }
        input.reset(&start);
    }

    let typed = try_parse_typed_declaration(input, &property_id, allocator, depth);
    let typed_grammar_supported = typed.is_some();
    if let Some(Ok(declaration)) = typed
        && let Some(important) = parse_declaration_end(input)
        && !input.saw_comments_since(&start)
    {
        let _ = input.try_parse(Compiler::expect_semicolon);
        return Ok((declaration, important));
    }
    input.reset(&start);

    parse_declaration_fallback(
        input,
        allocator,
        name,
        depth,
        property_id,
        typed_grammar_supported,
    )
}

fn parse_declaration_fallback<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
    property_id: PropertyId<'i>,
    typed_grammar_supported: bool,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let value_start = input.position();
    let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
        if name.starts_with("--") {
            collect_custom_property_tokens(input, allocator, depth + 1)
        } else {
            collect_tokens(input, allocator, depth + 1)
        }
    })?;
    let raw_value = input.slice(value_start..input.position());
    let _ = input.try_parse(Compiler::expect_semicolon);
    let important = remove_important(input.ast_context(), &mut value);

    let declaration = if name.starts_with("--") {
        let value = store_vec(value, input);
        Declaration::Custom(store_node(
            CustomProperty {
                name: store_node(CustomPropertyName::Custom(name), input),
                value,
            },
            input,
        ))
    } else {
        trim_leading_whitespace(input.ast_context(), &mut value);
        let reason = unparsed_reason(
            input.ast_context(),
            &property_id,
            &value,
            typed_grammar_supported,
        );
        unparsed_declaration(
            property_id,
            value,
            reason,
            preserve_unparsed_value(raw_value, important, allocator),
            input,
        )
    };

    Ok((declaration, important))
}

pub(super) fn unparsed_declaration<'i>(
    property_id: PropertyId<'i>,
    value: Vec<'i, TokenOrValue<'i>>,
    reason: UnparsedPropertyReason,
    raw_value: Option<&'i str>,
    input: &mut Compiler<'i>,
) -> Declaration<'i> {
    let value = store_vec(value, input);
    Declaration::Unparsed(store_node(
        UnparsedProperty {
            property_id: store_node(property_id, input),
            reason,
            raw_value,
            value,
        },
        input,
    ))
}

fn preserve_unparsed_value<'i>(
    raw_value: &'i str,
    important: bool,
    allocator: &'i Allocator,
) -> Option<&'i str> {
    let raw_value = trim_css_whitespace(raw_value);
    if !important {
        return Some(raw_value);
    }

    let (bang, important) = find_trailing_important(raw_value)?;
    let mut without_important = String::with_capacity(raw_value.len());
    without_important.push_str(&raw_value[..bang.start]);
    without_important.push_str(&raw_value[bang.end..important.start]);
    without_important.push_str(&raw_value[important.end..]);
    Some(allocator.alloc_str(trim_css_whitespace(&without_important)))
}

fn trim_css_whitespace(value: &str) -> &str {
    value.trim_matches([' ', '\t', '\n', '\r', '\x0C'])
}

fn find_trailing_important(value: &str) -> Option<(Range<usize>, Range<usize>)> {
    let mut tokenizer = Tokenizer::new(value);
    let mut depth = 0usize;
    let mut previous = None;
    let mut last = None;

    while let Ok(token) = tokenizer.next() {
        let is_opening = matches!(
            token.token,
            LexicalToken::Function
                | LexicalToken::ParenthesisBlock
                | LexicalToken::SquareBracketBlock
                | LexicalToken::CurlyBracketBlock
        );
        let is_closing = matches!(
            token.token,
            LexicalToken::CloseParenthesis
                | LexicalToken::CloseSquareBracket
                | LexicalToken::CloseCurlyBracket
        );

        if depth == 0
            && !matches!(
                token.token,
                LexicalToken::WhiteSpace | LexicalToken::Comment
            )
        {
            previous = last;
            last = Some(token);
        }

        if is_opening {
            depth += 1;
        } else if is_closing {
            depth = depth.saturating_sub(1);
        }
    }

    let last = last?;
    let previous = previous?;
    if last.token != LexicalToken::Ident
        || !crate::unescape(&value[last.span.start as usize..last.span.end as usize])
            .eq_ignore_ascii_case("important")
        || previous.token != LexicalToken::Delim
        || &value[previous.span.start as usize..previous.span.end as usize] != "!"
    {
        return None;
    }

    Some((
        previous.span.start as usize..previous.span.end as usize,
        last.span.start as usize..last.span.end as usize,
    ))
}

fn unparsed_reason<'i>(
    ast: &AstContext<'i>,
    property_id: &PropertyId<'i>,
    value: &[TokenOrValue<'i>],
    typed_grammar_supported: bool,
) -> UnparsedPropertyReason {
    if matches!(property_id, PropertyId::Custom(_)) {
        return UnparsedPropertyReason::UnknownProperty;
    }
    if value.iter().any(|value| token_value_is_comment(ast, value)) {
        return UnparsedPropertyReason::OpaqueValue;
    }
    if !typed_grammar_supported {
        return UnparsedPropertyReason::UnsupportedGrammar;
    }
    // `background` currently has a typed fast path for color-only values, but
    // its full shorthand grammar is not implemented yet. A failed fast path
    // therefore means "unsupported grammar", not invalid syntax.
    if matches!(property_id, PropertyId::Background) {
        return UnparsedPropertyReason::UnsupportedGrammar;
    }
    if value.iter().any(token_value_is_opaque) {
        return UnparsedPropertyReason::OpaqueValue;
    }
    UnparsedPropertyReason::InvalidValue
}

fn token_value_is_comment<'i>(ast: &AstContext<'i>, value: &TokenOrValue<'i>) -> bool {
    matches!(
        value,
        TokenOrValue::Token(token) if matches!(ast.node(*token), ValueToken::Comment(_))
    )
}

fn token_value_is_opaque(value: &TokenOrValue<'_>) -> bool {
    matches!(
        value,
        TokenOrValue::Function(_) | TokenOrValue::Var(_) | TokenOrValue::Env(_)
    )
}

macro_rules! generate_typed_parser {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:tt)?)
                $([$strategy:ident $( : $($strategy_args:tt)+)?])?,
        )+
    ) => {
        fn try_parse_typed_declaration<'i>(
            input: &mut Compiler<'i>,
            property_id: &PropertyId<'i>,
            allocator: &'i Allocator,
            depth: usize,
        ) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
            $(
                generate_typed_parser!(
                    @dispatch
                    $($strategy $( : $($strategy_args)+ )? )?
                    ;
                    $property, $value;
                    $($vp)?;
                    input, property_id, allocator, depth
                );
            )+
            None
        }
    };

    (@dispatch ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {};
    (@dispatch ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {};
    (@dispatch unsupported ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {};
    (@dispatch unsupported ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {};

    (@dispatch node : CssColor<'i> ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                parse_css_color,
            ).map(Declaration::$property));
        }
    };
    (@dispatch node : CssColor<'i> ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                parse_css_color,
            ).map(|value| Declaration::$property(value, *prefix)));
        }
    };

    (@dispatch parse : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(value)));
        }
    };
    (@dispatch parse : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(value, *prefix)));
        }
    };

    (@dispatch boxed : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input))));
        }
    };
    (@dispatch boxed : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input), *prefix)));
        }
    };

    (@dispatch comma_separated : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| super::values::parse_comma_separated(input, <$parser as Parse>::parse),
            ).map(|value| Declaration::$property(store_vec(value, $input))));
        }
    };
    (@dispatch comma_separated : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| super::values::parse_comma_separated(input, <$parser as Parse>::parse),
            ).map(|value| Declaration::$property(store_vec(value, $input), *prefix)));
        }
    };

    (@dispatch whitespace_separated : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input))));
        }
    };
    (@dispatch whitespace_separated : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input), *prefix)));
        }
    };

    (@dispatch rect : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input))));
        }
    };
    (@dispatch rect : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input), *prefix)));
        }
    };

    (@dispatch two_value : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input))));
        }
    };
    (@dispatch two_value : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(store_node(value, $input), *prefix)));
        }
    };

    (@dispatch custom : $adapter:ident ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return $adapter($input, $allocator, $depth);
        }
    };
    (@dispatch custom : $adapter:ident ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return $adapter($input, $allocator, *prefix, $depth);
        }
    };

    (@dispatch css_wide : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(CSSWideOr::Value(value))));
        }
    };
    (@dispatch css_wide : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(CSSWideOr::Value(value), *prefix)));
        }
    };

    (@dispatch css_wide_boxed : $parser:ty ; $property:ident, $value:ty;; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(CSSWideOr::Value(store_node(value, $input)))));
        }
    };
    (@dispatch css_wide_boxed : $parser:ty ; $property:ident, $value:ty; $vp:tt; $input:ident, $property_id:ident, $allocator:ident, $depth:ident) => {
        if let PropertyId::$property(prefix) = $property_id {
            return Some($input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                <$parser as Parse>::parse,
            ).map(|value| Declaration::$property(CSSWideOr::Value(store_node(value, $input)), *prefix)));
        }
    };
}

rocketcss_ast::for_each_property!(generate_typed_parser);

fn parse_all<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            let ident = input.expect_ident()?;
            css_wide_keyword(ident)
                .map(Declaration::All)
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))
        }),
    )
}

fn parse_background<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            let mut values = allocator.vec();
            let value = Background::parse(input)?;
            values.push(store_node(value, input));
            Ok(Declaration::Background(store_vec(values, input)))
        }),
    )
}

fn parse_opacity<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            let location = input.current_source_location();
            match input.next()?.clone() {
                ValueToken::Number(value) | ValueToken::Percentage(value) => {
                    Ok(Declaration::Opacity(value))
                }
                _ => Err(location.new_custom_error(ParserError::InvalidValue)),
            }
        }),
    )
}

fn parse_font_family<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            let families = parse_font_family_list(input, depth)?;
            if families
                .iter()
                .any(|family| matches!(family, FontFamily::Unparsed(_)))
            {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            Ok(Declaration::FontFamily(store_vec(families, input)))
        }),
    )
}

fn parse_transform<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    prefix: VendorPrefix,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    if value_contains_comment(input) {
        return None;
    }
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return None;
    }
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            parse_transform_list(input)
                .map(|value| Declaration::Transform(store_vec(value, input), prefix))
        }),
    )
}

fn parse_transition<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    prefix: VendorPrefix,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    if value_contains_comment(input) {
        return None;
    }
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            parse_comma_separated(input, Transition::parse)
                .map(|value| Declaration::Transition(store_vec(value, input), prefix))
        }),
    )
}

fn parse_transition_property<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    prefix: VendorPrefix,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            parse_transition_property_list(input)
                .map(|value| Declaration::TransitionProperty(store_vec(value, input), prefix))
        }),
    )
}

fn parse_animation<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    prefix: VendorPrefix,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    if value_contains_comment(input) {
        return None;
    }
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            parse_animation_list(input)
                .map(|value| Declaration::Animation(store_vec(value, input), prefix))
        }),
    )
}

fn parse_animation_name<'i>(
    input: &mut Compiler<'i>,
    _allocator: &'i Allocator,
    prefix: VendorPrefix,
    _depth: usize,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    if value_contains_comment(input) {
        return None;
    }
    Some(
        input.parse_until_before_stop_on_error(Delimiter::Bang | Delimiter::Semicolon, |input| {
            parse_comma_separated(input, AnimationName::parse)
                .map(|value| Declaration::AnimationName(store_vec(value, input), prefix))
        }),
    )
}

macro_rules! prefixed_comma_adapter {
    ($name:ident, $variant:ident, $value:ty) => {
        fn $name<'i>(
            input: &mut Compiler<'i>,
            _allocator: &'i Allocator,
            prefix: VendorPrefix,
            _depth: usize,
        ) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
            if value_contains_comment(input) {
                return None;
            }
            Some(input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| {
                    parse_comma_separated(input, <$value as Parse>::parse)
                        .map(|value| Declaration::$variant(store_vec(value, input), prefix))
                },
            ))
        }
    };
}

prefixed_comma_adapter!(parse_animation_duration, AnimationDuration, Time);
prefixed_comma_adapter!(parse_animation_delay, AnimationDelay, Time);
macro_rules! prefixed_comma_node_adapter {
    ($name:ident, $variant:ident, $value:ty) => {
        fn $name<'i>(
            input: &mut Compiler<'i>,
            allocator: &'i Allocator,
            prefix: VendorPrefix,
            _depth: usize,
        ) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
            if value_contains_comment(input) {
                return None;
            }
            Some(input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| {
                    let parsed = parse_comma_separated(input, <$value as Parse>::parse)?;
                    let mut values = allocator.vec();
                    for value in parsed {
                        values.push(store_node(value, input));
                    }
                    Ok(Declaration::$variant(store_vec(values, input), prefix))
                },
            ))
        }
    };
}

prefixed_comma_node_adapter!(
    parse_animation_timing,
    AnimationTimingFunction,
    EasingFunction
);
prefixed_comma_adapter!(
    parse_animation_iteration,
    AnimationIterationCount,
    AnimationIterationCount
);
prefixed_comma_adapter!(
    parse_animation_direction,
    AnimationDirection,
    AnimationDirection
);
prefixed_comma_adapter!(parse_animation_fill, AnimationFillMode, AnimationFillMode);
prefixed_comma_adapter!(
    parse_animation_play_state,
    AnimationPlayState,
    AnimationPlayState
);
prefixed_comma_adapter!(parse_transition_duration, TransitionDuration, Time);
prefixed_comma_adapter!(parse_transition_delay, TransitionDelay, Time);
prefixed_comma_node_adapter!(
    parse_transition_timing,
    TransitionTimingFunction,
    EasingFunction
);

macro_rules! prefixed_number_adapter {
    ($name:ident, $variant:ident) => {
        fn $name<'i>(
            input: &mut Compiler<'i>,
            _allocator: &'i Allocator,
            prefix: VendorPrefix,
            _depth: usize,
        ) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
            Some(input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| {
                    let value = input.expect_number()?;
                    Ok(Declaration::$variant(value, prefix))
                },
            ))
        }
    };
}

prefixed_number_adapter!(parse_flex_grow, FlexGrow);
prefixed_number_adapter!(parse_flex_shrink, FlexShrink);
prefixed_number_adapter!(parse_order, Order);
prefixed_number_adapter!(parse_box_ordinal_group, BoxOrdinalGroup);
prefixed_number_adapter!(parse_box_flex, BoxFlex);
prefixed_number_adapter!(parse_box_flex_group, BoxFlexGroup);
prefixed_number_adapter!(parse_flex_order, FlexOrder);
prefixed_number_adapter!(parse_flex_positive, FlexPositive);
prefixed_number_adapter!(parse_flex_negative, FlexNegative);

macro_rules! number_adapter {
    ($name:ident, $variant:ident) => {
        fn $name<'i>(
            input: &mut Compiler<'i>,
            _allocator: &'i Allocator,
            _depth: usize,
        ) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
            Some(input.parse_until_before_stop_on_error(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| {
                    let value = input.expect_number()?;
                    Ok(Declaration::$variant(value))
                },
            ))
        }
    };
}

number_adapter!(parse_fill_opacity, FillOpacity);
number_adapter!(parse_stroke_opacity, StrokeOpacity);
number_adapter!(parse_stroke_miterlimit, StrokeMiterlimit);

fn parse_declaration_end<'i>(input: &mut Compiler<'i>) -> Option<bool> {
    let important = input
        .try_parse(|input| {
            input.expect_delim('!')?;
            input.expect_ident_matching("important")
        })
        .is_ok();
    input
        .parse_until_before(Delimiter::Semicolon, |input| {
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })
        .ok()
        .map(|()| important)
}

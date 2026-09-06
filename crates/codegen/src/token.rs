use crate::prelude::*;
use std::borrow::Borrow;

impl<'ghost> ToCss<'ghost> for Unit {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(unit) => unit.to_css(dest, _cx),
            Self::Deg => dest.write_str("deg"),
            Self::Rad => dest.write_str("rad"),
            Self::Grad => dest.write_str("grad"),
            Self::Turn => dest.write_str("turn"),
            Self::Seconds => dest.write_str("s"),
            Self::Milliseconds => dest.write_str("ms"),
            Self::Hertz => dest.write_str("hz"),
            Self::Kilohertz => dest.write_str("khz"),
            Self::Dpi => dest.write_str("dpi"),
            Self::Dpcm => dest.write_str("dpcm"),
            Self::Dppx => dest.write_str("dppx"),
            Self::ResolutionX => dest.write_str("x"),
            Self::Flex => dest.write_str("fr"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Token<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        use cssparser::{CowRcStr, ToCss as CssParserToCss, Token as CssToken};

        match self {
            Self::Ident(value) => {
                CssToken::Ident(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::AtKeyword(value) => {
                CssToken::AtKeyword(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::Hash(value) => {
                CssToken::Hash(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::IdHash(value) => {
                CssToken::IDHash(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::MinifiedHash(value) => write_minified_hash(_cx.ast_context().str(*value), dest),
            Self::String(value) => {
                CssToken::QuotedString(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::UnquotedFont(value) => write_unquoted_font(_cx.ast_context().str(*value), dest),
            Self::UnquotedUrl(value) => {
                CssToken::UnquotedUrl(CowRcStr::from(_cx.ast_context().str(*value))).to_css(dest)
            }
            Self::Delim(value) => {
                for character in _cx.ast_context().str(*value).chars() {
                    CssToken::Delim(character).to_css(dest)?;
                }
                Ok(())
            }
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::Dimension { unit, value } => serialize_dimension(*value, unit, dest, _cx),
            Self::UnknownDimension { unit, value } => {
                serialize_number(*value, dest)?;
                dest.write_str(_cx.ast_context().str(*unit))
            }
            Self::WhiteSpace(value) => {
                if dest.prettify() {
                    dest.write_str(_cx.ast_context().str(*value))
                } else {
                    dest.write_char(' ')
                }
            }
            Self::Comment(value) => {
                if !dest.prettify() {
                    return Ok(());
                }
                dest.write_str("/*")?;
                dest.write_str(_cx.ast_context().str(*value))?;
                dest.write_str("*/")
            }
            Self::Colon => dest.write_char(':'),
            Self::Semicolon => dest.write_char(';'),
            Self::Comma => dest.write_char(','),
            Self::IncludeMatch => dest.write_str("~="),
            Self::DashMatch => dest.write_str("|="),
            Self::PrefixMatch => dest.write_str("^="),
            Self::SuffixMatch => dest.write_str("$="),
            Self::SubstringMatch => dest.write_str("*="),
            Self::Cdo => dest.write_str("<!--"),
            Self::Cdc => dest.write_str("-->"),
            Self::Function(value) => {
                serialize_identifier(_cx.ast_context().str(*value), dest)?;
                dest.write_char('(')
            }
            Self::ParenthesisBlock => dest.write_char('('),
            Self::SquareBracketBlock => dest.write_char('['),
            Self::CurlyBracketBlock => dest.write_char('{'),
            Self::BadUrl(value) => {
                dest.write_str("url(")?;
                dest.write_str(_cx.ast_context().str(*value))
            }
            Self::BadString(value) => dest.write_str(_cx.ast_context().str(*value)),
            Self::CloseParenthesis => dest.write_char(')'),
            Self::CloseSquareBracket => dest.write_char(']'),
            Self::CloseCurlyBracket => dest.write_char('}'),
        }
    }
}

fn write_unquoted_font<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    let mut characters = value.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if character == ' ' {
            if characters.peek().is_none() {
                dest.write_char('\\')?;
            } else if index == 0
                || characters
                    .peek()
                    .is_some_and(|(_, next)| next.is_ascii_digit())
            {
                dest.write_str("\\ ")?;
            } else {
                dest.write_char(' ')?;
            }
        } else if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
            dest.write_char(character)?;
        } else {
            dest.write_char('\\')?;
            dest.write_char(character)?;
        }
    }
    Ok(())
}

fn write_minified_hash<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    let bytes = value.as_bytes();
    let length = match bytes.len() {
        8 if bytes[6].eq_ignore_ascii_case(&b'f') && bytes[7].eq_ignore_ascii_case(&b'f') => 6,
        4 if bytes[3].eq_ignore_ascii_case(&b'f') => 3,
        length => length,
    };
    let collapse_pairs = matches!(length, 6 | 8)
        && bytes[..length]
            .as_chunks::<2>()
            .0
            .iter()
            .all(|pair| pair[0].eq_ignore_ascii_case(&pair[1]));

    dest.write_char('#')?;
    let step = if collapse_pairs { 2 } else { 1 };
    for index in (0..length).step_by(step) {
        dest.write_char((bytes[index] as char).to_ascii_lowercase())?;
    }
    Ok(())
}

pub(crate) fn write_token_list<'ast, 'ghost, PrinterT, I>(
    values: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: std::borrow::Borrow<TokenOrValue<'ast>>,
{
    let mut needs_separator = false;
    for current in values {
        match current.borrow() {
            TokenOrValue::Token(id) => {
                let token = cx.ast_context().resolve_node(*id);
                if needs_separator
                    && !matches!(
                        token,
                        Token::WhiteSpace(_)
                            | Token::Comma
                            | Token::Semicolon
                            | Token::CloseParenthesis
                            | Token::CloseSquareBracket
                            | Token::CloseCurlyBracket
                    )
                {
                    dest.write_char(' ')?;
                }
                token.to_css(dest, cx)?;
                needs_separator = false;
            }
            TokenOrValue::Function(id) => {
                if needs_separator {
                    dest.write_char(' ')?;
                }
                needs_separator = crate::rules::stylesheet::write_stored_function(*id, dest, cx)?;
            }
            value => {
                if needs_separator {
                    dest.write_char(' ')?;
                }
                value.to_css(dest, cx)?;
                needs_separator = false;
            }
        }
    }
    Ok(())
}

/// Serialize a fallback value without applying any value-level normalization.
///
/// Unparsed declarations are semantic barriers. In particular, dropping a
/// comment between two tokens or using a cached function replacement can join
/// tokens that the parser deliberately kept opaque.
pub(crate) fn write_unparsed_token_list<'ast, 'ghost, PrinterT, I>(
    values: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator,
    I::Item: std::borrow::Borrow<TokenOrValue<'ast>>,
{
    for value in values {
        write_unparsed_token_or_value(value.borrow(), dest, cx)?;
    }
    Ok(())
}

fn write_unparsed_token_or_value<'ghost, PrinterT: PrinterTrait>(
    value: &TokenOrValue<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast_context();
    match value {
        TokenOrValue::Token(token) => match ast.resolve_node(*token) {
            Token::Comment(comment) => {
                dest.write_str("/*")?;
                dest.write_str(ast.str(comment))?;
                dest.write_str("*/")
            }
            Token::WhiteSpace(whitespace) => dest.write_str(ast.str(whitespace)),
            token => token.to_css(dest, cx),
        },
        TokenOrValue::Function(function) => {
            let function = ast.function(*function);
            let arguments = ast.vec_iter(function.arguments());
            dest.write_str(ast.str(function.name()))?;
            dest.write_char('(')?;
            write_unparsed_token_list(arguments, dest, cx)?;
            if function.kind().is_variable() && token_list_ends_with_comma(function.arguments(), cx)
            {
                dest.write_char(' ')?;
            }
            dest.write_char(')')
        }
        value => value.to_css(dest, cx),
    }
}

/// Empty var/env fallbacks require a trailing space. Read only the last token
/// instead of decoding each argument a second time while serializing the list.
pub(crate) fn token_list_ends_with_comma(
    values: AstVec<'_, TokenOrValue<'_>>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    let ast = cx.ast_context();
    matches!(
        values.len().checked_sub(1).and_then(|index| ast.vec_get(values, index)),
        Some(TokenOrValue::Token(token)) if matches!(ast.resolve_node(token), Token::Comma)
    )
}

pub(crate) fn write_token_list_without_outer_whitespace<'ghost, PrinterT: PrinterTrait>(
    values: AstVec<'_, TokenOrValue<'_>>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast_context();
    let is_whitespace = |index| matches!(ast.vec_get(values, index), Some(TokenOrValue::Token(token)) if matches!(ast.resolve_node(token), Token::WhiteSpace(_)));
    let start = (0..values.len())
        .find(|&index| !is_whitespace(index))
        .unwrap_or(values.len());
    let end = (start..values.len())
        .rev()
        .find(|&index| !is_whitespace(index))
        .map_or(start, |index| index + 1);
    write_token_list(ast.vec_iter(values).skip(start).take(end - start), dest, cx)
}

impl<'ghost> ToCss<'ghost> for TokenOrValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Token(value) => value.to_css(dest, _cx),
            Self::Color(value) => value.to_css(dest, _cx),
            Self::UnresolvedColor(value) => value.to_css(dest, _cx),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Var(value) => value.to_css(dest, _cx),
            Self::Env(value) => value.to_css(dest, _cx),
            Self::Function(value) => value.to_css(dest, _cx),
            Self::Length(value) => serialize_dimension(value.value, &value.unit, dest, _cx),
            Self::Angle(value) => value.to_css(dest, _cx),
            Self::Time(value) => value.to_css(dest, _cx),
            Self::Resolution(value) => value.to_css(dest, _cx),
            Self::DashedIdent(value) => value.to_css(dest, _cx),
            Self::AnimationName(value) => value.to_css(dest, _cx),
        }
    }
}

pub(crate) fn write_dashed_ident<PrinterT: PrinterTrait>(
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str("--")?;
    serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
}

impl<'ghost> ToCss<'ghost> for EnvironmentVariableName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::UA(value) => value.to_css(dest, _cx),
            Self::Custom(value) => value.to_css(dest, _cx),
            Self::Unknown(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for UAEnvironmentVariable {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("UA environment variables are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for Specifier<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Global => dest.write_str("global"),
            Self::File(value) => serialize_string(_cx.ast_context().str(*value), dest),
            Self::SourceIndex(_) => Ok(()),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AnimationName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Ident(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
            Self::String(value) => {
                let value = _cx.ast_context().str(*value);
                rocketcss_ast::match_ignore_ascii_case!(value,
                    "none" | "initial" | "inherit" | "unset" | "default" | "revert" | "revert-layer" => serialize_string(value, dest),
                    _ => serialize_identifier(value, dest),
                )
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for DashedIdent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_dashed_ident(cx.ast_context().str(self.value), dest)
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use rocketcss_common::{Allocator, GhostToken};

    #[test]
    fn range_trimming_preserves_internal_whitespace_and_comment_boundaries() {
        GhostToken::scope(|token| {
            let allocator = Allocator::new();
            let mut ast = AstContext::new_in(&allocator);
            let space = ast.add_str(" \t");
            let ident = ast.add_str("a");
            let comment = ast.add_str("x");
            for (tokens, compact, pretty) in [
                (std::vec![], "", ""),
                (
                    std::vec![Token::WhiteSpace(space), Token::WhiteSpace(space)],
                    "",
                    "",
                ),
                (
                    std::vec![
                        Token::WhiteSpace(space),
                        Token::Ident(ident),
                        Token::WhiteSpace(space)
                    ],
                    "a",
                    "a",
                ),
                (
                    std::vec![
                        Token::Ident(ident),
                        Token::WhiteSpace(space),
                        Token::Ident(ident)
                    ],
                    "a a",
                    "a \ta",
                ),
                (
                    std::vec![
                        Token::WhiteSpace(space),
                        Token::Comment(comment),
                        Token::Ident(ident),
                        Token::Comment(comment),
                        Token::WhiteSpace(space)
                    ],
                    "a",
                    "/*x*/a/*x*/",
                ),
            ] {
                let mut values = allocator.vec();
                values.extend(
                    tokens
                        .into_iter()
                        .map(|value| TokenOrValue::Token(ast.alloc_node(value, DUMMY_SP))),
                );
                let range = ast.alloc_vec(values);
                let checkpoint = ast.node_checkpoint();
                let bytes = ast.string_pool().extra_len();
                for (prettify, expected) in [(false, compact), (true, pretty)] {
                    let cx = ToCssContext::with_ast(&token, &ast);
                    let mut output = String::new();
                    write_token_list_without_outer_whitespace(
                        range,
                        &mut Printer::new(&mut output, PrinterOptions { prettify }),
                        &cx,
                    )
                    .unwrap();
                    assert_eq!(output, expected);
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), bytes);
            }
        });
    }

    fn copy_tail<'a>(value: &TokenOrValue<'a>) -> TokenOrValue<'a> {
        match value {
            TokenOrValue::Token(id) => TokenOrValue::Token(*id),
            TokenOrValue::Function(id) => TokenOrValue::Function(*id),
            TokenOrValue::Time(time) => TokenOrValue::Time(*time),
            _ => unreachable!("test tail must be a token, function or time"),
        }
    }

    #[test]
    fn color_replacement_boundaries_preserve_tokens_and_iterators() {
        GhostToken::scope(|token| {
            let allocator = Allocator::new();
            let mut ast = AstContext::new_in(&allocator);
            let text = ast.add_str("x");
            let space = ast.add_str(" ");
            let empty = ast.alloc_vec(allocator.vec::<TokenOrValue<'_>>());
            let function = Function::new("rgb", empty, &mut ast);
            let left = ast.alloc_node(function, DUMMY_SP);
            let function = Function::new("f", empty, &mut ast);
            let next_function = ast.alloc_node(function, DUMMY_SP);
            let mut tails = std::vec::Vec::new();
            for (value, needs_space) in [
                (Token::Ident(text), true),
                (Token::WhiteSpace(space), false),
                (Token::Comma, false),
                (Token::Semicolon, false),
                (Token::CloseParenthesis, false),
                (Token::CloseSquareBracket, false),
                (Token::CloseCurlyBracket, false),
                (Token::Colon, true),
                (Token::Comment(text), true),
                (Token::Number(1.0), true),
                (Token::Function(text), true),
            ] {
                tails.push((
                    TokenOrValue::Token(ast.alloc_node(value, DUMMY_SP)),
                    needs_space,
                ));
            }
            tails.push((TokenOrValue::Function(next_function), true));
            tails.push((TokenOrValue::Time(Time::Seconds(1.0)), true));
            for (replacement, color_boundary) in [
                (None, false),
                (
                    Some(FunctionReplacement::Rgb {
                        red: 0,
                        green: 0,
                        blue: 0,
                    }),
                    true,
                ),
                (
                    Some(FunctionReplacement::Rgba {
                        red: 0,
                        green: 0,
                        blue: 0,
                        alpha: 0.0,
                        use_hex: false,
                    }),
                    true,
                ),
                (
                    Some(FunctionReplacement::Rgba {
                        red: 0,
                        green: 0,
                        blue: 0,
                        alpha: 0.5,
                        use_hex: true,
                    }),
                    true,
                ),
                (
                    Some(FunctionReplacement::Rgba {
                        red: 0,
                        green: 0,
                        blue: 0,
                        alpha: 0.5,
                        use_hex: false,
                    }),
                    false,
                ),
                (
                    Some(FunctionReplacement::GrayAlpha {
                        lightness: 0.5,
                        alpha: 0.5,
                    }),
                    false,
                ),
                (Some(FunctionReplacement::Number(3.0)), false),
            ] {
                ast.mutate_node(left, |value, _| value.replacement = replacement);
                let checkpoint = ast.node_checkpoint();
                let pool_len = ast.string_pool().extra_len();
                let cx = ToCssContext::with_ast(&token, &ast);
                for prettify in [false, true] {
                    let options = PrinterOptions { prettify };
                    let prefix = ast.resolve_node(left).to_css_string(options, &cx).unwrap();
                    for (tail, needs_space) in &tails {
                        let suffix = tail.to_css_string(options, &cx).unwrap();
                        let expected = format!(
                            "{prefix}{}{suffix}",
                            if color_boundary && *needs_space {
                                " "
                            } else {
                                ""
                            }
                        );
                        for borrowed in [false, true] {
                            let values = [TokenOrValue::Function(left), copy_tail(tail)];
                            let mut output = String::new();
                            let mut printer = Printer::new(&mut output, options);
                            if borrowed {
                                write_token_list(&values, &mut printer, &cx).unwrap();
                            } else {
                                write_token_list(values, &mut printer, &cx).unwrap();
                            }
                            assert_eq!(output, expected);
                        }
                        // A completed tail must clear the preceding color boundary.
                        let mut output = String::new();
                        write_token_list(
                            [
                                TokenOrValue::Function(left),
                                copy_tail(tail),
                                copy_tail(&tails[0].0),
                            ],
                            &mut Printer::new(&mut output, options),
                            &cx,
                        )
                        .unwrap();
                        assert_eq!(output, format!("{expected}x"));
                    }
                    let mut output = String::new();
                    write_token_list(
                        std::iter::empty::<TokenOrValue<'_>>(),
                        &mut Printer::new(&mut output, options),
                        &cx,
                    )
                    .unwrap();
                    assert!(output.is_empty());
                }
                assert_eq!(ast.node_checkpoint(), checkpoint);
                assert_eq!(ast.string_pool().extra_len(), pool_len);
            }
        });
    }
}

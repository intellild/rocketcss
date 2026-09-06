use crate::parser::media::{
    parse_media_condition_or_unknown, parse_media_type, parse_qualifier, parse_supports_condition,
};
mod compilation;

use super::*;

pub(in crate::parser) fn parse_layer_names<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<Vec<'i, AstVec<'i, AstStr<'i>>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let mut name = allocator.vec();
            let part = input.expect_ident()?;
            name.push(input.add_str(part));
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                let part = input.expect_ident()?;
                name.push(input.add_str(part));
            }
            input.expect_exhausted()?;
            Ok(store_vec(name, input))
        })?;
        let mut names = allocator.vec();
        names.extend(parsed);
        Ok(names)
    })
}

pub(in crate::parser) fn validate_moz_document_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, allocator);
    parser.expect_function_matching("url-prefix")?;
    parser.parse_nested_block(|input| {
        if !input.is_exhausted() && !input.expect_string()?.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.expect_exhausted()?;
        Ok(())
    })?;
    parser.expect_exhausted()?;
    Ok(())
}

type ScopePrelude<'i> = (Option<SelectorList<'i>>, Option<SelectorList<'i>>);

pub(in crate::parser) fn parse_scope_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
    depth: usize,
) -> Result<ScopePrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let scope_start = if input.try_parse(Compiler::expect_parenthesis_block).is_ok() {
            Some(
                input
                    .parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
            )
        } else {
            None
        };

        let scope_end = if input
            .try_parse(|input| input.expect_ident_matching("to"))
            .is_ok()
        {
            input.expect_parenthesis_block()?;
            Some(
                input
                    .parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
            )
        } else {
            None
        };
        input.expect_exhausted()?;
        Ok((scope_start, scope_end))
    })
}

pub(in crate::parser) const MAX_NESTING_DEPTH: usize = 500;

/// Parses a stylesheet using the span-only tokenizer and arena-backed AST.
pub fn parse<'i, 'ghost>(
    source: &'i str,
    allocator: &'i Allocator,
    token: &mut GhostToken<'ghost>,
    options: ParserOptions<'i>,
) -> Result<AstContext<'i>, Error<'i>> {
    Compiler::new(allocator).parse(source, token, options)
}

pub(in crate::parser) fn check_depth<'i>(
    input: &Compiler<'i>,
    depth: usize,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if depth > MAX_NESTING_DEPTH {
        Err(input.new_custom_error(ParserError::MaximumNestingDepth))
    } else {
        Ok(())
    }
}

pub(in crate::parser) fn span_from(start: &ParserState, end: SourcePosition) -> Span {
    Span::new(
        start.position().byte_index() as u32,
        end.byte_index() as u32,
    )
}

pub(in crate::parser) fn recover_rule(input: &mut Compiler<'_>) {
    let _ = input.next_including_whitespace_and_comments();
}

pub(in crate::parser) fn recover_declaration(input: &mut Compiler<'_>) {
    let _: Result<(), ParseError<'_, ()>> =
        input.parse_until_after(Delimiter::Semicolon, |_| Ok(()));
}

pub(in crate::parser) fn into_error<'i>(
    error: ParseError<'i, ParserError<'i>>,
    filename: &'i str,
) -> Error<'i> {
    let kind = match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => {
            ParserError::UnexpectedToken(token)
        }
        ParseErrorKind::Basic(BasicParseErrorKind::AtRuleInvalid(name)) => {
            ParserError::InvalidAtRule(name)
        }
        ParseErrorKind::Basic(_) => ParserError::InvalidRule,
    };
    Error {
        kind,
        filename,
        location: error.location,
    }
}

pub(in crate::parser) fn parse_import_rule<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<ImportRule<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let url = input.expect_url_or_string()?;
        let url = input.add_str(url);

        let layer = if input
            .try_parse(|input| input.expect_ident_matching("layer"))
            .is_ok()
        {
            Some(allocator.vec())
        } else if input
            .try_parse(|input| input.expect_function_matching("layer"))
            .is_ok()
        {
            Some(input.parse_nested_block(|input| {
                let mut name = allocator.vec();
                let part = input.expect_ident()?;
                name.push(input.add_str(part));
                while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                    let part = input.expect_ident()?;
                    name.push(input.add_str(part));
                }
                input.expect_exhausted()?;
                Ok::<_, ParseError<'i, ParserError<'i>>>(name)
            })?)
        } else {
            None
        };
        let layer = layer.map(|layer| store_vec(layer, input));

        let supports = if input
            .try_parse(|input| input.expect_function_matching("supports"))
            .is_ok()
        {
            Some(store_node(
                input.parse_nested_block(|input| {
                    let start = input.position();
                    input.expect_no_error_token()?;
                    let raw = input.slice_from(start).trim();
                    if raw.is_empty() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    Ok::<_, ParseError<'i, ParserError<'i>>>(parse_supports_condition(
                        input.add_str(raw),
                    ))
                })?,
                input,
            ))
        } else {
            None
        };

        let media = if input.is_exhausted() {
            None
        } else {
            let rest = input
                .slice(input.position()..SourcePosition(prelude.len()))
                .trim();
            if rest.is_empty() {
                None
            } else {
                let media = parse_media_list(input, rest)?;
                Some(store_node(media, input))
            }
        };
        Ok(ImportRule {
            layer,
            media,
            supports,
            url,
        })
    })
}

pub(in crate::parser) fn parse_media_list<'i>(
    input: &mut Compiler<'i>,
    source: &'i str,
) -> Result<MediaList<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    if source.trim().is_empty() {
        return Ok(MediaList {
            media_queries: store_vec(allocator.vec(), input),
        });
    }
    input.with_source(source, |input| {
        let parsed = input.parse_comma_separated(|input| {
            input
                .try_parse(|input| parse_media_query(input, allocator))
                .or_else(|_| parse_unknown_media_query(input, allocator))
        })?;
        let mut media_queries = allocator.vec();
        for query in parsed {
            media_queries.push(store_node(query, input));
        }
        Ok(MediaList {
            media_queries: store_vec(media_queries, input),
        })
    })
}

fn parse_media_query<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
) -> Result<MediaQuery<'i>, ParseError<'i, ParserError<'i>>> {
    // As in Lightning CSS, parse the qualifier and media type together. This
    // is important for `not (color)`: `not` is part of the condition there,
    // not a media query qualifier.
    let explicit = input
        .try_parse(|input| {
            let qualifier = input.try_parse(parse_qualifier).ok();
            let media_type = parse_media_type(input)?;
            Ok::<_, ParseError<'i, ParserError<'i>>>((qualifier, media_type))
        })
        .ok();

    let (qualifier, media_type, condition) = if let Some((qualifier, media_type)) = explicit {
        let condition = if input.is_exhausted() {
            None
        } else {
            input.expect_ident_matching("and")?;
            Some(parse_media_condition_or_unknown(input, allocator, false)?)
        };
        (qualifier, media_type, condition)
    } else {
        (
            None,
            MediaType::All,
            Some(parse_media_condition_or_unknown(input, allocator, true)?),
        )
    };

    let condition = condition.map(|condition| store_node(condition, input));
    input.expect_exhausted()?;
    Ok(MediaQuery {
        condition,
        media_type,
        qualifier,
    })
}

fn parse_unknown_media_query<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
) -> Result<MediaQuery<'i>, ParseError<'i, ParserError<'i>>> {
    Ok(MediaQuery {
        condition: Some(store_node(
            MediaCondition::Unknown(store_vec(collect_tokens(input, allocator, 0)?, input)),
            input,
        )),
        media_type: MediaType::All,
        qualifier: None,
    })
}

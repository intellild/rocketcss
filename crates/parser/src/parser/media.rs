use super::{stylesheet::span_from, values::collect_tokens};
use crate::prelude::*;

pub(super) fn parse_import<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
    start: &ParserState,
    end: SourcePosition,
) -> Result<CssRule<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let url = parser.expect_url_or_string()?;

    let layer = if parser
        .try_parse(|input| input.expect_ident_matching("layer"))
        .is_ok()
    {
        Some(allocator.vec())
    } else if parser
        .try_parse(|input| input.expect_function_matching("layer"))
        .is_ok()
    {
        Some(parser.parse_nested_block(|input| {
            let mut name = allocator.vec();
            name.push(input.expect_ident()?);
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                name.push(input.expect_ident()?);
            }
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(name)
        })?)
    } else {
        None
    };

    let supports = if parser
        .try_parse(|input| input.expect_function_matching("supports"))
        .is_ok()
    {
        Some(allocator.boxed(parser.parse_nested_block(|input| {
            let start = input.position();
            input.expect_no_error_token()?;
            let raw = input.slice_from(start).trim();
            if raw.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            Ok::<_, ParseError<'i, ParserError<'i>>>(parse_supports_condition(raw, allocator))
        })?))
    } else {
        None
    };

    let media = if parser.is_exhausted() {
        None
    } else {
        let rest = parser
            .slice(parser.position()..SourcePosition(prelude.len()))
            .trim();
        if rest.is_empty() {
            None
        } else {
            Some(allocator.boxed(parse_media_list(rest, allocator)?))
        }
    };
    Ok(CssRule::Import(allocator.boxed(ImportRule {
        layer,
        span: span_from(start, end),
        media,
        supports,
        url,
    })))
}

pub(super) fn parse_media_list<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> Result<MediaList<'i>, ParseError<'i, ParserError<'i>>> {
    if source.trim().is_empty() {
        return Ok(MediaList {
            media_queries: allocator.vec(),
        });
    }
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let qualifier = input
            .try_parse(|input| {
                let name = input.expect_ident()?;
                match_ignore_ascii_case!(
                    name,
                    "only" => Ok::<_, ParseError<'i, ParserError<'i>>>(Qualifier::Only),
                    "not" => Ok(Qualifier::Not),
                    _ => Err(input.new_custom_error(ParserError::InvalidValue)),
                )
            })
            .ok();

        let type_state = input.state();
        let media_type = match input.try_parse(Parser::expect_ident) {
            Ok(name) => match_ignore_ascii_case!(
                name,
                "all" => MediaType::All,
                "print" => MediaType::Print,
                "screen" => MediaType::Screen,
                "and" | "or" => {
                    input.reset(&type_state);
                    MediaType::All
                },
                _ => MediaType::Custom(name),
            ),
            Err(_) => {
                input.reset(&type_state);
                MediaType::All
            }
        };

        let condition = if input.is_exhausted() {
            None
        } else {
            Some(allocator.boxed(MediaCondition::Unknown(collect_tokens(
                input, allocator, 0,
            )?)))
        };
        Ok(MediaQuery {
            condition,
            media_type: allocator.boxed(media_type),
            qualifier,
        })
    })?;
    let mut media_queries = allocator.vec();
    media_queries.extend(parsed);
    Ok(MediaList { media_queries })
}

pub(super) fn parse_supports_condition<'i>(
    source: &str,
    allocator: &'i Allocator,
) -> SupportsCondition<'i> {
    SupportsCondition::Unknown(allocator.alloc_str(source))
}

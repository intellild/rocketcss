use super::*;

pub(in crate::parser) fn parse_namespace<'i>(
    prelude: &'i str,
    input: &mut Compiler<'i>,
) -> Result<(Option<AstStr<'i>>, AstStr<'i>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let state = input.state();
        if let Ok(prefix) = input.try_parse(Compiler::expect_ident)
            && let Ok(url) = input.expect_url_or_string()
        {
            input.expect_exhausted()?;
            return Ok((Some(input.add_str(prefix)), input.add_str(url)));
        }
        input.reset(&state);
        let url = input.expect_url_or_string()?;
        input.expect_exhausted()?;
        Ok((None, input.add_str(url)))
    })
}

pub(in crate::parser) fn parse_charset<'i>(
    prelude: &'i str,
    input: &mut Compiler<'i>,
) -> Result<AstStr<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let encoding = input.expect_string()?;
        input.expect_exhausted()?;
        Ok(input.add_str(encoding))
    })
}

pub(in crate::parser) fn parse_custom_media<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(AstStr<'i>, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let name = input.expect_ident()?;
        if !name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let query = input
            .slice(input.position()..SourcePosition(prelude.len()))
            .trim();
        if query.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        // Keep custom media definitions lossless until custom-media expansion is
        // implemented. Parsing these into normalized range features would change
        // their public serialization even though this crate does not consume the
        // definition yet.
        let tokens = input.with_source(query, |input| collect_tokens(input, allocator, 0))?;
        let condition = MediaCondition::Unknown(store_vec(tokens, input));
        let mut media_queries = allocator.vec();
        media_queries.push(store_node(
            MediaQuery {
                condition: Some(store_node(condition, input)),
                media_type: MediaType::All,
                qualifier: None,
            },
            input,
        ));
        Ok((
            input.add_str(name),
            MediaList {
                media_queries: store_vec(media_queries, input),
            },
        ))
    })
}

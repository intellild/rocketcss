use super::css_rule::parse_rule_list;
use crate::prelude::*;

pub(super) const MAX_NESTING_DEPTH: usize = 500;

/// Parses a stylesheet into compilation-owned source-order stores.
pub fn parse<'i, 'ghost>(
    source: &'i str,
    token: &mut GhostToken<'ghost>,
    options: ParserOptions<'i>,
) -> Result<Compilation<'i>, Error<'i>> {
    Compiler::new().parse(source, token, options)
}

pub(crate) fn parse_stylesheet<'i, 'ghost>(
    compiler: &mut Compiler<'i>,
    token: &mut GhostToken<'ghost>,
    options: ParserOptions<'i>,
) -> Result<StyleSheet<'i>, Error<'i>> {
    let mut license_comments = std::vec::Vec::new();

    let mut state = compiler.state();
    while let Ok(token) = compiler.next_including_whitespace_and_comments() {
        match token {
            ValueToken::WhiteSpace(_) => {}
            ValueToken::Comment(comment) if comment.starts_with('!') => {
                license_comments.push(*comment);
            }
            _ => break,
        }
        state = compiler.state();
    }
    compiler.reset(&state);

    let rules = parse_rule_list(compiler, token, &options, 0)
        .map_err(|error| into_error(error, options.filename))?;

    Ok(StyleSheet {
        license_comments,
        rules,
    })
}

pub(super) fn check_depth<'i>(
    input: &Compiler<'i>,
    depth: usize,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if depth > MAX_NESTING_DEPTH {
        Err(input.new_custom_error(ParserError::MaximumNestingDepth))
    } else {
        Ok(())
    }
}

pub(super) fn span_from(start: &ParserState, end: SourcePosition) -> Span {
    Span::new(
        start.position().byte_index() as u32,
        end.byte_index() as u32,
    )
}

pub(super) fn recover_rule(input: &mut Compiler<'_>) {
    let _ = input.next_including_whitespace_and_comments();
}

pub(super) fn recover_declaration(input: &mut Compiler<'_>) {
    let _: Result<(), ParseError<'_, ()>> =
        input.parse_until_after(Delimiter::Semicolon, |_| Ok(()));
}

pub(super) fn into_error<'i>(
    error: ParseError<'i, ParserError<'i>>,
    filename: &'i str,
) -> Error<'i> {
    let kind = match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => {
            ParserError::UnexpectedToken(token)
        }
        ParseErrorKind::Basic(BasicParseErrorKind::AtRuleInvalid(name)) => {
            ParserError::InvalidAtRule(name.to_owned().into())
        }
        ParseErrorKind::Basic(_) => ParserError::InvalidRule,
    };
    Error {
        kind,
        filename,
        location: error.location,
    }
}

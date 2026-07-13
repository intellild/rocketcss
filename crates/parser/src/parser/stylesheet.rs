use super::css_rule::parse_rule_list;
use crate::prelude::*;

pub(super) const MAX_NESTING_DEPTH: usize = 500;

/// A CSS compiler context whose inputs share one lifetime.
///
/// Keeping the source, allocator, and options together makes it possible for
/// every reference in the returned AST to use the same lifetime.
pub struct Compiler<'a> {
    source: &'a str,
    allocator: &'a Allocator,
    options: ParserOptions<'a>,
    source_map_url: Option<&'a str>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str, allocator: &'a Allocator, options: ParserOptions<'a>) -> Self {
        Self {
            source,
            allocator,
            options,
            source_map_url: None,
        }
    }

    pub fn parse(&mut self) -> Result<StyleSheet<'a>, Error<'a>> {
        let (stylesheet, source_map_url) =
            parse_stylesheet(self.source, self.allocator, self.options)?;
        self.source_map_url = source_map_url;
        Ok(stylesheet)
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn allocator(&self) -> &'a Allocator {
        self.allocator
    }

    pub fn options(&self) -> &ParserOptions<'a> {
        &self.options
    }

    pub fn source_map_url(&self) -> Option<&'a str> {
        self.source_map_url
    }
}

/// Parses a stylesheet using a temporary [`Compiler`].
pub fn parse<'a>(
    source: &'a str,
    allocator: &'a Allocator,
    options: ParserOptions<'a>,
) -> Result<StyleSheet<'a>, Error<'a>> {
    Compiler::new(source, allocator, options).parse()
}

/// Parses a stylesheet using the span-only tokenizer and arena-backed AST.
fn parse_stylesheet<'i>(
    source: &'i str,
    allocator: &'i Allocator,
    options: ParserOptions<'i>,
) -> Result<(StyleSheet<'i>, Option<&'i str>), Error<'i>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let mut license_comments = allocator.vec();

    let mut state = parser.state();
    while let Ok(token) = parser.next_including_whitespace_and_comments() {
        match token {
            ValueToken::WhiteSpace(_) => {}
            ValueToken::Comment(comment) if comment.starts_with('!') => {
                license_comments.push(*comment);
            }
            _ => break,
        }
        state = parser.state();
    }
    parser.reset(&state);

    let rules = parse_rule_list(&mut parser, allocator, &options, 0)
        .map_err(|error| into_error(error, options.filename))?;
    let source_map_url = parser.current_source_map_url();

    Ok((
        StyleSheet {
            license_comments,
            rules,
        },
        source_map_url,
    ))
}

pub(super) fn check_depth<'i>(
    input: &Parser<'i, '_>,
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

pub(super) fn recover_rule(input: &mut Parser<'_, '_>) {
    let _ = input.next_including_whitespace_and_comments();
}

pub(super) fn recover_declaration(input: &mut Parser<'_, '_>) {
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

use std::borrow::Cow;

use rocketcss_parser::{SourceLocation, Span, Token, TokenAndSpan, Tokenizer, unescape};

fn tokenize(input: &str) -> Vec<TokenAndSpan> {
    let mut tokenizer = Tokenizer::new(input);
    std::iter::from_fn(|| tokenizer.next().ok()).collect()
}

fn token_kinds(input: &str) -> Vec<Token> {
    tokenize(input)
        .into_iter()
        .map(|token| token.token)
        .collect()
}

fn next_kind(tokenizer: &mut Tokenizer<'_>) -> Result<Token, ()> {
    tokenizer.next().map(|token| token.token)
}

fn source_text(source: &str, span: Span) -> &str {
    &source[span.start as usize..span.end as usize]
}

#[test]
fn tokenizes_css_syntax_tokens_without_values() {
    assert_eq!(
        token_kinds(
            "#id #123 @media 12px 50% 1.5 url(foo) \"bar\" /*x*/ ~= |= ^= $= *= <!-- --> :;,()[]{}"
        ),
        vec![
            Token::IDHash,
            Token::WhiteSpace,
            Token::Hash,
            Token::WhiteSpace,
            Token::AtKeyword,
            Token::WhiteSpace,
            Token::Dimension,
            Token::WhiteSpace,
            Token::Percentage,
            Token::WhiteSpace,
            Token::Number,
            Token::WhiteSpace,
            Token::UnquotedUrl,
            Token::WhiteSpace,
            Token::QuotedString,
            Token::WhiteSpace,
            Token::Comment,
            Token::WhiteSpace,
            Token::IncludeMatch,
            Token::WhiteSpace,
            Token::DashMatch,
            Token::WhiteSpace,
            Token::PrefixMatch,
            Token::WhiteSpace,
            Token::SuffixMatch,
            Token::WhiteSpace,
            Token::SubstringMatch,
            Token::WhiteSpace,
            Token::CDO,
            Token::WhiteSpace,
            Token::CDC,
            Token::WhiteSpace,
            Token::Colon,
            Token::Semicolon,
            Token::Comma,
            Token::ParenthesisBlock,
            Token::CloseParenthesis,
            Token::SquareBracketBlock,
            Token::CloseSquareBracket,
            Token::CurlyBracketBlock,
            Token::CloseCurlyBracket,
        ]
    );
}

#[test]
fn records_complete_spans_for_deferred_value_parsing() {
    let source = r#"plain fo\6f  "b\61 r" url(a\20 b) 12px"#;
    let tokens = tokenize(source);

    assert_eq!(tokens[0], TokenAndSpan::new(Token::Ident, Span::new(0, 5)));
    assert_eq!(tokens[2].token, Token::Ident);
    let raw_ident = source_text(source, tokens[2].span);
    assert_eq!(raw_ident, r"fo\6f ");
    assert_eq!(unescape(raw_ident), "foo");

    assert_eq!(tokens[4].token, Token::QuotedString);
    let raw_string = source_text(source, tokens[4].span);
    assert_eq!(raw_string, r#""b\61 r""#);
    assert_eq!(unescape(&raw_string[1..raw_string.len() - 1]), "bar");

    assert_eq!(tokens[6].token, Token::UnquotedUrl);
    let raw_url = source_text(source, tokens[6].span);
    assert_eq!(raw_url, r"url(a\20 b)");
    assert_eq!(unescape(&raw_url[4..raw_url.len() - 1]), "a b");

    assert_eq!(tokens[8].token, Token::Dimension);
    assert_eq!(source_text(source, tokens[8].span), "12px");
}

#[test]
fn unescape_only_allocates_when_needed() {
    assert!(matches!(unescape("plain"), Cow::Borrowed("plain")));
    assert_eq!(unescape("a\\\r\nb"), "ab");
    assert_eq!(unescape("\\0 "), "�");
    assert_eq!(unescape("\\110000"), "�");
}

#[test]
fn escaped_newline_updates_token_span_and_location() {
    let source = "\"a\\\r\nb\"";
    let mut tokenizer = Tokenizer::new(source);

    let token = tokenizer.next().unwrap();
    assert_eq!(token.token, Token::QuotedString);
    assert_eq!(source_text(source, token.span), source);
    assert_eq!(
        tokenizer.current_source_location(),
        SourceLocation { line: 1, column: 3 }
    );
}

#[test]
fn escaped_url_and_function_names_keep_tokenizer_semantics() {
    let mut tokenizer = Tokenizer::new(r"u\72 l(foo) v\61 r(--x)");

    assert_eq!(next_kind(&mut tokenizer), Ok(Token::UnquotedUrl));
    assert_eq!(next_kind(&mut tokenizer), Ok(Token::WhiteSpace));
    assert_eq!(next_kind(&mut tokenizer), Ok(Token::Function));
}

/// https://github.com/servo/rust-cssparser/issues/174
#[test]
fn bad_url_slice_out_of_bounds() {
    let source = "url(\u{1}\\";
    let mut tokenizer = Tokenizer::new(source);
    assert_eq!(
        tokenizer.next(),
        Ok(TokenAndSpan::new(
            Token::BadUrl,
            Span::new(0, source.len() as u32)
        ))
    );
}

/// https://bugzilla.mozilla.org/show_bug.cgi?id=1383975
#[test]
fn bad_url_slice_not_at_char_boundary() {
    let source = "url(9\n۰";
    let mut tokenizer = Tokenizer::new(source);
    assert_eq!(
        tokenizer.next(),
        Ok(TokenAndSpan::new(
            Token::BadUrl,
            Span::new(0, source.len() as u32)
        ))
    );
}

#[test]
fn tracks_locations_and_restores_state() {
    let mut tokenizer = Tokenizer::new("a\r\n🆒 b");

    assert_eq!(
        tokenizer.current_source_location(),
        SourceLocation { line: 0, column: 1 }
    );
    assert_eq!(next_kind(&mut tokenizer), Ok(Token::Ident));
    let state = tokenizer.state();

    assert_eq!(next_kind(&mut tokenizer), Ok(Token::WhiteSpace));
    assert_eq!(next_kind(&mut tokenizer), Ok(Token::Ident));
    assert_eq!(
        tokenizer.current_source_location(),
        SourceLocation { line: 1, column: 3 }
    );

    tokenizer.reset(&state);
    assert_eq!(state.position().byte_index(), 1);
    assert_eq!(
        state.source_location(),
        SourceLocation { line: 0, column: 2 }
    );
    assert_eq!(next_kind(&mut tokenizer), Ok(Token::WhiteSpace));
}

#[test]
fn comment_contents_are_deferred_to_the_parser() {
    let source = "/*# sourceMappingURL=style.css.map*/";
    let token = tokenize(source)[0];

    assert_eq!(token.token, Token::Comment);
    assert_eq!(source_text(source, token.span), source);
}

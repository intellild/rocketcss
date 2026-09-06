//! CSS tokenizer and parser infrastructure.

macro_rules! match_byte {
    ($value:expr, $($rest:tt)*) => {
        match $value {
            $($rest)+
        }
    };
}

mod compiler;
mod escape;
mod parser;
pub mod prelude;
mod tokenizer;

pub use compiler::Compiler;
pub use escape::unescape;
pub use parser::ValueToken;
pub use parser::parse;
pub use parser::{
    BasicParseError, BasicParseErrorKind, Delimiter, Delimiters, Error, Parse, ParseError,
    ParseErrorKind, ParseUntilErrorBehavior, ParserError, ParserOptions, ParserState,
};
pub use rocketcss_ast::Span;
pub use tokenizer::{
    SourceLocation, SourcePosition, Token, TokenAndSpan, Tokenizer, TokenizerState,
};

#[cfg(test)]
mod tests {
    use rocketcss_ast::{DUMMY_SP, FontFamily, match_ignore_ascii_case};
    use rocketcss_common::Allocator;

    use crate::Compiler;

    #[test]
    fn ascii_case_match_evaluates_input_once_and_supports_aliases() {
        let mut evaluations = 0;
        let result = match_ignore_ascii_case!(
            {
                evaluations += 1;
                "ScReEn"
            },
            "all" => 0,
            "print" | "screen" => 1,
            _ => 2,
        );

        assert_eq!(evaluations, 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn string_ranges_survive_rollback_and_temporary_sources() {
        let allocator = Allocator::new();
        let source = "root nested";
        let mut compiler = Compiler::new_with_source(source, &allocator);
        let root = compiler.intern(&source[..4]);
        let nested = compiler.with_source(&source[5..], |compiler| compiler.intern(&source[5..]));
        assert_eq!(compiler.string_pool().extra_len(), 0);
        let mut speculative = None;
        let result: Result<(), ()> = compiler.try_parse(|compiler| {
            speculative = Some(compiler.intern("decoded"));
            Err(())
        });
        assert_eq!(result, Err(()));
        let decoded = speculative.unwrap();
        assert_eq!(decoded, compiler.intern("decoded"));
        compiler.with_source("external", |compiler| {
            compiler.intern("external");
        });
        compiler.intern(&"x".repeat(8192));
        let context = compiler.into_ast_context();
        assert_eq!(context.str(root), "root");
        assert_eq!(context.str(nested), "nested");
        assert_eq!(context.str(decoded), "decoded");
    }

    #[test]
    fn ordinary_ranges_survive_temporary_sources_and_repeated_failed_parses() {
        use rocketcss_ast::{Token, TokenOrValue};

        let allocator = Allocator::new();
        let source = "root é";
        let external = String::from("outside é");
        let mut compiler = Compiler::new_with_source(source, &allocator);
        let root = compiler.add_str(&source[..4]);
        let nested = compiler.with_source(&source[5..], |compiler| compiler.add_str(&source[5..]));
        assert_eq!(compiler.string_pool().extra_len(), 0);
        let temporary = compiler.with_source(&external, |compiler| compiler.add_str(&external));
        assert_eq!(
            compiler
                .string_pool()
                .get(compiler.string_pool().source_range(0, source.len() as u32)),
            source
        );
        compiler.expect_ident_matching("root").unwrap();
        assert_eq!(compiler.string_pool().extra_len(), external.len());

        let canonical = compiler.intern("shared");
        let checkpoint = compiler.ast_context().node_checkpoint();
        let pool_bytes = compiler.string_pool().extra_len();
        let mut retained_ranges = std::vec::Vec::new();
        const DECODED: &str = "decoded-é";
        for _ in 0..64 {
            let result: Result<(), ()> = compiler.try_parse(|compiler| {
                let decoded = crate::unescape(r"decoded-\e9");
                let text = compiler.add_str(&decoded);
                retained_ranges.push(text);
                assert_eq!(compiler.intern("shared"), canonical);
                let ast = compiler.ast_context_mut();
                let node = ast.alloc_node(Token::Ident(text), DUMMY_SP);
                let mut values = allocator.vec();
                values.push(TokenOrValue::Token(node));
                ast.alloc_vec(values);
                Err(())
            });
            assert_eq!(result, Err(()));
            assert_eq!(compiler.ast_context().node_checkpoint(), checkpoint);
        }
        assert!(retained_ranges.windows(2).all(|pair| pair[0] != pair[1]));
        // Rollback reclaims logical node/list entries, but ordinary strings
        // remain append-only: exactly 64 * 10 bytes, with no new intern keys.
        assert_eq!(
            compiler.string_pool().extra_len(),
            pool_bytes + 64 * DECODED.len()
        );
        assert_eq!(compiler.string_pool().len(), 1);
        compiler.add_str(&"x".repeat(8192));
        let context = compiler.into_ast_context();
        assert_eq!(context.str(root), "root");
        assert_eq!(context.str(nested), "é");
        assert_eq!(context.str(temporary), external);
        assert_eq!(context.str(canonical), "shared");
        for range in retained_ranges {
            assert_eq!(context.str(range), DECODED);
        }
    }

    #[test]
    fn ordinary_pool_ranges_keep_decoded_unicode_after_temporary_buffers_drop() {
        let allocator = Allocator::new();
        let mut compiler = Compiler::new_with_source("root", &allocator);
        let mut ranges = std::vec::Vec::new();
        for (source, expected) in [
            ("x\0y", "x�y"),
            (r"\0", "�"),
            (r"\d800", "�"),
            (r"\110000", "�"),
            (r"\1f600", "😀"),
            (r"\e9", "é"),
            (r"\000041 B", "AB"),
            ("a\\\nb", "ab"),
        ] {
            let decoded = crate::unescape(source);
            assert_eq!(decoded, expected);
            ranges.push((compiler.add_str(&decoded), expected));
        }
        // Equal replacement characters remain separate ordinary strings.
        assert_ne!(ranges[1].0, ranges[2].0);
        assert_ne!(ranges[2].0, ranges[3].0);
        assert_eq!(compiler.string_pool().len(), 0);
        compiler.add_str(&"x".repeat(8192));
        for (range, expected) in ranges {
            assert_eq!(compiler.string_pool().get(range), expected);
        }
    }

    #[test]
    fn failed_speculative_parses_roll_back_node_allocations() {
        let allocator = Allocator::new();
        let mut compiler = Compiler::new_with_source("", &allocator);

        for value in [
            FontFamily::Serif,
            FontFamily::SansSerif,
            FontFamily::Monospace,
            FontFamily::Cursive,
        ] {
            let result: Result<(), ()> = compiler.try_parse(|compiler| {
                compiler.ast_context_mut().alloc_node(value, DUMMY_SP);
                Err(())
            });
            assert_eq!(result, Err(()));
        }

        let committed = compiler
            .ast_context_mut()
            .alloc_node(FontFamily::Fantasy, DUMMY_SP);
        assert_eq!(committed.index(), 0);
        assert_eq!(compiler.ast_context().node(committed), FontFamily::Fantasy);
    }
}

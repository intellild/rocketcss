use crate::prelude::*;

macro_rules! keyword_parse {
    ($ty:ty, $($name:literal => $variant:expr),+ $(,)?) => {
        impl<'i> Parse<'i> for $ty {
            fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                let ident = input.expect_ident()?;
                match_ignore_ascii_case!(
                    ident,
                    $( $name => Ok($variant), )+
                    _ => Err(input.new_custom_error(ParserError::InvalidValue)),
                )
            }
        }
    };
}

keyword_parse!(
    TextTransformCase,
    "none" => Self::None,
    "uppercase" => Self::Uppercase,
    "lowercase" => Self::Lowercase,
    "capitalize" => Self::Capitalize,
);
keyword_parse!(
    WhiteSpace,
    "normal" => Self::Normal,
    "pre" => Self::Pre,
    "nowrap" => Self::Nowrap,
    "pre-wrap" => Self::PreWrap,
    "break-spaces" => Self::BreakSpaces,
    "pre-line" => Self::PreLine,
);
keyword_parse!(
    WordBreak,
    "normal" => Self::Normal,
    "keep-all" => Self::KeepAll,
    "break-all" => Self::BreakAll,
    "break-word" => Self::BreakWord,
);
keyword_parse!(
    LineBreak,
    "auto" => Self::Auto,
    "loose" => Self::Loose,
    "normal" => Self::Normal,
    "strict" => Self::Strict,
    "anywhere" => Self::Anywhere,
);
keyword_parse!(
    Hyphens,
    "none" => Self::None,
    "manual" => Self::Manual,
    "auto" => Self::Auto,
);
keyword_parse!(
    OverflowWrap,
    "normal" => Self::Normal,
    "anywhere" => Self::Anywhere,
    "break-word" => Self::BreakWord,
);
keyword_parse!(
    TextAlign,
    "start" => Self::Start,
    "end" => Self::End,
    "left" => Self::Left,
    "right" => Self::Right,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "match-parent" => Self::MatchParent,
    "justify-all" => Self::JustifyAll,
);
keyword_parse!(
    TextAlignLast,
    "auto" => Self::Auto,
    "start" => Self::Start,
    "end" => Self::End,
    "left" => Self::Left,
    "right" => Self::Right,
    "center" => Self::Center,
    "justify" => Self::Justify,
    "match-parent" => Self::MatchParent,
);
keyword_parse!(
    TextJustify,
    "auto" => Self::Auto,
    "none" => Self::None,
    "inter-word" => Self::InterWord,
    "inter-character" => Self::InterCharacter,
);
keyword_parse!(
    TextDecorationStyle,
    "solid" => Self::Solid,
    "double" => Self::Double,
    "dotted" => Self::Dotted,
    "dashed" => Self::Dashed,
    "wavy" => Self::Wavy,
);
keyword_parse!(
    TextDecorationSkipInk,
    "auto" => Self::Auto,
    "none" => Self::None,
    "all" => Self::All,
);
keyword_parse!(TextDirection, "ltr" => Self::Ltr, "rtl" => Self::Rtl,);
keyword_parse!(
    UnicodeBidi,
    "normal" => Self::Normal,
    "embed" => Self::Embed,
    "isolate" => Self::Isolate,
    "bidi-override" => Self::BidiOverride,
    "isolate-override" => Self::IsolateOverride,
    "plaintext" => Self::Plaintext,
);
keyword_parse!(BoxDecorationBreak, "slice" => Self::Slice, "clone" => Self::Clone,);
keyword_parse!(TextOverflow, "clip" => Self::Clip, "ellipsis" => Self::Ellipsis,);
keyword_parse!(
    Resize,
    "none" => Self::None,
    "both" => Self::Both,
    "horizontal" => Self::Horizontal,
    "vertical" => Self::Vertical,
    "block" => Self::Block,
    "inline" => Self::Inline,
);

impl<'i> Parse<'i> for TextTransform {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let case = TextTransformCase::parse(input)?;
        let mut full_width = false;
        let mut full_size_kana = false;
        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            match_ignore_ascii_case!(
                ident,
                "full-width" => full_width = true,
                "full-size-kana" => full_size_kana = true,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        if matches!(case, TextTransformCase::None) && (full_width || full_size_kana) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            case,
            full_size_kana,
            full_width,
        })
    }
}

impl<'i> Parse<'i> for Spacing<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(Self::Normal);
        }
        Ok(Self::Length(store_node(Length::parse(input)?, input)))
    }
}

impl<'i> Parse<'i> for TextDecorationLine<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first = input.expect_ident()?;
        if first.eq_ignore_ascii_case("none") {
            return Ok(Self::ExclusiveTextDecorationLine(
                ExclusiveTextDecorationLine::None,
            ));
        }
        if first.eq_ignore_ascii_case("spelling-error") {
            return Ok(Self::ExclusiveTextDecorationLine(
                ExclusiveTextDecorationLine::SpellingError,
            ));
        }
        if first.eq_ignore_ascii_case("grammar-error") {
            return Ok(Self::ExclusiveTextDecorationLine(
                ExclusiveTextDecorationLine::GrammarError,
            ));
        }

        let mut values = input.allocator().vec();
        values.push(parse_text_decoration_line(first, input)?);
        while !input.is_exhausted() {
            values.push(parse_text_decoration_line(input.expect_ident()?, input)?);
        }
        Ok(Self::Value(values))
    }
}

fn parse_text_decoration_line<'i>(
    ident: &'i str,
    input: &mut Compiler<'i>,
) -> Result<OtherTextDecorationLine, ParseError<'i, ParserError<'i>>> {
    match_ignore_ascii_case!(
        ident,
        "underline" => Ok(OtherTextDecorationLine::Underline),
        "overline" => Ok(OtherTextDecorationLine::Overline),
        "line-through" => Ok(OtherTextDecorationLine::LineThrough),
        "blink" => Ok(OtherTextDecorationLine::Blink),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

impl<'i> Parse<'i> for TextDecorationThickness<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        if input
            .try_parse(|input| input.expect_ident_matching("from-font"))
            .is_ok()
        {
            return Ok(Self::FromFont);
        }
        Ok(Self::LengthPercentage(store_node(
            LengthPercentage::parse(input)?,
            input,
        )))
    }
}

impl<'i> Parse<'i> for TextSizeAdjust {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(Self::None);
        }
        Ok(Self::Percentage(input.expect_percentage()?))
    }
}

impl<'i> Parse<'i> for TextIndent<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = store_node(LengthPercentage::parse(input)?, input);
        let mut each_line = false;
        let mut hanging = false;
        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            match_ignore_ascii_case!(
                ident,
                "each-line" => each_line = true,
                "hanging" => hanging = true,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        Ok(Self {
            each_line,
            hanging,
            value,
        })
    }
}

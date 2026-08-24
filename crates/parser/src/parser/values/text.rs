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
        Ok(Self::Length(input.allocator().boxed(Length::parse(input)?)))
    }
}

impl<'i> Parse<'i> for TextDecorationLine {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut value = Self::empty();
        let mut any = false;
        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            let flag = match_ignore_ascii_case!(
                ident,
                "none" => {
                    if any {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    Self::empty()
                },
                "underline" => Self::UNDERLINE,
                "overline" => Self::OVERLINE,
                "line-through" => Self::LINE_THROUGH,
                "blink" => Self::BLINK,
                "spelling-error" => {
                    if any {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    Self::SPELLING_ERROR
                },
                "grammar-error" => {
                    if any {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    Self::GRAMMAR_ERROR
                },
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
            if flag.is_empty()
                || flag.intersects(Self::SPELLING_ERROR)
                || flag.intersects(Self::GRAMMAR_ERROR)
            {
                if any || !input.is_exhausted() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                return Ok(flag);
            }
            value.insert(flag);
            any = true;
        }
        if !any {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(value)
    }
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
        Ok(Self::LengthPercentage(
            input.allocator().boxed(LengthPercentage::parse(input)?),
        ))
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
        let value = input.allocator().boxed(LengthPercentage::parse(input)?);
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

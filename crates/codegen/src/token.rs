use crate::prelude::*;

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
            Self::Ident(value) => CssToken::Ident(CowRcStr::from(*value)).to_css(dest),
            Self::AtKeyword(value) => CssToken::AtKeyword(CowRcStr::from(*value)).to_css(dest),
            Self::Hash(value) => CssToken::Hash(CowRcStr::from(*value)).to_css(dest),
            Self::IdHash(value) => CssToken::IDHash(CowRcStr::from(*value)).to_css(dest),
            Self::MinifiedHash(value) => write_minified_hash(value, dest),
            Self::String(value) => CssToken::QuotedString(CowRcStr::from(*value)).to_css(dest),
            Self::UnquotedFont(value) => write_unquoted_font(value, dest),
            Self::UnquotedUrl(value) => CssToken::UnquotedUrl(CowRcStr::from(*value)).to_css(dest),
            Self::Delim(value) => {
                for character in value.chars() {
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
                dest.write_str(unit)
            }
            Self::WhiteSpace(value) => {
                if dest.prettify() {
                    dest.write_str(value)
                } else {
                    dest.write_char(' ')
                }
            }
            Self::Comment(value) => {
                if !dest.prettify() {
                    return Ok(());
                }
                dest.write_str("/*")?;
                dest.write_str(value)?;
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
                serialize_identifier(value, dest)?;
                dest.write_char('(')
            }
            Self::ParenthesisBlock => dest.write_char('('),
            Self::SquareBracketBlock => dest.write_char('['),
            Self::CurlyBracketBlock => dest.write_char('{'),
            Self::BadUrl(value) => {
                dest.write_str("url(")?;
                dest.write_str(value)
            }
            Self::BadString(value) => dest.write_str(value),
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

pub(crate) fn write_token_list<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 && minified_color_needs_separator(&values[index - 1], value, cx) {
            dest.write_char(' ')?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

/// Serialize a fallback value without applying any value-level normalization.
///
/// Unparsed declarations are semantic barriers. In particular, dropping a
/// comment between two tokens or using a cached function replacement can join
/// tokens that the parser deliberately kept opaque.
pub(crate) fn write_unparsed_token_list<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    for value in values {
        write_unparsed_token_or_value(value, dest, cx)?;
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
                dest.write_str(comment)?;
                dest.write_str("*/")
            }
            Token::WhiteSpace(whitespace) => dest.write_str(whitespace),
            token => token.to_css(dest, cx),
        },
        TokenOrValue::Function(function) => {
            let function = ast.resolve_node(*function);
            let arguments = ast.vec(function.arguments);
            dest.write_str(function.name())?;
            dest.write_char('(')?;
            write_unparsed_token_list(arguments, dest, cx)?;
            if function.kind().is_variable()
                && matches!(
                    arguments.last(),
                    Some(TokenOrValue::Token(token)) if matches!(ast.resolve_node(*token), Token::Comma)
                )
            {
                dest.write_char(' ')?;
            }
            dest.write_char(')')
        }
        value => value.to_css(dest, cx),
    }
}

fn minified_color_needs_separator(
    left: &TokenOrValue<'_>,
    right: &TokenOrValue<'_>,
    cx: &ToCssContext<'_, '_, '_>,
) -> bool {
    let TokenOrValue::Function(function) = left else {
        return false;
    };
    let ast = cx.ast_context();
    let function = ast.resolve_node(*function);
    let ends_as_identifier_or_hash = matches!(
        function.replacement,
        Some(FunctionReplacement::Rgb { .. })
            | Some(FunctionReplacement::Rgba { alpha: 0.0, .. })
            | Some(FunctionReplacement::Rgba { use_hex: true, .. })
    );
    ends_as_identifier_or_hash
        && !matches!(right, TokenOrValue::Token(token)
            if matches!(ast.resolve_node(*token), Token::WhiteSpace(_) | Token::Comma | Token::Semicolon
                | Token::CloseParenthesis | Token::CloseSquareBracket | Token::CloseCurlyBracket))
}

pub(crate) fn write_token_list_trimmed<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let start = values
        .iter()
        .position(|value| {
            !matches!(value, TokenOrValue::Token(token) if matches!(cx.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
        })
        .unwrap_or(values.len());
    write_token_list(&values[start..], dest, cx)
}

pub(crate) fn write_token_list_without_outer_whitespace<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let is_whitespace = |value: &TokenOrValue<'_>| matches!(value, TokenOrValue::Token(token) if matches!(cx.ast_context().resolve_node(*token), Token::WhiteSpace(_)));
    let start = values
        .iter()
        .position(|value| !is_whitespace(value))
        .unwrap_or(values.len());
    let end = values
        .iter()
        .rposition(|value| !is_whitespace(value))
        .map_or(start, |index| index + 1);
    write_token_list(&values[start..end], dest, cx)
}

fn starts_with_whitespace(values: &[TokenOrValue<'_>], cx: &ToCssContext<'_, '_, '_>) -> bool {
    matches!(values.first(), Some(TokenOrValue::Token(token)) if matches!(cx.ast_context().resolve_node(*token), Token::WhiteSpace(_)))
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
            Self::DashedIdent(value) => write_dashed_ident(value, dest),
            Self::AnimationName(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_dashed_ident<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    dest.write_str("--")?;
    serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
}

impl<'ghost> ToCss<'ghost> for Url<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("url(")?;
        if !dest.prettify() && can_write_unquoted_url(self.url) {
            write_unquoted_url(self.url, dest)?;
        } else {
            serialize_string(self.url, dest)?;
        }
        dest.write_char(')')
    }
}

fn can_write_unquoted_url(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '(' | ')' | '\\')
        })
}

fn write_unquoted_url<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    let mut start = 0;
    for (index, character) in value.char_indices() {
        let replacement = match character {
            '"' => "%22",
            '\'' => "%27",
            _ => continue,
        };
        dest.write_str(&value[start..index])?;
        dest.write_str(replacement)?;
        start = index + character.len_utf8();
    }
    dest.write_str(&value[start..])
}

impl<'ghost> ToCss<'ghost> for Variable<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("var(")?;
        self.name.to_css(dest, _cx)?;
        if let Some(fallback) = &self.fallback {
            let fallback = _cx.ast_context().vec(*fallback);
            dest.write_char(',')?;
            if fallback.is_empty() {
                dest.write_char(' ')?;
            } else if !starts_with_whitespace(fallback, _cx) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest, _cx)?;
            if matches!(fallback.last(), Some(TokenOrValue::Token(token)) if matches!(_cx.ast_context().resolve_node(*token), Token::Comma))
            {
                dest.write_char(' ')?;
            }
        }
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for EnvironmentVariable<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("env(")?;
        self.name.to_css(dest, _cx)?;
        for index in _cx.ast_context().vec(self.indices) {
            dest.write_char(' ')?;
            serialize_int(*index, dest)?;
        }
        if let Some(fallback) = &self.fallback {
            let fallback = _cx.ast_context().vec(*fallback);
            dest.write_char(',')?;
            if fallback.is_empty() {
                dest.write_char(' ')?;
            } else if !starts_with_whitespace(fallback, _cx) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest, _cx)?;
            if matches!(fallback.last(), Some(TokenOrValue::Token(token)) if matches!(_cx.ast_context().resolve_node(*token), Token::Comma))
            {
                dest.write_char(' ')?;
            }
        }
        dest.write_char(')')
    }
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
            Self::Unknown(value) => serialize_identifier(value, dest),
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

impl<'ghost> ToCss<'ghost> for DashedIdentReference<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_dashed_ident(self.ident, dest)?;
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest, _cx)?;
        }
        Ok(())
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
            Self::File(value) => serialize_string(value, dest),
            Self::SourceIndex(_) => Ok(()),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Function<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(replacement) = self.replacement {
            return match replacement {
                FunctionReplacement::GrayAlpha { alpha, lightness } => {
                    dest.write_str("hsla(0,0%,")?;
                    serialize_number(lightness * 100.0, dest)?;
                    dest.write_str("%,")?;
                    serialize_number(alpha, dest)?;
                    dest.write_char(')')
                }
                FunctionReplacement::Number(value) => serialize_number(value, dest),
                FunctionReplacement::Dimension { unit, value } => {
                    serialize_dimension(value, &unit, dest, _cx)
                }
                FunctionReplacement::Percentage(value) => {
                    serialize_number(value * 100.0, dest)?;
                    dest.write_char('%')
                }
                FunctionReplacement::Rgb { blue, green, red } => {
                    write_minified_rgb(red, green, blue, dest)
                }
                FunctionReplacement::Rgba {
                    alpha,
                    blue,
                    green,
                    red,
                    use_hex,
                } => write_minified_rgba(red, green, blue, alpha, use_hex, dest),
            };
        }
        serialize_identifier(self.name(), dest)?;
        if self.is_identifier() {
            return Ok(());
        }
        dest.write_char('(')?;
        let arguments = _cx.ast_context().vec(self.arguments);
        if self.is_unquoted_url() {
            let [TokenOrValue::Token(token)] = arguments else {
                unreachable!("unquoted URL functions retain one string token")
            };
            let Token::String(value) = _cx.ast_context().resolve_node(*token) else {
                unreachable!("unquoted URL functions retain one string token")
            };
            write_unquoted_url(value, dest)?;
            return dest.write_char(')');
        }
        write_token_list(arguments, dest, _cx)?;
        if self.kind().is_variable()
            && matches!(arguments.last(), Some(TokenOrValue::Token(token)) if matches!(_cx.ast_context().resolve_node(*token), Token::Comma))
        {
            dest.write_char(' ')?;
        }
        dest.write_char(')')
    }
}

fn write_minified_rgba<PrinterT: PrinterTrait>(
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
    use_hex: bool,
    dest: &mut PrinterT,
) -> fmt::Result {
    if alpha == 0.0 {
        return dest.write_str("#0000");
    }
    if !use_hex {
        dest.write_str("rgba(")?;
        serialize_int(red, dest)?;
        dest.write_char(',')?;
        serialize_int(green, dest)?;
        dest.write_char(',')?;
        serialize_int(blue, dest)?;
        dest.write_char(',')?;
        serialize_number(alpha, dest)?;
        return dest.write_char(')');
    }
    let alpha = (alpha * 255.0).round() as u8;
    let rgba = u32::from_be_bytes([red, green, blue, alpha]);
    dest.write_char('#')?;
    let values = [red, green, blue, alpha];
    if values.iter().all(|value| value >> 4 == value & 15) {
        let rgba = ((rgba >> 12) & 0xf000)
            | ((rgba >> 8) & 0x0f00)
            | ((rgba >> 4) & 0x00f0)
            | (rgba & 0x000f);
        serialize_hex(rgba, 4, false, dest)
    } else {
        serialize_hex(rgba, 8, false, dest)
    }
}

fn write_minified_rgb<PrinterT: PrinterTrait>(
    red: u8,
    green: u8,
    blue: u8,
    dest: &mut PrinterT,
) -> fmt::Result {
    if (red, green, blue) == (255, 0, 0) {
        return dest.write_str("red");
    }
    let rgb = u32::from_be_bytes([0, red, green, blue]);
    dest.write_char('#')?;
    if red >> 4 == red & 15 && green >> 4 == green & 15 && blue >> 4 == blue & 15 {
        let rgb = ((rgb >> 12) & 0x0f00) | ((rgb >> 8) & 0x00f0) | ((rgb >> 4) & 0x000f);
        serialize_hex(rgb, 3, false, dest)
    } else {
        serialize_hex(rgb, 6, false, dest)
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
            Self::Ident(value) => serialize_identifier(value, dest),
            Self::String(value)
                if matches!(
                    value.to_ascii_lowercase().as_str(),
                    "none"
                        | "initial"
                        | "inherit"
                        | "unset"
                        | "default"
                        | "revert"
                        | "revert-layer"
                ) =>
            {
                serialize_string(value, dest)
            }
            Self::String(value) => serialize_identifier(value, dest),
        }
    }
}

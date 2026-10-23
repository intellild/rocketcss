use crate::prelude::*;

impl<'ast, 'ghost> ToCss<'ghost> for Unit<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            UnitData::Length(unit) => unit.to_css(dest, _cx),
            UnitData::Deg => dest.write_str("deg"),
            UnitData::Rad => dest.write_str("rad"),
            UnitData::Grad => dest.write_str("grad"),
            UnitData::Turn => dest.write_str("turn"),
            UnitData::Seconds => dest.write_str("s"),
            UnitData::Milliseconds => dest.write_str("ms"),
            UnitData::Hertz => dest.write_str("hz"),
            UnitData::Kilohertz => dest.write_str("khz"),
            UnitData::Dpi => dest.write_str("dpi"),
            UnitData::Dpcm => dest.write_str("dpcm"),
            UnitData::Dppx => dest.write_str("dppx"),
            UnitData::ResolutionX => dest.write_str("x"),
            UnitData::Flex => dest.write_str("fr"),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Token<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        use cssparser::{CowRcStr, ToCss as CssParserToCss, Token as CssToken};

        match self.data(_cx.ast()) {
            TokenData::Ident(value) => CssToken::Ident(CowRcStr::from(value)).to_css(dest),
            TokenData::AtKeyword(value) => CssToken::AtKeyword(CowRcStr::from(value)).to_css(dest),
            TokenData::Hash(value) => CssToken::Hash(CowRcStr::from(value)).to_css(dest),
            TokenData::IdHash(value) => CssToken::IDHash(CowRcStr::from(value)).to_css(dest),
            TokenData::MinifiedHash(value) => write_minified_hash(value, dest),
            TokenData::String(value) => CssToken::QuotedString(CowRcStr::from(value)).to_css(dest),
            TokenData::UnquotedFont(value) => write_unquoted_font(value, dest),
            TokenData::UnquotedUrl(value) => {
                CssToken::UnquotedUrl(CowRcStr::from(value)).to_css(dest)
            }
            TokenData::Delim(value) => {
                for character in value.chars() {
                    CssToken::Delim(character).to_css(dest)?;
                }
                Ok(())
            }
            TokenData::Number(value) => serialize_number(value, dest),
            TokenData::Percentage(value) => {
                serialize_number(value * 100.0, dest)?;
                dest.write_char('%')
            }
            TokenData::Dimension { unit, value } => serialize_dimension(value, &unit, dest, _cx),
            TokenData::UnknownDimension { unit, value } => {
                serialize_number(value, dest)?;
                dest.write_str(unit)
            }
            TokenData::WhiteSpace(value) => {
                if dest.prettify() {
                    dest.write_str(value)
                } else {
                    dest.write_char(' ')
                }
            }
            TokenData::Comment(value) => {
                if !dest.prettify() {
                    return Ok(());
                }
                dest.write_str("/*")?;
                dest.write_str(value)?;
                dest.write_str("*/")
            }
            TokenData::Colon => dest.write_char(':'),
            TokenData::Semicolon => dest.write_char(';'),
            TokenData::Comma => dest.write_char(','),
            TokenData::IncludeMatch => dest.write_str("~="),
            TokenData::DashMatch => dest.write_str("|="),
            TokenData::PrefixMatch => dest.write_str("^="),
            TokenData::SuffixMatch => dest.write_str("$="),
            TokenData::SubstringMatch => dest.write_str("*="),
            TokenData::Cdo => dest.write_str("<!--"),
            TokenData::Cdc => dest.write_str("-->"),
            TokenData::Function(value) => {
                serialize_identifier(value, dest)?;
                dest.write_char('(')
            }
            TokenData::ParenthesisBlock => dest.write_char('('),
            TokenData::SquareBracketBlock => dest.write_char('['),
            TokenData::CurlyBracketBlock => dest.write_char('{'),
            TokenData::BadUrl(value) => {
                dest.write_str("url(")?;
                dest.write_str(value)
            }
            TokenData::BadString(value) => dest.write_str(value),
            TokenData::CloseParenthesis => dest.write_char(')'),
            TokenData::CloseSquareBracket => dest.write_char(']'),
            TokenData::CloseCurlyBracket => dest.write_char('}'),
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
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 && minified_color_needs_separator(cx.ast(), &values[index - 1], value) {
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
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    for value in values {
        write_unparsed_token_or_value(value, dest, cx)?;
    }
    Ok(())
}

fn write_unparsed_token_or_value<'ghost, PrinterT: PrinterTrait>(
    value: &TokenOrValue<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast();
    match value.data(ast) {
        TokenOrValueData::Token(token) => match token.data(ast) {
            TokenData::Comment(comment) => {
                dest.write_str("/*")?;
                dest.write_str(comment)?;
                dest.write_str("*/")
            }
            TokenData::WhiteSpace(whitespace) => dest.write_str(whitespace),
            _ => token.to_css(dest, cx),
        },
        TokenOrValueData::Function(function) => {
            dest.write_str(function.name(ast))?;
            dest.write_char('(')?;
            write_unparsed_token_list(function.arguments(ast), dest, cx)?;
            let arguments = function.arguments(ast);
            if function.kind(ast).is_variable()
                && matches!(
                    arguments.last(),
                    Some(value)
                        if matches!(
                            value.data(ast),
                            TokenOrValueData::Token(token)
                                if matches!(token.data(ast), TokenData::Comma)
                        )
                )
            {
                dest.write_char(' ')?;
            }
            dest.write_char(')')
        }
        _ => value.to_css(dest, cx),
    }
}

fn minified_color_needs_separator<'ast>(
    ast: &Ast<'ast>,
    left: &TokenOrValue<'ast>,
    right: &TokenOrValue<'ast>,
) -> bool {
    let TokenOrValueData::Function(function) = left.data(ast) else {
        return false;
    };
    let ends_as_identifier_or_hash = matches!(
        function.replacement(ast).map(|value| value.data(ast)),
        Some(FunctionReplacementData::Rgb { .. })
            | Some(FunctionReplacementData::Rgba { alpha: 0.0, .. })
            | Some(FunctionReplacementData::Rgba { use_hex: true, .. })
    );
    ends_as_identifier_or_hash
        && !matches!(right.data(ast), TokenOrValueData::Token(token)
            if matches!(token.data(ast), TokenData::WhiteSpace(_) | TokenData::Comma | TokenData::Semicolon
                | TokenData::CloseParenthesis | TokenData::CloseSquareBracket | TokenData::CloseCurlyBracket))
}

pub(crate) fn write_token_list_trimmed<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let start = values
        .iter()
        .position(|value| {
            !matches!(value.data(cx.ast()), TokenOrValueData::Token(token) if matches!(token.data(cx.ast()), TokenData::WhiteSpace(_)))
        })
        .unwrap_or(values.len());
    write_token_list(&values[start..], dest, cx)
}

pub(crate) fn write_token_list_without_outer_whitespace<'ghost, PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let ast = cx.ast();
    let is_whitespace = |value: &TokenOrValue<'_>| matches!(value.data(ast), TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::WhiteSpace(_)));
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

fn starts_with_whitespace<'ast>(ast: &Ast<'ast>, values: &[TokenOrValue<'ast>]) -> bool {
    matches!(values.first(), Some(value) if matches!(value.data(ast), TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::WhiteSpace(_))))
}

impl<'ast, 'ghost> ToCss<'ghost> for TokenOrValue<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            TokenOrValueData::Token(value) => value.to_css(dest, _cx),
            TokenOrValueData::Color(value) => value.to_css(dest, _cx),
            TokenOrValueData::UnresolvedColor(value) => value.to_css(dest, _cx),
            TokenOrValueData::Url(value) => value.to_css(dest, _cx),
            TokenOrValueData::Var(value) => value.to_css(dest, _cx),
            TokenOrValueData::Env(value) => value.to_css(dest, _cx),
            TokenOrValueData::Function(value) => value.to_css(dest, _cx),
            TokenOrValueData::Length(value) => {
                serialize_dimension(value.value(_cx.ast()), &value.unit(_cx.ast()), dest, _cx)
            }
            TokenOrValueData::Angle(value) => value.to_css(dest, _cx),
            TokenOrValueData::Time(value) => value.to_css(dest, _cx),
            TokenOrValueData::Resolution(value) => value.to_css(dest, _cx),
            TokenOrValueData::DashedIdent(value) => write_dashed_ident(value, dest),
            TokenOrValueData::AnimationName(value) => value.to_css(dest, _cx),
        }
    }
}

fn write_dashed_ident<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    dest.write_str("--")?;
    serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
}

impl<'ast, 'ghost> ToCss<'ghost> for Url<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("url(")?;
        if !dest.prettify() && can_write_unquoted_url(data.url) {
            write_unquoted_url(data.url, dest)?;
        } else {
            serialize_string(data.url, dest)?;
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

impl<'ast, 'ghost> ToCss<'ghost> for Variable<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("var(")?;
        data.name.to_css(dest, _cx)?;
        if let Some(fallback) = data.fallback {
            dest.write_char(',')?;
            if fallback.is_empty() {
                dest.write_char(' ')?;
            } else if !starts_with_whitespace(_cx.ast(), fallback) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest, _cx)?;
            if matches!(fallback.last(), Some(value) if matches!(value.data(_cx.ast()), TokenOrValueData::Token(token) if matches!(token.data(_cx.ast()), TokenData::Comma)))
            {
                dest.write_char(' ')?;
            }
        }
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for EnvironmentVariable<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        dest.write_str("env(")?;
        data.name.to_css(dest, _cx)?;
        for index in data.indices {
            dest.write_char(' ')?;
            serialize_int(*index, dest)?;
        }
        if let Some(fallback) = data.fallback {
            dest.write_char(',')?;
            if fallback.is_empty() {
                dest.write_char(' ')?;
            } else if !starts_with_whitespace(_cx.ast(), fallback) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest, _cx)?;
            if matches!(fallback.last(), Some(value) if matches!(value.data(_cx.ast()), TokenOrValueData::Token(token) if matches!(token.data(_cx.ast()), TokenData::Comma)))
            {
                dest.write_char(' ')?;
            }
        }
        dest.write_char(')')
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for EnvironmentVariableName<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            EnvironmentVariableNameData::UA(value) => value.to_css(dest, _cx),
            EnvironmentVariableNameData::Custom(value) => value.to_css(dest, _cx),
            EnvironmentVariableNameData::Unknown(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for UAEnvironmentVariable {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("UA environment variables are static keywords"),
        )
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for DashedIdentReference<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_dashed_ident(data.ident, dest)?;
        if let Some(from) = &data.from {
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
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Global => dest.write_str("global"),
            Self::File(value) => serialize_string(value, dest),
            Self::SourceIndex(_) => Ok(()),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Function<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        let ast = _cx.ast();
        if let Some(replacement) = data.replacement {
            return match replacement.data(ast) {
                FunctionReplacementData::GrayAlpha { alpha, lightness } => {
                    dest.write_str("hsla(0,0%,")?;
                    serialize_number(lightness * 100.0, dest)?;
                    dest.write_str("%,")?;
                    serialize_number(alpha, dest)?;
                    dest.write_char(')')
                }
                FunctionReplacementData::Number(value) => serialize_number(value, dest),
                FunctionReplacementData::Dimension { unit, value } => {
                    serialize_dimension(value, &unit, dest, _cx)
                }
                FunctionReplacementData::Percentage(value) => {
                    serialize_number(value * 100.0, dest)?;
                    dest.write_char('%')
                }
                FunctionReplacementData::Rgb { blue, green, red } => {
                    write_minified_rgb(red, green, blue, dest)
                }
                FunctionReplacementData::Rgba {
                    alpha,
                    blue,
                    green,
                    red,
                    use_hex,
                } => write_minified_rgba(red, green, blue, alpha, use_hex, dest),
            };
        }
        serialize_identifier(data.name, dest)?;
        if data.flags.contains(FunctionFlags::IS_IDENTIFIER) {
            return Ok(());
        }
        dest.write_char('(')?;
        if data.flags.contains(FunctionFlags::UNQUOTED_URL) {
            let [value] = data.arguments else {
                unreachable!("unquoted URL functions retain one string token")
            };
            let TokenOrValueData::Token(token) = value.data(ast) else {
                unreachable!("unquoted URL functions retain one string token")
            };
            let TokenData::String(value) = token.data(ast) else {
                unreachable!("unquoted URL functions retain one string token")
            };
            write_unquoted_url(value, dest)?;
            return dest.write_char(')');
        }
        write_token_list(data.arguments, dest, _cx)?;
        if data.kind.is_variable()
            && matches!(data.arguments.last(), Some(value) if matches!(value.data(ast), TokenOrValueData::Token(token) if matches!(token.data(ast), TokenData::Comma)))
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
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
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

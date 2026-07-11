use crate::prelude::*;

impl ToCss for Token<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        use cssparser::{CowRcStr, ToCss as CssParserToCss, Token as CssToken};

        match self {
            Self::Ident(value) => CssToken::Ident(CowRcStr::from(*value)).to_css(dest),
            Self::AtKeyword(value) => CssToken::AtKeyword(CowRcStr::from(*value)).to_css(dest),
            Self::Hash(value) => CssToken::Hash(CowRcStr::from(*value)).to_css(dest),
            Self::IdHash(value) => CssToken::IDHash(CowRcStr::from(*value)).to_css(dest),
            Self::String(value) => CssToken::QuotedString(CowRcStr::from(*value)).to_css(dest),
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
            Self::Dimension { unit, value } => serialize_dimension(*value, unit, dest),
            Self::WhiteSpace(value) => {
                if dest.prettify() {
                    dest.write_str(value)
                } else {
                    dest.write_char(' ')
                }
            }
            Self::Comment(value) => {
                if dest.prettify() {
                    dest.write_str("/*")?;
                    dest.write_str(value)?;
                    dest.write_str("*/")
                } else {
                    Ok(())
                }
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

pub(crate) fn write_token_list<PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
) -> fmt::Result {
    for value in values {
        value.to_css(dest)?;
    }
    Ok(())
}

pub(crate) fn write_token_list_trimmed<PrinterT: PrinterTrait>(
    values: &[TokenOrValue<'_>],
    dest: &mut PrinterT,
) -> fmt::Result {
    let start = values
        .iter()
        .position(|value| {
            !matches!(value, TokenOrValue::Token(token) if matches!(**token, Token::WhiteSpace(_)))
        })
        .unwrap_or(values.len());
    write_token_list(&values[start..], dest)
}

fn starts_with_whitespace(values: &[TokenOrValue<'_>]) -> bool {
    matches!(values.first(), Some(TokenOrValue::Token(token)) if matches!(**token, Token::WhiteSpace(_)))
}

impl ToCss for TokenOrValue<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Token(value) => value.to_css(dest),
            Self::Color(value) => value.to_css(dest),
            Self::UnresolvedColor(value) => value.to_css(dest),
            Self::Url(value) => value.to_css(dest),
            Self::Var(value) => value.to_css(dest),
            Self::Env(value) => value.to_css(dest),
            Self::Function(value) => value.to_css(dest),
            Self::Length(value) => serialize_dimension(value.value, length_unit(value), dest),
            Self::Angle(value) => value.to_css(dest),
            Self::Time(value) => value.to_css(dest),
            Self::Resolution(value) => value.to_css(dest),
            Self::DashedIdent(value) => write_dashed_ident(value, dest),
            Self::AnimationName(value) => value.to_css(dest),
        }
    }
}

fn length_unit(value: &LengthValue) -> &'static str {
    match value.unit {
        LengthUnit::Px => "px",
        LengthUnit::In => "in",
        LengthUnit::Cm => "cm",
        LengthUnit::Mm => "mm",
        LengthUnit::Q => "q",
        LengthUnit::Pt => "pt",
        LengthUnit::Pc => "pc",
        LengthUnit::Em => "em",
        LengthUnit::Rem => "rem",
        LengthUnit::Ex => "ex",
        LengthUnit::Rex => "rex",
        LengthUnit::Ch => "ch",
        LengthUnit::Rch => "rch",
        LengthUnit::Cap => "cap",
        LengthUnit::Rcap => "rcap",
        LengthUnit::Ic => "ic",
        LengthUnit::Ric => "ric",
        LengthUnit::Lh => "lh",
        LengthUnit::Rlh => "rlh",
        LengthUnit::Vw => "vw",
        LengthUnit::Lvw => "lvw",
        LengthUnit::Svw => "svw",
        LengthUnit::Dvw => "dvw",
        LengthUnit::Cqw => "cqw",
        LengthUnit::Vh => "vh",
        LengthUnit::Lvh => "lvh",
        LengthUnit::Svh => "svh",
        LengthUnit::Dvh => "dvh",
        LengthUnit::Cqh => "cqh",
        LengthUnit::Vi => "vi",
        LengthUnit::Svi => "svi",
        LengthUnit::Lvi => "lvi",
        LengthUnit::Dvi => "dvi",
        LengthUnit::Cqi => "cqi",
        LengthUnit::Vb => "vb",
        LengthUnit::Svb => "svb",
        LengthUnit::Lvb => "lvb",
        LengthUnit::Dvb => "dvb",
        LengthUnit::Cqb => "cqb",
        LengthUnit::Vmin => "vmin",
        LengthUnit::Svmin => "svmin",
        LengthUnit::Lvmin => "lvmin",
        LengthUnit::Dvmin => "dvmin",
        LengthUnit::Cqmin => "cqmin",
        LengthUnit::Vmax => "vmax",
        LengthUnit::Svmax => "svmax",
        LengthUnit::Lvmax => "lvmax",
        LengthUnit::Dvmax => "dvmax",
        LengthUnit::Cqmax => "cqmax",
    }
}

fn write_dashed_ident<PrinterT: PrinterTrait>(value: &str, dest: &mut PrinterT) -> fmt::Result {
    dest.write_str("--")?;
    serialize_name(value.strip_prefix("--").unwrap_or(value), dest)
}

impl ToCss for Url<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if !dest.prettify() {
            use cssparser::{CowRcStr, ToCss as CssParserToCss, Token as CssToken};

            let mut unquoted = String::new();
            CssToken::UnquotedUrl(CowRcStr::from(self.url)).to_css(&mut unquoted)?;
            let mut quoted = String::from("url(");
            cssparser::serialize_string(self.url, &mut quoted)?;
            quoted.push(')');
            return dest.write_str(if unquoted.len() <= quoted.len() {
                &unquoted
            } else {
                &quoted
            });
        }
        dest.write_str("url(")?;
        serialize_string(self.url, dest)?;
        dest.write_char(')')
    }
}

impl ToCss for Variable<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("var(")?;
        self.name.to_css(dest)?;
        if let Some(fallback) = &self.fallback {
            dest.write_char(',')?;
            if !starts_with_whitespace(fallback) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest)?;
        }
        dest.write_char(')')
    }
}

impl ToCss for EnvironmentVariable<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str("env(")?;
        self.name.to_css(dest)?;
        for index in &self.indices {
            dest.write_char(' ')?;
            write!(dest, "{index}")?;
        }
        if let Some(fallback) = &self.fallback {
            dest.write_char(',')?;
            if !starts_with_whitespace(fallback) {
                dest.whitespace()?;
            }
            write_token_list(fallback, dest)?;
        }
        dest.write_char(')')
    }
}

impl ToCss for EnvironmentVariableName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::UA(value) => value.to_css(dest),
            Self::Custom(value) => value.to_css(dest),
            Self::Unknown(value) => serialize_identifier(value, dest),
        }
    }
}

impl ToCss for UAEnvironmentVariable {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::SafeAreaInsetTop => "safe-area-inset-top",
            Self::SafeAreaInsetRight => "safe-area-inset-right",
            Self::SafeAreaInsetBottom => "safe-area-inset-bottom",
            Self::SafeAreaInsetLeft => "safe-area-inset-left",
            Self::ViewportSegmentWidth => "viewport-segment-width",
            Self::ViewportSegmentHeight => "viewport-segment-height",
            Self::ViewportSegmentTop => "viewport-segment-top",
            Self::ViewportSegmentLeft => "viewport-segment-left",
            Self::ViewportSegmentBottom => "viewport-segment-bottom",
            Self::ViewportSegmentRight => "viewport-segment-right",
        })
    }
}

impl ToCss for DashedIdentReference<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_dashed_ident(self.ident, dest)?;
        if let Some(from) = &self.from {
            dest.write_str(" from ")?;
            from.to_css(dest)?;
        }
        Ok(())
    }
}

impl ToCss for Specifier<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Global => dest.write_str("global"),
            Self::File(value) => serialize_string(value, dest),
            Self::SourceIndex(_) => Ok(()),
        }
    }
}

impl ToCss for Function<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        serialize_identifier(self.name, dest)?;
        dest.write_char('(')?;
        write_token_list(&self.arguments, dest)?;
        dest.write_char(')')
    }
}

impl ToCss for AnimationName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
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

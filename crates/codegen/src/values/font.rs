use super::*;

impl<'ghost> ToCss<'ghost> for FontFamily<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Custom(value) => write_custom_font_family(_cx.ast_context().str(*value), dest),
            Self::Unparsed(value) => {
                crate::token::write_token_list(_cx.ast_context().vec_iter(*value), dest, _cx)
            }
            Self::Tombstone => Ok(()),
            _ => dest.write_str(
                self.as_css_str()
                    .expect("known font families are static keywords"),
            ),
        }
    }
}

pub(crate) fn write_custom_font_family<PrinterT: PrinterTrait>(
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    let requires_quotes = value.is_empty()
        || FontFamily::from_known_name(value).is_some()
        || value
            .split_ascii_whitespace()
            .any(|part| FontFamily::from_known_name(part).is_some())
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.contains("  ")
        || value
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() && byte != b' ');
    if requires_quotes {
        return serialize_string(value, dest);
    }

    let mut identifier = String::new();
    for (index, part) in value.split(' ').enumerate() {
        if index > 0 {
            identifier.push(' ');
        }
        serialize_identifier(part, &mut identifier)?;
    }
    let mut string = String::new();
    serialize_string(value, &mut string)?;
    if identifier.len() < string.len() {
        dest.write_str(&identifier)
    } else {
        dest.write_str(&string)
    }
}

keyword_values! {
    AbsoluteFontSize,
    RelativeFontSize,
    FontStretchKeyword,
    FontVariantCaps,
    VerticalAlignKeyword,
}

impl<'ghost> ToCss<'ghost> for FontWeight {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Absolute(value) => value.to_css(dest, _cx),
            Self::Bolder => dest.write_str("bolder"),
            Self::Lighter => dest.write_str("lighter"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AbsoluteFontWeight {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Weight(value) => serialize_number(*value, dest),
            Self::Normal => dest.write_str("normal"),
            Self::Bold => dest.write_str("bold"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Absolute(value) => value.to_css(dest, _cx),
            Self::Relative(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontStretch {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontStyle {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Italic => dest.write_str("italic"),
            Self::Oblique(value) => {
                dest.write_str("oblique")?;
                dest.write_char(' ')?;
                value.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for LineHeight<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for VerticalAlign<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'a, 'ghost> ToCss<'ghost> for AstVec<'a, NodeId<'a, FontFamily<'a>>> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let mut first = true;
        for family in _cx
            .ast_context()
            .vec_iter(*self)
            .filter(|family| !_cx.ast_context().resolve_node(*family).is_tombstone())
        {
            if !first {
                dest.delim(Delimiter::Comma)?;
            }
            family.to_css(dest, _cx)?;
            first = false;
        }
        Ok(())
    }
}

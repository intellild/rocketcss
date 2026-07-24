use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for FontFamily<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Custom(value) => write_custom_font_family(value, dest),
            Self::Unparsed(value) => crate::token::write_token_list(value, dest, _cx),
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
        || !matches!(FontFamily::from_name(value), FontFamily::Custom(_))
        || value
            .split_ascii_whitespace()
            .any(|part| !matches!(FontFamily::from_name(part), FontFamily::Custom(_)))
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

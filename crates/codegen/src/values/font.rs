use crate::prelude::*;

impl ToCss for FontFamily<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Custom(value) => write_custom_font_family(value, dest),
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
            .any(|part| FontFamily::from_name(part).is_generic())
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.contains("  ")
        || value
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() && byte != b' ');
    if requires_quotes {
        return serialize_string(value, dest);
    }

    for (index, part) in value.split(' ').enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        serialize_identifier(part, dest)?;
    }
    Ok(())
}

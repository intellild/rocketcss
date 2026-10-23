use crate::prelude::*;

impl<'ast, 'ghost> ToCss<'ghost> for FontFamily<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            FontFamilyData::Custom(value) => write_custom_font_family(value, dest),
            FontFamilyData::Unparsed(value) => crate::token::write_token_list(value, dest, _cx),
            FontFamilyData::Tombstone => Ok(()),
            FontFamilyData::Serif => dest.write_str("serif"),
            FontFamilyData::SansSerif => dest.write_str("sans-serif"),
            FontFamilyData::Cursive => dest.write_str("cursive"),
            FontFamilyData::Fantasy => dest.write_str("fantasy"),
            FontFamilyData::Monospace => dest.write_str("monospace"),
            FontFamilyData::SystemUi => dest.write_str("system-ui"),
            FontFamilyData::Emoji => dest.write_str("emoji"),
            FontFamilyData::Math => dest.write_str("math"),
            FontFamilyData::Fangsong => dest.write_str("fangsong"),
            FontFamilyData::UiSerif => dest.write_str("ui-serif"),
            FontFamilyData::UiSansSerif => dest.write_str("ui-sans-serif"),
            FontFamilyData::UiMonospace => dest.write_str("ui-monospace"),
            FontFamilyData::UiRounded => dest.write_str("ui-rounded"),
            FontFamilyData::Initial => dest.write_str("initial"),
            FontFamilyData::Inherit => dest.write_str("inherit"),
            FontFamilyData::Unset => dest.write_str("unset"),
            FontFamilyData::Default => dest.write_str("default"),
            FontFamilyData::Revert => dest.write_str("revert"),
            FontFamilyData::RevertLayer => dest.write_str("revert-layer"),
        }
    }
}

pub(crate) fn write_custom_font_family<PrinterT: PrinterTrait>(
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    let requires_quotes = value.is_empty()
        || FontFamily::is_generic_name(value)
        || value
            .split_ascii_whitespace()
            .any(FontFamily::is_generic_name)
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

use super::*;

impl<'ghost> ToCss<'ghost> for Font<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        let font = cx.ast_context().font(id);
        let (style, weight) = font.style_and_weight();
        style.to_css(dest, cx)?;
        dest.write_char(' ')?;
        font.variant_caps().to_css(dest, cx)?;
        dest.write_char(' ')?;
        weight.to_css(dest, cx)?;
        dest.write_char(' ')?;
        font.stretch().to_css(dest, cx)?;
        dest.write_char(' ')?;
        font.size().to_css(dest, cx)?;
        dest.write_str(" / ")?;
        font.line_height().to_css(dest, cx)?;
        dest.write_char(' ')?;
        write_comma_separated(cx.ast_context().vec_iter(font.family()), dest, cx)
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.style.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.variant_caps.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.weight.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.stretch.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        self.size.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.line_height.to_css(dest, _cx)?;
        dest.write_char(' ')?;
        write_comma_separated(_cx.ast_context().vec_iter(self.family), dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for FamilyName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        crate::values::font::write_custom_font_family(_cx.ast_context().str(self.0), dest)
    }
}

impl<'ghost> ToCss<'ghost> for FontFaceProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Source(values) => {
                write_comma_separated(_cx.ast_context().vec_iter(*values), dest, _cx)
            }
            Self::FontFamily(value) => value.to_css(dest, _cx),
            Self::FontStyle(value) => value.to_css(dest, _cx),
            Self::FontWeight(value) => value.to_css(dest, _cx),
            Self::FontStretch(value) => value.to_css(dest, _cx),
            Self::UnicodeRange(values) => {
                write_comma_separated(_cx.ast_context().vec_iter(*values), dest, _cx)
            }
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl NamedProperty for FontFaceProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a AstContext<'_>) -> &'a str {
        match self {
            FontFaceProperty::Source(_) => "src",
            FontFaceProperty::FontFamily(_) => "font-family",
            FontFaceProperty::FontStyle(_) => "font-style",
            FontFaceProperty::FontWeight(_) => "font-weight",
            FontFaceProperty::FontStretch(_) => "font-stretch",
            FontFaceProperty::UnicodeRange(_) => "unicode-range",
            FontFaceProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => {
                        ast.str(name)
                    }
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Source<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Local(value) => {
                dest.write_str("local(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontFormat<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        let value = match self {
            Self::String(value) => _cx.ast_context().str(*value),
            value => value
                .as_css_str()
                .expect("custom font format handled separately"),
        };
        serialize_string(value, dest)
    }
}

impl<'ghost> ToCss<'ghost> for FontTechnology {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font technologies are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for FontFaceStyle<'_> {
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
                let value = _cx.ast_context().resolve_node(*value);
                let is_default = matches!(
                    (
                        _cx.ast_context().resolve_node(value.0),
                        _cx.ast_context().resolve_node(value.1),
                    ),
                    (Angle::Deg(first), Angle::Deg(second)) if first == 14.0 && second == 14.0
                );
                if !is_default {
                    dest.write_char(' ')?;
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontPaletteValuesProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::FontFamily(value) => value.to_css(dest, _cx),
            Self::BasePalette(value) => value.to_css(dest, _cx),
            Self::OverrideColors(values) => {
                write_comma_separated(_cx.ast_context().vec_iter(*values), dest, _cx)
            }
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl NamedProperty for FontPaletteValuesProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a AstContext<'_>) -> &'a str {
        match self {
            FontPaletteValuesProperty::FontFamily(_) => "font-family",
            FontPaletteValuesProperty::BasePalette(_) => "base-palette",
            FontPaletteValuesProperty::OverrideColors(_) => "override-colors",
            FontPaletteValuesProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => {
                        ast.str(name)
                    }
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for BasePalette {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Light => dest.write_str("light"),
            Self::Dark => dest.write_str("dark"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for FontFeatureSubruleType {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("font feature subrule types are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for UrlSource<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.url.to_css(dest, _cx)?;
        if let Some(format) = &self.format {
            dest.write_str(" format(")?;
            format.to_css(dest, _cx)?;
            dest.write_char(')')?;
        }
        if !self.tech.is_empty() {
            dest.write_str(" tech(")?;
            write_comma_separated(_cx.ast_context().vec_iter(self.tech), dest, _cx)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for UnicodeRange {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        for wildcard_digits in 1..=6 {
            let bits = wildcard_digits * 4;
            let mask = (1_u32 << bits) - 1;
            if self.start & mask == 0 && self.end == self.start | mask {
                dest.write_str("U+")?;
                serialize_hex(self.start >> bits, 1, true, dest)?;
                for _ in 0..wildcard_digits {
                    dest.write_char('?')?;
                }
                return Ok(());
            }
        }
        dest.write_str("U+")?;
        serialize_hex(self.start, 1, true, dest)?;
        if self.start != self.end {
            dest.write_char('-')?;
            serialize_hex(self.end, 1, true, dest)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for OverrideColors<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_int(self.index, dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for FontFeatureDeclaration<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        serialize_identifier(_cx.ast_context().str(self.name), dest)?;
        dest.delim(Delimiter::Colon)?;
        for (index, value) in _cx.ast_context().vec_iter(self.values).enumerate() {
            if index > 0 {
                dest.write_char(' ')?;
            }
            serialize_int(value, dest)?;
        }
        Ok(())
    }
}

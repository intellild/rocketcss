use super::*;

impl<'ghost> ToCss<'ghost> for ViewTransitionProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Navigation(value) => value.to_css(dest, _cx),
            Self::Types(value) => value.to_css(dest, _cx),
            Self::Custom(value) => value.to_css(dest, _cx),
        }
    }
}

impl NamedProperty for ViewTransitionProperty<'_> {
    fn css_name<'a>(&'a self, ast: &'a AstContext<'_>) -> &'a str {
        match self {
            ViewTransitionProperty::Navigation(_) => "navigation",
            ViewTransitionProperty::Types(_) => "types",
            ViewTransitionProperty::Custom(value) => {
                match ast.resolve_node(ast.resolve_node(*value).name) {
                    CustomPropertyName::Custom(name) | CustomPropertyName::Unknown(name) => {
                        ast.str(name)
                    }
                }
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Navigation {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("navigation values are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ViewTransitionPartSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(name) = &self.name {
            name.to_css(dest, _cx)?;
        }
        for class in _cx.ast_context().vec_iter(self.classes) {
            dest.write_char('.')?;
            serialize_identifier(_cx.ast_context().str(class), dest)?;
        }
        Ok(())
    }
}

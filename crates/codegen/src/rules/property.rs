use super::*;

impl<'ghost> ToCss<'ghost> for UnparsedProperty<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: rocketcss_ast::AstNodeStorage<'id>,
    {
        let property = cx.ast_context().unparsed_property(id);
        if let Some(raw) = property.raw_value() {
            dest.write_str(cx.ast_context().str(raw))
        } else {
            crate::token::write_unparsed_token_list(
                cx.ast_context().vec_iter(property.value()),
                dest,
                cx,
            )
        }
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if let Some(raw_value) = self.raw_value {
            dest.write_str(_cx.ast_context().str(raw_value))
        } else {
            crate::token::write_unparsed_token_list(
                _cx.ast_context().vec_iter(self.value),
                dest,
                _cx,
            )
        }
    }
}

impl<'ghost> ToCss<'ghost> for ParsedComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Length(value) => value.to_css(dest, _cx),
            Self::Number(value) => serialize_number(*value, dest),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::String(value) => serialize_string(_cx.ast_context().str(*value), dest),
            Self::Color(value) => value.to_css(dest, _cx),
            Self::Image(value) => value.to_css(dest, _cx),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Integer(value) => serialize_int(*value, dest),
            Self::Angle(value) => value.to_css(dest, _cx),
            Self::Time(value) => value.to_css(dest, _cx),
            Self::Resolution(value) => value.to_css(dest, _cx),
            Self::TransformFunction(value) => value.to_css(dest, _cx),
            Self::TransformList(values) => {
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::CustomIdent(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
            Self::Literal(value) => dest.write_str(_cx.ast_context().str(*value)),
            Self::Repeated {
                components,
                multiplier,
            } => {
                let delimiter = match multiplier {
                    Multiplier::None => "",
                    Multiplier::Space => " ",
                    Multiplier::Comma => ", ",
                };
                for (index, value) in _cx.ast_context().vec_iter(*components).enumerate() {
                    if index > 0 {
                        dest.write_str(delimiter)?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::TokenList(values) => {
                crate::token::write_token_list(_cx.ast_context().vec_iter(*values), dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for CustomProperty<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        crate::token::write_token_list(_cx.ast_context().vec_iter(self.value), dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Multiplier {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(match self {
            Self::None => "",
            Self::Space => "+",
            Self::Comma => "#",
        })
    }
}

impl<'ghost> ToCss<'ghost> for SyntaxString<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Universal => dest.write_char('*'),
            Self::Components(values) => {
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.write_str(" | ")?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for SyntaxComponentKind<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Literal(value) => dest.write_str(_cx.ast_context().str(*value)),
            value => {
                dest.write_char('<')?;
                dest.write_str(
                    value
                        .as_css_str()
                        .expect("literal syntax component handled separately"),
                )?;
                dest.write_char('>')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for SyntaxComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.kind.to_css(dest, _cx)?;
        self.multiplier.to_css(dest, _cx)
    }
}

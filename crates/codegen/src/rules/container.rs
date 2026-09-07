use super::*;

impl<'ghost> ToCss<'ghost> for Container<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.name.to_css(dest, _cx)?;
        dest.write_str(" / ")?;
        self.container_type.to_css(dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for ContainerSizeFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("container size features are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ScrollStateFeatureId {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(
            self.as_css_str()
                .expect("scroll state features are static keywords"),
        )
    }
}

impl<'ghost> ToCss<'ghost> for ContainerCondition<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec_iter(*conditions).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    _cx.ast_context().resolve_node(value).to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Style(value) => {
                dest.write_str("style(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::ScrollState(value) => {
                dest.write_str("scroll-state(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Unknown(values) => {
                crate::token::write_token_list(_cx.ast_context().vec_iter(*values), dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for StyleQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Declaration(value) => _cx
                .ast_context()
                .declaration_at_index(value.index())
                .and_then(|record| record.payload().as_property())
                .expect("a style query declaration must reference a property declaration")
                .to_css(dest, _cx),
            Self::Property(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec_iter(*conditions).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    _cx.ast_context().resolve_node(value).to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for ScrollStateQuery<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Feature(value) => value.to_css(dest, _cx),
            Self::Not(value) => {
                dest.write_str("not ")?;
                value.to_css(dest, _cx)
            }
            Self::Operation {
                conditions,
                operator,
            } => {
                for (index, value) in _cx.ast_context().vec_iter(*conditions).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                        operator.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                    }
                    _cx.ast_context().resolve_node(value).to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

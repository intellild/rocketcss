use super::*;

keyword_values! {
    DisplayKeyword,
    DisplayOutside,
    Visibility,
    BoxSizing,
    OverflowKeyword,
    TextOverflow,
    BoxDecorationBreak,
}

impl<'ghost> ToCss<'ghost> for Display {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Keyword(value) => value.to_css(dest, _cx),
            Self::Pair {
                inside,
                is_list_item,
                outside,
            } => {
                if *is_list_item
                    && matches!(outside, DisplayOutside::Block)
                    && matches!(inside, DisplayInside::Flow)
                {
                    return dest.write_str("list-item");
                }
                match (outside, inside) {
                    (DisplayOutside::Block, DisplayInside::Flow) => dest.write_str("block")?,
                    (DisplayOutside::Inline, DisplayInside::Flow) => dest.write_str("inline")?,
                    (DisplayOutside::Block, DisplayInside::FlowRoot) => {
                        dest.write_str("flow-root")?
                    }
                    (DisplayOutside::Inline, DisplayInside::FlowRoot) => {
                        dest.write_str("inline-block")?
                    }
                    (DisplayOutside::Block, DisplayInside::Flex { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Inline, DisplayInside::Flex { vendor_prefix }) => {
                        dest.write_str("inline-")?;
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("flex")?;
                    }
                    (DisplayOutside::Block, DisplayInside::Box { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("box")?;
                    }
                    (DisplayOutside::Inline, DisplayInside::Box { vendor_prefix }) => {
                        vendor_prefix.to_css(dest, _cx)?;
                        dest.write_str("inline-box")?;
                    }
                    (DisplayOutside::Block, DisplayInside::Grid) => dest.write_str("grid")?,
                    (DisplayOutside::Inline, DisplayInside::Grid) => {
                        dest.write_str("inline-grid")?
                    }
                    (DisplayOutside::Inline, DisplayInside::Ruby) => dest.write_str("ruby")?,
                    (DisplayOutside::Block, DisplayInside::Table) => dest.write_str("table")?,
                    (DisplayOutside::Inline, DisplayInside::Table) => {
                        dest.write_str("inline-table")?
                    }
                    _ => {
                        outside.to_css(dest, _cx)?;
                        dest.write_char(' ')?;
                        inside.to_css(dest, _cx)?;
                    }
                }
                if *is_list_item {
                    dest.write_str(" list-item")?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for DisplayInside {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Flow => dest.write_str("flow"),
            Self::FlowRoot => dest.write_str("flow-root"),
            Self::Table => dest.write_str("table"),
            Self::Flex { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("flex")
            }
            Self::Box { vendor_prefix } => {
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("box")
            }
            Self::Grid => dest.write_str("grid"),
            Self::Ruby => dest.write_str("ruby"),
        }
    }
}

fn write_prefixed_keyword<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    value: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    prefix.to_css(dest, cx)?;
    dest.write_str(value)
}

impl<'ghost> ToCss<'ghost> for Size<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::MathFunction(value) => value.to_css(dest, _cx),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest, _cx)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest, _cx)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest, _cx)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest, _cx)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for MaxSize<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::MathFunction(value) => value.to_css(dest, _cx),
            Self::MinContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "min-content", dest, _cx)
            }
            Self::MaxContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "max-content", dest, _cx)
            }
            Self::FitContent { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "fit-content", dest, _cx)
            }
            Self::FitContentFunction(value) => {
                dest.write_str("fit-content(")?;
                value.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Stretch { vendor_prefix } => {
                write_prefixed_keyword(vendor_prefix, "stretch", dest, _cx)
            }
            Self::Contain => dest.write_str("contain"),
        }
    }
}

impl<'ghost> ToCss<'ghost> for PositionProperty {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Static => dest.write_str("static"),
            Self::Relative => dest.write_str("relative"),
            Self::Absolute => dest.write_str("absolute"),
            Self::Sticky(prefix) => write_prefixed_keyword(prefix, "sticky", dest, _cx),
            Self::Fixed => dest.write_str("fixed"),
        }
    }
}

impl<'ast, 'ghost, T> ToCss<'ghost> for Size2D<'ast, T>
where
    T: ToCss<'ghost> + PartialEq + AstNodeStorage<'ast>,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.0.to_css(dest, _cx)?;
        if self.0 != self.1 {
            dest.write_char(' ')?;
            self.1.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ast, 'ghost, T> ToCss<'ghost> for Rect<'ast, T>
where
    T: ToCss<'ghost> + PartialEq + AstNodeStorage<'ast>,
{
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        self.0.to_css(dest, _cx)?;
        if self.0 == self.1 && self.0 == self.2 && self.0 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.1.to_css(dest, _cx)?;
        if self.0 == self.2 && self.1 == self.3 {
            return Ok(());
        }
        dest.write_char(' ')?;
        self.2.to_css(dest, _cx)?;
        if self.1 != self.3 {
            dest.write_char(' ')?;
            self.3.to_css(dest, _cx)?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for ZIndex {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(value) => serialize_int(*value, dest),
        }
    }
}

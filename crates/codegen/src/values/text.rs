use super::*;

keyword_values! {
    TextTransformCase,
    WhiteSpace,
    WordBreak,
    LineBreak,
    Hyphens,
    OverflowWrap,
    TextAlign,
    TextAlignLast,
    TextJustify,
    ExclusiveTextDecorationLine,
    OtherTextDecorationLine,
    TextDecorationStyle,
    TextDecorationSkipInk,
    TextEmphasisFillMode,
    TextEmphasisShape,
    TextEmphasisPositionHorizontal,
    TextEmphasisPositionVertical,
    TextDirection,
    UnicodeBidi,
}

impl<'ghost> ToCss<'ghost> for Content<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        crate::token::write_token_list(cx.ast_context().vec_iter(self.value), dest, cx)
    }
}

impl<'ghost> ToCss<'ghost> for Spacing<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextDecorationLine<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::ExclusiveTextDecorationLine(value) => value.to_css(dest, _cx),
            Self::Value(values) => {
                for (index, value) in _cx.ast_context().vec_iter(*values).enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    value.to_css(dest, _cx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextDecorationThickness<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::FromFont => dest.write_str("from-font"),
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextEmphasisStyle<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Keyword { fill, shape } => {
                fill.to_css(dest, _cx)?;
                if let Some(shape) = shape {
                    dest.write_char(' ')?;
                    shape.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::String(value) => serialize_string(_cx.ast_context().str(*value), dest),
        }
    }
}

impl<'ghost> ToCss<'ghost> for TextSizeAdjust {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::None => dest.write_str("none"),
            Self::Percentage(value) => {
                serialize_number(*value * 100.0, dest)?;
                dest.write_char('%')
            }
        }
    }
}

use super::*;

keyword_values! {
    GeometryBox,
}

impl<'ghost> ToCss<'ghost> for ClipPath<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Url(value) => value.to_css(dest, _cx),
            Self::Shape {
                reference_box,
                shape,
            } => {
                shape.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                reference_box.to_css(dest, _cx)
            }
            Self::Box(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for BasicShape<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Inset(value) => value.to_css(dest, _cx),
            Self::Circle(value) => value.to_css(dest, _cx),
            Self::Ellipse(value) => value.to_css(dest, _cx),
            Self::Polygon(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for ShapeRadius<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(dest, _cx),
            Self::ClosestSide => dest.write_str("closest-side"),
            Self::FarthestSide => dest.write_str("farthest-side"),
        }
    }
}

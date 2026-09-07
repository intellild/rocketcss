use super::*;

keyword_values! {
    TransformStyle,
    TransformBox,
    BackfaceVisibility,
}

fn write_comma_values<'ghost, PrinterT: PrinterTrait, T: ToCss<'ghost>>(
    values: &[&T],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        value.to_css(dest, cx)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for Transform<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        write_transform(cx.ast_context().transform(id), dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_transform((*self).into(), dest, cx)
    }
}
fn write_transform<'ghost, PrinterT: PrinterTrait>(
    value: TransformRead<'_, '_, '_>,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    match &value {
        TransformRead::Translate((x, y)) => write_function_values("translate", dest, |dest| {
            write_comma_values(&[x, y], dest, _cx)
        }),
        TransformRead::TranslateX(value) => {
            write_function_values("translateX", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::TranslateY(value) => {
            write_function_values("translateY", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::TranslateZ(value) => {
            write_function_values("translateZ", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::Translate3d((x, y, z)) => {
            write_function_values("translate3d", dest, |dest| {
                x.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                y.to_css(dest, _cx)?;
                dest.delim(Delimiter::Comma)?;
                z.get().to_css(dest, _cx)
            })
        }
        TransformRead::Scale((x, y)) => {
            write_function_values("scale", dest, |dest| write_comma_values(&[x, y], dest, _cx))
        }
        TransformRead::ScaleX(value) => {
            write_function_values("scaleX", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::ScaleY(value) => {
            write_function_values("scaleY", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::ScaleZ(value) => {
            write_function_values("scaleZ", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::Scale3d((x, y, z)) => write_function_values("scale3d", dest, |dest| {
            x.to_css(dest, _cx)?;
            dest.delim(Delimiter::Comma)?;
            y.to_css(dest, _cx)?;
            dest.delim(Delimiter::Comma)?;
            z.get().to_css(dest, _cx)
        }),
        TransformRead::Rotate(value) => {
            write_function_values("rotate", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::RotateX(value) => {
            write_function_values("rotateX", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::RotateY(value) => {
            write_function_values("rotateY", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::RotateZ(value) => {
            write_function_values("rotateZ", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::Rotate3d((x, y, tail)) => write_function_values("rotate3d", dest, |dest| {
            let (z, angle) = tail.get();
            for value in [x, y, &z] {
                serialize_number(*value, dest)?;
                dest.delim(Delimiter::Comma)?;
            }
            angle.to_css(dest, _cx)
        }),
        TransformRead::Skew((x, y)) => {
            write_function_values("skew", dest, |dest| write_comma_values(&[x, y], dest, _cx))
        }
        TransformRead::SkewX(value) => {
            write_function_values("skewX", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::SkewY(value) => {
            write_function_values("skewY", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::Perspective(value) => {
            write_function_values("perspective", dest, |dest| value.to_css(dest, _cx))
        }
        TransformRead::Matrix(value) => value.to_css(dest, _cx),
        TransformRead::Matrix3d(value) => value.to_css(dest, _cx),
    }
}

impl<'ghost> ToCss<'ghost> for Perspective<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Length(value) => value.to_css(dest, _cx),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Translate<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Xyz { x, y, z } => {
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                z.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for Scale {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::None => dest.write_str("none"),
            Self::Xyz { x, y, z } => {
                x.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                y.to_css(dest, _cx)?;
                dest.write_char(' ')?;
                z.to_css(dest, _cx)
            }
        }
    }
}

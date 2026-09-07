use super::*;

fn write_numbers<PrinterT: PrinterTrait>(
    values: impl IntoIterator<Item = f32>,
    dest: &mut PrinterT,
) -> fmt::Result {
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            dest.delim(Delimiter::Comma)?;
        }
        serialize_number(value, dest)?;
    }
    Ok(())
}

impl<'ghost> ToCss<'ghost> for MatrixForFloat {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: Sized + AstNodeStorage<'id>,
    {
        let (head, tail) = cx.ast_context().matrix_components(id);
        dest.write_str("matrix(")?;
        write_numbers(head.into_iter().chain(tail), dest)?;
        dest.write_char(')')
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("matrix(")?;
        write_numbers([self.a, self.b, self.c, self.d, self.e, self.f], dest)?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Matrix3DForFloat {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: Sized + AstNodeStorage<'id>,
    {
        let (head, tail) = cx.ast_context().matrix_3d_components(id);
        dest.write_str("matrix3d(")?;
        write_numbers(head.into_iter().chain(tail), dest)?;
        dest.write_char(')')
    }

    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str("matrix3d(")?;
        write_numbers(
            [
                self.m11, self.m12, self.m13, self.m14, self.m21, self.m22, self.m23, self.m24,
                self.m31, self.m32, self.m33, self.m34, self.m41, self.m42, self.m43, self.m44,
            ],
            dest,
        )?;
        dest.write_char(')')
    }
}

impl<'ghost> ToCss<'ghost> for Rotate {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        if self.x == 0.0 && self.y == 0.0 && self.z == 1.0 {
            return self.angle.to_css(dest, _cx);
        }
        write_numbers([self.x, self.y, self.z], dest)?;
        dest.write_char(' ')?;
        self.angle.to_css(dest, _cx)
    }
}

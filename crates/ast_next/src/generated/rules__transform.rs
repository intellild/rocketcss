use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MatrixForFloat<'ast>(pub(crate) NodeId<'ast, MatrixForFloatData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatrixForFloatData {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}
impl<'ast> NodeStorage<'ast> for MatrixForFloat<'ast> {
    const KIND: NodeKind = super::kind::KIND_MATRIX_FOR_FLOAT;
    type Data = MatrixForFloatData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, MatrixForFloatData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, MatrixForFloatData>) -> Self {
        Self(id)
    }
}
impl<'ast> MatrixForFloat<'ast> {
    #[inline]
    pub fn a(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).a
    }
    #[inline]
    pub fn set_a(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.a = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn b(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).b
    }
    #[inline]
    pub fn set_b(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.b = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn c(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).c
    }
    #[inline]
    pub fn set_c(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.c = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn d(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).d
    }
    #[inline]
    pub fn set_d(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.d = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn e(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).e
    }
    #[inline]
    pub fn set_e(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.e = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn f(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).f
    }
    #[inline]
    pub fn set_f(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.f = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for MatrixForFloat<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for MatrixForFloat<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_matrix_for_float(
        &mut self,
        data: MatrixForFloatData,
        span: Span,
    ) -> MatrixForFloat<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Matrix3DForFloat<'ast>(pub(crate) NodeId<'ast, Matrix3DForFloatData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3DForFloatData {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}
impl<'ast> NodeStorage<'ast> for Matrix3DForFloat<'ast> {
    const KIND: NodeKind = super::kind::KIND_MATRIX_3_D_FOR_FLOAT;
    type Data = Matrix3DForFloatData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, Matrix3DForFloatData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, Matrix3DForFloatData>) -> Self {
        Self(id)
    }
}
impl<'ast> Matrix3DForFloat<'ast> {
    #[inline]
    pub fn m11(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m11
    }
    #[inline]
    pub fn set_m11(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m11 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m12(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m12
    }
    #[inline]
    pub fn set_m12(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m12 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m13(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m13
    }
    #[inline]
    pub fn set_m13(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m13 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m14(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m14
    }
    #[inline]
    pub fn set_m14(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m14 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m21(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m21
    }
    #[inline]
    pub fn set_m21(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m21 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m22(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m22
    }
    #[inline]
    pub fn set_m22(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m22 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m23(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m23
    }
    #[inline]
    pub fn set_m23(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m23 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m24(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m24
    }
    #[inline]
    pub fn set_m24(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m24 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m31(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m31
    }
    #[inline]
    pub fn set_m31(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m31 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m32(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m32
    }
    #[inline]
    pub fn set_m32(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m32 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m33(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m33
    }
    #[inline]
    pub fn set_m33(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m33 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m34(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m34
    }
    #[inline]
    pub fn set_m34(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m34 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m41(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m41
    }
    #[inline]
    pub fn set_m41(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m41 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m42(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m42
    }
    #[inline]
    pub fn set_m42(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m42 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m43(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m43
    }
    #[inline]
    pub fn set_m43(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m43 = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn m44(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).m44
    }
    #[inline]
    pub fn set_m44(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.m44 = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Matrix3DForFloat<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Matrix3DForFloat<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_matrix_3_d_for_float(
        &mut self,
        data: Matrix3DForFloatData,
        span: Span,
    ) -> Matrix3DForFloat<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rotate<'ast>(pub(crate) NodeId<'ast, RotateData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotateData {
    pub angle: Angle,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl<'ast> NodeStorage<'ast> for Rotate<'ast> {
    const KIND: NodeKind = super::kind::KIND_ROTATE;
    type Data = RotateData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, RotateData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, RotateData>) -> Self {
        Self(id)
    }
}
impl<'ast> Rotate<'ast> {
    #[inline]
    pub fn angle(self, ast: &Ast<'ast>) -> Angle {
        ast.data(self).angle
    }
    #[inline]
    pub fn set_angle(self, ast: &mut Ast<'ast>, value: Angle) {
        let mut data = ast.data(self);
        data.angle = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn x(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).x
    }
    #[inline]
    pub fn set_x(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.x = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn y(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).y
    }
    #[inline]
    pub fn set_y(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.y = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn z(self, ast: &Ast<'ast>) -> f32 {
        ast.data(self).z
    }
    #[inline]
    pub fn set_z(self, ast: &mut Ast<'ast>, value: f32) {
        let mut data = ast.data(self);
        data.z = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for Rotate<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Rotate<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_rotate(&mut self, data: RotateData, span: Span) -> Rotate<'ast> {
        self.alloc(data, span)
    }
}

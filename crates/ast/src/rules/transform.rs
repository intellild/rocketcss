use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub struct MatrixForFloat {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Matrix3DForFloat {
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

#[derive(Debug, PartialEq, Visit)]
pub struct Rotate<'a> {
    pub angle: Box<'a, Angle>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

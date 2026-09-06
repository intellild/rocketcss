use crate::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Transform<'a> {
    Translate(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
        ),
    ),
    TranslateX(NodeId<'a, LengthPercentage<'a>>),
    TranslateY(NodeId<'a, LengthPercentage<'a>>),
    TranslateZ(NodeId<'a, Length<'a>>),
    Translate3d(
        (
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, LengthPercentage<'a>>,
            NodeId<'a, Length<'a>>,
        ),
    ),
    Scale((NumberOrPercentage, NumberOrPercentage)),
    ScaleX(NumberOrPercentage),
    ScaleY(NumberOrPercentage),
    ScaleZ(NumberOrPercentage),
    Scale3d((NumberOrPercentage, NumberOrPercentage, NumberOrPercentage)),
    Rotate(Angle),
    RotateX(Angle),
    RotateY(Angle),
    RotateZ(Angle),
    Rotate3d((f32, f32, f32, Angle)),
    Skew((Angle, Angle)),
    SkewX(Angle),
    SkewY(Angle),
    Perspective(NodeId<'a, Length<'a>>),
    Matrix(NodeId<'a, MatrixForFloat>),
    Matrix3d(NodeId<'a, Matrix3DForFloat>),
}

#[derive(Clone, Copy)]
enum TransformAngleUnit {
    Deg,
    Rad,
    Grad,
    Turn,
}
impl TransformAngleUnit {
    fn split(value: Angle) -> (Self, f32) {
        match value {
            Angle::Deg(v) => (Self::Deg, v),
            Angle::Rad(v) => (Self::Rad, v),
            Angle::Grad(v) => (Self::Grad, v),
            Angle::Turn(v) => (Self::Turn, v),
        }
    }
    fn angle(self, value: f32) -> Angle {
        match self {
            Self::Deg => Angle::Deg(value),
            Self::Rad => Angle::Rad(value),
            Self::Grad => Angle::Grad(value),
            Self::Turn => Angle::Turn(value),
        }
    }
}

// Flatten only the variants whose nested scalar values cannot fit in 12 bytes.
#[repr(u8)]
#[derive(Clone, Copy)]
enum TransformData<'ast> {
    Translate(
        (
            NodeId<'ast, LengthPercentage<'ast>>,
            NodeId<'ast, LengthPercentage<'ast>>,
        ),
    ),
    TranslateX(NodeId<'ast, LengthPercentage<'ast>>),
    TranslateY(NodeId<'ast, LengthPercentage<'ast>>),
    TranslateZ(NodeId<'ast, Length<'ast>>),
    ScaleX(NumberOrPercentage),
    ScaleY(NumberOrPercentage),
    ScaleZ(NumberOrPercentage),
    Rotate(Angle),
    RotateX(Angle),
    RotateY(Angle),
    RotateZ(Angle),
    SkewX(Angle),
    SkewY(Angle),
    Perspective(NodeId<'ast, Length<'ast>>),
    Matrix(NodeId<'ast, MatrixForFloat>),
    Matrix3d(NodeId<'ast, Matrix3DForFloat>),
    Translate3d {
        x: NodeId<'ast, LengthPercentage<'ast>>,
        y: NodeId<'ast, LengthPercentage<'ast>>,
    },
    Scale {
        x_percentage: bool,
        y_percentage: bool,
        x: f32,
        y: f32,
    },
    Scale3d {
        x_percentage: bool,
        y_percentage: bool,
        z_percentage: bool,
        x: f32,
        y: f32,
    },
    Rotate3d {
        unit: TransformAngleUnit,
        x: f32,
        y: f32,
    },
    Skew {
        x_unit: TransformAngleUnit,
        y_unit: TransformAngleUnit,
        x: f32,
        y: f32,
    },
}
#[derive(Clone, Copy)]
struct TransformHeader<'ast> {
    data: TransformData<'ast>,
    extra: u32,
}
const _: () = {
    assert!(std::mem::size_of::<TransformHeader<'_>>() == 16);
};

pub use transform_access::{RotationTailRead, ScaleZRead, TransformFieldRead, TransformRead};
mod transform_access {
    use super::*;
    pub enum TransformRead<'context, 'storage, 'a> {
        Translate(
            (
                NodeId<'a, LengthPercentage<'a>>,
                NodeId<'a, LengthPercentage<'a>>,
            ),
        ),
        TranslateX(NodeId<'a, LengthPercentage<'a>>),
        TranslateY(NodeId<'a, LengthPercentage<'a>>),
        TranslateZ(NodeId<'a, Length<'a>>),
        Translate3d(
            (
                NodeId<'a, LengthPercentage<'a>>,
                NodeId<'a, LengthPercentage<'a>>,
                TransformFieldRead<'context, 'storage, NodeId<'a, Length<'a>>>,
            ),
        ),
        Scale((NumberOrPercentage, NumberOrPercentage)),
        ScaleX(NumberOrPercentage),
        ScaleY(NumberOrPercentage),
        ScaleZ(NumberOrPercentage),
        Scale3d(
            (
                NumberOrPercentage,
                NumberOrPercentage,
                ScaleZRead<'context, 'storage>,
            ),
        ),
        Rotate(Angle),
        RotateX(Angle),
        RotateY(Angle),
        RotateZ(Angle),
        Rotate3d((f32, f32, RotationTailRead<'context, 'storage>)),
        Skew((Angle, Angle)),
        SkewX(Angle),
        SkewY(Angle),
        Perspective(NodeId<'a, Length<'a>>),
        Matrix(NodeId<'a, MatrixForFloat>),
        Matrix3d(NodeId<'a, Matrix3DForFloat>),
    }

    enum FieldSource<'context, 'storage, T> {
        Stored {
            context: &'context AstContext<'storage>,
            extra: u32,
        },
        Value(T),
    }
    pub struct TransformFieldRead<'context, 'storage, T>(FieldSource<'context, 'storage, T>);
    impl<T: Copy> TransformFieldRead<'_, '_, T> {
        pub fn get(&self) -> T {
            match self.0 {
                FieldSource::Value(value) => value,
                FieldSource::Stored { context, extra } => {
                    // SAFETY: private construction matches T to the owning variant's native extra slot.
                    unsafe { context.extra_slot(extra as usize).read_value() }
                }
            }
        }
    }
    pub struct ScaleZRead<'context, 'storage>(ScaleZSource<'context, 'storage>);
    enum ScaleZSource<'context, 'storage> {
        Stored {
            percentage: bool,
            value: TransformFieldRead<'context, 'storage, f32>,
        },
        Value(NumberOrPercentage),
    }
    impl ScaleZRead<'_, '_> {
        pub fn get(&self) -> NumberOrPercentage {
            match &self.0 {
                ScaleZSource::Value(value) => *value,
                ScaleZSource::Stored { percentage, value } => {
                    number_value(*percentage, value.get())
                }
            }
        }
    }
    pub struct RotationTailRead<'context, 'storage>(RotationTailSource<'context, 'storage>);
    enum RotationTailSource<'context, 'storage> {
        Stored {
            unit: TransformAngleUnit,
            value: TransformFieldRead<'context, 'storage, [f32; 2]>,
        },
        Value(f32, Angle),
    }
    impl RotationTailRead<'_, '_> {
        pub fn get(&self) -> (f32, Angle) {
            match &self.0 {
                RotationTailSource::Value(z, angle) => (*z, *angle),
                RotationTailSource::Stored { unit, value } => {
                    let [z, angle] = value.get();
                    (z, unit.angle(angle))
                }
            }
        }
    }
    impl<'storage> AstContext<'storage> {
        pub fn transform<'id>(
            &self,
            id: NodeId<'id, Transform<'id>>,
        ) -> TransformRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning kind before the native header read.
            let header: TransformHeader<'id> = unsafe { self.node_payload(id).read_value() };
            match header.data {
                TransformData::Translate(value) => TransformRead::Translate(value),
                TransformData::TranslateX(value) => TransformRead::TranslateX(value),
                TransformData::TranslateY(value) => TransformRead::TranslateY(value),
                TransformData::TranslateZ(value) => TransformRead::TranslateZ(value),
                TransformData::ScaleX(value) => TransformRead::ScaleX(value),
                TransformData::ScaleY(value) => TransformRead::ScaleY(value),
                TransformData::ScaleZ(value) => TransformRead::ScaleZ(value),
                TransformData::Rotate(value) => TransformRead::Rotate(value),
                TransformData::RotateX(value) => TransformRead::RotateX(value),
                TransformData::RotateY(value) => TransformRead::RotateY(value),
                TransformData::RotateZ(value) => TransformRead::RotateZ(value),
                TransformData::SkewX(value) => TransformRead::SkewX(value),
                TransformData::SkewY(value) => TransformRead::SkewY(value),
                TransformData::Perspective(value) => TransformRead::Perspective(value),
                TransformData::Matrix(value) => TransformRead::Matrix(value),
                TransformData::Matrix3d(value) => TransformRead::Matrix3d(value),
                TransformData::Translate3d { x, y } => TransformRead::Translate3d((
                    x,
                    y,
                    TransformFieldRead(FieldSource::Stored {
                        context: self,
                        extra: header.extra,
                    }),
                )),
                TransformData::Scale {
                    x_percentage,
                    y_percentage,
                    x,
                    y,
                } => TransformRead::Scale((
                    number_value(x_percentage, x),
                    number_value(y_percentage, y),
                )),
                TransformData::Scale3d {
                    x_percentage,
                    y_percentage,
                    z_percentage,
                    x,
                    y,
                } => TransformRead::Scale3d((
                    number_value(x_percentage, x),
                    number_value(y_percentage, y),
                    ScaleZRead(ScaleZSource::Stored {
                        percentage: z_percentage,
                        value: TransformFieldRead(FieldSource::Stored {
                            context: self,
                            extra: header.extra,
                        }),
                    }),
                )),
                TransformData::Rotate3d { unit, x, y } => TransformRead::Rotate3d((
                    x,
                    y,
                    RotationTailRead(RotationTailSource::Stored {
                        unit,
                        value: TransformFieldRead(FieldSource::Stored {
                            context: self,
                            extra: header.extra,
                        }),
                    }),
                )),
                TransformData::Skew {
                    x_unit,
                    y_unit,
                    x,
                    y,
                } => TransformRead::Skew((x_unit.angle(x), y_unit.angle(y))),
            }
        }
    }
    impl<'id> From<Transform<'id>> for TransformRead<'_, '_, 'id> {
        fn from(value: Transform<'id>) -> Self {
            match value {
                Transform::Translate(value) => Self::Translate(value),
                Transform::TranslateX(value) => Self::TranslateX(value),
                Transform::TranslateY(value) => Self::TranslateY(value),
                Transform::TranslateZ(value) => Self::TranslateZ(value),
                Transform::Scale(value) => Self::Scale(value),
                Transform::ScaleX(value) => Self::ScaleX(value),
                Transform::ScaleY(value) => Self::ScaleY(value),
                Transform::ScaleZ(value) => Self::ScaleZ(value),
                Transform::Rotate(value) => Self::Rotate(value),
                Transform::RotateX(value) => Self::RotateX(value),
                Transform::RotateY(value) => Self::RotateY(value),
                Transform::RotateZ(value) => Self::RotateZ(value),
                Transform::Skew(value) => Self::Skew(value),
                Transform::SkewX(value) => Self::SkewX(value),
                Transform::SkewY(value) => Self::SkewY(value),
                Transform::Perspective(value) => Self::Perspective(value),
                Transform::Matrix(value) => Self::Matrix(value),
                Transform::Matrix3d(value) => Self::Matrix3d(value),
                Transform::Translate3d((x, y, z)) => {
                    Self::Translate3d((x, y, TransformFieldRead(FieldSource::Value(z))))
                }
                Transform::Scale3d((x, y, z)) => {
                    Self::Scale3d((x, y, ScaleZRead(ScaleZSource::Value(z))))
                }
                Transform::Rotate3d((x, y, z, angle)) => {
                    Self::Rotate3d((x, y, RotationTailRead(RotationTailSource::Value(z, angle))))
                }
            }
        }
    }
}

// SAFETY: this kind stores TransformHeader. The three overflow-bearing variants
// publish their typed slot before the header; all other variants never read it.
unsafe impl<'ast> AstNodeStorage<'ast> for Transform<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000f_0006);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: TransformHeader<'ast> = unsafe { payload.read_value() };
        match header.data {
            TransformData::Translate(value) => Self::Translate(value),
            TransformData::TranslateX(value) => Self::TranslateX(value),
            TransformData::TranslateY(value) => Self::TranslateY(value),
            TransformData::TranslateZ(value) => Self::TranslateZ(value),
            TransformData::ScaleX(value) => Self::ScaleX(value),
            TransformData::ScaleY(value) => Self::ScaleY(value),
            TransformData::ScaleZ(value) => Self::ScaleZ(value),
            TransformData::Rotate(value) => Self::Rotate(value),
            TransformData::RotateX(value) => Self::RotateX(value),
            TransformData::RotateY(value) => Self::RotateY(value),
            TransformData::RotateZ(value) => Self::RotateZ(value),
            TransformData::SkewX(value) => Self::SkewX(value),
            TransformData::SkewY(value) => Self::SkewY(value),
            TransformData::Perspective(value) => Self::Perspective(value),
            TransformData::Matrix(value) => Self::Matrix(value),
            TransformData::Matrix3d(value) => Self::Matrix3d(value),
            TransformData::Translate3d { x, y } => Self::Translate3d((x, y, unsafe {
                context.extra_slot(header.extra as usize).read_value()
            })),
            TransformData::Scale {
                x_percentage,
                y_percentage,
                x,
                y,
            } => Self::Scale((number_value(x_percentage, x), number_value(y_percentage, y))),
            TransformData::Scale3d {
                x_percentage,
                y_percentage,
                z_percentage,
                x,
                y,
            } => {
                let z = unsafe { context.extra_slot(header.extra as usize).read_value() };
                Self::Scale3d((
                    number_value(x_percentage, x),
                    number_value(y_percentage, y),
                    number_value(z_percentage, z),
                ))
            }
            TransformData::Rotate3d { unit, x, y } => {
                let [z, value]: [f32; 2] =
                    unsafe { context.extra_slot(header.extra as usize).read_value() };
                Self::Rotate3d((x, y, z, unit.angle(value)))
            }
            TransformData::Skew {
                x_unit,
                y_unit,
                x,
                y,
            } => Self::Skew((x_unit.angle(x), y_unit.angle(y))),
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_transform(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: TransformHeader<'ast> = unsafe { current.read_value() };
        store_transform(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Transform<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Translate((x, y)) => {
                Self::Translate((context.clone_encoded_node(x), context.clone_encoded_node(y)))
            }
            Self::TranslateX(value) => Self::TranslateX(context.clone_encoded_node(value)),
            Self::TranslateY(value) => Self::TranslateY(context.clone_encoded_node(value)),
            Self::TranslateZ(value) => Self::TranslateZ(context.clone_encoded_node(value)),
            Self::Translate3d((x, y, z)) => Self::Translate3d((
                context.clone_encoded_node(x),
                context.clone_encoded_node(y),
                context.clone_encoded_node(z),
            )),
            Self::Scale(value) => Self::Scale(value),
            Self::ScaleX(value) => Self::ScaleX(value),
            Self::ScaleY(value) => Self::ScaleY(value),
            Self::ScaleZ(value) => Self::ScaleZ(value),
            Self::Scale3d(value) => Self::Scale3d(value),
            Self::Rotate(value) => Self::Rotate(value),
            Self::RotateX(value) => Self::RotateX(value),
            Self::RotateY(value) => Self::RotateY(value),
            Self::RotateZ(value) => Self::RotateZ(value),
            Self::Rotate3d(value) => Self::Rotate3d(value),
            Self::Skew(value) => Self::Skew(value),
            Self::SkewX(value) => Self::SkewX(value),
            Self::SkewY(value) => Self::SkewY(value),
            Self::Perspective(value) => Self::Perspective(context.clone_encoded_node(value)),
            Self::Matrix(value) => Self::Matrix(context.clone_encoded_node(value)),
            Self::Matrix3d(value) => Self::Matrix3d(context.clone_encoded_node(value)),
        }
    }
}

fn store_transform<'ast>(
    value: Transform<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut slot = ExtraData::default();
    let data = match value {
        Transform::Translate(value) => TransformData::Translate(value),
        Transform::TranslateX(value) => TransformData::TranslateX(value),
        Transform::TranslateY(value) => TransformData::TranslateY(value),
        Transform::TranslateZ(value) => TransformData::TranslateZ(value),
        Transform::ScaleX(value) => TransformData::ScaleX(value),
        Transform::ScaleY(value) => TransformData::ScaleY(value),
        Transform::ScaleZ(value) => TransformData::ScaleZ(value),
        Transform::Rotate(value) => TransformData::Rotate(value),
        Transform::RotateX(value) => TransformData::RotateX(value),
        Transform::RotateY(value) => TransformData::RotateY(value),
        Transform::RotateZ(value) => TransformData::RotateZ(value),
        Transform::SkewX(value) => TransformData::SkewX(value),
        Transform::SkewY(value) => TransformData::SkewY(value),
        Transform::Perspective(value) => TransformData::Perspective(value),
        Transform::Matrix(value) => TransformData::Matrix(value),
        Transform::Matrix3d(value) => TransformData::Matrix3d(value),
        Transform::Translate3d((x, y, z)) => {
            slot = ExtraData::from_value(z);
            TransformData::Translate3d { x, y }
        }
        Transform::Scale((x, y)) => {
            let (x_percentage, x) = number_parts(x);
            let (y_percentage, y) = number_parts(y);
            TransformData::Scale {
                x_percentage,
                y_percentage,
                x,
                y,
            }
        }
        Transform::Scale3d((x, y, z)) => {
            let (x_percentage, x) = number_parts(x);
            let (y_percentage, y) = number_parts(y);
            let (z_percentage, z) = number_parts(z);
            slot = ExtraData::from_value(z);
            TransformData::Scale3d {
                x_percentage,
                y_percentage,
                z_percentage,
                x,
                y,
            }
        }
        Transform::Rotate3d((x, y, z, angle)) => {
            let (unit, value) = TransformAngleUnit::split(angle);
            slot = ExtraData::from_value([z, value]);
            TransformData::Rotate3d { unit, x, y }
        }
        Transform::Skew((x, y)) => {
            let (x_unit, x) = TransformAngleUnit::split(x);
            let (y_unit, y) = TransformAngleUnit::split(y);
            TransformData::Skew {
                x_unit,
                y_unit,
                x,
                y,
            }
        }
    };
    let extra = match existing {
        Some(index) => {
            context.set_extra_slot(index, slot);
            index
        }
        None => context.alloc_extra_slots([slot]),
    };
    NodePayload::from_value(TransformHeader {
        data,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformStyle {
    Flat,
    Preserve3d,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TransformBox {
    ContentBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ViewBox,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Perspective<'a> {
    None,
    Length(NodeId<'a, Length<'a>>),
}

impl_inline_node!(Perspective<'ast>, 0x000f0003);

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Translate<'a> {
    None,
    Xyz {
        x: NodeId<'a, LengthPercentage<'a>>,
        y: NodeId<'a, LengthPercentage<'a>>,
        z: NodeId<'a, Length<'a>>,
    },
}

impl_inline_node!(Translate<'ast>, 0x000f0004);

#[derive(Debug, PartialEq, Visit)]
pub enum Scale {
    None,
    Xyz {
        x: NumberOrPercentage,
        y: NumberOrPercentage,
        z: NumberOrPercentage,
    },
}

// Flatten three nested number/percentage values to fit the full scale in 16 bytes.
#[repr(u8)]
#[derive(Clone, Copy)]
enum ScaleSlot {
    None,
    Xyz {
        x_percentage: bool,
        y_percentage: bool,
        z_percentage: bool,
        x: f32,
        y: f32,
        z: f32,
    },
}

fn number_parts(value: NumberOrPercentage) -> (bool, f32) {
    match value {
        NumberOrPercentage::Number(value) => (false, value),
        NumberOrPercentage::Percentage(value) => (true, value),
    }
}
fn number_value(percentage: bool, value: f32) -> NumberOrPercentage {
    if percentage {
        NumberOrPercentage::Percentage(value)
    } else {
        NumberOrPercentage::Number(value)
    }
}

// SAFETY: this kind always stores ScaleSlot, preserving all three type flags.
unsafe impl AstNodeStorage<'_> for Scale {
    const KIND: NodeKind = NodeKind::new(0x000f_0005);
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        match unsafe { payload.read_value::<ScaleSlot>() } {
            ScaleSlot::None => Self::None,
            ScaleSlot::Xyz {
                x_percentage,
                y_percentage,
                z_percentage,
                x,
                y,
                z,
            } => Self::Xyz {
                x: number_value(x_percentage, x),
                y: number_value(y_percentage, y),
                z: number_value(z_percentage, z),
            },
        }
    }
    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        let value = match self {
            Self::None => ScaleSlot::None,
            Self::Xyz { x, y, z } => {
                let (x_percentage, x) = number_parts(x);
                let (y_percentage, y) = number_parts(y);
                let (z_percentage, z) = number_parts(z);
                ScaleSlot::Xyz {
                    x_percentage,
                    y_percentage,
                    z_percentage,
                    x,
                    y,
                    z,
                }
            }
        };
        NodePayload::from_value(value)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        self.encode_new(context)
    }
}

impl AstNodeClone<'_> for Scale {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        Angle, AstContext, DUMMY_SP, DimensionPercentage, Length, LengthUnit, LengthValue,
        NumberOrPercentage, Perspective, Scale, Transform, Translate,
    };

    #[test]
    fn native_transform_switches_all_variants_with_one_slot() {
        use crate::{Matrix3DForFloat, MatrixForFloat};
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let x = ast.alloc_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let y = ast.alloc_node(DimensionPercentage::Percentage(20.0), DUMMY_SP);
        let z = ast.alloc_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 30.0,
            }),
            DUMMY_SP,
        );
        let matrix = ast.alloc_node(
            MatrixForFloat {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0,
            },
            DUMMY_SP,
        );
        let matrix3d = ast.alloc_node(
            Matrix3DForFloat {
                m11: 1.0,
                m12: 2.0,
                m13: 3.0,
                m14: 4.0,
                m21: 5.0,
                m22: 6.0,
                m23: 7.0,
                m24: 8.0,
                m31: 9.0,
                m32: 10.0,
                m33: 11.0,
                m34: 12.0,
                m41: 13.0,
                m42: 14.0,
                m43: 15.0,
                m44: 16.0,
            },
            DUMMY_SP,
        );
        let n = NumberOrPercentage::Number(0.5);
        let p = NumberOrPercentage::Percentage(50.0);
        let before = ast.encoded_extra_len();
        let node = ast.alloc_node(Transform::TranslateX(x), DUMMY_SP);
        assert_eq!(ast.encoded_extra_len(), before + 1);
        let checkpoint = ast.node_checkpoint();
        for expected in [
            Transform::Translate((x, y)),
            Transform::TranslateX(x),
            Transform::TranslateY(y),
            Transform::TranslateZ(z),
            Transform::Translate3d((x, y, z)),
            Transform::Scale((n, p)),
            Transform::ScaleX(n),
            Transform::ScaleY(p),
            Transform::ScaleZ(n),
            Transform::Scale3d((n, p, n)),
            Transform::Rotate(Angle::Deg(30.0)),
            Transform::RotateX(Angle::Rad(1.0)),
            Transform::RotateY(Angle::Grad(45.0)),
            Transform::RotateZ(Angle::Turn(0.5)),
            Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Turn(0.25))),
            Transform::Skew((Angle::Deg(10.0), Angle::Rad(2.0))),
            Transform::SkewX(Angle::Grad(25.0)),
            Transform::SkewY(Angle::Turn(0.5)),
            Transform::Perspective(z),
            Transform::Matrix(matrix),
            Transform::Matrix3d(matrix3d),
        ] {
            ast.mutate_node(node, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(node), expected);
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
        for bits in [
            0,
            0x8000_0000,
            1,
            0x7f7f_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_1234,
        ] {
            for position in 0..4 {
                let mut expected = [1.25_f32, 2.5, 3.75, 0.625].map(f32::to_bits);
                expected[position] = bits;
                let [x, y, z, number] = expected.map(f32::from_bits);
                for angle in [
                    Angle::Deg(number),
                    Angle::Rad(number),
                    Angle::Grad(number),
                    Angle::Turn(number),
                ] {
                    let check_angle = |actual: Angle| {
                        assert_eq!(
                            std::mem::discriminant(&actual),
                            std::mem::discriminant(&angle)
                        );
                        let (Angle::Deg(value)
                        | Angle::Rad(value)
                        | Angle::Grad(value)
                        | Angle::Turn(value)) = actual;
                        assert_eq!(value.to_bits(), expected[3]);
                    };
                    ast.mutate_node(node, |value, _| {
                        *value = Transform::Rotate3d((x, y, z, angle))
                    });
                    let Transform::Rotate3d((a, b, c, actual)) = ast.resolve_node(node) else {
                        panic!("expected rotate3d");
                    };
                    assert_eq!(
                        [a, b, c].map(f32::to_bits),
                        [expected[0], expected[1], expected[2]]
                    );
                    check_angle(actual);
                    for view in [
                        ast.transform(node),
                        super::TransformRead::from(Transform::Rotate3d((x, y, z, angle))),
                    ] {
                        let super::TransformRead::Rotate3d((a, b, tail)) = view else {
                            panic!("expected rotate3d view");
                        };
                        let (c, actual) = tail.get();
                        assert_eq!(
                            [a, b, c].map(f32::to_bits),
                            [expected[0], expected[1], expected[2]]
                        );
                        check_angle(actual);
                    }
                }
            }
            for position in 0..3 {
                let mut expected = [1.25_f32, 2.5, 3.75].map(f32::to_bits);
                expected[position] = bits;
                for flags in 0..8 {
                    let values: [NumberOrPercentage; 3] = std::array::from_fn(|index| {
                        let value = f32::from_bits(expected[index]);
                        if flags & (1 << index) != 0 {
                            NumberOrPercentage::Percentage(value)
                        } else {
                            NumberOrPercentage::Number(value)
                        }
                    });
                    let [x, y, z] = values;
                    let check = |values: [NumberOrPercentage; 3]| {
                        for (index, actual) in values.into_iter().enumerate() {
                            let (percentage, value) = match actual {
                                NumberOrPercentage::Number(value) => (false, value),
                                NumberOrPercentage::Percentage(value) => (true, value),
                            };
                            assert_eq!(percentage, flags & (1 << index) != 0);
                            assert_eq!(value.to_bits(), expected[index]);
                        }
                    };
                    ast.mutate_node(node, |value, _| *value = Transform::Scale3d((x, y, z)));
                    let Transform::Scale3d((a, b, c)) = ast.resolve_node(node) else {
                        panic!("expected scale3d");
                    };
                    check([a, b, c]);
                    for view in [
                        ast.transform(node),
                        super::TransformRead::from(Transform::Scale3d((x, y, z))),
                    ] {
                        let super::TransformRead::Scale3d((a, b, tail)) = view else {
                            panic!("expected scale3d view");
                        };
                        check([a, b, tail.get()]);
                    }
                }
            }
            assert_eq!(ast.node_checkpoint(), checkpoint);
        }
    }

    #[test]
    fn native_scale_preserves_all_type_flags_and_float_bits() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let scale = ast.alloc_node(Scale::None, DUMMY_SP);
        let checkpoint = ast.node_checkpoint();
        for flags in 0..8 {
            let x = super::number_value(flags & 1 != 0, -0.0);
            let y = super::number_value(flags & 2 != 0, f32::INFINITY);
            let z = super::number_value(flags & 4 != 0, f32::from_bits(0x7fc0_1234));
            ast.mutate_node(scale, |value, _| *value = Scale::Xyz { x, y, z });
            let Scale::Xyz { x, y, z } = ast.resolve_node(scale) else {
                panic!("expected scale")
            };
            for (actual, flag, bits) in [
                (x, flags & 1 != 0, 0x8000_0000),
                (y, flags & 2 != 0, 0x7f80_0000),
                (z, flags & 4 != 0, 0x7fc0_1234),
            ] {
                let (percentage, value) = super::number_parts(actual);
                assert_eq!(percentage, flag);
                assert_eq!(value.to_bits(), bits);
            }
            ast.mutate_node(scale, |value, _| *value = Scale::None);
            assert_eq!(ast.resolve_node(scale), Scale::None);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }

    #[test]
    fn transform_codec_reuses_overflow_and_deep_clones_child_nodes() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let x = context.alloc_encoded_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let y = context.alloc_encoded_node(DimensionPercentage::Percentage(20.0), DUMMY_SP);
        let z = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 30.0,
            }),
            DUMMY_SP,
        );
        let before = context.encoded_extra_len();
        let transform = context.alloc_encoded_node(Transform::Translate3d((x, y, z)), DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before + 1);
        assert_eq!(
            context.encoded_node(transform),
            Transform::Translate3d((x, y, z))
        );

        let cloned = context.clone_encoded_node(transform);
        let Transform::Translate3d((cloned_x, cloned_y, cloned_z)) = context.encoded_node(cloned)
        else {
            panic!("expected translate3d")
        };
        assert_ne!((cloned_x, cloned_y, cloned_z), (x, y, z));

        context.mutate_encoded_node(transform, |value, _| {
            *value = Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Turn(0.25)));
        });
        assert_eq!(context.encoded_extra_len(), before + 2);
        assert_eq!(
            context.encoded_node(transform),
            Transform::Rotate3d((1.0, 2.0, 3.0, Angle::Turn(0.25)))
        );
    }

    #[test]
    fn transform_property_node_codecs_preserve_variants_and_child_ids() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let x = context.alloc_encoded_node(DimensionPercentage::Percentage(10.0), DUMMY_SP);
        let y = context.alloc_encoded_node(DimensionPercentage::Percentage(20.0), DUMMY_SP);
        let z = context.alloc_encoded_node(
            Length::Value(LengthValue {
                unit: LengthUnit::Px,
                value: 30.0,
            }),
            DUMMY_SP,
        );
        let translate = context.alloc_encoded_node(Translate::Xyz { x, y, z }, DUMMY_SP);
        assert_eq!(context.encoded_node(translate), Translate::Xyz { x, y, z });

        let perspective = context.alloc_encoded_node(Perspective::Length(z), DUMMY_SP);
        assert_eq!(context.encoded_node(perspective), Perspective::Length(z));

        let scale = context.alloc_encoded_node(
            Scale::Xyz {
                x: NumberOrPercentage::Number(1.0),
                y: NumberOrPercentage::Percentage(50.0),
                z: NumberOrPercentage::Number(-0.0),
            },
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(scale),
            Scale::Xyz {
                x: NumberOrPercentage::Number(1.0),
                y: NumberOrPercentage::Percentage(50.0),
                z: NumberOrPercentage::Number(-0.0),
            }
        );
    }
}

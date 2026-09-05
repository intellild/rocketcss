use crate::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub struct MatrixForFloat {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl AstNodeStorage<'_> for MatrixForFloat {
    const KIND: NodeKind = NodeKind::new(0x000f_0001);

    fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        let extra = payload.extra_start();
        Self {
            a: read_inline_float(&bytes, 0),
            b: read_inline_float(&bytes, 4),
            c: read_inline_float(&bytes, 8),
            d: read_extra_float(context, extra),
            e: read_extra_float(context, extra + 1),
            f: read_extra_float(context, extra + 2),
        }
    }

    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        encode_matrix(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        encode_matrix(self, Some(current.extra_start()), context)
    }
}

impl AstNodeClone<'_> for MatrixForFloat {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_matrix(
    value: MatrixForFloat,
    existing_extra: Option<usize>,
    context: &mut AstContext<'_>,
) -> NodePayload {
    let mut inline = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_float(&mut inline, 0, value.a);
    write_float(&mut inline, 4, value.b);
    write_float(&mut inline, 8, value.c);
    let extra_values = [value.d, value.e, value.f].map(float_extra);
    let extra = write_fixed_extra(existing_extra, extra_values, context);
    NodePayload::with_extra(&inline, extra)
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

impl AstNodeStorage<'_> for Matrix3DForFloat {
    const KIND: NodeKind = NodeKind::new(0x000f_0002);

    fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        let extra = payload.extra_start();
        let mut values = [0.0; 16];
        values[0] = read_inline_float(&bytes, 0);
        values[1] = read_inline_float(&bytes, 4);
        values[2] = read_inline_float(&bytes, 8);
        for (offset, value) in values[3..].iter_mut().enumerate() {
            *value = read_extra_float(context, extra + offset);
        }
        let [
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
            m41,
            m42,
            m43,
            m44,
        ] = values;
        Self {
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
            m41,
            m42,
            m43,
            m44,
        }
    }

    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        encode_matrix_3d(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        encode_matrix_3d(self, Some(current.extra_start()), context)
    }
}

impl AstNodeClone<'_> for Matrix3DForFloat {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_matrix_3d(
    value: Matrix3DForFloat,
    existing_extra: Option<usize>,
    context: &mut AstContext<'_>,
) -> NodePayload {
    let values = [
        value.m11, value.m12, value.m13, value.m14, value.m21, value.m22, value.m23, value.m24,
        value.m31, value.m32, value.m33, value.m34, value.m41, value.m42, value.m43, value.m44,
    ];
    let mut inline = [0; NodePayload::PARTIAL_INLINE_BYTES];
    for (offset, value) in values[..3].iter().copied().enumerate() {
        write_float(&mut inline, offset * 4, value);
    }
    let extra_values = [
        values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10],
        values[11], values[12], values[13], values[14], values[15],
    ]
    .map(float_extra);
    let extra = write_fixed_extra(existing_extra, extra_values, context);
    NodePayload::with_extra(&inline, extra)
}

#[derive(Debug, PartialEq, Visit)]
pub struct Rotate {
    pub angle: Angle,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn write_fixed_extra<const N: usize>(
    existing_extra: Option<usize>,
    values: [ExtraData; N],
    context: &mut AstContext<'_>,
) -> usize {
    match existing_extra {
        Some(extra) => {
            for (offset, value) in values.into_iter().enumerate() {
                context.set_extra_slot(extra + offset, value);
            }
            extra
        }
        None => context.alloc_extra_slots(values),
    }
}

fn float_extra(value: f32) -> ExtraData {
    ExtraData::from_u64(value.to_bits() as u64)
}

fn read_extra_float(context: &AstContext<'_>, index: usize) -> f32 {
    f32::from_bits(context.extra_slot(index).as_u64() as u32)
}

fn write_float(bytes: &mut [u8], offset: usize, value: f32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_bits().to_le_bytes());
}

fn read_inline_float(bytes: &[u8], offset: usize) -> f32 {
    f32::from_bits(u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact transform field is four bytes"),
    ))
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{AstContext, DUMMY_SP, Matrix3DForFloat, MatrixForFloat};

    #[test]
    fn matrix_codecs_reuse_their_fixed_overflow_slots() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let before = context.encoded_extra_len();
        let matrix = context.alloc_encoded_node(
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
        assert_eq!(context.encoded_extra_len(), before + 3);
        context.mutate_encoded_node(matrix, |value, _| value.f = 60.0);
        assert_eq!(context.encoded_extra_len(), before + 3);
        assert_eq!(context.encoded_node(matrix).f, 60.0);

        let before_3d = context.encoded_extra_len();
        let matrix_3d_id = context.alloc_encoded_node(matrix_3d(), DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before_3d + 13);
        assert_eq!(context.encoded_node(matrix_3d_id), matrix_3d());
    }

    fn matrix_3d() -> Matrix3DForFloat {
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
        }
    }
}

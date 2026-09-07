use crate::*;

use crate::{AstNodeClone, AstNodeStorage, ExtraData, NodeKind, NodePayload};

#[derive(Clone, Copy)]
struct MatrixHeader {
    values: [f32; 3],
    extra: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct MatrixForFloat {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

// SAFETY: this kind stores MatrixHeader and a native tail array in 2 opaque slots.
unsafe impl AstNodeStorage<'_> for MatrixForFloat {
    const KIND: NodeKind = NodeKind::new(0x000f_0001);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let header: MatrixHeader = unsafe { payload.read_value() };
        let [a, b, c] = header.values;
        let [d, e, f]: [f32; 3] = unsafe {
            ExtraData::read_value_array::<_, 2>(std::array::from_fn(|i| {
                context.extra_slot(header.extra as usize + i)
            }))
        };
        Self { a, b, c, d, e, f }
    }
    fn encode_new(self, context: &mut AstContext<'_>) -> NodePayload {
        self.store(None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let header: MatrixHeader = unsafe { current.read_value() };
        self.store(Some(header.extra as usize), context)
    }
}
impl MatrixForFloat {
    fn store(self, existing: Option<usize>, context: &mut AstContext<'_>) -> NodePayload {
        let fields = ExtraData::from_value_array::<_, 2>([self.d, self.e, self.f]);
        let extra = write_fixed_extra(existing, fields, context);
        NodePayload::from_value(MatrixHeader {
            values: [self.a, self.b, self.c],
            extra: u32::try_from(extra).expect("extra index exceeds u32"),
        })
    }
}

impl AstNodeClone<'_> for MatrixForFloat {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
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

// SAFETY: this kind stores MatrixHeader and a native tail array in 7 opaque slots.
unsafe impl AstNodeStorage<'_> for Matrix3DForFloat {
    const KIND: NodeKind = NodeKind::new(0x000f_0002);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'_>) -> Self {
        let header: MatrixHeader = unsafe { payload.read_value() };
        let [m11, m12, m13] = header.values;
        let [
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
        ]: [f32; 13] = unsafe {
            ExtraData::read_value_array::<_, 7>(std::array::from_fn(|i| {
                context.extra_slot(header.extra as usize + i)
            }))
        };
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
        self.store(None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'_>,
    ) -> NodePayload {
        let header: MatrixHeader = unsafe { current.read_value() };
        self.store(Some(header.extra as usize), context)
    }
}
impl Matrix3DForFloat {
    fn store(self, existing: Option<usize>, context: &mut AstContext<'_>) -> NodePayload {
        let fields = ExtraData::from_value_array::<_, 7>([
            self.m14, self.m21, self.m22, self.m23, self.m24, self.m31, self.m32, self.m33,
            self.m34, self.m41, self.m42, self.m43, self.m44,
        ]);
        let extra = write_fixed_extra(existing, fields, context);
        NodePayload::from_value(MatrixHeader {
            values: [self.m11, self.m12, self.m13],
            extra: u32::try_from(extra).expect("extra index exceeds u32"),
        })
    }
}

impl AstNodeClone<'_> for Matrix3DForFloat {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

impl AstContext<'_> {
    /// Returns matrix components in serialization order without rebuilding the node.
    #[inline]
    pub fn matrix_components(&self, id: NodeId<'_, MatrixForFloat>) -> ([f32; 3], [f32; 3]) {
        // SAFETY: node_payload validates the matrix kind, which stores this header.
        let header: MatrixHeader = unsafe { self.node_payload(id).read_value() };
        // SAFETY: this kind writes a native [f32; 3] across its two overflow slots.
        let tail = unsafe {
            ExtraData::read_value_array::<_, 2>(std::array::from_fn(|i| {
                self.extra_slot(header.extra as usize + i)
            }))
        };
        (header.values, tail)
    }

    /// Returns 3D matrix components in serialization order without rebuilding the node.
    #[inline]
    pub fn matrix_3d_components(&self, id: NodeId<'_, Matrix3DForFloat>) -> ([f32; 3], [f32; 13]) {
        // SAFETY: node_payload validates the 3D matrix kind, which stores this header.
        let header: MatrixHeader = unsafe { self.node_payload(id).read_value() };
        // SAFETY: this kind writes a native [f32; 13] across its seven overflow slots.
        let tail = unsafe {
            ExtraData::read_value_array::<_, 7>(std::array::from_fn(|i| {
                self.extra_slot(header.extra as usize + i)
            }))
        };
        (header.values, tail)
    }
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
        assert_eq!(context.encoded_extra_len(), before + 2);
        context.mutate_encoded_node(matrix, |value, _| value.f = 60.0);
        assert_eq!(context.encoded_extra_len(), before + 2);
        assert_eq!(context.encoded_node(matrix).f, 60.0);

        let before_3d = context.encoded_extra_len();
        let matrix_3d_id = context.alloc_encoded_node(matrix_3d(), DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), before_3d + 7);
        assert_eq!(context.encoded_node(matrix_3d_id), matrix_3d());
        let checkpoint = context.node_checkpoint();
        for bits in [0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
            let number = f32::from_bits(bits);
            context.mutate_encoded_node(matrix, |value, _| {
                value.a = number;
                value.d = number;
                value.f = number;
            });
            let value = context.encoded_node(matrix);
            let (head, tail) = context.matrix_components(matrix);
            assert_eq!(
                head.map(f32::to_bits),
                [bits, 2.0_f32.to_bits(), 3.0_f32.to_bits()]
            );
            assert_eq!(tail.map(f32::to_bits), [bits, 5.0_f32.to_bits(), bits]);
            assert_eq!(
                [value.a.to_bits(), value.d.to_bits(), value.f.to_bits()],
                [bits; 3]
            );
            assert_eq!((value.b, value.c, value.e), (2.0, 3.0, 5.0));
            context.mutate_encoded_node(matrix_3d_id, |value, _| {
                value.m11 = number;
                value.m14 = number;
                value.m44 = number;
            });
            let value = context.encoded_node(matrix_3d_id);
            let (head, tail) = context.matrix_3d_components(matrix_3d_id);
            assert_eq!(
                head.map(f32::to_bits),
                [bits, 2.0_f32.to_bits(), 3.0_f32.to_bits()]
            );
            assert_eq!(tail[0].to_bits(), bits);
            assert_eq!(tail[12].to_bits(), bits);
            assert_eq!(
                &tail[1..12],
                &[5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]
            );
            assert_eq!(
                [
                    value.m11.to_bits(),
                    value.m14.to_bits(),
                    value.m44.to_bits()
                ],
                [bits; 3]
            );
            assert_eq!(
                (value.m12, value.m13, value.m21, value.m43),
                (2.0, 3.0, 5.0, 15.0)
            );
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
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

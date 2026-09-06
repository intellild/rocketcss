use crate::*;
use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};
#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

// Rust's native enum layout fits the full selector in one extra slot.
impl_inline_extra!(KeyframeSelector);

impl ExtraDataClone<'_> for KeyframeSelector {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum KeyframesName<'a> {
    Ident(AstStr<'a>),
    Custom(AstStr<'a>),
}

// SAFETY: this KIND always stores and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for KeyframesName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0015_0001);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Custom(a), Self::Custom(b)) | (Self::Ident(a), Self::Ident(b)) => {
                context.str(*a) == context.str(*b)
            }
            _ => self == other,
        }
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
}

impl<'ast> AstNodeClone<'ast> for KeyframesName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}

#[cfg(test)]
mod native_tests {
    use super::*;

    #[test]
    fn keyframe_slots_preserve_keywords_and_timeline_percentage_bits() {
        assert_eq!(std::mem::size_of::<KeyframeSelector>(), 8);
        let allocator = rocketcss_common::Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let values =
            ast.alloc_encoded_vec([KeyframeSelector::From, KeyframeSelector::To].into_iter());
        assert_eq!(ast.encoded_vec_get(values, 0), Some(KeyframeSelector::From));
        assert_eq!(ast.encoded_vec_get(values, 1), Some(KeyframeSelector::To));
        let checkpoint = ast.node_checkpoint();
        for bits in [0, 0x8000_0000, 1, 0x7f80_0000, 0x7fc0_0123] {
            ast.encoded_vec_set(
                values,
                0,
                KeyframeSelector::Percentage(f32::from_bits(bits)),
            );
            let Some(KeyframeSelector::Percentage(value)) = ast.encoded_vec_get(values, 0) else {
                panic!()
            };
            assert_eq!(value.to_bits(), bits);
            for name in [
                TimelineRangeName::Cover,
                TimelineRangeName::Contain,
                TimelineRangeName::Entry,
                TimelineRangeName::Exit,
                TimelineRangeName::EntryCrossing,
                TimelineRangeName::ExitCrossing,
            ] {
                ast.encoded_vec_set(
                    values,
                    1,
                    KeyframeSelector::TimelineRangePercentage(TimelineRangePercentage {
                        name,
                        percentage: f32::from_bits(bits),
                    }),
                );
                let Some(KeyframeSelector::TimelineRangePercentage(value)) =
                    ast.encoded_vec_get(values, 1)
                else {
                    panic!()
                };
                assert_eq!(value.name, name);
                assert_eq!(value.percentage.to_bits(), bits);
            }
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
    }
}

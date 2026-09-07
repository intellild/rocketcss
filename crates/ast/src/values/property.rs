use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Eq, Visit)]
pub enum CSSWideKeyword {
    Initial,
    Inherit,
    Unset,
    Revert,
    RevertLayer,
}

/// A typed property value or a CSS-wide keyword.
#[derive(Debug, PartialEq, Visit)]
pub enum CSSWideOr<T> {
    Value(T),
    CSSWide(CSSWideKeyword),
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum CustomPropertyName<'a> {
    Custom(AstStr<'a>),
    Unknown(AstStr<'a>),
}

unsafe impl<'ast> AstNodeStorage<'ast> for CustomPropertyName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0014_0001);
    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        unsafe { payload.read_value() }
    }
    #[inline]
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        NodePayload::from_value(self)
    }
    #[inline]
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        NodePayload::from_value(self)
    }
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Custom(left), Self::Custom(right))
            | (Self::Unknown(left), Self::Unknown(right)) => {
                left == right || context.str(*left) == context.str(*right)
            }
            _ => false,
        }
    }
}

impl<'ast> AstNodeClone<'ast> for CustomPropertyName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ViewTransitionName<'a> {
    None,
    Auto,
    Custom(AstStr<'a>),
}

// SAFETY: this KIND always stores and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for ViewTransitionName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0002);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Custom(a), Self::Custom(b)) => context.str(*a) == context.str(*b),
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

impl<'ast> AstNodeClone<'ast> for ViewTransitionName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum NoneOrCustomIdentList<'a> {
    None,
    Idents(Vec<'a, AstStr<'a>>),
}

// SAFETY: this KIND publishes native NoneOrCustomIdentList values.
unsafe impl<'ast> AstNodeStorage<'ast> for NoneOrCustomIdentList<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0001);
    #[inline]
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        // SAFETY: the typed context validated KIND.
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
}

impl<'ast> AstNodeClone<'ast> for NoneOrCustomIdentList<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Idents(values) => Self::Idents(context.clone_encoded_vec(values)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum ViewTransitionGroup<'a> {
    Normal,
    Contain,
    Nearest,
    Custom(AstStr<'a>),
}

// SAFETY: this KIND always stores and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for ViewTransitionGroup<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0016_0003);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::Custom(a), Self::Custom(b)) => context.str(*a) == context.str(*b),
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

impl<'ast> AstNodeClone<'ast> for ViewTransitionGroup<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn transition_names_compare_contents_and_reuse_slots() {
        assert_eq!(std::mem::size_of::<ViewTransitionName<'_>>(), 12);
        assert_eq!(std::mem::size_of::<ViewTransitionGroup<'_>>(), 12);
        assert_eq!(std::mem::size_of::<KeyframesName<'_>>(), 12);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("名称");
        let second = context.add_str("名称");
        assert_ne!(first, second);
        let name = context.alloc_encoded_node(ViewTransitionName::Custom(first), DUMMY_SP);
        let equal = context.alloc_encoded_node(ViewTransitionName::Custom(second), DUMMY_SP);
        assert!(context.nodes_eq(name, equal));
        let group = context.alloc_encoded_node(ViewTransitionGroup::Custom(first), DUMMY_SP);
        let equal_group = context.alloc_encoded_node(ViewTransitionGroup::Custom(second), DUMMY_SP);
        assert!(context.nodes_eq(group, equal_group));
        let ident = context.alloc_encoded_node(KeyframesName::Ident(first), DUMMY_SP);
        let string = context.alloc_encoded_node(KeyframesName::Custom(second), DUMMY_SP);
        assert!(!context.nodes_eq(ident, string));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for value in [
            ViewTransitionName::None,
            ViewTransitionName::Auto,
            ViewTransitionName::Custom(second),
        ] {
            context.mutate_encoded_node(name, |node, _| *node = value);
            assert_eq!(context.encoded_node(name), value);
        }
        for value in [
            ViewTransitionGroup::Normal,
            ViewTransitionGroup::Contain,
            ViewTransitionGroup::Nearest,
            ViewTransitionGroup::Custom(second),
        ] {
            context.mutate_encoded_node(group, |node, _| *node = value);
            assert_eq!(context.encoded_node(group), value);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }
}

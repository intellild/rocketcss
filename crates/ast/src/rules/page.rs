use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PageMarginBox {
    TopLeftCorner,
    TopLeft,
    TopCenter,
    TopRight,
    TopRightCorner,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeftCorner,
    BottomLeft,
    BottomCenter,
    BottomRight,
    BottomRightCorner,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

// SAFETY: each slot contains the same native PagePseudoClass value.
unsafe impl ExtraDataCompact<'_> for PagePseudoClass {
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
    }
}

impl ExtraDataClone<'_> for PagePseudoClass {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct PageSelector<'a> {
    pub name: Option<AstStr<'a>>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}

#[derive(Clone, Copy)]
struct PageSelectorSlot<'a> {
    // Reuse the checked optional-string representation, including Some(EMPTY).
    name: ExtraData,
    pseudo_classes: Vec<'a, PagePseudoClass>,
}

// SAFETY: KIND identifies PageSelectorSlot. Its name field is always written
// by Option<AstStr>::encode_extra and read through the matching decode_extra.
unsafe impl<'ast> AstNodeStorage<'ast> for PageSelector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0025_0001);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        self.pseudo_classes == other.pseudo_classes
            && self.name.map(|name| context.str(name)) == other.name.map(|name| context.str(name))
    }
    unsafe fn decode(payload: NodePayload, _context: &AstContext<'ast>) -> Self {
        let slot: PageSelectorSlot<'ast> = unsafe { payload.read_value() };
        Self {
            name: unsafe { Option::<AstStr<'ast>>::decode_extra(slot.name) },
            pseudo_classes: slot.pseudo_classes,
        }
    }
    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        self.into_payload()
    }
    unsafe fn encode_existing(
        self,
        _current: NodePayload,
        _context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.into_payload()
    }
}
impl PageSelector<'_> {
    fn into_payload(self) -> NodePayload {
        NodePayload::from_value(PageSelectorSlot {
            name: self.name.encode_extra(),
            pseudo_classes: self.pseudo_classes,
        })
    }
}

impl<'ast> AstNodeClone<'ast> for PageSelector<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            name: self.name,
            pseudo_classes: context.clone_encoded_vec(self.pseudo_classes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn page_names_preserve_optional_empty_without_overflow() {
        assert_eq!(std::mem::size_of::<PageSelectorSlot<'_>>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let names = [
            context.add_str("Invoice"),
            context.add_str("Invoice"),
            AstStr::EMPTY,
        ];
        let pseudo_classes = context.alloc_encoded_vec(
            [
                PagePseudoClass::Left,
                PagePseudoClass::Right,
                PagePseudoClass::First,
                PagePseudoClass::Last,
                PagePseudoClass::Blank,
            ]
            .into_iter(),
        );
        let before = context.encoded_extra_len();
        let node = context.alloc_encoded_node(
            PageSelector {
                name: None,
                pseudo_classes,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before);
        context.mutate_encoded_node(node, |value, _| value.name = Some(AstStr::EMPTY));
        assert_eq!(context.encoded_extra_len(), before);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for name in [
            None,
            Some(names[0]),
            None,
            Some(names[1]),
            Some(AstStr::EMPTY),
        ] {
            context.mutate_encoded_node(node, |value, _| value.name = name);
            assert_eq!(context.encoded_node(node).name, name);
            assert_eq!(context.encoded_node(node).pseudo_classes, pseudo_classes);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        context.mutate_encoded_node(node, |value, _| value.name = None);
        assert_eq!(context.encoded_node(node).name, None);
        assert_eq!(context.encoded_extra_len(), before);
        let first = context.alloc_encoded_node(
            PageSelector {
                name: Some(names[0]),
                pseudo_classes,
            },
            DUMMY_SP,
        );
        let second = context.alloc_encoded_node(
            PageSelector {
                name: Some(names[1]),
                pseudo_classes,
            },
            DUMMY_SP,
        );
        assert!(context.nodes_eq(first, second));
        assert_eq!(context.encoded_extra_len(), before);
        assert_eq!(
            context.encoded_vec_get(pseudo_classes, 4),
            Some(PagePseudoClass::Blank)
        );
    }
}

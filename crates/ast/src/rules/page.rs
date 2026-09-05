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

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum PagePseudoClass {
    Left,
    Right,
    First,
    Last,
    Blank,
}

impl ExtraDataCompact<'_> for PagePseudoClass {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::First => 2,
            Self::Last => 3,
            Self::Blank => 4,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::First,
            3 => Self::Last,
            4 => Self::Blank,
            _ => panic!("invalid encoded PagePseudoClass"),
        }
    }
}

impl ExtraDataClone<'_> for PagePseudoClass {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct PageSelector<'a> {
    pub name: Option<&'a str>,
    pub pseudo_classes: Vec<'a, PagePseudoClass>,
}

impl<'ast> AstNodeStorage<'ast> for PageSelector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0025_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let name = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let start = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        let end = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        Self {
            name: (name != u32::MAX).then(|| context.resolve_string(name as u64)),
            pseudo_classes: context.encoded_vec_range(start, end),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let name = self
            .name
            .map_or(u32::MAX, |name| context.store_string(name));
        bytes[0..4].copy_from_slice(&name.to_le_bytes());
        bytes[4..8].copy_from_slice(
            &u32::try_from(self.pseudo_classes.start_index())
                .expect("AST range start exceeds four bytes")
                .to_le_bytes(),
        );
        bytes[8..12].copy_from_slice(
            &u32::try_from(self.pseudo_classes.end_index())
                .expect("AST range end exceeds four bytes")
                .to_le_bytes(),
        );
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
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

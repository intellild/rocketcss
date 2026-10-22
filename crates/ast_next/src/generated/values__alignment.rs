use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlignContent {
    Normal,
    BaselinePosition(BaselinePosition),
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum BaselinePosition {
    First,
    Last,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ContentDistribution {
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum OverflowPosition {
    Safe,
    Unsafe,
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum ContentPosition {
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JustifyContent {
    Normal,
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlignSelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum SelfPosition {
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JustifySelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlignItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum JustifyItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
    Legacy(LegacyJustify),
}
use crate::*;
#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum LegacyJustify {
    Left,
    Right,
    Center,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GapValue<'ast>(pub(crate) NodeId<'ast, GapValueData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GapValueData<'ast> {
    Normal,
    LengthPercentage(LengthPercentage<'ast>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GapValueKind {
    Normal,
    LengthPercentage,
}
impl<'ast> NodeStorage<'ast> for GapValue<'ast> {
    const KIND: NodeKind = super::kind::KIND_GAP_VALUE;
    type Data = GapValueData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, GapValueData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, GapValueData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> GapValue<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> GapValueData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> GapValueKind {
        match ast.data(self) {
            GapValueData::Normal { .. } => GapValueKind::Normal,
            GapValueData::LengthPercentage { .. } => GapValueKind::LengthPercentage,
        }
    }
}
impl<'ast> GetSpan<'ast> for GapValue<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for GapValue<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_gap_value(&mut self, data: GapValueData<'ast>, span: Span) -> GapValue<'ast> {
        self.alloc(data, span)
    }
}

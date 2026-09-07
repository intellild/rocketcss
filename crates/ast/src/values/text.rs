use crate::*;

use crate::{AstNodeStorage, ExtraDataClone, NodeKind, NodePayload};

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WhiteSpace {
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpaces,
    PreLine,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WordBreak {
    Normal,
    KeepAll,
    BreakAll,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum LineBreak {
    Auto,
    Loose,
    Normal,
    Strict,
    Anywhere,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum Hyphens {
    None,
    Manual,
    Auto,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowWrap {
    Normal,
    Anywhere,
    BreakWord,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
    JustifyAll,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextAlignLast {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    InterCharacter,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Spacing<'a> {
    Normal,
    Length(NodeId<'a, Length<'a>>),
}

impl_inline_node!(Spacing<'ast>, 0x00100001);

impl<'ast> AstNodeClone<'ast> for Spacing<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            Self::Normal => Self::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextDecorationLine<'a> {
    ExclusiveTextDecorationLine(ExclusiveTextDecorationLine),
    Value(Vec<'a, OtherTextDecorationLine>),
}

impl_inline_node!(TextDecorationLine<'ast>, 0x00100002);

impl<'ast> AstNodeClone<'ast> for TextDecorationLine<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Value(value) => Self::Value(context.clone_encoded_vec(value)),
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

impl_inline_extra!(OtherTextDecorationLine);

impl ExtraDataClone<'_> for OtherTextDecorationLine {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextDecorationThickness<'a> {
    Auto,
    FromFont,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

impl_inline_node!(TextDecorationThickness<'ast>, 0x00100003);

impl<'ast> AstNodeClone<'ast> for TextDecorationThickness<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::LengthPercentage(value) => {
                Self::LengthPercentage(context.clone_encoded_node(value))
            }
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextEmphasisStyle<'a> {
    None,
    Keyword {
        fill: TextEmphasisFillMode,
        shape: Option<TextEmphasisShape>,
    },
    String(AstStr<'a>),
}

// SAFETY: this KIND always publishes and reads the same native Copy type.
unsafe impl<'ast> AstNodeStorage<'ast> for TextEmphasisStyle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0010_0004);
    fn eq_in_context(&self, other: &Self, context: &AstContext<'_>) -> bool {
        match (self, other) {
            (Self::String(a), Self::String(b)) => context.str(*a) == context.str(*b),
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

impl<'ast> AstNodeClone<'ast> for TextEmphasisStyle<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum TextEmphasisShape {
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Sesame,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisPositionHorizontal {
    Left,
    Right,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisPositionVertical {
    Over,
    Under,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextSizeAdjust {
    Auto,
    None,
    Percentage(f32),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct Content<'a> {
    pub value: Vec<'a, TokenOrValue<'a>>,
}

impl_inline_node!(Content<'ast>, 0x00100005);

impl<'ast> AstNodeClone<'ast> for Content<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            value: context.clone_encoded_vec(self.value),
        }
    }
}

#[cfg(test)]
mod storage_tests {
    use rocketcss_common::Allocator;

    use crate::{
        AstContext, DUMMY_SP, DimensionPercentage, ExclusiveTextDecorationLine,
        OtherTextDecorationLine, Spacing, TextDecorationLine, TextDecorationThickness,
    };

    #[test]
    fn text_node_codecs_preserve_ranges_and_keyword_variants() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let values = context.alloc_encoded_vec(
            [
                OtherTextDecorationLine::Underline,
                OtherTextDecorationLine::LineThrough,
            ]
            .into_iter(),
        );
        let line = context.alloc_encoded_node(TextDecorationLine::Value(values), DUMMY_SP);
        assert_eq!(
            context.encoded_node(line),
            TextDecorationLine::Value(values)
        );

        let exclusive = context.alloc_encoded_node(
            TextDecorationLine::ExclusiveTextDecorationLine(
                ExclusiveTextDecorationLine::GrammarError,
            ),
            DUMMY_SP,
        );
        assert_eq!(
            context.encoded_node(exclusive),
            TextDecorationLine::ExclusiveTextDecorationLine(
                ExclusiveTextDecorationLine::GrammarError,
            )
        );

        let length = context.alloc_encoded_node(DimensionPercentage::Percentage(2.0), DUMMY_SP);
        let thickness =
            context.alloc_encoded_node(TextDecorationThickness::LengthPercentage(length), DUMMY_SP);
        assert_eq!(
            context.encoded_node(thickness),
            TextDecorationThickness::LengthPercentage(length)
        );
        let spacing = context.alloc_encoded_node(Spacing::Normal, DUMMY_SP);
        assert_eq!(context.encoded_node(spacing), Spacing::Normal);
    }
}

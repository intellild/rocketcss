use crate::*;

use crate::{AstNodeStorage, ExtraData, ExtraDataClone, ExtraDataCompact, NodeKind, NodePayload};

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

#[derive(Debug, PartialEq, Visit)]
pub enum Spacing<'a> {
    Normal,
    Length(NodeId<'a, Length<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for Spacing<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0010_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Normal,
            1 => Self::Length(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded Spacing variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Normal => bytes[0] = 0,
            Self::Length(value) => write_node(&mut bytes, 1, value),
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextDecorationLine<'a> {
    ExclusiveTextDecorationLine(ExclusiveTextDecorationLine),
    Value(Vec<'a, OtherTextDecorationLine>),
}

impl<'ast> AstNodeStorage<'ast> for TextDecorationLine<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0010_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::ExclusiveTextDecorationLine(decode_exclusive_line(bytes[1])),
            1 => Self::Value(
                context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
            ),
            _ => panic!("invalid encoded TextDecorationLine variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::ExclusiveTextDecorationLine(value) => {
                bytes[0] = 0;
                bytes[1] = encode_exclusive_line(value);
            }
            Self::Value(value) => {
                bytes[0] = 1;
                write_range(&mut bytes, 4, value);
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ExclusiveTextDecorationLine {
    None,
    SpellingError,
    GrammarError,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OtherTextDecorationLine {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

impl ExtraDataCompact<'_> for OtherTextDecorationLine {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Underline => 0,
            Self::Overline => 1,
            Self::LineThrough => 2,
            Self::Blink => 3,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Underline,
            1 => Self::Overline,
            2 => Self::LineThrough,
            3 => Self::Blink,
            _ => panic!("invalid encoded OtherTextDecorationLine"),
        }
    }
}

impl ExtraDataClone<'_> for OtherTextDecorationLine {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextDecorationThickness<'a> {
    Auto,
    FromFont,
    LengthPercentage(NodeId<'a, LengthPercentage<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for TextDecorationThickness<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0010_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Auto,
            1 => Self::FromFont,
            2 => Self::LengthPercentage(context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            _ => panic!("invalid encoded TextDecorationThickness variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Auto => bytes[0] = 0,
            Self::FromFont => bytes[0] = 1,
            Self::LengthPercentage(value) => write_node(&mut bytes, 2, value),
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

fn write_node<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_u32(
        bytes,
        4,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn write_range<T>(bytes: &mut [u8], offset: usize, value: Vec<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        offset + 4,
        u32::try_from(value.end_index()).expect("AST range end exceeds four bytes"),
    );
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn encode_exclusive_line(value: ExclusiveTextDecorationLine) -> u8 {
    match value {
        ExclusiveTextDecorationLine::None => 0,
        ExclusiveTextDecorationLine::SpellingError => 1,
        ExclusiveTextDecorationLine::GrammarError => 2,
    }
}

fn decode_exclusive_line(value: u8) -> ExclusiveTextDecorationLine {
    match value {
        0 => ExclusiveTextDecorationLine::None,
        1 => ExclusiveTextDecorationLine::SpellingError,
        2 => ExclusiveTextDecorationLine::GrammarError,
        _ => panic!("invalid encoded ExclusiveTextDecorationLine"),
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextDecorationSkipInk {
    Auto,
    None,
    All,
}

#[derive(Debug, PartialEq, Visit)]
pub enum TextEmphasisStyle<'a> {
    None,
    Keyword {
        fill: TextEmphasisFillMode,
        shape: Option<TextEmphasisShape>,
    },
    String(&'a str),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum TextEmphasisFillMode {
    Filled,
    Open,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

#[derive(Debug, PartialEq, Visit)]
pub struct Content<'a> {
    pub value: Vec<'a, TokenOrValue<'a>>,
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

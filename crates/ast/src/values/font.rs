use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, PartialEq, Visit)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}

impl AstNodeStorage<'_> for FontWeight {
    const KIND: NodeKind = NodeKind::new(0x000c_0001);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Absolute(AbsoluteFontWeight::Weight(f32::from_bits(read_u32(
                &bytes, 4,
            )))),
            1 => Self::Absolute(AbsoluteFontWeight::Normal),
            2 => Self::Absolute(AbsoluteFontWeight::Bold),
            3 => Self::Bolder,
            4 => Self::Lighter,
            _ => panic!("invalid encoded FontWeight variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_font_weight(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_font_weight(self)
    }
}

impl AstNodeClone<'_> for FontWeight {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_font_weight(value: FontWeight) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        FontWeight::Absolute(AbsoluteFontWeight::Weight(value)) => {
            bytes[0] = 0;
            write_u32(&mut bytes, 4, value.to_bits());
        }
        FontWeight::Absolute(AbsoluteFontWeight::Normal) => bytes[0] = 1,
        FontWeight::Absolute(AbsoluteFontWeight::Bold) => bytes[0] = 2,
        FontWeight::Bolder => bytes[0] = 3,
        FontWeight::Lighter => bytes[0] = 4,
    }
    NodePayload::inline(&bytes)
}

#[derive(Debug, PartialEq, Visit)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontSize<'a> {
    Length(NodeId<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum AbsoluteFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

impl AstNodeStorage<'_> for FontStretch {
    const KIND: NodeKind = NodeKind::new(0x000c_0002);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Keyword(decode_font_stretch_keyword(bytes[1])),
            1 => Self::Percentage(f32::from_bits(read_u32(&bytes, 4))),
            _ => panic!("invalid encoded FontStretch variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        encode_font_stretch(self)
    }

    fn encode_existing(self, _current: NodePayload, _context: &mut AstContext<'_>) -> NodePayload {
        encode_font_stretch(self)
    }
}

impl AstNodeClone<'_> for FontStretch {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

fn encode_font_stretch(value: FontStretch) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        FontStretch::Keyword(value) => {
            bytes[0] = 0;
            bytes[1] = encode_font_stretch_keyword(value);
        }
        FontStretch::Percentage(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, value.to_bits());
        }
    }
    NodePayload::inline(&bytes)
}

fn encode_font_stretch_keyword(value: FontStretchKeyword) -> u8 {
    match value {
        FontStretchKeyword::Normal => 0,
        FontStretchKeyword::UltraCondensed => 1,
        FontStretchKeyword::ExtraCondensed => 2,
        FontStretchKeyword::Condensed => 3,
        FontStretchKeyword::SemiCondensed => 4,
        FontStretchKeyword::SemiExpanded => 5,
        FontStretchKeyword::Expanded => 6,
        FontStretchKeyword::ExtraExpanded => 7,
        FontStretchKeyword::UltraExpanded => 8,
    }
}

fn decode_font_stretch_keyword(value: u8) -> FontStretchKeyword {
    match value {
        0 => FontStretchKeyword::Normal,
        1 => FontStretchKeyword::UltraCondensed,
        2 => FontStretchKeyword::ExtraCondensed,
        3 => FontStretchKeyword::Condensed,
        4 => FontStretchKeyword::SemiCondensed,
        5 => FontStretchKeyword::SemiExpanded,
        6 => FontStretchKeyword::Expanded,
        7 => FontStretchKeyword::ExtraExpanded,
        8 => FontStretchKeyword::UltraExpanded,
        _ => panic!("invalid encoded FontStretchKeyword"),
    }
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("compact font field is four bytes"),
    )
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontStretchKeyword {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFamily<'a> {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
    SystemUi,
    Emoji,
    Math,
    Fangsong,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Initial,
    Inherit,
    Unset,
    Default,
    Revert,
    RevertLayer,
    Unparsed(Vec<'a, TokenOrValue<'a>>),
    /// Tombstone for a family entry removed by an in-place transform.
    #[css_keyword("")]
    Tombstone,
    Custom(&'a str),
}

impl<'a> FontFamily<'a> {
    pub fn from_name(name: &'a str) -> Self {
        match_ignore_ascii_case!(
            name,
            "serif" => Self::Serif,
            "sans-serif" => Self::SansSerif,
            "cursive" => Self::Cursive,
            "fantasy" => Self::Fantasy,
            "monospace" => Self::Monospace,
            "system-ui" => Self::SystemUi,
            "emoji" => Self::Emoji,
            "math" => Self::Math,
            "fangsong" => Self::Fangsong,
            "ui-serif" => Self::UiSerif,
            "ui-sans-serif" => Self::UiSansSerif,
            "ui-monospace" => Self::UiMonospace,
            "ui-rounded" => Self::UiRounded,
            "initial" => Self::Initial,
            "inherit" => Self::Inherit,
            "unset" => Self::Unset,
            "default" => Self::Default,
            "revert" => Self::Revert,
            "revert-layer" => Self::RevertLayer,
            _ => Self::Custom(name),
        )
    }

    #[inline]
    pub const fn is_generic(&self) -> bool {
        matches!(
            self,
            Self::Serif
                | Self::SansSerif
                | Self::Cursive
                | Self::Fantasy
                | Self::Monospace
                | Self::SystemUi
                | Self::Emoji
                | Self::Math
                | Self::Fangsong
                | Self::UiSerif
                | Self::UiSansSerif
                | Self::UiMonospace
                | Self::UiRounded
        )
    }

    #[inline]
    pub const fn is_tombstone(&self) -> bool {
        matches!(self, Self::Tombstone)
    }
}

impl EqIgnoringTombstones for FontFamily<'_> {
    #[inline]
    fn eq_ignoring_tombstones(&self, other: &Self, _ast: &AstContext<'_>) -> bool {
        self == other
    }
}

impl<'a> EqIgnoringTombstones for Vec<'a, FontFamily<'a>> {
    fn eq_ignoring_tombstones(&self, other: &Self, ast: &AstContext<'_>) -> bool {
        let left = ast.vec(*self).iter().filter(|value| !value.is_tombstone());
        let right = ast.vec(*other).iter().filter(|value| !value.is_tombstone());
        left.eq(right)
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[derive(Debug, PartialEq, Visit)]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(NodeId<'a, LengthPercentage<'a>>),
}

#[derive(Debug, PartialEq, Visit)]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(NodeId<'a, LengthPercentage<'a>>),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum VerticalAlignKeyword {
    Baseline,
    Sub,
    Super,
    Top,
    TextTop,
    Middle,
    Bottom,
    TextBottom,
}

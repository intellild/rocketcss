use crate::*;

use crate::{AstNodeClone, AstNodeStorage, NodeKind, NodePayload};

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontWeight {
    Absolute(AbsoluteFontWeight),
    Bolder,
    Lighter,
}

impl_inline_node!(FontWeight, 0x000c_0001);

impl AstNodeClone<'_> for FontWeight {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontSize<'a> {
    Length(NodeId<'a, LengthPercentage<'a>>),
    Absolute(AbsoluteFontSize),
    Relative(RelativeFontSize),
}

impl_inline_node!(FontSize<'ast>, 0x000c_0003);

impl<'ast> AstNodeClone<'ast> for FontSize<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum RelativeFontSize {
    Smaller,
    Larger,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontStretch {
    Keyword(FontStretchKeyword),
    Percentage(f32),
}

impl_inline_node!(FontStretch, 0x000c_0002);

impl AstNodeClone<'_> for FontStretch {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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
    Custom(AstStr<'a>),
}

// SAFETY: this KIND always stores and reads native FontFamily values.
unsafe impl<'ast> AstNodeStorage<'ast> for FontFamily<'ast> {
    const KIND: NodeKind = NodeKind::new(0x000c_0005);
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

impl<'ast> AstNodeClone<'ast> for FontFamily<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Unparsed(values) => Self::Unparsed(context.clone_encoded_vec(values)),
            value => value,
        }
    }
}

impl<'a> FontFamily<'a> {
    pub fn from_known_name(name: &str) -> Option<Self> {
        match_ignore_ascii_case!(
            name,
            "serif" => Some(Self::Serif),
            "sans-serif" => Some(Self::SansSerif),
            "cursive" => Some(Self::Cursive),
            "fantasy" => Some(Self::Fantasy),
            "monospace" => Some(Self::Monospace),
            "system-ui" => Some(Self::SystemUi),
            "emoji" => Some(Self::Emoji),
            "math" => Some(Self::Math),
            "fangsong" => Some(Self::Fangsong),
            "ui-serif" => Some(Self::UiSerif),
            "ui-sans-serif" => Some(Self::UiSansSerif),
            "ui-monospace" => Some(Self::UiMonospace),
            "ui-rounded" => Some(Self::UiRounded),
            "initial" => Some(Self::Initial),
            "inherit" => Some(Self::Inherit),
            "unset" => Some(Self::Unset),
            "default" => Some(Self::Default),
            "revert" => Some(Self::Revert),
            "revert-layer" => Some(Self::RevertLayer),
            _ => None,
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
    fn eq_ignoring_tombstones(&self, other: &Self, ast: &AstContext<'_>) -> bool {
        self.eq_in_context(other, ast)
    }
}

impl<'a> EqIgnoringTombstones for Vec<'a, NodeId<'a, FontFamily<'a>>> {
    fn eq_ignoring_tombstones(&self, other: &Self, ast: &AstContext<'_>) -> bool {
        let left = ast
            .vec_iter(*self)
            .map(|value| ast.resolve_node(value))
            .filter(|value| !value.is_tombstone());
        let right = ast
            .vec_iter(*other)
            .map(|value| ast.resolve_node(value))
            .filter(|value| !value.is_tombstone());
        let mut left = left;
        let mut right = right;
        loop {
            match (left.next(), right.next()) {
                (None, None) => return true,
                (Some(a), Some(b)) if a.eq_in_context(&b, ast) => {}
                _ => return false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}

impl_inline_node!(FontStyle, 0x000c_0006);

impl AstNodeClone<'_> for FontStyle {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontVariantCaps {
    Normal,
    SmallCaps,
    AllSmallCaps,
    PetiteCaps,
    AllPetiteCaps,
    Unicase,
    TitlingCaps,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum LineHeight<'a> {
    Normal,
    Number(f32),
    Length(NodeId<'a, LengthPercentage<'a>>),
}

impl_inline_node!(LineHeight<'ast>, 0x000c_0004);

impl<'ast> AstNodeClone<'ast> for LineHeight<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum VerticalAlign<'a> {
    Keyword(VerticalAlignKeyword),
    Length(NodeId<'a, LengthPercentage<'a>>),
}

impl_inline_node!(VerticalAlign<'ast>, 0x000c_0007);

impl<'ast> AstNodeClone<'ast> for VerticalAlign<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Length(value) => Self::Length(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
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

#[cfg(test)]
mod storage_tests {
    use super::FontFamily;
    use rocketcss_common::Allocator;

    use crate::{
        AbsoluteFontSize, AstContext, DUMMY_SP, DimensionPercentage, FontSize, LineHeight,
        RelativeFontSize,
    };

    #[test]
    fn native_font_names_keep_content_equality_and_reuse_storage() {
        assert_eq!(std::mem::size_of::<FontFamily<'_>>(), 12);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("Fancy 字体");
        let second = context.add_str("Fancy 字体");
        assert_ne!(first, second);
        let node = context.alloc_encoded_node(FontFamily::Custom(first), DUMMY_SP);
        let equal = context.alloc_encoded_node(FontFamily::Custom(second), DUMMY_SP);
        assert!(context.nodes_eq(node, equal));
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for value in [
            FontFamily::Serif,
            FontFamily::Tombstone,
            FontFamily::Custom(second),
            FontFamily::Inherit,
            FontFamily::Custom(first),
        ] {
            context.mutate_encoded_node(node, |stored, _| *stored = value);
            assert_eq!(context.encoded_node(node), value);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        assert_eq!(
            FontFamily::from_known_name("SANS-SERIF"),
            Some(FontFamily::SansSerif)
        );
        assert_eq!(FontFamily::from_known_name("Fancy 字体"), None);
    }

    #[test]
    fn font_size_and_line_height_codecs_preserve_variant_identity() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let size =
            context.alloc_encoded_node(FontSize::Absolute(AbsoluteFontSize::XxxLarge), DUMMY_SP);
        assert_eq!(
            context.encoded_node(size),
            FontSize::Absolute(AbsoluteFontSize::XxxLarge)
        );
        context.mutate_encoded_node(size, |value, _| {
            *value = FontSize::Relative(RelativeFontSize::Smaller);
        });
        assert_eq!(
            context.encoded_node(size),
            FontSize::Relative(RelativeFontSize::Smaller)
        );

        let length = context.alloc_encoded_node(DimensionPercentage::Percentage(125.0), DUMMY_SP);
        let line_height = context.alloc_encoded_node(LineHeight::Length(length), DUMMY_SP);
        assert_eq!(
            context.encoded_node(line_height),
            LineHeight::Length(length)
        );
    }
}

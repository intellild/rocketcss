use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum FontFaceProperty<'a> {
    Source(Vec<'a, Source<'a>>),
    FontFamily(NodeId<'a, FontFamily<'a>>),
    FontStyle(NodeId<'a, FontFaceStyle<'a>>),
    FontWeight(NodeId<'a, Size2D<'a, FontWeight>>),
    FontStretch(NodeId<'a, Size2D<'a, FontStretch>>),
    UnicodeRange(Vec<'a, UnicodeRange>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum Source<'a> {
    Url(NodeId<'a, UrlSource<'a>>),
    Local(NodeId<'a, FontFamily<'a>>),
}

impl_inline_extra!(Source<'ast>);

impl<'ast> ExtraDataClone<'ast> for Source<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Local(value) => Self::Local(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontFormat<'a> {
    Woff,
    Woff2,
    Truetype,
    Opentype,
    EmbeddedOpentype,
    Collection,
    Svg,
    String(AstStr<'a>),
}

// SAFETY: this KIND always stores and reads native FontFormat values.
unsafe impl<'ast> AstNodeStorage<'ast> for FontFormat<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0001);
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

impl<'ast> AstNodeClone<'ast> for FontFormat<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontTechnology {
    FeaturesOpentype,
    FeaturesAat,
    FeaturesGraphite,
    ColorColrv0,
    ColorColrv1,
    ColorSvg,
    ColorSbix,
    ColorCbdt,
    Variations,
    Palettes,
    Incremental,
}

impl_inline_extra!(FontTechnology);

impl ExtraDataClone<'_> for FontTechnology {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum FontFaceStyle<'a> {
    Normal,
    Italic,
    Oblique(NodeId<'a, Size2D<'a, Angle>>),
}

impl_inline_node!(FontFaceStyle<'ast>, 0x001f_0002);

impl<'ast> AstNodeClone<'ast> for FontFaceStyle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Oblique(value) => Self::Oblique(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontPaletteValuesProperty<'a> {
    FontFamily(NodeId<'a, FontFamily<'a>>),
    BasePalette(NodeId<'a, BasePalette>),
    OverrideColors(Vec<'a, OverrideColors<'a>>),
    Custom(NodeId<'a, CustomProperty<'a>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}

impl_inline_node!(BasePalette, 0x001f_0003);

impl AstNodeClone<'_> for BasePalette {
    fn clone_in_context(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFeatureSubruleType {
    Stylistic,
    HistoricalForms,
    Styleset,
    CharacterVariant,
    Swash,
    Ornaments,
    Annotation,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Font<'a> {
    pub family: Vec<'a, NodeId<'a, FontFamily<'a>>>,
    pub line_height: NodeId<'a, LineHeight<'a>>,
    pub size: NodeId<'a, FontSize<'a>>,
    pub stretch: FontStretch,
    pub style: NodeId<'a, FontStyle>,
    pub variant_caps: FontVariantCaps,
    pub weight: NodeId<'a, FontWeight>,
}

#[derive(Clone, Copy)]
struct FontHeader<'ast> {
    line_height: NodeId<'ast, LineHeight<'ast>>,
    size: NodeId<'ast, FontSize<'ast>>,
    extra: u32,
    variant_caps: FontVariantCaps,
}
pub use font_access::FontRead;

// Transient read view, excluded from persistent AST visitor generation.
mod font_access {
    use super::*;

    pub struct FontRead<'context, 'storage, 'id> {
        context: &'context AstContext<'storage>,
        header: FontHeader<'id>,
    }

    impl<'id> FontRead<'_, '_, 'id> {
        pub fn style_and_weight(&self) -> (NodeId<'id, FontStyle>, NodeId<'id, FontWeight>) {
            // SAFETY: slot one stores this native handle pair.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 1)
                    .read_value()
            }
        }
        pub fn family(&self) -> Vec<'id, NodeId<'id, FontFamily<'id>>> {
            // SAFETY: slot zero stores the family range.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize)
                    .read_value()
            }
        }
        pub fn stretch(&self) -> FontStretch {
            // SAFETY: slot two stores the native FontStretch enum.
            unsafe {
                self.context
                    .extra_slot(self.header.extra as usize + 2)
                    .read_value()
            }
        }
        pub fn variant_caps(&self) -> FontVariantCaps {
            self.header.variant_caps
        }
        pub fn size(&self) -> NodeId<'id, FontSize<'id>> {
            self.header.size
        }
        pub fn line_height(&self) -> NodeId<'id, LineHeight<'id>> {
            self.header.line_height
        }
    }

    impl<'storage> AstContext<'storage> {
        pub fn font<'id>(&self, id: NodeId<'id, Font<'id>>) -> FontRead<'_, 'storage, 'id> {
            // SAFETY: node_payload checks the owning Font kind before reading its header.
            FontRead {
                context: self,
                header: unsafe { self.node_payload(id).read_value() },
            }
        }
    }
}

// SAFETY: this kind stores FontHeader plus a family range, handle pair and
// FontStretch in three separately typed slots.
unsafe impl<'ast> AstNodeStorage<'ast> for Font<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0004);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: FontHeader<'ast> = unsafe { payload.read_value() };
        let extra = header.extra as usize;
        let (style, weight) = unsafe { context.extra_slot(extra + 1).read_value() };
        Self {
            family: unsafe { context.extra_slot(extra).read_value() },
            line_height: header.line_height,
            size: header.size,
            stretch: unsafe { context.extra_slot(extra + 2).read_value() },
            style,
            variant_caps: header.variant_caps,
            weight,
        }
    }
    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        store_font(self, None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: FontHeader<'ast> = unsafe { current.read_value() };
        store_font(self, Some(header.extra as usize), context)
    }
}

impl<'ast> AstNodeClone<'ast> for Font<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            family: context.clone_encoded_vec(self.family),
            line_height: context.clone_encoded_node(self.line_height),
            size: context.clone_encoded_node(self.size),
            stretch: self.stretch,
            style: context.clone_encoded_node(self.style),
            variant_caps: self.variant_caps,
            weight: context.clone_encoded_node(self.weight),
        }
    }
}

fn store_font<'ast>(
    value: Font<'ast>,
    existing: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let slots = [
        ExtraData::from_value(value.family),
        ExtraData::from_value((value.style, value.weight)),
        ExtraData::from_value(value.stretch),
    ];
    let extra = match existing {
        Some(index) => {
            for (offset, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(index + offset, slot);
            }
            index
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(FontHeader {
        line_height: value.line_height,
        size: value.size,
        variant_caps: value.variant_caps,
        extra: u32::try_from(extra).expect("extra index exceeds u32"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct UrlSource<'a> {
    pub format: Option<NodeId<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: NodeId<'a, Url<'a>>,
}

impl_inline_node!(UrlSource<'ast>, 0x001f_0005);

impl<'ast> AstNodeClone<'ast> for UrlSource<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            format: self.format.map(|value| context.clone_encoded_node(value)),
            tech: context.clone_encoded_vec(self.tech),
            url: context.clone_encoded_node(self.url),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

impl_inline_extra!(UnicodeRange);

impl ExtraDataClone<'_> for UnicodeRange {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct OverrideColors<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub index: u16,
}

impl_inline_extra!(OverrideColors<'ast>);

impl<'ast> ExtraDataClone<'ast> for OverrideColors<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            color: context.clone_encoded_node(self.color),
            index: self.index,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct FontFeatureDeclaration<'a> {
    pub name: AstStr<'a>,
    pub values: Vec<'a, i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Visit)]
pub struct FamilyName<'a>(pub AstStr<'a>);

// SAFETY: FamilyName slots preserve the native eight-byte range value.
unsafe impl<'ast> ExtraDataCompact<'ast> for FamilyName<'ast> {
    fn encode_extra(self) -> ExtraData {
        ExtraData::from_value(self)
    }
    unsafe fn decode_extra(data: ExtraData) -> Self {
        unsafe { data.read_value() }
    }
}

impl<'ast> ExtraDataClone<'ast> for FamilyName<'ast> {
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn native_font_storage_preserves_float_bits_and_reuses_overflow() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let family_node = ast.alloc_node(FontFamily::Serif, DUMMY_SP);
        let family = ast.alloc_encoded_vec([family_node].into_iter());
        let line_height = ast.alloc_node(LineHeight::Normal, DUMMY_SP);
        let size = ast.alloc_node(FontSize::Absolute(AbsoluteFontSize::Medium), DUMMY_SP);
        let style = ast.alloc_node(FontStyle::Normal, DUMMY_SP);
        let weight = ast.alloc_node(FontWeight::Absolute(AbsoluteFontWeight::Normal), DUMMY_SP);
        let before = ast.encoded_extra_len();
        let font = ast.alloc_node(
            Font {
                family,
                line_height,
                size,
                stretch: FontStretch::Keyword(FontStretchKeyword::Normal),
                style,
                variant_caps: FontVariantCaps::Normal,
                weight,
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before + 3);
        let checkpoint = ast.node_checkpoint();
        for bits in [0x8000_0000, 0x7f80_0000, 0xff80_0000, 0x7fc0_1234] {
            let number = f32::from_bits(bits);
            ast.mutate_node(font, |value, _| {
                value.stretch = FontStretch::Percentage(number);
                value.variant_caps = FontVariantCaps::AllSmallCaps;
            });
            ast.mutate_node(weight, |value, _| {
                *value = FontWeight::Absolute(AbsoluteFontWeight::Weight(number))
            });
            ast.mutate_node(style, |value, _| {
                *value = FontStyle::Oblique(Angle::Turn(number))
            });
            ast.mutate_node(line_height, |value, _| *value = LineHeight::Number(number));
            let actual = ast.resolve_node(font);
            assert_eq!(
                (
                    actual.family,
                    actual.line_height,
                    actual.size,
                    actual.style,
                    actual.weight
                ),
                (family, line_height, size, style, weight)
            );
            assert_eq!(actual.variant_caps, FontVariantCaps::AllSmallCaps);
            let view = ast.font(font);
            assert_eq!(view.family(), family);
            assert_eq!(view.style_and_weight(), (style, weight));
            assert_eq!(view.size(), size);
            assert_eq!(view.line_height(), line_height);
            assert_eq!(view.variant_caps(), FontVariantCaps::AllSmallCaps);
            let FontStretch::Percentage(stretch) = view.stretch() else {
                panic!("expected percentage view")
            };
            assert_eq!(stretch.to_bits(), bits);
            let FontStretch::Percentage(value) = actual.stretch else {
                panic!("expected percentage")
            };
            assert_eq!(value.to_bits(), bits);
            let FontWeight::Absolute(AbsoluteFontWeight::Weight(value)) = ast.resolve_node(weight)
            else {
                panic!("expected numeric weight")
            };
            assert_eq!(value.to_bits(), bits);
            let FontStyle::Oblique(Angle::Turn(value)) = ast.resolve_node(style) else {
                panic!("expected turns")
            };
            assert_eq!(value.to_bits(), bits);
            let LineHeight::Number(value) = ast.resolve_node(line_height) else {
                panic!("expected number")
            };
            assert_eq!(value.to_bits(), bits);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        for caps in [
            FontVariantCaps::Normal,
            FontVariantCaps::SmallCaps,
            FontVariantCaps::AllSmallCaps,
            FontVariantCaps::PetiteCaps,
            FontVariantCaps::AllPetiteCaps,
            FontVariantCaps::Unicase,
            FontVariantCaps::TitlingCaps,
        ] {
            for keyword in [
                FontStretchKeyword::Normal,
                FontStretchKeyword::UltraCondensed,
                FontStretchKeyword::ExtraCondensed,
                FontStretchKeyword::Condensed,
                FontStretchKeyword::SemiCondensed,
                FontStretchKeyword::SemiExpanded,
                FontStretchKeyword::Expanded,
                FontStretchKeyword::ExtraExpanded,
                FontStretchKeyword::UltraExpanded,
            ] {
                ast.mutate_node(font, |value, _| {
                    value.variant_caps = caps;
                    value.stretch = FontStretch::Keyword(keyword);
                });
                let actual = ast.resolve_node(font);
                assert_eq!(actual.variant_caps, caps);
                assert_eq!(actual.stretch, FontStretch::Keyword(keyword));
                let view = ast.font(font);
                assert_eq!(view.variant_caps(), caps);
                assert_eq!(view.stretch(), FontStretch::Keyword(keyword));
                assert_eq!(view.family(), family);
                assert_eq!(view.style_and_weight(), (style, weight));
                assert_eq!(ast.node_checkpoint(), checkpoint);
            }
        }
        let clone = ast.clone_node(font);
        let cloned_family = ast.resolve_node(clone).family;
        let cloned_name = ast.encoded_vec_get(cloned_family, 0).unwrap();
        assert_ne!(cloned_name, family_node);
        ast.mutate_node(cloned_name, |value, _| *value = FontFamily::Monospace);
        assert_eq!(ast.resolve_node(family_node), FontFamily::Serif);
    }

    #[test]
    fn native_font_source_and_palette_slots_preserve_boundaries() {
        let allocator = Allocator::new();
        let mut ast = AstContext::new_in(&allocator);
        let format = ast.alloc_node(FontFormat::Woff2, DUMMY_SP);
        let text = ast.add_str("font.woff2");
        let url = ast.alloc_node(Url { url: text }, DUMMY_SP);
        let tech = ast.alloc_encoded_vec(
            [
                FontTechnology::FeaturesOpentype,
                FontTechnology::Incremental,
            ]
            .into_iter(),
        );
        let before = ast.encoded_extra_len();
        let source = ast.alloc_node(
            UrlSource {
                format: None,
                tech,
                url,
            },
            DUMMY_SP,
        );
        assert_eq!(ast.encoded_extra_len(), before);
        let checkpoint = ast.node_checkpoint();
        for format in [Some(format), None] {
            let expected = UrlSource { format, tech, url };
            ast.mutate_node(source, |value, _| *value = expected);
            assert_eq!(ast.resolve_node(source), expected);
        }
        assert_eq!(ast.node_checkpoint(), checkpoint);
        let sources = ast.alloc_encoded_vec([Source::Url(source)].into_iter());
        assert_eq!(ast.encoded_vec_get(sources, 0), Some(Source::Url(source)));
        let ranges = ast.alloc_encoded_vec(
            [
                UnicodeRange {
                    start: 0,
                    end: 0x10ffff,
                },
                UnicodeRange {
                    start: u32::MAX,
                    end: u32::MAX,
                },
            ]
            .into_iter(),
        );
        assert_eq!(
            ast.encoded_vec_get(ranges, 0),
            Some(UnicodeRange {
                start: 0,
                end: 0x10ffff
            })
        );
        assert_eq!(
            ast.encoded_vec_get(ranges, 1),
            Some(UnicodeRange {
                start: u32::MAX,
                end: u32::MAX
            })
        );
        let color = ast.alloc_node(CssColor::CurrentColor, DUMMY_SP);
        let overrides = ast.alloc_encoded_vec(
            [OverrideColors {
                color,
                index: u16::MAX,
            }]
            .into_iter(),
        );
        assert_eq!(
            ast.encoded_vec_get(overrides, 0),
            Some(OverrideColors {
                color,
                index: u16::MAX
            })
        );
    }

    #[test]
    fn font_format_and_family_slots_keep_ranges_without_reference_rows() {
        assert_eq!(std::mem::size_of::<FontFormat<'_>>(), 12);
        assert_eq!(std::mem::size_of::<FamilyName<'_>>(), 8);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.add_str("woff3");
        let second = context.add_str("woff3");
        let format = context.alloc_encoded_node(FontFormat::String(first), DUMMY_SP);
        let equal = context.alloc_encoded_node(FontFormat::String(second), DUMMY_SP);
        assert!(context.nodes_eq(format, equal));
        let names = context.alloc_encoded_vec([FamilyName(first), FamilyName(second)].into_iter());
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for value in [
            FontFormat::Woff,
            FontFormat::Woff2,
            FontFormat::Truetype,
            FontFormat::Opentype,
            FontFormat::EmbeddedOpentype,
            FontFormat::Collection,
            FontFormat::Svg,
            FontFormat::String(second),
        ] {
            context.mutate_encoded_node(format, |node, _| *node = value);
            assert_eq!(context.encoded_node(format), value);
            assert_eq!(context.encoded_vec_get(names, 0), Some(FamilyName(first)));
            assert_eq!(context.encoded_vec_get(names, 1), Some(FamilyName(second)));
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }
}

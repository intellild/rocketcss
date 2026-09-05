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

#[derive(Debug, PartialEq, Visit)]
pub enum Source<'a> {
    Url(NodeId<'a, UrlSource<'a>>),
    Local(NodeId<'a, FontFamily<'a>>),
}

impl<'ast> ExtraDataCompact<'ast> for Source<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        let (tag, id) = match self {
            Self::Url(value) => (0, node_index(value)),
            Self::Local(value) => (1, node_index(value)),
        };
        ExtraData::from_u64((id as u64) << 8 | tag)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let encoded = data.as_u64();
        let id = (encoded >> 8) as u32 as usize;
        match encoded as u8 {
            0 => Self::Url(context.encoded_node_id_at(id)),
            1 => Self::Local(context.encoded_node_id_at(id)),
            _ => panic!("invalid encoded font source"),
        }
    }
}

impl<'ast> ExtraDataClone<'ast> for Source<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Url(value) => Self::Url(context.clone_encoded_node(value)),
            Self::Local(value) => Self::Local(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum FontFormat<'a> {
    Woff,
    Woff2,
    Truetype,
    Opentype,
    EmbeddedOpentype,
    Collection,
    Svg,
    String(&'a str),
}

impl<'ast> AstNodeStorage<'ast> for FontFormat<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Woff,
            1 => Self::Woff2,
            2 => Self::Truetype,
            3 => Self::Opentype,
            4 => Self::EmbeddedOpentype,
            5 => Self::Collection,
            6 => Self::Svg,
            7 => Self::String(context.resolve_string(read_u32(&bytes, 4) as u64)),
            _ => panic!("invalid encoded FontFormat variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Woff => bytes[0] = 0,
            Self::Woff2 => bytes[0] = 1,
            Self::Truetype => bytes[0] = 2,
            Self::Opentype => bytes[0] = 3,
            Self::EmbeddedOpentype => bytes[0] = 4,
            Self::Collection => bytes[0] = 5,
            Self::Svg => bytes[0] = 6,
            Self::String(value) => {
                bytes[0] = 7;
                write_u32(&mut bytes, 4, context.store_string(value));
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for FontFormat<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
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

impl ExtraDataCompact<'_> for FontTechnology {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(encode_font_technology(self) as u64)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        decode_font_technology(data.as_u64() as u8)
    }
}

impl ExtraDataClone<'_> for FontTechnology {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum FontFaceStyle<'a> {
    Normal,
    Italic,
    Oblique(NodeId<'a, Size2D<'a, Angle>>),
}

impl<'ast> AstNodeStorage<'ast> for FontFaceStyle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Normal,
            1 => Self::Italic,
            2 => Self::Oblique(read_node_id(&bytes, 4, context)),
            _ => panic!("invalid encoded FontFaceStyle variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Normal => bytes[0] = 0,
            Self::Italic => bytes[0] = 1,
            Self::Oblique(value) => {
                bytes[0] = 2;
                write_u32(&mut bytes, 4, node_index(value));
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

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

#[derive(Debug, PartialEq, Visit)]
pub enum BasePalette {
    Light,
    Dark,
    Integer(u16),
}

impl AstNodeStorage<'_> for BasePalette {
    const KIND: NodeKind = NodeKind::new(0x001f_0003);

    fn decode(payload: NodePayload, _context: &AstContext<'_>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Light,
            1 => Self::Dark,
            2 => Self::Integer(u16::from_le_bytes([bytes[2], bytes[3]])),
            _ => panic!("invalid encoded BasePalette variant"),
        }
    }

    fn encode_new(self, _context: &mut AstContext<'_>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Light => bytes[0] = 0,
            Self::Dark => bytes[0] = 1,
            Self::Integer(value) => {
                bytes[0] = 2;
                bytes[2..4].copy_from_slice(&value.to_le_bytes());
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'_>) -> NodePayload {
        self.encode_new(context)
    }
}

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

impl<'ast> AstNodeStorage<'ast> for Font<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let family = context.extra_slot(payload.extra_start()).as_u64();
        let ids = context.extra_slot(payload.extra_start() + 1).as_u64();
        let stretch = context.extra_slot(payload.extra_start() + 2).bytes();
        let (style, weight) = unpack_ids(ids);
        Self {
            family: context
                .encoded_vec_range(family as u32 as usize, (family >> 32) as u32 as usize),
            line_height: read_node_id(&bytes, 4, context),
            size: read_node_id(&bytes, 8, context),
            stretch: decode_font_stretch(&stretch),
            style: context.encoded_node_id_at(style),
            variant_caps: decode_font_variant_caps(bytes[0]),
            weight: context.encoded_node_id_at(weight),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_font(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_font(self, Some(current.extra_start()), context)
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

fn encode_font<'ast>(
    value: Font<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    bytes[0] = encode_font_variant_caps(value.variant_caps);
    write_u32(&mut bytes, 4, node_index(value.line_height));
    write_u32(&mut bytes, 8, node_index(value.size));
    let slots = [
        ExtraData::from_u64(pack_ids(
            value.family.start_index(),
            value.family.end_index(),
        )),
        ExtraData::from_u64(pack_ids(value.style.index(), value.weight.index())),
        ExtraData::from_bytes(&encode_font_stretch(value.stretch)),
    ];
    let extra_start = match existing_extra {
        Some(extra_start) => {
            for (offset, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra_start + offset, slot);
            }
            extra_start
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Visit)]
pub struct UrlSource<'a> {
    pub format: Option<NodeId<'a, FontFormat<'a>>>,
    pub tech: Vec<'a, FontTechnology>,
    pub url: NodeId<'a, Url<'a>>,
}

impl<'ast> AstNodeStorage<'ast> for UrlSource<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001f_0005);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let tech = context.extra_slot(payload.extra_start()).as_u64();
        Self {
            format: (read_u32(&bytes, 4) != u32::MAX)
                .then(|| context.encoded_node_id_at(read_u32(&bytes, 4) as usize)),
            tech: context.encoded_vec_range(tech as u32 as usize, (tech >> 32) as u32 as usize),
            url: read_node_id(&bytes, 8, context),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_url_source(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_url_source(self, Some(current.extra_start()), context)
    }
}

impl<'ast> AstNodeClone<'ast> for UrlSource<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            format: self.format.map(|value| context.clone_encoded_node(value)),
            tech: context.clone_encoded_vec(self.tech),
            url: context.clone_encoded_node(self.url),
        }
    }
}

fn encode_url_source<'ast>(
    value: UrlSource<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_u32(&mut bytes, 4, value.format.map_or(u32::MAX, node_index));
    write_u32(&mut bytes, 8, node_index(value.url));
    let slot = ExtraData::from_u64(pack_ids(value.tech.start_index(), value.tech.end_index()));
    let extra_start = match existing_extra {
        Some(extra_start) => {
            context.set_extra_slot(extra_start, slot);
            extra_start
        }
        None => context.alloc_extra_slots([slot]),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Visit)]
pub struct UnicodeRange {
    pub end: u32,
    pub start: u32,
}

impl ExtraDataCompact<'_> for UnicodeRange {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64((self.end as u64) << 32 | self.start as u64)
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        Self {
            end: (data.as_u64() >> 32) as u32,
            start: data.as_u64() as u32,
        }
    }
}

impl ExtraDataClone<'_> for UnicodeRange {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub struct OverrideColors<'a> {
    pub color: NodeId<'a, CssColor<'a>>,
    pub index: u16,
}

impl<'ast> ExtraDataCompact<'ast> for OverrideColors<'ast> {
    fn encode_extra(self, _context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64((node_index(self.color) as u64) << 16 | self.index as u64)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self {
            color: context.encoded_node_id_at((data.as_u64() >> 16) as u32 as usize),
            index: data.as_u64() as u16,
        }
    }
}

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
    pub name: &'a str,
    pub values: Vec<'a, i32>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct FamilyName<'a>(pub &'a str);

impl<'ast> ExtraDataCompact<'ast> for FamilyName<'ast> {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        ExtraData::from_u64(context.store_string(self.0) as u64)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        Self(context.resolve_string(data.as_u64()))
    }
}

impl<'ast> ExtraDataClone<'ast> for FamilyName<'ast> {
    fn clone_extra(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn read_node_id<'ast, T>(
    bytes: &[u8],
    offset: usize,
    context: &AstContext<'ast>,
) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, offset) as usize)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

fn pack_ids(first: usize, second: usize) -> u64 {
    let first = u32::try_from(first).expect("AST compact index exceeds four bytes");
    let second = u32::try_from(second).expect("AST compact index exceeds four bytes");
    (second as u64) << 32 | first as u64
}

fn unpack_ids(value: u64) -> (usize, usize) {
    (value as u32 as usize, (value >> 32) as u32 as usize)
}

fn encode_font_stretch(value: FontStretch) -> [u8; ExtraData::BYTES] {
    let mut bytes = [0; ExtraData::BYTES];
    match value {
        FontStretch::Keyword(value) => {
            bytes[0] = 0;
            bytes[1] = match value {
                FontStretchKeyword::Normal => 0,
                FontStretchKeyword::UltraCondensed => 1,
                FontStretchKeyword::ExtraCondensed => 2,
                FontStretchKeyword::Condensed => 3,
                FontStretchKeyword::SemiCondensed => 4,
                FontStretchKeyword::SemiExpanded => 5,
                FontStretchKeyword::Expanded => 6,
                FontStretchKeyword::ExtraExpanded => 7,
                FontStretchKeyword::UltraExpanded => 8,
            };
        }
        FontStretch::Percentage(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, value.to_bits());
        }
    }
    bytes
}

fn decode_font_stretch(bytes: &[u8]) -> FontStretch {
    match bytes[0] {
        0 => FontStretch::Keyword(match bytes[1] {
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
        }),
        1 => FontStretch::Percentage(f32::from_bits(read_u32(bytes, 4))),
        _ => panic!("invalid encoded FontStretch"),
    }
}

fn encode_font_variant_caps(value: FontVariantCaps) -> u8 {
    match value {
        FontVariantCaps::Normal => 0,
        FontVariantCaps::SmallCaps => 1,
        FontVariantCaps::AllSmallCaps => 2,
        FontVariantCaps::PetiteCaps => 3,
        FontVariantCaps::AllPetiteCaps => 4,
        FontVariantCaps::Unicase => 5,
        FontVariantCaps::TitlingCaps => 6,
    }
}

fn decode_font_variant_caps(value: u8) -> FontVariantCaps {
    match value {
        0 => FontVariantCaps::Normal,
        1 => FontVariantCaps::SmallCaps,
        2 => FontVariantCaps::AllSmallCaps,
        3 => FontVariantCaps::PetiteCaps,
        4 => FontVariantCaps::AllPetiteCaps,
        5 => FontVariantCaps::Unicase,
        6 => FontVariantCaps::TitlingCaps,
        _ => panic!("invalid encoded FontVariantCaps"),
    }
}

fn encode_font_technology(value: FontTechnology) -> u8 {
    match value {
        FontTechnology::FeaturesOpentype => 0,
        FontTechnology::FeaturesAat => 1,
        FontTechnology::FeaturesGraphite => 2,
        FontTechnology::ColorColrv0 => 3,
        FontTechnology::ColorColrv1 => 4,
        FontTechnology::ColorSvg => 5,
        FontTechnology::ColorSbix => 6,
        FontTechnology::ColorCbdt => 7,
        FontTechnology::Variations => 8,
        FontTechnology::Palettes => 9,
        FontTechnology::Incremental => 10,
    }
}

fn decode_font_technology(value: u8) -> FontTechnology {
    match value {
        0 => FontTechnology::FeaturesOpentype,
        1 => FontTechnology::FeaturesAat,
        2 => FontTechnology::FeaturesGraphite,
        3 => FontTechnology::ColorColrv0,
        4 => FontTechnology::ColorColrv1,
        5 => FontTechnology::ColorSvg,
        6 => FontTechnology::ColorSbix,
        7 => FontTechnology::ColorCbdt,
        8 => FontTechnology::Variations,
        9 => FontTechnology::Palettes,
        10 => FontTechnology::Incremental,
        _ => panic!("invalid encoded FontTechnology"),
    }
}

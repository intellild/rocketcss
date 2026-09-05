use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum ListStyleType<'a> {
    None,
    String(&'a str),
    CounterStyle(NodeId<'a, CounterStyle<'a>>),
}

impl<'ast> AstNodeStorage<'ast> for ListStyleType<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0020_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let value = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::None,
            1 => Self::String(context.resolve_string(value as u64)),
            2 => Self::CounterStyle(context.encoded_node_id_at(value as usize)),
            _ => panic!("invalid encoded ListStyleType variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        let value = match self {
            Self::None => 0,
            Self::String(value) => {
                bytes[0] = 1;
                context.store_string(value)
            }
            Self::CounterStyle(value) => {
                bytes[0] = 2;
                node_index(value)
            }
        };
        write_u32(&mut bytes, 4, value);
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for ListStyleType<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::CounterStyle(value) => Self::CounterStyle(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum CounterStyle<'a> {
    Predefined(PredefinedCounterStyle),
    Name(&'a str),
    Symbols {
        symbols: Vec<'a, Symbol<'a>>,
        system: SymbolsType,
    },
}

impl<'ast> AstNodeStorage<'ast> for CounterStyle<'ast> {
    const KIND: NodeKind = NodeKind::new(0x0020_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Predefined(decode_predefined_counter_style(bytes[1])),
            1 => Self::Name(context.resolve_string(read_u32(&bytes, 4) as u64)),
            2 => Self::Symbols {
                symbols: context
                    .encoded_vec_range(read_u32(&bytes, 4) as usize, read_u32(&bytes, 8) as usize),
                system: decode_symbols_type(bytes[1]),
            },
            _ => panic!("invalid encoded CounterStyle variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        let mut bytes = [0; NodePayload::INLINE_BYTES];
        match self {
            Self::Predefined(value) => {
                bytes[0] = 0;
                bytes[1] = value as u8;
            }
            Self::Name(value) => {
                bytes[0] = 1;
                write_u32(&mut bytes, 4, context.store_string(value));
            }
            Self::Symbols { symbols, system } => {
                bytes[0] = 2;
                bytes[1] = encode_symbols_type(system);
                write_u32(&mut bytes, 4, symbols.start_index());
                write_u32(&mut bytes, 8, symbols.end_index());
            }
        }
        NodePayload::inline(&bytes)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        self.encode_new(context)
    }
}

impl<'ast> AstNodeClone<'ast> for CounterStyle<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Symbols { symbols, system } => Self::Symbols {
                symbols: context.clone_encoded_vec(symbols),
                system,
            },
            value => value,
        }
    }
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Visit)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    #[default]
    Symbolic,
    Fixed,
}

#[repr(u8)]
#[derive(CssKeyword, Clone, Copy, Debug, PartialEq, Visit)]
pub enum PredefinedCounterStyle {
    Decimal,
    DecimalLeadingZero,
    ArabicIndic,
    Armenian,
    UpperArmenian,
    LowerArmenian,
    Bengali,
    Cambodian,
    Khmer,
    CjkDecimal,
    Devanagari,
    Georgian,
    Gujarati,
    Gurmukhi,
    Hebrew,
    Kannada,
    Lao,
    Malayalam,
    Mongolian,
    Myanmar,
    Oriya,
    Persian,
    LowerRoman,
    UpperRoman,
    Tamil,
    Telugu,
    Thai,
    Tibetan,
    LowerAlpha,
    LowerLatin,
    UpperAlpha,
    UpperLatin,
    LowerGreek,
    Hiragana,
    HiraganaIroha,
    Katakana,
    KatakanaIroha,
    Disc,
    Circle,
    Square,
    DisclosureOpen,
    DisclosureClosed,
    CjkEarthlyBranch,
    CjkHeavenlyStem,
    JapaneseInformal,
    JapaneseFormal,
    KoreanHangulFormal,
    KoreanHanjaInformal,
    KoreanHanjaFormal,
    SimpChineseInformal,
    SimpChineseFormal,
    TradChineseInformal,
    TradChineseFormal,
    EthiopicNumeric,
}

#[derive(Debug, PartialEq, Visit)]
pub enum Symbol<'a> {
    String(&'a str),
    Image(NodeId<'a, Image<'a>>),
}

impl<'ast> ExtraDataCompact<'ast> for Symbol<'ast> {
    fn encode_extra(self, context: &mut AstContext<'ast>) -> ExtraData {
        let mut bytes = [0; ExtraData::BYTES];
        let value = match self {
            Self::String(value) => context.store_string(value),
            Self::Image(value) => {
                bytes[0] = 1;
                node_index(value)
            }
        };
        bytes[4..8].copy_from_slice(&value.to_le_bytes());
        ExtraData::from_bytes(&bytes)
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'ast>) -> Self {
        let bytes = data.bytes();
        let value = read_u32(&bytes, 4);
        match bytes[0] {
            0 => Self::String(context.resolve_string(value as u64)),
            1 => Self::Image(context.encoded_node_id_at(value as usize)),
            _ => panic!("invalid encoded Symbol variant"),
        }
    }
}

impl<'ast> ExtraDataClone<'ast> for Symbol<'ast> {
    fn clone_extra(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::String(value) => Self::String(value),
            Self::Image(value) => Self::Image(context.clone_encoded_node(value)),
        }
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MarkerSide {
    MatchSelf,
    MatchParent,
}

fn node_index<T>(id: NodeId<'_, T>) -> u32 {
    u32::try_from(id.index()).expect("AST node ID exceeds four bytes")
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn write_u32(bytes: &mut [u8], offset: usize, value: impl TryInto<u32>) {
    bytes[offset..offset + 4].copy_from_slice(
        &value
            .try_into()
            .unwrap_or_else(|_| panic!("AST compact value exceeds four bytes"))
            .to_le_bytes(),
    );
}

fn encode_symbols_type(value: SymbolsType) -> u8 {
    match value {
        SymbolsType::Cyclic => 0,
        SymbolsType::Numeric => 1,
        SymbolsType::Alphabetic => 2,
        SymbolsType::Symbolic => 3,
        SymbolsType::Fixed => 4,
    }
}

fn decode_symbols_type(value: u8) -> SymbolsType {
    match value {
        0 => SymbolsType::Cyclic,
        1 => SymbolsType::Numeric,
        2 => SymbolsType::Alphabetic,
        3 => SymbolsType::Symbolic,
        4 => SymbolsType::Fixed,
        _ => panic!("invalid encoded SymbolsType"),
    }
}

fn decode_predefined_counter_style(value: u8) -> PredefinedCounterStyle {
    const VALUES: &[PredefinedCounterStyle] = &[
        PredefinedCounterStyle::Decimal,
        PredefinedCounterStyle::DecimalLeadingZero,
        PredefinedCounterStyle::ArabicIndic,
        PredefinedCounterStyle::Armenian,
        PredefinedCounterStyle::UpperArmenian,
        PredefinedCounterStyle::LowerArmenian,
        PredefinedCounterStyle::Bengali,
        PredefinedCounterStyle::Cambodian,
        PredefinedCounterStyle::Khmer,
        PredefinedCounterStyle::CjkDecimal,
        PredefinedCounterStyle::Devanagari,
        PredefinedCounterStyle::Georgian,
        PredefinedCounterStyle::Gujarati,
        PredefinedCounterStyle::Gurmukhi,
        PredefinedCounterStyle::Hebrew,
        PredefinedCounterStyle::Kannada,
        PredefinedCounterStyle::Lao,
        PredefinedCounterStyle::Malayalam,
        PredefinedCounterStyle::Mongolian,
        PredefinedCounterStyle::Myanmar,
        PredefinedCounterStyle::Oriya,
        PredefinedCounterStyle::Persian,
        PredefinedCounterStyle::LowerRoman,
        PredefinedCounterStyle::UpperRoman,
        PredefinedCounterStyle::Tamil,
        PredefinedCounterStyle::Telugu,
        PredefinedCounterStyle::Thai,
        PredefinedCounterStyle::Tibetan,
        PredefinedCounterStyle::LowerAlpha,
        PredefinedCounterStyle::LowerLatin,
        PredefinedCounterStyle::UpperAlpha,
        PredefinedCounterStyle::UpperLatin,
        PredefinedCounterStyle::LowerGreek,
        PredefinedCounterStyle::Hiragana,
        PredefinedCounterStyle::HiraganaIroha,
        PredefinedCounterStyle::Katakana,
        PredefinedCounterStyle::KatakanaIroha,
        PredefinedCounterStyle::Disc,
        PredefinedCounterStyle::Circle,
        PredefinedCounterStyle::Square,
        PredefinedCounterStyle::DisclosureOpen,
        PredefinedCounterStyle::DisclosureClosed,
        PredefinedCounterStyle::CjkEarthlyBranch,
        PredefinedCounterStyle::CjkHeavenlyStem,
        PredefinedCounterStyle::JapaneseInformal,
        PredefinedCounterStyle::JapaneseFormal,
        PredefinedCounterStyle::KoreanHangulFormal,
        PredefinedCounterStyle::KoreanHanjaInformal,
        PredefinedCounterStyle::KoreanHanjaFormal,
        PredefinedCounterStyle::SimpChineseInformal,
        PredefinedCounterStyle::SimpChineseFormal,
        PredefinedCounterStyle::TradChineseInformal,
        PredefinedCounterStyle::TradChineseFormal,
        PredefinedCounterStyle::EthiopicNumeric,
    ];
    VALUES
        .get(value as usize)
        .copied()
        .expect("invalid encoded PredefinedCounterStyle")
}

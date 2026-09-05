use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

impl ExtraDataCompact<'_> for MaskMode {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Luminance => 0,
            Self::Alpha => 1,
            Self::MatchSource => 2,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Luminance,
            1 => Self::Alpha,
            2 => Self::MatchSource,
            _ => panic!("invalid encoded MaskMode"),
        }
    }
}

impl ExtraDataClone<'_> for MaskMode {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}

impl ExtraDataCompact<'_> for MaskClip {
    fn encode_extra(self, context: &mut AstContext<'_>) -> ExtraData {
        match self {
            Self::GeometryBox(value) => {
                let encoded = value.encode_extra(context).as_u64();
                ExtraData::from_u64(encoded << 8)
            }
            Self::NoClip => ExtraData::from_u64(1),
        }
    }

    fn decode_extra(data: ExtraData, context: &AstContext<'_>) -> Self {
        let encoded = data.as_u64();
        match encoded as u8 {
            0 => Self::GeometryBox(GeometryBox::decode_extra(
                ExtraData::from_u64(encoded >> 8),
                context,
            )),
            1 => Self::NoClip,
            _ => panic!("invalid encoded MaskClip"),
        }
    }
}

impl ExtraDataClone<'_> for MaskClip {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

impl ExtraDataCompact<'_> for MaskComposite {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Add => 0,
            Self::Subtract => 1,
            Self::Intersect => 2,
            Self::Exclude => 3,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Add,
            1 => Self::Subtract,
            2 => Self::Intersect,
            3 => Self::Exclude,
            _ => panic!("invalid encoded MaskComposite"),
        }
    }
}

impl ExtraDataClone<'_> for MaskComposite {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WebKitMaskComposite {
    Clear,
    Copy,
    SourceOver,
    SourceIn,
    SourceOut,
    SourceAtop,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    DestinationAtop,
    Xor,
}

impl ExtraDataCompact<'_> for WebKitMaskComposite {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Clear => 0,
            Self::Copy => 1,
            Self::SourceOver => 2,
            Self::SourceIn => 3,
            Self::SourceOut => 4,
            Self::SourceAtop => 5,
            Self::DestinationOver => 6,
            Self::DestinationIn => 7,
            Self::DestinationOut => 8,
            Self::DestinationAtop => 9,
            Self::Xor => 10,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Clear,
            1 => Self::Copy,
            2 => Self::SourceOver,
            3 => Self::SourceIn,
            4 => Self::SourceOut,
            5 => Self::SourceAtop,
            6 => Self::DestinationOver,
            7 => Self::DestinationIn,
            8 => Self::DestinationOut,
            9 => Self::DestinationAtop,
            10 => Self::Xor,
            _ => panic!("invalid encoded WebKitMaskComposite"),
        }
    }
}

impl ExtraDataClone<'_> for WebKitMaskComposite {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}

impl ExtraDataCompact<'_> for WebKitMaskSourceType {
    fn encode_extra(self, _context: &mut AstContext<'_>) -> ExtraData {
        ExtraData::from_u64(match self {
            Self::Auto => 0,
            Self::Luminance => 1,
            Self::Alpha => 2,
        })
    }

    fn decode_extra(data: ExtraData, _context: &AstContext<'_>) -> Self {
        match data.as_u64() {
            0 => Self::Auto,
            1 => Self::Luminance,
            2 => Self::Alpha,
            _ => panic!("invalid encoded WebKitMaskSourceType"),
        }
    }
}

impl ExtraDataClone<'_> for WebKitMaskSourceType {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskType {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

#[derive(CssKeyword, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}

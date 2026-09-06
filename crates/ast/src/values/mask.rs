use crate::*;

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum MaskMode {
    Luminance,
    Alpha,
    MatchSource,
}

impl_inline_extra!(MaskMode);

impl ExtraDataClone<'_> for MaskMode {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Visit, Clone, Copy)]
pub enum MaskClip {
    GeometryBox(GeometryBox),
    NoClip,
}

impl_inline_extra!(MaskClip);

impl ExtraDataClone<'_> for MaskClip {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum MaskComposite {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

impl_inline_extra!(MaskComposite);

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

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Visit)]
pub enum MaskBorderMode {
    Luminance,
    Alpha,
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
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

impl_inline_extra!(WebKitMaskComposite);

impl ExtraDataClone<'_> for WebKitMaskComposite {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

#[derive(CssKeyword, Debug, PartialEq, Visit, Clone, Copy)]
pub enum WebKitMaskSourceType {
    Auto,
    Luminance,
    Alpha,
}

impl_inline_extra!(WebKitMaskSourceType);

impl ExtraDataClone<'_> for WebKitMaskSourceType {
    fn clone_extra(self, _context: &mut AstContext<'_>) -> Self {
        self
    }
}

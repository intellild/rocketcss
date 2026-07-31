use crate::*;
#[derive(Debug, PartialEq, Visit)]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

#[derive(Debug, PartialEq, Visit)]
pub enum KeyframesName<'a> {
    Ident(Atom<'a>),
    Custom(Atom<'a>),
}

#[derive(Debug, PartialEq, Visit)]
pub struct KeyframesRule<'a> {
    pub keyframes: std::vec::Vec<Keyframe>,
    pub span: Span,
    pub name: std::boxed::Box<KeyframesName<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Keyframe {
    #[visit(with = visit_declaration_block_id, with_mut = visit_declaration_block_id_mut)]
    pub declarations: DeclarationBlockId,
    pub selectors: std::vec::Vec<KeyframeSelector>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}

#[ast]
pub enum KeyframeSelector {
    Percentage(f32),
    From,
    To,
    TimelineRangePercentage(TimelineRangePercentage),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyframesName<'a> {
    Ident(&'a str),
    Custom(&'a str),
}

#[ast]
pub struct TimelineRangePercentage {
    pub name: TimelineRangeName,
    pub percentage: f32,
}

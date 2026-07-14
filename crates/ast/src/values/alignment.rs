use crate::*;

#[derive(Debug, PartialEq, Visit)]
pub enum AlignContent {
    Normal,
    BaselinePosition(BaselinePosition),
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum BaselinePosition {
    First,
    Last,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContentDistribution {
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum OverflowPosition {
    Safe,
    Unsafe,
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum ContentPosition {
    Center,
    Start,
    End,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifyContent {
    Normal,
    ContentDistribution(ContentDistribution),
    ContentPosition {
        overflow: Option<OverflowPosition>,
        value: ContentPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum AlignSelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum SelfPosition {
    Center,
    Start,
    End,
    SelfStart,
    SelfEnd,
    FlexStart,
    FlexEnd,
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifySelf {
    Auto,
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum AlignItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub enum JustifyItems {
    Normal,
    Stretch,
    BaselinePosition(BaselinePosition),
    SelfPosition {
        overflow: Option<OverflowPosition>,
        value: SelfPosition,
    },
    Left {
        overflow: Option<OverflowPosition>,
    },
    Right {
        overflow: Option<OverflowPosition>,
    },
    Legacy(LegacyJustify),
}

#[derive(CssKeyword, Debug, PartialEq, Visit)]
pub enum LegacyJustify {
    Left,
    Right,
    Center,
}

#[derive(Debug, PartialEq, Visit)]
pub enum GapValue<'a> {
    Normal,
    LengthPercentage(Box<'a, LengthPercentage<'a>>),
}

use super::image::is_non_negative_length_percentage;
use crate::prelude::*;

keyword_parse!(BaselinePosition, "first" => Self::First, "last" => Self::Last,);
keyword_parse!(
    ContentDistribution,
    "space-between" => Self::SpaceBetween,
    "space-around" => Self::SpaceAround,
    "space-evenly" => Self::SpaceEvenly,
    "stretch" => Self::Stretch,
);
keyword_parse!(OverflowPosition, "safe" => Self::Safe, "unsafe" => Self::Unsafe,);
keyword_parse!(
    ContentPosition,
    "center" => Self::Center,
    "start" => Self::Start,
    "end" => Self::End,
    "flex-start" => Self::FlexStart,
    "flex-end" => Self::FlexEnd,
);
keyword_parse!(
    SelfPosition,
    "center" => Self::Center,
    "start" => Self::Start,
    "end" => Self::End,
    "self-start" => Self::SelfStart,
    "self-end" => Self::SelfEnd,
    "flex-start" => Self::FlexStart,
    "flex-end" => Self::FlexEnd,
);
keyword_parse!(
    LegacyJustify,
    "left" => Self::Left,
    "right" => Self::Right,
    "center" => Self::Center,
);

pub(in crate::parser) fn parse_align_content_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<AlignContent, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("normal") {
        return Ok(AlignContent::Normal);
    }
    if ident.eq_ignore_ascii_case("first") || ident.eq_ignore_ascii_case("last") {
        let baseline = if ident.eq_ignore_ascii_case("first") {
            BaselinePosition::First
        } else {
            BaselinePosition::Last
        };
        input.expect_ident_matching("baseline")?;
        return Ok(AlignContent::BaselinePosition(baseline));
    }
    if let Some(distribution) = match_ignore_ascii_case!(
        ident,
        "space-between" => Some(ContentDistribution::SpaceBetween),
        "space-around" => Some(ContentDistribution::SpaceAround),
        "space-evenly" => Some(ContentDistribution::SpaceEvenly),
        "stretch" => Some(ContentDistribution::Stretch),
        _ => None,
    ) {
        return Ok(AlignContent::ContentDistribution(distribution));
    }
    let overflow = if ident.eq_ignore_ascii_case("safe") {
        Some(OverflowPosition::Safe)
    } else if ident.eq_ignore_ascii_case("unsafe") {
        Some(OverflowPosition::Unsafe)
    } else {
        None
    };
    let value = if overflow.is_some() {
        ContentPosition::parse(input)?
    } else {
        match_ignore_ascii_case!(
            ident,
            "center" => ContentPosition::Center,
            "start" => ContentPosition::Start,
            "end" => ContentPosition::End,
            "flex-start" => ContentPosition::FlexStart,
            "flex-end" => ContentPosition::FlexEnd,
            _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    };
    Ok(AlignContent::ContentPosition { overflow, value })
}

impl<'i> Parse<'i> for AlignContent {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_align_content_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_justify_content_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<JustifyContent, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("normal") {
        return Ok(JustifyContent::Normal);
    }
    if let Some(distribution) = match_ignore_ascii_case!(
        ident,
        "space-between" => Some(ContentDistribution::SpaceBetween),
        "space-around" => Some(ContentDistribution::SpaceAround),
        "space-evenly" => Some(ContentDistribution::SpaceEvenly),
        "stretch" => Some(ContentDistribution::Stretch),
        _ => None,
    ) {
        return Ok(JustifyContent::ContentDistribution(distribution));
    }
    let overflow = if ident.eq_ignore_ascii_case("safe") {
        Some(OverflowPosition::Safe)
    } else if ident.eq_ignore_ascii_case("unsafe") {
        Some(OverflowPosition::Unsafe)
    } else {
        None
    };
    if overflow.is_some() {
        let value = ContentPosition::parse(input)?;
        return Ok(JustifyContent::ContentPosition { overflow, value });
    }
    match_ignore_ascii_case!(
        ident,
        "center" => Ok(JustifyContent::ContentPosition { overflow, value: ContentPosition::Center }),
        "start" => Ok(JustifyContent::ContentPosition { overflow, value: ContentPosition::Start }),
        "end" => Ok(JustifyContent::ContentPosition { overflow, value: ContentPosition::End }),
        "flex-start" => Ok(JustifyContent::ContentPosition { overflow, value: ContentPosition::FlexStart }),
        "flex-end" => Ok(JustifyContent::ContentPosition { overflow, value: ContentPosition::FlexEnd }),
        "left" => Ok(JustifyContent::Left { overflow }),
        "right" => Ok(JustifyContent::Right { overflow }),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

impl<'i> Parse<'i> for JustifyContent {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_justify_content_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_align_self_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<AlignSelf, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("auto") {
        return Ok(AlignSelf::Auto);
    }
    if ident.eq_ignore_ascii_case("normal") {
        return Ok(AlignSelf::Normal);
    }
    if ident.eq_ignore_ascii_case("stretch") {
        return Ok(AlignSelf::Stretch);
    }
    if ident.eq_ignore_ascii_case("first") || ident.eq_ignore_ascii_case("last") {
        let baseline = if ident.eq_ignore_ascii_case("first") {
            BaselinePosition::First
        } else {
            BaselinePosition::Last
        };
        input.expect_ident_matching("baseline")?;
        return Ok(AlignSelf::BaselinePosition(baseline));
    }
    let overflow = if ident.eq_ignore_ascii_case("safe") {
        Some(OverflowPosition::Safe)
    } else if ident.eq_ignore_ascii_case("unsafe") {
        Some(OverflowPosition::Unsafe)
    } else {
        None
    };
    let value = if overflow.is_some() {
        SelfPosition::parse(input)?
    } else {
        match_ignore_ascii_case!(
            ident,
            "center" => SelfPosition::Center,
            "start" => SelfPosition::Start,
            "end" => SelfPosition::End,
            "self-start" => SelfPosition::SelfStart,
            "self-end" => SelfPosition::SelfEnd,
            "flex-start" => SelfPosition::FlexStart,
            "flex-end" => SelfPosition::FlexEnd,
            _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    };
    Ok(AlignSelf::SelfPosition { overflow, value })
}

impl<'i> Parse<'i> for AlignSelf {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_align_self_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_align_items_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<AlignItems, ParseError<'i, ParserError<'i>>> {
    let value = parse_align_self_value(input)?;
    let value = match value {
        AlignSelf::Normal => AlignItems::Normal,
        AlignSelf::Stretch => AlignItems::Stretch,
        AlignSelf::BaselinePosition(value) => AlignItems::BaselinePosition(value),
        AlignSelf::SelfPosition { overflow, value } => AlignItems::SelfPosition { overflow, value },
        AlignSelf::Auto => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };
    Ok(value)
}

impl<'i> Parse<'i> for AlignItems {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_align_items_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_justify_self_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<JustifySelf, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("auto") {
        return Ok(JustifySelf::Auto);
    }
    if ident.eq_ignore_ascii_case("normal") {
        return Ok(JustifySelf::Normal);
    }
    if ident.eq_ignore_ascii_case("stretch") {
        return Ok(JustifySelf::Stretch);
    }
    if ident.eq_ignore_ascii_case("first") || ident.eq_ignore_ascii_case("last") {
        let baseline = if ident.eq_ignore_ascii_case("first") {
            BaselinePosition::First
        } else {
            BaselinePosition::Last
        };
        input.expect_ident_matching("baseline")?;
        return Ok(JustifySelf::BaselinePosition(baseline));
    }
    let overflow = if ident.eq_ignore_ascii_case("safe") {
        Some(OverflowPosition::Safe)
    } else if ident.eq_ignore_ascii_case("unsafe") {
        Some(OverflowPosition::Unsafe)
    } else {
        None
    };
    if overflow.is_some() {
        return Ok(JustifySelf::SelfPosition {
            overflow,
            value: SelfPosition::parse(input)?,
        });
    }
    match_ignore_ascii_case!(
        ident,
        "center" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::Center }),
        "start" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::Start }),
        "end" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::End }),
        "self-start" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::SelfStart }),
        "self-end" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::SelfEnd }),
        "flex-start" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::FlexStart }),
        "flex-end" => Ok(JustifySelf::SelfPosition { overflow, value: SelfPosition::FlexEnd }),
        "left" => Ok(JustifySelf::Left { overflow }),
        "right" => Ok(JustifySelf::Right { overflow }),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

impl<'i> Parse<'i> for JustifySelf {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_justify_self_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_justify_items_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<JustifyItems, ParseError<'i, ParserError<'i>>> {
    if input
        .try_parse(|input| input.expect_ident_matching("legacy"))
        .is_ok()
    {
        let value = LegacyJustify::parse(input)?;
        return Ok(JustifyItems::Legacy(value));
    }

    let ident = input.expect_ident()?;
    let overflow = if ident.eq_ignore_ascii_case("safe") {
        Some(OverflowPosition::Safe)
    } else if ident.eq_ignore_ascii_case("unsafe") {
        Some(OverflowPosition::Unsafe)
    } else {
        None
    };
    let value = if overflow.is_some() {
        SelfPosition::parse(input)?
    } else {
        match_ignore_ascii_case!(
            ident,
            "normal" => return Ok(JustifyItems::Normal),
            "stretch" => return Ok(JustifyItems::Stretch),
            "center" => SelfPosition::Center,
            "start" => SelfPosition::Start,
            "end" => SelfPosition::End,
            "self-start" => SelfPosition::SelfStart,
            "self-end" => SelfPosition::SelfEnd,
            "flex-start" => SelfPosition::FlexStart,
            "flex-end" => SelfPosition::FlexEnd,
            "left" => return Ok(JustifyItems::Left { overflow }),
            "right" => return Ok(JustifyItems::Right { overflow }),
            _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    };
    Ok(JustifyItems::SelfPosition { overflow, value })
}

impl<'i> Parse<'i> for JustifyItems {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_justify_items_value(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

impl<'i> Parse<'i> for GapValue<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(Self::Normal);
        }
        let value = LengthPercentage::parse(input)?;
        if !is_non_negative_length_percentage(&value) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::LengthPercentage(store_node(value, input)))
    }
}

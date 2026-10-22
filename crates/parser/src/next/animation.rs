//! Animation and transition parsing into the flattened AST.

use super::prelude::*;

impl<'i> Parse<'i> for Time {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        match input.next()?.clone() {
            OldToken::Dimension {
                unit: OldUnit::Seconds,
                value,
            } => Ok(Self::Seconds(value)),
            OldToken::Dimension {
                unit: OldUnit::Milliseconds,
                value,
            } => Ok(Self::Milliseconds(value)),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for Transition<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut property = None;
        let mut duration = None;
        let mut timing_function = None;
        let mut delay = None;

        while !input.is_exhausted() {
            if duration.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                duration = Some(value);
                continue;
            }
            if timing_function.is_none()
                && let Ok(value) = input.try_parse(EasingFunction::parse)
            {
                timing_function = Some(value);
                continue;
            }
            if delay.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                delay = Some(value);
                continue;
            }
            if property.is_none()
                && let Ok(name) = input.try_parse(Compiler::expect_ident)
            {
                property = Some(PropertyId::from_name(name));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        Ok(input.ast_mut().alloc_transition(
            TransitionData {
                delay: delay.unwrap_or(Time::Seconds(0.0)),
                duration: duration.unwrap_or(Time::Seconds(0.0)),
                property: property.unwrap_or(PropertyId::All),
                timing_function: timing_function.unwrap_or(EasingFunction::Ease),
            },
            DUMMY_SP,
        ))
    }
}

pub(crate) fn parse_transition_property_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<&'i [PropertyId<'i>], ParseError<'i, ParserError<'i>>> {
    let mut values = Vec::new();
    loop {
        let name = input.expect_ident()?;
        values.push(PropertyId::from_name(name));
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(input.ast().alloc_slice(&values))
}

impl<'i> Parse<'i> for StepPosition {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "start" | "jump-start" => Ok(Self::Start),
            "end" | "jump-end" => Ok(Self::End),
            "jump-none" => Ok(Self::JumpNone),
            "jump-both" => Ok(Self::JumpBoth),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for EasingFunction {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            return match_ignore_ascii_case!(
                ident,
                "linear" => Ok(Self::Linear),
                "ease" => Ok(Self::Ease),
                "ease-in" => Ok(Self::EaseIn),
                "ease-out" => Ok(Self::EaseOut),
                "ease-in-out" => Ok(Self::EaseInOut),
                "step-start" => Ok(Self::Steps {
                    count: 1,
                    position: StepPosition::Start,
                }),
                "step-end" => Ok(Self::Steps {
                    count: 1,
                    position: StepPosition::End,
                }),
                _ => Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }
        let location = input.current_source_location();
        let function = input.expect_function()?;
        match_ignore_ascii_case!(
            function,
            "cubic-bezier" => input.parse_nested_block(|input| {
                let x1 = input.expect_number()?;
                input.expect_comma()?;
                let y1 = input.expect_number()?;
                input.expect_comma()?;
                let x2 = input.expect_number()?;
                input.expect_comma()?;
                let y2 = input.expect_number()?;
                Ok(Self::CubicBezier { x1, x2, y1, y2 })
            }),
            "frames" => input.parse_nested_block(|input| {
                let count = input.expect_integer()?;
                input.expect_exhausted()?;
                if count <= 0 {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok(Self::Frames(count))
            }),
            "steps" => input.parse_nested_block(|input| {
                let count = input.expect_integer()?;
                let position = input
                    .try_parse(|input| {
                        input.expect_comma()?;
                        StepPosition::parse(input)
                    })
                    .unwrap_or(StepPosition::End);
                Ok(Self::Steps { count, position })
            }),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for AnimationIterationCount {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("infinite"))
            .is_ok()
        {
            return Ok(Self::Infinite);
        }
        Ok(Self::Number(input.expect_number()?))
    }
}

impl<'i> Parse<'i> for AnimationDirection {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "normal" => Ok(Self::Normal),
            "reverse" => Ok(Self::Reverse),
            "alternate" => Ok(Self::Alternate),
            "alternate-reverse" => Ok(Self::AlternateReverse),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for AnimationFillMode {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "none" => Ok(Self::None),
            "forwards" => Ok(Self::Forwards),
            "backwards" => Ok(Self::Backwards),
            "both" => Ok(Self::Both),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for AnimationPlayState {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "running" => Ok(Self::Running),
            "paused" => Ok(Self::Paused),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

fn is_css_wide_keyword(value: &str) -> bool {
    match_ignore_ascii_case!(
        value,
        "initial" | "inherit" | "unset" | "default" | "revert" | "revert-layer" => true,
        _ => false,
    )
}

impl<'i> Parse<'i> for AnimationName<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            if ident.eq_ignore_ascii_case("none") {
                return Ok(Self::None);
            }
            if is_css_wide_keyword(ident) || ident.eq_ignore_ascii_case("default") {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            return Ok(Self::Ident(ident));
        }
        Ok(Self::String(input.expect_string()?))
    }
}

impl<'i> Parse<'i> for Animation<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        let mut components = Vec::new();
        let mut duration_claimed = false;
        let mut timing_function_claimed = false;
        let mut delay_claimed = false;
        let mut iteration_count_claimed = false;
        let mut direction_claimed = false;
        let mut fill_mode_claimed = false;
        let mut play_state_claimed = false;
        let mut name_claimed = false;

        while !input.is_exhausted() {
            if !duration_claimed && let Ok(value) = input.try_parse(Time::parse) {
                duration_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::Duration(value),
                    span,
                ));
                continue;
            }
            if !timing_function_claimed && let Ok(value) = input.try_parse(EasingFunction::parse) {
                timing_function_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::TimingFunction(value),
                    span,
                ));
                continue;
            }
            if !delay_claimed && let Ok(value) = input.try_parse(Time::parse) {
                delay_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::Delay(value),
                    span,
                ));
                continue;
            }
            if !iteration_count_claimed
                && let Ok(value) = input.try_parse(AnimationIterationCount::parse)
            {
                iteration_count_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::IterationCount(value),
                    span,
                ));
                continue;
            }
            if !direction_claimed && let Ok(value) = input.try_parse(AnimationDirection::parse) {
                direction_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::Direction(value),
                    span,
                ));
                continue;
            }
            if !fill_mode_claimed && let Ok(value) = input.try_parse(AnimationFillMode::parse) {
                fill_mode_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::FillMode(value),
                    span,
                ));
                continue;
            }
            if !play_state_claimed && let Ok(value) = input.try_parse(AnimationPlayState::parse) {
                play_state_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::PlayState(value),
                    span,
                ));
                continue;
            }
            if !name_claimed && let Ok(value) = input.try_parse(AnimationName::parse) {
                name_claimed = true;
                components.push(input.ast_mut().alloc_animation_component(
                    AnimationComponentData::Name(value),
                    span,
                ));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        if components.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let components = input.ast().alloc_slice(&components);
        Ok(input
            .ast_mut()
            .alloc_animation(AnimationData { components }, span))
    }
}

pub(crate) fn parse_animation_list<'i>(
    input: &mut Compiler<'i>,
) -> Result<&'i [Animation<'i>], ParseError<'i, ParserError<'i>>> {
    let mut values = Vec::new();
    loop {
        values.push(input.parse_until_before(Delimiter::Comma, Animation::parse)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(input.ast().alloc_slice(&values))
}

#[allow(dead_code)]
pub(crate) fn parse_comma_separated<'i, T: Copy>(
    input: &mut Compiler<'i>,
    mut parser: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
) -> Result<&'i [T], ParseError<'i, ParserError<'i>>> {
    let mut values = Vec::new();
    loop {
        values.push(parser(input)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    Ok(input.ast().alloc_slice(&values))
}

pub(crate) fn value_contains_comment<'i>(input: &mut Compiler<'i>) -> bool {
    let start = input.state();
    let contains = input
        .parse_until_before(Delimiter::Bang | Delimiter::Semicolon, scan_comment)
        .unwrap_or(false);
    input.reset(&start);
    contains
}

fn scan_comment<'i>(input: &mut Compiler<'i>) -> Result<bool, ParseError<'i, ParserError<'i>>> {
    let mut found = false;
    loop {
        let token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(_) => return Ok(found),
        };
        match token {
            OldToken::Comment(_) => found = true,
            OldToken::Function(_)
            | OldToken::ParenthesisBlock
            | OldToken::SquareBracketBlock
            | OldToken::CurlyBracketBlock => {
                found |= input.parse_nested_block(scan_comment)?;
            }
            _ => {}
        }
    }
}

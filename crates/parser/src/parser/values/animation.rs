use super::css_wide_keyword;
use crate::prelude::*;

impl<'i> Parse<'i> for Time {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let location = input.current_source_location();
        match input.next()?.clone() {
            ValueToken::Dimension {
                unit: Unit::Seconds,
                value,
            } => Ok(Self::Seconds(value)),
            ValueToken::Dimension {
                unit: Unit::Milliseconds,
                value,
            } => Ok(Self::Milliseconds(value)),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for StepPosition {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            // `jump-start`/`jump-end` canonicalize to `start`/`end` at parse
            // time, mirroring lightningcss.
            "start" | "jump-start" => Ok(Self::Start),
            "end" | "jump-end" => Ok(Self::End),
            "jump-none" => Ok(Self::JumpNone),
            "jump-both" => Ok(Self::JumpBoth),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for EasingFunction {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Parser::expect_ident) {
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
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "running" => Ok(Self::Running),
            "paused" => Ok(Self::Paused),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for AnimationName<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Parser::expect_ident) {
            if ident.eq_ignore_ascii_case("none") {
                return Ok(Self::None);
            }
            // Custom idents exclude CSS-wide keywords and `default`.
            if css_wide_keyword(ident).is_some() || ident.eq_ignore_ascii_case("default") {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            return Ok(Self::Ident(ident));
        }
        Ok(Self::String(input.expect_string()?))
    }
}

impl<'i> Parse<'i> for Animation<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let mut duration = None;
        let mut timing_function = None;
        let mut delay = None;
        let mut iteration_count = None;
        let mut direction = None;
        let mut fill_mode = None;
        let mut play_state = None;
        let mut name = None;

        // Component classes are claimed in a fixed order with the keyframes
        // name as the last resort, mirroring lightningcss and stylo. The first
        // <time> is the duration and the second the delay; a keyword whose
        // class is already claimed (e.g. `ease 1s linear`) falls through to
        // the name. animation-timeline is never parsed from the shorthand.
        while !input.is_exhausted() {
            if duration.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                duration = Some(allocator.boxed(value));
                continue;
            }
            if timing_function.is_none()
                && let Ok(value) = input.try_parse(EasingFunction::parse)
            {
                timing_function = Some(allocator.boxed(value));
                continue;
            }
            if delay.is_none()
                && let Ok(value) = input.try_parse(Time::parse)
            {
                delay = Some(allocator.boxed(value));
                continue;
            }
            if iteration_count.is_none()
                && let Ok(value) = input.try_parse(AnimationIterationCount::parse)
            {
                iteration_count = Some(allocator.boxed(value));
                continue;
            }
            if direction.is_none()
                && let Ok(value) = input.try_parse(AnimationDirection::parse)
            {
                direction = Some(value);
                continue;
            }
            if fill_mode.is_none()
                && let Ok(value) = input.try_parse(AnimationFillMode::parse)
            {
                fill_mode = Some(value);
                continue;
            }
            if play_state.is_none()
                && let Ok(value) = input.try_parse(AnimationPlayState::parse)
            {
                play_state = Some(value);
                continue;
            }
            if name.is_none()
                && let Ok(value) = input.try_parse(AnimationName::parse)
            {
                name = Some(allocator.boxed(value));
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        if duration.is_none()
            && timing_function.is_none()
            && delay.is_none()
            && iteration_count.is_none()
            && direction.is_none()
            && fill_mode.is_none()
            && play_state.is_none()
            && name.is_none()
        {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            delay,
            direction,
            duration,
            fill_mode,
            iteration_count,
            name,
            play_state,
            timeline: None,
            timing_function,
        })
    }
}

pub(crate) fn parse_animation_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<'i, Animation<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut values = allocator.vec();
    loop {
        values.push(input.parse_until_before(Delimiter::Comma, Animation::parse)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    Ok(values)
}

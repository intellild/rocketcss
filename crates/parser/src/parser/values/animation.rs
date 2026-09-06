use crate::prelude::*;

impl<'i> Parse<'i> for StepPosition {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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

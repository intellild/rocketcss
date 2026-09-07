use super::*;

pub(in crate::parser) fn parse_keyframes_name<'i>(
    prelude: &'i str,
    input: &mut Compiler<'i>,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
        let name = match *parser.next()? {
            ValueToken::Ident(name)
                if !matches_ignore_case(
                    name,
                    &[
                        "none",
                        "initial",
                        "inherit",
                        "unset",
                        "default",
                        "revert",
                        "revert-layer",
                    ],
                ) =>
            {
                KeyframesName::Ident(parser.add_str(name))
            }
            ValueToken::String(name) => KeyframesName::Custom(parser.add_str(name)),
            _ => return Err(parser.new_custom_error(ParserError::InvalidValue)),
        };
        parser.expect_exhausted()?;
        Ok(name)
    })
}

pub(in crate::parser) fn parse_keyframe_selector<'i>(
    input: &mut Compiler<'i>,
) -> Result<KeyframeSelector, ParseError<'i, ParserError<'i>>> {
    match input.next()? {
        ValueToken::Percentage(value) if (0.0..=1.0).contains(value) => {
            Ok(KeyframeSelector::Percentage(*value))
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("from") => Ok(KeyframeSelector::From),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("to") => Ok(KeyframeSelector::To),
        ValueToken::Ident(name) => {
            let name = match_ignore_ascii_case!(
                name,
                "cover" => TimelineRangeName::Cover,
                "contain" => TimelineRangeName::Contain,
                "entry" => TimelineRangeName::Entry,
                "exit" => TimelineRangeName::Exit,
                "entry-crossing" => TimelineRangeName::EntryCrossing,
                "exit-crossing" => TimelineRangeName::ExitCrossing,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
            let percentage = input.expect_percentage()?;
            Ok(KeyframeSelector::TimelineRangePercentage(
                TimelineRangePercentage { name, percentage },
            ))
        }
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

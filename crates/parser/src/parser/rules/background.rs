use crate::parser::values::image::{parse_position_components, zero_position};
use crate::prelude::*;

impl<'i> Parse<'i> for Background<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let color = parse_css_color(input)?;
        let x = zero_position(input);
        let y = zero_position(input);
        let position = store_node(BackgroundPosition { x, y }, input);
        let height = store_node(LengthPercentageOrAuto::Auto, input);
        let width = store_node(LengthPercentageOrAuto::Auto, input);
        let size = store_node(BackgroundSize::Explicit { height, width }, input);
        Ok(Self {
            attachment: BackgroundAttachment::Scroll,
            clip: BackgroundClip::BorderBox,
            color,
            image: store_node(Image::None, input),
            origin: BackgroundOrigin::PaddingBox,
            position,
            repeat: BackgroundRepeat {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
            size,
        })
    }
}

impl<'i> Parse<'i> for BackgroundRepeat {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_background_repeat(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(in crate::parser) fn parse_background_repeat<'i>(
    input: &mut Compiler<'i>,
) -> Result<BackgroundRepeat, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("repeat-x") {
        return Ok(BackgroundRepeat {
            x: BackgroundRepeatKeyword::Repeat,
            y: BackgroundRepeatKeyword::NoRepeat,
        });
    }
    if ident.eq_ignore_ascii_case("repeat-y") {
        return Ok(BackgroundRepeat {
            x: BackgroundRepeatKeyword::NoRepeat,
            y: BackgroundRepeatKeyword::Repeat,
        });
    }
    let x = match_ignore_ascii_case!(
        ident,
        "repeat" => Ok(BackgroundRepeatKeyword::Repeat),
        "space" => Ok(BackgroundRepeatKeyword::Space),
        "round" => Ok(BackgroundRepeatKeyword::Round),
        "no-repeat" => Ok(BackgroundRepeatKeyword::NoRepeat),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )?;
    let y = input
        .try_parse(BackgroundRepeatKeyword::parse)
        .unwrap_or_else(|_| clone_repeat_keyword(&x));
    Ok(BackgroundRepeat { x, y })
}

fn clone_repeat_keyword(value: &BackgroundRepeatKeyword) -> BackgroundRepeatKeyword {
    match value {
        BackgroundRepeatKeyword::Repeat => BackgroundRepeatKeyword::Repeat,
        BackgroundRepeatKeyword::Space => BackgroundRepeatKeyword::Space,
        BackgroundRepeatKeyword::Round => BackgroundRepeatKeyword::Round,
        BackgroundRepeatKeyword::NoRepeat => BackgroundRepeatKeyword::NoRepeat,
    }
}

impl<'i> Parse<'i> for Position<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let (x, y) = parse_position_components(input)?;
        input.expect_exhausted()?;
        Ok(Self { x, y })
    }
}

impl<'i> Parse<'i> for BackgroundPosition<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let (x, y) = parse_position_components(input)?;
        input.expect_exhausted()?;
        Ok(Self { x, y })
    }
}

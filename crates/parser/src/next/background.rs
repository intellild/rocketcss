//! Background and position parsing into the flattened AST.

use super::prelude::*;

type PositionComponents<'i> = (
    PositionComponent<'i, HorizontalPositionKeyword>,
    PositionComponent<'i, VerticalPositionKeyword>,
);

fn zero_position<'i, S: Copy>(ast: &mut Ast<'i>) -> PositionComponent<'i, S> {
    let value = ast.alloc_dimension_percentage(
        DimensionPercentageData::<LengthValue>::Percentage(0.0),
        DUMMY_SP,
    );
    ast.alloc_position_component(PositionComponentData::Length(value), DUMMY_SP)
}

impl<'i> Parse<'i> for BackgroundRepeatKeyword {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "repeat" => Ok(Self::Repeat),
            "space" => Ok(Self::Space),
            "round" => Ok(Self::Round),
            "no-repeat" => Ok(Self::NoRepeat),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BackgroundAttachment {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "scroll" => Ok(Self::Scroll),
            "fixed" => Ok(Self::Fixed),
            "local" => Ok(Self::Local),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BackgroundClip {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "border-box" => Ok(Self::BorderBox),
            "padding-box" => Ok(Self::PaddingBox),
            "content-box" => Ok(Self::ContentBox),
            "border" => Ok(Self::Border),
            "text" => Ok(Self::Text),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BackgroundOrigin {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "border-box" => Ok(Self::BorderBox),
            "padding-box" => Ok(Self::PaddingBox),
            "content-box" => Ok(Self::ContentBox),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for Background<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
        let color = CssColor::parse(input)?;
        let image = input.ast_mut().alloc_image(ImageData::None, span);
        let x = zero_position::<HorizontalPositionKeyword>(input.ast_mut());
        let y = zero_position::<VerticalPositionKeyword>(input.ast_mut());
        let position = input
            .ast_mut()
            .alloc_background_position(BackgroundPositionData { x, y }, span);
        let repeat = input.ast_mut().alloc_background_repeat(
            BackgroundRepeatData {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
            span,
        );
        let height = input
            .ast_mut()
            .alloc_length_percentage_or_auto(LengthPercentageOrAutoData::Auto, span);
        let width = input
            .ast_mut()
            .alloc_length_percentage_or_auto(LengthPercentageOrAutoData::Auto, span);
        let size = input
            .ast_mut()
            .alloc_background_size(BackgroundSizeData::Explicit { height, width }, span);
        Ok(input.ast_mut().alloc_background(
            BackgroundData {
                attachment: BackgroundAttachment::Scroll,
                clip: BackgroundClip::BorderBox,
                color,
                image,
                origin: BackgroundOrigin::PaddingBox,
                position,
                repeat,
                size,
            },
            span,
        ))
    }
}

impl<'i> Parse<'i> for BackgroundRepeat<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_background_repeat(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(super) fn parse_background_repeat<'i>(
    input: &mut Compiler<'i>,
) -> Result<BackgroundRepeat<'i>, ParseError<'i, ParserError<'i>>> {
    let span = token_span(input);
    let ident = input.expect_ident()?;
    if ident.eq_ignore_ascii_case("repeat-x") {
        return Ok(input.ast_mut().alloc_background_repeat(
            BackgroundRepeatData {
                x: BackgroundRepeatKeyword::Repeat,
                y: BackgroundRepeatKeyword::NoRepeat,
            },
            span,
        ));
    }
    if ident.eq_ignore_ascii_case("repeat-y") {
        return Ok(input.ast_mut().alloc_background_repeat(
            BackgroundRepeatData {
                x: BackgroundRepeatKeyword::NoRepeat,
                y: BackgroundRepeatKeyword::Repeat,
            },
            span,
        ));
    }
    let x = match_ignore_ascii_case!(
        ident,
        "repeat" => Ok(BackgroundRepeatKeyword::Repeat),
        "space" => Ok(BackgroundRepeatKeyword::Space),
        "round" => Ok(BackgroundRepeatKeyword::Round),
        "no-repeat" => Ok(BackgroundRepeatKeyword::NoRepeat),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )?;
    let y = input.try_parse(BackgroundRepeatKeyword::parse).unwrap_or(x);
    Ok(input
        .ast_mut()
        .alloc_background_repeat(BackgroundRepeatData { x, y }, span))
}

impl<'i> Parse<'i> for BackgroundSize<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_background_size(input)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

pub(super) fn parse_background_size<'i>(
    input: &mut Compiler<'i>,
) -> Result<BackgroundSize<'i>, ParseError<'i, ParserError<'i>>> {
    let span = token_span(input);
    if input
        .try_parse(|input| input.expect_ident_matching("cover"))
        .is_ok()
    {
        return Ok(input
            .ast_mut()
            .alloc_background_size(BackgroundSizeData::Cover, span));
    }
    if input
        .try_parse(|input| input.expect_ident_matching("contain"))
        .is_ok()
    {
        return Ok(input
            .ast_mut()
            .alloc_background_size(BackgroundSizeData::Contain, span));
    }

    let width = LengthPercentageOrAuto::parse(input)?;
    let height = input
        .try_parse(LengthPercentageOrAuto::parse)
        .unwrap_or_else(|_| {
            input
                .ast_mut()
                .alloc_length_percentage_or_auto(LengthPercentageOrAutoData::Auto, span)
        });
    Ok(input
        .ast_mut()
        .alloc_background_size(BackgroundSizeData::Explicit { height, width }, span))
}

impl<'i> Parse<'i> for Position<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let (x, y) = parse_position_components(input)?;
        input.expect_exhausted()?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_position(PositionData { x, y }, span)))
    }
}

impl<'i> Parse<'i> for BackgroundPosition<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let (x, y) = parse_position_components(input)?;
        input.expect_exhausted()?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_background_position(
                BackgroundPositionData { x, y },
                span,
            )))
    }
}

pub(super) fn parse_position_components<'i>(
    input: &mut Compiler<'i>,
) -> Result<PositionComponents<'i>, ParseError<'i, ParserError<'i>>> {
    if let Ok(components) = input.try_parse(parse_horizontal_first_position) {
        return Ok(components);
    }
    parse_vertical_first_position(input)
}

fn parse_horizontal_first_position<'i>(
    input: &mut Compiler<'i>,
) -> Result<PositionComponents<'i>, ParseError<'i, ParserError<'i>>> {
    let span = token_span(input);
    let x = parse_position_component::<HorizontalPositionKeyword>(
        input,
        parse_horizontal_position_keyword,
    )?;
    let y = if input.is_exhausted() {
        input
            .ast_mut()
            .alloc_position_component(PositionComponentData::Center, span)
    } else {
        input
            .try_parse(|input| {
                parse_position_component::<VerticalPositionKeyword>(
                    input,
                    parse_vertical_position_keyword,
                )
            })
            .unwrap_or_else(|_| {
                input
                    .ast_mut()
                    .alloc_position_component(PositionComponentData::Center, span)
            })
    };
    Ok((x, y))
}

fn parse_vertical_first_position<'i>(
    input: &mut Compiler<'i>,
) -> Result<PositionComponents<'i>, ParseError<'i, ParserError<'i>>> {
    let span = token_span(input);
    let y = parse_position_component::<VerticalPositionKeyword>(
        input,
        parse_vertical_position_keyword,
    )?;
    let x = if input.is_exhausted() {
        input
            .ast_mut()
            .alloc_position_component(PositionComponentData::Center, span)
    } else {
        input
            .try_parse(|input| {
                parse_position_component::<HorizontalPositionKeyword>(
                    input,
                    parse_horizontal_position_keyword,
                )
            })
            .unwrap_or_else(|_| {
                input
                    .ast_mut()
                    .alloc_position_component(PositionComponentData::Center, span)
            })
    };
    Ok((x, y))
}

fn parse_horizontal_position_keyword<'i>(
    input: &mut Compiler<'i>,
) -> Result<HorizontalPositionKeyword, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    match_ignore_ascii_case!(
        ident,
        "left" => Ok(HorizontalPositionKeyword::Left),
        "right" => Ok(HorizontalPositionKeyword::Right),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

fn parse_vertical_position_keyword<'i>(
    input: &mut Compiler<'i>,
) -> Result<VerticalPositionKeyword, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    match_ignore_ascii_case!(
        ident,
        "top" => Ok(VerticalPositionKeyword::Top),
        "bottom" => Ok(VerticalPositionKeyword::Bottom),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

impl<'i> Parse<'i> for PositionComponent<'i, HorizontalPositionKeyword> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_position_component(input, parse_horizontal_position_keyword)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

impl<'i> Parse<'i> for PositionComponent<'i, VerticalPositionKeyword> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let value = parse_position_component(input, parse_vertical_position_keyword)?;
        input.expect_exhausted()?;
        Ok(value)
    }
}

fn parse_position_component<'i, S: Copy>(
    input: &mut Compiler<'i>,
    mut parse_side: impl FnMut(&mut Compiler<'i>) -> Result<S, ParseError<'i, ParserError<'i>>>,
) -> Result<PositionComponent<'i, S>, ParseError<'i, ParserError<'i>>> {
    let span = token_span(input);
    if input
        .try_parse(|input| input.expect_ident_matching("center"))
        .is_ok()
    {
        return Ok(input
            .ast_mut()
            .alloc_position_component(PositionComponentData::Center, span));
    }
    if let Ok(side) = input.try_parse(&mut parse_side) {
        let offset = input.try_parse(LengthPercentage::parse).ok();
        return Ok(input
            .ast_mut()
            .alloc_position_component(PositionComponentData::Side { offset, side }, span));
    }
    let value = LengthPercentage::parse(input)?;
    Ok(input
        .ast_mut()
        .alloc_position_component(PositionComponentData::Length(value), span))
}

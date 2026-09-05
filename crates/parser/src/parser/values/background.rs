use crate::prelude::*;

type PositionComponents<'i> = (
    NodeId<'i, PositionComponent<'i, HorizontalPositionKeyword>>,
    NodeId<'i, PositionComponent<'i, VerticalPositionKeyword>>,
);

fn zero_position<'i, S>(input: &mut Compiler<'i>) -> NodeId<'i, PositionComponent<'i, S>>
where
    PositionComponent<'i, S>: AstNodeStorage<'i>,
{
    let length = store_node(DimensionPercentage::Percentage(0.0), input);
    store_node(PositionComponent::Length(length), input)
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

pub(super) fn parse_background_repeat<'i>(
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
    if input
        .try_parse(|input| input.expect_ident_matching("cover"))
        .is_ok()
    {
        return Ok(BackgroundSize::Cover);
    }
    if input
        .try_parse(|input| input.expect_ident_matching("contain"))
        .is_ok()
    {
        return Ok(BackgroundSize::Contain);
    }

    let width = store_node(LengthPercentageOrAuto::parse(input)?, input);
    let height = input
        .try_parse(LengthPercentageOrAuto::parse)
        .unwrap_or(LengthPercentageOrAuto::Auto);
    Ok(BackgroundSize::Explicit {
        height: store_node(height, input),
        width,
    })
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
    let x = store_node(
        parse_position_component::<HorizontalPositionKeyword>(
            input,
            parse_horizontal_position_keyword,
        )?,
        input,
    );
    let y = if input.is_exhausted() {
        store_node(PositionComponent::Center, input)
    } else {
        store_node(
            input
                .try_parse(|input| {
                    parse_position_component::<VerticalPositionKeyword>(
                        input,
                        parse_vertical_position_keyword,
                    )
                })
                .unwrap_or(PositionComponent::Center),
            input,
        )
    };
    Ok((x, y))
}

fn parse_vertical_first_position<'i>(
    input: &mut Compiler<'i>,
) -> Result<PositionComponents<'i>, ParseError<'i, ParserError<'i>>> {
    let y = store_node(
        parse_position_component::<VerticalPositionKeyword>(
            input,
            parse_vertical_position_keyword,
        )?,
        input,
    );
    let x = if input.is_exhausted() {
        store_node(PositionComponent::Center, input)
    } else {
        store_node(
            input
                .try_parse(|input| {
                    parse_position_component::<HorizontalPositionKeyword>(
                        input,
                        parse_horizontal_position_keyword,
                    )
                })
                .unwrap_or(PositionComponent::Center),
            input,
        )
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

fn parse_position_component<'i, S>(
    input: &mut Compiler<'i>,
    mut parse_side: impl FnMut(&mut Compiler<'i>) -> Result<S, ParseError<'i, ParserError<'i>>>,
) -> Result<PositionComponent<'i, S>, ParseError<'i, ParserError<'i>>> {
    if input
        .try_parse(|input| input.expect_ident_matching("center"))
        .is_ok()
    {
        return Ok(PositionComponent::Center);
    }
    if let Ok(side) = input.try_parse(&mut parse_side) {
        let offset = input
            .try_parse(LengthPercentage::parse)
            .ok()
            .map(|value| store_node(value, input));
        return Ok(PositionComponent::Side { offset, side });
    }
    Ok(PositionComponent::Length(store_node(
        LengthPercentage::parse(input)?,
        input,
    )))
}

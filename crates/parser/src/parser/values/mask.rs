use crate::prelude::*;

use super::background::{
    parse_background_repeat, parse_background_size, parse_position_components,
};

impl<'i> Parse<'i> for Mask<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut image = None;
        let mut position = None;
        let mut size = None;
        let mut repeat = None;
        let mut clip = None;
        let mut origin = None;
        let mut composite = None;
        let mut mode = None;

        loop {
            if image.is_none()
                && let Ok(value) = input.try_parse(Image::parse)
            {
                image = Some(store_node(value, input));
                continue;
            }

            if position.is_none()
                && let Ok((x, y)) = input.try_parse(parse_position_components)
            {
                position = Some(store_node(Position { x, y }, input));
                size = input
                    .try_parse(|input| {
                        input.expect_delim('/')?;
                        parse_background_size(input)
                    })
                    .ok()
                    .map(|value| store_node(value, input));
                continue;
            }

            if repeat.is_none()
                && let Ok(value) = input.try_parse(parse_background_repeat)
            {
                repeat = Some(value);
                continue;
            }

            if origin.is_none()
                && let Ok(value) = input.try_parse(GeometryBox::parse)
            {
                origin = Some(value);
                continue;
            }

            if clip.is_none()
                && let Ok(value) = input.try_parse(MaskClip::parse)
            {
                clip = Some(value);
                continue;
            }

            if composite.is_none()
                && let Ok(value) = input.try_parse(MaskComposite::parse)
            {
                composite = Some(value);
                continue;
            }

            if mode.is_none()
                && let Ok(value) = input.try_parse(MaskMode::parse)
            {
                mode = Some(value);
                continue;
            }

            break;
        }

        let origin = origin.unwrap_or(GeometryBox::BorderBox);
        let clip = clip.unwrap_or_else(|| MaskClip::GeometryBox(clone_geometry_box(&origin)));
        let image = image.unwrap_or_else(|| store_node(Image::None, input));
        let position = match position {
            Some(position) => position,
            None => {
                let position = default_mask_position(input);
                store_node(position, input)
            }
        };
        let size = match size {
            Some(size) => size,
            None => {
                let size = default_mask_size(input);
                store_node(size, input)
            }
        };
        Ok(Self {
            image,
            position,
            size,
            repeat: repeat.unwrap_or(default_mask_repeat()),
            clip,
            composite: composite.unwrap_or(MaskComposite::Add),
            mode: mode.unwrap_or(MaskMode::MatchSource),
            origin,
        })
    }
}

fn default_mask_position<'i>(input: &mut Compiler<'i>) -> Position<'i> {
    let x = zero_mask_position_component(input);
    let y = zero_mask_position_component(input);
    Position { x, y }
}

fn zero_mask_position_component<'i, S>(
    input: &mut Compiler<'i>,
) -> NodeId<'i, PositionComponent<'i, S>> {
    let length = store_node(DimensionPercentage::Percentage(0.0), input);
    store_node(PositionComponent::Length(length), input)
}

fn default_mask_size<'i>(input: &mut Compiler<'i>) -> BackgroundSize<'i> {
    let height = store_node(LengthPercentageOrAuto::Auto, input);
    let width = store_node(LengthPercentageOrAuto::Auto, input);
    BackgroundSize::Explicit { height, width }
}

fn default_mask_repeat() -> BackgroundRepeat {
    BackgroundRepeat {
        x: BackgroundRepeatKeyword::Repeat,
        y: BackgroundRepeatKeyword::Repeat,
    }
}

fn clone_geometry_box(value: &GeometryBox) -> GeometryBox {
    match value {
        GeometryBox::BorderBox => GeometryBox::BorderBox,
        GeometryBox::PaddingBox => GeometryBox::PaddingBox,
        GeometryBox::ContentBox => GeometryBox::ContentBox,
        GeometryBox::MarginBox => GeometryBox::MarginBox,
        GeometryBox::FillBox => GeometryBox::FillBox,
        GeometryBox::StrokeBox => GeometryBox::StrokeBox,
        GeometryBox::ViewBox => GeometryBox::ViewBox,
    }
}

impl<'i> Parse<'i> for MaskMode {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            "match-source" => Ok(Self::MatchSource),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskComposite {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "intersect" => Ok(Self::Intersect),
            "exclude" => Ok(Self::Exclude),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for WebKitMaskComposite {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "clear" => Ok(Self::Clear),
            "copy" => Ok(Self::Copy),
            "source-over" => Ok(Self::SourceOver),
            "source-in" => Ok(Self::SourceIn),
            "source-out" => Ok(Self::SourceOut),
            "source-atop" => Ok(Self::SourceAtop),
            "destination-over" => Ok(Self::DestinationOver),
            "destination-in" => Ok(Self::DestinationIn),
            "destination-out" => Ok(Self::DestinationOut),
            "destination-atop" => Ok(Self::DestinationAtop),
            "xor" => Ok(Self::Xor),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskType {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskBorderMode {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        MaskType::parse(input).map(|value| match value {
            MaskType::Luminance => Self::Luminance,
            MaskType::Alpha => Self::Alpha,
        })
    }
}

impl<'i> Parse<'i> for WebKitMaskSourceType {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "auto" => Ok(Self::Auto),
            "luminance" => Ok(Self::Luminance),
            "alpha" => Ok(Self::Alpha),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for GeometryBox {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "border-box" => Ok(Self::BorderBox),
            "padding-box" => Ok(Self::PaddingBox),
            "content-box" => Ok(Self::ContentBox),
            "margin-box" => Ok(Self::MarginBox),
            "fill-box" => Ok(Self::FillBox),
            "stroke-box" => Ok(Self::StrokeBox),
            "view-box" => Ok(Self::ViewBox),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for MaskClip {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("no-clip"))
            .is_ok()
        {
            return Ok(Self::NoClip);
        }
        GeometryBox::parse(input).map(Self::GeometryBox)
    }
}

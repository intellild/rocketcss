//! Mask value parsing into the flattened AST.

use super::{
    background::{parse_background_repeat, parse_background_size, parse_position_components},
    prelude::*,
};

impl<'i> Parse<'i> for Mask<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
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
                image = Some(value);
                continue;
            }

            if position.is_none()
                && let Ok((x, y)) = input.try_parse(parse_position_components)
            {
                position = Some(input.ast_mut().alloc_position(PositionData { x, y }, span));
                size = input
                    .try_parse(|input| {
                        input.expect_delim('/')?;
                        parse_background_size(input)
                    })
                    .ok();
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
        let clip = clip.unwrap_or(MaskClip::GeometryBox(origin));
        let image = match image {
            Some(image) => image,
            None => input.ast_mut().alloc_image(ImageData::None, span),
        };
        let position = match position {
            Some(position) => position,
            None => default_mask_position(input.ast_mut(), span),
        };
        let size = match size {
            Some(size) => size,
            None => default_mask_size(input.ast_mut(), span),
        };
        let repeat = match repeat {
            Some(repeat) => repeat,
            None => default_mask_repeat(input.ast_mut(), span),
        };
        Ok(input.ast_mut().alloc_mask(
            MaskData {
                image,
                position,
                size,
                repeat,
                clip,
                composite: composite.unwrap_or(MaskComposite::Add),
                mode: mode.unwrap_or(MaskMode::MatchSource),
                origin,
            },
            span,
        ))
    }
}

fn default_mask_position<'i>(ast: &mut Ast<'i>, span: Span) -> Position<'i> {
    let x = zero_mask_position_component(ast, span);
    let y = zero_mask_position_component(ast, span);
    ast.alloc_position(PositionData { x, y }, span)
}

fn zero_mask_position_component<'i, S: Copy>(
    ast: &mut Ast<'i>,
    span: Span,
) -> PositionComponent<'i, S> {
    let value = ast.alloc_dimension_percentage(
        DimensionPercentageData::<LengthValue>::Percentage(0.0),
        span,
    );
    ast.alloc_position_component(PositionComponentData::Length(value), span)
}

fn default_mask_size<'i>(ast: &mut Ast<'i>, span: Span) -> BackgroundSize<'i> {
    let height = ast.alloc_length_percentage_or_auto(LengthPercentageOrAutoData::Auto, span);
    let width = ast.alloc_length_percentage_or_auto(LengthPercentageOrAutoData::Auto, span);
    ast.alloc_background_size(BackgroundSizeData::Explicit { height, width }, span)
}

fn default_mask_repeat<'i>(ast: &mut Ast<'i>, span: Span) -> BackgroundRepeat<'i> {
    ast.alloc_background_repeat(
        BackgroundRepeatData {
            x: BackgroundRepeatKeyword::Repeat,
            y: BackgroundRepeatKeyword::Repeat,
        },
        span,
    )
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

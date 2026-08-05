use crate::prelude::*;

use super::background::parse_position_components;

impl<'i> Parse<'i> for ObjectFit {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "fill" => Ok(Self::Fill),
            "contain" => Ok(Self::Contain),
            "cover" => Ok(Self::Cover),
            "none" => Ok(Self::None),
            "scale-down" => Ok(Self::ScaleDown),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for Image<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = input.current_token_span().unwrap_or_default();
        let token = input.next()?.clone();
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("none") => Ok(Self::None),
            ValueToken::UnquotedUrl(url) => {
                Ok(Self::Url(input.allocator().boxed(Url { span, url })))
            }
            ValueToken::Function(name) if name.eq_ignore_ascii_case("url") => {
                let url = input.parse_nested_block(|input| {
                    let url = input.expect_string()?;
                    input.expect_exhausted()?;
                    Ok(url)
                })?;
                Ok(Self::Url(input.allocator().boxed(Url { span, url })))
            }
            ValueToken::Function(name)
                if name.eq_ignore_ascii_case("linear-gradient")
                    || name.eq_ignore_ascii_case("-webkit-linear-gradient")
                    || name.eq_ignore_ascii_case("-moz-linear-gradient")
                    || name.eq_ignore_ascii_case("-o-linear-gradient")
                    || name.eq_ignore_ascii_case("repeating-linear-gradient")
                    || name.eq_ignore_ascii_case("-webkit-repeating-linear-gradient")
                    || name.eq_ignore_ascii_case("-moz-repeating-linear-gradient")
                    || name.eq_ignore_ascii_case("-o-repeating-linear-gradient") =>
            {
                let vendor_prefix = gradient_vendor_prefix(name);
                let repeating = name.to_ascii_lowercase().contains("repeating-");
                let gradient = input.parse_nested_block(|input| {
                    parse_linear_gradient(input, vendor_prefix, repeating)
                })?;
                Ok(Self::Gradient(input.allocator().boxed(gradient)))
            }
            ValueToken::Function(name)
                if name.eq_ignore_ascii_case("radial-gradient")
                    || name.eq_ignore_ascii_case("-webkit-radial-gradient")
                    || name.eq_ignore_ascii_case("-moz-radial-gradient")
                    || name.eq_ignore_ascii_case("-o-radial-gradient")
                    || name.eq_ignore_ascii_case("repeating-radial-gradient")
                    || name.eq_ignore_ascii_case("-webkit-repeating-radial-gradient")
                    || name.eq_ignore_ascii_case("-moz-repeating-radial-gradient")
                    || name.eq_ignore_ascii_case("-o-repeating-radial-gradient") =>
            {
                let vendor_prefix = gradient_vendor_prefix(name);
                let repeating = name.to_ascii_lowercase().contains("repeating-");
                let gradient = input.parse_nested_block(|input| {
                    parse_radial_gradient(input, vendor_prefix, repeating)
                })?;
                Ok(Self::Gradient(input.allocator().boxed(gradient)))
            }
            ValueToken::Function(name)
                if name.eq_ignore_ascii_case("conic-gradient")
                    || name.eq_ignore_ascii_case("repeating-conic-gradient") =>
            {
                let repeating = name.eq_ignore_ascii_case("repeating-conic-gradient");
                let gradient =
                    input.parse_nested_block(|input| parse_conic_gradient(input, repeating))?;
                Ok(Self::Gradient(input.allocator().boxed(gradient)))
            }
            ValueToken::Function(name)
                if name.eq_ignore_ascii_case("image-set")
                    || name.eq_ignore_ascii_case("-webkit-image-set") =>
            {
                let vendor_prefix = if name.eq_ignore_ascii_case("-webkit-image-set") {
                    VendorPrefix::WEBKIT
                } else {
                    VendorPrefix::NONE
                };
                let image_set =
                    input.parse_nested_block(|input| parse_image_set(input, vendor_prefix))?;
                Ok(Self::ImageSet(input.allocator().boxed(image_set)))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

fn gradient_vendor_prefix(name: &str) -> VendorPrefix {
    if name
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-webkit-"))
    {
        VendorPrefix::WEBKIT
    } else if name
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-moz-"))
    {
        VendorPrefix::MOZ
    } else if name
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-o-"))
    {
        VendorPrefix::O
    } else {
        VendorPrefix::NONE
    }
}

fn parse_image_set<'i>(
    input: &mut Compiler<'i>,
    vendor_prefix: VendorPrefix,
) -> Result<ImageSet<'i>, ParseError<'i, ParserError<'i>>> {
    let options = super::animation::parse_comma_separated(input, |input| {
        parse_image_set_option(input, vendor_prefix)
    })?;
    Ok(ImageSet {
        options,
        vendor_prefix,
    })
}

fn parse_image_set_option<'i>(
    input: &mut Compiler<'i>,
    vendor_prefix: VendorPrefix,
) -> Result<ImageSetOption<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let span = input.current_token_span().unwrap_or_default();
    let image = if let Ok(url) = input.try_parse(Compiler::expect_string) {
        Image::Url(allocator.boxed(Url { span, url }))
    } else {
        Image::parse(input)?
    };

    let first_resolution = input.try_parse(parse_image_resolution).ok();
    let mut file_type = if first_resolution.is_none() {
        input.try_parse(parse_image_set_file_type).ok()
    } else {
        None
    };
    let resolution = first_resolution.unwrap_or_else(|| {
        input
            .try_parse(parse_image_resolution)
            .unwrap_or(Resolution::Dppx(1.0))
    });
    if file_type.is_none() {
        file_type = input.try_parse(parse_image_set_file_type).ok();
    }
    if vendor_prefix != VendorPrefix::NONE && file_type.is_some() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(ImageSetOption {
        file_type,
        image: allocator.boxed(image),
        resolution,
    })
}

fn parse_image_resolution<'i>(
    input: &mut Compiler<'i>,
) -> Result<Resolution, ParseError<'i, ParserError<'i>>> {
    match input.next()?.clone() {
        ValueToken::Dimension {
            unit: Unit::Dpi,
            value,
        } => Ok(Resolution::Dpi(value)),
        ValueToken::Dimension {
            unit: Unit::Dpcm,
            value,
        } => Ok(Resolution::Dpcm(value)),
        ValueToken::Dimension {
            unit: Unit::Dppx | Unit::ResolutionX,
            value,
        } => Ok(Resolution::Dppx(value)),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

fn parse_image_set_file_type<'i>(
    input: &mut Compiler<'i>,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    input.expect_function_matching("type")?;
    input.parse_nested_block(|input| {
        let value = input.expect_string()?;
        input.expect_exhausted()?;
        Ok(value)
    })
}

fn parse_linear_gradient<'i>(
    input: &mut Compiler<'i>,
    vendor_prefix: VendorPrefix,
    repeating: bool,
) -> Result<Gradient<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let direction = if let Ok(direction) = input.try_parse(parse_line_direction) {
        input.expect_comma()?;
        direction
    } else {
        LineDirection::Vertical(VerticalPositionKeyword::Bottom)
    };
    let mut items = allocator.vec();
    loop {
        items.push(input.parse_until_before(Delimiter::Comma, parse_gradient_item)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted()?;
    if items.len() < 2 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(if repeating {
        Gradient::RepeatingLinear {
            direction,
            items,
            vendor_prefix,
        }
    } else {
        Gradient::Linear {
            direction,
            items,
            vendor_prefix,
        }
    })
}

fn parse_radial_gradient<'i>(
    input: &mut Compiler<'i>,
    vendor_prefix: VendorPrefix,
    repeating: bool,
) -> Result<Gradient<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let default_shape =
        || EndingShape::Ellipse(allocator.boxed(Ellipse::Extent(ShapeExtent::FarthestCorner)));
    let default_position = || Position {
        x: allocator.boxed(PositionComponent::Center),
        y: allocator.boxed(PositionComponent::Center),
    };

    let header = input.try_parse(|input| {
        let shape = input.try_parse(parse_radial_shape).ok();
        let has_position = input
            .try_parse(|input| input.expect_ident_matching("at"))
            .is_ok();
        if shape.is_none() && !has_position {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let position = if has_position {
            input.parse_until_before(Delimiter::Comma, |input| {
                let (x, y) = parse_position_components(input)?;
                Ok(Position { x, y })
            })?
        } else {
            default_position()
        };
        input.expect_comma()?;
        Ok((shape.unwrap_or_else(default_shape), position))
    });

    let (shape, position) = match header {
        Ok(header) => header,
        Err(_) => (default_shape(), default_position()),
    };
    let shape = allocator.boxed(shape);
    let position = allocator.boxed(position);

    let mut items = allocator.vec();
    loop {
        items.push(input.parse_until_before(Delimiter::Comma, parse_gradient_item)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted()?;
    if items.len() < 2 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(if repeating {
        Gradient::RepeatingRadial {
            items,
            position,
            shape,
            vendor_prefix,
        }
    } else {
        Gradient::Radial {
            items,
            position,
            shape,
            vendor_prefix,
        }
    })
}

fn parse_conic_gradient<'i>(
    input: &mut Compiler<'i>,
    repeating: bool,
) -> Result<Gradient<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let default_position = || Position {
        x: allocator.boxed(PositionComponent::Center),
        y: allocator.boxed(PositionComponent::Center),
    };
    let header = input.try_parse(|input| {
        let has_from = input
            .try_parse(|input| input.expect_ident_matching("from"))
            .is_ok();
        let angle = if has_from {
            Angle::parse(input)?
        } else {
            Angle::Deg(0.0)
        };
        let has_position = input
            .try_parse(|input| input.expect_ident_matching("at"))
            .is_ok();
        if !has_from && !has_position {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let position = if has_position {
            input.parse_until_before(Delimiter::Comma, |input| {
                let (x, y) = parse_position_components(input)?;
                Ok(Position { x, y })
            })?
        } else {
            default_position()
        };
        input.expect_comma()?;
        Ok((angle, position))
    });
    let (angle, position) = match header {
        Ok(header) => header,
        Err(_) => (Angle::Deg(0.0), default_position()),
    };
    let position = allocator.boxed(position);

    let mut items = allocator.vec();
    loop {
        items.push(input.parse_until_before(Delimiter::Comma, parse_conic_gradient_item)?);
        if input.try_parse(Compiler::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted()?;
    if items.len() < 2 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(if repeating {
        Gradient::RepeatingConic {
            angle,
            items,
            position,
        }
    } else {
        Gradient::Conic {
            angle,
            items,
            position,
        }
    })
}

fn parse_radial_shape<'i>(
    input: &mut Compiler<'i>,
) -> Result<EndingShape<'i>, ParseError<'i, ParserError<'i>>> {
    if let Ok(value) = input.try_parse(Length::parse) {
        return Ok(EndingShape::Circle(
            input
                .allocator()
                .boxed(Circle::Radius(input.allocator().boxed(value))),
        ));
    }
    let ident = input.expect_ident()?;
    let shape = match_ignore_ascii_case!(
        ident,
        "circle" => Some(true),
        "ellipse" => Some(false),
        "closest-side" | "farthest-side" | "closest-corner" | "farthest-corner" => None,
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    );
    let extent = match ident.to_ascii_lowercase().as_str() {
        "closest-side" => Some(ShapeExtent::ClosestSide),
        "farthest-side" => Some(ShapeExtent::FarthestSide),
        "closest-corner" => Some(ShapeExtent::ClosestCorner),
        "farthest-corner" => Some(ShapeExtent::FarthestCorner),
        _ => None,
    };

    if let Some(circle) = shape {
        if circle {
            let value = if let Ok(extent) = input.try_parse(parse_shape_extent) {
                Circle::Extent(extent)
            } else if let Ok(value) = input.try_parse(Length::parse) {
                Circle::Radius(input.allocator().boxed(value))
            } else {
                Circle::Extent(ShapeExtent::FarthestCorner)
            };
            return Ok(EndingShape::Circle(input.allocator().boxed(value)));
        }

        if let Ok(extent) = input.try_parse(parse_shape_extent) {
            return Ok(EndingShape::Ellipse(
                input.allocator().boxed(Ellipse::Extent(extent)),
            ));
        }
        if let Ok(x) = input.try_parse(LengthPercentage::parse) {
            let y = LengthPercentage::parse(input)?;
            return Ok(EndingShape::Ellipse(input.allocator().boxed(
                Ellipse::Size {
                    x: input.allocator().boxed(x),
                    y: input.allocator().boxed(y),
                },
            )));
        }
        return Ok(EndingShape::Ellipse(
            input
                .allocator()
                .boxed(Ellipse::Extent(ShapeExtent::FarthestCorner)),
        ));
    }

    Ok(EndingShape::Ellipse(input.allocator().boxed(
        Ellipse::Extent(extent.unwrap_or(ShapeExtent::FarthestCorner)),
    )))
}

fn parse_shape_extent<'i>(
    input: &mut Compiler<'i>,
) -> Result<ShapeExtent, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    match_ignore_ascii_case!(
        ident,
        "closest-side" => Ok(ShapeExtent::ClosestSide),
        "farthest-side" => Ok(ShapeExtent::FarthestSide),
        "closest-corner" => Ok(ShapeExtent::ClosestCorner),
        "farthest-corner" => Ok(ShapeExtent::FarthestCorner),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

fn parse_line_direction<'i>(
    input: &mut Compiler<'i>,
) -> Result<LineDirection, ParseError<'i, ParserError<'i>>> {
    if input
        .try_parse(|input| input.expect_ident_matching("to"))
        .is_ok()
    {
        let first = input.expect_ident()?;
        let horizontal = match_ignore_ascii_case!(
            first,
            "left" => Some(HorizontalPositionKeyword::Left),
            "right" => Some(HorizontalPositionKeyword::Right),
            _ => None,
        );
        let vertical = match_ignore_ascii_case!(
            first,
            "top" => Some(VerticalPositionKeyword::Top),
            "bottom" => Some(VerticalPositionKeyword::Bottom),
            _ => None,
        );
        if let Some(horizontal) = horizontal {
            if let Ok(second) = input.try_parse(Compiler::expect_ident) {
                let vertical = match_ignore_ascii_case!(
                    second,
                    "top" => Some(VerticalPositionKeyword::Top),
                    "bottom" => Some(VerticalPositionKeyword::Bottom),
                    _ => None,
                )
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                return Ok(LineDirection::Corner {
                    horizontal,
                    vertical,
                });
            }
            return Ok(LineDirection::Horizontal(horizontal));
        }
        if let Some(vertical) = vertical {
            if input.try_parse(Compiler::expect_ident).is_ok() {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            return Ok(LineDirection::Vertical(vertical));
        }
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let token = input.next()?.clone();
    let ValueToken::Dimension { unit, value } = token else {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    };
    let angle = match unit {
        Unit::Deg => Angle::Deg(value),
        Unit::Rad => Angle::Rad(value),
        Unit::Grad => Angle::Grad(value),
        Unit::Turn => Angle::Turn(value),
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };
    Ok(LineDirection::Angle(angle))
}

fn parse_gradient_item<'i>(
    input: &mut Compiler<'i>,
) -> Result<GradientItem<'i, LengthValue>, ParseError<'i, ParserError<'i>>> {
    if let Ok(color) = input.try_parse(CssColor::parse) {
        let color = input.allocator().boxed(color);
        let position = input
            .try_parse(LengthPercentage::parse)
            .ok()
            .map(|value| input.allocator().boxed(value));
        input.expect_exhausted()?;
        return Ok(GradientItem::ColorStop { color, position });
    }
    let value = LengthPercentage::parse(input)?;
    input.expect_exhausted()?;
    Ok(GradientItem::Hint(input.allocator().boxed(value)))
}

fn parse_conic_gradient_item<'i>(
    input: &mut Compiler<'i>,
) -> Result<GradientItem<'i, Angle>, ParseError<'i, ParserError<'i>>> {
    if let Ok(color) = input.try_parse(CssColor::parse) {
        let color = input.allocator().boxed(color);
        let position = input
            .try_parse(parse_angle_percentage)
            .ok()
            .map(|value| input.allocator().boxed(value));
        input.expect_exhausted()?;
        return Ok(GradientItem::ColorStop { color, position });
    }
    let value = parse_angle_percentage(input)?;
    input.expect_exhausted()?;
    Ok(GradientItem::Hint(input.allocator().boxed(value)))
}

fn parse_angle_percentage<'i>(
    input: &mut Compiler<'i>,
) -> Result<AnglePercentage<'i>, ParseError<'i, ParserError<'i>>> {
    match input.next()?.clone() {
        ValueToken::Percentage(value) => Ok(DimensionPercentage::Percentage(value)),
        ValueToken::Number(0.0) => Ok(DimensionPercentage::Dimension(Angle::Deg(0.0))),
        ValueToken::Dimension { unit, value } => {
            let angle = match unit {
                Unit::Deg => Angle::Deg(value),
                Unit::Rad => Angle::Rad(value),
                Unit::Grad => Angle::Grad(value),
                Unit::Turn => Angle::Turn(value),
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            };
            Ok(DimensionPercentage::Dimension(angle))
        }
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

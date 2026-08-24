use crate::prelude::*;

fn parse_four_values<'i, T>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
    clone: impl Fn(&T, &'i Allocator) -> Option<Box<'i, T>>,
) -> Result<[Box<'i, T>; 4], ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut values: [Option<Box<'i, T>>; 4] = std::array::from_fn(|_| None);
    let mut count = 0;
    while !input.is_exhausted() {
        if count == values.len() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        values[count] = Some(allocator.boxed(parse(input)?));
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let top = values[0].take().unwrap();
    let right = match count {
        1 => clone(&top, allocator),
        _ => values[1].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let bottom = match count {
        1 | 2 => clone(&top, allocator),
        _ => values[2].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let left = match count {
        1 => clone(&top, allocator),
        2 | 3 => clone(&right, allocator),
        _ => values[3].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    Ok([top, right, bottom, left])
}

fn parse_two_values<'i, T>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
    clone: impl Fn(&T, &'i Allocator) -> Option<Box<'i, T>>,
) -> Result<[Box<'i, T>; 2], ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let first = allocator.boxed(parse(input)?);
    let second = if input.is_exhausted() {
        clone(&first, allocator).ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?
    } else {
        allocator.boxed(parse(input)?)
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

fn clone_color<'i>(
    value: &CssColor<'i>,
    allocator: &'i Allocator,
) -> Option<Box<'i, CssColor<'i>>> {
    let value = match value {
        CssColor::CurrentColor => CssColor::CurrentColor,
        CssColor::Rgba(value) => CssColor::Rgba(*value),
        _ => return None,
    };
    Some(allocator.boxed(value))
}

fn clone_line_style<'i>(value: &LineStyle, allocator: &'i Allocator) -> Option<Box<'i, LineStyle>> {
    let value = match value {
        LineStyle::None => LineStyle::None,
        LineStyle::Hidden => LineStyle::Hidden,
        LineStyle::Inset => LineStyle::Inset,
        LineStyle::Groove => LineStyle::Groove,
        LineStyle::Outset => LineStyle::Outset,
        LineStyle::Ridge => LineStyle::Ridge,
        LineStyle::Dotted => LineStyle::Dotted,
        LineStyle::Dashed => LineStyle::Dashed,
        LineStyle::Solid => LineStyle::Solid,
        LineStyle::Double => LineStyle::Double,
    };
    Some(allocator.boxed(value))
}

fn clone_length<'i>(value: &Length<'i>, allocator: &'i Allocator) -> Option<Box<'i, Length<'i>>> {
    let Length::Value(value) = value else {
        return None;
    };
    Some(allocator.boxed(Length::Value(LengthValue {
        unit: value.unit,
        value: value.value,
    })))
}

fn clone_border_side_width<'i>(
    value: &BorderSideWidth<'i>,
    allocator: &'i Allocator,
) -> Option<Box<'i, BorderSideWidth<'i>>> {
    let value = match value {
        BorderSideWidth::Thin => BorderSideWidth::Thin,
        BorderSideWidth::Medium => BorderSideWidth::Medium,
        BorderSideWidth::Thick => BorderSideWidth::Thick,
        BorderSideWidth::Length(value) => BorderSideWidth::Length(clone_length(value, allocator)?),
    };
    Some(allocator.boxed(value))
}

impl<'i> Parse<'i> for BorderColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_values(input, CssColor::parse, clone_color)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl<'i> Parse<'i> for BorderStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] =
            parse_four_values(input, LineStyle::parse, clone_line_style)?;
        Ok(Self {
            top: Box::into_inner(top),
            right: Box::into_inner(right),
            bottom: Box::into_inner(bottom),
            left: Box::into_inner(left),
        })
    }
}

impl<'i> Parse<'i> for BorderWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] =
            parse_four_values(input, BorderSideWidth::parse, clone_border_side_width)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl<'i> Parse<'i> for BorderBlockColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, CssColor::parse, clone_color)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderBlockStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, LineStyle::parse, clone_line_style)?;
        Ok(Self {
            start: Box::into_inner(start),
            end: Box::into_inner(end),
        })
    }
}

impl<'i> Parse<'i> for BorderBlockWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] =
            parse_two_values(input, BorderSideWidth::parse, clone_border_side_width)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderInlineColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, CssColor::parse, clone_color)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderInlineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, LineStyle::parse, clone_line_style)?;
        Ok(Self {
            start: Box::into_inner(start),
            end: Box::into_inner(end),
        })
    }
}

impl<'i> Parse<'i> for BorderInlineWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] =
            parse_two_values(input, BorderSideWidth::parse, clone_border_side_width)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for OutlineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        LineStyle::parse(input).map(Self::LineStyle)
    }
}

impl<'i> Parse<'i> for Size2D<'i, LengthPercentage<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [x, y] = parse_two_values(input, LengthPercentage::parse, |value, allocator| {
            let value = match value {
                LengthPercentage::Dimension(value) => LengthPercentage::Dimension(LengthValue {
                    unit: value.unit,
                    value: value.value,
                }),
                LengthPercentage::Percentage(value) => LengthPercentage::Percentage(*value),
                LengthPercentage::Zero => LengthPercentage::Zero,
                LengthPercentage::Calc(_) => return None,
            };
            Some(allocator.boxed(value))
        })?;
        Ok(Self(x, y))
    }
}

impl<'i> Parse<'i> for Size2D<'i, Length<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let first_state = input.state();
        let first = allocator.boxed(Length::parse(input)?);
        let second = if input.is_exhausted() {
            input.reset(&first_state);
            allocator.boxed(Length::parse(input)?)
        } else {
            allocator.boxed(Length::parse(input)?)
        };
        Ok(Self(first, second))
    }
}

fn parse_radius_four<'i>(
    input: &mut Compiler<'i>,
) -> Result<[Box<'i, LengthPercentage<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut values: [Option<Box<'i, LengthPercentage<'i>>>; 4] = std::array::from_fn(|_| None);
    let mut count = 0;
    while count < values.len() && !input.is_exhausted() {
        let state = input.state();
        if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            input.reset(&state);
            break;
        }
        values[count] = Some(allocator.boxed(LengthPercentage::parse(input)?));
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let clone = |value: &Box<'i, LengthPercentage<'i>>| {
        clone_length_percentage(value, allocator)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))
    };
    let top = values[0].take().unwrap();
    let right = match count {
        1 => clone(&top)?,
        _ => values[1].take().unwrap(),
    };
    let bottom = match count {
        1 | 2 => clone(&top)?,
        _ => values[2].take().unwrap(),
    };
    let left = match count {
        1 => clone(&top)?,
        2 | 3 => clone(&right)?,
        _ => values[3].take().unwrap(),
    };
    Ok([top, right, bottom, left])
}

fn clone_length_percentage<'i>(
    value: &LengthPercentage<'i>,
    allocator: &'i Allocator,
) -> Option<Box<'i, LengthPercentage<'i>>> {
    let value = match value {
        LengthPercentage::Dimension(value) => LengthPercentage::Dimension(LengthValue {
            unit: value.unit,
            value: value.value,
        }),
        LengthPercentage::Percentage(value) => LengthPercentage::Percentage(*value),
        LengthPercentage::Zero => LengthPercentage::Zero,
        LengthPercentage::Calc(_) => return None,
    };
    Some(allocator.boxed(value))
}

impl<'i> Parse<'i> for BorderRadius<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let horizontal = parse_radius_four(input)?;
        let vertical = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            parse_radius_four(input)?
        } else {
            [
                clone_length_percentage(&horizontal[0], input.allocator())
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(&horizontal[1], input.allocator())
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(&horizontal[2], input.allocator())
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(&horizontal[3], input.allocator())
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
            ]
        };
        let [top_left_x, top_right_x, bottom_right_x, bottom_left_x] = horizontal;
        let [top_left_y, top_right_y, bottom_right_y, bottom_left_y] = vertical;
        Ok(Self {
            top_left: input.allocator().boxed(Size2D(top_left_x, top_left_y)),
            top_right: input.allocator().boxed(Size2D(top_right_x, top_right_y)),
            bottom_right: input
                .allocator()
                .boxed(Size2D(bottom_right_x, bottom_right_y)),
            bottom_left: input
                .allocator()
                .boxed(Size2D(bottom_left_x, bottom_left_y)),
        })
    }
}

fn parse_generic_border<'i, S>(
    input: &mut Compiler<'i>,
    mut parse_style: impl FnMut(&mut Compiler<'i>) -> Result<S, ParseError<'i, ParserError<'i>>>,
    default_style: S,
) -> Result<GenericBorder<'i, S>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut color = None;
    let mut style = None;
    let mut width = None;

    while !input.is_exhausted() {
        if width.is_none()
            && let Ok(value) = input.try_parse(BorderSideWidth::parse)
        {
            width = Some(allocator.boxed(value));
            continue;
        }
        if style.is_none()
            && let Ok(value) = input.try_parse(&mut parse_style)
        {
            style = Some(value);
            continue;
        }
        if color.is_none()
            && let Ok(value) = input.try_parse(CssColor::parse)
        {
            color = Some(allocator.boxed(value));
            continue;
        }
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    if width.is_none() && style.is_none() && color.is_none() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    Ok(GenericBorder {
        color: color.unwrap_or_else(|| allocator.boxed(CssColor::CurrentColor)),
        style: style.unwrap_or(default_style),
        width: width.unwrap_or_else(|| allocator.boxed(BorderSideWidth::Medium)),
    })
}

impl<'i> Parse<'i> for GenericBorder<'i, LineStyle> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        parse_generic_border(input, LineStyle::parse, LineStyle::None)
    }
}

impl<'i> Parse<'i> for GenericBorder<'i, OutlineStyle> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        parse_generic_border(
            input,
            OutlineStyle::parse,
            OutlineStyle::LineStyle(LineStyle::None),
        )
    }
}

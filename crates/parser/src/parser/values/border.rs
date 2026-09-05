use crate::prelude::*;

fn parse_four_nodes<'i, T: 'i + AstNodeStorage<'i>>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
    clone: impl Fn(NodeId<'i, T>, &mut Compiler<'i>) -> Option<NodeId<'i, T>>,
) -> Result<[NodeId<'i, T>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<NodeId<'i, T>>; 4] = [None; 4];
    let mut count = 0;
    while !input.is_exhausted() {
        if count == values.len() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        values[count] = Some(store_node(parse(input)?, input));
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let top = values[0].take().unwrap();
    let right = match count {
        1 => clone(top, input),
        _ => values[1].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let bottom = match count {
        1 | 2 => clone(top, input),
        _ => values[2].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let left = match count {
        1 => clone(top, input),
        2 | 3 => clone(right, input),
        _ => values[3].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    Ok([top, right, bottom, left])
}

fn parse_two_nodes<'i, T: 'i + AstNodeStorage<'i>>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
    clone: impl Fn(NodeId<'i, T>, &mut Compiler<'i>) -> Option<NodeId<'i, T>>,
) -> Result<[NodeId<'i, T>; 2], ParseError<'i, ParserError<'i>>> {
    let first = store_node(parse(input)?, input);
    let second = if input.is_exhausted() {
        clone(first, input).ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?
    } else {
        store_node(parse(input)?, input)
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

fn parse_four_colors<'i>(
    input: &mut Compiler<'i>,
) -> Result<[NodeId<'i, CssColor<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values = [None; 4];
    let mut count = 0;
    while !input.is_exhausted() {
        if count == values.len() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        values[count] = Some(parse_css_color(input)?);
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    let top = values[0].unwrap();
    let right = values[1].unwrap_or(top);
    let bottom = values[2].unwrap_or(top);
    let left = values[3].unwrap_or(if count == 2 { right } else { top });
    Ok([top, right, bottom, left])
}

fn parse_two_colors<'i>(
    input: &mut Compiler<'i>,
) -> Result<[NodeId<'i, CssColor<'i>>; 2], ParseError<'i, ParserError<'i>>> {
    let first = parse_css_color(input)?;
    let second = if input.is_exhausted() {
        first
    } else {
        parse_css_color(input)?
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

fn clone_line_style(value: &LineStyle) -> LineStyle {
    match value {
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
    }
}

fn clone_length<'i>(
    id: NodeId<'i, Length<'i>>,
    input: &mut Compiler<'i>,
) -> Option<NodeId<'i, Length<'i>>> {
    let Length::Value(value) = input.ast_context().node(id) else {
        return None;
    };
    let value = LengthValue {
        unit: value.unit,
        value: value.value,
    };
    Some(store_node(Length::Value(value), input))
}

fn clone_border_side_width<'i>(
    id: NodeId<'i, BorderSideWidth<'i>>,
    input: &mut Compiler<'i>,
) -> Option<NodeId<'i, BorderSideWidth<'i>>> {
    let value = match input.ast_context().node(id) {
        BorderSideWidth::Thin => BorderSideWidth::Thin,
        BorderSideWidth::Medium => BorderSideWidth::Medium,
        BorderSideWidth::Thick => BorderSideWidth::Thick,
        BorderSideWidth::Length(value) => BorderSideWidth::Length(clone_length(value, input)?),
    };
    Some(store_node(value, input))
}

fn parse_four_line_styles<'i>(
    input: &mut Compiler<'i>,
) -> Result<[LineStyle; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<LineStyle>; 4] = std::array::from_fn(|_| None);
    let mut count = 0;
    while !input.is_exhausted() {
        if count == values.len() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        values[count] = Some(LineStyle::parse(input)?);
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    let top = values[0].take().unwrap();
    let right = values[1].take().unwrap_or_else(|| clone_line_style(&top));
    let bottom = values[2].take().unwrap_or_else(|| clone_line_style(&top));
    let left = values[3].take().unwrap_or_else(|| {
        if count == 2 {
            clone_line_style(&right)
        } else {
            clone_line_style(&top)
        }
    });
    Ok([top, right, bottom, left])
}

fn parse_two_line_styles<'i>(
    input: &mut Compiler<'i>,
) -> Result<[LineStyle; 2], ParseError<'i, ParserError<'i>>> {
    let first = LineStyle::parse(input)?;
    let second = if input.is_exhausted() {
        clone_line_style(&first)
    } else {
        LineStyle::parse(input)?
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

impl<'i> Parse<'i> for BorderColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_colors(input)?;
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
        let [top, right, bottom, left] = parse_four_line_styles(input)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl<'i> Parse<'i> for BorderWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] =
            parse_four_nodes(input, BorderSideWidth::parse, clone_border_side_width)?;
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
        let [start, end] = parse_two_colors(input)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderBlockStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_line_styles(input)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderBlockWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_nodes(input, BorderSideWidth::parse, clone_border_side_width)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderInlineColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_colors(input)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderInlineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_line_styles(input)?;
        Ok(Self { start, end })
    }
}

impl<'i> Parse<'i> for BorderInlineWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_nodes(input, BorderSideWidth::parse, clone_border_side_width)?;
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
        let [x, y] = parse_two_nodes(input, LengthPercentage::parse, |id, input| {
            let value = match input.ast_context().node(id) {
                LengthPercentage::Dimension(value) => LengthPercentage::Dimension(LengthValue {
                    unit: value.unit,
                    value: value.value,
                }),
                LengthPercentage::Percentage(value) => LengthPercentage::Percentage(value),
                LengthPercentage::Zero => LengthPercentage::Zero,
                LengthPercentage::Calc(_) => return None,
            };
            Some(store_node(value, input))
        })?;
        Ok(Self(x, y))
    }
}

impl<'i> Parse<'i> for Size2D<'i, Length<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first_state = input.state();
        let first = store_node(Length::parse(input)?, input);
        let second = if input.is_exhausted() {
            input.reset(&first_state);
            store_node(Length::parse(input)?, input)
        } else {
            store_node(Length::parse(input)?, input)
        };
        Ok(Self(first, second))
    }
}

fn parse_radius_four<'i>(
    input: &mut Compiler<'i>,
) -> Result<[NodeId<'i, LengthPercentage<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<NodeId<'i, LengthPercentage<'i>>>; 4] = [None; 4];
    let mut count = 0;
    while count < values.len() && !input.is_exhausted() {
        let state = input.state();
        if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            input.reset(&state);
            break;
        }
        values[count] = Some(store_node(LengthPercentage::parse(input)?, input));
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let top = values[0].take().unwrap();
    let right = match count {
        1 => clone_length_percentage(top, input)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
        _ => values[1].take().unwrap(),
    };
    let bottom = match count {
        1 | 2 => clone_length_percentage(top, input)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
        _ => values[2].take().unwrap(),
    };
    let left = match count {
        1 => clone_length_percentage(top, input)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
        2 | 3 => clone_length_percentage(right, input)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
        _ => values[3].take().unwrap(),
    };
    Ok([top, right, bottom, left])
}

fn clone_length_percentage<'i>(
    id: NodeId<'i, LengthPercentage<'i>>,
    input: &mut Compiler<'i>,
) -> Option<NodeId<'i, LengthPercentage<'i>>> {
    let value = match input.ast_context().node(id) {
        LengthPercentage::Dimension(value) => LengthPercentage::Dimension(LengthValue {
            unit: value.unit,
            value: value.value,
        }),
        LengthPercentage::Percentage(value) => LengthPercentage::Percentage(value),
        LengthPercentage::Zero => LengthPercentage::Zero,
        LengthPercentage::Calc(_) => return None,
    };
    Some(store_node(value, input))
}

impl<'i> Parse<'i> for BorderRadius<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let horizontal = parse_radius_four(input)?;
        let vertical = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            parse_radius_four(input)?
        } else {
            [
                clone_length_percentage(horizontal[0], input)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(horizontal[1], input)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(horizontal[2], input)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
                clone_length_percentage(horizontal[3], input)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?,
            ]
        };
        let [top_left_x, top_right_x, bottom_right_x, bottom_left_x] = horizontal;
        let [top_left_y, top_right_y, bottom_right_y, bottom_left_y] = vertical;
        Ok(Self {
            top_left: store_node(Size2D(top_left_x, top_left_y), input),
            top_right: store_node(Size2D(top_right_x, top_right_y), input),
            bottom_right: store_node(Size2D(bottom_right_x, bottom_right_y), input),
            bottom_left: store_node(Size2D(bottom_left_x, bottom_left_y), input),
        })
    }
}

fn parse_generic_border<'i, S>(
    input: &mut Compiler<'i>,
    mut parse_style: impl FnMut(&mut Compiler<'i>) -> Result<S, ParseError<'i, ParserError<'i>>>,
    default_style: S,
) -> Result<GenericBorder<'i, S>, ParseError<'i, ParserError<'i>>> {
    let mut color = None;
    let mut style = None;
    let mut width = None;

    while !input.is_exhausted() {
        if width.is_none()
            && let Ok(value) = input.try_parse(BorderSideWidth::parse)
        {
            width = Some(store_node(value, input));
            continue;
        }
        if style.is_none()
            && let Ok(value) = input.try_parse(&mut parse_style)
        {
            style = Some(value);
            continue;
        }
        if color.is_none()
            && let Ok(value) = input.try_parse(parse_css_color)
        {
            color = Some(value);
            continue;
        }
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    if width.is_none() && style.is_none() && color.is_none() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    Ok(GenericBorder {
        color: color.unwrap_or_else(|| {
            input
                .ast_context_mut()
                .alloc_node_without_span(CssColor::CurrentColor)
        }),
        style: style.unwrap_or(default_style),
        width: width.unwrap_or_else(|| store_node(BorderSideWidth::Medium, input)),
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

//! Border value parsing into the flattened AST.

use super::prelude::*;

impl<'i> Parse<'i> for LineStyle {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "none" => Ok(Self::None),
            "hidden" => Ok(Self::Hidden),
            "inset" => Ok(Self::Inset),
            "groove" => Ok(Self::Groove),
            "outset" => Ok(Self::Outset),
            "ridge" => Ok(Self::Ridge),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "solid" => Ok(Self::Solid),
            "double" => Ok(Self::Double),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BorderSideWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if let Ok(ident) = input.try_parse(Compiler::expect_ident) {
            let data = match_ignore_ascii_case!(
                ident,
                "thin" => BorderSideWidthData::Thin,
                "medium" => BorderSideWidthData::Medium,
                "thick" => BorderSideWidthData::Thick,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
            return Ok(input.ast_mut().alloc_border_side_width(data, DUMMY_SP));
        }
        let length = Length::parse(input)?;
        if !is_non_negative_length(&length, input.ast()) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(input.ast_mut().alloc_border_side_width(
            BorderSideWidthData::Length(length),
            DUMMY_SP,
        ))
    }
}

pub(super) fn is_non_negative_length<'i>(value: &Length<'i>, ast: &Ast<'i>) -> bool {
    match value.data(ast) {
        LengthData::Value(value) => value.value(ast) >= 0.0,
        LengthData::Calc(_) => true,
    }
}

pub(super) fn is_non_negative_length_percentage<'i>(value: &LengthPercentage<'i>, ast: &Ast<'i>) -> bool {
    match value.data(ast) {
        DimensionPercentageData::Dimension(value) => value.value(ast) >= 0.0,
        DimensionPercentageData::Percentage(value) => value >= 0.0,
        DimensionPercentageData::Zero | DimensionPercentageData::Calc(_) => true,
    }
}

fn parse_four_values<'i, T: Copy>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
) -> Result<[T; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<T>; 4] = [None; 4];
    let mut count = 0;
    while !input.is_exhausted() {
        if count == values.len() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        values[count] = Some(parse(input)?);
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    let top = values[0].unwrap();
    let right = if count == 1 { top } else { values[1].unwrap() };
    let bottom = if count <= 2 { top } else { values[2].unwrap() };
    let left = if count == 1 {
        top
    } else if count <= 3 {
        right
    } else {
        values[3].unwrap()
    };
    Ok([top, right, bottom, left])
}

fn parse_two_values<'i, T: Copy>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(&mut Compiler<'i>) -> Result<T, ParseError<'i, ParserError<'i>>>,
) -> Result<[T; 2], ParseError<'i, ParserError<'i>>> {
    let first = parse(input)?;
    let second = if input.is_exhausted() { first } else { parse(input)? };
    input.expect_exhausted()?;
    Ok([first, second])
}

impl<'i> Parse<'i> for BorderColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] =
            parse_four_values(input, CssColor::parse)?;
        Ok(input.ast_mut().alloc_border_color(
            BorderColorData {
                top,
                right,
                bottom,
                left,
            },
            DUMMY_SP,
        ))
    }
}

impl<'i> Parse<'i> for BorderStyle<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_values(input, LineStyle::parse)?;
        Ok(input.ast_mut().alloc_border_style(
            BorderStyleData {
                top,
                right,
                bottom,
                left,
            },
            DUMMY_SP,
        ))
    }
}

impl<'i> Parse<'i> for BorderWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] =
            parse_four_values(input, BorderSideWidth::parse)?;
        Ok(input.ast_mut().alloc_border_width(
            BorderWidthData {
                top,
                right,
                bottom,
                left,
            },
            DUMMY_SP,
        ))
    }
}

impl<'i> Parse<'i> for BorderBlockColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, CssColor::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_block_color(BorderBlockColorData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for BorderBlockStyle<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, LineStyle::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_block_style(BorderBlockStyleData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for BorderBlockWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, BorderSideWidth::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_block_width(BorderBlockWidthData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for BorderInlineColor<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, CssColor::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_inline_color(BorderInlineColorData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for BorderInlineStyle<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, LineStyle::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_inline_style(BorderInlineStyleData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for BorderInlineWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [start, end] = parse_two_values(input, BorderSideWidth::parse)?;
        Ok(input
            .ast_mut()
            .alloc_border_inline_width(BorderInlineWidthData { start, end }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for OutlineStyle<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = DUMMY_SP;
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(input
                .ast_mut()
                .alloc_outline_style(OutlineStyleData::Auto, span));
        }
        let style = LineStyle::parse(input)?;
        Ok(input
            .ast_mut()
            .alloc_outline_style(OutlineStyleData::LineStyle(style), span))
    }
}

impl<'i> Parse<'i> for Size2D<'i, LengthPercentage<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [x, y] = parse_two_values(input, LengthPercentage::parse)?;
        Ok(input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: x, field_1: y }, DUMMY_SP))
    }
}

impl<'i> Parse<'i> for Size2D<'i, Length<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let state = input.state();
        let first = Length::parse(input)?;
        let second = if input.is_exhausted() {
            input.reset(&state);
            Length::parse(input)?
        } else {
            Length::parse(input)?
        };
        Ok(input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: first, field_1: second }, DUMMY_SP))
    }
}

fn parse_radius_four<'i>(
    input: &mut Compiler<'i>,
) -> Result<[LengthPercentage<'i>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<LengthPercentage<'i>>; 4] = [None; 4];
    let mut count = 0;
    while count < values.len() && !input.is_exhausted() {
        let state = input.state();
        if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            input.reset(&state);
            break;
        }
        values[count] = Some(LengthPercentage::parse(input)?);
        count += 1;
    }
    if count == 0 {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    let top = values[0].unwrap();
    let right = if count == 1 { top } else { values[1].unwrap() };
    let bottom = if count <= 2 { top } else { values[2].unwrap() };
    let left = if count == 1 {
        top
    } else if count <= 3 {
        right
    } else {
        values[3].unwrap()
    };
    Ok([top, right, bottom, left])
}

impl<'i> Parse<'i> for BorderRadius<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let horizontal = parse_radius_four(input)?;
        let vertical = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            parse_radius_four(input)?
        } else {
            horizontal
        };
        let [top_left_x, top_right_x, bottom_right_x, bottom_left_x] = horizontal;
        let [top_left_y, top_right_y, bottom_right_y, bottom_left_y] = vertical;
        let top_left = input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: top_left_x, field_1: top_left_y }, DUMMY_SP);
        let top_right = input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: top_right_x, field_1: top_right_y }, DUMMY_SP);
        let bottom_right = input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: bottom_right_x, field_1: bottom_right_y }, DUMMY_SP);
        let bottom_left = input
            .ast_mut()
            .alloc_size_2_d(Size2DData { field_0: bottom_left_x, field_1: bottom_left_y }, DUMMY_SP);
        Ok(input.ast_mut().alloc_border_radius(
            BorderRadiusData {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            },
            DUMMY_SP,
        ))
    }
}

fn parse_generic_border<'i, S: Copy>(
    input: &mut Compiler<'i>,
    mut parse_style: impl FnMut(&mut Compiler<'i>) -> Result<S, ParseError<'i, ParserError<'i>>>,
    default_style: S,
) -> Result<GenericBorder<'i, S>, ParseError<'i, ParserError<'i>>> {
    let span = DUMMY_SP;
    let mut color = None;
    let mut style = None;
    let mut width = None;

    while !input.is_exhausted() {
        if width.is_none()
            && let Ok(value) = input.try_parse(BorderSideWidth::parse)
        {
            width = Some(value);
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
            color = Some(value);
            continue;
        }
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    if width.is_none() && style.is_none() && color.is_none() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let color = match color {
        Some(color) => color,
        None => input
            .ast_mut()
            .alloc_css_color(CssColorData::CurrentColor, span),
    };
    let width = match width {
        Some(width) => width,
        None => input
            .ast_mut()
            .alloc_border_side_width(BorderSideWidthData::Medium, span),
    };
    Ok(input.ast_mut().alloc_generic_border(
        GenericBorderData {
            color,
            style: style.unwrap_or(default_style),
            width,
        },
        span,
    ))
}

impl<'i> Parse<'i> for GenericBorder<'i, LineStyle> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        parse_generic_border(input, LineStyle::parse, LineStyle::None)
    }
}

impl<'i> Parse<'i> for GenericBorder<'i, OutlineStyle<'i>> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let default_style = input
            .ast_mut()
            .alloc_outline_style(OutlineStyleData::LineStyle(LineStyle::None), DUMMY_SP);
        parse_generic_border(input, OutlineStyle::parse, default_style)
    }
}

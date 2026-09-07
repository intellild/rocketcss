use crate::parser::length::is_non_negative_length;
use crate::parser::values::alignment::{
    parse_align_content_value, parse_align_items_value, parse_align_self_value,
    parse_justify_content_value, parse_justify_items_value, parse_justify_self_value,
};
use crate::prelude::*;

impl<'i> Parse<'i> for ColumnRule<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut width = None;
        let mut style = None;
        let mut color = None;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(BorderSideWidth::parse)
            {
                width = Some(store_node(value, input));
                continue;
            }
            if style.is_none()
                && let Ok(value) = input.try_parse(LineStyle::parse)
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
        Ok(Self {
            color,
            style,
            width,
        })
    }
}

impl<'i> Parse<'i> for ColumnWidth<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        let length = Length::parse(input)?;
        if !is_non_negative_length(&length) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Length(store_node(length, input)))
    }
}

impl<'i> Parse<'i> for ColumnCount {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        let value = input.expect_integer()?;
        if value < 1 {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self::Integer(value))
    }
}

impl<'i> Parse<'i> for Columns<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut width = None;
        let mut count = None;
        let mut auto_count = 0u8;

        while !input.is_exhausted() {
            if width.is_none()
                && let Ok(value) = input.try_parse(Length::parse)
                && is_non_negative_length(&value)
            {
                width = Some(ColumnWidth::Length(store_node(value, input)));
                continue;
            }
            if count.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_integer)
                && value >= 1
            {
                count = Some(ColumnCount::Integer(value));
                continue;
            }
            if auto_count < 2
                && input
                    .try_parse(|input| input.expect_ident_matching("auto"))
                    .is_ok()
            {
                auto_count += 1;
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let missing_components = u8::from(width.is_none()) + u8::from(count.is_none());
        if missing_components == 2 && auto_count == 0 || auto_count > missing_components {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            count: count.unwrap_or(ColumnCount::Auto),
            width: width.unwrap_or(ColumnWidth::Auto),
        })
    }
}

impl<'i> Parse<'i> for Gap<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first_state = input.state();
        let row = store_node(GapValue::parse(input)?, input);
        let column = if input.is_exhausted() {
            input.reset(&first_state);
            store_node(GapValue::parse(input)?, input)
        } else {
            store_node(GapValue::parse(input)?, input)
        };
        Ok(Self { row, column })
    }
}

impl<'i> Parse<'i> for PlaceContent {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first_state = input.state();
        let align = parse_align_content_value(input)?;
        let justify = if input.is_exhausted() {
            input.reset(&first_state);
            parse_justify_content_value(input)?
        } else {
            parse_justify_content_value(input)?
        };
        input.expect_exhausted()?;
        Ok(Self { align, justify })
    }
}

impl<'i> Parse<'i> for PlaceSelf {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first_state = input.state();
        let align = parse_align_self_value(input)?;
        let justify = if input.is_exhausted() {
            input.reset(&first_state);
            parse_justify_self_value(input)?
        } else {
            parse_justify_self_value(input)?
        };
        input.expect_exhausted()?;
        Ok(Self { align, justify })
    }
}

impl<'i> Parse<'i> for PlaceItems {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let first_state = input.state();
        let align = parse_align_items_value(input)?;
        let justify = if input.is_exhausted() {
            input.reset(&first_state);
            parse_justify_items_value(input)?
        } else {
            parse_justify_items_value(input)?
        };
        input.expect_exhausted()?;
        Ok(Self { align, justify })
    }
}

impl<'i> Parse<'i> for FlexFlow {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut direction = None;
        let mut wrap = None;
        while !input.is_exhausted() {
            if direction.is_none()
                && let Ok(value) = input.try_parse(FlexDirection::parse)
            {
                direction = Some(value);
                continue;
            }
            if wrap.is_none()
                && let Ok(value) = input.try_parse(FlexWrap::parse)
            {
                wrap = Some(value);
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        if direction.is_none() && wrap.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Self {
            direction: direction.unwrap_or(FlexDirection::Row),
            wrap: wrap.unwrap_or(FlexWrap::Nowrap),
        })
    }
}

impl<'i> Parse<'i> for Flex<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            input.expect_exhausted()?;
            return Ok(Self {
                basis: store_node(LengthPercentageOrAuto::Auto, input),
                grow: 0.0,
                shrink: 0.0,
            });
        }
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            input.expect_exhausted()?;
            return Ok(Self {
                basis: store_node(LengthPercentageOrAuto::Auto, input),
                grow: 1.0,
                shrink: 1.0,
            });
        }

        let mut grow = None;
        let mut shrink = None;
        let mut basis = None;

        while !input.is_exhausted() {
            if grow.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_number)
            {
                grow = Some(value);
                continue;
            }
            if grow.is_some()
                && shrink.is_none()
                && let Ok(value) = input.try_parse(Compiler::expect_number)
            {
                shrink = Some(value);
                continue;
            }
            if basis.is_none()
                && let Ok(value) = input.try_parse(LengthPercentageOrAuto::parse)
            {
                basis = Some(value);
                continue;
            }
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }

        let grow = grow.unwrap_or(1.0);
        let shrink = shrink.unwrap_or(1.0);
        let basis = basis.unwrap_or_else(|| {
            LengthPercentageOrAuto::LengthPercentage(store_node(LengthPercentage::Zero, input))
        });
        input.expect_exhausted()?;
        Ok(Self {
            basis: store_node(basis, input),
            grow,
            shrink,
        })
    }
}

impl<'i> Parse<'i> for Overflow {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let x = OverflowKeyword::parse(input)?;
        let y = input
            .try_parse(OverflowKeyword::parse)
            .unwrap_or_else(|_| clone_overflow_keyword(&x));
        input.expect_exhausted()?;
        Ok(Self { x, y })
    }
}

fn clone_overflow_keyword(value: &OverflowKeyword) -> OverflowKeyword {
    match value {
        OverflowKeyword::Visible => OverflowKeyword::Visible,
        OverflowKeyword::Hidden => OverflowKeyword::Hidden,
        OverflowKeyword::Clip => OverflowKeyword::Clip,
        OverflowKeyword::Scroll => OverflowKeyword::Scroll,
        OverflowKeyword::Auto => OverflowKeyword::Auto,
    }
}

impl<'i> Parse<'i> for AspectRatio {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let auto = input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok();
        let ratio = input.try_parse(parse_ratio).ok();
        if !auto && ratio.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.expect_exhausted()?;
        Ok(Self { auto, ratio })
    }
}

fn parse_ratio<'i>(input: &mut Compiler<'i>) -> Result<Ratio, ParseError<'i, ParserError<'i>>> {
    let numerator = input.expect_number()?;
    let denominator = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(input.expect_number()?)
    } else {
        None
    };
    Ok(Ratio::new(numerator, denominator))
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

fn clone_length_percentage_or_auto<'i>(
    id: NodeId<'i, LengthPercentageOrAuto<'i>>,
    input: &mut Compiler<'i>,
) -> Option<NodeId<'i, LengthPercentageOrAuto<'i>>> {
    let length_percentage = match input.ast_context().node(id) {
        LengthPercentageOrAuto::Auto => None,
        LengthPercentageOrAuto::LengthPercentage(value) => Some(value),
    };
    let value = match length_percentage {
        None => LengthPercentageOrAuto::Auto,
        Some(value) => {
            LengthPercentageOrAuto::LengthPercentage(clone_length_percentage(value, input)?)
        }
    };
    Some(store_node(value, input))
}

fn parse_four_box_values<'i>(
    input: &mut Compiler<'i>,
) -> Result<[NodeId<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    parse_four_box_values_with(input, LengthPercentageOrAuto::parse)
}

fn parse_four_box_values_without_auto<'i>(
    input: &mut Compiler<'i>,
) -> Result<[NodeId<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    parse_four_box_values_with(input, |input| {
        let value = LengthPercentageOrAuto::parse(input)?;
        if matches!(value, LengthPercentageOrAuto::Auto) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(value)
    })
}

fn parse_four_box_values_with<'i>(
    input: &mut Compiler<'i>,
    mut parse: impl FnMut(
        &mut Compiler<'i>,
    ) -> Result<LengthPercentageOrAuto<'i>, ParseError<'i, ParserError<'i>>>,
) -> Result<[NodeId<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<NodeId<'i, LengthPercentageOrAuto<'i>>>; 4] = [None; 4];
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
        1 => clone_length_percentage_or_auto(top, input),
        _ => values[1].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let bottom = match count {
        1 | 2 => clone_length_percentage_or_auto(top, input),
        _ => values[2].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let left = match count {
        1 => clone_length_percentage_or_auto(top, input),
        2 | 3 => clone_length_percentage_or_auto(right, input),
        _ => values[3].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    Ok([top, right, bottom, left])
}

fn parse_logical_box_values<'i>(
    input: &mut Compiler<'i>,
    allow_auto: bool,
) -> Result<[NodeId<'i, LengthPercentageOrAuto<'i>>; 2], ParseError<'i, ParserError<'i>>> {
    let parse = |input: &mut Compiler<'i>| {
        let value = LengthPercentageOrAuto::parse(input)?;
        if !allow_auto && matches!(value, LengthPercentageOrAuto::Auto) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(value)
    };
    let first = store_node(parse(input)?, input);
    let second = if input.is_exhausted() {
        clone_length_percentage_or_auto(first, input)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?
    } else {
        store_node(parse(input)?, input)
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

macro_rules! parse_physical_box_values {
    ($($ty:ident),+ $(,)?) => {
        $(
            impl<'i> Parse<'i> for $ty<'i> {
                fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                    let [top, right, bottom, left] = parse_four_box_values(input)?;
                    Ok(Self { top, right, bottom, left })
                }
            }
        )+
    };
}

parse_physical_box_values!(Inset, Margin);

impl<'i> Parse<'i> for ScrollMargin<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl<'i> Parse<'i> for ScrollPadding<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

impl<'i> Parse<'i> for Padding<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }
}

macro_rules! parse_logical_box_values {
    ($($ty:ident, $first:ident, $second:ident, $allow_auto:expr),+ $(,)?) => {
        $(
            impl<'i> Parse<'i> for $ty<'i> {
                fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                    let [$first, $second] = parse_logical_box_values(input, $allow_auto)?;
                    Ok(Self { $first, $second })
                }
            }
        )+
    };
}

parse_logical_box_values!(
    InsetBlock,
    block_start,
    block_end,
    true,
    InsetInline,
    inline_start,
    inline_end,
    true,
    MarginBlock,
    block_start,
    block_end,
    true,
    MarginInline,
    inline_start,
    inline_end,
    true,
    PaddingBlock,
    block_start,
    block_end,
    false,
    PaddingInline,
    inline_start,
    inline_end,
    false,
    ScrollMarginBlock,
    block_start,
    block_end,
    false,
    ScrollMarginInline,
    inline_start,
    inline_end,
    false,
    ScrollPaddingBlock,
    block_start,
    block_end,
    false,
    ScrollPaddingInline,
    inline_start,
    inline_end,
    false,
);

use crate::prelude::*;

impl<'i> Parse<'i> for Display {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let mut outside = None;
        let mut inside = None;
        let mut is_list_item = false;

        while !input.is_exhausted() {
            let ident = input.expect_ident()?;
            let keyword = match_ignore_ascii_case!(
                ident,
                "none" => Some(DisplayKeyword::None),
                "contents" => Some(DisplayKeyword::Contents),
                "table-row-group" => Some(DisplayKeyword::TableRowGroup),
                "table-header-group" => Some(DisplayKeyword::TableHeaderGroup),
                "table-footer-group" => Some(DisplayKeyword::TableFooterGroup),
                "table-row" => Some(DisplayKeyword::TableRow),
                "table-cell" => Some(DisplayKeyword::TableCell),
                "table-column-group" => Some(DisplayKeyword::TableColumnGroup),
                "table-column" => Some(DisplayKeyword::TableColumn),
                "table-caption" => Some(DisplayKeyword::TableCaption),
                "ruby-base" => Some(DisplayKeyword::RubyBase),
                "ruby-text" => Some(DisplayKeyword::RubyText),
                "ruby-base-container" => Some(DisplayKeyword::RubyBaseContainer),
                "ruby-text-container" => Some(DisplayKeyword::RubyTextContainer),
                _ => None,
            );
            if let Some(keyword) = keyword {
                if outside.is_some() || inside.is_some() || is_list_item {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                if !input.is_exhausted() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                return Ok(Display::Keyword(keyword));
            }

            match_ignore_ascii_case!(
                ident,
                "block" => set_once(input, &mut outside, DisplayOutside::Block)?,
                "inline" => set_once(input, &mut outside, DisplayOutside::Inline)?,
                "run-in" => set_once(input, &mut outside, DisplayOutside::RunIn)?,
                "flow" => set_once(input, &mut inside, DisplayInside::Flow)?,
                "flow-root" => set_once(input, &mut inside, DisplayInside::FlowRoot)?,
                "table" => set_once(input, &mut inside, DisplayInside::Table)?,
                "flex" => set_once(
                    input,
                    &mut inside,
                    DisplayInside::Flex {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                )?,
                "-webkit-box" => set_once(
                    input,
                    &mut inside,
                    DisplayInside::Box {
                        vendor_prefix: VendorPrefix::WEBKIT,
                    },
                )?,
                "-moz-box" => set_once(
                    input,
                    &mut inside,
                    DisplayInside::Box {
                        vendor_prefix: VendorPrefix::MOZ,
                    },
                )?,
                "grid" => set_once(input, &mut inside, DisplayInside::Grid)?,
                "ruby" => set_once(input, &mut inside, DisplayInside::Ruby)?,
                "list-item" => {
                    if is_list_item {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    is_list_item = true;
                },
                "inline-flex" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::Flex {
                        vendor_prefix: VendorPrefix::NONE,
                    });
                },
                "inline-grid" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::Grid);
                },
                "inline-table" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::Table);
                },
                "inline-block" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::FlowRoot);
                },
                "-webkit-inline-box" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::Box {
                        vendor_prefix: VendorPrefix::WEBKIT,
                    });
                },
                "-moz-inline-box" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(DisplayInside::Box {
                        vendor_prefix: VendorPrefix::MOZ,
                    });
                },
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }

        if outside.is_none() && inside.is_none() && !is_list_item {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let outside = outside.unwrap_or({
            if matches!(inside, Some(DisplayInside::Ruby)) {
                DisplayOutside::Inline
            } else {
                DisplayOutside::Block
            }
        });
        let inside = inside.unwrap_or(DisplayInside::Flow);
        if is_list_item && !matches!(inside, DisplayInside::Flow | DisplayInside::FlowRoot) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(Display::Pair {
            inside,
            is_list_item,
            outside,
        })
    }
}

fn set_once<'i, T>(
    input: &mut Compiler<'i>,
    slot: &mut Option<T>,
    value: T,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if slot.is_some() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    *slot = Some(value);
    Ok(())
}

impl<'i> Parse<'i> for Visibility {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "visible" => Ok(Visibility::Visible),
            "hidden" => Ok(Visibility::Hidden),
            "collapse" => Ok(Visibility::Collapse),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for BoxSizing {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "content-box" => Ok(Self::ContentBox),
            "border-box" => Ok(Self::BorderBox),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for OverflowKeyword {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "visible" => Ok(Self::Visible),
            "hidden" => Ok(Self::Hidden),
            "clip" => Ok(Self::Clip),
            "scroll" => Ok(Self::Scroll),
            "auto" => Ok(Self::Auto),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
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

impl<'i> Parse<'i> for LengthPercentageOrAuto<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        LengthPercentage::parse(input)
            .map(|value| Self::LengthPercentage(input.allocator().boxed(value)))
    }
}

impl<'i> Parse<'i> for PositionProperty {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let ident = input.expect_ident()?;
        match_ignore_ascii_case!(
            ident,
            "static" => Ok(Self::Static),
            "relative" => Ok(Self::Relative),
            "absolute" => Ok(Self::Absolute),
            "sticky" => Ok(Self::Sticky(VendorPrefix::NONE)),
            "-webkit-sticky" => Ok(Self::Sticky(VendorPrefix::WEBKIT)),
            "-moz-sticky" => Ok(Self::Sticky(VendorPrefix::MOZ)),
            "fixed" => Ok(Self::Fixed),
            _ => Err(input.new_custom_error(ParserError::InvalidValue)),
        )
    }
}

impl<'i> Parse<'i> for ZIndex {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(Self::Auto);
        }
        Ok(Self::Integer(input.expect_integer()?))
    }
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

fn clone_length_percentage_or_auto<'i>(
    value: &LengthPercentageOrAuto<'i>,
    allocator: &'i Allocator,
) -> Option<Box<'i, LengthPercentageOrAuto<'i>>> {
    let value = match value {
        LengthPercentageOrAuto::Auto => LengthPercentageOrAuto::Auto,
        LengthPercentageOrAuto::LengthPercentage(value) => {
            LengthPercentageOrAuto::LengthPercentage(clone_length_percentage(value, allocator)?)
        }
    };
    Some(allocator.boxed(value))
}

fn parse_four_box_values<'i>(
    input: &mut Compiler<'i>,
) -> Result<[Box<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    parse_four_box_values_with(input, LengthPercentageOrAuto::parse)
}

fn parse_four_box_values_without_auto<'i>(
    input: &mut Compiler<'i>,
) -> Result<[Box<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
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
) -> Result<[Box<'i, LengthPercentageOrAuto<'i>>; 4], ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let mut values: [Option<Box<'i, LengthPercentageOrAuto<'i>>>; 4] =
        std::array::from_fn(|_| None);
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
        1 => clone_length_percentage_or_auto(&top, allocator),
        _ => values[1].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let bottom = match count {
        1 | 2 => clone_length_percentage_or_auto(&top, allocator),
        _ => values[2].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let left = match count {
        1 => clone_length_percentage_or_auto(&top, allocator),
        2 | 3 => clone_length_percentage_or_auto(&right, allocator),
        _ => values[3].take(),
    }
    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    Ok([top, right, bottom, left])
}

fn parse_logical_box_values<'i>(
    input: &mut Compiler<'i>,
    allow_auto: bool,
) -> Result<[Box<'i, LengthPercentageOrAuto<'i>>; 2], ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    let parse = |input: &mut Compiler<'i>| {
        let value = LengthPercentageOrAuto::parse(input)?;
        if !allow_auto && matches!(value, LengthPercentageOrAuto::Auto) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(value)
    };
    let first = allocator.boxed(parse(input)?);
    let second = if input.is_exhausted() {
        clone_length_percentage_or_auto(&first, allocator)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?
    } else {
        allocator.boxed(parse(input)?)
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

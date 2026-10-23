//! Box model and display parsing into the flattened AST.

use super::prelude::*;

impl<'i> Parse<'i> for Display<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let span = token_span(input);
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
                return Ok(input
                    .ast_mut()
                    .alloc_display(DisplayData::Keyword(keyword), span));
            }

            match_ignore_ascii_case!(
                ident,
                "block" => set_once(input, &mut outside, DisplayOutside::Block)?,
                "inline" => set_once(input, &mut outside, DisplayOutside::Inline)?,
                "run-in" => set_once(input, &mut outside, DisplayOutside::RunIn)?,
                "flow" => set_inside(input, &mut inside, DisplayInsideData::Flow, span)?,
                "flow-root" => {
                    set_inside(input, &mut inside, DisplayInsideData::FlowRoot, span)?
                },
                "table" => set_inside(input, &mut inside, DisplayInsideData::Table, span)?,
                "flex" => set_inside(
                    input,
                    &mut inside,
                    DisplayInsideData::Flex {
                        vendor_prefix: VendorPrefix::NONE,
                    },
                    span,
                )?,
                "-webkit-box" => set_inside(
                    input,
                    &mut inside,
                    DisplayInsideData::Box {
                        vendor_prefix: VendorPrefix::WEBKIT,
                    },
                    span,
                )?,
                "-moz-box" => set_inside(
                    input,
                    &mut inside,
                    DisplayInsideData::Box {
                        vendor_prefix: VendorPrefix::MOZ,
                    },
                    span,
                )?,
                "grid" => set_inside(input, &mut inside, DisplayInsideData::Grid, span)?,
                "ruby" => set_inside(input, &mut inside, DisplayInsideData::Ruby, span)?,
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
                    inside = Some(input.ast_mut().alloc_display_inside(
                        DisplayInsideData::Flex {
                            vendor_prefix: VendorPrefix::NONE,
                        },
                        span,
                    ));
                },
                "inline-grid" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(
                        input
                            .ast_mut()
                            .alloc_display_inside(DisplayInsideData::Grid, span),
                    );
                },
                "inline-table" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(
                        input
                            .ast_mut()
                            .alloc_display_inside(DisplayInsideData::Table, span),
                    );
                },
                "inline-block" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(input.ast_mut().alloc_display_inside(
                        DisplayInsideData::FlowRoot,
                        span,
                    ));
                },
                "-webkit-inline-box" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(input.ast_mut().alloc_display_inside(
                        DisplayInsideData::Box {
                            vendor_prefix: VendorPrefix::WEBKIT,
                        },
                        span,
                    ));
                },
                "-moz-inline-box" => {
                    if outside.is_some() || inside.is_some() {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                    outside = Some(DisplayOutside::Inline);
                    inside = Some(input.ast_mut().alloc_display_inside(
                        DisplayInsideData::Box {
                            vendor_prefix: VendorPrefix::MOZ,
                        },
                        span,
                    ));
                },
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
        }

        if outside.is_none() && inside.is_none() && !is_list_item {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let outside = outside.unwrap_or({
            if inside
                .is_some_and(|inside| matches!(inside.data(input.ast()), DisplayInsideData::Ruby))
            {
                DisplayOutside::Inline
            } else {
                DisplayOutside::Block
            }
        });
        let inside = match inside {
            Some(inside) => inside,
            None => input
                .ast_mut()
                .alloc_display_inside(DisplayInsideData::Flow, span),
        };
        if is_list_item
            && !matches!(
                inside.data(input.ast()),
                DisplayInsideData::Flow | DisplayInsideData::FlowRoot
            )
        {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(input.ast_mut().alloc_display(
            DisplayData::Pair {
                inside,
                is_list_item,
                outside,
            },
            span,
        ))
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

fn set_inside<'i>(
    input: &mut Compiler<'i>,
    slot: &mut Option<DisplayInside<'i>>,
    value: DisplayInsideData,
    span: Span,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    if slot.is_some() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    *slot = Some(input.ast_mut().alloc_display_inside(value, span));
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

impl<'i> Parse<'i> for Overflow<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let x = OverflowKeyword::parse(input)?;
        let y = input.try_parse(OverflowKeyword::parse).unwrap_or(x);
        input.expect_exhausted()?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_overflow(OverflowData { x, y }, span)))
    }
}

impl<'i> Parse<'i> for AspectRatio<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let auto = input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok();
        let ratio = input.try_parse(parse_ratio).ok();
        if !auto && ratio.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.expect_exhausted()?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_aspect_ratio(AspectRatioData { auto, ratio }, span)))
    }
}

fn parse_ratio<'i>(input: &mut Compiler<'i>) -> Result<Ratio<'i>, ParseError<'i, ParserError<'i>>> {
    let numerator = input.expect_number()?;
    let denominator = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(input.expect_number()?)
    } else {
        None
    };
    Ok(with_token_span!(input, |span| input.ast_mut().alloc_ratio(
        RatioData {
            denominator,
            numerator,
        },
        span,
    )))
}

impl<'i> Parse<'i> for LengthPercentageOrAuto<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        if input
            .try_parse(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(with_token_span!(input, |span| input
                .ast_mut()
                .alloc_length_percentage_or_auto(
                    LengthPercentageOrAutoData::Auto,
                    span
                )));
        }
        let value = <LengthPercentage>::parse(input)?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_length_percentage_or_auto(
                LengthPercentageOrAutoData::LengthPercentage(value),
                span,
            )))
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

fn parse_four_box_values<'i>(
    input: &mut Compiler<'i>,
) -> Result<[LengthPercentageOrAuto<'i>; 4], ParseError<'i, ParserError<'i>>> {
    parse_four_box_values_with(input, LengthPercentageOrAuto::parse)
}

fn parse_four_box_values_without_auto<'i>(
    input: &mut Compiler<'i>,
) -> Result<[LengthPercentageOrAuto<'i>; 4], ParseError<'i, ParserError<'i>>> {
    parse_four_box_values_with(input, |input| {
        let value = LengthPercentageOrAuto::parse(input)?;
        if matches!(value.data(input.ast()), LengthPercentageOrAutoData::Auto) {
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
) -> Result<[LengthPercentageOrAuto<'i>; 4], ParseError<'i, ParserError<'i>>> {
    let mut values: [Option<LengthPercentageOrAuto<'i>>; 4] = [None; 4];
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

fn parse_logical_box_values<'i>(
    input: &mut Compiler<'i>,
    allow_auto: bool,
) -> Result<[LengthPercentageOrAuto<'i>; 2], ParseError<'i, ParserError<'i>>> {
    let parse = |input: &mut Compiler<'i>| {
        let value = LengthPercentageOrAuto::parse(input)?;
        if !allow_auto && matches!(value.data(input.ast()), LengthPercentageOrAutoData::Auto) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(value)
    };
    let first = parse(input)?;
    let second = if input.is_exhausted() {
        first
    } else {
        parse(input)?
    };
    input.expect_exhausted()?;
    Ok([first, second])
}

macro_rules! parse_physical_box_values {
    ($($ty:ident, $data:ident => $alloc:ident),+ $(,)?) => {
        $(
            impl<'i> Parse<'i> for $ty<'i> {
                fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                    let [top, right, bottom, left] = parse_four_box_values(input)?;
                    let data = $data { top, right, bottom, left };
                    Ok(with_token_span!(input, |span| input.ast_mut().$alloc(data, span)))
                }
            }
        )+
    };
}

parse_physical_box_values!(
    Inset, InsetData => alloc_inset,
    Margin, MarginData => alloc_margin,
);

impl<'i> Parse<'i> for ScrollMargin<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_scroll_margin(
                ScrollMarginData {
                    top,
                    right,
                    bottom,
                    left,
                },
                span,
            )))
    }
}

impl<'i> Parse<'i> for ScrollPadding<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_scroll_padding(
                ScrollPaddingData {
                    top,
                    right,
                    bottom,
                    left,
                },
                span,
            )))
    }
}

impl<'i> Parse<'i> for Padding<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let [top, right, bottom, left] = parse_four_box_values_without_auto(input)?;
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_padding(
                PaddingData {
                    top,
                    right,
                    bottom,
                    left,
                },
                span,
            )))
    }
}

macro_rules! parse_logical_box_values {
    ($($ty:ident, $data:ident => $alloc:ident, $first:ident, $second:ident, $allow_auto:expr),+ $(,)?) => {
        $(
            impl<'i> Parse<'i> for $ty<'i> {
                fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
                    let [$first, $second] = parse_logical_box_values(input, $allow_auto)?;
                    let data = $data { $first, $second };
                    Ok(with_token_span!(input, |span| input.ast_mut().$alloc(data, span)))
                }
            }
        )+
    };
}

parse_logical_box_values!(
    InsetBlock, InsetBlockData => alloc_inset_block, block_start, block_end, true,
    InsetInline, InsetInlineData => alloc_inset_inline, inline_start, inline_end, true,
    MarginBlock, MarginBlockData => alloc_margin_block, block_start, block_end, true,
    MarginInline, MarginInlineData => alloc_margin_inline, inline_start, inline_end, true,
    PaddingBlock, PaddingBlockData => alloc_padding_block, block_start, block_end, false,
    PaddingInline, PaddingInlineData => alloc_padding_inline, inline_start, inline_end, false,
    ScrollMarginBlock, ScrollMarginBlockData => alloc_scroll_margin_block, block_start, block_end, false,
    ScrollMarginInline, ScrollMarginInlineData => alloc_scroll_margin_inline, inline_start, inline_end, false,
    ScrollPaddingBlock, ScrollPaddingBlockData => alloc_scroll_padding_block, block_start, block_end, false,
    ScrollPaddingInline, ScrollPaddingInlineData => alloc_scroll_padding_inline, inline_start, inline_end, false,
);

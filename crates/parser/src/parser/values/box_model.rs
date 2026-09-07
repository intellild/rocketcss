use super::{collect_tokens, token_values_contain_opaque};
use crate::parser::length::parse_length_unit;
use crate::parser::values::parse_two_nodes;
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

keyword_parse!(BoxDecorationBreak, "slice" => Self::Slice, "clone" => Self::Clone,);
keyword_parse!(TextOverflow, "clip" => Self::Clip, "ellipsis" => Self::Ellipsis,);

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

impl<'i> Parse<'i> for Size<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let location = input.current_source_location();
        let token = *input.next()?;
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("auto") => Ok(Size::Auto),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
                Ok(Size::MinContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
                Ok(Size::MaxContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
                Ok(Size::FitContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => Ok(Size::Stretch {
                vendor_prefix: VendorPrefix::NONE,
            }),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("contain") => Ok(Size::Contain),
            ValueToken::Function(name) if name.eq_ignore_ascii_case("fit-content") => {
                let value = input.parse_nested_block(|input| {
                    let value = LengthPercentage::parse(input)?;
                    input.expect_exhausted()?;
                    Ok(value)
                })?;
                Ok(Size::FitContentFunction(store_node(value, input)))
            }
            ValueToken::Function(name) if KnownFunction::from_name(name).is_math() => {
                let arguments =
                    input.parse_nested_block(|input| collect_tokens(input, allocator, 1))?;
                if token_values_contain_opaque(input.ast_context(), &arguments) {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                let arguments = store_vec(arguments, input);
                Ok(Size::MathFunction(store_node(
                    Function::new(name, arguments, input.ast_context_mut()),
                    input,
                )))
            }
            ValueToken::Percentage(value) => Ok(Size::LengthPercentage(store_node(
                DimensionPercentage::Percentage(value),
                input,
            ))),
            ValueToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(Size::LengthPercentage(store_node(
                    DimensionPercentage::Dimension(LengthValue { unit, value }),
                    input,
                )))
            }
            ValueToken::Number(0.0) => Ok(Size::LengthPercentage(store_node(
                DimensionPercentage::Dimension(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                }),
                input,
            ))),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

impl<'i> Parse<'i> for MaxSize<'i> {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let location = input.current_source_location();
        let token = *input.next()?;
        match token {
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("none") => Ok(Self::None),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("min-content") => {
                Ok(Self::MinContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("max-content") => {
                Ok(Self::MaxContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("fit-content") => {
                Ok(Self::FitContent {
                    vendor_prefix: VendorPrefix::NONE,
                })
            }
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("stretch") => Ok(Self::Stretch {
                vendor_prefix: VendorPrefix::NONE,
            }),
            ValueToken::Ident(name) if name.eq_ignore_ascii_case("contain") => Ok(Self::Contain),
            ValueToken::Function(name) if name.eq_ignore_ascii_case("fit-content") => {
                let value = input.parse_nested_block(|input| {
                    let value = LengthPercentage::parse(input)?;
                    input.expect_exhausted()?;
                    Ok(value)
                })?;
                Ok(Self::FitContentFunction(store_node(value, input)))
            }
            ValueToken::Function(name) if KnownFunction::from_name(name).is_math() => {
                let arguments =
                    input.parse_nested_block(|input| collect_tokens(input, allocator, 1))?;
                if token_values_contain_opaque(input.ast_context(), &arguments) {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                let arguments = store_vec(arguments, input);
                Ok(Self::MathFunction(store_node(
                    Function::new(name, arguments, input.ast_context_mut()),
                    input,
                )))
            }
            ValueToken::Percentage(value) => Ok(Self::LengthPercentage(store_node(
                DimensionPercentage::Percentage(value),
                input,
            ))),
            ValueToken::Dimension { unit, value } => {
                let unit = parse_length_unit(&unit)
                    .ok_or_else(|| location.new_custom_error(ParserError::InvalidValue))?;
                Ok(Self::LengthPercentage(store_node(
                    DimensionPercentage::Dimension(LengthValue { unit, value }),
                    input,
                )))
            }
            ValueToken::Number(0.0) => Ok(Self::LengthPercentage(store_node(
                DimensionPercentage::Dimension(LengthValue {
                    unit: LengthUnit::Px,
                    value: 0.0,
                }),
                input,
            ))),
            _ => Err(location.new_custom_error(ParserError::InvalidValue)),
        }
    }
}

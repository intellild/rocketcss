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

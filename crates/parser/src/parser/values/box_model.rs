use crate::prelude::*;

impl<'i> Parse<'i> for Display<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        let ident = input.expect_ident()?;
        let (outside, inside, is_list_item) = match_ignore_ascii_case!(
            ident,
            "none" => return Ok(Display::Keyword(DisplayKeyword::None)),
            "contents" => return Ok(Display::Keyword(DisplayKeyword::Contents)),
            "block" => (DisplayOutside::Block, DisplayInside::Flow, false),
            "inline" => (DisplayOutside::Inline, DisplayInside::Flow, false),
            "flow-root" => (DisplayOutside::Block, DisplayInside::FlowRoot, false),
            "flex" => (
                DisplayOutside::Block,
                DisplayInside::Flex {
                    vendor_prefix: VendorPrefix::NONE,
                },
                false,
            ),
            "inline-flex" => (
                DisplayOutside::Inline,
                DisplayInside::Flex {
                    vendor_prefix: VendorPrefix::NONE,
                },
                false,
            ),
            "grid" => (DisplayOutside::Block, DisplayInside::Grid, false),
            "inline-grid" => (DisplayOutside::Inline, DisplayInside::Grid, false),
            "list-item" => (DisplayOutside::Block, DisplayInside::Flow, true),
            _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
        );
        Ok(Display::Pair {
            inside: allocator.boxed(inside),
            is_list_item,
            outside,
        })
    }
}

impl<'i> Parse<'i> for Visibility {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
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

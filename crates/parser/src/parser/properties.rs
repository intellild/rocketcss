use super::{
    color::parse_color,
    length::parse_size,
    values::{
        collect_tokens, css_wide_keyword, remove_important, single_token, token_ident,
        trim_leading_whitespace,
    },
};
use crate::prelude::*;

pub(super) fn parse_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
        collect_tokens(input, allocator, depth + 1)
    })?;
    let _ = input.try_parse(Parser::expect_semicolon);
    let important = remove_important(&mut value);
    trim_leading_whitespace(&mut value);

    let declaration = if name.starts_with("--") {
        Declaration::Custom(allocator.boxed(CustomProperty {
            name: allocator.boxed(CustomPropertyName::Custom(name)),
            value,
        }))
    } else if let Some(declaration) = parse_typed_declaration(name, &value, allocator) {
        declaration
    } else if name.eq_ignore_ascii_case("all") && value.len() == 1 {
        match token_ident(&value[0]).and_then(css_wide_keyword) {
            Some(keyword) => Declaration::All(keyword),
            None => unparsed_declaration(name, value, allocator),
        }
    } else {
        unparsed_declaration(name, value, allocator)
    };

    Ok((declaration, important))
}

pub(super) fn unparsed_declaration<'i>(
    name: &'i str,
    value: Vec<'i, TokenOrValue<'i>>,
    allocator: &'i Allocator,
) -> Declaration<'i> {
    Declaration::Unparsed(allocator.boxed(UnparsedProperty {
        property_id: allocator.boxed(PropertyId::from_name(name)),
        value,
    }))
}

pub(super) fn parse_typed_declaration<'i>(
    name: &str,
    value: &[TokenOrValue<'i>],
    allocator: &'i Allocator,
) -> Option<Declaration<'i>> {
    if name.eq_ignore_ascii_case("color") {
        return parse_color(value, allocator)
            .map(|color| Declaration::Color(allocator.boxed(color)));
    }
    if name.eq_ignore_ascii_case("background-color") {
        return parse_color(value, allocator)
            .map(|color| Declaration::BackgroundColor(allocator.boxed(color)));
    }
    if name.eq_ignore_ascii_case("opacity") {
        return match single_token(value)? {
            ValueToken::Number(value) => Some(Declaration::Opacity(*value)),
            ValueToken::Percentage(value) => Some(Declaration::Opacity(*value)),
            _ => None,
        };
    }
    if name.eq_ignore_ascii_case("visibility") {
        let value = token_ident(value.first()?)?;
        let visibility = if value.eq_ignore_ascii_case("visible") {
            Visibility::Visible
        } else if value.eq_ignore_ascii_case("hidden") {
            Visibility::Hidden
        } else if value.eq_ignore_ascii_case("collapse") {
            Visibility::Collapse
        } else {
            return None;
        };
        return Some(Declaration::Visibility(visibility));
    }
    if name.eq_ignore_ascii_case("display") {
        return parse_display(value, allocator)
            .map(|display| Declaration::Display(allocator.boxed(display)));
    }

    let size = parse_size(value, allocator)?;
    let size = allocator.boxed(size);
    if name.eq_ignore_ascii_case("width") {
        Some(Declaration::Width(size))
    } else if name.eq_ignore_ascii_case("height") {
        Some(Declaration::Height(size))
    } else if name.eq_ignore_ascii_case("min-width") {
        Some(Declaration::MinWidth(size))
    } else if name.eq_ignore_ascii_case("min-height") {
        Some(Declaration::MinHeight(size))
    } else if name.eq_ignore_ascii_case("block-size") {
        Some(Declaration::BlockSize(size))
    } else if name.eq_ignore_ascii_case("inline-size") {
        Some(Declaration::InlineSize(size))
    } else if name.eq_ignore_ascii_case("min-block-size") {
        Some(Declaration::MinBlockSize(size))
    } else if name.eq_ignore_ascii_case("min-inline-size") {
        Some(Declaration::MinInlineSize(size))
    } else {
        None
    }
}

pub(super) fn parse_display<'i>(
    value: &[TokenOrValue<'i>],
    allocator: &'i Allocator,
) -> Option<Display<'i>> {
    let ident = token_ident(value.first()?)?;
    if value.len() != 1 {
        return None;
    }
    if ident.eq_ignore_ascii_case("none") {
        return Some(Display::Keyword(DisplayKeyword::None));
    }
    if ident.eq_ignore_ascii_case("contents") {
        return Some(Display::Keyword(DisplayKeyword::Contents));
    }

    let (outside, inside, is_list_item) = if ident.eq_ignore_ascii_case("block") {
        (DisplayOutside::Block, DisplayInside::Flow, false)
    } else if ident.eq_ignore_ascii_case("inline") {
        (DisplayOutside::Inline, DisplayInside::Flow, false)
    } else if ident.eq_ignore_ascii_case("flow-root") {
        (DisplayOutside::Block, DisplayInside::FlowRoot, false)
    } else if ident.eq_ignore_ascii_case("flex") {
        (
            DisplayOutside::Block,
            DisplayInside::Flex {
                vendor_prefix: VendorPrefix::NONE,
            },
            false,
        )
    } else if ident.eq_ignore_ascii_case("inline-flex") {
        (
            DisplayOutside::Inline,
            DisplayInside::Flex {
                vendor_prefix: VendorPrefix::NONE,
            },
            false,
        )
    } else if ident.eq_ignore_ascii_case("grid") {
        (DisplayOutside::Block, DisplayInside::Grid, false)
    } else if ident.eq_ignore_ascii_case("inline-grid") {
        (DisplayOutside::Inline, DisplayInside::Grid, false)
    } else if ident.eq_ignore_ascii_case("list-item") {
        (DisplayOutside::Block, DisplayInside::Flow, true)
    } else {
        return None;
    };
    Some(Display::Pair {
        inside: allocator.boxed(inside),
        is_list_item,
        outside,
    })
}

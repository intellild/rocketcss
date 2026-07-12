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

    let declaration = if name.starts_with("--") {
        Declaration::Custom(allocator.boxed(CustomProperty {
            name: allocator.boxed(CustomPropertyName::Custom(name)),
            value,
        }))
    } else {
        trim_leading_whitespace(&mut value);
        if let Some(declaration) = parse_typed_declaration(name, &value, allocator) {
            declaration
        } else if name.eq_ignore_ascii_case("all") && value.len() == 1 {
            match token_ident(&value[0]).and_then(css_wide_keyword) {
                Some(keyword) => Declaration::All(keyword),
                None => unparsed_declaration(name, value, allocator),
            }
        } else {
            unparsed_declaration(name, value, allocator)
        }
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
        let visibility = match_ignore_ascii_case!(
            value,
            "visible" => Visibility::Visible,
            "hidden" => Visibility::Hidden,
            "collapse" => Visibility::Collapse,
            _ => return None,
        );
        return Some(Declaration::Visibility(visibility));
    }
    if name.eq_ignore_ascii_case("display") {
        return parse_display(value, allocator)
            .map(|display| Declaration::Display(allocator.boxed(display)));
    }
    if name.eq_ignore_ascii_case("z-index") {
        let z_index = match single_token(value)? {
            ValueToken::Ident(value) if value.eq_ignore_ascii_case("auto") => ZIndex::Auto,
            ValueToken::Number(value)
                if value.fract() == 0.0
                    && *value >= i32::MIN as f32
                    && *value < -(i32::MIN as f32) =>
            {
                ZIndex::Integer(*value as i32)
            }
            _ => return None,
        };
        return Some(Declaration::ZIndex(allocator.boxed(z_index)));
    }

    let size = parse_size(value, allocator)?;
    let size = allocator.boxed(size);
    match_ignore_ascii_case!(
        name,
        "width" => Some(Declaration::Width(size)),
        "height" => Some(Declaration::Height(size)),
        "min-width" => Some(Declaration::MinWidth(size)),
        "min-height" => Some(Declaration::MinHeight(size)),
        "block-size" => Some(Declaration::BlockSize(size)),
        "inline-size" => Some(Declaration::InlineSize(size)),
        "min-block-size" => Some(Declaration::MinBlockSize(size)),
        "min-inline-size" => Some(Declaration::MinInlineSize(size)),
        _ => None,
    )
}

pub(super) fn parse_display<'i>(
    value: &[TokenOrValue<'i>],
    allocator: &'i Allocator,
) -> Option<Display<'i>> {
    let ident = token_ident(value.first()?)?;
    if value.len() != 1 {
        return None;
    }
    let (outside, inside, is_list_item) = match_ignore_ascii_case!(
        ident,
        "none" => return Some(Display::Keyword(DisplayKeyword::None)),
        "contents" => return Some(Display::Keyword(DisplayKeyword::Contents)),
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
        _ => return None,
    );
    Some(Display::Pair {
        inside: allocator.boxed(inside),
        is_list_item,
        outside,
    })
}

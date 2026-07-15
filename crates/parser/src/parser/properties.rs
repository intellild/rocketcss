use super::values::{
    collect_tokens, css_wide_keyword, parse_font_family_list, remove_important,
    trim_leading_whitespace,
};
use crate::prelude::*;

pub(super) fn parse_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let property_id = PropertyId::from_name(name);

    if !name.starts_with("--") {
        let start = input.state();
        if let Some(Ok(declaration)) = try_parse_typed_declaration(input, &property_id, allocator)
            && let Some(important) = parse_declaration_end(input)
        {
            let _ = input.try_parse(Parser::expect_semicolon);
            return Ok((declaration, important));
        }
        input.reset(&start);
    }

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
        unparsed_declaration(property_id, value, allocator)
    };

    Ok((declaration, important))
}

pub(super) fn unparsed_declaration<'i>(
    property_id: PropertyId<'i>,
    value: Vec<'i, TokenOrValue<'i>>,
    allocator: &'i Allocator,
) -> Declaration<'i> {
    Declaration::Unparsed(allocator.boxed(UnparsedProperty {
        property_id: allocator.boxed(property_id),
        value,
    }))
}

fn try_parse_typed_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    property_id: &PropertyId<'i>,
    allocator: &'i Allocator,
) -> Option<Result<Declaration<'i>, ParseError<'i, ParserError<'i>>>> {
    let delimiters = Delimiter::Bang | Delimiter::Semicolon;
    macro_rules! parse {
        ($parser:expr) => {
            Some(input.parse_until_before(delimiters, $parser))
        };
    }

    match property_id {
        PropertyId::Color => parse!(|input| {
            CssColor::parse(input).map(|value| Declaration::Color(allocator.boxed(value)))
        }),
        PropertyId::BackgroundColor => parse!(|input| {
            CssColor::parse(input).map(|value| Declaration::BackgroundColor(allocator.boxed(value)))
        }),
        PropertyId::Opacity => {
            parse!(|input| parse_opacity(input).map(Declaration::Opacity))
        }
        PropertyId::Visibility => {
            parse!(|input| Visibility::parse(input).map(Declaration::Visibility))
        }
        PropertyId::Display => parse!(|input| {
            Display::parse(input).map(|value| Declaration::Display(allocator.boxed(value)))
        }),
        PropertyId::FontFamily => {
            parse!(|input| parse_font_family_list(input).map(Declaration::FontFamily))
        }
        PropertyId::ColumnRule(prefix) => parse!(|input| {
            ColumnRule::parse(input)
                .map(|value| Declaration::ColumnRule(allocator.boxed(value), *prefix))
        }),
        PropertyId::Columns(prefix) => parse!(|input| {
            Columns::parse(input).map(|value| Declaration::Columns(allocator.boxed(value), *prefix))
        }),
        PropertyId::GridColumnGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::GridColumnGap(allocator.boxed(value)))
        }),
        PropertyId::GridRowGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::GridRowGap(allocator.boxed(value)))
        }),
        property_id @ (PropertyId::Width
        | PropertyId::Height
        | PropertyId::MinWidth
        | PropertyId::MinHeight
        | PropertyId::BlockSize
        | PropertyId::InlineSize
        | PropertyId::MinBlockSize
        | PropertyId::MinInlineSize) => parse!(|input| {
            let value = allocator.boxed(Size::parse(input)?);
            Ok(match property_id {
                PropertyId::Width => Declaration::Width(value),
                PropertyId::Height => Declaration::Height(value),
                PropertyId::MinWidth => Declaration::MinWidth(value),
                PropertyId::MinHeight => Declaration::MinHeight(value),
                PropertyId::BlockSize => Declaration::BlockSize(value),
                PropertyId::InlineSize => Declaration::InlineSize(value),
                PropertyId::MinBlockSize => Declaration::MinBlockSize(value),
                PropertyId::MinInlineSize => Declaration::MinInlineSize(value),
                _ => unreachable!(),
            })
        }),
        PropertyId::All => parse!(|input| {
            let ident = input.expect_ident()?;
            css_wide_keyword(ident)
                .map(Declaration::All)
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))
        }),
        _ => None,
    }
}

fn parse_declaration_end<'i, 't>(input: &mut Parser<'i, 't>) -> Option<bool> {
    let important = input
        .try_parse(|input| {
            input.expect_delim('!')?;
            input.expect_ident_matching("important")
        })
        .is_ok();
    input
        .parse_until_before(Delimiter::Semicolon, |input| {
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })
        .ok()
        .map(|()| important)
}

fn parse_opacity<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<f32, ParseError<'i, ParserError<'i>>> {
    let location = input.current_source_location();
    match input.next()?.clone() {
        ValueToken::Number(value) | ValueToken::Percentage(value) => Ok(value),
        _ => Err(location.new_custom_error(ParserError::InvalidValue)),
    }
}

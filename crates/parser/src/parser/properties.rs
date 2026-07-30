use super::values::{
    collect_custom_property_tokens, collect_tokens, css_wide_keyword, parse_animation_list,
    parse_comma_separated, parse_font_family_list, remove_important, trim_leading_whitespace,
    value_contains_comment,
};
use crate::prelude::*;

pub(super) fn parse_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    name: &'i str,
    depth: usize,
) -> Result<(Declaration<'i>, bool), ParseError<'i, ParserError<'i>>> {
    let property_id = PropertyId::from_name(name);
    let mut typed_grammar_supported = false;

    if !name.starts_with("--") {
        let start = input.state();
        if property_id.known_id().is_some() {
            if let Ok(keyword) = input.parse_until_before(
                Delimiter::Bang | Delimiter::Semicolon,
                parse_css_wide_keyword,
            ) && let Some(important) = parse_declaration_end(input)
            {
                let _ = input.try_parse(Parser::expect_semicolon);
                let declaration = match property_id {
                    PropertyId::All => Declaration::All(keyword),
                    PropertyId::ColumnWidth(prefix) => {
                        Declaration::ColumnWidth(CSSWideOr::CSSWide(keyword), prefix)
                    }
                    PropertyId::ColumnCount(prefix) => {
                        Declaration::ColumnCount(CSSWideOr::CSSWide(keyword), prefix)
                    }
                    PropertyId::Columns(prefix) => {
                        Declaration::Columns(CSSWideOr::CSSWide(keyword), prefix)
                    }
                    _ => Declaration::CSSWide(allocator.boxed(property_id), keyword),
                };
                return Ok((declaration, important));
            }
            input.reset(&start);
        }

        let typed = try_parse_typed_declaration(input, &property_id, allocator, depth);
        typed_grammar_supported = typed.is_some();
        if let Some(Ok(declaration)) = typed
            && let Some(important) = parse_declaration_end(input)
        {
            let _ = input.try_parse(Parser::expect_semicolon);
            return Ok((declaration, important));
        }
        input.reset(&start);
    }

    let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
        if name.starts_with("--") {
            collect_custom_property_tokens(input, allocator, depth + 1)
        } else {
            collect_tokens(input, allocator, depth + 1)
        }
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
        let reason = unparsed_reason(&property_id, &value, typed_grammar_supported);
        unparsed_declaration(property_id, value, reason, allocator)
    };

    Ok((declaration, important))
}

pub(super) fn unparsed_declaration<'i>(
    property_id: PropertyId<'i>,
    value: Vec<'i, TokenOrValue<'i>>,
    reason: UnparsedPropertyReason,
    allocator: &'i Allocator,
) -> Declaration<'i> {
    Declaration::Unparsed(allocator.boxed(UnparsedProperty {
        property_id: allocator.boxed(property_id),
        reason,
        value,
    }))
}

fn unparsed_reason(
    property_id: &PropertyId<'_>,
    value: &[TokenOrValue<'_>],
    typed_grammar_supported: bool,
) -> UnparsedPropertyReason {
    if matches!(property_id, PropertyId::Custom(_)) {
        return UnparsedPropertyReason::UnknownProperty;
    }
    if !typed_grammar_supported {
        return UnparsedPropertyReason::UnsupportedGrammar;
    }
    // `background` currently has a typed fast path for color-only values, but
    // its full shorthand grammar is not implemented yet. A failed fast path
    // therefore means "unsupported grammar", not invalid syntax.
    if matches!(property_id, PropertyId::Background) {
        return UnparsedPropertyReason::UnsupportedGrammar;
    }
    if value.iter().any(token_value_is_opaque) {
        return UnparsedPropertyReason::OpaqueValue;
    }
    UnparsedPropertyReason::InvalidValue
}

fn token_value_is_opaque(value: &TokenOrValue<'_>) -> bool {
    match value {
        TokenOrValue::Function(_) | TokenOrValue::Var(_) | TokenOrValue::Env(_) => true,
        TokenOrValue::Token(token) => matches!(**token, ValueToken::Comment(_)),
        _ => false,
    }
}

fn try_parse_typed_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    property_id: &PropertyId<'i>,
    allocator: &'i Allocator,
    depth: usize,
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
        property_id @ (PropertyId::BorderTopColor
        | PropertyId::BorderBottomColor
        | PropertyId::BorderLeftColor
        | PropertyId::BorderRightColor
        | PropertyId::BorderBlockStartColor
        | PropertyId::BorderBlockEndColor
        | PropertyId::BorderInlineStartColor
        | PropertyId::BorderInlineEndColor
        | PropertyId::OutlineColor) => parse!(|input| {
            let value = allocator.boxed(CssColor::parse(input)?);
            Ok(match property_id {
                PropertyId::BorderTopColor => Declaration::BorderTopColor(value),
                PropertyId::BorderBottomColor => Declaration::BorderBottomColor(value),
                PropertyId::BorderLeftColor => Declaration::BorderLeftColor(value),
                PropertyId::BorderRightColor => Declaration::BorderRightColor(value),
                PropertyId::BorderBlockStartColor => Declaration::BorderBlockStartColor(value),
                PropertyId::BorderBlockEndColor => Declaration::BorderBlockEndColor(value),
                PropertyId::BorderInlineStartColor => Declaration::BorderInlineStartColor(value),
                PropertyId::BorderInlineEndColor => Declaration::BorderInlineEndColor(value),
                PropertyId::OutlineColor => Declaration::OutlineColor(value),
                _ => unreachable!(),
            })
        }),
        PropertyId::TextDecorationColor(prefix) => parse!(|input| {
            CssColor::parse(input)
                .map(|value| Declaration::TextDecorationColor(allocator.boxed(value), *prefix))
        }),
        PropertyId::TextEmphasisColor(prefix) => parse!(|input| {
            CssColor::parse(input)
                .map(|value| Declaration::TextEmphasisColor(allocator.boxed(value), *prefix))
        }),
        PropertyId::Background => parse!(|input| {
            let mut values = allocator.vec();
            values.push(Background::parse(input)?);
            Ok(Declaration::Background(values))
        }),
        PropertyId::Opacity => {
            parse!(|input| parse_opacity(input).map(Declaration::Opacity))
        }
        PropertyId::Visibility => {
            parse!(|input| Visibility::parse(input).map(Declaration::Visibility))
        }
        PropertyId::Display => parse!(|input| { Display::parse(input).map(Declaration::Display) }),
        PropertyId::FontFamily => {
            parse!(|input| parse_font_family_list(input, depth).map(Declaration::FontFamily))
        }
        PropertyId::ColumnRule(prefix) => parse!(|input| {
            ColumnRule::parse(input)
                .map(|value| Declaration::ColumnRule(allocator.boxed(value), *prefix))
        }),
        PropertyId::ColumnWidth(prefix) => parse!(|input| {
            if let Ok(keyword) = input.try_parse(parse_css_wide_keyword) {
                return Ok(Declaration::ColumnWidth(
                    CSSWideOr::CSSWide(keyword),
                    *prefix,
                ));
            }
            ColumnWidth::parse(input)
                .map(|value| Declaration::ColumnWidth(CSSWideOr::Value(value), *prefix))
        }),
        PropertyId::ColumnCount(prefix) => parse!(|input| {
            if let Ok(keyword) = input.try_parse(parse_css_wide_keyword) {
                return Ok(Declaration::ColumnCount(
                    CSSWideOr::CSSWide(keyword),
                    *prefix,
                ));
            }
            ColumnCount::parse(input)
                .map(|value| Declaration::ColumnCount(CSSWideOr::Value(value), *prefix))
        }),
        PropertyId::Columns(prefix) => parse!(|input| {
            if let Ok(keyword) = input.try_parse(parse_css_wide_keyword) {
                return Ok(Declaration::Columns(CSSWideOr::CSSWide(keyword), *prefix));
            }
            Columns::parse(input).map(|value| {
                Declaration::Columns(CSSWideOr::Value(allocator.boxed(value)), *prefix)
            })
        }),
        PropertyId::GridColumnGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::GridColumnGap(allocator.boxed(value)))
        }),
        PropertyId::GridRowGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::GridRowGap(allocator.boxed(value)))
        }),
        PropertyId::RowGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::RowGap(allocator.boxed(value)))
        }),
        PropertyId::ColumnGap => parse!(|input| {
            GapValue::parse(input).map(|value| Declaration::ColumnGap(allocator.boxed(value)))
        }),
        property_id @ (PropertyId::BorderTopStyle
        | PropertyId::BorderBottomStyle
        | PropertyId::BorderLeftStyle
        | PropertyId::BorderRightStyle
        | PropertyId::BorderBlockStartStyle
        | PropertyId::BorderBlockEndStyle
        | PropertyId::BorderInlineStartStyle
        | PropertyId::BorderInlineEndStyle) => parse!(|input| {
            let value = LineStyle::parse(input)?;
            Ok(match property_id {
                PropertyId::BorderTopStyle => Declaration::BorderTopStyle(value),
                PropertyId::BorderBottomStyle => Declaration::BorderBottomStyle(value),
                PropertyId::BorderLeftStyle => Declaration::BorderLeftStyle(value),
                PropertyId::BorderRightStyle => Declaration::BorderRightStyle(value),
                PropertyId::BorderBlockStartStyle => Declaration::BorderBlockStartStyle(value),
                PropertyId::BorderBlockEndStyle => Declaration::BorderBlockEndStyle(value),
                PropertyId::BorderInlineStartStyle => Declaration::BorderInlineStartStyle(value),
                PropertyId::BorderInlineEndStyle => Declaration::BorderInlineEndStyle(value),
                _ => unreachable!(),
            })
        }),
        property_id @ (PropertyId::BorderTopWidth
        | PropertyId::BorderBottomWidth
        | PropertyId::BorderLeftWidth
        | PropertyId::BorderRightWidth
        | PropertyId::BorderBlockStartWidth
        | PropertyId::BorderBlockEndWidth
        | PropertyId::BorderInlineStartWidth
        | PropertyId::BorderInlineEndWidth
        | PropertyId::OutlineWidth) => parse!(|input| {
            let value = allocator.boxed(BorderSideWidth::parse(input)?);
            Ok(match property_id {
                PropertyId::BorderTopWidth => Declaration::BorderTopWidth(value),
                PropertyId::BorderBottomWidth => Declaration::BorderBottomWidth(value),
                PropertyId::BorderLeftWidth => Declaration::BorderLeftWidth(value),
                PropertyId::BorderRightWidth => Declaration::BorderRightWidth(value),
                PropertyId::BorderBlockStartWidth => Declaration::BorderBlockStartWidth(value),
                PropertyId::BorderBlockEndWidth => Declaration::BorderBlockEndWidth(value),
                PropertyId::BorderInlineStartWidth => Declaration::BorderInlineStartWidth(value),
                PropertyId::BorderInlineEndWidth => Declaration::BorderInlineEndWidth(value),
                PropertyId::OutlineWidth => Declaration::OutlineWidth(value),
                _ => unreachable!(),
            })
        }),
        PropertyId::Animation(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_animation_list(input).map(|values| Declaration::Animation(values, *prefix))
            })
        }
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
        property_id @ (PropertyId::MaxWidth
        | PropertyId::MaxHeight
        | PropertyId::MaxBlockSize
        | PropertyId::MaxInlineSize) => parse!(|input| {
            let value = allocator.boxed(MaxSize::parse(input)?);
            Ok(match property_id {
                PropertyId::MaxWidth => Declaration::MaxWidth(value),
                PropertyId::MaxHeight => Declaration::MaxHeight(value),
                PropertyId::MaxBlockSize => Declaration::MaxBlockSize(value),
                PropertyId::MaxInlineSize => Declaration::MaxInlineSize(value),
                _ => unreachable!(),
            })
        }),
        PropertyId::AnimationName(prefix) if !value_contains_comment(input) => parse!(|input| {
            parse_comma_separated(input, AnimationName::parse)
                .map(|value| Declaration::AnimationName(value, *prefix))
        }),
        PropertyId::AnimationDuration(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, Time::parse)
                    .map(|value| Declaration::AnimationDuration(value, *prefix))
            })
        }
        PropertyId::AnimationDelay(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, Time::parse)
                    .map(|value| Declaration::AnimationDelay(value, *prefix))
            })
        }
        PropertyId::AnimationTimingFunction(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, EasingFunction::parse)
                    .map(|value| Declaration::AnimationTimingFunction(value, *prefix))
            })
        }
        PropertyId::AnimationIterationCount(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, AnimationIterationCount::parse)
                    .map(|value| Declaration::AnimationIterationCount(value, *prefix))
            })
        }
        PropertyId::AnimationDirection(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, AnimationDirection::parse)
                    .map(|value| Declaration::AnimationDirection(value, *prefix))
            })
        }
        PropertyId::AnimationFillMode(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, AnimationFillMode::parse)
                    .map(|value| Declaration::AnimationFillMode(value, *prefix))
            })
        }
        PropertyId::AnimationPlayState(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, AnimationPlayState::parse)
                    .map(|value| Declaration::AnimationPlayState(value, *prefix))
            })
        }
        PropertyId::TransitionDuration(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, Time::parse)
                    .map(|value| Declaration::TransitionDuration(value, *prefix))
            })
        }
        PropertyId::TransitionDelay(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, Time::parse)
                    .map(|value| Declaration::TransitionDelay(value, *prefix))
            })
        }
        PropertyId::TransitionTimingFunction(prefix) if !value_contains_comment(input) => {
            parse!(|input| {
                parse_comma_separated(input, EasingFunction::parse)
                    .map(|value| Declaration::TransitionTimingFunction(value, *prefix))
            })
        }
        PropertyId::All => parse!(|input| {
            let ident = input.expect_ident()?;
            css_wide_keyword(ident)
                .map(Declaration::All)
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))
        }),
        _ => None,
    }
}

fn parse_css_wide_keyword<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CSSWideKeyword, ParseError<'i, ParserError<'i>>> {
    let ident = input.expect_ident()?;
    css_wide_keyword(ident).ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))
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

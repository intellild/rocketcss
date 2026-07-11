use super::{
    css_rule::parse_style_contents,
    media::parse_media_list,
    properties::parse_declaration,
    selector::parse_selector_list,
    stylesheet::{check_depth, recover_declaration, span_from},
    values::{
        collect_tokens, matches_ignore_case, remove_important, single_token, token_ident,
        trim_leading_whitespace,
    },
};
use crate::prelude::*;

pub(super) fn parse_font_face_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, rs_css_ast::FontFaceProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }

        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let value_start = input.position();
                let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, allocator, depth + 1)
                })?;
                let raw_value = input.slice(value_start..input.position());
                let _ = input.try_parse(Parser::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                if name.eq_ignore_ascii_case("unicode-range") {
                    let ranges = parse_unicode_ranges(raw_value, allocator)
                        .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                    Ok(rs_css_ast::FontFaceProperty::UnicodeRange(ranges))
                } else {
                    trim_leading_whitespace(&mut value);
                    Ok(rs_css_ast::FontFaceProperty::Custom(allocator.boxed(
                        CustomProperty {
                            name: allocator.boxed(CustomPropertyName::Unknown(name)),
                            value,
                        },
                    )))
                }
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

fn parse_unicode_ranges<'i>(
    source: &str,
    allocator: &'i Allocator,
) -> Option<Vec<'i, UnicodeRange>> {
    let mut ranges = allocator.vec();
    for value in source.split(',') {
        let value = value.trim();
        let body = value
            .strip_prefix("U+")
            .or_else(|| value.strip_prefix("u+"))?;
        let (start, end) = if body.contains('?') {
            let prefix = body.trim_end_matches('?');
            let wildcard_digits = body.len().checked_sub(prefix.len())?;
            if wildcard_digits == 0
                || wildcard_digits > 6
                || prefix.contains('?')
                || prefix.len() + wildcard_digits > 6
            {
                return None;
            }
            let prefix = if prefix.is_empty() {
                0
            } else {
                u32::from_str_radix(prefix, 16).ok()?
            };
            let bits = wildcard_digits * 4;
            let start = prefix << bits;
            (start, start | ((1_u32 << bits) - 1))
        } else if let Some((start, end)) = body.split_once('-') {
            (
                u32::from_str_radix(start, 16).ok()?,
                u32::from_str_radix(end, 16).ok()?,
            )
        } else {
            let value = u32::from_str_radix(body, 16).ok()?;
            (value, value)
        };
        if start > end || end > 0x10ffff {
            return None;
        }
        ranges.push(UnicodeRange { start, end });
    }
    (!ranges.is_empty()).then_some(ranges)
}

pub(super) fn parse_namespace<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(Option<&'i str>, &'i str), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let state = parser.state();
    if let Ok(prefix) = parser.try_parse(Parser::expect_ident)
        && let Ok(url) = parser.expect_url_or_string()
    {
        parser.expect_exhausted()?;
        return Ok((Some(prefix), url));
    }
    parser.reset(&state);
    let url = parser.expect_url_or_string()?;
    parser.expect_exhausted()?;
    Ok((None, url))
}

pub(super) fn validate_charset<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    parser.expect_string()?;
    parser.expect_exhausted()?;
    Ok(())
}

pub(super) fn parse_layer_names<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, Vec<'i, &'i str>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let mut name = allocator.vec();
        name.push(input.expect_ident()?);
        while input.try_parse(|input| input.expect_delim('.')).is_ok() {
            name.push(input.expect_ident()?);
        }
        input.expect_exhausted()?;
        Ok(name)
    })?;
    let mut names = allocator.vec();
    names.extend(parsed);
    Ok(names)
}

pub(super) fn parse_custom_media<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(&'i str, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.expect_ident()?;
    if !name.starts_with("--") {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    let query = parser
        .slice(parser.position()..SourcePosition(prelude.len()))
        .trim();
    if query.is_empty() {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    Ok((name, parse_media_list(query, allocator)?))
}

pub(super) fn parse_single_ident<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.expect_ident()?;
    parser.expect_exhausted()?;
    Ok(name)
}

pub(super) fn parse_keyframes_name<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = match parser.next()? {
        ValueToken::Ident(name)
            if !matches_ignore_case(
                name,
                &[
                    "none",
                    "initial",
                    "inherit",
                    "unset",
                    "default",
                    "revert",
                    "revert-layer",
                ],
            ) =>
        {
            KeyframesName::Ident(name)
        }
        ValueToken::String(name) => KeyframesName::Custom(name),
        _ => return Err(parser.new_custom_error(ParserError::InvalidValue)),
    };
    parser.expect_exhausted()?;
    Ok(name)
}

pub(super) fn parse_keyframe_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, Keyframe<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut keyframes = allocator.vec();
    loop {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        let parsed = input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.parse_comma_separated(parse_keyframe_selector)
        });
        input.expect_curly_bracket_block()?;
        if parsed.is_err() {
            input.parse_nested_block(|input| {
                while input.next_including_whitespace_and_comments().is_ok() {}
                Ok::<_, ParseError<'i, ParserError<'i>>>(())
            })?;
            continue;
        }
        let mut selectors = allocator.vec();
        selectors.extend(parsed?);
        let declarations = input.parse_nested_block(|input| {
            parse_declaration_block(input, allocator, options, depth + 1)
        })?;
        keyframes.push(Keyframe {
            declarations: allocator.boxed(declarations),
            selectors,
        });
    }
    Ok(keyframes)
}

pub(super) fn parse_keyframe_selector<'i>(
    input: &mut Parser<'i, '_>,
) -> Result<KeyframeSelector<'i>, ParseError<'i, ParserError<'i>>> {
    match input.next()? {
        ValueToken::Percentage(value) if (0.0..=1.0).contains(value) => {
            Ok(KeyframeSelector::Percentage(*value))
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("from") => Ok(KeyframeSelector::From),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("to") => Ok(KeyframeSelector::To),
        ValueToken::Ident(name) => {
            let name = match_ignore_ascii_case!(
                name,
                "cover" => TimelineRangeName::Cover,
                "contain" => TimelineRangeName::Contain,
                "entry" => TimelineRangeName::Entry,
                "exit" => TimelineRangeName::Exit,
                "entry-crossing" => TimelineRangeName::EntryCrossing,
                "exit-crossing" => TimelineRangeName::ExitCrossing,
                _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
            );
            let percentage = input.expect_percentage()?;
            Ok(KeyframeSelector::TimelineRangePercentage(
                input
                    .allocator()
                    .boxed(TimelineRangePercentage { name, percentage }),
            ))
        }
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

pub(super) fn parse_declaration_block<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<DeclarationBlock<'i>, ParseError<'i, ParserError<'i>>> {
    let (declarations, rules) = parse_style_contents(input, allocator, options, depth)?;
    if !rules.is_empty() {
        return Err(input.new_custom_error(ParserError::InvalidDeclaration));
    }
    Ok(declarations)
}

pub(super) fn at_rule_vendor_prefix(name: &str) -> VendorPrefix {
    if name
        .get(..8)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-webkit-"))
    {
        VendorPrefix::WEBKIT
    } else if name
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-moz-"))
    {
        VendorPrefix::MOZ
    } else if name
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-ms-"))
    {
        VendorPrefix::MS
    } else if name
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("-o-"))
    {
        VendorPrefix::O
    } else {
        VendorPrefix::NONE
    }
}

pub(super) fn validate_moz_document_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    parser.expect_function_matching("url-prefix")?;
    parser.parse_nested_block(|input| {
        if !input.is_exhausted() && !input.expect_string()?.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.expect_exhausted()?;
        Ok(())
    })?;
    parser.expect_exhausted()?;
    Ok(())
}

type ContainerPrelude<'i> = (
    Option<&'i str>,
    Option<rs_css_allocator::boxed::Box<'i, rs_css_ast::ContainerCondition<'i>>>,
);

pub(super) fn parse_container_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<ContainerPrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let name = parser.try_parse(Parser::expect_ident).ok();
    let condition = if parser.is_exhausted() {
        None
    } else {
        Some(
            allocator.boxed(rs_css_ast::ContainerCondition::Unknown(collect_tokens(
                &mut parser,
                allocator,
                0,
            )?)),
        )
    };
    if name.is_none() && condition.is_none() {
        return Err(parser.new_custom_error(ParserError::InvalidValue));
    }
    Ok((name, condition))
}

type ScopePrelude<'i> = (
    Option<rs_css_allocator::boxed::Box<'i, SelectorList<'i>>>,
    Option<rs_css_allocator::boxed::Box<'i, SelectorList<'i>>>,
);

pub(super) fn parse_scope_prelude<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<ScopePrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let scope_start = if parser.try_parse(Parser::expect_parenthesis_block).is_ok() {
        Some(allocator.boxed(
            parser.parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
        ))
    } else {
        None
    };

    let scope_end = if parser
        .try_parse(|input| input.expect_ident_matching("to"))
        .is_ok()
    {
        parser.expect_parenthesis_block()?;
        Some(allocator.boxed(
            parser.parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
        ))
    } else {
        None
    };
    parser.expect_exhausted()?;
    Ok((scope_start, scope_end))
}

pub(super) fn parse_page_selectors<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, PageSelector<'i>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    let mut input = ParserInput::new(prelude, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        let name = input.try_parse(Parser::expect_ident).ok();
        let mut pseudo_classes = allocator.vec();
        while input.try_parse(Parser::expect_colon).is_ok() {
            let pseudo = input.expect_ident()?;
            pseudo_classes.push(match_ignore_ascii_case!(
                pseudo,
                "left" => PagePseudoClass::Left,
                "right" => PagePseudoClass::Right,
                "first" => PagePseudoClass::First,
                "last" => PagePseudoClass::Last,
                "blank" => PagePseudoClass::Blank,
                _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
            ));
        }
        if name.is_none() && pseudo_classes.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidSelector));
        }
        input.expect_exhausted()?;
        Ok(PageSelector {
            name,
            pseudo_classes,
        })
    })?;
    let mut selectors = allocator.vec();
    selectors.extend(parsed);
    Ok(selectors)
}

pub(super) fn parse_page_body<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<(DeclarationBlock<'i>, Vec<'i, PageMarginRule<'i>>), ParseError<'i, ParserError<'i>>> {
    let mut declarations = DeclarationBlock::new(allocator);
    let mut rules = allocator.vec();

    loop {
        let start = input.state();
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }

        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let (declaration, important) =
                    parse_declaration(input, allocator, name, depth + 1)?;
                declarations.push(declaration, important);
                Ok(None)
            }
            ValueToken::AtKeyword(name) => {
                let margin_box = page_margin_box(name)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
                input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
                    input.expect_exhausted()?;
                    Ok::<_, ParseError<'i, ParserError<'i>>>(())
                })?;
                input.expect_curly_bracket_block()?;
                let declarations = input.parse_nested_block(|input| {
                    parse_declaration_block(input, allocator, options, depth + 1)
                })?;
                Ok(Some(PageMarginRule {
                    declarations: allocator.boxed(declarations),
                    span: span_from(&start, input.position()),
                    margin_box,
                }))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok(Some(rule)) => rules.push(rule),
            Ok(None) => {}
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }

    Ok((declarations, rules))
}

pub(super) fn parse_family_names<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, FamilyName<'i>>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    let parsed = parser.parse_comma_separated(|input| {
        if let Ok(name) = input.try_parse(Parser::expect_string) {
            input.expect_exhausted()?;
            return Ok(FamilyName(name));
        }
        let mut name = std::string::String::new();
        while !input.is_exhausted() {
            if !name.is_empty() {
                name.push(' ');
            }
            name.push_str(input.expect_ident()?);
        }
        if name.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok(FamilyName(allocator.alloc_str(&name)))
    })?;
    let mut names = allocator.vec();
    names.extend(parsed);
    Ok(names)
}

pub(super) fn parse_font_feature_subrules<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontFeatureSubrule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = allocator.vec();
    loop {
        let start = input.state();
        let name = match input.next() {
            Ok(ValueToken::AtKeyword(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidRule)),
            Err(error) => return Err(error.into()),
        };
        let kind = font_feature_subrule_type(name)
            .ok_or_else(|| input.new_custom_error(ParserError::InvalidAtRule(name)))?;
        input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })?;
        input.expect_curly_bracket_block()?;
        let declarations = input.parse_nested_block(|input| {
            parse_font_feature_declarations(input, allocator, options, depth + 1)
        })?;
        rules.push(FontFeatureSubrule {
            declarations,
            span: span_from(&start, input.position()),
            name: kind,
        });
    }
    Ok(rules)
}

pub(super) fn parse_font_feature_declarations<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontFeatureDeclaration<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = allocator.vec();
    loop {
        let name = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let values = input.parse_until_before(Delimiter::Semicolon, |input| {
                let mut values = allocator.vec();
                while !input.is_exhausted() {
                    values.push(input.expect_integer()?);
                }
                if values.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok(values)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            Ok::<_, ParseError<'i, ParserError<'i>>>(FontFeatureDeclaration { name, values })
        })();
        match result {
            Ok(declaration) => declarations.push(declaration),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(declarations)
}

pub(super) fn font_feature_subrule_type(name: &str) -> Option<FontFeatureSubruleType> {
    match_ignore_ascii_case!(
        name,
        "stylistic" => Some(FontFeatureSubruleType::Stylistic),
        "historical-forms" => Some(FontFeatureSubruleType::HistoricalForms),
        "styleset" => Some(FontFeatureSubruleType::Styleset),
        "character-variant" => Some(FontFeatureSubruleType::CharacterVariant),
        "swash" => Some(FontFeatureSubruleType::Swash),
        "ornaments" => Some(FontFeatureSubruleType::Ornaments),
        "annotation" => Some(FontFeatureSubruleType::Annotation),
        _ => None,
    )
}

pub(super) fn parse_font_palette_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, FontPaletteValuesProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, ValueToken::Semicolon) {
            continue;
        }
        let result = match token {
            ValueToken::Ident(name) => {
                input.expect_colon()?;
                let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, allocator, depth + 1)
                })?;
                let _ = input.try_parse(Parser::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                trim_leading_whitespace(&mut value);
                Ok(FontPaletteValuesProperty::Custom(allocator.boxed(
                    CustomProperty {
                        name: allocator.boxed(CustomPropertyName::Unknown(name)),
                        value,
                    },
                )))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };
        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

pub(super) fn parse_property_rule<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    name: &'i str,
) -> Result<PropertyRule<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut syntax = None;
    let mut inherits = None;
    let mut initial_value = None;

    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, allocator, depth + 1)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            if remove_important(&mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(&mut value);

            if descriptor.eq_ignore_ascii_case("syntax") {
                let Some(ValueToken::String(value)) = single_token(&value) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                syntax = Some(parse_syntax_string(value, allocator)?);
            } else if descriptor.eq_ignore_ascii_case("inherits") {
                let Some(value) = value.first().and_then(token_ident) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                inherits = Some(match_ignore_ascii_case!(
                    value,
                    "true" => true,
                    "false" => false,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                ));
            } else if descriptor.eq_ignore_ascii_case("initial-value") {
                initial_value = Some(allocator.boxed(ParsedComponent::TokenList(value)));
            }
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })();
        if let Err(error) = result {
            if options.error_recovery {
                recover_declaration(input);
            } else {
                return Err(error);
            }
        }
    }

    let syntax = syntax.ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let is_universal = matches!(syntax, SyntaxString::Universal);
    let inherits = inherits.ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    if !is_universal && initial_value.is_none() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok(PropertyRule {
        inherits,
        initial_value,
        span: Span::default(),
        name,
        syntax: allocator.boxed(syntax),
    })
}

pub(super) fn parse_syntax_string<'i>(
    value: &'i str,
    allocator: &'i Allocator,
) -> Result<SyntaxString<'i>, ParseError<'i, ParserError<'i>>> {
    if value == "*" {
        return Ok(SyntaxString::Universal);
    }
    let mut components = allocator.vec();
    for raw_component in value.split('|') {
        let raw_component = raw_component.trim();
        let (component, multiplier) = if let Some(component) = raw_component.strip_suffix('+') {
            (component.trim_end(), Multiplier::Space)
        } else if let Some(component) = raw_component.strip_suffix('#') {
            (component.trim_end(), Multiplier::Comma)
        } else {
            (raw_component, Multiplier::None)
        };
        let kind = match_ignore_ascii_case!(
            component,
            "<length>" => SyntaxComponentKind::Length,
            "<number>" => SyntaxComponentKind::Number,
            "<percentage>" => SyntaxComponentKind::Percentage,
            "<length-percentage>" => SyntaxComponentKind::LengthPercentage,
            "<string>" => SyntaxComponentKind::String,
            "<color>" => SyntaxComponentKind::Color,
            "<image>" => SyntaxComponentKind::Image,
            "<url>" => SyntaxComponentKind::Url,
            "<integer>" => SyntaxComponentKind::Integer,
            "<angle>" => SyntaxComponentKind::Angle,
            "<time>" => SyntaxComponentKind::Time,
            "<resolution>" => SyntaxComponentKind::Resolution,
            "<transform-function>" => SyntaxComponentKind::TransformFunction,
            "<transform-list>" => SyntaxComponentKind::TransformList,
            "<custom-ident>" => SyntaxComponentKind::CustomIdent,
            _ => if !component.is_empty()
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            {
                SyntaxComponentKind::Literal(component)
            } else {
                return Err(
                    crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue)
                );
            },
        );
        components.push(SyntaxComponent {
            kind: allocator.boxed(kind),
            multiplier,
        });
    }
    if components.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    Ok(SyntaxString::Components(components))
}

pub(super) fn parse_view_transition_contents<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<'i, ViewTransitionProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = allocator.vec();
    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, allocator, depth + 1)
            })?;
            let _ = input.try_parse(Parser::expect_semicolon);
            if remove_important(&mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(&mut value);

            let property = if descriptor.eq_ignore_ascii_case("navigation") {
                let value = value
                    .first()
                    .and_then(token_ident)
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                ViewTransitionProperty::Navigation(match_ignore_ascii_case!(
                    value,
                    "auto" => Navigation::Auto,
                    "none" => Navigation::None,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                ))
            } else if descriptor.eq_ignore_ascii_case("types") {
                let mut idents = allocator.vec();
                for token in &value {
                    if let Some(ident) = token_ident(token) {
                        idents.push(ident);
                    } else if !matches!(token, TokenOrValue::Token(token) if matches!(**token, ValueToken::WhiteSpace(_)))
                    {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                }
                let types = if idents.len() == 1 && idents[0].eq_ignore_ascii_case("none") {
                    NoneOrCustomIdentList::None
                } else if idents.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                } else {
                    NoneOrCustomIdentList::Idents(idents)
                };
                ViewTransitionProperty::Types(allocator.boxed(types))
            } else {
                ViewTransitionProperty::Custom(allocator.boxed(CustomProperty {
                    name: allocator.boxed(CustomPropertyName::Unknown(descriptor)),
                    value,
                }))
            };
            Ok::<_, ParseError<'i, ParserError<'i>>>(property)
        })();

        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

pub(super) fn page_margin_box(name: &str) -> Option<PageMarginBox> {
    match_ignore_ascii_case!(
        name,
        "top-left-corner" => Some(PageMarginBox::TopLeftCorner),
        "top-left" => Some(PageMarginBox::TopLeft),
        "top-center" => Some(PageMarginBox::TopCenter),
        "top-right" => Some(PageMarginBox::TopRight),
        "top-right-corner" => Some(PageMarginBox::TopRightCorner),
        "left-top" => Some(PageMarginBox::LeftTop),
        "left-middle" => Some(PageMarginBox::LeftMiddle),
        "left-bottom" => Some(PageMarginBox::LeftBottom),
        "right-top" => Some(PageMarginBox::RightTop),
        "right-middle" => Some(PageMarginBox::RightMiddle),
        "right-bottom" => Some(PageMarginBox::RightBottom),
        "bottom-left-corner" => Some(PageMarginBox::BottomLeftCorner),
        "bottom-left" => Some(PageMarginBox::BottomLeft),
        "bottom-center" => Some(PageMarginBox::BottomCenter),
        "bottom-right" => Some(PageMarginBox::BottomRight),
        "bottom-right-corner" => Some(PageMarginBox::BottomRightCorner),
        _ => None,
    )
}

use super::{
    properties::parse_declaration,
    selector::parse_selector_list,
    stylesheet::{check_depth, recover_declaration, span_from},
    values::{
        collect_tokens, matches_ignore_case, remove_important, single_token, token_ident,
        trim_leading_whitespace,
    },
};
use crate::prelude::*;

pub(super) fn parse_font_face_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<rocketcss_ast::FontFaceProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = std::vec::Vec::new();
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
                    collect_tokens(input, depth + 1)
                })?;
                let raw_value = input.slice(value_start..input.position());
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                if name.eq_ignore_ascii_case("unicode-range") {
                    let ranges = parse_unicode_ranges(raw_value)
                        .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                    Ok(rocketcss_ast::FontFaceProperty::UnicodeRange(ranges))
                } else {
                    trim_leading_whitespace(&mut value);
                    Ok(rocketcss_ast::FontFaceProperty::Custom(
                        std::boxed::Box::new(CustomProperty {
                            name: std::boxed::Box::new(CustomPropertyName::Unknown(name)),
                            value,
                        }),
                    ))
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

fn parse_unicode_ranges(source: &str) -> Option<std::vec::Vec<UnicodeRange>> {
    let mut ranges = std::vec::Vec::new();
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
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(Option<Atom<'i>>, Atom<'i>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
        let state = parser.state();
        if let Ok(prefix) = parser.try_parse(Compiler::expect_ident)
            && let Ok(url) = parser.expect_url_or_string()
        {
            parser.expect_exhausted()?;
            return Ok((Some(prefix), url));
        }
        parser.reset(&state);
        let url = parser.expect_url_or_string()?;
        parser.expect_exhausted()?;
        Ok((None, url))
    })
}

pub(super) fn parse_charset<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<Atom<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
        let encoding = parser.expect_string()?;
        parser.expect_exhausted()?;
        Ok(encoding)
    })
}

pub(super) fn parse_layer_names<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<std::vec::Vec<std::vec::Vec<Atom<'i>>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(std::vec::Vec::new());
    }
    input.with_source(prelude, |parser| {
        let parsed = parser.parse_comma_separated(|input| {
            let mut name = std::vec::Vec::new();
            name.push(input.expect_ident()?);
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                name.push(input.expect_ident()?);
            }
            input.expect_exhausted()?;
            Ok(name)
        })?;
        let mut names = std::vec::Vec::new();
        names.extend(parsed);
        Ok(names)
    })
}

pub(super) fn parse_custom_media<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(Atom<'i>, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
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
        // Keep custom media definitions lossless until custom-media expansion
        // is implemented.
        let condition = parser.with_source(query, |query_parser| {
            Ok::<_, ParseError<'i, ParserError<'i>>>(MediaCondition::Unknown(collect_tokens(
                query_parser,
                0,
            )?))
        })?;
        let media_queries = std::vec![std::boxed::Box::new(MediaQuery {
            condition: Some(condition),
            media_type: MediaType::All,
            qualifier: None,
        })];
        Ok((name, MediaList { media_queries }))
    })
}

pub(super) fn parse_single_ident<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<Atom<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
        let name = parser.expect_ident()?;
        parser.expect_exhausted()?;
        Ok(name)
    })
}

pub(super) fn parse_keyframes_name<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
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
                KeyframesName::Ident(name.clone())
            }
            ValueToken::String(name) => KeyframesName::Custom(name.clone()),
            _ => return Err(parser.new_custom_error(ParserError::InvalidValue)),
        };
        parser.expect_exhausted()?;
        Ok(name)
    })
}

pub(super) fn parse_keyframe_list<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<Keyframe>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut keyframes = std::vec::Vec::new();
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
        let mut selectors = std::vec::Vec::new();
        selectors.extend(parsed?);
        let declarations =
            input.parse_nested_block(|input| parse_declaration_block(input, options, depth + 1))?;
        keyframes.push(Keyframe {
            declarations,
            selectors,
        });
    }
    Ok(keyframes)
}

pub(super) fn parse_keyframe_selector<'i>(
    input: &mut Compiler<'i>,
) -> Result<KeyframeSelector, ParseError<'i, ParserError<'i>>> {
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
                TimelineRangePercentage { name, percentage },
            ))
        }
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

pub(super) fn parse_declaration_block<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<DeclarationBlockId, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let declarations = input.begin_declaration_block();

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
                parse_declaration(input, name, depth)
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok((declaration, important)) => {
                input.push_declaration(declarations, declaration, important);
            }
            Err(_) if options.error_recovery => {
                input.reset(&start);
                recover_declaration(input);
            }
            Err(error) => return Err(error),
        }
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
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
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
    })
}

type ContainerPrelude<'i> = (
    Option<Atom<'i>>,
    Option<std::boxed::Box<rocketcss_ast::ContainerCondition<'i>>>,
);

pub(super) fn parse_container_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<ContainerPrelude<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |parser| {
        let name = parser.try_parse(Compiler::expect_ident).ok();
        let condition = if parser.is_exhausted() {
            None
        } else {
            Some(std::boxed::Box::new(
                rocketcss_ast::ContainerCondition::Unknown(collect_tokens(parser, 0)?),
            ))
        };
        if name.is_none() && condition.is_none() {
            return Err(parser.new_custom_error(ParserError::InvalidValue));
        }
        Ok((name, condition))
    })
}

type ScopePrelude<'i> = (
    Option<std::boxed::Box<SelectorList<'i>>>,
    Option<std::boxed::Box<SelectorList<'i>>>,
);

pub(super) fn parse_scope_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
    depth: usize,
) -> Result<ScopePrelude<'i>, ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let scope_start = if input.try_parse(Compiler::expect_parenthesis_block).is_ok() {
            Some(std::boxed::Box::new(input.parse_nested_block(|input| {
                parse_selector_list(input, depth + 1)
            })?))
        } else {
            None
        };

        let scope_end = if input
            .try_parse(|input| input.expect_ident_matching("to"))
            .is_ok()
        {
            input.expect_parenthesis_block()?;
            Some(std::boxed::Box::new(input.parse_nested_block(|input| {
                parse_selector_list(input, depth + 1)
            })?))
        } else {
            None
        };
        input.expect_exhausted()?;
        Ok((scope_start, scope_end))
    })
}

pub(super) fn parse_page_selectors<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<std::vec::Vec<PageSelector<'i>>, ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(std::vec::Vec::new());
    }
    input.with_source(prelude, |parser| {
        let parsed = parser.parse_comma_separated(|input| {
            let name = input.try_parse(Compiler::expect_ident).ok();
            let mut pseudo_classes = std::vec::Vec::new();
            while input.try_parse(Compiler::expect_colon).is_ok() {
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
        let mut selectors = std::vec::Vec::new();
        selectors.extend(parsed);
        Ok(selectors)
    })
}

pub(super) fn parse_page_body<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<(DeclarationBlockId, std::vec::Vec<PageMarginRule>), ParseError<'i, ParserError<'i>>> {
    let declarations = input.begin_declaration_block();
    let mut rules = std::vec::Vec::new();

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
                let (declaration, important) = parse_declaration(input, name, depth + 1)?;
                input.push_declaration(declarations, declaration, important);
                Ok(None)
            }
            ValueToken::AtKeyword(name) => {
                let margin_box = page_margin_box(&name).ok_or_else(|| {
                    input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into()))
                })?;
                input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
                    input.expect_exhausted()?;
                    Ok::<_, ParseError<'i, ParserError<'i>>>(())
                })?;
                input.expect_curly_bracket_block()?;
                let declarations = input.parse_nested_block(|input| {
                    parse_declaration_block(input, options, depth + 1)
                })?;
                Ok(Some(PageMarginRule {
                    declarations,
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
    input: &mut Compiler<'i>,
    source: &'i str,
) -> Result<std::vec::Vec<FamilyName<'i>>, ParseError<'i, ParserError<'i>>> {
    input.with_source(source, |parser| {
        let parsed = parser.parse_comma_separated(|input| {
            if let Ok(name) = input.try_parse(Compiler::expect_string) {
                input.expect_exhausted()?;
                return Ok(FamilyName(name));
            }
            let mut name = std::string::String::new();
            while !input.is_exhausted() {
                if !name.is_empty() {
                    name.push(' ');
                }
                name.push_str(&input.expect_ident()?);
            }
            if name.is_empty() {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            Ok(FamilyName(input.intern(&name)))
        })?;
        let mut names = std::vec::Vec::new();
        names.extend(parsed);
        Ok(names)
    })
}

pub(super) fn parse_font_feature_subrules<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<FontFeatureSubrule<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut rules = std::vec::Vec::new();
    loop {
        let start = input.state();
        let name = match input.next() {
            Ok(ValueToken::AtKeyword(name)) => name.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidRule)),
            Err(error) => return Err(error.into()),
        };
        let kind = font_feature_subrule_type(&name).ok_or_else(|| {
            input.new_custom_error(ParserError::InvalidAtRule(name.to_string().into()))
        })?;
        input.parse_until_before(Delimiter::CurlyBracketBlock, |input| {
            input.expect_exhausted()?;
            Ok::<_, ParseError<'i, ParserError<'i>>>(())
        })?;
        input.expect_curly_bracket_block()?;
        let declarations = input.parse_nested_block(|input| {
            parse_font_feature_declarations(input, options, depth + 1)
        })?;
        rules.push(FontFeatureSubrule {
            declarations,
            span: span_from(&start, input.position()),
            name: kind,
        });
    }
    Ok(rules)
}

pub(super) fn parse_font_feature_declarations<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<FontFeatureDeclaration<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = std::vec::Vec::new();
    loop {
        let name = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => name.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let values = input.parse_until_before(Delimiter::Semicolon, |input| {
                let mut values = std::vec::Vec::new();
                while !input.is_exhausted() {
                    values.push(input.expect_integer()?);
                }
                if values.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok(values)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
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

pub(super) fn parse_font_palette_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<FontPaletteValuesProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = std::vec::Vec::new();
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
                    collect_tokens(input, depth + 1)
                })?;
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(&mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                trim_leading_whitespace(&mut value);
                Ok(FontPaletteValuesProperty::Custom(std::boxed::Box::new(
                    CustomProperty {
                        name: std::boxed::Box::new(CustomPropertyName::Unknown(name)),
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

pub(super) fn parse_property_rule<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
    name: Atom<'i>,
) -> Result<PropertyRule<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut syntax = None;
    let mut inherits = None;
    let mut initial_value = None;

    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => name.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, depth + 1)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(&mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(&mut value);

            if descriptor.eq_ignore_ascii_case("syntax") {
                let Some(ValueToken::String(value)) = single_token(&value) else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                syntax = Some(parse_syntax_string(value.as_str())?);
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
                initial_value = Some(std::boxed::Box::new(ParsedComponent::TokenList(value)));
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
        syntax: std::boxed::Box::new(syntax),
    })
}

pub(super) fn parse_syntax_string<'i>(
    value: &str,
) -> Result<SyntaxString, ParseError<'i, ParserError<'i>>> {
    if value == "*" {
        return Ok(SyntaxString::Universal);
    }
    let mut components = std::vec::Vec::new();
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
                SyntaxComponentKind::Literal(component.to_owned())
            } else {
                return Err(
                    crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue)
                );
            },
        );
        components.push(SyntaxComponent {
            kind: std::boxed::Box::new(kind),
            multiplier,
        });
    }
    if components.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    Ok(SyntaxString::Components(components))
}

pub(super) fn parse_view_transition_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<std::vec::Vec<ViewTransitionProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = std::vec::Vec::new();
    loop {
        let descriptor = match input.next() {
            Ok(ValueToken::Semicolon) => continue,
            Ok(ValueToken::Ident(name)) => name.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let mut value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, depth + 1)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
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
                let mut idents = std::vec::Vec::new();
                for token in &value {
                    if token_ident(token).is_some()
                        && let TokenOrValue::Token(token) = token
                        && let ValueToken::Ident(ident) = &**token
                    {
                        idents.push(ident.clone());
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
                ViewTransitionProperty::Types(std::boxed::Box::new(types))
            } else {
                ViewTransitionProperty::Custom(std::boxed::Box::new(CustomProperty {
                    name: std::boxed::Box::new(CustomPropertyName::Unknown(descriptor)),
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

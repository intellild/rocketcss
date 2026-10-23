//! Descriptor and prelude parsers for at-rules.

use super::super::{
    prelude::*,
    values::{collect_tokens, remove_important, token_ident, trim_leading_whitespace},
};

pub(crate) fn parse_font_face_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<FontFaceProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = Vec::new();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, OldToken::Semicolon) {
            continue;
        }

        let result = match token {
            OldToken::Ident(name) => {
                input.expect_colon()?;
                let value_start = input.position();
                let value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, depth + 1)
                })?;
                let raw_value = input.slice(value_start..input.position());
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(input.ast(), value).0 {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                if name.eq_ignore_ascii_case("unicode-range") {
                    let ranges = parse_unicode_ranges(input, raw_value)
                        .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                    Ok(with_token_span!(input, |span| input
                        .ast_mut()
                        .alloc_font_face_property(
                            FontFacePropertyData::UnicodeRange(ranges),
                            span,
                        )))
                } else {
                    let value = trim_leading_whitespace(input.ast(), value);
                    let custom =
                        alloc_custom_property(input, CustomPropertyName::Unknown(name), value);
                    Ok(with_token_span!(input, |span| input
                        .ast_mut()
                        .alloc_font_face_property(
                            FontFacePropertyData::Custom(custom),
                            span,
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
    input: &mut Compiler<'i>,
    source: &str,
) -> Option<&'i [UnicodeRange<'i>]> {
    let mut ranges = Vec::new();
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
        ranges.push(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_unicode_range(UnicodeRangeData { start, end }, span,)));
    }
    (!ranges.is_empty()).then(|| input.ast().alloc_slice(&ranges))
}

pub(crate) fn alloc_custom_property<'i>(
    input: &mut Compiler<'i>,
    name: CustomPropertyName<'i>,
    value: &'i [TokenOrValue<'i>],
) -> CustomProperty<'i> {
    with_token_span!(input, |span| input
        .ast_mut()
        .alloc_custom_property(CustomPropertyData { name, value }, span))
}

pub(crate) fn parse_namespace<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(Option<&'i str>, &'i str), ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, input.allocator());
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
}

pub(crate) fn parse_charset<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, input.allocator());
    let encoding = parser.expect_string()?;
    parser.expect_exhausted()?;
    Ok(encoding)
}

pub(crate) fn parse_layer_names<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<&'i [&'i [&'i str]], ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(input.ast().alloc_slice::<&'i [&'i str]>(&[]));
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let mut name = Vec::new();
            name.push(input.expect_ident()?);
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                name.push(input.expect_ident()?);
            }
            input.expect_exhausted()?;
            Ok(input.ast().alloc_slice(&name))
        })?;
        Ok(input.ast().alloc_slice(&parsed))
    })
}

pub(crate) fn parse_custom_media<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(&'i str, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let name = input.expect_ident()?;
        if !name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let query = input
            .slice(input.position()..crate::SourcePosition(prelude.len()))
            .trim();
        if query.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        input.with_source(query, |input| {
            let tokens = collect_tokens(input, 0)?;
            let condition = with_token_span!(input, |span| input
                .ast_mut()
                .alloc_media_condition(MediaConditionData::Unknown(tokens), span));
            let media_query = with_token_span!(input, |span| input.ast_mut().alloc_media_query(
                MediaQueryData {
                    condition: Some(condition),
                    media_type: MediaType::All,
                    qualifier: None,
                },
                span,
            ));
            let media_queries = input.ast().alloc_slice(&[media_query]);
            let media_list = with_token_span!(input, |span| input
                .ast_mut()
                .alloc_media_list(MediaListData { media_queries }, span));
            Ok((name, media_list))
        })
    })
}

pub(crate) fn parse_single_ident<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, input.allocator());
    let name = parser.expect_ident()?;
    parser.expect_exhausted()?;
    Ok(name)
}

pub(crate) fn parse_keyframes_name<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, input.allocator());
    let name = match parser.next()? {
        OldToken::Ident(name)
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
        OldToken::String(name) => KeyframesName::Custom(name),
        _ => return Err(parser.new_custom_error(ParserError::InvalidValue)),
    };
    parser.expect_exhausted()?;
    Ok(name)
}

pub(crate) fn parse_keyframe_selector<'i>(
    input: &mut Compiler<'i>,
) -> Result<KeyframeSelector<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let data = match input.next()? {
        OldToken::Percentage(value) if (0.0..=1.0).contains(value) => {
            KeyframeSelectorData::Percentage(*value)
        }
        OldToken::Ident(name) if name.eq_ignore_ascii_case("from") => KeyframeSelectorData::From,
        OldToken::Ident(name) if name.eq_ignore_ascii_case("to") => KeyframeSelectorData::To,
        OldToken::Ident(name) => {
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
            let timeline_span = span_from(&start, input.position());
            let timeline = input.ast_mut().alloc_timeline_range_percentage(
                TimelineRangePercentageData { name, percentage },
                timeline_span,
            );
            KeyframeSelectorData::TimelineRangePercentage(timeline)
        }
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_keyframe_selector(data, span))
}

pub(crate) fn at_rule_vendor_prefix(name: &str) -> VendorPrefix {
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

pub(crate) fn validate_moz_document_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, input.allocator());
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

pub(crate) fn parse_container_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(Option<&'i str>, Option<ContainerCondition<'i>>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let name = input.try_parse(Compiler::expect_ident).ok();
        input.skip_whitespace();
        let condition = if input.is_exhausted() {
            None
        } else {
            let tokens = collect_tokens(input, 0)?;
            Some(with_token_span!(input, |span| input
                .ast_mut()
                .alloc_container_condition(
                    ContainerConditionData::Unknown(tokens),
                    span
                )))
        };
        if name.is_none() && condition.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok((name, condition))
    })
}

pub(crate) fn parse_scope_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
    depth: usize,
) -> Result<(Option<SelectorList<'i>>, Option<SelectorList<'i>>), ParseError<'i, ParserError<'i>>> {
    input.with_source(prelude, |input| {
        let scope_start = if input.try_parse(Compiler::expect_parenthesis_block).is_ok() {
            Some(input.parse_nested_block(|input| {
                super::super::selector::parse_selector_list(input, depth + 1)
            })?)
        } else {
            None
        };

        let scope_end = if input
            .try_parse(|input| input.expect_ident_matching("to"))
            .is_ok()
        {
            input.expect_parenthesis_block()?;
            Some(input.parse_nested_block(|input| {
                super::super::selector::parse_selector_list(input, depth + 1)
            })?)
        } else {
            None
        };
        input.expect_exhausted()?;
        Ok((scope_start, scope_end))
    })
}

pub(crate) fn parse_page_selectors<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<&'i [PageSelector<'i>], ParseError<'i, ParserError<'i>>> {
    if prelude.is_empty() {
        return Ok(input.ast().alloc_slice::<PageSelector<'i>>(&[]));
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let name = input.try_parse(Compiler::expect_ident).ok();
            let mut pseudo_classes = Vec::new();
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
            let pseudo_classes = input.ast().alloc_slice(&pseudo_classes);
            Ok(with_token_span!(input, |span| input
                .ast_mut()
                .alloc_page_selector(
                    PageSelectorData {
                        name,
                        pseudo_classes,
                    },
                    span,
                )))
        })?;
        Ok(input.ast().alloc_slice(&parsed))
    })
}

pub(crate) fn parse_family_names<'i>(
    input: &mut Compiler<'i>,
    source: &'i str,
) -> Result<&'i [FamilyName<'i>], ParseError<'i, ParserError<'i>>> {
    input.with_source(source, |input| {
        let parsed = input.parse_comma_separated(|input| {
            if let Ok(name) = input.try_parse(Compiler::expect_string) {
                input.expect_exhausted()?;
                return Ok(with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_family_name(FamilyNameData { field_0: name }, span)));
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
            let name = input.ast().alloc_str(&name);
            Ok(with_token_span!(input, |span| input
                .ast_mut()
                .alloc_family_name(FamilyNameData { field_0: name }, span)))
        })?;
        Ok(input.ast().alloc_slice(&parsed))
    })
}

pub(crate) fn parse_font_feature_declarations<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<FontFeatureDeclaration<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut declarations = Vec::new();
    loop {
        let name = match input.next() {
            Ok(OldToken::Semicolon) => continue,
            Ok(OldToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let values = input.parse_until_before(Delimiter::Semicolon, |input| {
                let mut values = Vec::new();
                while !input.is_exhausted() {
                    values.push(input.expect_integer()?);
                }
                if values.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok(input.ast().alloc_slice(&values))
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
            Ok::<_, ParseError<'i, ParserError<'i>>>(with_token_span!(input, |span| {
                input.ast_mut().alloc_font_feature_declaration(
                    FontFeatureDeclarationData { name, values },
                    span,
                )
            }))
        })();
        match result {
            Ok(declaration) => declarations.push(declaration),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(declarations)
}

pub(crate) fn font_feature_subrule_type(name: &str) -> Option<FontFeatureSubruleType> {
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

pub(crate) fn parse_font_palette_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<FontPaletteValuesProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = Vec::new();
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };
        if matches!(token, OldToken::Semicolon) {
            continue;
        }
        let result = match token {
            OldToken::Ident(name) => {
                input.expect_colon()?;
                let value = input.parse_until_before(Delimiter::Semicolon, |input| {
                    collect_tokens(input, depth + 1)
                })?;
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(input.ast(), value).0 {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                let value = trim_leading_whitespace(input.ast(), value);
                let custom = alloc_custom_property(input, CustomPropertyName::Unknown(name), value);
                Ok(with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_font_palette_values_property(
                        FontPaletteValuesPropertyData::Custom(custom),
                        span,
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

#[derive(Default)]
pub(crate) struct PropertyRuleDescriptors<'i> {
    pub(crate) syntax: Option<SyntaxString<'i>>,
    pub(crate) inherits: Option<bool>,
    pub(crate) initial_value: Option<ParsedComponent<'i>>,
}

pub(crate) fn parse_property_rule_descriptors<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<
    (PropertyRuleDescriptors<'i>, Vec<PropertyRuleDescriptor<'i>>),
    ParseError<'i, ParserError<'i>>,
> {
    check_depth(input, depth)?;
    let mut descriptors = PropertyRuleDescriptors::default();
    let mut pushed = Vec::new();
    let mut has_syntax = false;
    let mut syntax_is_universal = false;
    let mut has_inherits = false;
    let mut has_initial_value = false;

    loop {
        let descriptor = match input.next() {
            Ok(OldToken::Semicolon) => continue,
            Ok(OldToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, depth + 1)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(input.ast(), value).0 {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            let value = trim_leading_whitespace(input.ast(), value);

            let parsed = if descriptor.eq_ignore_ascii_case("syntax") {
                let syntax_string = match value {
                    [token] => match token.data(input.ast()) {
                        TokenOrValueData::Token(token) => match token.data(input.ast()) {
                            TokenData::String(value) => Some(value),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                }
                .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                let syntax = parse_syntax_string(input, syntax_string)?;
                syntax_is_universal =
                    matches!(syntax.data(input.ast()), SyntaxStringData::Universal);
                has_syntax = true;
                descriptors.syntax = Some(syntax);
                with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_property_rule_descriptor(
                        PropertyRuleDescriptorData::Syntax(syntax),
                        span,
                    ))
            } else if descriptor.eq_ignore_ascii_case("inherits") {
                let value = value
                    .first()
                    .and_then(|value| token_ident(input.ast(), value))
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                let inherits = match_ignore_ascii_case!(
                    value,
                    "true" => true,
                    "false" => false,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                );
                has_inherits = true;
                descriptors.inherits = Some(inherits);
                with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_property_rule_descriptor(
                        PropertyRuleDescriptorData::Inherits(inherits),
                        span,
                    ))
            } else if descriptor.eq_ignore_ascii_case("initial-value") {
                has_initial_value = true;
                let component = with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_parsed_component(ParsedComponentData::TokenList(value), span,));
                descriptors.initial_value = Some(component);
                with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_property_rule_descriptor(
                        PropertyRuleDescriptorData::InitialValue(component),
                        span,
                    ))
            } else {
                let custom =
                    alloc_custom_property(input, CustomPropertyName::Unknown(descriptor), value);
                with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_property_rule_descriptor(
                        PropertyRuleDescriptorData::Unknown(custom),
                        span,
                    ))
            };
            pushed.push(parsed);
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

    if !has_syntax || !has_inherits || (!syntax_is_universal && !has_initial_value) {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    Ok((descriptors, pushed))
}

pub(crate) fn parse_syntax_string<'i>(
    input: &mut Compiler<'i>,
    value: &'i str,
) -> Result<SyntaxString<'i>, ParseError<'i, ParserError<'i>>> {
    if value == "*" {
        return Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_syntax_string(SyntaxStringData::Universal, span)));
    }
    let mut components = Vec::new();
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
            "<length>" => SyntaxComponentKindData::Length,
            "<number>" => SyntaxComponentKindData::Number,
            "<percentage>" => SyntaxComponentKindData::Percentage,
            "<length-percentage>" => SyntaxComponentKindData::LengthPercentage,
            "<string>" => SyntaxComponentKindData::String,
            "<color>" => SyntaxComponentKindData::Color,
            "<image>" => SyntaxComponentKindData::Image,
            "<url>" => SyntaxComponentKindData::Url,
            "<integer>" => SyntaxComponentKindData::Integer,
            "<angle>" => SyntaxComponentKindData::Angle,
            "<time>" => SyntaxComponentKindData::Time,
            "<resolution>" => SyntaxComponentKindData::Resolution,
            "<transform-function>" => SyntaxComponentKindData::TransformFunction,
            "<transform-list>" => SyntaxComponentKindData::TransformList,
            "<custom-ident>" => SyntaxComponentKindData::CustomIdent,
            _ => if !component.is_empty()
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            {
                SyntaxComponentKindData::Literal(component)
            } else {
                return Err(
                    crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue)
                );
            },
        );
        let kind = with_token_span!(input, |span| input
            .ast_mut()
            .alloc_syntax_component_kind(kind, span));
        components.push(with_token_span!(input, |span| {
            input
                .ast_mut()
                .alloc_syntax_component(SyntaxComponentData { kind, multiplier }, span)
        }));
    }
    if components.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    let components = input.ast().alloc_slice(&components);
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_syntax_string(
            SyntaxStringData::Components(components),
            span
        )))
}

pub(crate) fn parse_view_transition_contents<'i>(
    input: &mut Compiler<'i>,
    options: &ParserOptions<'i>,
    depth: usize,
) -> Result<Vec<ViewTransitionProperty<'i>>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut properties = Vec::new();
    loop {
        let descriptor = match input.next() {
            Ok(OldToken::Semicolon) => continue,
            Ok(OldToken::Ident(name)) => *name,
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => return Err(input.new_custom_error(ParserError::InvalidDeclaration)),
            Err(error) => return Err(error.into()),
        };
        let result = (|| {
            input.expect_colon()?;
            let value = input.parse_until_before(Delimiter::Semicolon, |input| {
                collect_tokens(input, depth + 1)
            })?;
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(input.ast(), value).0 {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            let value = trim_leading_whitespace(input.ast(), value);

            let data = if descriptor.eq_ignore_ascii_case("navigation") {
                let value = value
                    .first()
                    .and_then(|value| token_ident(input.ast(), value))
                    .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                ViewTransitionPropertyData::Navigation(match_ignore_ascii_case!(
                    value,
                    "auto" => Navigation::Auto,
                    "none" => Navigation::None,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                ))
            } else if descriptor.eq_ignore_ascii_case("types") {
                let mut idents = Vec::new();
                for token in value {
                    if let Some(ident) = token_ident(input.ast(), token) {
                        idents.push(ident);
                    } else if !matches!(
                        token.data(input.ast()),
                        TokenOrValueData::Token(token)
                            if matches!(token.data(input.ast()), TokenData::WhiteSpace(_))
                    ) {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                }
                let types = if idents.len() == 1 && idents[0].eq_ignore_ascii_case("none") {
                    NoneOrCustomIdentListData::None
                } else if idents.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                } else {
                    NoneOrCustomIdentListData::Idents(input.ast().alloc_slice(&idents))
                };
                let types = with_token_span!(input, |span| input
                    .ast_mut()
                    .alloc_none_or_custom_ident_list(types, span));
                ViewTransitionPropertyData::Types(types)
            } else {
                let custom =
                    alloc_custom_property(input, CustomPropertyName::Unknown(descriptor), value);
                ViewTransitionPropertyData::Custom(custom)
            };
            Ok::<_, ParseError<'i, ParserError<'i>>>(with_token_span!(input, |span| input
                .ast_mut()
                .alloc_view_transition_property(data, span,)))
        })();

        match result {
            Ok(property) => properties.push(property),
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(properties)
}

pub(crate) fn page_margin_box(name: &str) -> Option<PageMarginBox> {
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

pub(crate) fn matches_ignore_case(value: &str, expected: &[&str]) -> bool {
    expected.iter().any(|item| value.eq_ignore_ascii_case(item))
}

pub(crate) fn check_depth<'i>(
    input: &Compiler<'i>,
    depth: usize,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    super::super::prelude::check_depth(input, depth)
}

pub(crate) fn recover_declaration(input: &mut Compiler<'_>) {
    let _: Result<(), ParseError<'_, ()>> =
        input.parse_until_after(Delimiter::Semicolon, |_| Ok(()));
}

pub(crate) fn recover_rule(input: &mut Compiler<'_>) {
    let _ = input.next_including_whitespace_and_comments();
}

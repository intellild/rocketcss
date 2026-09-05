use super::{
    selector::parse_selector_list,
    stylesheet::{check_depth, recover_declaration},
    values::{
        collect_tokens, matches_ignore_case, remove_important, single_token, token_ident,
        trim_leading_whitespace,
    },
};
use crate::prelude::*;
use rocketcss_ast::PropertyRuleDescriptor;

pub(super) fn parse_font_face_contents_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(
        &mut Compiler<'i>,
        rocketcss_ast::FontFaceProperty<'i>,
    ) -> Result<(), ParseError<'i, ParserError<'i>>>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
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
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(input.ast_context(), &mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                if name.eq_ignore_ascii_case("unicode-range") {
                    let ranges = parse_unicode_ranges(raw_value, allocator)
                        .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
                    Ok(rocketcss_ast::FontFaceProperty::UnicodeRange(store_vec(
                        ranges, input,
                    )))
                } else {
                    trim_leading_whitespace(input.ast_context(), &mut value);
                    let value = store_vec(value, input);
                    Ok(rocketcss_ast::FontFaceProperty::Custom(store_node(
                        CustomProperty {
                            name: store_node(CustomPropertyName::Unknown(name), input),
                            value,
                        },
                        input,
                    )))
                }
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };

        match result {
            Ok(property) => push(input, property)?,
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
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
    let mut parser = Compiler::new_with_source(prelude, allocator);
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

pub(super) fn parse_charset<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, allocator);
    let encoding = parser.expect_string()?;
    parser.expect_exhausted()?;
    Ok(encoding)
}

pub(super) fn parse_layer_names<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<Vec<'i, AstVec<'i, &'i str>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let mut name = allocator.vec();
            name.push(input.expect_ident()?);
            while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                name.push(input.expect_ident()?);
            }
            input.expect_exhausted()?;
            Ok(store_vec(name, input))
        })?;
        let mut names = allocator.vec();
        names.extend(parsed);
        Ok(names)
    })
}

pub(super) fn parse_custom_media<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<(&'i str, MediaList<'i>), ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let name = input.expect_ident()?;
        if !name.starts_with("--") {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let query = input
            .slice(input.position()..SourcePosition(prelude.len()))
            .trim();
        if query.is_empty() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        // Keep custom media definitions lossless until custom-media expansion is
        // implemented. Parsing these into normalized range features would change
        // their public serialization even though this crate does not consume the
        // definition yet.
        let tokens = input.with_source(query, |input| collect_tokens(input, allocator, 0))?;
        let condition = MediaCondition::Unknown(store_vec(tokens, input));
        let mut media_queries = allocator.vec();
        media_queries.push(store_node(
            MediaQuery {
                condition: Some(store_node(condition, input)),
                media_type: MediaType::All,
                qualifier: None,
            },
            input,
        ));
        Ok((
            name,
            MediaList {
                media_queries: store_vec(media_queries, input),
            },
        ))
    })
}

pub(super) fn parse_single_ident<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<&'i str, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, allocator);
    let name = parser.expect_ident()?;
    parser.expect_exhausted()?;
    Ok(name)
}

pub(super) fn parse_keyframes_name<'i>(
    prelude: &'i str,
    allocator: &'i Allocator,
) -> Result<KeyframesName<'i>, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(prelude, allocator);
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
    let mut parser = Compiler::new_with_source(prelude, allocator);
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
    Option<NodeId<'i, rocketcss_ast::ContainerCondition<'i>>>,
);

pub(super) fn parse_container_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<ContainerPrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let name = input.try_parse(Compiler::expect_ident).ok();
        input.skip_whitespace();
        let condition = if input.is_exhausted() {
            None
        } else {
            let tokens = collect_tokens(input, allocator, 0)?;
            Some(store_node(
                rocketcss_ast::ContainerCondition::Unknown(store_vec(tokens, input)),
                input,
            ))
        };
        if name.is_none() && condition.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok((name, condition))
    })
}

type ScopePrelude<'i> = (
    Option<NodeId<'i, SelectorList<'i>>>,
    Option<NodeId<'i, SelectorList<'i>>>,
);

pub(super) fn parse_scope_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
    depth: usize,
) -> Result<ScopePrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let scope_start = if input.try_parse(Compiler::expect_parenthesis_block).is_ok() {
            Some(store_node(
                input
                    .parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
                input,
            ))
        } else {
            None
        };

        let scope_end = if input
            .try_parse(|input| input.expect_ident_matching("to"))
            .is_ok()
        {
            input.expect_parenthesis_block()?;
            Some(store_node(
                input
                    .parse_nested_block(|input| parse_selector_list(input, allocator, depth + 1))?,
                input,
            ))
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
) -> Result<Vec<'i, PageSelector<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    if prelude.is_empty() {
        return Ok(allocator.vec());
    }
    input.with_source(prelude, |input| {
        let parsed = input.parse_comma_separated(|input| {
            let name = input.try_parse(Compiler::expect_ident).ok();
            let mut pseudo_classes = allocator.vec();
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
                pseudo_classes: store_vec(pseudo_classes, input),
            })
        })?;
        let mut selectors = allocator.vec();
        selectors.extend(parsed);
        Ok(selectors)
    })
}

pub(super) fn parse_family_names<'i>(
    source: &'i str,
    allocator: &'i Allocator,
) -> Result<Vec<'i, FamilyName<'i>>, ParseError<'i, ParserError<'i>>> {
    let mut parser = Compiler::new_with_source(source, allocator);
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

pub(super) fn parse_font_feature_declarations_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(
        &mut Compiler<'i>,
        FontFeatureDeclaration<'i>,
    ) -> Result<(), ParseError<'i, ParserError<'i>>>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
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
            let _ = input.try_parse(Compiler::expect_semicolon);
            Ok::<_, ParseError<'i, ParserError<'i>>>(FontFeatureDeclaration {
                name,
                values: store_vec(values, input),
            })
        })();
        match result {
            Ok(declaration) => push(input, declaration)?,
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
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

pub(super) fn parse_font_palette_contents_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(
        &mut Compiler<'i>,
        FontPaletteValuesProperty<'i>,
    ) -> Result<(), ParseError<'i, ParserError<'i>>>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
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
                let _ = input.try_parse(Compiler::expect_semicolon);
                if remove_important(input.ast_context(), &mut value) {
                    return Err(input.new_custom_error(ParserError::InvalidDeclaration));
                }
                trim_leading_whitespace(input.ast_context(), &mut value);
                let value = store_vec(value, input);
                Ok(FontPaletteValuesProperty::Custom(store_node(
                    CustomProperty {
                        name: store_node(CustomPropertyName::Unknown(name), input),
                        value,
                    },
                    input,
                )))
            }
            _ => Err(input.new_custom_error(ParserError::InvalidDeclaration)),
        };
        match result {
            Ok(property) => push(input, property)?,
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

pub(super) fn parse_property_rule_descriptors_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(&mut Compiler<'i>, PropertyRuleDescriptor<'i>),
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut has_syntax = false;
    let mut syntax_is_universal = false;
    let mut has_inherits = false;
    let mut has_initial_value = false;

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
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(input.ast_context(), &mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(input.ast_context(), &mut value);

            let parsed = if descriptor.eq_ignore_ascii_case("syntax") {
                let Some(ValueToken::String(value)) = single_token(input.ast_context(), &value)
                else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                let syntax = parse_syntax_string(input, value)?;
                syntax_is_universal = matches!(syntax, SyntaxString::Universal);
                has_syntax = true;
                PropertyRuleDescriptor::Syntax(store_node(syntax, input))
            } else if descriptor.eq_ignore_ascii_case("inherits") {
                let Some(value) = value
                    .first()
                    .and_then(|token| token_ident(input.ast_context(), token))
                else {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                };
                let inherits = match_ignore_ascii_case!(
                    value,
                    "true" => true,
                    "false" => false,
                    _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
                );
                has_inherits = true;
                PropertyRuleDescriptor::Inherits(inherits)
            } else if descriptor.eq_ignore_ascii_case("initial-value") {
                has_initial_value = true;
                let value = store_vec(value, input);
                PropertyRuleDescriptor::InitialValue(store_node(
                    ParsedComponent::TokenList(value),
                    input,
                ))
            } else {
                let value = store_vec(value, input);
                PropertyRuleDescriptor::Unknown(store_node(
                    CustomProperty {
                        name: store_node(CustomPropertyName::Unknown(descriptor), input),
                        value,
                    },
                    input,
                ))
            };
            push(input, parsed);
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
    Ok(())
}

pub(super) fn parse_syntax_string<'i>(
    input: &mut Compiler<'i>,
    value: &'i str,
) -> Result<SyntaxString<'i>, ParseError<'i, ParserError<'i>>> {
    if value == "*" {
        return Ok(SyntaxString::Universal);
    }
    let mut components = input.allocator().vec();
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
            kind: store_node(kind, input),
            multiplier,
        });
    }
    if components.is_empty() {
        return Err(crate::SourceLocation::default().new_custom_error(ParserError::InvalidValue));
    }
    Ok(SyntaxString::Components(store_vec(components, input)))
}

pub(super) fn parse_view_transition_contents_into<'i>(
    input: &mut Compiler<'i>,
    allocator: &'i Allocator,
    options: &ParserOptions<'i>,
    depth: usize,
    mut push: impl FnMut(
        &mut Compiler<'i>,
        ViewTransitionProperty<'i>,
    ) -> Result<(), ParseError<'i, ParserError<'i>>>,
) -> Result<(), ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
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
            let _ = input.try_parse(Compiler::expect_semicolon);
            if remove_important(input.ast_context(), &mut value) {
                return Err(input.new_custom_error(ParserError::InvalidDeclaration));
            }
            trim_leading_whitespace(input.ast_context(), &mut value);

            let property = if descriptor.eq_ignore_ascii_case("navigation") {
                let value = value
                    .first()
                    .and_then(|token| token_ident(input.ast_context(), token))
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
                    if let Some(ident) = token_ident(input.ast_context(), token) {
                        idents.push(ident);
                    } else if !matches!(token, TokenOrValue::Token(token) if matches!(input.ast_context().node(*token), ValueToken::WhiteSpace(_)))
                    {
                        return Err(input.new_custom_error(ParserError::InvalidValue));
                    }
                }
                let types = if idents.len() == 1 && idents[0].eq_ignore_ascii_case("none") {
                    NoneOrCustomIdentList::None
                } else if idents.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                } else {
                    NoneOrCustomIdentList::Idents(store_vec(idents, input))
                };
                ViewTransitionProperty::Types(store_node(types, input))
            } else {
                let value = store_vec(value, input);
                ViewTransitionProperty::Custom(store_node(
                    CustomProperty {
                        name: store_node(CustomPropertyName::Unknown(descriptor), input),
                        value,
                    },
                    input,
                ))
            };
            Ok::<_, ParseError<'i, ParserError<'i>>>(property)
        })();

        match result {
            Ok(property) => push(input, property)?,
            Err(_) if options.error_recovery => recover_declaration(input),
            Err(error) => return Err(error),
        }
    }
    Ok(())
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

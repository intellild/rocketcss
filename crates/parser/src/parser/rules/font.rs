use super::*;

pub(in crate::parser) fn parse_font_face_contents_into<'i>(
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
            Ok(token) => *token,
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
                            name: store_node(
                                CustomPropertyName::Unknown(input.add_str(name)),
                                input,
                            ),
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

pub(in crate::parser) fn parse_family_names<'i>(
    source: &'i str,
    input: &mut Compiler<'i>,
) -> Result<Vec<'i, FamilyName<'i>>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(source, |parser| {
        let parsed = parser.parse_comma_separated(|input| {
            if let Ok(name) = input.try_parse(Compiler::expect_string) {
                input.expect_exhausted()?;
                return Ok(FamilyName(input.add_str(name)));
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
            Ok(FamilyName(input.add_str(&name)))
        })?;
        let mut names = allocator.vec();
        names.extend(parsed);
        Ok(names)
    })
}

pub(in crate::parser) fn parse_font_feature_declarations_into<'i>(
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
                name: input.add_str(name),
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

pub(in crate::parser) fn font_feature_subrule_type(name: &str) -> Option<FontFeatureSubruleType> {
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

pub(in crate::parser) fn parse_font_palette_contents_into<'i>(
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
            Ok(token) => *token,
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
                        name: store_node(CustomPropertyName::Unknown(input.add_str(name)), input),
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

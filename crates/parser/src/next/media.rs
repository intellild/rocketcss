//! Media query and supports condition parsing into the flattened AST.

use super::{prelude::*, values::collect_tokens};

pub(crate) fn parse_import_rule<'i>(
    prelude: &'i str,
    input: &mut Compiler<'i>,
    start: &crate::ParserState,
    end: crate::SourcePosition,
) -> Result<ImportRule<'i>, ParseError<'i, ParserError<'i>>> {
    let span = span_from(start, end);
    input.with_source(prelude, |input| {
        let url = input.expect_url_or_string()?;

        let layer = if input
            .try_parse(|input| input.expect_ident_matching("layer"))
            .is_ok()
        {
            Some(input.ast().alloc_slice::<&'i str>(&[]))
        } else if input
            .try_parse(|input| input.expect_function_matching("layer"))
            .is_ok()
        {
            Some(input.parse_nested_block(|input| {
                let mut name = Vec::new();
                name.push(input.expect_ident()?);
                while input.try_parse(|input| input.expect_delim('.')).is_ok() {
                    name.push(input.expect_ident()?);
                }
                input.expect_exhausted()?;
                Ok::<_, ParseError<'i, ParserError<'i>>>(input.ast().alloc_slice(&name))
            })?)
        } else {
            None
        };

        let supports = if input
            .try_parse(|input| input.expect_function_matching("supports"))
            .is_ok()
        {
            Some(input.parse_nested_block(|input| {
                let start = input.position();
                input.expect_no_error_token()?;
                let raw = input.slice_from(start).trim();
                if raw.is_empty() {
                    return Err(input.new_custom_error(ParserError::InvalidValue));
                }
                Ok::<_, ParseError<'i, ParserError<'i>>>(parse_supports_condition(input, raw))
            })?)
        } else {
            None
        };

        let media = if input.is_exhausted() {
            None
        } else {
            let rest = input
                .slice(input.position()..crate::SourcePosition(prelude.len()))
                .trim();
            if rest.is_empty() {
                None
            } else {
                Some(parse_media_list(input, rest)?)
            }
        };
        Ok(input.ast_mut().alloc_import_rule(
            ImportRuleData {
                layer,
                media,
                supports,
                url,
            },
            span,
        ))
    })
}

pub(crate) fn parse_media_list<'i>(
    input: &mut Compiler<'i>,
    source: &'i str,
) -> Result<MediaList<'i>, ParseError<'i, ParserError<'i>>> {
    if source.trim().is_empty() {
        let media_queries = input.ast().alloc_slice::<MediaQuery<'i>>(&[]);
        return Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_media_list(MediaListData { media_queries }, span)));
    }
    input.with_source(source, |input| {
        let parsed = input.parse_comma_separated(|input| {
            input
                .try_parse(|input| parse_media_query(input))
                .or_else(|_| parse_unknown_media_query(input))
        })?;
        let media_queries = input.ast().alloc_slice(&parsed);
        Ok(with_token_span!(input, |span| input
            .ast_mut()
            .alloc_media_list(MediaListData { media_queries }, span)))
    })
}

fn parse_media_query<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaQuery<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let explicit = input
        .try_parse(|input| {
            let qualifier = input.try_parse(parse_qualifier).ok();
            let media_type = parse_media_type(input)?;
            Ok::<_, ParseError<'i, ParserError<'i>>>((qualifier, media_type))
        })
        .ok();

    let (qualifier, media_type, condition) = if let Some((qualifier, media_type)) = explicit {
        let condition = if input.is_exhausted() {
            None
        } else {
            input.expect_ident_matching("and")?;
            Some(parse_media_condition_or_unknown(input, false)?)
        };
        (qualifier, media_type, condition)
    } else {
        (
            None,
            MediaType::All,
            Some(parse_media_condition_or_unknown(input, true)?),
        )
    };

    input.expect_exhausted()?;
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_media_query(
        MediaQueryData {
            condition,
            media_type,
            qualifier,
        },
        span,
    ))
}

fn parse_unknown_media_query<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaQuery<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let tokens = collect_tokens(input, 0)?;
    let condition = with_token_span!(input, |span| input
        .ast_mut()
        .alloc_media_condition(MediaConditionData::Unknown(tokens), span));
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_media_query(
        MediaQueryData {
            condition: Some(condition),
            media_type: MediaType::All,
            qualifier: None,
        },
        span,
    ))
}

fn parse_qualifier<'i>(
    input: &mut Compiler<'i>,
) -> Result<Qualifier, ParseError<'i, ParserError<'i>>> {
    let name = input.expect_ident()?;
    match_ignore_ascii_case!(
        name,
        "only" => Ok(Qualifier::Only),
        "not" => Ok(Qualifier::Not),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

fn parse_media_type<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaType<'i>, ParseError<'i, ParserError<'i>>> {
    let name = input.expect_ident()?;
    match_ignore_ascii_case!(
        name,
        "all" => Ok(MediaType::All),
        "print" => Ok(MediaType::Print),
        "screen" => Ok(MediaType::Screen),
        "and" | "or" | "not" | "only" => {
            Err(input.new_custom_error(ParserError::InvalidValue))
        },
        _ => Ok(MediaType::Custom(name)),
    )
}

fn parse_media_condition_or_unknown<'i>(
    input: &mut Compiler<'i>,
    allow_or: bool,
) -> Result<MediaCondition<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    if let Ok(condition) = input.try_parse(|input| -> Result<_, ParseError<'i, ParserError<'i>>> {
        let condition = parse_media_condition(input, allow_or)?;
        input.expect_exhausted()?;
        Ok(condition)
    }) {
        return Ok(condition);
    }
    input.reset(&start);
    let tokens = collect_tokens(input, 0)?;
    Ok(with_token_span!(input, |span| input
        .ast_mut()
        .alloc_media_condition(
            MediaConditionData::Unknown(tokens),
            span
        )))
}

fn parse_media_condition<'i>(
    input: &mut Compiler<'i>,
    allow_or: bool,
) -> Result<MediaCondition<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let first = match input.next()? {
        OldToken::ParenthesisBlock => parse_parenthesized_condition(input)?,
        OldToken::Ident(name) if name.eq_ignore_ascii_case("not") => {
            let condition = parse_parenthesis(input)?;
            let span = span_from(&start, input.position());
            return Ok(input
                .ast_mut()
                .alloc_media_condition(MediaConditionData::Not(condition), span));
        }
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };

    let operator = match input.try_parse(parse_operator) {
        Ok(operator) => operator,
        Err(_) => return Ok(first),
    };
    if !allow_or && matches!(operator, Operator::Or) {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let mut conditions = Vec::new();
    conditions.push(first);
    conditions.push(parse_parenthesis(input)?);
    let delimiter = match operator {
        Operator::And => "and",
        Operator::Or => "or",
    };
    while input
        .try_parse(|input| input.expect_ident_matching(delimiter))
        .is_ok()
    {
        conditions.push(parse_parenthesis(input)?);
    }
    let conditions = input.ast().alloc_slice(&conditions);
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_media_condition(
        MediaConditionData::Operation {
            conditions,
            operator,
        },
        span,
    ))
}

fn parse_operator<'i>(
    input: &mut Compiler<'i>,
) -> Result<Operator, ParseError<'i, ParserError<'i>>> {
    let name = input.expect_ident()?;
    match_ignore_ascii_case!(
        name,
        "and" => Ok(Operator::And),
        "or" => Ok(Operator::Or),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    )
}

fn parse_parenthesis<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaCondition<'i>, ParseError<'i, ParserError<'i>>> {
    input.expect_parenthesis_block()?;
    parse_parenthesized_condition(input)
}

fn parse_parenthesized_condition<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaCondition<'i>, ParseError<'i, ParserError<'i>>> {
    input.parse_nested_block(|input| {
        if input.is_exhausted() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        if let Ok(condition) = input.try_parse(|input| parse_media_condition(input, true)) {
            return Ok(condition);
        }
        let start = input.state();
        let feature = parse_media_feature(input)?;
        let span = span_from(&start, input.position());
        Ok(input
            .ast_mut()
            .alloc_media_condition(MediaConditionData::Feature(feature), span))
    })
}

fn parse_media_feature<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaFeature<'i>, ParseError<'i, ParserError<'i>>> {
    match input.try_parse(parse_name_first_feature) {
        Ok(feature) => Ok(feature),
        Err(_) => parse_value_first_feature(input),
    }
}

fn parse_name_first_feature<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaFeature<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let (name, legacy_operator) = parse_media_feature_name(input)?;
    let value_type = media_feature_name_type(&name);
    let operator = match input.try_parse(|input| consume_comparison_or_colon(input, true)) {
        Ok(operator) => operator,
        Err(_) => {
            let span = span_from(&start, input.position());
            return Ok(input
                .ast_mut()
                .alloc_query_feature::<MediaFeatureId>(QueryFeatureData::Boolean { name }, span));
        }
    };
    if operator.is_some() && legacy_operator.is_some() {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    let value = parse_media_feature_value(input, value_type)?;
    if !media_feature_value_matches(input.ast(), &value, value_type) {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }
    let data = if let Some(operator) = operator.or(legacy_operator) {
        if !value_type.allows_ranges() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        QueryFeatureData::Range {
            name,
            operator,
            value,
        }
    } else {
        QueryFeatureData::Plain { name, value }
    };
    let span = span_from(&start, input.position());
    Ok(input
        .ast_mut()
        .alloc_query_feature::<MediaFeatureId>(data, span))
}

fn parse_value_first_feature<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaFeature<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let value_type = loop {
        if let Ok((name, legacy_operator)) = input.try_parse(parse_media_feature_name) {
            if legacy_operator.is_some() {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            break media_feature_name_type(&name);
        }
        if input.next().is_err() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
    };
    input.reset(&start);

    let start_value = parse_media_feature_value(input, value_type)?;
    let start_operator = consume_comparison_or_colon(input, false)?
        .ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
    let (name, legacy_operator) = parse_media_feature_name(input)?;
    if legacy_operator.is_some()
        || !value_type.allows_ranges()
        || !media_feature_value_matches(input.ast(), &start_value, value_type)
    {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    }

    if let Ok(end_operator) = input.try_parse(|input| consume_comparison_or_colon(input, false)) {
        let end_operator =
            end_operator.ok_or_else(|| input.new_custom_error(ParserError::InvalidValue))?;
        if !comparisons_form_interval(&start_operator, &end_operator) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let end_value = parse_media_feature_value(input, value_type)?;
        if !media_feature_value_matches(input.ast(), &end_value, value_type) {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        let span = span_from(&start, input.position());
        Ok(input.ast_mut().alloc_query_feature::<MediaFeatureId>(
            QueryFeatureData::Interval {
                end: end_value,
                end_operator,
                name,
                start: start_value,
                start_operator,
            },
            span,
        ))
    } else {
        let span = span_from(&start, input.position());
        Ok(input.ast_mut().alloc_query_feature::<MediaFeatureId>(
            QueryFeatureData::Range {
                name,
                operator: opposite_comparison(start_operator),
                value: start_value,
            },
            span,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MediaFeatureType {
    Length,
    Number,
    Integer,
    Boolean,
    Resolution,
    Ratio,
    Ident,
    Unknown,
}

impl MediaFeatureType {
    const fn allows_ranges(self) -> bool {
        matches!(
            self,
            Self::Length
                | Self::Number
                | Self::Integer
                | Self::Resolution
                | Self::Ratio
                | Self::Unknown
        )
    }
}

fn parse_media_feature_name<'i>(
    input: &mut Compiler<'i>,
) -> Result<
    (
        MediaFeatureName<'i, MediaFeatureId>,
        Option<MediaFeatureComparison>,
    ),
    ParseError<'i, ParserError<'i>>,
> {
    let ident = input.expect_ident()?;
    if ident.starts_with("--") {
        return Ok((MediaFeatureName::Custom(ident), None));
    }

    let (mut name, webkit_prefixed) = if starts_with_ignore_ascii_case(ident, "-webkit-") {
        (&ident[8..], true)
    } else {
        (ident, false)
    };
    let legacy_operator = if starts_with_ignore_ascii_case(name, "min-") {
        name = &name[4..];
        Some(MediaFeatureComparison::GreaterThanEqual)
    } else if starts_with_ignore_ascii_case(name, "max-") {
        name = &name[4..];
        Some(MediaFeatureComparison::LessThanEqual)
    } else {
        None
    };

    let standard = if webkit_prefixed {
        if name.eq_ignore_ascii_case("device-pixel-ratio") {
            Some(MediaFeatureId::WebkitDevicePixelRatio)
        } else {
            None
        }
    } else {
        parse_media_feature_id(name)
    };
    match standard {
        Some(id) => Ok((MediaFeatureName::Standard(id), legacy_operator)),
        None => Ok((MediaFeatureName::Unknown(ident), None)),
    }
}

fn starts_with_ignore_ascii_case(value: &str, prefix: &str) -> bool {
    value
        .get(..prefix.len())
        .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

fn parse_media_feature_id(name: &str) -> Option<MediaFeatureId> {
    match_ignore_ascii_case!(
        name,
        "width" => Some(MediaFeatureId::Width),
        "height" => Some(MediaFeatureId::Height),
        "aspect-ratio" => Some(MediaFeatureId::AspectRatio),
        "orientation" => Some(MediaFeatureId::Orientation),
        "overflow-block" => Some(MediaFeatureId::OverflowBlock),
        "overflow-inline" => Some(MediaFeatureId::OverflowInline),
        "horizontal-viewport-segments" => Some(MediaFeatureId::HorizontalViewportSegments),
        "vertical-viewport-segments" => Some(MediaFeatureId::VerticalViewportSegments),
        "display-mode" => Some(MediaFeatureId::DisplayMode),
        "resolution" => Some(MediaFeatureId::Resolution),
        "scan" => Some(MediaFeatureId::Scan),
        "grid" => Some(MediaFeatureId::Grid),
        "update" => Some(MediaFeatureId::Update),
        "environment-blending" => Some(MediaFeatureId::EnvironmentBlending),
        "color" => Some(MediaFeatureId::Color),
        "color-index" => Some(MediaFeatureId::ColorIndex),
        "monochrome" => Some(MediaFeatureId::Monochrome),
        "color-gamut" => Some(MediaFeatureId::ColorGamut),
        "dynamic-range" => Some(MediaFeatureId::DynamicRange),
        "inverted-colors" => Some(MediaFeatureId::InvertedColors),
        "pointer" => Some(MediaFeatureId::Pointer),
        "hover" => Some(MediaFeatureId::Hover),
        "any-pointer" => Some(MediaFeatureId::AnyPointer),
        "any-hover" => Some(MediaFeatureId::AnyHover),
        "nav-controls" => Some(MediaFeatureId::NavControls),
        "video-color-gamut" => Some(MediaFeatureId::VideoColorGamut),
        "video-dynamic-range" => Some(MediaFeatureId::VideoDynamicRange),
        "scripting" => Some(MediaFeatureId::Scripting),
        "prefers-reduced-motion" => Some(MediaFeatureId::PrefersReducedMotion),
        "prefers-reduced-transparency" => Some(MediaFeatureId::PrefersReducedTransparency),
        "prefers-contrast" => Some(MediaFeatureId::PrefersContrast),
        "forced-colors" => Some(MediaFeatureId::ForcedColors),
        "prefers-color-scheme" => Some(MediaFeatureId::PrefersColorScheme),
        "prefers-reduced-data" => Some(MediaFeatureId::PrefersReducedData),
        "device-width" => Some(MediaFeatureId::DeviceWidth),
        "device-height" => Some(MediaFeatureId::DeviceHeight),
        "device-aspect-ratio" => Some(MediaFeatureId::DeviceAspectRatio),
        "-webkit-device-pixel-ratio" => Some(MediaFeatureId::WebkitDevicePixelRatio),
        "-moz-device-pixel-ratio" => Some(MediaFeatureId::MozDevicePixelRatio),
        _ => None,
    )
}

const fn media_feature_id_type(id: &MediaFeatureId) -> MediaFeatureType {
    match id {
        MediaFeatureId::Width
        | MediaFeatureId::Height
        | MediaFeatureId::DeviceWidth
        | MediaFeatureId::DeviceHeight => MediaFeatureType::Length,
        MediaFeatureId::AspectRatio | MediaFeatureId::DeviceAspectRatio => MediaFeatureType::Ratio,
        MediaFeatureId::HorizontalViewportSegments
        | MediaFeatureId::VerticalViewportSegments
        | MediaFeatureId::Color
        | MediaFeatureId::ColorIndex
        | MediaFeatureId::Monochrome => MediaFeatureType::Integer,
        MediaFeatureId::Resolution => MediaFeatureType::Resolution,
        MediaFeatureId::Grid => MediaFeatureType::Boolean,
        MediaFeatureId::WebkitDevicePixelRatio | MediaFeatureId::MozDevicePixelRatio => {
            MediaFeatureType::Number
        }
        MediaFeatureId::Orientation
        | MediaFeatureId::OverflowBlock
        | MediaFeatureId::OverflowInline
        | MediaFeatureId::DisplayMode
        | MediaFeatureId::Scan
        | MediaFeatureId::Update
        | MediaFeatureId::EnvironmentBlending
        | MediaFeatureId::ColorGamut
        | MediaFeatureId::DynamicRange
        | MediaFeatureId::InvertedColors
        | MediaFeatureId::Pointer
        | MediaFeatureId::Hover
        | MediaFeatureId::AnyPointer
        | MediaFeatureId::AnyHover
        | MediaFeatureId::NavControls
        | MediaFeatureId::VideoColorGamut
        | MediaFeatureId::VideoDynamicRange
        | MediaFeatureId::Scripting
        | MediaFeatureId::PrefersReducedMotion
        | MediaFeatureId::PrefersReducedTransparency
        | MediaFeatureId::PrefersContrast
        | MediaFeatureId::ForcedColors
        | MediaFeatureId::PrefersColorScheme
        | MediaFeatureId::PrefersReducedData => MediaFeatureType::Ident,
    }
}

const fn media_feature_name_type(name: &MediaFeatureName<'_, MediaFeatureId>) -> MediaFeatureType {
    match name {
        MediaFeatureName::Standard(id) => media_feature_id_type(id),
        MediaFeatureName::Custom(_) | MediaFeatureName::Unknown(_) => MediaFeatureType::Unknown,
    }
}

fn consume_comparison_or_colon<'i>(
    input: &mut Compiler<'i>,
    allow_colon: bool,
) -> Result<Option<MediaFeatureComparison>, ParseError<'i, ParserError<'i>>> {
    let comparison = match input.next()? {
        OldToken::Colon if allow_colon => None,
        OldToken::Delim(value) if *value == "=" => Some(MediaFeatureComparison::Equal),
        OldToken::Delim(value) if *value == ">" => {
            if input.try_parse(|input| input.expect_delim('=')).is_ok() {
                Some(MediaFeatureComparison::GreaterThanEqual)
            } else {
                Some(MediaFeatureComparison::GreaterThan)
            }
        }
        OldToken::Delim(value) if *value == "<" => {
            if input.try_parse(|input| input.expect_delim('=')).is_ok() {
                Some(MediaFeatureComparison::LessThanEqual)
            } else {
                Some(MediaFeatureComparison::LessThan)
            }
        }
        _ => return Err(input.new_custom_error(ParserError::InvalidValue)),
    };
    Ok(comparison)
}

fn opposite_comparison(operator: MediaFeatureComparison) -> MediaFeatureComparison {
    match operator {
        MediaFeatureComparison::Equal => MediaFeatureComparison::Equal,
        MediaFeatureComparison::GreaterThan => MediaFeatureComparison::LessThan,
        MediaFeatureComparison::GreaterThanEqual => MediaFeatureComparison::LessThanEqual,
        MediaFeatureComparison::LessThan => MediaFeatureComparison::GreaterThan,
        MediaFeatureComparison::LessThanEqual => MediaFeatureComparison::GreaterThanEqual,
    }
}

fn comparisons_form_interval(start: &MediaFeatureComparison, end: &MediaFeatureComparison) -> bool {
    matches!(
        (start, end),
        (
            MediaFeatureComparison::GreaterThan | MediaFeatureComparison::GreaterThanEqual,
            MediaFeatureComparison::GreaterThan | MediaFeatureComparison::GreaterThanEqual
        ) | (
            MediaFeatureComparison::LessThan | MediaFeatureComparison::LessThanEqual,
            MediaFeatureComparison::LessThan | MediaFeatureComparison::LessThanEqual
        )
    )
}

fn parse_media_feature_value<'i>(
    input: &mut Compiler<'i>,
    expected: MediaFeatureType,
) -> Result<MediaFeatureValue<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    if !matches!(expected, MediaFeatureType::Unknown)
        && let Ok(data) = input.try_parse(|input| parse_known_media_feature_value(input, expected))
    {
        let span = span_from(&start, input.position());
        return Ok(input.ast_mut().alloc_media_feature_value(data, span));
    }
    let data = parse_unknown_media_feature_value(input)?;
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_media_feature_value(data, span))
}

fn parse_known_media_feature_value<'i>(
    input: &mut Compiler<'i>,
    expected: MediaFeatureType,
) -> Result<MediaFeatureValueData<'i>, ParseError<'i, ParserError<'i>>> {
    Ok(match expected {
        MediaFeatureType::Length => MediaFeatureValueData::Length(Length::parse(input)?),
        MediaFeatureType::Number => MediaFeatureValueData::Number(input.expect_number()?),
        MediaFeatureType::Integer => MediaFeatureValueData::Integer(input.expect_integer()?),
        MediaFeatureType::Boolean => {
            let value = input.expect_integer()?;
            if !matches!(value, 0 | 1) {
                return Err(input.new_custom_error(ParserError::InvalidValue));
            }
            MediaFeatureValueData::Boolean(value == 1)
        }
        MediaFeatureType::Resolution => MediaFeatureValueData::Resolution(parse_resolution(input)?),
        MediaFeatureType::Ratio => MediaFeatureValueData::Ratio(parse_ratio(input, false)?),
        MediaFeatureType::Ident => MediaFeatureValueData::Ident(input.expect_ident()?),
        MediaFeatureType::Unknown => {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
    })
}

fn parse_unknown_media_feature_value<'i>(
    input: &mut Compiler<'i>,
) -> Result<MediaFeatureValueData<'i>, ParseError<'i, ParserError<'i>>> {
    if let Ok(value) = input.try_parse(|input| parse_ratio(input, true)) {
        return Ok(MediaFeatureValueData::Ratio(value));
    }
    if let Ok(value) = input.try_parse(|input| input.expect_number()) {
        return Ok(MediaFeatureValueData::Number(value));
    }
    if let Ok(value) = input.try_parse(Length::parse) {
        return Ok(MediaFeatureValueData::Length(value));
    }
    if let Ok(value) = input.try_parse(parse_resolution) {
        return Ok(MediaFeatureValueData::Resolution(value));
    }
    if let Ok(value) = input.try_parse(parse_environment_variable) {
        return Ok(MediaFeatureValueData::Env(value));
    }
    Ok(MediaFeatureValueData::Ident(input.expect_ident()?))
}

fn media_feature_value_matches<'i>(
    ast: &Ast<'i>,
    value: &MediaFeatureValue<'i>,
    expected: MediaFeatureType,
) -> bool {
    matches!(expected, MediaFeatureType::Unknown)
        || matches!(
            (value.data(ast), expected),
            (MediaFeatureValueData::Length(_), MediaFeatureType::Length)
                | (MediaFeatureValueData::Number(_), MediaFeatureType::Number)
                | (MediaFeatureValueData::Integer(_), MediaFeatureType::Integer)
                | (MediaFeatureValueData::Boolean(_), MediaFeatureType::Boolean)
                | (
                    MediaFeatureValueData::Resolution(_),
                    MediaFeatureType::Resolution
                )
                | (MediaFeatureValueData::Ratio(_), MediaFeatureType::Ratio)
                | (MediaFeatureValueData::Ident(_), MediaFeatureType::Ident)
                | (MediaFeatureValueData::Env(_), _)
        )
}

fn parse_resolution<'i>(
    input: &mut Compiler<'i>,
) -> Result<Resolution, ParseError<'i, ParserError<'i>>> {
    match input.next()? {
        OldToken::Dimension {
            unit: OldUnit::Dpi,
            value,
        } => Ok(Resolution::Dpi(*value)),
        OldToken::Dimension {
            unit: OldUnit::Dpcm,
            value,
        } => Ok(Resolution::Dpcm(*value)),
        OldToken::Dimension {
            unit: OldUnit::Dppx | OldUnit::ResolutionX,
            value,
        } => Ok(Resolution::Dppx(*value)),
        _ => Err(input.new_custom_error(ParserError::InvalidValue)),
    }
}

fn parse_ratio<'i>(
    input: &mut Compiler<'i>,
    require_slash: bool,
) -> Result<Ratio<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let numerator = input.expect_number()?;
    let denominator = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(input.expect_number()?)
    } else if require_slash {
        return Err(input.new_custom_error(ParserError::InvalidValue));
    } else {
        None
    };
    let span = span_from(&start, input.position());
    Ok(input.ast_mut().alloc_ratio(
        RatioData {
            denominator,
            numerator,
        },
        span,
    ))
}

fn parse_environment_variable<'i>(
    input: &mut Compiler<'i>,
) -> Result<EnvironmentVariable<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    input.expect_function_matching("env")?;
    let environment = input.parse_nested_block(|input| {
        let ident = input.expect_ident()?;
        let name = if ident.starts_with("--") {
            let reference =
                with_token_span!(input, |span| input.ast_mut().alloc_dashed_ident_reference(
                    DashedIdentReferenceData { from: None, ident },
                    span,
                ));
            EnvironmentVariableNameData::Custom(reference)
        } else if let Some(name) = parse_ua_environment_variable(ident) {
            EnvironmentVariableNameData::UA(name)
        } else {
            EnvironmentVariableNameData::Unknown(ident)
        };
        let name = with_token_span!(input, |span| input
            .ast_mut()
            .alloc_environment_variable_name(name, span));
        let mut indices = Vec::new();
        while let Ok(index) = input.try_parse(|input| input.expect_integer()) {
            indices.push(index);
        }
        let fallback = if input.try_parse(|input| input.expect_comma()).is_ok() {
            Some(collect_tokens(input, 0)?)
        } else {
            None
        };
        input.expect_exhausted()?;
        let indices = input.ast().alloc_slice(&indices);
        Ok::<_, ParseError<'i, ParserError<'i>>>(EnvironmentVariableData {
            fallback,
            indices,
            name,
        })
    })?;
    let span = span_from(&start, input.position());
    Ok(input
        .ast_mut()
        .alloc_environment_variable(environment, span))
}

fn parse_ua_environment_variable(name: &str) -> Option<UAEnvironmentVariable> {
    match_ignore_ascii_case!(
        name,
        "safe-area-inset-top" => Some(UAEnvironmentVariable::SafeAreaInsetTop),
        "safe-area-inset-right" => Some(UAEnvironmentVariable::SafeAreaInsetRight),
        "safe-area-inset-bottom" => Some(UAEnvironmentVariable::SafeAreaInsetBottom),
        "safe-area-inset-left" => Some(UAEnvironmentVariable::SafeAreaInsetLeft),
        "viewport-segment-width" => Some(UAEnvironmentVariable::ViewportSegmentWidth),
        "viewport-segment-height" => Some(UAEnvironmentVariable::ViewportSegmentHeight),
        "viewport-segment-top" => Some(UAEnvironmentVariable::ViewportSegmentTop),
        "viewport-segment-left" => Some(UAEnvironmentVariable::ViewportSegmentLeft),
        "viewport-segment-bottom" => Some(UAEnvironmentVariable::ViewportSegmentBottom),
        "viewport-segment-right" => Some(UAEnvironmentVariable::ViewportSegmentRight),
        _ => None,
    )
}

pub(crate) fn parse_supports_condition<'i>(
    input: &mut Compiler<'i>,
    source: &'i str,
) -> SupportsCondition<'i> {
    with_token_span!(input, |span| input
        .ast_mut()
        .alloc_supports_condition(SupportsConditionData::Unknown(source), span))
}

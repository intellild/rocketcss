//! Selector parsing into the flattened AST.

use super::{prelude::*, values::collect_tokens};

impl<'i> Parse<'i> for &'i [Selector<'i>] {
    fn parse(input: &mut Compiler<'i>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        parse_selector_list(input, 0)
    }
}

pub(crate) fn parse_selector_list<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<&'i [Selector<'i>], ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let parsed = input.parse_comma_separated(|input| parse_selector(input, depth + 1))?;
    Ok(input.ast().alloc_slice(&parsed))
}

pub(crate) fn parse_selector_list_with_recovery<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<&'i [Selector<'i>], ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut selectors = Vec::new();

    loop {
        input.skip_whitespace();
        let start_state = input.state();
        let start = input.position();
        match input.parse_until_before(Delimiter::Comma, |input| parse_selector(input, depth + 1)) {
            Ok(selector) => selectors.push(selector),
            Err(_) => {
                let raw = input.slice(start..input.position()).trim();
                let atom = input.intern(raw);
                let span = span_from(&start_state, input.position());
                selectors.push(
                    input
                        .ast_mut()
                        .alloc_selector(SelectorData::Unparsed(atom), span),
                );
            }
        }

        match input.next() {
            Ok(OldToken::Comma) => {}
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => unreachable!(),
            Err(error) => return Err(error.into()),
        }
    }

    Ok(input.ast().alloc_slice(&selectors))
}

pub(crate) fn parse_selector<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
) -> Result<Selector<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let start = input.state();
    let mut components = Vec::new();
    let mut pending_descendant = false;
    let mut can_have_descendant = false;

    loop {
        let token = match input.next_including_whitespace() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        if matches!(token, OldToken::WhiteSpace(_)) {
            pending_descendant |= can_have_descendant;
            continue;
        }

        let explicit_combinator = match &token {
            OldToken::Delim(">") => Some(Combinator::Child),
            OldToken::Delim("+") => Some(Combinator::NextSibling),
            OldToken::Delim("~") => Some(Combinator::LaterSibling),
            OldToken::Delim("/")
                if input
                    .try_parse(|input| {
                        input.expect_ident_matching("deep")?;
                        input.expect_delim('/')
                    })
                    .is_ok() =>
            {
                Some(Combinator::Deep)
            }
            _ => None,
        };
        if let Some(combinator) = explicit_combinator {
            push_component(
                input,
                &mut components,
                SelectorComponentData::Combinator(combinator),
            );
            pending_descendant = false;
            can_have_descendant = false;
            continue;
        }

        if pending_descendant && can_have_descendant {
            push_component(
                input,
                &mut components,
                SelectorComponentData::Combinator(Combinator::Descendant),
            );
        }
        pending_descendant = false;

        match token {
            OldToken::Ident(name) if input.try_parse(|input| input.expect_delim('|')).is_ok() => {
                let prefix = input.intern(name);
                let url = input.intern("");
                push_component(
                    input,
                    &mut components,
                    SelectorComponentData::Namespace { prefix, url },
                );
                let data = parse_type_selector(input)?;
                push_component(input, &mut components, data);
            }
            OldToken::Ident(name) => {
                let data = local_name(name, input);
                push_component(input, &mut components, data);
            }
            OldToken::IdHash(id) => {
                let id = input.intern(id);
                push_component(input, &mut components, SelectorComponentData::Id(id));
            }
            OldToken::Delim("*") if input.try_parse(|input| input.expect_delim('|')).is_ok() => {
                push_component(
                    input,
                    &mut components,
                    SelectorComponentData::ExplicitAnyNamespace,
                );
                let data = parse_type_selector(input)?;
                push_component(input, &mut components, data);
            }
            OldToken::Delim("*") => push_component(
                input,
                &mut components,
                SelectorComponentData::ExplicitUniversalType,
            ),
            OldToken::Delim("|") => {
                push_component(
                    input,
                    &mut components,
                    SelectorComponentData::ExplicitNoNamespace,
                );
                let data = parse_type_selector(input)?;
                push_component(input, &mut components, data);
            }
            OldToken::Delim("&") => {
                push_component(input, &mut components, SelectorComponentData::Nesting);
            }
            OldToken::Delim(".") => {
                let name = input.expect_ident()?;
                let name = input.intern(name);
                push_component(input, &mut components, SelectorComponentData::Class(name));
            }
            OldToken::Colon => {
                let span = token_span(input);
                let data = parse_pseudo(input, depth + 1, span)?;
                components.push(input.ast_mut().alloc_selector_component(data, span));
            }
            OldToken::SquareBracketBlock => {
                let span = token_span(input);
                let data = input.parse_nested_block(|input| parse_attribute(input))?;
                components.push(input.ast_mut().alloc_selector_component(data, span));
            }
            _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
        }
        can_have_descendant = true;
    }

    if components.is_empty()
        || matches!(
            input.ast().data(components[components.len() - 1]),
            SelectorComponentData::Combinator(_)
        )
    {
        return Err(input.new_custom_error(ParserError::InvalidSelector));
    }
    let span = span_from(&start, input.position());
    let components = input.ast().alloc_slice(&components);
    Ok(input
        .ast_mut()
        .alloc_selector(SelectorData::Parsed(components), span))
}

#[inline]
fn push_component<'i>(
    input: &mut Compiler<'i>,
    components: &mut Vec<SelectorComponent<'i>>,
    data: SelectorComponentData<'i>,
) {
    let span = token_span(input);
    components.push(input.ast_mut().alloc_selector_component(data, span));
}

fn local_name<'i>(name: &'i str, input: &mut Compiler<'i>) -> SelectorComponentData<'i> {
    SelectorComponentData::LocalName {
        name: input.intern(name),
        lower_name: input.intern_ascii_lowercase(name),
    }
}

fn parse_type_selector<'i>(
    input: &mut Compiler<'i>,
) -> Result<SelectorComponentData<'i>, ParseError<'i, ParserError<'i>>> {
    match input.next()?.clone() {
        OldToken::Ident(name) => Ok(local_name(name, input)),
        OldToken::Delim("*") => Ok(SelectorComponentData::ExplicitUniversalType),
        _ => Err(input.new_custom_error(ParserError::InvalidSelector)),
    }
}

pub(crate) fn parse_pseudo<'i>(
    input: &mut Compiler<'i>,
    depth: usize,
    span: Span,
) -> Result<SelectorComponentData<'i>, ParseError<'i, ParserError<'i>>> {
    let is_element = input.try_parse(Compiler::expect_colon).is_ok();
    let token = input.next()?.clone();

    match token {
        OldToken::Ident(name) if is_element => {
            let data = pseudo_element(name, input);
            Ok(SelectorComponentData::PseudoElement(
                input.ast_mut().alloc_pseudo_element(data, span),
            ))
        }
        OldToken::Ident(name) if name.eq_ignore_ascii_case("root") => {
            Ok(SelectorComponentData::Root)
        }
        OldToken::Ident(name) if name.eq_ignore_ascii_case("empty") => {
            Ok(SelectorComponentData::Empty)
        }
        OldToken::Ident(name) if name.eq_ignore_ascii_case("scope") => {
            Ok(SelectorComponentData::Scope)
        }
        OldToken::Ident(name) => {
            let data = pseudo_class(name, input);
            Ok(SelectorComponentData::PseudoClass(
                input.ast_mut().alloc_pseudo_class(data, span),
            ))
        }
        OldToken::Function(name) if is_element => {
            let arguments = input.parse_nested_block(|input| collect_tokens(input, depth + 1))?;
            let data = PseudoElementData::CustomFunction {
                name: input.intern(name),
                arguments,
            };
            Ok(SelectorComponentData::PseudoElement(
                input.ast_mut().alloc_pseudo_element(data, span),
            ))
        }
        OldToken::Function(name) => {
            if let Some(kind) = nth_type(name) {
                let (data, selectors) = input.parse_nested_block(|input| {
                    let start = input.position();
                    input.expect_no_error_token()?;
                    let raw = input.slice_from(start).trim();
                    let lower = raw.to_ascii_lowercase();
                    let (affine, selector_source) = if let Some(index) = lower.find(" of") {
                        (&raw[..index], Some(raw[index + 3..].trim()))
                    } else {
                        (raw, None)
                    };
                    let data = parse_nth_affine(affine, kind)
                        .ok_or_else(|| input.new_custom_error(ParserError::InvalidSelector))?;
                    let selectors = selector_source
                        .map(|source| parse_selector_string(input, source, depth + 1))
                        .transpose()?;
                    Ok::<_, ParseError<'i, ParserError<'i>>>((data, selectors))
                })?;
                let data = input.ast_mut().alloc_nth_selector_data(data, span);
                if let Some(selectors) = selectors {
                    Ok(SelectorComponentData::NthOf { data, selectors })
                } else {
                    Ok(SelectorComponentData::Nth(data))
                }
            } else if name.eq_ignore_ascii_case("not") {
                Ok(SelectorComponentData::Negation(input.parse_nested_block(
                    |input| parse_selector_list(input, depth + 1),
                )?))
            } else if name.eq_ignore_ascii_case("is") {
                Ok(SelectorComponentData::Is(input.parse_nested_block(
                    |input| parse_selector_list(input, depth + 1),
                )?))
            } else if name.eq_ignore_ascii_case("where") {
                Ok(SelectorComponentData::Where(input.parse_nested_block(
                    |input| {
                        input.skip_whitespace();
                        if input.is_exhausted() {
                            Ok(input.ast().alloc_slice::<Selector<'i>>(&[]))
                        } else {
                            parse_selector_list(input, depth + 1)
                        }
                    },
                )?))
            } else if name.eq_ignore_ascii_case("has") {
                Ok(SelectorComponentData::Has(input.parse_nested_block(
                    |input| parse_selector_list(input, depth + 1),
                )?))
            } else {
                let arguments =
                    input.parse_nested_block(|input| collect_tokens(input, depth + 1))?;
                let data = PseudoClassData::CustomFunction {
                    name: input.intern(name),
                    arguments,
                };
                Ok(SelectorComponentData::PseudoClass(
                    input.ast_mut().alloc_pseudo_class(data, span),
                ))
            }
        }
        _ => Err(input.new_custom_error(ParserError::InvalidSelector)),
    }
}

fn nth_type(name: &str) -> Option<NthType> {
    match_ignore_ascii_case!(
        name,
        "nth-child" => Some(NthType::Child),
        "nth-last-child" => Some(NthType::LastChild),
        "nth-of-type" => Some(NthType::OfType),
        "nth-last-of-type" => Some(NthType::LastOfType),
        "nth-col" => Some(NthType::Col),
        "nth-last-col" => Some(NthType::LastCol),
        _ => None,
    )
}

fn parse_nth_affine(source: &str, kind: NthType) -> Option<NthSelectorDataData> {
    let value: String = source
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    let (a, b) = if value == "odd" {
        (2, 1)
    } else if value == "even" {
        (2, 0)
    } else if let Some(index) = value.find('n') {
        let a = match &value[..index] {
            "" | "+" => 1,
            "-" => -1,
            value => value.parse().ok()?,
        };
        let b = if index + 1 == value.len() {
            0
        } else {
            value[index + 1..].parse().ok()?
        };
        (a, b)
    } else {
        (0, value.parse().ok()?)
    };
    Some(NthSelectorDataData {
        kind,
        is_function: true,
        a,
        b,
    })
}

pub(crate) fn parse_attribute<'i>(
    input: &mut Compiler<'i>,
) -> Result<SelectorComponentData<'i>, ParseError<'i, ParserError<'i>>> {
    let start = input.state();
    let first = input.next()?.clone();
    let (namespace, name) = match first {
        OldToken::Delim("|") => (None, input.expect_ident()?),
        OldToken::Delim("*") => {
            input.expect_delim('|')?;
            (Some(NamespaceConstraint::Any), input.expect_ident()?)
        }
        OldToken::Ident(prefix) if input.try_parse(|input| input.expect_delim('|')).is_ok() => (
            Some(NamespaceConstraint::Specific {
                prefix: input.intern(prefix),
                url: input.intern(""),
            }),
            input.expect_ident()?,
        ),
        OldToken::Ident(name) => (None, name),
        _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
    };
    let lower_name = input.intern_ascii_lowercase(name);
    let name = input.intern(name);
    let span = span_from(&start, input.position());
    if input.is_exhausted() {
        return Ok(match namespace {
            None => SelectorComponentData::AttributeInNoNamespaceExists {
                local_name: name,
                local_name_lower: lower_name,
            },
            Some(namespace) => {
                let attr = input.ast_mut().alloc_attr_selector(
                    AttrSelectorData {
                        namespace: Some(namespace),
                        local_name: name,
                        local_name_lower: lower_name,
                        operation: AttrOperation::Exists,
                        never_matches: false,
                    },
                    span,
                );
                SelectorComponentData::AttributeOther(attr)
            }
        });
    }

    let operator = match input.next()? {
        OldToken::Delim("=") => AttrSelectorOperator::Equal,
        OldToken::IncludeMatch => AttrSelectorOperator::Includes,
        OldToken::DashMatch => AttrSelectorOperator::DashMatch,
        OldToken::PrefixMatch => AttrSelectorOperator::Prefix,
        OldToken::SubstringMatch => AttrSelectorOperator::Substring,
        OldToken::SuffixMatch => AttrSelectorOperator::Suffix,
        _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
    };
    let value = input.expect_ident_or_string()?;
    let value = input.intern(value);
    let case_sensitivity = if let Ok(flag) = input.try_parse(Compiler::expect_ident) {
        match_ignore_ascii_case!(
            flag,
            "i" => ParsedCaseSensitivity::AsciiCaseInsensitive,
            "s" => ParsedCaseSensitivity::ExplicitCaseSensitive,
            _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
        )
    } else {
        ParsedCaseSensitivity::CaseSensitive
    };
    input.expect_exhausted()?;

    Ok(match namespace {
        None => SelectorComponentData::AttributeInNoNamespace {
            local_name: name,
            operator,
            value,
            case_sensitivity,
            never_matches: false,
        },
        Some(namespace) => {
            let span = span_from(&start, input.position());
            let attr = input.ast_mut().alloc_attr_selector(
                AttrSelectorData {
                    namespace: Some(namespace),
                    local_name: name,
                    local_name_lower: lower_name,
                    operation: AttrOperation::WithValue {
                        operator,
                        case_sensitivity,
                        expected_value: value,
                    },
                    never_matches: false,
                },
                span,
            );
            SelectorComponentData::AttributeOther(attr)
        }
    })
}

pub(crate) fn pseudo_class<'i>(name: &str, input: &mut Compiler<'i>) -> PseudoClassData<'i> {
    match_ignore_ascii_case!(
        name,
        "hover" => PseudoClassData::Hover,
        "active" => PseudoClassData::Active,
        "focus" => PseudoClassData::Focus,
        "focus-visible" => PseudoClassData::FocusVisible,
        "focus-within" => PseudoClassData::FocusWithin,
        "visited" => PseudoClassData::Visited,
        "link" => PseudoClassData::Link,
        "checked" => PseudoClassData::Checked,
        "disabled" => PseudoClassData::Disabled,
        "enabled" => PseudoClassData::Enabled,
        _ => PseudoClassData::Custom {
            name: input.intern(name),
        },
    )
}

pub(crate) fn pseudo_element<'i>(name: &str, input: &mut Compiler<'i>) -> PseudoElementData<'i> {
    match_ignore_ascii_case!(
        name,
        "before" => PseudoElementData::Before,
        "after" => PseudoElementData::After,
        "first-line" => PseudoElementData::FirstLine,
        "first-letter" => PseudoElementData::FirstLetter,
        "marker" => PseudoElementData::Marker,
        "-moz-selection" => PseudoElementData::Selection(VendorPrefix::MOZ),
        "placeholder" => PseudoElementData::Placeholder(VendorPrefix::NONE),
        "-webkit-placeholder" => PseudoElementData::Placeholder(VendorPrefix::WEBKIT),
        _ => PseudoElementData::Custom {
            name: input.intern(name),
        },
    )
}

pub(crate) fn parse_selector_string<'i>(
    input: &mut Compiler<'i>,
    source: &'i str,
    depth: usize,
) -> Result<&'i [Selector<'i>], ParseError<'i, ParserError<'i>>> {
    input.with_source(source, |input| {
        input.parse_entirely(|input| parse_selector_list(input, depth))
    })
}

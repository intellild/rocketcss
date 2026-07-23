use super::{
    stylesheet::check_depth,
    values::{ascii_lowercase, collect_tokens},
};
use crate::prelude::*;

impl<'i> Parse<'i> for SelectorList<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ParserError<'i>>> {
        let allocator = input.allocator();
        parse_selector_list(input, allocator, 0)
    }
}

pub(super) fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorList<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let parsed =
        input.parse_comma_separated(|input| parse_selector(input, allocator, depth + 1))?;
    let mut selectors = allocator.vec();
    selectors.extend(parsed);
    Ok(selectors)
}

pub(super) fn parse_selector_list_with_recovery<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorList<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut selectors = allocator.vec();

    loop {
        input.skip_whitespace();
        let start = input.position();
        match input.parse_until_before(Delimiter::Comma, |input| {
            parse_selector(input, allocator, depth + 1)
        }) {
            Ok(selector) => selectors.push(selector),
            Err(_) => {
                let raw = input.slice(start..input.position()).trim();
                selectors.push(Selector::Unparsed(raw));
            }
        }

        match input.next() {
            Ok(ValueToken::Comma) => {}
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Ok(_) => unreachable!(),
            Err(error) => return Err(error.into()),
        }
    }

    Ok(selectors)
}

pub(super) fn parse_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<Selector<'i>, ParseError<'i, ParserError<'i>>> {
    check_depth(input, depth)?;
    let mut selector = allocator.vec();
    let mut pending_descendant = false;
    let mut can_have_descendant = false;

    loop {
        let token = match input.next_including_whitespace() {
            Ok(token) => token.clone(),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(error.into()),
        };

        if matches!(token, ValueToken::WhiteSpace(_)) {
            pending_descendant |= can_have_descendant;
            continue;
        }

        let explicit_combinator = match &token {
            ValueToken::Delim(">") => Some(Combinator::Child),
            ValueToken::Delim("+") => Some(Combinator::NextSibling),
            ValueToken::Delim("~") => Some(Combinator::LaterSibling),
            ValueToken::Delim("/")
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
            selector.push(SelectorComponent::Combinator(combinator));
            pending_descendant = false;
            can_have_descendant = false;
            continue;
        }

        if pending_descendant && can_have_descendant {
            selector.push(SelectorComponent::Combinator(Combinator::Descendant));
        }
        pending_descendant = false;

        let component = match token {
            ValueToken::Ident(name) if input.try_parse(|input| input.expect_delim('|')).is_ok() => {
                selector.push(SelectorComponent::Namespace {
                    prefix: name,
                    url: "",
                });
                parse_type_selector(input, allocator)?
            }
            ValueToken::Ident(name) => local_name(name, allocator),
            ValueToken::IdHash(id) => SelectorComponent::Id(id),
            ValueToken::Delim("*") if input.try_parse(|input| input.expect_delim('|')).is_ok() => {
                selector.push(SelectorComponent::ExplicitAnyNamespace);
                parse_type_selector(input, allocator)?
            }
            ValueToken::Delim("*") => SelectorComponent::ExplicitUniversalType,
            ValueToken::Delim("|") => {
                selector.push(SelectorComponent::ExplicitNoNamespace);
                parse_type_selector(input, allocator)?
            }
            ValueToken::Delim("&") => SelectorComponent::Nesting,
            ValueToken::Delim(".") => SelectorComponent::Class(input.expect_ident()?),
            ValueToken::Colon => parse_pseudo(input, allocator, depth + 1)?,
            ValueToken::SquareBracketBlock => {
                input.parse_nested_block(|input| parse_attribute(input, allocator))?
            }
            _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
        };
        selector.push(component);
        can_have_descendant = true;
    }

    if selector.is_empty() || matches!(selector.last(), Some(SelectorComponent::Combinator(_))) {
        return Err(input.new_custom_error(ParserError::InvalidSelector));
    }
    Ok(Selector::parsed(selector))
}

fn local_name<'i>(name: &'i str, allocator: &'i Allocator) -> SelectorComponent<'i> {
    SelectorComponent::LocalName {
        name,
        lower_name: ascii_lowercase(name, allocator),
    }
}

fn parse_type_selector<'i>(
    input: &mut Parser<'i, '_>,
    allocator: &'i Allocator,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    match input.next()? {
        ValueToken::Ident(name) => Ok(local_name(name, allocator)),
        ValueToken::Delim("*") => Ok(SelectorComponent::ExplicitUniversalType),
        _ => Err(input.new_custom_error(ParserError::InvalidSelector)),
    }
}

pub(super) fn parse_pseudo<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    let is_element = input.try_parse(Parser::expect_colon).is_ok();
    let token = input.next()?.clone();

    match token {
        ValueToken::Ident(name) if is_element => Ok(SelectorComponent::PseudoElement(
            allocator.boxed(pseudo_element(name)),
        )),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("root") => Ok(SelectorComponent::Root),
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("empty") => {
            Ok(SelectorComponent::Empty)
        }
        ValueToken::Ident(name) if name.eq_ignore_ascii_case("scope") => {
            Ok(SelectorComponent::Scope)
        }
        ValueToken::Ident(name) => Ok(SelectorComponent::PseudoClass(
            allocator.boxed(pseudo_class(name)),
        )),
        ValueToken::Function(name) if is_element => {
            let arguments =
                input.parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
            Ok(SelectorComponent::PseudoElement(
                allocator.boxed(PseudoElement::CustomFunction { name, arguments }),
            ))
        }
        ValueToken::Function(name) => {
            let component =
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
                            .map(|source| parse_selector_string(source, allocator, depth + 1))
                            .transpose()?;
                        Ok::<_, ParseError<'i, ParserError<'i>>>((data, selectors))
                    })?;
                    if let Some(selectors) = selectors {
                        SelectorComponent::NthOf { data, selectors }
                    } else {
                        SelectorComponent::Nth(data)
                    }
                } else if name.eq_ignore_ascii_case("not") {
                    SelectorComponent::Negation(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("is") {
                    SelectorComponent::Is(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("where") {
                    SelectorComponent::Where(input.parse_nested_block(|input| {
                        input.skip_whitespace();
                        if input.is_exhausted() {
                            Ok(allocator.vec())
                        } else {
                            parse_selector_list(input, allocator, depth + 1)
                        }
                    })?)
                } else if name.eq_ignore_ascii_case("has") {
                    SelectorComponent::Has(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else {
                    let arguments = input
                        .parse_nested_block(|input| collect_tokens(input, allocator, depth + 1))?;
                    SelectorComponent::PseudoClass(
                        allocator.boxed(PseudoClass::CustomFunction { name, arguments }),
                    )
                };
            Ok(component)
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

fn parse_nth_affine(source: &str, kind: NthType) -> Option<NthSelectorData> {
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
    Some(NthSelectorData {
        kind,
        is_function: true,
        a,
        b,
    })
}

pub(super) fn parse_attribute<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    let first = input.next()?.clone();
    let (namespace, name) = match first {
        ValueToken::Delim("|") => (None, input.expect_ident()?),
        ValueToken::Delim("*") => {
            input.expect_delim('|')?;
            (Some(NamespaceConstraint::Any), input.expect_ident()?)
        }
        ValueToken::Ident(prefix) if input.try_parse(|input| input.expect_delim('|')).is_ok() => (
            Some(NamespaceConstraint::Specific { prefix, url: "" }),
            input.expect_ident()?,
        ),
        ValueToken::Ident(name) => (None, name),
        _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
    };
    let lower_name = ascii_lowercase(name, allocator);
    if input.is_exhausted() {
        return Ok(match namespace {
            None => SelectorComponent::AttributeInNoNamespaceExists {
                local_name: name,
                local_name_lower: lower_name,
            },
            Some(namespace) => SelectorComponent::AttributeOther(allocator.boxed(AttrSelector {
                namespace: Some(namespace),
                local_name: name,
                local_name_lower: lower_name,
                operation: AttrOperation::Exists,
                never_matches: false,
            })),
        });
    }

    let operator = match input.next()? {
        ValueToken::Delim("=") => AttrSelectorOperator::Equal,
        ValueToken::IncludeMatch => AttrSelectorOperator::Includes,
        ValueToken::DashMatch => AttrSelectorOperator::DashMatch,
        ValueToken::PrefixMatch => AttrSelectorOperator::Prefix,
        ValueToken::SubstringMatch => AttrSelectorOperator::Substring,
        ValueToken::SuffixMatch => AttrSelectorOperator::Suffix,
        _ => return Err(input.new_custom_error(ParserError::InvalidSelector)),
    };
    let value = input.expect_ident_or_string()?;
    let case_sensitivity = if let Ok(flag) = input.try_parse(Parser::expect_ident) {
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
        None => SelectorComponent::AttributeInNoNamespace {
            local_name: name,
            operator,
            value,
            case_sensitivity,
            never_matches: false,
        },
        Some(namespace) => SelectorComponent::AttributeOther(allocator.boxed(AttrSelector {
            namespace: Some(namespace),
            local_name: name,
            local_name_lower: lower_name,
            operation: AttrOperation::WithValue {
                operator,
                case_sensitivity,
                expected_value: value,
            },
            never_matches: false,
        })),
    })
}

pub(super) fn pseudo_class(name: &str) -> PseudoClass<'_> {
    match_ignore_ascii_case!(
        name,
        "hover" => PseudoClass::Hover,
        "active" => PseudoClass::Active,
        "focus" => PseudoClass::Focus,
        "focus-visible" => PseudoClass::FocusVisible,
        "focus-within" => PseudoClass::FocusWithin,
        "visited" => PseudoClass::Visited,
        "link" => PseudoClass::Link,
        "checked" => PseudoClass::Checked,
        "disabled" => PseudoClass::Disabled,
        "enabled" => PseudoClass::Enabled,
        _ => PseudoClass::Custom { name },
    )
}

pub(super) fn pseudo_element(name: &str) -> PseudoElement<'_> {
    match_ignore_ascii_case!(
        name,
        "before" => PseudoElement::Before,
        "after" => PseudoElement::After,
        "first-line" => PseudoElement::FirstLine,
        "first-letter" => PseudoElement::FirstLetter,
        "marker" => PseudoElement::Marker,
        "selection" => PseudoElement::Selection(VendorPrefix::NONE),
        "-webkit-selection" => PseudoElement::Selection(VendorPrefix::WEBKIT),
        "-moz-selection" => PseudoElement::Selection(VendorPrefix::MOZ),
        "-ms-selection" => PseudoElement::Selection(VendorPrefix::MS),
        "-o-selection" => PseudoElement::Selection(VendorPrefix::O),
        "placeholder" => PseudoElement::Placeholder(VendorPrefix::NONE),
        "-webkit-placeholder" => PseudoElement::Placeholder(VendorPrefix::WEBKIT),
        "-moz-placeholder" => PseudoElement::Placeholder(VendorPrefix::MOZ),
        "-ms-placeholder" => PseudoElement::Placeholder(VendorPrefix::MS),
        "-o-placeholder" => PseudoElement::Placeholder(VendorPrefix::O),
        _ => PseudoElement::Custom { name },
    )
}

pub(super) fn parse_selector_string<'i>(
    source: &'i str,
    allocator: &'i Allocator,
    depth: usize,
) -> Result<SelectorList<'i>, ParseError<'i, ParserError<'i>>> {
    let mut input = ParserInput::new(source, allocator);
    let mut parser = Parser::new(&mut input);
    parser.parse_entirely(|input| parse_selector_list(input, allocator, depth))
}

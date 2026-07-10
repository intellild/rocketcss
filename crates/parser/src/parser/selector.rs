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
            ValueToken::Ident(name) => SelectorComponent::LocalName {
                name,
                lower_name: ascii_lowercase(name, allocator),
            },
            ValueToken::IdHash(id) => SelectorComponent::Id(id),
            ValueToken::Delim("*") => SelectorComponent::ExplicitUniversalType,
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
    Ok(selector)
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
                if name.eq_ignore_ascii_case("not") {
                    SelectorComponent::Negation(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("is") {
                    SelectorComponent::Is(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
                    })?)
                } else if name.eq_ignore_ascii_case("where") {
                    SelectorComponent::Where(input.parse_nested_block(|input| {
                        parse_selector_list(input, allocator, depth + 1)
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

pub(super) fn parse_attribute<'i, 't>(
    input: &mut Parser<'i, 't>,
    allocator: &'i Allocator,
) -> Result<SelectorComponent<'i>, ParseError<'i, ParserError<'i>>> {
    let name = input.expect_ident()?;
    let lower_name = ascii_lowercase(name, allocator);
    if input.is_exhausted() {
        return Ok(SelectorComponent::AttributeInNoNamespaceExists {
            local_name: name,
            local_name_lower: lower_name,
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
        if flag.eq_ignore_ascii_case("i") {
            ParsedCaseSensitivity::AsciiCaseInsensitive
        } else if flag.eq_ignore_ascii_case("s") {
            ParsedCaseSensitivity::ExplicitCaseSensitive
        } else {
            return Err(input.new_custom_error(ParserError::InvalidSelector));
        }
    } else {
        ParsedCaseSensitivity::CaseSensitive
    };
    input.expect_exhausted()?;

    Ok(SelectorComponent::AttributeInNoNamespace {
        local_name: name,
        operator,
        value,
        case_sensitivity,
        never_matches: false,
    })
}

pub(super) fn pseudo_class(name: &str) -> PseudoClass<'_> {
    if name.eq_ignore_ascii_case("hover") {
        PseudoClass::Hover
    } else if name.eq_ignore_ascii_case("active") {
        PseudoClass::Active
    } else if name.eq_ignore_ascii_case("focus") {
        PseudoClass::Focus
    } else if name.eq_ignore_ascii_case("focus-visible") {
        PseudoClass::FocusVisible
    } else if name.eq_ignore_ascii_case("focus-within") {
        PseudoClass::FocusWithin
    } else if name.eq_ignore_ascii_case("visited") {
        PseudoClass::Visited
    } else if name.eq_ignore_ascii_case("link") {
        PseudoClass::Link
    } else if name.eq_ignore_ascii_case("checked") {
        PseudoClass::Checked
    } else if name.eq_ignore_ascii_case("disabled") {
        PseudoClass::Disabled
    } else if name.eq_ignore_ascii_case("enabled") {
        PseudoClass::Enabled
    } else {
        PseudoClass::Custom { name }
    }
}

pub(super) fn pseudo_element(name: &str) -> PseudoElement<'_> {
    if name.eq_ignore_ascii_case("before") {
        PseudoElement::Before
    } else if name.eq_ignore_ascii_case("after") {
        PseudoElement::After
    } else if name.eq_ignore_ascii_case("first-line") {
        PseudoElement::FirstLine
    } else if name.eq_ignore_ascii_case("first-letter") {
        PseudoElement::FirstLetter
    } else if name.eq_ignore_ascii_case("marker") {
        PseudoElement::Marker
    } else if name.eq_ignore_ascii_case("placeholder") {
        PseudoElement::Placeholder(VendorPrefix::NONE)
    } else {
        PseudoElement::Custom { name }
    }
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

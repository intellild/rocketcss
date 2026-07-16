use crate::prelude::*;

impl ToCss for SelectorList<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_selector_list(self, dest)
    }
}

impl ToCss for Selector<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Parsed(components) => {
                for component in components {
                    component.to_css(dest)?;
                }
                Ok(())
            }
            Self::Unparsed(raw) => dest.write_str(raw),
            Self::Tombstone => Ok(()),
        }
    }
}

fn write_selector_list<PrinterT: PrinterTrait>(
    selectors: &[Selector<'_>],
    dest: &mut PrinterT,
) -> fmt::Result {
    let mut wrote_selector = false;
    for selector in selectors {
        if selector.is_tombstone() {
            continue;
        }
        if wrote_selector {
            dest.delim(Delimiter::Comma)?;
        }
        selector.to_css(dest)?;
        wrote_selector = true;
    }
    Ok(())
}

impl ToCss for SelectorComponent<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Combinator(value) => value.to_css(dest),
            Self::ExplicitAnyNamespace => dest.write_str("*|"),
            Self::ExplicitNoNamespace => dest.write_char('|'),
            Self::DefaultNamespace(_) => Ok(()),
            Self::Namespace { prefix, .. } => {
                serialize_identifier(prefix, dest)?;
                dest.write_char('|')
            }
            Self::ExplicitUniversalType => dest.write_char('*'),
            Self::LocalName { name, .. } => serialize_identifier(name, dest),
            Self::Id(value) => {
                dest.write_char('#')?;
                serialize_identifier(value, dest)
            }
            Self::Class(value) => {
                dest.write_char('.')?;
                serialize_identifier(value, dest)
            }
            Self::AttributeInNoNamespaceExists { local_name, .. } => {
                dest.write_char('[')?;
                serialize_identifier(local_name, dest)?;
                dest.write_char(']')
            }
            Self::AttributeInNoNamespace {
                local_name,
                operator,
                value,
                case_sensitivity,
                ..
            } => write_attribute(
                None,
                local_name,
                Some((*operator, value, *case_sensitivity)),
                dest,
            ),
            Self::AttributeOther(value) => value.to_css(dest),
            Self::Negation(selectors) => {
                dest.write_str(":not(")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::Root => dest.write_str(":root"),
            Self::Empty => dest.write_str(":empty"),
            Self::Scope => dest.write_str(":scope"),
            Self::Nth(value) => value.to_css(dest),
            Self::NthOf { data, selectors } => {
                write_nth_start(data, true, dest)?;
                write_nth_affine(data, dest)?;
                dest.write_str(" of ")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::PseudoClass(value) => value.to_css(dest),
            Self::Slotted(selector) => {
                dest.write_str("::slotted(")?;
                selector.to_css(dest)?;
                dest.write_char(')')
            }
            Self::Part(parts) => {
                dest.write_str("::part(")?;
                for (index, part) in parts.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    serialize_identifier(part, dest)?;
                }
                dest.write_char(')')
            }
            Self::Host(selector) => {
                dest.write_str(":host")?;
                if let Some(selector) = selector {
                    dest.write_char('(')?;
                    selector.to_css(dest)?;
                    dest.write_char(')')?;
                }
                Ok(())
            }
            Self::Where(selectors) => {
                dest.write_str(":where(")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::Is(selectors) => {
                if selectors.len() == 1
                    && !selectors[0]
                        .iter()
                        .any(|component| matches!(component, SelectorComponent::Combinator(_)))
                    && !selectors[0].iter().any(|component| {
                        matches!(
                            component,
                            SelectorComponent::LocalName { .. }
                                | SelectorComponent::ExplicitUniversalType
                        )
                    })
                {
                    return selectors[0].to_css(dest);
                }
                dest.write_str(":is(")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::Any {
                vendor_prefix,
                selectors,
            } => {
                dest.write_char(':')?;
                vendor_prefix.to_css(dest)?;
                dest.write_str("any(")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::Has(selectors) => {
                dest.write_str(":has(")?;
                write_selector_list(selectors, dest)?;
                dest.write_char(')')
            }
            Self::PseudoElement(value) => value.to_css(dest),
            Self::Nesting => dest.write_char('&'),
        }
    }
}

impl ToCss for Combinator {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Child => dest.delim(Delimiter::ChildCombinator),
            Self::Descendant => dest.write_char(' '),
            Self::NextSibling => dest.delim(Delimiter::NextSiblingCombinator),
            Self::LaterSibling => dest.delim(Delimiter::LaterSiblingCombinator),
            Self::PseudoElement | Self::SlotAssignment | Self::Part => Ok(()),
            Self::DeepDescendant => dest.write_str(" >>> "),
            Self::Deep => dest.write_str(" /deep/ "),
        }
    }
}

impl ToCss for AttrSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        let namespace = self.namespace.as_ref();
        match &self.operation {
            AttrOperation::Exists => write_attribute(namespace, self.local_name, None, dest),
            AttrOperation::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => write_attribute(
                namespace,
                self.local_name,
                Some((*operator, expected_value, *case_sensitivity)),
                dest,
            ),
        }
    }
}

fn write_attribute<PrinterT: PrinterTrait>(
    namespace: Option<&NamespaceConstraint<'_>>,
    local_name: &str,
    operation: Option<(AttrSelectorOperator, &str, ParsedCaseSensitivity)>,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_char('[')?;
    if let Some(namespace) = namespace {
        namespace.to_css(dest)?;
    }
    serialize_identifier(local_name, dest)?;
    if let Some((operator, value, case_sensitivity)) = operation {
        operator.to_css(dest)?;
        if !dest.prettify() && !value.is_empty() {
            let mut identifier = String::new();
            serialize_identifier(value, &mut identifier)?;
            let mut string = String::new();
            serialize_string(value, &mut string)?;
            if identifier.len() < string.len() {
                dest.write_str(&identifier)?;
            } else {
                dest.write_str(&string)?;
            }
        } else {
            serialize_string(value, dest)?;
        }
        case_sensitivity.to_css(dest)?;
    }
    dest.write_char(']')
}

impl ToCss for NamespaceConstraint<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Any => dest.write_str("*|"),
            Self::Specific { prefix, .. } => {
                serialize_identifier(prefix, dest)?;
                dest.write_char('|')
            }
        }
    }
}

impl ToCss for AttrOperation<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Exists => Ok(()),
            Self::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => {
                operator.to_css(dest)?;
                serialize_string(expected_value, dest)?;
                case_sensitivity.to_css(dest)
            }
        }
    }
}

impl ToCss for ParsedCaseSensitivity {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::ExplicitCaseSensitive => dest.write_str(" s"),
            Self::AsciiCaseInsensitive => dest.write_str(" i"),
            Self::CaseSensitive | Self::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => Ok(()),
        }
    }
}

impl ToCss for AttrSelectorOperator {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::Equal => "=",
            Self::Includes => "~=",
            Self::DashMatch => "|=",
            Self::Prefix => "^=",
            Self::Substring => "*=",
            Self::Suffix => "$=",
        })
    }
}

impl ToCss for NthSelectorData {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        write_nth_start(self, self.is_function, dest)?;
        if self.is_function {
            write_nth_affine(self, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl ToCss for NthType {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("nth types are static keywords"))
    }
}

fn write_nth_start<PrinterT: PrinterTrait>(
    value: &NthSelectorData,
    is_function: bool,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str(match (value.kind, is_function) {
        (NthType::Child, true) => ":nth-child(",
        (NthType::Child, false) => ":first-child",
        (NthType::LastChild, true) => ":nth-last-child(",
        (NthType::LastChild, false) => ":last-child",
        (NthType::OnlyChild, _) => ":only-child",
        (NthType::OfType, true) => ":nth-of-type(",
        (NthType::OfType, false) => ":first-of-type",
        (NthType::LastOfType, true) => ":nth-last-of-type(",
        (NthType::LastOfType, false) => ":last-of-type",
        (NthType::OnlyOfType, _) => ":only-of-type",
        (NthType::Col, _) => ":nth-col(",
        (NthType::LastCol, _) => ":nth-last-col(",
    })
}

fn write_nth_affine<PrinterT: PrinterTrait>(
    value: &NthSelectorData,
    dest: &mut PrinterT,
) -> fmt::Result {
    match (value.a, value.b) {
        (0, 0) => dest.write_char('0'),
        (1, 0) => dest.write_char('n'),
        (-1, 0) => dest.write_str("-n"),
        (a, 0) => {
            serialize_int(a, dest)?;
            dest.write_char('n')
        }
        (2, 1) => dest.write_str("odd"),
        (0, b) => serialize_int(b, dest),
        (1, b) => {
            dest.write_char('n')?;
            write_nth_offset(b, dest)
        }
        (-1, b) => {
            dest.write_str("-n")?;
            write_nth_offset(b, dest)
        }
        (a, b) => {
            serialize_int(a, dest)?;
            dest.write_char('n')?;
            write_nth_offset(b, dest)
        }
    }
}

fn write_nth_offset<PrinterT: PrinterTrait>(value: i32, dest: &mut PrinterT) -> fmt::Result {
    if value >= 0 {
        dest.write_char('+')?;
    }
    serialize_int(value, dest)
}

impl ToCss for Direction {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("directions are static keywords"))
    }
}

impl ToCss for PseudoClass<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Lang { languages } => {
                dest.write_str(":lang(")?;
                for (index, language) in languages.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(language, dest)?;
                }
                dest.write_char(')')
            }
            Self::Dir { direction } => {
                dest.write_str(":dir(")?;
                direction.to_css(dest)?;
                dest.write_char(')')
            }
            Self::Fullscreen(prefix) => write_prefixed_pseudo(prefix, "fullscreen", dest),
            Self::AnyLink(prefix) => write_prefixed_pseudo(prefix, "any-link", dest),
            Self::ReadOnly(prefix) => write_prefixed_pseudo(prefix, "read-only", dest),
            Self::ReadWrite(prefix) => write_prefixed_pseudo(prefix, "read-write", dest),
            Self::PlaceholderShown(prefix) => {
                write_prefixed_pseudo(prefix, "placeholder-shown", dest)
            }
            Self::Autofill(prefix) => write_prefixed_pseudo(prefix, "autofill", dest),
            Self::ActiveViewTransitionType { kinds } => {
                dest.write_str(":active-view-transition-type(")?;
                for (index, kind) in kinds.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(kind, dest)?;
                }
                dest.write_char(')')
            }
            Self::State { state } => {
                dest.write_str(":state(")?;
                serialize_identifier(state, dest)?;
                dest.write_char(')')
            }
            Self::Local { selector } => selector.to_css(dest),
            Self::Global { selector } => selector.to_css(dest),
            Self::WebKitScrollbar(value) => value.to_css(dest),
            Self::Custom { name } => {
                dest.write_char(':')?;
                dest.write_str(name)
            }
            Self::CustomFunction { name, arguments } => {
                dest.write_char(':')?;
                dest.write_str(name)?;
                dest.write_char('(')?;
                crate::token::write_token_list(arguments, dest)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_class_name(value)),
        }
    }
}

fn write_prefixed_pseudo<PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_char(':')?;
    prefix.to_css(dest)?;
    dest.write_str(name)
}

fn pseudo_class_name(value: &PseudoClass<'_>) -> &'static str {
    match value {
        PseudoClass::Hover => ":hover",
        PseudoClass::Active => ":active",
        PseudoClass::Focus => ":focus",
        PseudoClass::FocusVisible => ":focus-visible",
        PseudoClass::FocusWithin => ":focus-within",
        PseudoClass::Current => ":current",
        PseudoClass::Past => ":past",
        PseudoClass::Future => ":future",
        PseudoClass::Playing => ":playing",
        PseudoClass::Paused => ":paused",
        PseudoClass::Seeking => ":seeking",
        PseudoClass::Buffering => ":buffering",
        PseudoClass::Stalled => ":stalled",
        PseudoClass::Muted => ":muted",
        PseudoClass::VolumeLocked => ":volume-locked",
        PseudoClass::Open => ":open",
        PseudoClass::Closed => ":closed",
        PseudoClass::Modal => ":modal",
        PseudoClass::PictureInPicture => ":picture-in-picture",
        PseudoClass::PopoverOpen => ":popover-open",
        PseudoClass::Defined => ":defined",
        PseudoClass::Link => ":link",
        PseudoClass::LocalLink => ":local-link",
        PseudoClass::Target => ":target",
        PseudoClass::TargetCurrent => ":target-current",
        PseudoClass::TargetBefore => ":target-before",
        PseudoClass::TargetAfter => ":target-after",
        PseudoClass::TargetWithin => ":target-within",
        PseudoClass::Visited => ":visited",
        PseudoClass::Enabled => ":enabled",
        PseudoClass::Disabled => ":disabled",
        PseudoClass::Default => ":default",
        PseudoClass::Checked => ":checked",
        PseudoClass::Indeterminate => ":indeterminate",
        PseudoClass::Blank => ":blank",
        PseudoClass::Valid => ":valid",
        PseudoClass::Invalid => ":invalid",
        PseudoClass::InRange => ":in-range",
        PseudoClass::OutOfRange => ":out-of-range",
        PseudoClass::Required => ":required",
        PseudoClass::Optional => ":optional",
        PseudoClass::UserValid => ":user-valid",
        PseudoClass::UserInvalid => ":user-invalid",
        PseudoClass::ActiveViewTransition => ":active-view-transition",
        _ => unreachable!(),
    }
}

impl ToCss for WebKitScrollbarPseudoClass {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::Horizontal => ":horizontal",
            Self::Vertical => ":vertical",
            Self::Decrement => ":decrement",
            Self::Increment => ":increment",
            Self::Start => ":start",
            Self::End => ":end",
            Self::DoubleButton => ":double-button",
            Self::SingleButton => ":single-button",
            Self::NoButton => ":no-button",
            Self::CornerPresent => ":corner-present",
            Self::WindowInactive => ":window-inactive",
        })
    }
}

impl ToCss for PseudoElement<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::Selection(prefix) => write_prefixed_element(prefix, "selection", dest),
            Self::Placeholder(prefix) => write_prefixed_element(prefix, "placeholder", dest),
            Self::Backdrop(prefix) => write_prefixed_element(prefix, "backdrop", dest),
            Self::FileSelectorButton(prefix) => {
                write_prefixed_element(prefix, "file-selector-button", dest)
            }
            Self::HighlightFunction { name } => write_element_function("highlight", name, dest),
            Self::WebKitScrollbar(value) => value.to_css(dest),
            Self::CueFunction { selector } => write_selector_function("cue", selector, dest),
            Self::CueRegionFunction { selector } => {
                write_selector_function("cue-region", selector, dest)
            }
            Self::ViewTransitionGroup { part } => {
                write_part_function("view-transition-group", part, dest)
            }
            Self::ViewTransitionImagePair { part } => {
                write_part_function("view-transition-image-pair", part, dest)
            }
            Self::ViewTransitionOld { part } => {
                write_part_function("view-transition-old", part, dest)
            }
            Self::ViewTransitionNew { part } => {
                write_part_function("view-transition-new", part, dest)
            }
            Self::PickerFunction { identifier } => {
                write_element_function("picker", identifier, dest)
            }
            Self::Custom { name } => {
                dest.write_str("::")?;
                dest.write_str(name)
            }
            Self::CustomFunction { name, arguments } => {
                dest.write_str("::")?;
                dest.write_str(name)?;
                dest.write_char('(')?;
                crate::token::write_token_list(arguments, dest)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_element_name(value)),
        }
    }
}

fn write_prefixed_element<PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str("::")?;
    prefix.to_css(dest)?;
    dest.write_str(name)
}

fn write_element_function<PrinterT: PrinterTrait>(
    name: &str,
    value: &str,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    serialize_identifier(value, dest)?;
    dest.write_char(')')
}

fn write_selector_function<PrinterT: PrinterTrait>(
    name: &str,
    selector: &Selector<'_>,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    selector.to_css(dest)?;
    dest.write_char(')')
}

fn write_part_function<PrinterT: PrinterTrait>(
    name: &str,
    part: &ViewTransitionPartSelector<'_>,
    dest: &mut PrinterT,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    part.to_css(dest)?;
    dest.write_char(')')
}

fn pseudo_element_name(value: &PseudoElement<'_>) -> &'static str {
    match value {
        PseudoElement::After => ":after",
        PseudoElement::Before => ":before",
        PseudoElement::FirstLine => ":first-line",
        PseudoElement::FirstLetter => ":first-letter",
        PseudoElement::DetailsContent => "::details-content",
        PseudoElement::TargetText => "::target-text",
        PseudoElement::SearchText => "::search-text",
        PseudoElement::Marker => "::marker",
        PseudoElement::Cue => "::cue",
        PseudoElement::CueRegion => "::cue-region",
        PseudoElement::ViewTransition => "::view-transition",
        PseudoElement::PickerIcon => "::picker-icon",
        PseudoElement::Checkmark => "::checkmark",
        PseudoElement::GrammarError => "::grammar-error",
        PseudoElement::SpellingError => "::spelling-error",
        _ => unreachable!(),
    }
}

impl ToCss for WebKitScrollbarPseudoElement {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        dest.write_str(match self {
            Self::Scrollbar => "::-webkit-scrollbar",
            Self::Button => "::-webkit-scrollbar-button",
            Self::Track => "::-webkit-scrollbar-track",
            Self::TrackPiece => "::-webkit-scrollbar-track-piece",
            Self::Thumb => "::-webkit-scrollbar-thumb",
            Self::Corner => "::-webkit-scrollbar-corner",
            Self::Resizer => "::-webkit-resizer",
        })
    }
}

impl ToCss for ViewTransitionPartName<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        match self {
            Self::All => dest.write_char('*'),
            Self::Name(value) => serialize_identifier(value, dest),
        }
    }
}

impl ToCss for ViewTransitionPartSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(&self, dest: &mut PrinterT) -> fmt::Result {
        if let Some(name) = &self.name {
            name.to_css(dest)?;
        }
        for class in &self.classes {
            dest.write_char('.')?;
            serialize_identifier(class, dest)?;
        }
        Ok(())
    }
}

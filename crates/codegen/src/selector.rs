use crate::prelude::*;

impl<'ast, 'ghost> ToCss<'ghost> for &'ast [Selector<'ast>] {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        write_selector_list(self, dest, _cx)
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for Selector<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SelectorData::Parsed(components) => {
                for component in components {
                    component.to_css(dest, _cx)?;
                }
                Ok(())
            }
            SelectorData::Unparsed(raw) => dest.write_str(&raw),
            SelectorData::Tombstone => Ok(()),
        }
    }
}

fn selector_components<'ast>(
    ast: &Ast<'ast>,
    selector: Selector<'ast>,
) -> &'ast [SelectorComponent<'ast>] {
    match selector.data(ast) {
        SelectorData::Parsed(components) => components,
        _ => &[],
    }
}

fn write_selector_list<'ghost, PrinterT: PrinterTrait>(
    selectors: &[Selector<'_>],
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    let mut wrote_selector = false;
    for selector in selectors {
        if matches!(selector.data(cx.ast()), SelectorData::Tombstone) {
            continue;
        }
        if wrote_selector {
            dest.delim(Delimiter::Comma)?;
        }
        selector.to_css(dest, cx)?;
        wrote_selector = true;
    }
    Ok(())
}

impl<'ast, 'ghost> ToCss<'ghost> for SelectorComponent<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            SelectorComponentData::Combinator(value) => value.to_css(dest, _cx),
            SelectorComponentData::ExplicitAnyNamespace => dest.write_str("*|"),
            SelectorComponentData::ExplicitNoNamespace => dest.write_char('|'),
            SelectorComponentData::DefaultNamespace(_) => Ok(()),
            SelectorComponentData::Namespace { prefix, .. } => {
                serialize_identifier(&prefix, dest)?;
                dest.write_char('|')
            }
            SelectorComponentData::ExplicitUniversalType => dest.write_char('*'),
            SelectorComponentData::LocalName { name, .. } => serialize_identifier(&name, dest),
            SelectorComponentData::Id(value) => {
                dest.write_char('#')?;
                serialize_identifier(&value, dest)
            }
            SelectorComponentData::Class(value) => {
                dest.write_char('.')?;
                serialize_identifier(&value, dest)
            }
            SelectorComponentData::AttributeInNoNamespaceExists { local_name, .. } => {
                dest.write_char('[')?;
                serialize_identifier(&local_name, dest)?;
                dest.write_char(']')
            }
            SelectorComponentData::AttributeInNoNamespace {
                local_name,
                operator,
                value,
                case_sensitivity,
                ..
            } => write_attribute(
                None,
                &local_name,
                Some((operator, value.as_ref(), case_sensitivity)),
                dest,
                _cx,
            ),
            SelectorComponentData::AttributeOther(value) => value.to_css(dest, _cx),
            SelectorComponentData::Negation(selectors) => {
                dest.write_str(":not(")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::Root => dest.write_str(":root"),
            SelectorComponentData::Empty => dest.write_str(":empty"),
            SelectorComponentData::Scope => dest.write_str(":scope"),
            SelectorComponentData::Nth(value) => value.to_css(dest, _cx),
            SelectorComponentData::NthOf { data, selectors } => {
                let data = _cx.ast().data(data);
                write_nth_start(&data, true, dest)?;
                write_nth_affine(&data, dest)?;
                dest.write_str(" of ")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::PseudoClass(value) => value.to_css(dest, _cx),
            SelectorComponentData::Slotted(selector) => {
                dest.write_str("::slotted(")?;
                selector.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::Part(parts) => {
                dest.write_str("::part(")?;
                for (index, part) in parts.iter().enumerate() {
                    if index > 0 {
                        dest.write_char(' ')?;
                    }
                    serialize_identifier(part, dest)?;
                }
                dest.write_char(')')
            }
            SelectorComponentData::Host(selector) => {
                dest.write_str(":host")?;
                if let Some(selector) = selector {
                    dest.write_char('(')?;
                    selector.to_css(dest, _cx)?;
                    dest.write_char(')')?;
                }
                Ok(())
            }
            SelectorComponentData::Where(selectors) => {
                dest.write_str(":where(")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::Is(selectors) => {
                if selectors.len() == 1
                    && !selector_components(_cx.ast(), selectors[0])
                        .iter()
                        .any(|component| {
                            matches!(
                                component.data(_cx.ast()),
                                SelectorComponentData::Combinator(_)
                            )
                        })
                    && !selector_components(_cx.ast(), selectors[0])
                        .iter()
                        .any(|component| {
                            matches!(
                                component.data(_cx.ast()),
                                SelectorComponentData::LocalName { .. }
                                    | SelectorComponentData::ExplicitUniversalType
                            )
                        })
                {
                    return selectors[0].to_css(dest, _cx);
                }
                dest.write_str(":is(")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::Any {
                vendor_prefix,
                selectors,
            } => {
                dest.write_char(':')?;
                vendor_prefix.to_css(dest, _cx)?;
                dest.write_str("any(")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::Has(selectors) => {
                dest.write_str(":has(")?;
                write_selector_list(selectors, dest, _cx)?;
                dest.write_char(')')
            }
            SelectorComponentData::PseudoElement(value) => value.to_css(dest, _cx),
            SelectorComponentData::Nesting => dest.write_char('&'),
        }
    }
}

impl<'ghost> ToCss<'ghost> for Combinator {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ast, 'ghost> ToCss<'ghost> for AttrSelector<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        let namespace = data.namespace.as_ref();
        match &data.operation {
            AttrOperation::Exists => write_attribute(namespace, &data.local_name, None, dest, _cx),
            AttrOperation::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => write_attribute(
                namespace,
                &data.local_name,
                Some((*operator, expected_value, *case_sensitivity)),
                dest,
                _cx,
            ),
        }
    }
}

fn write_attribute<'ghost, PrinterT: PrinterTrait>(
    namespace: Option<&NamespaceConstraint<'_>>,
    local_name: &str,
    operation: Option<(AttrSelectorOperator, &str, ParsedCaseSensitivity)>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_char('[')?;
    if let Some(namespace) = namespace {
        namespace.to_css(dest, cx)?;
    }
    serialize_identifier(local_name, dest)?;
    if let Some((operator, value, case_sensitivity)) = operation {
        operator.to_css(dest, cx)?;
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
        case_sensitivity.to_css(dest, cx)?;
    }
    dest.write_char(']')
}

impl<'ghost> ToCss<'ghost> for NamespaceConstraint<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Any => dest.write_str("*|"),
            Self::Specific { prefix, .. } => {
                serialize_identifier(prefix, dest)?;
                dest.write_char('|')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for AttrOperation<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Exists => Ok(()),
            Self::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => {
                operator.to_css(dest, _cx)?;
                serialize_string(expected_value, dest)?;
                case_sensitivity.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for ParsedCaseSensitivity {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::ExplicitCaseSensitive => dest.write_str(" s"),
            Self::AsciiCaseInsensitive => dest.write_str(" i"),
            Self::CaseSensitive | Self::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => Ok(()),
        }
    }
}

impl<'ghost> ToCss<'ghost> for AttrSelectorOperator {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ast, 'ghost> ToCss<'ghost> for NthSelectorData<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        write_nth_start(&data, data.is_function, dest)?;
        if data.is_function {
            write_nth_affine(&data, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for NthType {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("nth types are static keywords"))
    }
}

fn write_nth_start<PrinterT: PrinterTrait>(
    value: &NthSelectorDataData,
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
    value: &NthSelectorDataData,
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

impl<'ghost> ToCss<'ghost> for Direction {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("directions are static keywords"))
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for PseudoClass<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            PseudoClassData::Lang { languages } => {
                dest.write_str(":lang(")?;
                for (index, language) in languages.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(language, dest)?;
                }
                dest.write_char(')')
            }
            PseudoClassData::Dir { direction } => {
                dest.write_str(":dir(")?;
                direction.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            PseudoClassData::Fullscreen(prefix) => {
                write_prefixed_pseudo(&prefix, "fullscreen", dest, _cx)
            }
            PseudoClassData::AnyLink(prefix) => {
                write_prefixed_pseudo(&prefix, "any-link", dest, _cx)
            }
            PseudoClassData::ReadOnly(prefix) => {
                write_prefixed_pseudo(&prefix, "read-only", dest, _cx)
            }
            PseudoClassData::ReadWrite(prefix) => {
                write_prefixed_pseudo(&prefix, "read-write", dest, _cx)
            }
            PseudoClassData::PlaceholderShown(prefix) => {
                write_prefixed_pseudo(&prefix, "placeholder-shown", dest, _cx)
            }
            PseudoClassData::Autofill(prefix) => {
                write_prefixed_pseudo(&prefix, "autofill", dest, _cx)
            }
            PseudoClassData::ActiveViewTransitionType { kinds } => {
                dest.write_str(":active-view-transition-type(")?;
                for (index, kind) in kinds.iter().enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(kind, dest)?;
                }
                dest.write_char(')')
            }
            PseudoClassData::State { state } => {
                dest.write_str(":state(")?;
                serialize_identifier(&state, dest)?;
                dest.write_char(')')
            }
            PseudoClassData::Local { selector } => selector.to_css(dest, _cx),
            PseudoClassData::Global { selector } => selector.to_css(dest, _cx),
            PseudoClassData::WebKitScrollbar(value) => value.to_css(dest, _cx),
            PseudoClassData::Custom { name } => {
                dest.write_char(':')?;
                dest.write_str(&name)
            }
            PseudoClassData::CustomFunction { name, arguments } => {
                dest.write_char(':')?;
                dest.write_str(&name)?;
                dest.write_char('(')?;
                crate::token::write_token_list(arguments, dest, _cx)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_class_name(&value)),
        }
    }
}

fn write_prefixed_pseudo<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_char(':')?;
    prefix.to_css(dest, cx)?;
    dest.write_str(name)
}

fn pseudo_class_name(value: &PseudoClassData<'_>) -> &'static str {
    match value {
        PseudoClassData::Hover => ":hover",
        PseudoClassData::Active => ":active",
        PseudoClassData::Focus => ":focus",
        PseudoClassData::FocusVisible => ":focus-visible",
        PseudoClassData::FocusWithin => ":focus-within",
        PseudoClassData::Current => ":current",
        PseudoClassData::Past => ":past",
        PseudoClassData::Future => ":future",
        PseudoClassData::Playing => ":playing",
        PseudoClassData::Paused => ":paused",
        PseudoClassData::Seeking => ":seeking",
        PseudoClassData::Buffering => ":buffering",
        PseudoClassData::Stalled => ":stalled",
        PseudoClassData::Muted => ":muted",
        PseudoClassData::VolumeLocked => ":volume-locked",
        PseudoClassData::Open => ":open",
        PseudoClassData::Closed => ":closed",
        PseudoClassData::Modal => ":modal",
        PseudoClassData::PictureInPicture => ":picture-in-picture",
        PseudoClassData::PopoverOpen => ":popover-open",
        PseudoClassData::Defined => ":defined",
        PseudoClassData::Link => ":link",
        PseudoClassData::LocalLink => ":local-link",
        PseudoClassData::Target => ":target",
        PseudoClassData::TargetCurrent => ":target-current",
        PseudoClassData::TargetBefore => ":target-before",
        PseudoClassData::TargetAfter => ":target-after",
        PseudoClassData::TargetWithin => ":target-within",
        PseudoClassData::Visited => ":visited",
        PseudoClassData::Enabled => ":enabled",
        PseudoClassData::Disabled => ":disabled",
        PseudoClassData::Default => ":default",
        PseudoClassData::Checked => ":checked",
        PseudoClassData::Indeterminate => ":indeterminate",
        PseudoClassData::Blank => ":blank",
        PseudoClassData::Valid => ":valid",
        PseudoClassData::Invalid => ":invalid",
        PseudoClassData::InRange => ":in-range",
        PseudoClassData::OutOfRange => ":out-of-range",
        PseudoClassData::Required => ":required",
        PseudoClassData::Optional => ":optional",
        PseudoClassData::UserValid => ":user-valid",
        PseudoClassData::UserInvalid => ":user-invalid",
        PseudoClassData::ActiveViewTransition => ":active-view-transition",
        _ => unreachable!(),
    }
}

impl<'ghost> ToCss<'ghost> for WebKitScrollbarPseudoClass {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ast, 'ghost> ToCss<'ghost> for PseudoElement<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self.data(_cx.ast()) {
            PseudoElementData::Selection(prefix) => {
                write_prefixed_element(&prefix, "selection", dest, _cx)
            }
            PseudoElementData::Placeholder(prefix) => {
                write_prefixed_element(&prefix, "placeholder", dest, _cx)
            }
            PseudoElementData::Backdrop(prefix) => {
                write_prefixed_element(&prefix, "backdrop", dest, _cx)
            }
            PseudoElementData::FileSelectorButton(prefix) => {
                write_prefixed_element(&prefix, "file-selector-button", dest, _cx)
            }
            PseudoElementData::HighlightFunction { name } => {
                write_element_function("highlight", &name, dest)
            }
            PseudoElementData::WebKitScrollbar(value) => value.to_css(dest, _cx),
            PseudoElementData::CueFunction { selector } => {
                write_selector_function("cue", &selector, dest, _cx)
            }
            PseudoElementData::CueRegionFunction { selector } => {
                write_selector_function("cue-region", &selector, dest, _cx)
            }
            PseudoElementData::ViewTransitionGroup { part } => {
                write_part_function("view-transition-group", &part, dest, _cx)
            }
            PseudoElementData::ViewTransitionImagePair { part } => {
                write_part_function("view-transition-image-pair", &part, dest, _cx)
            }
            PseudoElementData::ViewTransitionOld { part } => {
                write_part_function("view-transition-old", &part, dest, _cx)
            }
            PseudoElementData::ViewTransitionNew { part } => {
                write_part_function("view-transition-new", &part, dest, _cx)
            }
            PseudoElementData::PickerFunction { identifier } => {
                write_element_function("picker", &identifier, dest)
            }
            PseudoElementData::Custom { name } => {
                dest.write_str("::")?;
                dest.write_str(&name)
            }
            PseudoElementData::CustomFunction { name, arguments } => {
                dest.write_str("::")?;
                dest.write_str(&name)?;
                dest.write_char('(')?;
                crate::token::write_token_list(arguments, dest, _cx)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_element_name(&value)),
        }
    }
}

fn write_prefixed_element<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("::")?;
    prefix.to_css(dest, cx)?;
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

fn write_selector_function<'ghost, PrinterT: PrinterTrait>(
    name: &str,
    selector: &Selector<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    selector.to_css(dest, cx)?;
    dest.write_char(')')
}

fn write_part_function<'ghost, PrinterT: PrinterTrait>(
    name: &str,
    part: &ViewTransitionPartSelector<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    part.to_css(dest, cx)?;
    dest.write_char(')')
}

fn pseudo_element_name(value: &PseudoElementData<'_>) -> &'static str {
    match value {
        PseudoElementData::After => ":after",
        PseudoElementData::Before => ":before",
        PseudoElementData::FirstLine => ":first-line",
        PseudoElementData::FirstLetter => ":first-letter",
        PseudoElementData::DetailsContent => "::details-content",
        PseudoElementData::TargetText => "::target-text",
        PseudoElementData::SearchText => "::search-text",
        PseudoElementData::Marker => "::marker",
        PseudoElementData::Cue => "::cue",
        PseudoElementData::CueRegion => "::cue-region",
        PseudoElementData::ViewTransition => "::view-transition",
        PseudoElementData::PickerIcon => "::picker-icon",
        PseudoElementData::Checkmark => "::checkmark",
        PseudoElementData::GrammarError => "::grammar-error",
        PseudoElementData::SpellingError => "::spelling-error",
        _ => unreachable!(),
    }
}

impl<'ghost> ToCss<'ghost> for WebKitScrollbarPseudoElement {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ghost> ToCss<'ghost> for ViewTransitionPartName<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::All => dest.write_char('*'),
            Self::Name(value) => serialize_identifier(value, dest),
        }
    }
}

impl<'ast, 'ghost> ToCss<'ghost> for ViewTransitionPartSelector<'ast> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, '_, 'ghost>,
    ) -> fmt::Result {
        let data = _cx.ast().data(*self);
        if let Some(name) = &data.name {
            name.to_css(dest, _cx)?;
        }
        for class in data.classes {
            dest.write_char('.')?;
            serialize_identifier(class, dest)?;
        }
        Ok(())
    }
}

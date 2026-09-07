use crate::prelude::*;

impl<'ghost> ToCss<'ghost> for SelectorList<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_selector_list(_cx.ast_context().vec_iter(*self), dest, _cx)
    }
}

impl<'ghost> ToCss<'ghost> for Selector<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Parsed(components) => {
                for component in _cx.ast_context().vec_iter(*components) {
                    component.to_css(dest, _cx)?;
                }
                Ok(())
            }
            Self::Unparsed(raw) => dest.write_str(_cx.ast_context().str(*raw)),
            Self::Tombstone => Ok(()),
        }
    }
}

fn write_selector_list<'ast, 'ghost, PrinterT, I>(
    selectors: I,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result
where
    PrinterT: PrinterTrait,
    I: IntoIterator<Item = NodeId<'ast, Selector<'ast>>>,
{
    let mut wrote_selector = false;
    for selector in selectors {
        let selector = cx.ast_context().resolve_node(selector);
        if selector.is_tombstone() {
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

impl<'ghost> ToCss<'ghost> for SelectorComponent<'_> {
    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: AstNodeStorage<'id>,
    {
        write_selector_component(cx.ast_context().selector_component_syntax(id), dest, cx)
    }
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_selector_component(self.into(), dest, cx)
    }
}
fn write_selector_component<'ghost, PrinterT: PrinterTrait>(
    value: SelectorComponentSyntax<'_>,
    dest: &mut PrinterT,
    _cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    match &value {
        SelectorComponentSyntax::Combinator(value) => value.to_css(dest, _cx),
        SelectorComponentSyntax::ExplicitAnyNamespace => dest.write_str("*|"),
        SelectorComponentSyntax::ExplicitNoNamespace => dest.write_char('|'),
        SelectorComponentSyntax::DefaultNamespace => Ok(()),
        SelectorComponentSyntax::NamespacePrefix(prefix) => {
            serialize_identifier(_cx.ast_context().str(*prefix), dest)?;
            dest.write_char('|')
        }
        SelectorComponentSyntax::ExplicitUniversalType => dest.write_char('*'),
        SelectorComponentSyntax::LocalName(name) => {
            serialize_identifier(_cx.ast_context().str(*name), dest)
        }
        SelectorComponentSyntax::IdName(value) => {
            dest.write_char('#')?;
            serialize_identifier(_cx.ast_context().str(*value), dest)
        }
        SelectorComponentSyntax::ClassName(value) => {
            dest.write_char('.')?;
            serialize_identifier(_cx.ast_context().str(*value), dest)
        }
        SelectorComponentSyntax::AttributeExists(local_name) => {
            dest.write_char('[')?;
            serialize_identifier(_cx.ast_context().str(*local_name), dest)?;
            dest.write_char(']')
        }
        SelectorComponentSyntax::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            ..
        } => write_attribute(
            None,
            _cx.ast_context().str(*local_name),
            Some((*operator, _cx.ast_context().str(*value), *case_sensitivity)),
            dest,
            _cx,
        ),
        SelectorComponentSyntax::AttributeOther(value) => value.to_css(dest, _cx),
        SelectorComponentSyntax::Negation(selectors) => {
            dest.write_str(":not(")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::Root => dest.write_str(":root"),
        SelectorComponentSyntax::Empty => dest.write_str(":empty"),
        SelectorComponentSyntax::Scope => dest.write_str(":scope"),
        SelectorComponentSyntax::Nth(value) => value.to_css(dest, _cx),
        SelectorComponentSyntax::NthOf { data, selectors } => {
            write_nth_start(data, true, dest)?;
            write_nth_affine(data, dest)?;
            dest.write_str(" of ")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::PseudoClass(value) => value.to_css(dest, _cx),
        SelectorComponentSyntax::Slotted(selector) => {
            dest.write_str("::slotted(")?;
            selector.to_css(dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::Part(parts) => {
            dest.write_str("::part(")?;
            for (index, part) in _cx.ast_context().vec_iter(*parts).enumerate() {
                if index > 0 {
                    dest.write_char(' ')?;
                }
                serialize_identifier(_cx.ast_context().str(part), dest)?;
            }
            dest.write_char(')')
        }
        SelectorComponentSyntax::Host(selector) => {
            dest.write_str(":host")?;
            if let Some(selector) = selector {
                dest.write_char('(')?;
                selector.to_css(dest, _cx)?;
                dest.write_char(')')?;
            }
            Ok(())
        }
        SelectorComponentSyntax::Where(selectors) => {
            dest.write_str(":where(")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::Is(selectors) => {
            let selector_count = _cx.ast_context().vec_len(*selectors);
            let selector = (selector_count == 1).then(|| {
                _cx.ast_context().resolve_node(
                    _cx.ast_context()
                        .vec_get(*selectors, 0)
                        .expect("single selector range has one value"),
                )
            });
            if selector_count == 1
                && !selector.as_ref().is_some_and(|selector| {
                    selector.as_parsed().is_some_and(|components| {
                        _cx.ast_context().vec_iter(components).any(|component| {
                            _cx.ast_context()
                                .selector_component_is_combinator_or_type(component)
                        })
                    })
                })
            {
                return selector
                    .expect("one selector was resolved")
                    .to_css(dest, _cx);
            }
            dest.write_str(":is(")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::Any {
            vendor_prefix,
            selectors,
        } => {
            dest.write_char(':')?;
            vendor_prefix.to_css(dest, _cx)?;
            dest.write_str("any(")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::Has(selectors) => {
            dest.write_str(":has(")?;
            write_selector_list(_cx.ast_context().vec_iter(*selectors), dest, _cx)?;
            dest.write_char(')')
        }
        SelectorComponentSyntax::PseudoElement(value) => value.to_css(dest, _cx),
        SelectorComponentSyntax::Nesting => dest.write_char('&'),
    }
}

impl<'ghost> ToCss<'ghost> for Combinator {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for AttrSelector<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_attribute_syntax(self.local_name, self.namespace, self.operation, dest, cx)
    }

    fn to_css_node<'id, PrinterT: PrinterTrait>(
        id: NodeId<'id, Self>,
        dest: &mut PrinterT,
        cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result
    where
        Self: Sized + AstNodeStorage<'id>,
    {
        let (name, namespace, operation) = cx.ast_context().attr_selector_syntax(id);
        write_attribute_syntax(name, namespace, operation, dest, cx)
    }
}

fn write_attribute_syntax<'ghost, PrinterT: PrinterTrait>(
    name: Atom<'_>,
    namespace: Option<NamespaceConstraint<'_>>,
    operation: AttrOperation<'_>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    let operation = match operation {
        AttrOperation::Exists => None,
        AttrOperation::WithValue {
            operator,
            case_sensitivity,
            expected_value,
        } => Some((
            operator,
            cx.ast_context().str(expected_value),
            case_sensitivity,
        )),
    };
    write_attribute(
        namespace.as_ref(),
        cx.ast_context().str(name),
        operation,
        dest,
        cx,
    )
}

fn write_attribute<'ghost, PrinterT: PrinterTrait>(
    namespace: Option<&NamespaceConstraint<'_>>,
    local_name: &str,
    operation: Option<(AttrSelectorOperator, &str, ParsedCaseSensitivity)>,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Any => dest.write_str("*|"),
            Self::Specific { prefix, .. } => {
                serialize_identifier(_cx.ast_context().str(*prefix), dest)?;
                dest.write_char('|')
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for AttrOperation<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Exists => Ok(()),
            Self::WithValue {
                operator,
                case_sensitivity,
                expected_value,
            } => {
                operator.to_css(dest, _cx)?;
                serialize_string(_cx.ast_context().str(*expected_value), dest)?;
                case_sensitivity.to_css(dest, _cx)
            }
        }
    }
}

impl<'ghost> ToCss<'ghost> for ParsedCaseSensitivity {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for NthSelectorData {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        write_nth_start(self, self.is_function, dest)?;
        if self.is_function {
            write_nth_affine(self, dest)?;
            dest.write_char(')')?;
        }
        Ok(())
    }
}

impl<'ghost> ToCss<'ghost> for NthType {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
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

impl<'ghost> ToCss<'ghost> for Direction {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        dest.write_str(self.as_css_str().expect("directions are static keywords"))
    }
}

impl<'ghost> ToCss<'ghost> for PseudoClass<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Lang { languages } => {
                dest.write_str(":lang(")?;
                for (index, language) in _cx.ast_context().vec_iter(*languages).enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(_cx.ast_context().str(language), dest)?;
                }
                dest.write_char(')')
            }
            Self::Dir { direction } => {
                dest.write_str(":dir(")?;
                direction.to_css(dest, _cx)?;
                dest.write_char(')')
            }
            Self::Fullscreen(prefix) => write_prefixed_pseudo(prefix, "fullscreen", dest, _cx),
            Self::AnyLink(prefix) => write_prefixed_pseudo(prefix, "any-link", dest, _cx),
            Self::ReadOnly(prefix) => write_prefixed_pseudo(prefix, "read-only", dest, _cx),
            Self::ReadWrite(prefix) => write_prefixed_pseudo(prefix, "read-write", dest, _cx),
            Self::PlaceholderShown(prefix) => {
                write_prefixed_pseudo(prefix, "placeholder-shown", dest, _cx)
            }
            Self::Autofill(prefix) => write_prefixed_pseudo(prefix, "autofill", dest, _cx),
            Self::ActiveViewTransitionType { kinds } => {
                dest.write_str(":active-view-transition-type(")?;
                for (index, kind) in _cx.ast_context().vec_iter(*kinds).enumerate() {
                    if index > 0 {
                        dest.delim(Delimiter::Comma)?;
                    }
                    serialize_identifier(_cx.ast_context().str(kind), dest)?;
                }
                dest.write_char(')')
            }
            Self::State { state } => {
                dest.write_str(":state(")?;
                serialize_identifier(_cx.ast_context().str(*state), dest)?;
                dest.write_char(')')
            }
            Self::Local { selector } => selector.to_css(dest, _cx),
            Self::Global { selector } => selector.to_css(dest, _cx),
            Self::WebKitScrollbar(value) => value.to_css(dest, _cx),
            Self::Custom { name } => {
                dest.write_char(':')?;
                dest.write_str(_cx.ast_context().str(*name))
            }
            Self::CustomFunction { function } => {
                let CustomPseudoFunction { name, arguments } =
                    _cx.ast_context().resolve_node(*function);
                dest.write_char(':')?;
                dest.write_str(_cx.ast_context().str(name))?;
                dest.write_char('(')?;
                crate::token::write_token_list(_cx.ast_context().vec_iter(arguments), dest, _cx)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_class_name(value)),
        }
    }
}

fn write_prefixed_pseudo<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_char(':')?;
    prefix.to_css(dest, cx)?;
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

impl<'ghost> ToCss<'ghost> for WebKitScrollbarPseudoClass {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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

impl<'ghost> ToCss<'ghost> for PseudoElement<'_> {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::Selection(prefix) => write_prefixed_element(prefix, "selection", dest, _cx),
            Self::Placeholder(prefix) => write_prefixed_element(prefix, "placeholder", dest, _cx),
            Self::Backdrop(prefix) => write_prefixed_element(prefix, "backdrop", dest, _cx),
            Self::FileSelectorButton(prefix) => {
                write_prefixed_element(prefix, "file-selector-button", dest, _cx)
            }
            Self::HighlightFunction { name } => {
                write_element_function("highlight", _cx.ast_context().str(*name), dest)
            }
            Self::WebKitScrollbar(value) => value.to_css(dest, _cx),
            Self::CueFunction { selector } => write_selector_function(
                "cue",
                &_cx.ast_context().resolve_node(*selector),
                dest,
                _cx,
            ),
            Self::CueRegionFunction { selector } => write_selector_function(
                "cue-region",
                &_cx.ast_context().resolve_node(*selector),
                dest,
                _cx,
            ),
            Self::ViewTransitionGroup { part } => write_part_function(
                "view-transition-group",
                &_cx.ast_context().resolve_node(*part),
                dest,
                _cx,
            ),
            Self::ViewTransitionImagePair { part } => write_part_function(
                "view-transition-image-pair",
                &_cx.ast_context().resolve_node(*part),
                dest,
                _cx,
            ),
            Self::ViewTransitionOld { part } => write_part_function(
                "view-transition-old",
                &_cx.ast_context().resolve_node(*part),
                dest,
                _cx,
            ),
            Self::ViewTransitionNew { part } => write_part_function(
                "view-transition-new",
                &_cx.ast_context().resolve_node(*part),
                dest,
                _cx,
            ),
            Self::PickerFunction { identifier } => {
                write_element_function("picker", _cx.ast_context().str(*identifier), dest)
            }
            Self::Custom { name } => {
                dest.write_str("::")?;
                dest.write_str(_cx.ast_context().str(*name))
            }
            Self::CustomFunction { function } => {
                let CustomPseudoFunction { name, arguments } =
                    _cx.ast_context().resolve_node(*function);
                dest.write_str("::")?;
                dest.write_str(_cx.ast_context().str(name))?;
                dest.write_char('(')?;
                crate::token::write_token_list(_cx.ast_context().vec_iter(arguments), dest, _cx)?;
                dest.write_char(')')
            }
            value => dest.write_str(pseudo_element_name(value)),
        }
    }
}

fn write_prefixed_element<'ghost, PrinterT: PrinterTrait>(
    prefix: &VendorPrefix,
    name: &str,
    dest: &mut PrinterT,
    cx: &ToCssContext<'_, '_, 'ghost>,
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
    cx: &ToCssContext<'_, '_, 'ghost>,
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
    cx: &ToCssContext<'_, '_, 'ghost>,
) -> fmt::Result {
    dest.write_str("::")?;
    dest.write_str(name)?;
    dest.write_char('(')?;
    part.to_css(dest, cx)?;
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

impl<'ghost> ToCss<'ghost> for WebKitScrollbarPseudoElement {
    fn to_css<PrinterT: PrinterTrait>(
        &self,
        dest: &mut PrinterT,
        _cx: &ToCssContext<'_, '_, 'ghost>,
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
        _cx: &ToCssContext<'_, '_, 'ghost>,
    ) -> fmt::Result {
        match self {
            Self::All => dest.write_char('*'),
            Self::Name(value) => serialize_identifier(_cx.ast_context().str(*value), dest),
        }
    }
}

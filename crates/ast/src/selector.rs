use super::*;

/// A selector list in source order.
pub type SelectorList<'a> = Vec<'a, NodeId<'a, Selector<'a>>>;

/// A complex selector, a losslessly preserved invalid selector, or a removed selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub enum Selector<'a> {
    /// A valid selector. Components are stored in parse order.
    Parsed(Vec<'a, NodeId<'a, SelectorComponent<'a>>>),
    /// An invalid selector preserved by parser error recovery.
    #[visit(skip)]
    Unparsed(Atom<'a>),
    /// A selector removed by a transformation.
    Tombstone,
}

impl<'a> Selector<'a> {
    #[inline]
    pub fn parsed(components: Vec<'a, NodeId<'a, SelectorComponent<'a>>>) -> Self {
        Self::Parsed(components)
    }

    #[inline]
    pub fn as_parsed(&self) -> Option<Vec<'a, NodeId<'a, SelectorComponent<'a>>>> {
        match self {
            Self::Parsed(components) => Some(*components),
            Self::Unparsed(_) | Self::Tombstone => None,
        }
    }

    #[inline]
    pub fn as_parsed_mut(&mut self) -> Option<&mut Vec<'a, NodeId<'a, SelectorComponent<'a>>>> {
        match self {
            Self::Parsed(components) => Some(components),
            Self::Unparsed(_) | Self::Tombstone => None,
        }
    }

    #[inline]
    pub fn is_tombstone(&self) -> bool {
        matches!(self, Self::Tombstone)
    }
}

impl_inline_node!(Selector<'ast>, 0x001b_0001);

impl<'ast> AstNodeClone<'ast> for Selector<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Parsed(components) => Self::Parsed(context.clone_encoded_vec(components)),
            Self::Unparsed(value) => Self::Unparsed(value),
            Self::Tombstone => Self::Tombstone,
        }
    }
}

/// A CSS simple selector or combinator.
///
/// This mirrors `parcel_selectors::parser::Component`, specialized for
/// lightningcss' selector implementation and arena-backed containers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
pub enum SelectorComponent<'a> {
    Combinator(Combinator),

    ExplicitAnyNamespace,
    ExplicitNoNamespace,
    DefaultNamespace(Atom<'a>),
    Namespace {
        prefix: Atom<'a>,
        url: Atom<'a>,
    },

    ExplicitUniversalType,
    LocalName {
        name: Atom<'a>,
        lower_name: Atom<'a>,
    },

    Id(Atom<'a>),
    Class(Atom<'a>),

    AttributeInNoNamespaceExists {
        local_name: Atom<'a>,
        local_name_lower: Atom<'a>,
    },
    AttributeInNoNamespace {
        local_name: Atom<'a>,
        operator: AttrSelectorOperator,
        value: Atom<'a>,
        case_sensitivity: ParsedCaseSensitivity,
        never_matches: bool,
    },
    AttributeOther(NodeId<'a, AttrSelector<'a>>),

    Negation(Vec<'a, NodeId<'a, Selector<'a>>>),
    Root,
    Empty,
    Scope,
    Nth(NthSelectorData),
    NthOf {
        data: NthSelectorData,
        selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
    },
    PseudoClass(NodeId<'a, PseudoClass<'a>>),
    Slotted(NodeId<'a, Selector<'a>>),
    Part(Vec<'a, Atom<'a>>),
    Host(Option<NodeId<'a, Selector<'a>>>),
    Where(Vec<'a, NodeId<'a, Selector<'a>>>),
    Is(Vec<'a, NodeId<'a, Selector<'a>>>),
    Any {
        vendor_prefix: VendorPrefix,
        selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
    },
    Has(Vec<'a, NodeId<'a, Selector<'a>>>),
    PseudoElement(NodeId<'a, PseudoElement<'a>>),
    Nesting,
}

// The native slot keeps common variants inline. Only fields that exceed the
// payload capacity use typed overflow slots; no string reference IDs are stored.
#[derive(Clone, Copy)]
enum SelectorComponentSlot<'a> {
    Combinator(Combinator),

    ExplicitAnyNamespace,
    ExplicitNoNamespace,
    DefaultNamespace(Atom<'a>),
    Namespace {
        prefix: Atom<'a>,
        extra: u32,
    },

    ExplicitUniversalType,
    LocalName {
        name: Atom<'a>,
        extra: u32,
    },

    Id(Atom<'a>),
    Class(Atom<'a>),

    AttributeInNoNamespaceExists {
        local_name: Atom<'a>,
        extra: u32,
    },
    AttributeInNoNamespace {
        local_name: Atom<'a>,
        extra: u32,
    },
    AttributeOther(NodeId<'a, AttrSelector<'a>>),

    Negation(Vec<'a, NodeId<'a, Selector<'a>>>),
    Root,
    Empty,
    Scope,
    Nth(NthSelectorData),
    NthOf {
        extra: u32,
        selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
    },
    PseudoClass(NodeId<'a, PseudoClass<'a>>),
    Slotted(NodeId<'a, Selector<'a>>),
    Part(Vec<'a, Atom<'a>>),
    Host(Option<NodeId<'a, Selector<'a>>>),
    Where(Vec<'a, NodeId<'a, Selector<'a>>>),
    Is(Vec<'a, NodeId<'a, Selector<'a>>>),
    Any {
        vendor_prefix: VendorPrefix,
        selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
    },
    Has(Vec<'a, NodeId<'a, Selector<'a>>>),
    PseudoElement(NodeId<'a, PseudoElement<'a>>),
    Nesting,
}

pub use component_access::SelectorComponentSyntax;
// Transient authored syntax omits matching-only fields and does not enter visitor generation.
mod component_access {
    use super::*;
    pub enum SelectorComponentSyntax<'a> {
        Combinator(Combinator),

        ExplicitAnyNamespace,
        ExplicitNoNamespace,
        DefaultNamespace,
        NamespacePrefix(Atom<'a>),

        ExplicitUniversalType,
        LocalName(Atom<'a>),

        IdName(Atom<'a>),
        ClassName(Atom<'a>),

        AttributeExists(Atom<'a>),
        AttributeInNoNamespace {
            local_name: Atom<'a>,
            operator: AttrSelectorOperator,
            value: Atom<'a>,
            case_sensitivity: ParsedCaseSensitivity,
        },
        AttributeOther(NodeId<'a, AttrSelector<'a>>),

        Negation(Vec<'a, NodeId<'a, Selector<'a>>>),
        Root,
        Empty,
        Scope,
        Nth(NthSelectorData),
        NthOf {
            data: NthSelectorData,
            selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
        },
        PseudoClass(NodeId<'a, PseudoClass<'a>>),
        Slotted(NodeId<'a, Selector<'a>>),
        Part(Vec<'a, Atom<'a>>),
        Host(Option<NodeId<'a, Selector<'a>>>),
        Where(Vec<'a, NodeId<'a, Selector<'a>>>),
        Is(Vec<'a, NodeId<'a, Selector<'a>>>),
        Any {
            vendor_prefix: VendorPrefix,
            selectors: Vec<'a, NodeId<'a, Selector<'a>>>,
        },
        Has(Vec<'a, NodeId<'a, Selector<'a>>>),
        PseudoElement(NodeId<'a, PseudoElement<'a>>),
        Nesting,
    }

    impl<'a> From<&SelectorComponent<'a>> for SelectorComponentSyntax<'a> {
        fn from(value: &SelectorComponent<'a>) -> Self {
            match *value {
                SelectorComponent::Combinator(value) => Self::Combinator(value),
                SelectorComponent::AttributeOther(value) => Self::AttributeOther(value),
                SelectorComponent::Negation(value) => Self::Negation(value),
                SelectorComponent::Nth(value) => Self::Nth(value),
                SelectorComponent::PseudoClass(value) => Self::PseudoClass(value),
                SelectorComponent::Slotted(value) => Self::Slotted(value),
                SelectorComponent::Part(value) => Self::Part(value),
                SelectorComponent::Host(value) => Self::Host(value),
                SelectorComponent::Where(value) => Self::Where(value),
                SelectorComponent::Is(value) => Self::Is(value),
                SelectorComponent::Has(value) => Self::Has(value),
                SelectorComponent::PseudoElement(value) => Self::PseudoElement(value),
                SelectorComponent::ExplicitAnyNamespace => Self::ExplicitAnyNamespace,
                SelectorComponent::ExplicitNoNamespace => Self::ExplicitNoNamespace,
                SelectorComponent::ExplicitUniversalType => Self::ExplicitUniversalType,
                SelectorComponent::Root => Self::Root,
                SelectorComponent::Empty => Self::Empty,
                SelectorComponent::Scope => Self::Scope,
                SelectorComponent::Nesting => Self::Nesting,
                SelectorComponent::DefaultNamespace(_) => Self::DefaultNamespace,
                SelectorComponent::Namespace { prefix, .. } => Self::NamespacePrefix(prefix),
                SelectorComponent::LocalName { name, .. } => Self::LocalName(name),
                SelectorComponent::Id(value) => Self::IdName(value),
                SelectorComponent::Class(value) => Self::ClassName(value),
                SelectorComponent::AttributeInNoNamespaceExists { local_name, .. } => {
                    Self::AttributeExists(local_name)
                }
                SelectorComponent::AttributeInNoNamespace {
                    local_name,
                    operator,
                    value,
                    case_sensitivity,
                    ..
                } => Self::AttributeInNoNamespace {
                    local_name,
                    operator,
                    value,
                    case_sensitivity,
                },
                SelectorComponent::NthOf { data, selectors } => Self::NthOf { data, selectors },
                SelectorComponent::Any {
                    vendor_prefix,
                    selectors,
                } => Self::Any {
                    vendor_prefix,
                    selectors,
                },
            }
        }
    }
    impl AstContext<'_> {
        /// Tests only the native component tag, without reading matching or syntax overflow.
        #[inline]
        pub fn selector_component_is_combinator_or_type(
            &self,
            id: NodeId<'_, SelectorComponent<'_>>,
        ) -> bool {
            // SAFETY: node_payload validates the owning kind before reading its native slot.
            matches!(
                unsafe { self.node_payload(id).read_value::<SelectorComponentSlot>() },
                SelectorComponentSlot::Combinator(_)
                    | SelectorComponentSlot::LocalName { .. }
                    | SelectorComponentSlot::ExplicitUniversalType
            )
        }

        #[inline]
        pub fn selector_component_syntax<'id>(
            &self,
            id: NodeId<'id, SelectorComponent<'id>>,
        ) -> SelectorComponentSyntax<'id> {
            // SAFETY: node_payload validates the kind; each overflow branch reads its matching native type.
            let slot: SelectorComponentSlot<'id> = unsafe { self.node_payload(id).read_value() };
            match slot {
                SelectorComponentSlot::Combinator(value) => {
                    SelectorComponentSyntax::Combinator(value)
                }
                SelectorComponentSlot::ExplicitAnyNamespace => {
                    SelectorComponentSyntax::ExplicitAnyNamespace
                }
                SelectorComponentSlot::ExplicitNoNamespace => {
                    SelectorComponentSyntax::ExplicitNoNamespace
                }
                SelectorComponentSlot::DefaultNamespace(_) => {
                    SelectorComponentSyntax::DefaultNamespace
                }
                SelectorComponentSlot::Namespace { prefix, .. } => {
                    SelectorComponentSyntax::NamespacePrefix(prefix)
                }
                SelectorComponentSlot::ExplicitUniversalType => {
                    SelectorComponentSyntax::ExplicitUniversalType
                }
                SelectorComponentSlot::LocalName { name, .. } => {
                    SelectorComponentSyntax::LocalName(name)
                }
                SelectorComponentSlot::Id(value) => SelectorComponentSyntax::IdName(value),
                SelectorComponentSlot::Class(value) => SelectorComponentSyntax::ClassName(value),
                SelectorComponentSlot::AttributeInNoNamespaceExists { local_name, .. } => {
                    SelectorComponentSyntax::AttributeExists(local_name)
                }
                SelectorComponentSlot::AttributeInNoNamespace { local_name, extra } => {
                    let (value, operator, case_sensitivity, _) = unsafe {
                        read_component_extra::<
                            (Atom<'id>, AttrSelectorOperator, ParsedCaseSensitivity, bool),
                            2,
                        >(extra, self)
                    };
                    SelectorComponentSyntax::AttributeInNoNamespace {
                        local_name,
                        value,
                        operator,
                        case_sensitivity,
                    }
                }
                SelectorComponentSlot::AttributeOther(value) => {
                    SelectorComponentSyntax::AttributeOther(value)
                }
                SelectorComponentSlot::Negation(value) => SelectorComponentSyntax::Negation(value),
                SelectorComponentSlot::Root => SelectorComponentSyntax::Root,
                SelectorComponentSlot::Empty => SelectorComponentSyntax::Empty,
                SelectorComponentSlot::Scope => SelectorComponentSyntax::Scope,
                SelectorComponentSlot::Nth(value) => SelectorComponentSyntax::Nth(value),
                SelectorComponentSlot::NthOf { selectors, extra } => {
                    SelectorComponentSyntax::NthOf {
                        selectors,
                        data: unsafe { read_component_extra::<NthSelectorData, 2>(extra, self) },
                    }
                }
                SelectorComponentSlot::PseudoClass(value) => {
                    SelectorComponentSyntax::PseudoClass(value)
                }
                SelectorComponentSlot::Slotted(value) => SelectorComponentSyntax::Slotted(value),
                SelectorComponentSlot::Part(value) => SelectorComponentSyntax::Part(value),
                SelectorComponentSlot::Host(value) => SelectorComponentSyntax::Host(value),
                SelectorComponentSlot::Where(value) => SelectorComponentSyntax::Where(value),
                SelectorComponentSlot::Is(value) => SelectorComponentSyntax::Is(value),
                SelectorComponentSlot::Any {
                    vendor_prefix,
                    selectors,
                } => SelectorComponentSyntax::Any {
                    vendor_prefix,
                    selectors,
                },
                SelectorComponentSlot::Has(value) => SelectorComponentSyntax::Has(value),
                SelectorComponentSlot::PseudoElement(value) => {
                    SelectorComponentSyntax::PseudoElement(value)
                }
                SelectorComponentSlot::Nesting => SelectorComponentSyntax::Nesting,
            }
        }
    }
}

// SAFETY: KIND identifies SelectorComponentSlot; each overflow variant writes
// and reads its own fixed Copy type and slot count.
unsafe impl<'ast> AstNodeStorage<'ast> for SelectorComponent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0002);
    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let slot = unsafe { payload.read_value() };
        unsafe { Self::from_slot(slot, context) }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        self.into_payload(None, context)
    }
    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        self.into_payload(Some(unsafe { current.read_value() }), context)
    }
}
impl<'ast> SelectorComponent<'ast> {
    // SAFETY: slot and its overflow must originate in this context with this kind.
    unsafe fn from_slot(slot: SelectorComponentSlot<'ast>, context: &AstContext<'_>) -> Self {
        use SelectorComponentSlot as S;
        match slot {
            S::Combinator(value) => Self::Combinator(value),
            S::ExplicitAnyNamespace => Self::ExplicitAnyNamespace,
            S::ExplicitNoNamespace => Self::ExplicitNoNamespace,
            S::DefaultNamespace(value) => Self::DefaultNamespace(value),
            S::Namespace { prefix, extra } => Self::Namespace {
                prefix,
                url: unsafe { read_component_extra::<Atom<'ast>, 1>(extra, context) },
            },
            S::ExplicitUniversalType => Self::ExplicitUniversalType,
            S::LocalName { name, extra } => Self::LocalName {
                name,
                lower_name: unsafe { read_component_extra::<Atom<'ast>, 1>(extra, context) },
            },
            S::Id(value) => Self::Id(value),
            S::Class(value) => Self::Class(value),
            S::AttributeInNoNamespaceExists { local_name, extra } => {
                Self::AttributeInNoNamespaceExists {
                    local_name,
                    local_name_lower: unsafe {
                        read_component_extra::<Atom<'ast>, 1>(extra, context)
                    },
                }
            }
            S::AttributeInNoNamespace { local_name, extra } => {
                let (value, operator, case_sensitivity, never_matches) = unsafe {
                    read_component_extra::<
                        (
                            Atom<'ast>,
                            AttrSelectorOperator,
                            ParsedCaseSensitivity,
                            bool,
                        ),
                        2,
                    >(extra, context)
                };
                Self::AttributeInNoNamespace {
                    local_name,
                    value,
                    operator,
                    case_sensitivity,
                    never_matches,
                }
            }
            S::AttributeOther(value) => Self::AttributeOther(value),
            S::Negation(value) => Self::Negation(value),
            S::Root => Self::Root,
            S::Empty => Self::Empty,
            S::Scope => Self::Scope,
            S::Nth(value) => Self::Nth(value),
            S::NthOf { selectors, extra } => Self::NthOf {
                selectors,
                data: unsafe { read_component_extra::<NthSelectorData, 2>(extra, context) },
            },
            S::PseudoClass(value) => Self::PseudoClass(value),
            S::Slotted(value) => Self::Slotted(value),
            S::Part(value) => Self::Part(value),
            S::Host(value) => Self::Host(value),
            S::Where(value) => Self::Where(value),
            S::Is(value) => Self::Is(value),
            S::Any {
                vendor_prefix,
                selectors,
            } => Self::Any {
                vendor_prefix,
                selectors,
            },
            S::Has(value) => Self::Has(value),
            S::PseudoElement(value) => Self::PseudoElement(value),
            S::Nesting => Self::Nesting,
        }
    }

    fn into_payload(
        self,
        current: Option<SelectorComponentSlot<'ast>>,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        use SelectorComponentSlot as S;
        NodePayload::from_value(match self {
            Self::Combinator(value) => S::Combinator(value),
            Self::ExplicitAnyNamespace => S::ExplicitAnyNamespace,
            Self::ExplicitNoNamespace => S::ExplicitNoNamespace,
            Self::DefaultNamespace(value) => S::DefaultNamespace(value),
            Self::Namespace { prefix, url } => S::Namespace {
                prefix,
                extra: write_component_extra::<_, 1>(
                    url,
                    match current {
                        Some(S::Namespace { extra, .. }) => Some(extra),
                        _ => None,
                    },
                    context,
                ),
            },
            Self::ExplicitUniversalType => S::ExplicitUniversalType,
            Self::LocalName { name, lower_name } => S::LocalName {
                name,
                extra: write_component_extra::<_, 1>(
                    lower_name,
                    match current {
                        Some(S::LocalName { extra, .. }) => Some(extra),
                        _ => None,
                    },
                    context,
                ),
            },
            Self::Id(value) => S::Id(value),
            Self::Class(value) => S::Class(value),
            Self::AttributeInNoNamespaceExists {
                local_name,
                local_name_lower,
            } => S::AttributeInNoNamespaceExists {
                local_name,
                extra: write_component_extra::<_, 1>(
                    local_name_lower,
                    match current {
                        Some(S::AttributeInNoNamespaceExists { extra, .. }) => Some(extra),
                        _ => None,
                    },
                    context,
                ),
            },
            Self::AttributeInNoNamespace {
                local_name,
                value,
                operator,
                case_sensitivity,
                never_matches,
            } => S::AttributeInNoNamespace {
                local_name,
                extra: write_component_extra::<_, 2>(
                    (value, operator, case_sensitivity, never_matches),
                    match current {
                        Some(S::AttributeInNoNamespace { extra, .. }) => Some(extra),
                        _ => None,
                    },
                    context,
                ),
            },
            Self::AttributeOther(value) => S::AttributeOther(value),
            Self::Negation(value) => S::Negation(value),
            Self::Root => S::Root,
            Self::Empty => S::Empty,
            Self::Scope => S::Scope,
            Self::Nth(value) => S::Nth(value),
            Self::NthOf { selectors, data } => S::NthOf {
                selectors,
                extra: write_component_extra::<_, 2>(
                    data,
                    match current {
                        Some(S::NthOf { extra, .. }) => Some(extra),
                        _ => None,
                    },
                    context,
                ),
            },
            Self::PseudoClass(value) => S::PseudoClass(value),
            Self::Slotted(value) => S::Slotted(value),
            Self::Part(value) => S::Part(value),
            Self::Host(value) => S::Host(value),
            Self::Where(value) => S::Where(value),
            Self::Is(value) => S::Is(value),
            Self::Any {
                vendor_prefix,
                selectors,
            } => S::Any {
                vendor_prefix,
                selectors,
            },
            Self::Has(value) => S::Has(value),
            Self::PseudoElement(value) => S::PseudoElement(value),
            Self::Nesting => S::Nesting,
        })
    }
}

fn write_component_extra<T: Copy, const N: usize>(
    value: T,
    current: Option<u32>,
    context: &mut AstContext<'_>,
) -> u32 {
    let slots = ExtraData::from_value_array::<T, N>(value);
    match current {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra as usize + i, slot);
            }
            extra
        }
        None => u32::try_from(context.alloc_extra_slots(slots))
            .expect("selector overflow index exceeds u32"),
    }
}
// SAFETY: caller must identify the matching writer type and slot count.
unsafe fn read_component_extra<T: Copy, const N: usize>(extra: u32, context: &AstContext<'_>) -> T {
    unsafe {
        ExtraData::read_value_array::<T, N>(std::array::from_fn(|i| {
            context.extra_slot(extra as usize + i)
        }))
    }
}

impl<'ast> AstNodeClone<'ast> for SelectorComponent<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::AttributeOther(value) => Self::AttributeOther(context.clone_encoded_node(value)),
            Self::Negation(values) => Self::Negation(context.clone_encoded_vec(values)),
            Self::NthOf { data, selectors } => Self::NthOf {
                data,
                selectors: context.clone_encoded_vec(selectors),
            },
            Self::PseudoClass(value) => Self::PseudoClass(context.clone_encoded_node(value)),
            Self::Slotted(value) => Self::Slotted(context.clone_encoded_node(value)),
            Self::Part(values) => Self::Part(context.clone_encoded_vec(values)),
            Self::Host(value) => Self::Host(value.map(|value| context.clone_encoded_node(value))),
            Self::Where(values) => Self::Where(context.clone_encoded_vec(values)),
            Self::Is(values) => Self::Is(context.clone_encoded_vec(values)),
            Self::Any {
                vendor_prefix,
                selectors,
            } => Self::Any {
                vendor_prefix,
                selectors: context.clone_encoded_vec(selectors),
            },
            Self::Has(values) => Self::Has(context.clone_encoded_vec(values)),
            Self::PseudoElement(value) => Self::PseudoElement(context.clone_encoded_node(value)),
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum Combinator {
    Child,
    Descendant,
    NextSibling,
    LaterSibling,
    PseudoElement,
    SlotAssignment,
    Part,
    DeepDescendant,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub struct AttrSelector<'a> {
    pub namespace: Option<NamespaceConstraint<'a>>,
    pub local_name: Atom<'a>,
    pub local_name_lower: Atom<'a>,
    pub operation: AttrOperation<'a>,
    pub never_matches: bool,
}

#[derive(Clone, Copy)]
struct AttrSelectorHeader<'a> {
    local_name: Atom<'a>,
    extra: u32,
    never_matches: bool,
}

#[derive(Clone, Copy)]
struct AttrSelectorFields<'a> {
    namespace: Option<NamespaceConstraint<'a>>,
    operation: AttrOperation<'a>,
}

// SAFETY: this KIND stores a native header, one Atom slot, then four opaque
// slots written and read as the same AttrSelectorFields type.
unsafe impl<'ast> AstNodeStorage<'ast> for AttrSelector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0003);

    unsafe fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let header: AttrSelectorHeader<'ast> = unsafe { payload.read_value() };
        let fields: AttrSelectorFields<'ast> = unsafe {
            ExtraData::read_value_array::<_, 4>(std::array::from_fn(|i| {
                context.extra_slot(header.extra as usize + 1 + i)
            }))
        };
        Self {
            local_name: header.local_name,
            never_matches: header.never_matches,
            local_name_lower: unsafe { context.extra_slot(header.extra as usize).read_value() },
            namespace: fields.namespace,
            operation: fields.operation,
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_attr_selector(self, None, context)
    }

    unsafe fn encode_existing(
        self,
        current: NodePayload,
        context: &mut AstContext<'ast>,
    ) -> NodePayload {
        let header: AttrSelectorHeader<'ast> = unsafe { current.read_value() };
        encode_attr_selector(self, Some(header.extra as usize), context)
    }
}

impl AstContext<'_> {
    /// Reads authored attribute-selector fields without loading the lowercase
    /// matching name. The typed ID validates the node kind before slot access.
    pub fn attr_selector_syntax<'id>(
        &self,
        id: NodeId<'id, AttrSelector<'id>>,
    ) -> (
        Atom<'id>,
        Option<NamespaceConstraint<'id>>,
        AttrOperation<'id>,
    ) {
        let header: AttrSelectorHeader<'id> = unsafe { self.node_payload(id).read_value() };
        let fields: AttrSelectorFields<'id> = unsafe {
            ExtraData::read_value_array::<_, 4>(std::array::from_fn(|i| {
                self.extra_slot(header.extra as usize + 1 + i)
            }))
        };
        (header.local_name, fields.namespace, fields.operation)
    }
}

impl<'ast> AstNodeClone<'ast> for AttrSelector<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_attr_selector<'ast>(
    value: AttrSelector<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let [a, b, c, d] = ExtraData::from_value_array::<_, 4>(AttrSelectorFields {
        namespace: value.namespace,
        operation: value.operation,
    });
    let slots = [ExtraData::from_value(value.local_name_lower), a, b, c, d];
    let extra = match existing_extra {
        Some(extra) => {
            for (i, slot) in slots.into_iter().enumerate() {
                context.set_extra_slot(extra + i, slot);
            }
            extra
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::from_value(AttrSelectorHeader {
        local_name: value.local_name,
        never_matches: value.never_matches,
        extra: u32::try_from(extra).expect("AttrSelector overflow index exceeds u32"),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum NamespaceConstraint<'a> {
    Any,
    Specific { prefix: Atom<'a>, url: Atom<'a> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum AttrOperation<'a> {
    Exists,
    WithValue {
        operator: AttrSelectorOperator,
        case_sensitivity: ParsedCaseSensitivity,
        expected_value: Atom<'a>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Visit)]
pub enum ParsedCaseSensitivity {
    ExplicitCaseSensitive,
    AsciiCaseInsensitive,
    #[default]
    CaseSensitive,
    AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum AttrSelectorOperator {
    Equal,
    Includes,
    DashMatch,
    Prefix,
    Substring,
    Suffix,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum NthType {
    Child,
    LastChild,
    OnlyChild,
    OfType,
    LastOfType,
    OnlyOfType,
    Col,
    LastCol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub struct NthSelectorData {
    pub kind: NthType,
    pub is_function: bool,
    pub a: i32,
    pub b: i32,
}

#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum Direction {
    Ltr,
    Rtl,
}

/// Name and arguments of an authored custom pseudo function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub struct CustomPseudoFunction<'a> {
    pub name: Atom<'a>,
    pub arguments: Vec<'a, TokenOrValue<'a>>,
}

impl_inline_node!(CustomPseudoFunction<'ast>, 0x001b_0008);

impl<'ast> AstNodeClone<'ast> for CustomPseudoFunction<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        Self {
            name: self.name,
            arguments: context.clone_encoded_vec(self.arguments),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum PseudoClass<'a> {
    Lang {
        languages: Vec<'a, Atom<'a>>,
    },
    Dir {
        direction: Direction,
    },

    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Current,
    Past,
    Future,
    Playing,
    Paused,
    Seeking,
    Buffering,
    Stalled,
    Muted,
    VolumeLocked,
    Fullscreen(VendorPrefix),
    Open,
    Closed,
    Modal,
    PictureInPicture,
    PopoverOpen,
    Defined,
    AnyLink(VendorPrefix),
    Link,
    LocalLink,
    Target,
    TargetCurrent,
    TargetBefore,
    TargetAfter,
    TargetWithin,
    Visited,
    Enabled,
    Disabled,
    ReadOnly(VendorPrefix),
    ReadWrite(VendorPrefix),
    PlaceholderShown(VendorPrefix),
    Default,
    Checked,
    Indeterminate,
    Blank,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    Required,
    Optional,
    UserValid,
    UserInvalid,
    Autofill(VendorPrefix),
    ActiveViewTransition,
    ActiveViewTransitionType {
        kinds: Vec<'a, Atom<'a>>,
    },
    State {
        state: Atom<'a>,
    },
    Local {
        selector: NodeId<'a, Selector<'a>>,
    },
    Global {
        selector: NodeId<'a, Selector<'a>>,
    },
    WebKitScrollbar(WebKitScrollbarPseudoClass),
    Custom {
        name: Atom<'a>,
    },
    CustomFunction {
        function: NodeId<'a, CustomPseudoFunction<'a>>,
    },
}

impl_inline_node!(PseudoClass<'ast>, 0x001b_0004);

impl<'ast> AstNodeClone<'ast> for PseudoClass<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Lang { languages } => Self::Lang {
                languages: context.clone_encoded_vec(languages),
            },
            Self::ActiveViewTransitionType { kinds } => Self::ActiveViewTransitionType {
                kinds: context.clone_encoded_vec(kinds),
            },
            Self::Local { selector } => Self::Local {
                selector: context.clone_encoded_node(selector),
            },
            Self::Global { selector } => Self::Global {
                selector: context.clone_encoded_node(selector),
            },
            Self::CustomFunction { function } => Self::CustomFunction {
                function: context.clone_encoded_node(function),
            },
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum WebKitScrollbarPseudoClass {
    Horizontal,
    Vertical,
    Decrement,
    Increment,
    Start,
    End,
    DoubleButton,
    SingleButton,
    NoButton,
    CornerPresent,
    WindowInactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum PseudoElement<'a> {
    After,
    Before,
    FirstLine,
    FirstLetter,
    DetailsContent,
    TargetText,
    SearchText,
    Selection(VendorPrefix),
    Placeholder(VendorPrefix),
    HighlightFunction {
        name: Atom<'a>,
    },
    Marker,
    Backdrop(VendorPrefix),
    FileSelectorButton(VendorPrefix),
    WebKitScrollbar(WebKitScrollbarPseudoElement),
    Cue,
    CueRegion,
    CueFunction {
        selector: NodeId<'a, Selector<'a>>,
    },
    CueRegionFunction {
        selector: NodeId<'a, Selector<'a>>,
    },
    ViewTransition,
    ViewTransitionGroup {
        part: NodeId<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionImagePair {
        part: NodeId<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionOld {
        part: NodeId<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionNew {
        part: NodeId<'a, ViewTransitionPartSelector<'a>>,
    },
    PickerFunction {
        identifier: Atom<'a>,
    },
    PickerIcon,
    Checkmark,
    GrammarError,
    SpellingError,
    Custom {
        name: Atom<'a>,
    },
    CustomFunction {
        function: NodeId<'a, CustomPseudoFunction<'a>>,
    },
}

impl_inline_node!(PseudoElement<'ast>, 0x001b_0006);

impl<'ast> AstNodeClone<'ast> for PseudoElement<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::CueFunction { selector } => Self::CueFunction {
                selector: context.clone_encoded_node(selector),
            },
            Self::CueRegionFunction { selector } => Self::CueRegionFunction {
                selector: context.clone_encoded_node(selector),
            },
            Self::ViewTransitionGroup { part } => Self::ViewTransitionGroup {
                part: context.clone_encoded_node(part),
            },
            Self::ViewTransitionImagePair { part } => Self::ViewTransitionImagePair {
                part: context.clone_encoded_node(part),
            },
            Self::ViewTransitionOld { part } => Self::ViewTransitionOld {
                part: context.clone_encoded_node(part),
            },
            Self::ViewTransitionNew { part } => Self::ViewTransitionNew {
                part: context.clone_encoded_node(part),
            },
            Self::CustomFunction { function } => Self::CustomFunction {
                function: context.clone_encoded_node(function),
            },
            value => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Visit)]
pub enum WebKitScrollbarPseudoElement {
    Scrollbar,
    Button,
    Track,
    TrackPiece,
    Thumb,
    Corner,
    Resizer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Visit)]
pub enum ViewTransitionPartName<'a> {
    All,
    Name(Atom<'a>),
}

impl_inline_node!(ViewTransitionPartName<'ast>, 0x001b_0007);

impl<'ast> AstNodeClone<'ast> for ViewTransitionPartName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::Allocator;

    #[test]
    fn component_native_layout_reuses_overflow_and_keeps_simple_variants_inline() {
        assert_eq!(std::mem::size_of::<SelectorComponentSlot<'_>>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let first = context.intern("Name");
        let second = context.intern("name");
        let node = context.alloc_encoded_node(SelectorComponent::Class(first), DUMMY_SP);
        assert_eq!(context.encoded_extra_len(), 0);
        let checkpoint = context.node_checkpoint();
        for value in [
            SelectorComponent::Id(second),
            SelectorComponent::Root,
            SelectorComponent::Host(None),
            SelectorComponent::Combinator(Combinator::Deep),
            SelectorComponent::DefaultNamespace(first),
            SelectorComponent::Class(second),
        ] {
            context.mutate_encoded_node(node, |stored, _| *stored = value.clone());
            assert_eq!(context.encoded_node(node), value);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        for (value, slots) in [
            (
                SelectorComponent::Namespace {
                    prefix: first,
                    url: second,
                },
                1,
            ),
            (
                SelectorComponent::LocalName {
                    name: first,
                    lower_name: second,
                },
                1,
            ),
            (
                SelectorComponent::AttributeInNoNamespaceExists {
                    local_name: first,
                    local_name_lower: second,
                },
                1,
            ),
            (
                SelectorComponent::AttributeInNoNamespace {
                    local_name: first,
                    value: second,
                    operator: AttrSelectorOperator::Suffix,
                    case_sensitivity: ParsedCaseSensitivity::ExplicitCaseSensitive,
                    never_matches: true,
                },
                2,
            ),
        ] {
            let before = context.encoded_extra_len();
            context.mutate_encoded_node(node, |stored, _| *stored = value.clone());
            assert_eq!(context.encoded_extra_len(), before + slots);
            let checkpoint = context.node_checkpoint();
            let pool_bytes = context.string_pool().extra_len();
            for _ in 0..4 {
                assert_eq!(context.encoded_node(node), value);
                context.mutate_encoded_node(node, |stored, _| *stored = value.clone());
            }
            assert_eq!(context.node_checkpoint(), checkpoint);
            assert_eq!(context.string_pool().extra_len(), pool_bytes);
        }
    }

    #[test]
    fn native_pseudos_reuse_storage_and_clone_custom_arguments() {
        assert!(std::mem::size_of::<PseudoClass<'_>>() <= 16);
        assert!(std::mem::size_of::<PseudoElement<'_>>() <= 16);
        assert_eq!(std::mem::size_of::<CustomPseudoFunction<'_>>(), 16);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.intern("custom");
        let arguments =
            context.alloc_encoded_vec([TokenOrValue::Angle(Angle::Deg(45.0))].into_iter());
        let function =
            context.alloc_encoded_node(CustomPseudoFunction { name, arguments }, DUMMY_SP);
        let class = context.alloc_encoded_node(PseudoClass::Hover, DUMMY_SP);
        let element = context.alloc_encoded_node(PseudoElement::Before, DUMMY_SP);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..4 {
            context.mutate_encoded_node(class, |value, _| {
                *value = PseudoClass::CustomFunction { function }
            });
            context.mutate_encoded_node(element, |value, _| {
                *value = PseudoElement::CustomFunction { function }
            });
            assert_eq!(
                context.encoded_node(class),
                PseudoClass::CustomFunction { function }
            );
            assert_eq!(
                context.encoded_node(element),
                PseudoElement::CustomFunction { function }
            );
            context.mutate_encoded_node(class, |value, _| {
                *value = PseudoClass::State { state: name }
            });
            context.mutate_encoded_node(element, |value, _| {
                *value = PseudoElement::HighlightFunction { name }
            });
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
        let cloned = context.clone_encoded_node(function);
        let cloned = context.encoded_node(cloned);
        assert_eq!(cloned.name, name);
        assert_ne!(cloned.arguments, arguments);
        assert_eq!(
            context.encoded_vec_get(cloned.arguments, 0),
            context.encoded_vec_get(arguments, 0)
        );
    }

    #[test]
    fn inline_selector_names_do_not_allocate_reference_rows() {
        assert_eq!(std::mem::size_of::<Selector<'_>>(), 12);
        assert_eq!(std::mem::size_of::<ViewTransitionPartName<'_>>(), 12);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let name = context.intern("named");
        let selector = context.alloc_encoded_node(Selector::Tombstone, DUMMY_SP);
        let part = context.alloc_encoded_node(ViewTransitionPartName::All, DUMMY_SP);
        let checkpoint = context.node_checkpoint();
        let bytes = context.string_pool().extra_len();
        for _ in 0..4 {
            context.mutate_encoded_node(selector, |value, _| *value = Selector::Unparsed(name));
            context.mutate_encoded_node(part, |value, _| {
                *value = ViewTransitionPartName::Name(name);
            });
            assert_eq!(context.encoded_node(selector), Selector::Unparsed(name));
            assert_eq!(
                context.encoded_node(part),
                ViewTransitionPartName::Name(name)
            );
            context.mutate_encoded_node(selector, |value, _| *value = Selector::Tombstone);
            context.mutate_encoded_node(part, |value, _| *value = ViewTransitionPartName::All);
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), bytes);
    }

    #[test]
    fn selector_clone_recursively_clones_node_graph() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);

        let nested = context.intern("nested");
        let nested_component =
            context.alloc_encoded_node(SelectorComponent::Class(nested), Span::new(1, 7));
        let nested_components = context.alloc_encoded_vec([nested_component].into_iter());
        let nested_selector =
            context.alloc_encoded_node(Selector::Parsed(nested_components), Span::new(1, 7));
        let pseudo = context.alloc_encoded_node(
            PseudoClass::Local {
                selector: nested_selector,
            },
            Span::new(0, 8),
        );
        let component =
            context.alloc_encoded_node(SelectorComponent::PseudoClass(pseudo), Span::new(0, 8));
        let components = context.alloc_encoded_vec([component].into_iter());
        let selector = context.alloc_encoded_node(Selector::Parsed(components), Span::new(0, 8));

        let cloned = context.clone_encoded_node(selector);
        let Selector::Parsed(cloned_components) = context.encoded_node(cloned) else {
            panic!("expected parsed selector");
        };
        let cloned_component = context.encoded_vec_get(cloned_components, 0).unwrap();
        let SelectorComponent::PseudoClass(cloned_pseudo) = context.encoded_node(cloned_component)
        else {
            panic!("expected pseudo class component");
        };
        let PseudoClass::Local {
            selector: cloned_nested_selector,
        } = context.encoded_node(cloned_pseudo)
        else {
            panic!("expected local pseudo class");
        };

        assert_ne!(selector, cloned);
        assert_ne!(component, cloned_component);
        assert_ne!(pseudo, cloned_pseudo);
        assert_ne!(nested_selector, cloned_nested_selector);
        assert_eq!(context.encoded_node_span(cloned), Span::new(0, 8));
        assert!(matches!(
            context.encoded_node(cloned_nested_selector),
            Selector::Parsed(_)
        ));
    }

    #[test]
    fn selector_overflow_mutation_reuses_fixed_extra_slots() {
        assert!(std::mem::size_of::<AttrSelectorHeader<'_>>() <= 16);
        assert_eq!(std::mem::size_of::<AttrSelectorFields<'_>>(), 32);
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let prefix = context.intern("svg");
        let url = context.intern("urn:svg");
        let local_name = context.intern("HREF");
        let lower_name = context.intern("href");
        let expected_value = context.intern("icon");
        let before = context.encoded_extra_len();
        let attribute = context.alloc_encoded_node(
            AttrSelector {
                namespace: Some(NamespaceConstraint::Specific { prefix, url }),
                local_name,
                local_name_lower: lower_name,
                operation: AttrOperation::WithValue {
                    operator: AttrSelectorOperator::Prefix,
                    case_sensitivity: ParsedCaseSensitivity::AsciiCaseInsensitive,
                    expected_value,
                },
                never_matches: true,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 5);
        let checkpoint = context.node_checkpoint();
        let pool_bytes = context.string_pool().extra_len();
        for namespace in [
            None,
            Some(NamespaceConstraint::Any),
            Some(NamespaceConstraint::Specific { prefix, url }),
        ] {
            for operator in [
                AttrSelectorOperator::Equal,
                AttrSelectorOperator::Includes,
                AttrSelectorOperator::DashMatch,
                AttrSelectorOperator::Prefix,
                AttrSelectorOperator::Substring,
                AttrSelectorOperator::Suffix,
            ] {
                for case_sensitivity in [
                    ParsedCaseSensitivity::ExplicitCaseSensitive,
                    ParsedCaseSensitivity::AsciiCaseInsensitive,
                    ParsedCaseSensitivity::CaseSensitive,
                    ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
                ] {
                    let operation = AttrOperation::WithValue {
                        operator,
                        case_sensitivity,
                        expected_value,
                    };
                    context.mutate_encoded_node(attribute, |value, _| {
                        value.namespace = namespace;
                        value.operation = operation;
                    });
                    let decoded = context.encoded_node(attribute);
                    assert_eq!(decoded.namespace, namespace);
                    assert_eq!(decoded.operation, operation);
                    assert_eq!(decoded.local_name, local_name);
                    assert_eq!(decoded.local_name_lower, lower_name);
                    assert_eq!(
                        context.attr_selector_syntax(attribute),
                        (local_name, namespace, operation)
                    );
                    assert!(decoded.never_matches);
                }
            }
        }
        assert_eq!(context.node_checkpoint(), checkpoint);
        assert_eq!(context.string_pool().extra_len(), pool_bytes);

        context.mutate_encoded_node(attribute, |attribute, _| {
            attribute.namespace = Some(NamespaceConstraint::Any);
            attribute.operation = AttrOperation::Exists;
            attribute.never_matches = false;
        });

        assert_eq!(context.encoded_extra_len(), before + 5);
        assert_eq!(
            context.encoded_node(attribute),
            AttrSelector {
                namespace: Some(NamespaceConstraint::Any),
                local_name,
                local_name_lower: lower_name,
                operation: AttrOperation::Exists,
                never_matches: false,
            }
        );
    }

    #[test]
    fn nth_of_mutation_reuses_variant_overflow_slot() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let child = context.alloc_encoded_node(Selector::Tombstone, DUMMY_SP);
        let selectors = context.alloc_encoded_vec([child].into_iter());
        let before = context.encoded_extra_len();
        let component = context.alloc_encoded_node(
            SelectorComponent::NthOf {
                data: NthSelectorData {
                    kind: NthType::Child,
                    is_function: true,
                    a: 2,
                    b: 1,
                },
                selectors,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 2);

        context.mutate_encoded_node(component, |component, _| {
            let SelectorComponent::NthOf { data, .. } = component else {
                panic!("expected nth-of component");
            };
            data.b = -1;
        });

        assert_eq!(context.encoded_extra_len(), before + 2);
        assert!(matches!(
            context.encoded_node(component),
            SelectorComponent::NthOf {
                data: NthSelectorData { b: -1, .. },
                ..
            }
        ));
    }
}

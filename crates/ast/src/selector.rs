use super::*;

/// A selector list in source order.
pub type SelectorList<'a> = Vec<'a, NodeId<'a, Selector<'a>>>;

/// A complex selector, a losslessly preserved invalid selector, or a removed selector.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
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

impl<'ast> AstNodeStorage<'ast> for Selector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0001);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Parsed(read_range(&bytes, context)),
            1 => Self::Unparsed(context.resolve_atom(read_u32(&bytes, 4) as u64)),
            2 => Self::Tombstone,
            _ => panic!("invalid encoded Selector variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_selector(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_selector(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for Selector<'ast> {
    fn clone_in_context(self, context: &mut AstContext<'ast>) -> Self {
        match self {
            Self::Parsed(components) => Self::Parsed(context.clone_encoded_vec(components)),
            Self::Unparsed(value) => Self::Unparsed(value),
            Self::Tombstone => Self::Tombstone,
        }
    }
}

fn encode_selector<'ast>(value: Selector<'ast>, context: &mut AstContext<'ast>) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        Selector::Parsed(components) => write_range(&mut bytes, 0, components),
        Selector::Unparsed(value) => {
            bytes[0] = 1;
            write_u32(&mut bytes, 4, context.store_atom(value));
        }
        Selector::Tombstone => bytes[0] = 2,
    }
    NodePayload::inline(&bytes)
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

impl<'ast> AstNodeStorage<'ast> for SelectorComponent<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0002);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::Combinator(decode_combinator(bytes[1])),
            1 => Self::ExplicitAnyNamespace,
            2 => Self::ExplicitNoNamespace,
            3 => Self::DefaultNamespace(context.resolve_atom(read_u32(&bytes, 4) as u64)),
            4 => Self::Namespace {
                prefix: context.resolve_atom(read_u32(&bytes, 4) as u64),
                url: context.resolve_atom(read_u32(&bytes, 8) as u64),
            },
            5 => Self::ExplicitUniversalType,
            6 => Self::LocalName {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
                lower_name: context.resolve_atom(read_u32(&bytes, 8) as u64),
            },
            7 => Self::Id(context.resolve_atom(read_u32(&bytes, 4) as u64)),
            8 => Self::Class(context.resolve_atom(read_u32(&bytes, 4) as u64)),
            9 => Self::AttributeInNoNamespaceExists {
                local_name: context.resolve_atom(read_u32(&bytes, 4) as u64),
                local_name_lower: context.resolve_atom(read_u32(&bytes, 8) as u64),
            },
            10 => Self::AttributeInNoNamespace {
                local_name: context.resolve_atom(read_u32(&bytes, 4) as u64),
                operator: decode_attr_operator(bytes[2]),
                value: context.resolve_atom(read_u32(&bytes, 8) as u64),
                case_sensitivity: decode_case_sensitivity(bytes[3]),
                never_matches: bytes[1] != 0,
            },
            11 => Self::AttributeOther(read_node_id(&bytes, context)),
            12 => Self::Negation(read_range(&bytes, context)),
            13 => Self::Root,
            14 => Self::Empty,
            15 => Self::Scope,
            16 => Self::Nth(decode_nth_data(&bytes)),
            17 => Self::NthOf {
                data: decode_nth_data(&bytes),
                selectors: decode_extra_range(context.extra_slot(payload.extra_start()), context),
            },
            18 => Self::PseudoClass(read_node_id(&bytes, context)),
            19 => Self::Slotted(read_node_id(&bytes, context)),
            20 => Self::Part(read_range(&bytes, context)),
            21 => Self::Host(match bytes[1] {
                0 => None,
                1 => Some(read_node_id(&bytes, context)),
                _ => panic!("invalid encoded SelectorComponent host flag"),
            }),
            22 => Self::Where(read_range(&bytes, context)),
            23 => Self::Is(read_range(&bytes, context)),
            24 => Self::Any {
                vendor_prefix: VendorPrefix::from_bits_retain(bytes[1]),
                selectors: read_range(&bytes, context),
            },
            25 => Self::Has(read_range(&bytes, context)),
            26 => Self::PseudoElement(read_node_id(&bytes, context)),
            27 => Self::Nesting,
            _ => panic!("invalid encoded SelectorComponent variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_selector_component(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        let extra = (current.bytes()[0] == 17).then(|| current.extra_start());
        encode_selector_component(self, extra, context)
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

fn encode_selector_component<'ast>(
    value: SelectorComponent<'ast>,
    existing_extra: Option<usize>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        SelectorComponent::Combinator(value) => {
            bytes[0] = 0;
            bytes[1] = encode_combinator(value);
        }
        SelectorComponent::ExplicitAnyNamespace => bytes[0] = 1,
        SelectorComponent::ExplicitNoNamespace => bytes[0] = 2,
        SelectorComponent::DefaultNamespace(value) => write_atom(&mut bytes, 3, 4, value, context),
        SelectorComponent::Namespace { prefix, url } => {
            bytes[0] = 4;
            write_u32(&mut bytes, 4, context.store_atom(prefix));
            write_u32(&mut bytes, 8, context.store_atom(url));
        }
        SelectorComponent::ExplicitUniversalType => bytes[0] = 5,
        SelectorComponent::LocalName { name, lower_name } => {
            bytes[0] = 6;
            write_u32(&mut bytes, 4, context.store_atom(name));
            write_u32(&mut bytes, 8, context.store_atom(lower_name));
        }
        SelectorComponent::Id(value) => write_atom(&mut bytes, 7, 4, value, context),
        SelectorComponent::Class(value) => write_atom(&mut bytes, 8, 4, value, context),
        SelectorComponent::AttributeInNoNamespaceExists {
            local_name,
            local_name_lower,
        } => {
            bytes[0] = 9;
            write_u32(&mut bytes, 4, context.store_atom(local_name));
            write_u32(&mut bytes, 8, context.store_atom(local_name_lower));
        }
        SelectorComponent::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            never_matches,
        } => {
            bytes[0] = 10;
            bytes[1] = never_matches as u8;
            bytes[2] = encode_attr_operator(operator);
            bytes[3] = encode_case_sensitivity(case_sensitivity);
            write_u32(&mut bytes, 4, context.store_atom(local_name));
            write_u32(&mut bytes, 8, context.store_atom(value));
        }
        SelectorComponent::AttributeOther(value) => write_node_id(&mut bytes, 11, value),
        SelectorComponent::Negation(values) => write_range(&mut bytes, 12, values),
        SelectorComponent::Root => bytes[0] = 13,
        SelectorComponent::Empty => bytes[0] = 14,
        SelectorComponent::Scope => bytes[0] = 15,
        SelectorComponent::Nth(value) => {
            bytes[0] = 16;
            encode_nth_data(value, &mut bytes);
        }
        SelectorComponent::NthOf { data, selectors } => {
            bytes[0] = 17;
            encode_nth_data(data, &mut bytes);
            let extra = encode_range_as_extra(selectors);
            let extra_start = match existing_extra {
                Some(extra_start) => {
                    context.set_extra_slot(extra_start, extra);
                    extra_start
                }
                None => context.alloc_extra_slots([extra]),
            };
            return NodePayload::with_extra(
                &bytes[..NodePayload::PARTIAL_INLINE_BYTES],
                extra_start,
            );
        }
        SelectorComponent::PseudoClass(value) => write_node_id(&mut bytes, 18, value),
        SelectorComponent::Slotted(value) => write_node_id(&mut bytes, 19, value),
        SelectorComponent::Part(values) => write_range(&mut bytes, 20, values),
        SelectorComponent::Host(value) => {
            bytes[0] = 21;
            if let Some(value) = value {
                bytes[1] = 1;
                write_id_at(&mut bytes, 4, value);
            }
        }
        SelectorComponent::Where(values) => write_range(&mut bytes, 22, values),
        SelectorComponent::Is(values) => write_range(&mut bytes, 23, values),
        SelectorComponent::Any {
            vendor_prefix,
            selectors,
        } => {
            write_range(&mut bytes, 24, selectors);
            bytes[1] = vendor_prefix.bits();
        }
        SelectorComponent::Has(values) => write_range(&mut bytes, 25, values),
        SelectorComponent::PseudoElement(value) => write_node_id(&mut bytes, 26, value),
        SelectorComponent::Nesting => bytes[0] = 27,
    }
    NodePayload::inline(&bytes)
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

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub struct AttrSelector<'a> {
    pub namespace: Option<NamespaceConstraint<'a>>,
    pub local_name: Atom<'a>,
    pub local_name_lower: Atom<'a>,
    pub operation: AttrOperation<'a>,
    pub never_matches: bool,
}

impl<'ast> AstNodeStorage<'ast> for AttrSelector<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0003);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        let namespace = match bytes[0] {
            0 => None,
            1 => Some(NamespaceConstraint::Any),
            2 => {
                let extra = context.extra_slot(payload.extra_start()).bytes();
                Some(NamespaceConstraint::Specific {
                    prefix: context.resolve_atom(read_u32(&extra, 0) as u64),
                    url: context.resolve_atom(read_u32(&extra, 4) as u64),
                })
            }
            _ => panic!("invalid encoded NamespaceConstraint variant"),
        };
        let operation = match bytes[1] {
            0 => AttrOperation::Exists,
            1 => AttrOperation::WithValue {
                operator: decode_attr_operator(bytes[2]),
                case_sensitivity: decode_case_sensitivity(bytes[3] & 0x7f),
                expected_value: context.resolve_atom(read_u32(
                    &context.extra_slot(payload.extra_start() + 1).bytes(),
                    0,
                ) as u64),
            },
            _ => panic!("invalid encoded AttrOperation variant"),
        };
        Self {
            namespace,
            local_name: context.resolve_atom(read_u32(&bytes, 4) as u64),
            local_name_lower: context.resolve_atom(read_u32(&bytes, 8) as u64),
            operation,
            never_matches: bytes[3] & 0x80 != 0,
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_attr_selector(self, None, context)
    }

    fn encode_existing(self, current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_attr_selector(self, Some(current.extra_start()), context)
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
    let mut bytes = [0; NodePayload::PARTIAL_INLINE_BYTES];
    write_u32(&mut bytes, 4, context.store_atom(value.local_name));
    write_u32(&mut bytes, 8, context.store_atom(value.local_name_lower));
    let mut namespace = [0; ExtraData::BYTES];
    match value.namespace {
        None => bytes[0] = 0,
        Some(NamespaceConstraint::Any) => bytes[0] = 1,
        Some(NamespaceConstraint::Specific { prefix, url }) => {
            bytes[0] = 2;
            write_u32(&mut namespace, 0, context.store_atom(prefix));
            write_u32(&mut namespace, 4, context.store_atom(url));
        }
    }
    let mut operation = [0; ExtraData::BYTES];
    match value.operation {
        AttrOperation::Exists => bytes[1] = 0,
        AttrOperation::WithValue {
            operator,
            case_sensitivity,
            expected_value,
        } => {
            bytes[1] = 1;
            bytes[2] = encode_attr_operator(operator);
            bytes[3] = encode_case_sensitivity(case_sensitivity);
            write_u32(&mut operation, 0, context.store_atom(expected_value));
        }
    }
    bytes[3] |= (value.never_matches as u8) << 7;
    let slots = [
        ExtraData::from_bytes(&namespace),
        ExtraData::from_bytes(&operation),
    ];
    let extra_start = match existing_extra {
        Some(extra_start) => {
            context.set_extra_slot(extra_start, slots[0]);
            context.set_extra_slot(extra_start + 1, slots[1]);
            extra_start
        }
        None => context.alloc_extra_slots(slots),
    };
    NodePayload::with_extra(&bytes, extra_start)
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum NamespaceConstraint<'a> {
    Any,
    Specific { prefix: Atom<'a>, url: Atom<'a> },
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
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

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
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
        name: Atom<'a>,
        arguments: Vec<'a, TokenOrValue<'a>>,
    },
}

impl<'ast> AstNodeStorage<'ast> for PseudoClass<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0004);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        if let Some(value) = decode_unit_pseudo_class(bytes[0]) {
            return value;
        }
        match bytes[0] {
            240 => Self::Lang {
                languages: read_range(&bytes, context),
            },
            241 => Self::Dir {
                direction: decode_direction(bytes[1]),
            },
            242 => Self::Fullscreen(VendorPrefix::from_bits_retain(bytes[1])),
            243 => Self::AnyLink(VendorPrefix::from_bits_retain(bytes[1])),
            244 => Self::ReadOnly(VendorPrefix::from_bits_retain(bytes[1])),
            245 => Self::ReadWrite(VendorPrefix::from_bits_retain(bytes[1])),
            246 => Self::PlaceholderShown(VendorPrefix::from_bits_retain(bytes[1])),
            247 => Self::Autofill(VendorPrefix::from_bits_retain(bytes[1])),
            248 => Self::ActiveViewTransitionType {
                kinds: read_range(&bytes, context),
            },
            249 => Self::State {
                state: context.resolve_atom(read_u32(&bytes, 4) as u64),
            },
            250 => Self::Local {
                selector: read_node_id(&bytes, context),
            },
            251 => Self::Global {
                selector: read_node_id(&bytes, context),
            },
            252 => Self::WebKitScrollbar(decode_webkit_scrollbar_pseudo_class(bytes[1])),
            253 => Self::Custom {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
            },
            254 => Self::CustomFunction {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
                arguments: read_range_at(&bytes, 8, context),
            },
            _ => panic!("invalid encoded PseudoClass variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_pseudo_class(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_pseudo_class(self, context)
    }
}

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
            Self::CustomFunction { name, arguments } => Self::CustomFunction {
                name,
                arguments: context.clone_encoded_vec(arguments),
            },
            value => value,
        }
    }
}

fn encode_pseudo_class<'ast>(
    value: PseudoClass<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    if let Some(tag) = encode_unit_pseudo_class(&value) {
        bytes[0] = tag;
        return NodePayload::inline(&bytes);
    }
    match value {
        PseudoClass::Lang { languages } => write_range(&mut bytes, 240, languages),
        PseudoClass::Dir { direction } => {
            bytes[0] = 241;
            bytes[1] = encode_direction(direction);
        }
        PseudoClass::Fullscreen(value) => write_vendor_prefix(&mut bytes, 242, value),
        PseudoClass::AnyLink(value) => write_vendor_prefix(&mut bytes, 243, value),
        PseudoClass::ReadOnly(value) => write_vendor_prefix(&mut bytes, 244, value),
        PseudoClass::ReadWrite(value) => write_vendor_prefix(&mut bytes, 245, value),
        PseudoClass::PlaceholderShown(value) => write_vendor_prefix(&mut bytes, 246, value),
        PseudoClass::Autofill(value) => write_vendor_prefix(&mut bytes, 247, value),
        PseudoClass::ActiveViewTransitionType { kinds } => write_range(&mut bytes, 248, kinds),
        PseudoClass::State { state } => write_atom(&mut bytes, 249, 4, state, context),
        PseudoClass::Local { selector } => write_node_id(&mut bytes, 250, selector),
        PseudoClass::Global { selector } => write_node_id(&mut bytes, 251, selector),
        PseudoClass::WebKitScrollbar(value) => {
            bytes[0] = 252;
            bytes[1] = encode_webkit_scrollbar_pseudo_class(value);
        }
        PseudoClass::Custom { name } => write_atom(&mut bytes, 253, 4, name, context),
        PseudoClass::CustomFunction { name, arguments } => {
            bytes[0] = 254;
            write_u32(&mut bytes, 4, context.store_atom(name));
            write_range_at(&mut bytes, 8, arguments);
        }
        _ => unreachable!("unit pseudo class handled before data variants"),
    }
    NodePayload::inline(&bytes)
}

macro_rules! unit_pseudo_class_codec {
    ($($tag:literal => $variant:ident),+ $(,)?) => {
        fn encode_unit_pseudo_class(value: &PseudoClass<'_>) -> Option<u8> {
            match value {
                $(PseudoClass::$variant => Some($tag),)+
                _ => None,
            }
        }

        fn decode_unit_pseudo_class<'ast>(tag: u8) -> Option<PseudoClass<'ast>> {
            match tag {
                $($tag => Some(PseudoClass::$variant),)+
                _ => None,
            }
        }
    };
}

unit_pseudo_class_codec! {
    0 => Hover,
    1 => Active,
    2 => Focus,
    3 => FocusVisible,
    4 => FocusWithin,
    5 => Current,
    6 => Past,
    7 => Future,
    8 => Playing,
    9 => Paused,
    10 => Seeking,
    11 => Buffering,
    12 => Stalled,
    13 => Muted,
    14 => VolumeLocked,
    15 => Open,
    16 => Closed,
    17 => Modal,
    18 => PictureInPicture,
    19 => PopoverOpen,
    20 => Defined,
    21 => Link,
    22 => LocalLink,
    23 => Target,
    24 => TargetCurrent,
    25 => TargetBefore,
    26 => TargetAfter,
    27 => TargetWithin,
    28 => Visited,
    29 => Enabled,
    30 => Disabled,
    31 => Default,
    32 => Checked,
    33 => Indeterminate,
    34 => Blank,
    35 => Valid,
    36 => Invalid,
    37 => InRange,
    38 => OutOfRange,
    39 => Required,
    40 => Optional,
    41 => UserValid,
    42 => UserInvalid,
    43 => ActiveViewTransition,
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

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
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
        name: Atom<'a>,
        arguments: Vec<'a, TokenOrValue<'a>>,
    },
}

impl<'ast> AstNodeStorage<'ast> for PseudoElement<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0006);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        if let Some(value) = decode_unit_pseudo_element(bytes[0]) {
            return value;
        }
        match bytes[0] {
            240 => Self::Selection(VendorPrefix::from_bits_retain(bytes[1])),
            241 => Self::Placeholder(VendorPrefix::from_bits_retain(bytes[1])),
            242 => Self::HighlightFunction {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
            },
            243 => Self::Backdrop(VendorPrefix::from_bits_retain(bytes[1])),
            244 => Self::FileSelectorButton(VendorPrefix::from_bits_retain(bytes[1])),
            245 => Self::WebKitScrollbar(decode_webkit_scrollbar_pseudo_element(bytes[1])),
            246 => Self::CueFunction {
                selector: read_node_id(&bytes, context),
            },
            247 => Self::CueRegionFunction {
                selector: read_node_id(&bytes, context),
            },
            248 => Self::ViewTransitionGroup {
                part: read_node_id(&bytes, context),
            },
            249 => Self::ViewTransitionImagePair {
                part: read_node_id(&bytes, context),
            },
            250 => Self::ViewTransitionOld {
                part: read_node_id(&bytes, context),
            },
            251 => Self::ViewTransitionNew {
                part: read_node_id(&bytes, context),
            },
            252 => Self::PickerFunction {
                identifier: context.resolve_atom(read_u32(&bytes, 4) as u64),
            },
            253 => Self::Custom {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
            },
            254 => Self::CustomFunction {
                name: context.resolve_atom(read_u32(&bytes, 4) as u64),
                arguments: read_range_at(&bytes, 8, context),
            },
            _ => panic!("invalid encoded PseudoElement variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_pseudo_element(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_pseudo_element(self, context)
    }
}

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
            Self::CustomFunction { name, arguments } => Self::CustomFunction {
                name,
                arguments: context.clone_encoded_vec(arguments),
            },
            value => value,
        }
    }
}

fn encode_pseudo_element<'ast>(
    value: PseudoElement<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    if let Some(tag) = encode_unit_pseudo_element(&value) {
        bytes[0] = tag;
        return NodePayload::inline(&bytes);
    }
    match value {
        PseudoElement::Selection(value) => write_vendor_prefix(&mut bytes, 240, value),
        PseudoElement::Placeholder(value) => write_vendor_prefix(&mut bytes, 241, value),
        PseudoElement::HighlightFunction { name } => write_atom(&mut bytes, 242, 4, name, context),
        PseudoElement::Backdrop(value) => write_vendor_prefix(&mut bytes, 243, value),
        PseudoElement::FileSelectorButton(value) => write_vendor_prefix(&mut bytes, 244, value),
        PseudoElement::WebKitScrollbar(value) => {
            bytes[0] = 245;
            bytes[1] = encode_webkit_scrollbar_pseudo_element(value);
        }
        PseudoElement::CueFunction { selector } => write_node_id(&mut bytes, 246, selector),
        PseudoElement::CueRegionFunction { selector } => write_node_id(&mut bytes, 247, selector),
        PseudoElement::ViewTransitionGroup { part } => write_node_id(&mut bytes, 248, part),
        PseudoElement::ViewTransitionImagePair { part } => write_node_id(&mut bytes, 249, part),
        PseudoElement::ViewTransitionOld { part } => write_node_id(&mut bytes, 250, part),
        PseudoElement::ViewTransitionNew { part } => write_node_id(&mut bytes, 251, part),
        PseudoElement::PickerFunction { identifier } => {
            write_atom(&mut bytes, 252, 4, identifier, context)
        }
        PseudoElement::Custom { name } => write_atom(&mut bytes, 253, 4, name, context),
        PseudoElement::CustomFunction { name, arguments } => {
            bytes[0] = 254;
            write_u32(&mut bytes, 4, context.store_atom(name));
            write_range_at(&mut bytes, 8, arguments);
        }
        _ => unreachable!("unit pseudo element handled before data variants"),
    }
    NodePayload::inline(&bytes)
}

macro_rules! unit_pseudo_element_codec {
    ($($tag:literal => $variant:ident),+ $(,)?) => {
        fn encode_unit_pseudo_element(value: &PseudoElement<'_>) -> Option<u8> {
            match value {
                $(PseudoElement::$variant => Some($tag),)+
                _ => None,
            }
        }

        fn decode_unit_pseudo_element<'ast>(tag: u8) -> Option<PseudoElement<'ast>> {
            match tag {
                $($tag => Some(PseudoElement::$variant),)+
                _ => None,
            }
        }
    };
}

unit_pseudo_element_codec! {
    0 => After,
    1 => Before,
    2 => FirstLine,
    3 => FirstLetter,
    4 => DetailsContent,
    5 => TargetText,
    6 => SearchText,
    7 => Marker,
    8 => Cue,
    9 => CueRegion,
    10 => ViewTransition,
    11 => PickerIcon,
    12 => Checkmark,
    13 => GrammarError,
    14 => SpellingError,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Visit)]
pub enum ViewTransitionPartName<'a> {
    All,
    Name(Atom<'a>),
}

impl<'ast> AstNodeStorage<'ast> for ViewTransitionPartName<'ast> {
    const KIND: NodeKind = NodeKind::new(0x001b_0007);

    fn decode(payload: NodePayload, context: &AstContext<'ast>) -> Self {
        let bytes = payload.bytes();
        match bytes[0] {
            0 => Self::All,
            1 => Self::Name(context.resolve_atom(read_u32(&bytes, 4) as u64)),
            _ => panic!("invalid encoded ViewTransitionPartName variant"),
        }
    }

    fn encode_new(self, context: &mut AstContext<'ast>) -> NodePayload {
        encode_view_transition_part_name(self, context)
    }

    fn encode_existing(self, _current: NodePayload, context: &mut AstContext<'ast>) -> NodePayload {
        encode_view_transition_part_name(self, context)
    }
}

impl<'ast> AstNodeClone<'ast> for ViewTransitionPartName<'ast> {
    fn clone_in_context(self, _context: &mut AstContext<'ast>) -> Self {
        self
    }
}

fn encode_view_transition_part_name<'ast>(
    value: ViewTransitionPartName<'ast>,
    context: &mut AstContext<'ast>,
) -> NodePayload {
    let mut bytes = [0; NodePayload::INLINE_BYTES];
    match value {
        ViewTransitionPartName::All => bytes[0] = 0,
        ViewTransitionPartName::Name(value) => write_atom(&mut bytes, 1, 4, value, context),
    }
    NodePayload::inline(&bytes)
}

fn encode_combinator(value: Combinator) -> u8 {
    match value {
        Combinator::Child => 0,
        Combinator::Descendant => 1,
        Combinator::NextSibling => 2,
        Combinator::LaterSibling => 3,
        Combinator::PseudoElement => 4,
        Combinator::SlotAssignment => 5,
        Combinator::Part => 6,
        Combinator::DeepDescendant => 7,
        Combinator::Deep => 8,
    }
}

fn decode_combinator(value: u8) -> Combinator {
    match value {
        0 => Combinator::Child,
        1 => Combinator::Descendant,
        2 => Combinator::NextSibling,
        3 => Combinator::LaterSibling,
        4 => Combinator::PseudoElement,
        5 => Combinator::SlotAssignment,
        6 => Combinator::Part,
        7 => Combinator::DeepDescendant,
        8 => Combinator::Deep,
        _ => panic!("invalid encoded Combinator"),
    }
}

fn encode_attr_operator(value: AttrSelectorOperator) -> u8 {
    match value {
        AttrSelectorOperator::Equal => 0,
        AttrSelectorOperator::Includes => 1,
        AttrSelectorOperator::DashMatch => 2,
        AttrSelectorOperator::Prefix => 3,
        AttrSelectorOperator::Substring => 4,
        AttrSelectorOperator::Suffix => 5,
    }
}

fn decode_attr_operator(value: u8) -> AttrSelectorOperator {
    match value {
        0 => AttrSelectorOperator::Equal,
        1 => AttrSelectorOperator::Includes,
        2 => AttrSelectorOperator::DashMatch,
        3 => AttrSelectorOperator::Prefix,
        4 => AttrSelectorOperator::Substring,
        5 => AttrSelectorOperator::Suffix,
        _ => panic!("invalid encoded AttrSelectorOperator"),
    }
}

fn encode_case_sensitivity(value: ParsedCaseSensitivity) -> u8 {
    match value {
        ParsedCaseSensitivity::ExplicitCaseSensitive => 0,
        ParsedCaseSensitivity::AsciiCaseInsensitive => 1,
        ParsedCaseSensitivity::CaseSensitive => 2,
        ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => 3,
    }
}

fn decode_case_sensitivity(value: u8) -> ParsedCaseSensitivity {
    match value {
        0 => ParsedCaseSensitivity::ExplicitCaseSensitive,
        1 => ParsedCaseSensitivity::AsciiCaseInsensitive,
        2 => ParsedCaseSensitivity::CaseSensitive,
        3 => ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
        _ => panic!("invalid encoded ParsedCaseSensitivity"),
    }
}

fn encode_direction(value: Direction) -> u8 {
    match value {
        Direction::Ltr => 0,
        Direction::Rtl => 1,
    }
}

fn decode_direction(value: u8) -> Direction {
    match value {
        0 => Direction::Ltr,
        1 => Direction::Rtl,
        _ => panic!("invalid encoded Direction"),
    }
}

fn encode_webkit_scrollbar_pseudo_class(value: WebKitScrollbarPseudoClass) -> u8 {
    match value {
        WebKitScrollbarPseudoClass::Horizontal => 0,
        WebKitScrollbarPseudoClass::Vertical => 1,
        WebKitScrollbarPseudoClass::Decrement => 2,
        WebKitScrollbarPseudoClass::Increment => 3,
        WebKitScrollbarPseudoClass::Start => 4,
        WebKitScrollbarPseudoClass::End => 5,
        WebKitScrollbarPseudoClass::DoubleButton => 6,
        WebKitScrollbarPseudoClass::SingleButton => 7,
        WebKitScrollbarPseudoClass::NoButton => 8,
        WebKitScrollbarPseudoClass::CornerPresent => 9,
        WebKitScrollbarPseudoClass::WindowInactive => 10,
    }
}

fn decode_webkit_scrollbar_pseudo_class(value: u8) -> WebKitScrollbarPseudoClass {
    match value {
        0 => WebKitScrollbarPseudoClass::Horizontal,
        1 => WebKitScrollbarPseudoClass::Vertical,
        2 => WebKitScrollbarPseudoClass::Decrement,
        3 => WebKitScrollbarPseudoClass::Increment,
        4 => WebKitScrollbarPseudoClass::Start,
        5 => WebKitScrollbarPseudoClass::End,
        6 => WebKitScrollbarPseudoClass::DoubleButton,
        7 => WebKitScrollbarPseudoClass::SingleButton,
        8 => WebKitScrollbarPseudoClass::NoButton,
        9 => WebKitScrollbarPseudoClass::CornerPresent,
        10 => WebKitScrollbarPseudoClass::WindowInactive,
        _ => panic!("invalid encoded WebKitScrollbarPseudoClass"),
    }
}

fn encode_webkit_scrollbar_pseudo_element(value: WebKitScrollbarPseudoElement) -> u8 {
    match value {
        WebKitScrollbarPseudoElement::Scrollbar => 0,
        WebKitScrollbarPseudoElement::Button => 1,
        WebKitScrollbarPseudoElement::Track => 2,
        WebKitScrollbarPseudoElement::TrackPiece => 3,
        WebKitScrollbarPseudoElement::Thumb => 4,
        WebKitScrollbarPseudoElement::Corner => 5,
        WebKitScrollbarPseudoElement::Resizer => 6,
    }
}

fn decode_webkit_scrollbar_pseudo_element(value: u8) -> WebKitScrollbarPseudoElement {
    match value {
        0 => WebKitScrollbarPseudoElement::Scrollbar,
        1 => WebKitScrollbarPseudoElement::Button,
        2 => WebKitScrollbarPseudoElement::Track,
        3 => WebKitScrollbarPseudoElement::TrackPiece,
        4 => WebKitScrollbarPseudoElement::Thumb,
        5 => WebKitScrollbarPseudoElement::Corner,
        6 => WebKitScrollbarPseudoElement::Resizer,
        _ => panic!("invalid encoded WebKitScrollbarPseudoElement"),
    }
}

fn encode_nth_type(value: NthType) -> u8 {
    match value {
        NthType::Child => 0,
        NthType::LastChild => 1,
        NthType::OnlyChild => 2,
        NthType::OfType => 3,
        NthType::LastOfType => 4,
        NthType::OnlyOfType => 5,
        NthType::Col => 6,
        NthType::LastCol => 7,
    }
}

fn decode_nth_type(value: u8) -> NthType {
    match value {
        0 => NthType::Child,
        1 => NthType::LastChild,
        2 => NthType::OnlyChild,
        3 => NthType::OfType,
        4 => NthType::LastOfType,
        5 => NthType::OnlyOfType,
        6 => NthType::Col,
        7 => NthType::LastCol,
        _ => panic!("invalid encoded NthType"),
    }
}

fn encode_nth_data(value: NthSelectorData, bytes: &mut [u8]) {
    bytes[1] = encode_nth_type(value.kind);
    bytes[2] = value.is_function as u8;
    write_u32(bytes, 4, value.a as u32);
    write_u32(bytes, 8, value.b as u32);
}

fn decode_nth_data(bytes: &[u8]) -> NthSelectorData {
    NthSelectorData {
        kind: decode_nth_type(bytes[1]),
        is_function: match bytes[2] {
            0 => false,
            1 => true,
            _ => panic!("invalid encoded NthSelectorData function flag"),
        },
        a: read_u32(bytes, 4) as i32,
        b: read_u32(bytes, 8) as i32,
    }
}

fn write_vendor_prefix(bytes: &mut [u8], tag: u8, value: VendorPrefix) {
    bytes[0] = tag;
    bytes[1] = value.bits();
}

fn write_atom<'ast>(
    bytes: &mut [u8],
    tag: u8,
    offset: usize,
    value: Atom<'ast>,
    context: &mut AstContext<'ast>,
) {
    bytes[0] = tag;
    write_u32(bytes, offset, context.store_atom(value));
}

fn write_node_id<T>(bytes: &mut [u8], tag: u8, value: NodeId<'_, T>) {
    bytes[0] = tag;
    write_id_at(bytes, 4, value);
}

fn write_id_at<T>(bytes: &mut [u8], offset: usize, value: NodeId<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.index()).expect("AST node ID exceeds four bytes"),
    );
}

fn read_node_id<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> NodeId<'ast, T> {
    context.encoded_node_id_at(read_u32(bytes, 4) as usize)
}

fn write_range<T>(bytes: &mut [u8], tag: u8, value: Vec<'_, T>) {
    bytes[0] = tag;
    write_range_at(bytes, 4, value);
}

fn write_range_at<T>(bytes: &mut [u8], offset: usize, value: Vec<'_, T>) {
    write_u32(
        bytes,
        offset,
        u32::try_from(value.start_index()).expect("AST range start exceeds four bytes"),
    );
    write_u32(
        bytes,
        offset + 4,
        u32::try_from(value.end_index()).expect("AST range end exceeds four bytes"),
    );
}

fn read_range<'ast, T>(bytes: &[u8], context: &AstContext<'ast>) -> Vec<'ast, T> {
    read_range_at(bytes, 4, context)
}

fn read_range_at<'ast, T>(bytes: &[u8], offset: usize, context: &AstContext<'ast>) -> Vec<'ast, T> {
    context.encoded_vec_range(
        read_u32(bytes, offset) as usize,
        read_u32(bytes, offset + 4) as usize,
    )
}

fn encode_range_as_extra<T>(value: Vec<'_, T>) -> ExtraData {
    let start = u32::try_from(value.start_index()).expect("AST range start exceeds four bytes");
    let end = u32::try_from(value.end_index()).expect("AST range end exceeds four bytes");
    ExtraData::from_u64((end as u64) << 32 | start as u64)
}

fn decode_extra_range<'ast, T>(data: ExtraData, context: &AstContext<'ast>) -> Vec<'ast, T> {
    let value = data.as_u64();
    context.encoded_vec_range(value as u32 as usize, (value >> 32) as u32 as usize)
}

fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("u32 field"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocketcss_common::{Allocator, StringPool};

    #[test]
    fn selector_clone_recursively_clones_node_graph() {
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let mut strings = StringPool::new_in(&allocator);

        let nested_component = context.alloc_encoded_node(
            SelectorComponent::Class(strings.intern("nested")),
            Span::new(1, 7),
        );
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
        let allocator = Allocator::new();
        let mut context = AstContext::new_in(&allocator);
        let mut strings = StringPool::new_in(&allocator);
        let prefix = strings.intern("svg");
        let url = strings.intern("urn:svg");
        let local_name = strings.intern("href");
        let expected_value = strings.intern("icon");
        let before = context.encoded_extra_len();
        let attribute = context.alloc_encoded_node(
            AttrSelector {
                namespace: Some(NamespaceConstraint::Specific { prefix, url }),
                local_name,
                local_name_lower: local_name,
                operation: AttrOperation::WithValue {
                    operator: AttrSelectorOperator::Prefix,
                    case_sensitivity: ParsedCaseSensitivity::AsciiCaseInsensitive,
                    expected_value,
                },
                never_matches: true,
            },
            DUMMY_SP,
        );
        assert_eq!(context.encoded_extra_len(), before + 2);

        context.mutate_encoded_node(attribute, |attribute, _| {
            attribute.namespace = Some(NamespaceConstraint::Any);
            attribute.operation = AttrOperation::Exists;
            attribute.never_matches = false;
        });

        assert_eq!(context.encoded_extra_len(), before + 2);
        assert_eq!(
            context.encoded_node(attribute),
            AttrSelector {
                namespace: Some(NamespaceConstraint::Any),
                local_name,
                local_name_lower: local_name,
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
        assert_eq!(context.encoded_extra_len(), before + 1);

        context.mutate_encoded_node(component, |component, _| {
            let SelectorComponent::NthOf { data, .. } = component else {
                panic!("expected nth-of component");
            };
            data.b = -1;
        });

        assert_eq!(context.encoded_extra_len(), before + 1);
        assert!(matches!(
            context.encoded_node(component),
            SelectorComponent::NthOf {
                data: NthSelectorData { b: -1, .. },
                ..
            }
        ));
    }
}

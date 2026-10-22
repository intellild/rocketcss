pub type SelectorList<'ast> = &'ast [Selector<'ast>];
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Selector<'ast>(pub(crate) NodeId<'ast, SelectorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectorData<'ast> {
    Parsed(&'ast [SelectorComponent<'ast>]),
    Unparsed(Atom<'ast>),
    Tombstone,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectorKind {
    Parsed,
    Unparsed,
    Tombstone,
}
impl<'ast> NodeStorage<'ast> for Selector<'ast> {
    const KIND: NodeKind = super::kind::KIND_SELECTOR;
    type Data = SelectorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SelectorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SelectorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> Selector<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SelectorData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SelectorKind {
        match ast.data(self) {
            SelectorData::Parsed { .. } => SelectorKind::Parsed,
            SelectorData::Unparsed { .. } => SelectorKind::Unparsed,
            SelectorData::Tombstone { .. } => SelectorKind::Tombstone,
        }
    }
}
impl<'ast> GetSpan<'ast> for Selector<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for Selector<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_selector(&mut self, data: SelectorData<'ast>, span: Span) -> Selector<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SelectorComponent<'ast>(pub(crate) NodeId<'ast, SelectorComponentData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectorComponentData<'ast> {
    Combinator(Combinator),
    ExplicitAnyNamespace,
    ExplicitNoNamespace,
    DefaultNamespace(Atom<'ast>),
    Namespace {
        prefix: Atom<'ast>,
        url: Atom<'ast>,
    },
    ExplicitUniversalType,
    LocalName {
        name: Atom<'ast>,
        lower_name: Atom<'ast>,
    },
    Id(Atom<'ast>),
    Class(Atom<'ast>),
    AttributeInNoNamespaceExists {
        local_name: Atom<'ast>,
        local_name_lower: Atom<'ast>,
    },
    AttributeInNoNamespace {
        local_name: Atom<'ast>,
        operator: AttrSelectorOperator,
        value: Atom<'ast>,
        case_sensitivity: ParsedCaseSensitivity,
        never_matches: bool,
    },
    AttributeOther(AttrSelector<'ast>),
    Negation(&'ast [Selector<'ast>]),
    Root,
    Empty,
    Scope,
    Nth(NthSelectorData<'ast>),
    NthOf {
        data: NthSelectorData<'ast>,
        selectors: &'ast [Selector<'ast>],
    },
    PseudoClass(PseudoClass<'ast>),
    Slotted(Selector<'ast>),
    Part(&'ast [Atom<'ast>]),
    Host(Option<Selector<'ast>>),
    Where(&'ast [Selector<'ast>]),
    Is(&'ast [Selector<'ast>]),
    Any {
        vendor_prefix: VendorPrefix,
        selectors: &'ast [Selector<'ast>],
    },
    Has(&'ast [Selector<'ast>]),
    PseudoElement(PseudoElement<'ast>),
    Nesting,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectorComponentKind {
    Combinator,
    ExplicitAnyNamespace,
    ExplicitNoNamespace,
    DefaultNamespace,
    Namespace,
    ExplicitUniversalType,
    LocalName,
    Id,
    Class,
    AttributeInNoNamespaceExists,
    AttributeInNoNamespace,
    AttributeOther,
    Negation,
    Root,
    Empty,
    Scope,
    Nth,
    NthOf,
    PseudoClass,
    Slotted,
    Part,
    Host,
    Where,
    Is,
    Any,
    Has,
    PseudoElement,
    Nesting,
}
impl<'ast> NodeStorage<'ast> for SelectorComponent<'ast> {
    const KIND: NodeKind = super::kind::KIND_SELECTOR_COMPONENT;
    type Data = SelectorComponentData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, SelectorComponentData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, SelectorComponentData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> SelectorComponent<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> SelectorComponentData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> SelectorComponentKind {
        match ast.data(self) {
            SelectorComponentData::Combinator { .. } => SelectorComponentKind::Combinator,
            SelectorComponentData::ExplicitAnyNamespace { .. } => {
                SelectorComponentKind::ExplicitAnyNamespace
            }
            SelectorComponentData::ExplicitNoNamespace { .. } => {
                SelectorComponentKind::ExplicitNoNamespace
            }
            SelectorComponentData::DefaultNamespace { .. } => {
                SelectorComponentKind::DefaultNamespace
            }
            SelectorComponentData::Namespace { .. } => SelectorComponentKind::Namespace,
            SelectorComponentData::ExplicitUniversalType { .. } => {
                SelectorComponentKind::ExplicitUniversalType
            }
            SelectorComponentData::LocalName { .. } => SelectorComponentKind::LocalName,
            SelectorComponentData::Id { .. } => SelectorComponentKind::Id,
            SelectorComponentData::Class { .. } => SelectorComponentKind::Class,
            SelectorComponentData::AttributeInNoNamespaceExists { .. } => {
                SelectorComponentKind::AttributeInNoNamespaceExists
            }
            SelectorComponentData::AttributeInNoNamespace { .. } => {
                SelectorComponentKind::AttributeInNoNamespace
            }
            SelectorComponentData::AttributeOther { .. } => SelectorComponentKind::AttributeOther,
            SelectorComponentData::Negation { .. } => SelectorComponentKind::Negation,
            SelectorComponentData::Root { .. } => SelectorComponentKind::Root,
            SelectorComponentData::Empty { .. } => SelectorComponentKind::Empty,
            SelectorComponentData::Scope { .. } => SelectorComponentKind::Scope,
            SelectorComponentData::Nth { .. } => SelectorComponentKind::Nth,
            SelectorComponentData::NthOf { .. } => SelectorComponentKind::NthOf,
            SelectorComponentData::PseudoClass { .. } => SelectorComponentKind::PseudoClass,
            SelectorComponentData::Slotted { .. } => SelectorComponentKind::Slotted,
            SelectorComponentData::Part { .. } => SelectorComponentKind::Part,
            SelectorComponentData::Host { .. } => SelectorComponentKind::Host,
            SelectorComponentData::Where { .. } => SelectorComponentKind::Where,
            SelectorComponentData::Is { .. } => SelectorComponentKind::Is,
            SelectorComponentData::Any { .. } => SelectorComponentKind::Any,
            SelectorComponentData::Has { .. } => SelectorComponentKind::Has,
            SelectorComponentData::PseudoElement { .. } => SelectorComponentKind::PseudoElement,
            SelectorComponentData::Nesting { .. } => SelectorComponentKind::Nesting,
        }
    }
}
impl<'ast> GetSpan<'ast> for SelectorComponent<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for SelectorComponent<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_selector_component(
        &mut self,
        data: SelectorComponentData<'ast>,
        span: Span,
    ) -> SelectorComponent<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AttrSelector<'ast>(pub(crate) NodeId<'ast, AttrSelectorData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AttrSelectorData<'ast> {
    pub namespace: Option<NamespaceConstraint<'ast>>,
    pub local_name: Atom<'ast>,
    pub local_name_lower: Atom<'ast>,
    pub operation: AttrOperation<'ast>,
    pub never_matches: bool,
}
impl<'ast> NodeStorage<'ast> for AttrSelector<'ast> {
    const KIND: NodeKind = super::kind::KIND_ATTR_SELECTOR;
    type Data = AttrSelectorData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, AttrSelectorData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, AttrSelectorData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> AttrSelector<'ast> {
    #[inline]
    pub fn namespace(self, ast: &Ast<'ast>) -> Option<NamespaceConstraint<'ast>> {
        ast.data(self).namespace
    }
    #[inline]
    pub fn set_namespace(self, ast: &mut Ast<'ast>, value: Option<NamespaceConstraint<'ast>>) {
        let mut data = ast.data(self);
        data.namespace = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn local_name(self, ast: &Ast<'ast>) -> Atom<'ast> {
        ast.data(self).local_name
    }
    #[inline]
    pub fn set_local_name(self, ast: &mut Ast<'ast>, value: Atom<'ast>) {
        let mut data = ast.data(self);
        data.local_name = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn local_name_lower(self, ast: &Ast<'ast>) -> Atom<'ast> {
        ast.data(self).local_name_lower
    }
    #[inline]
    pub fn set_local_name_lower(self, ast: &mut Ast<'ast>, value: Atom<'ast>) {
        let mut data = ast.data(self);
        data.local_name_lower = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn operation(self, ast: &Ast<'ast>) -> AttrOperation<'ast> {
        ast.data(self).operation
    }
    #[inline]
    pub fn set_operation(self, ast: &mut Ast<'ast>, value: AttrOperation<'ast>) {
        let mut data = ast.data(self);
        data.operation = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn never_matches(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).never_matches
    }
    #[inline]
    pub fn set_never_matches(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.never_matches = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for AttrSelector<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for AttrSelector<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_attr_selector(
        &mut self,
        data: AttrSelectorData<'ast>,
        span: Span,
    ) -> AttrSelector<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NamespaceConstraint<'ast> {
    Any,
    Specific { prefix: Atom<'ast>, url: Atom<'ast> },
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AttrOperation<'ast> {
    Exists,
    WithValue {
        operator: AttrSelectorOperator,
        case_sensitivity: ParsedCaseSensitivity,
        expected_value: Atom<'ast>,
    },
}
use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ParsedCaseSensitivity {
    ExplicitCaseSensitive,
    AsciiCaseInsensitive,
    #[default]
    CaseSensitive,
    AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
}
use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttrSelectorOperator {
    Equal,
    Includes,
    DashMatch,
    Prefix,
    Substring,
    Suffix,
}
use crate::*;
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NthSelectorData<'ast>(pub(crate) NodeId<'ast, NthSelectorDataData>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NthSelectorDataData {
    pub kind: NthType,
    pub is_function: bool,
    pub a: i32,
    pub b: i32,
}
impl<'ast> NodeStorage<'ast> for NthSelectorData<'ast> {
    const KIND: NodeKind = super::kind::KIND_NTH_SELECTOR_DATA;
    type Data = NthSelectorDataData;
    #[inline]
    fn node_id(self) -> NodeId<'ast, NthSelectorDataData> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, NthSelectorDataData>) -> Self {
        Self(id)
    }
}
impl<'ast> NthSelectorData<'ast> {
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> NthType {
        ast.data(self).kind
    }
    #[inline]
    pub fn set_kind(self, ast: &mut Ast<'ast>, value: NthType) {
        let mut data = ast.data(self);
        data.kind = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn is_function(self, ast: &Ast<'ast>) -> bool {
        ast.data(self).is_function
    }
    #[inline]
    pub fn set_is_function(self, ast: &mut Ast<'ast>, value: bool) {
        let mut data = ast.data(self);
        data.is_function = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn a(self, ast: &Ast<'ast>) -> i32 {
        ast.data(self).a
    }
    #[inline]
    pub fn set_a(self, ast: &mut Ast<'ast>, value: i32) {
        let mut data = ast.data(self);
        data.a = value;
        ast.set_data(self, data);
    }
    #[inline]
    pub fn b(self, ast: &Ast<'ast>) -> i32 {
        ast.data(self).b
    }
    #[inline]
    pub fn set_b(self, ast: &mut Ast<'ast>, value: i32) {
        let mut data = ast.data(self);
        data.b = value;
        ast.set_data(self, data);
    }
}
impl<'ast> GetSpan<'ast> for NthSelectorData<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for NthSelectorData<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_nth_selector_data(
        &mut self,
        data: NthSelectorDataData,
        span: Span,
    ) -> NthSelectorData<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(CssKeyword, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Ltr,
    Rtl,
}
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PseudoClass<'ast>(pub(crate) NodeId<'ast, PseudoClassData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PseudoClassData<'ast> {
    Lang {
        languages: &'ast [Atom<'ast>],
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
        kinds: &'ast [Atom<'ast>],
    },
    State {
        state: Atom<'ast>,
    },
    Local {
        selector: Selector<'ast>,
    },
    Global {
        selector: Selector<'ast>,
    },
    WebKitScrollbar(WebKitScrollbarPseudoClass),
    Custom {
        name: Atom<'ast>,
    },
    CustomFunction {
        name: Atom<'ast>,
        arguments: &'ast [TokenOrValue<'ast>],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PseudoClassKind {
    Lang,
    Dir,
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
    Fullscreen,
    Open,
    Closed,
    Modal,
    PictureInPicture,
    PopoverOpen,
    Defined,
    AnyLink,
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
    ReadOnly,
    ReadWrite,
    PlaceholderShown,
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
    Autofill,
    ActiveViewTransition,
    ActiveViewTransitionType,
    State,
    Local,
    Global,
    WebKitScrollbar,
    Custom,
    CustomFunction,
}
impl<'ast> NodeStorage<'ast> for PseudoClass<'ast> {
    const KIND: NodeKind = super::kind::KIND_PSEUDO_CLASS;
    type Data = PseudoClassData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PseudoClassData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PseudoClassData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PseudoClass<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> PseudoClassData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> PseudoClassKind {
        match ast.data(self) {
            PseudoClassData::Lang { .. } => PseudoClassKind::Lang,
            PseudoClassData::Dir { .. } => PseudoClassKind::Dir,
            PseudoClassData::Hover { .. } => PseudoClassKind::Hover,
            PseudoClassData::Active { .. } => PseudoClassKind::Active,
            PseudoClassData::Focus { .. } => PseudoClassKind::Focus,
            PseudoClassData::FocusVisible { .. } => PseudoClassKind::FocusVisible,
            PseudoClassData::FocusWithin { .. } => PseudoClassKind::FocusWithin,
            PseudoClassData::Current { .. } => PseudoClassKind::Current,
            PseudoClassData::Past { .. } => PseudoClassKind::Past,
            PseudoClassData::Future { .. } => PseudoClassKind::Future,
            PseudoClassData::Playing { .. } => PseudoClassKind::Playing,
            PseudoClassData::Paused { .. } => PseudoClassKind::Paused,
            PseudoClassData::Seeking { .. } => PseudoClassKind::Seeking,
            PseudoClassData::Buffering { .. } => PseudoClassKind::Buffering,
            PseudoClassData::Stalled { .. } => PseudoClassKind::Stalled,
            PseudoClassData::Muted { .. } => PseudoClassKind::Muted,
            PseudoClassData::VolumeLocked { .. } => PseudoClassKind::VolumeLocked,
            PseudoClassData::Fullscreen { .. } => PseudoClassKind::Fullscreen,
            PseudoClassData::Open { .. } => PseudoClassKind::Open,
            PseudoClassData::Closed { .. } => PseudoClassKind::Closed,
            PseudoClassData::Modal { .. } => PseudoClassKind::Modal,
            PseudoClassData::PictureInPicture { .. } => PseudoClassKind::PictureInPicture,
            PseudoClassData::PopoverOpen { .. } => PseudoClassKind::PopoverOpen,
            PseudoClassData::Defined { .. } => PseudoClassKind::Defined,
            PseudoClassData::AnyLink { .. } => PseudoClassKind::AnyLink,
            PseudoClassData::Link { .. } => PseudoClassKind::Link,
            PseudoClassData::LocalLink { .. } => PseudoClassKind::LocalLink,
            PseudoClassData::Target { .. } => PseudoClassKind::Target,
            PseudoClassData::TargetCurrent { .. } => PseudoClassKind::TargetCurrent,
            PseudoClassData::TargetBefore { .. } => PseudoClassKind::TargetBefore,
            PseudoClassData::TargetAfter { .. } => PseudoClassKind::TargetAfter,
            PseudoClassData::TargetWithin { .. } => PseudoClassKind::TargetWithin,
            PseudoClassData::Visited { .. } => PseudoClassKind::Visited,
            PseudoClassData::Enabled { .. } => PseudoClassKind::Enabled,
            PseudoClassData::Disabled { .. } => PseudoClassKind::Disabled,
            PseudoClassData::ReadOnly { .. } => PseudoClassKind::ReadOnly,
            PseudoClassData::ReadWrite { .. } => PseudoClassKind::ReadWrite,
            PseudoClassData::PlaceholderShown { .. } => PseudoClassKind::PlaceholderShown,
            PseudoClassData::Default { .. } => PseudoClassKind::Default,
            PseudoClassData::Checked { .. } => PseudoClassKind::Checked,
            PseudoClassData::Indeterminate { .. } => PseudoClassKind::Indeterminate,
            PseudoClassData::Blank { .. } => PseudoClassKind::Blank,
            PseudoClassData::Valid { .. } => PseudoClassKind::Valid,
            PseudoClassData::Invalid { .. } => PseudoClassKind::Invalid,
            PseudoClassData::InRange { .. } => PseudoClassKind::InRange,
            PseudoClassData::OutOfRange { .. } => PseudoClassKind::OutOfRange,
            PseudoClassData::Required { .. } => PseudoClassKind::Required,
            PseudoClassData::Optional { .. } => PseudoClassKind::Optional,
            PseudoClassData::UserValid { .. } => PseudoClassKind::UserValid,
            PseudoClassData::UserInvalid { .. } => PseudoClassKind::UserInvalid,
            PseudoClassData::Autofill { .. } => PseudoClassKind::Autofill,
            PseudoClassData::ActiveViewTransition { .. } => PseudoClassKind::ActiveViewTransition,
            PseudoClassData::ActiveViewTransitionType { .. } => {
                PseudoClassKind::ActiveViewTransitionType
            }
            PseudoClassData::State { .. } => PseudoClassKind::State,
            PseudoClassData::Local { .. } => PseudoClassKind::Local,
            PseudoClassData::Global { .. } => PseudoClassKind::Global,
            PseudoClassData::WebKitScrollbar { .. } => PseudoClassKind::WebKitScrollbar,
            PseudoClassData::Custom { .. } => PseudoClassKind::Custom,
            PseudoClassData::CustomFunction { .. } => PseudoClassKind::CustomFunction,
        }
    }
}
impl<'ast> GetSpan<'ast> for PseudoClass<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PseudoClass<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_pseudo_class(
        &mut self,
        data: PseudoClassData<'ast>,
        span: Span,
    ) -> PseudoClass<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
use crate::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PseudoElement<'ast>(pub(crate) NodeId<'ast, PseudoElementData<'ast>>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PseudoElementData<'ast> {
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
        name: Atom<'ast>,
    },
    Marker,
    Backdrop(VendorPrefix),
    FileSelectorButton(VendorPrefix),
    WebKitScrollbar(WebKitScrollbarPseudoElement),
    Cue,
    CueRegion,
    CueFunction {
        selector: Selector<'ast>,
    },
    CueRegionFunction {
        selector: Selector<'ast>,
    },
    ViewTransition,
    ViewTransitionGroup {
        part: ViewTransitionPartSelector<'ast>,
    },
    ViewTransitionImagePair {
        part: ViewTransitionPartSelector<'ast>,
    },
    ViewTransitionOld {
        part: ViewTransitionPartSelector<'ast>,
    },
    ViewTransitionNew {
        part: ViewTransitionPartSelector<'ast>,
    },
    PickerFunction {
        identifier: Atom<'ast>,
    },
    PickerIcon,
    Checkmark,
    GrammarError,
    SpellingError,
    Custom {
        name: Atom<'ast>,
    },
    CustomFunction {
        name: Atom<'ast>,
        arguments: &'ast [TokenOrValue<'ast>],
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PseudoElementKind {
    After,
    Before,
    FirstLine,
    FirstLetter,
    DetailsContent,
    TargetText,
    SearchText,
    Selection,
    Placeholder,
    HighlightFunction,
    Marker,
    Backdrop,
    FileSelectorButton,
    WebKitScrollbar,
    Cue,
    CueRegion,
    CueFunction,
    CueRegionFunction,
    ViewTransition,
    ViewTransitionGroup,
    ViewTransitionImagePair,
    ViewTransitionOld,
    ViewTransitionNew,
    PickerFunction,
    PickerIcon,
    Checkmark,
    GrammarError,
    SpellingError,
    Custom,
    CustomFunction,
}
impl<'ast> NodeStorage<'ast> for PseudoElement<'ast> {
    const KIND: NodeKind = super::kind::KIND_PSEUDO_ELEMENT;
    type Data = PseudoElementData<'ast>;
    #[inline]
    fn node_id(self) -> NodeId<'ast, PseudoElementData<'ast>> {
        self.0
    }
    #[inline]
    unsafe fn from_node_id(id: NodeId<'ast, PseudoElementData<'ast>>) -> Self {
        Self(id)
    }
}
impl<'ast> PseudoElement<'ast> {
    #[inline]
    pub fn data(self, ast: &Ast<'ast>) -> PseudoElementData<'ast> {
        ast.data(self)
    }
    #[inline]
    pub fn kind(self, ast: &Ast<'ast>) -> PseudoElementKind {
        match ast.data(self) {
            PseudoElementData::After { .. } => PseudoElementKind::After,
            PseudoElementData::Before { .. } => PseudoElementKind::Before,
            PseudoElementData::FirstLine { .. } => PseudoElementKind::FirstLine,
            PseudoElementData::FirstLetter { .. } => PseudoElementKind::FirstLetter,
            PseudoElementData::DetailsContent { .. } => PseudoElementKind::DetailsContent,
            PseudoElementData::TargetText { .. } => PseudoElementKind::TargetText,
            PseudoElementData::SearchText { .. } => PseudoElementKind::SearchText,
            PseudoElementData::Selection { .. } => PseudoElementKind::Selection,
            PseudoElementData::Placeholder { .. } => PseudoElementKind::Placeholder,
            PseudoElementData::HighlightFunction { .. } => PseudoElementKind::HighlightFunction,
            PseudoElementData::Marker { .. } => PseudoElementKind::Marker,
            PseudoElementData::Backdrop { .. } => PseudoElementKind::Backdrop,
            PseudoElementData::FileSelectorButton { .. } => PseudoElementKind::FileSelectorButton,
            PseudoElementData::WebKitScrollbar { .. } => PseudoElementKind::WebKitScrollbar,
            PseudoElementData::Cue { .. } => PseudoElementKind::Cue,
            PseudoElementData::CueRegion { .. } => PseudoElementKind::CueRegion,
            PseudoElementData::CueFunction { .. } => PseudoElementKind::CueFunction,
            PseudoElementData::CueRegionFunction { .. } => PseudoElementKind::CueRegionFunction,
            PseudoElementData::ViewTransition { .. } => PseudoElementKind::ViewTransition,
            PseudoElementData::ViewTransitionGroup { .. } => PseudoElementKind::ViewTransitionGroup,
            PseudoElementData::ViewTransitionImagePair { .. } => {
                PseudoElementKind::ViewTransitionImagePair
            }
            PseudoElementData::ViewTransitionOld { .. } => PseudoElementKind::ViewTransitionOld,
            PseudoElementData::ViewTransitionNew { .. } => PseudoElementKind::ViewTransitionNew,
            PseudoElementData::PickerFunction { .. } => PseudoElementKind::PickerFunction,
            PseudoElementData::PickerIcon { .. } => PseudoElementKind::PickerIcon,
            PseudoElementData::Checkmark { .. } => PseudoElementKind::Checkmark,
            PseudoElementData::GrammarError { .. } => PseudoElementKind::GrammarError,
            PseudoElementData::SpellingError { .. } => PseudoElementKind::SpellingError,
            PseudoElementData::Custom { .. } => PseudoElementKind::Custom,
            PseudoElementData::CustomFunction { .. } => PseudoElementKind::CustomFunction,
        }
    }
}
impl<'ast> GetSpan<'ast> for PseudoElement<'ast> {
    #[inline]
    fn span(self, ast: &Ast<'ast>) -> Span {
        ast.span(self)
    }
}
impl<'ast> SetSpan<'ast> for PseudoElement<'ast> {
    #[inline]
    fn set_span(self, ast: &mut Ast<'ast>, span: Span) {
        ast.set_span(self, span);
    }
}
impl<'ast> Ast<'ast> {
    #[inline]
    pub fn alloc_pseudo_element(
        &mut self,
        data: PseudoElementData<'ast>,
        span: Span,
    ) -> PseudoElement<'ast> {
        self.alloc(data, span)
    }
}
use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebKitScrollbarPseudoElement {
    Scrollbar,
    Button,
    Track,
    TrackPiece,
    Thumb,
    Corner,
    Resizer,
}
use crate::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewTransitionPartName<'ast> {
    All,
    Name(Atom<'ast>),
}

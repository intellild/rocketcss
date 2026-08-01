use super::*;

/// A selector list in source order.
pub type SelectorList<'a> = Vec<'a, Selector<'a>>;

/// A complex selector, a losslessly preserved invalid selector, or a removed selector.
#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum Selector<'a> {
    /// A valid selector. Components are stored in parse order.
    Parsed(Vec<'a, SelectorComponent<'a>>),
    /// An invalid selector preserved by parser error recovery.
    #[visit(skip)]
    Unparsed(Atom<'a>),
    /// A selector removed by a transformation.
    Tombstone,
}

impl<'a> Selector<'a> {
    #[inline]
    pub fn parsed(components: Vec<'a, SelectorComponent<'a>>) -> Self {
        Self::Parsed(components)
    }

    #[inline]
    pub fn as_parsed(&self) -> Option<&Vec<'a, SelectorComponent<'a>>> {
        match self {
            Self::Parsed(components) => Some(components),
            Self::Unparsed(_) | Self::Tombstone => None,
        }
    }

    #[inline]
    pub fn as_parsed_mut(&mut self) -> Option<&mut Vec<'a, SelectorComponent<'a>>> {
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

impl<'a> std::ops::Deref for Selector<'a> {
    type Target = [SelectorComponent<'a>];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Parsed(components) => components,
            Self::Unparsed(_) | Self::Tombstone => &[],
        }
    }
}

impl std::ops::DerefMut for Selector<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Parsed(components) => components,
            Self::Unparsed(_) | Self::Tombstone => {
                panic!("only parsed selectors expose mutable components")
            }
        }
    }
}

/// A CSS simple selector or combinator.
///
/// This mirrors `parcel_selectors::parser::Component`, specialized for
/// lightningcss' selector implementation and arena-backed containers.
#[derive(Debug, PartialEq, Eq, Hash, Visit)]
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
    AttributeOther(Box<'a, AttrSelector<'a>>),

    Negation(Vec<'a, Selector<'a>>),
    Root,
    Empty,
    Scope,
    Nth(NthSelectorData),
    NthOf {
        data: NthSelectorData,
        selectors: Vec<'a, Selector<'a>>,
    },
    PseudoClass(Box<'a, PseudoClass<'a>>),
    Slotted(Box<'a, Selector<'a>>),
    Part(Vec<'a, Atom<'a>>),
    Host(Option<Box<'a, Selector<'a>>>),
    Where(Vec<'a, Selector<'a>>),
    Is(Vec<'a, Selector<'a>>),
    Any {
        vendor_prefix: VendorPrefix,
        selectors: Vec<'a, Selector<'a>>,
    },
    Has(Vec<'a, Selector<'a>>),
    PseudoElement(Box<'a, PseudoElement<'a>>),
    Nesting,
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
        selector: Box<'a, Selector<'a>>,
    },
    Global {
        selector: Box<'a, Selector<'a>>,
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
        selector: Box<'a, Selector<'a>>,
    },
    CueRegionFunction {
        selector: Box<'a, Selector<'a>>,
    },
    ViewTransition,
    ViewTransitionGroup {
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionImagePair {
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionOld {
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionNew {
        part: Box<'a, ViewTransitionPartSelector<'a>>,
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

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum ViewTransitionPartName<'a> {
    All,
    Name(Atom<'a>),
}

use super::*;

/// A selector list in source order.
pub type SelectorList<'a> = Vec<'a, Selector<'a>>;

/// A complex selector. Components are stored in parse order.
pub type Selector<'a> = Vec<'a, SelectorComponent<'a>>;

/// A CSS simple selector or combinator.
///
/// This mirrors `parcel_selectors::parser::Component`, specialized for
/// lightningcss' selector implementation and arena-backed containers.
#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum SelectorComponent<'a> {
    Combinator(Combinator),

    ExplicitAnyNamespace,
    ExplicitNoNamespace,
    DefaultNamespace(&'a str),
    Namespace {
        prefix: &'a str,
        url: &'a str,
    },

    ExplicitUniversalType,
    LocalName {
        name: &'a str,
        lower_name: &'a str,
    },

    Id(&'a str),
    Class(&'a str),

    AttributeInNoNamespaceExists {
        local_name: &'a str,
        local_name_lower: &'a str,
    },
    AttributeInNoNamespace {
        local_name: &'a str,
        operator: AttrSelectorOperator,
        value: &'a str,
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
    Part(Vec<'a, &'a str>),
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
    pub local_name: &'a str,
    pub local_name_lower: &'a str,
    pub operation: AttrOperation<'a>,
    pub never_matches: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum NamespaceConstraint<'a> {
    Any,
    Specific { prefix: &'a str, url: &'a str },
}

#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum AttrOperation<'a> {
    Exists,
    WithValue {
        operator: AttrSelectorOperator,
        case_sensitivity: ParsedCaseSensitivity,
        expected_value: &'a str,
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
        languages: Vec<'a, &'a str>,
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
        kinds: Vec<'a, &'a str>,
    },
    State {
        state: &'a str,
    },
    Local {
        selector: Box<'a, Selector<'a>>,
    },
    Global {
        selector: Box<'a, Selector<'a>>,
    },
    WebKitScrollbar(WebKitScrollbarPseudoClass),
    Custom {
        name: &'a str,
    },
    CustomFunction {
        name: &'a str,
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
        name: &'a str,
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
        identifier: &'a str,
    },
    PickerIcon,
    Checkmark,
    GrammarError,
    SpellingError,
    Custom {
        name: &'a str,
    },
    CustomFunction {
        name: &'a str,
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
    Name(&'a str),
}

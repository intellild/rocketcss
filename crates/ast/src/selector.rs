use super::*;
use rocketcss_common::{DenseRange, DenseStore, define_dense_id};

/// A selector list in source order.
pub type SelectorList<'a> = std::vec::Vec<Selector<'a>>;

define_dense_id!(pub struct SelectorId);
define_dense_id!(pub struct SelectorListId);

#[derive(Debug, Default, PartialEq, Visit)]
#[visit(skip)]
pub struct SelectorStore<'a> {
    #[visit(skip)]
    selectors: DenseStore<SelectorId, Selector<'a>>,
    #[visit(skip)]
    lists: DenseStore<SelectorListId, DenseRange<SelectorId>>,
}

impl<'a> SelectorStore<'a> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            selectors: DenseStore::new(),
            lists: DenseStore::new(),
        }
    }

    #[inline]
    pub fn push_list(&mut self, selectors: SelectorList<'a>) -> SelectorListId {
        let cursor = self.selectors.cursor();
        for selector in selectors {
            self.selectors.push(selector);
        }
        self.lists.push(self.selectors.range_since(cursor))
    }

    #[inline]
    pub fn get(&self, id: SelectorListId) -> &[Selector<'a>] {
        self.selectors.get_range(self.lists[id])
    }

    #[inline]
    pub fn get_mut(&mut self, id: SelectorListId) -> &mut [Selector<'a>] {
        self.selectors.get_range_mut(self.lists[id])
    }

    #[inline]
    pub fn slots(&self) -> impl ExactSizeIterator<Item = (SelectorId, &Selector<'a>)> {
        self.selectors.iter_enumerated()
    }

    #[inline]
    pub fn range(&self, id: SelectorListId) -> DenseRange<SelectorId> {
        self.lists[id]
    }

    pub fn compact(&mut self) {
        let mut source = std::mem::take(&mut self.selectors);
        let old_lists = std::mem::take(&mut self.lists);
        let mut selectors = DenseStore::new();
        let mut lists = DenseStore::with_capacity(old_lists.len());
        for range in old_lists.iter().copied() {
            let cursor = selectors.cursor();
            for index in range.as_usize_range() {
                let id = SelectorId::from_index(index)
                    .expect("a selector range contains valid dense IDs");
                let selector = std::mem::replace(source.get_mut(id), Selector::Tombstone);
                if !selector.is_tombstone() {
                    selectors.push(selector);
                }
            }
            lists.push(selectors.range_since(cursor));
        }
        self.selectors = selectors;
        self.lists = lists;
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        let mut owners = std::vec![false; self.selectors.len()];
        for range in self.lists.iter().copied() {
            if range.end() > self.selectors.len() {
                return Err("selector list range exceeds the selector tape");
            }
            for owned in &mut owners[range.as_usize_range()] {
                if std::mem::replace(owned, true) {
                    return Err("a selector occurrence belongs to multiple lists");
                }
            }
        }
        if owners.iter().any(|owned| !owned) {
            return Err("a selector occurrence has no list owner");
        }
        Ok(())
    }
}

/// A complex selector, a losslessly preserved invalid selector, or a removed selector.
#[derive(Debug, PartialEq, Eq, Hash, Visit)]
pub enum Selector<'a> {
    /// A valid selector. Components are stored in parse order.
    Parsed(std::vec::Vec<SelectorComponent<'a>>),
    /// An invalid selector preserved by parser error recovery.
    #[visit(skip)]
    Unparsed(Atom<'a>),
    /// A selector removed by a transformation.
    Tombstone,
}

impl<'a> Selector<'a> {
    #[inline]
    pub fn parsed(components: std::vec::Vec<SelectorComponent<'a>>) -> Self {
        Self::Parsed(components)
    }

    #[inline]
    pub fn as_parsed(&self) -> Option<&std::vec::Vec<SelectorComponent<'a>>> {
        match self {
            Self::Parsed(components) => Some(components),
            Self::Unparsed(_) | Self::Tombstone => None,
        }
    }

    #[inline]
    pub fn as_parsed_mut(&mut self) -> Option<&mut std::vec::Vec<SelectorComponent<'a>>> {
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
/// lightningcss' selector implementation and owned containers.
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
    AttributeOther(std::boxed::Box<AttrSelector<'a>>),

    Negation(std::vec::Vec<Selector<'a>>),
    Root,
    Empty,
    Scope,
    Nth(NthSelectorData),
    NthOf {
        data: NthSelectorData,
        selectors: std::vec::Vec<Selector<'a>>,
    },
    PseudoClass(std::boxed::Box<PseudoClass<'a>>),
    Slotted(std::boxed::Box<Selector<'a>>),
    Part(std::vec::Vec<Atom<'a>>),
    Host(Option<std::boxed::Box<Selector<'a>>>),
    Where(std::vec::Vec<Selector<'a>>),
    Is(std::vec::Vec<Selector<'a>>),
    Any {
        vendor_prefix: VendorPrefix,
        selectors: std::vec::Vec<Selector<'a>>,
    },
    Has(std::vec::Vec<Selector<'a>>),
    PseudoElement(std::boxed::Box<PseudoElement<'a>>),
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
        languages: std::vec::Vec<Atom<'a>>,
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
        kinds: std::vec::Vec<Atom<'a>>,
    },
    State {
        state: Atom<'a>,
    },
    Local {
        selector: std::boxed::Box<Selector<'a>>,
    },
    Global {
        selector: std::boxed::Box<Selector<'a>>,
    },
    WebKitScrollbar(WebKitScrollbarPseudoClass),
    Custom {
        name: Atom<'a>,
    },
    CustomFunction {
        name: Atom<'a>,
        arguments: std::vec::Vec<TokenOrValue<'a>>,
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
        selector: std::boxed::Box<Selector<'a>>,
    },
    CueRegionFunction {
        selector: std::boxed::Box<Selector<'a>>,
    },
    ViewTransition,
    ViewTransitionGroup {
        part: std::boxed::Box<ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionImagePair {
        part: std::boxed::Box<ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionOld {
        part: std::boxed::Box<ViewTransitionPartSelector<'a>>,
    },
    ViewTransitionNew {
        part: std::boxed::Box<ViewTransitionPartSelector<'a>>,
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
        arguments: std::vec::Vec<TokenOrValue<'a>>,
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

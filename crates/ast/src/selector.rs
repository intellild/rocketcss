use super::*;

#[derive(Debug, PartialEq)]
pub enum SelectorComponent<'a> {
    Combinator {
        value: Box<'a, Combinator<'a>>,
    },
    Universal,
    Value(()),
    Type {
        name: &'a str,
    },
    Id {
        name: &'a str,
    },
    Class {
        name: &'a str,
    },
    Attribute {
        name: &'a str,
        namespace: Option<Box<'a, NamespaceConstraint<'a>>>,
        operation: Option<Box<'a, AttrOperation<'a>>>,
    },
    Value2(()),
    Value3(()),
    Nesting,
}

#[derive(Debug, PartialEq)]
pub enum Combinator<'a> {
    Value(&'a str),
    PseudoElement,
    SlotAssignment,
    Part,
    DeepDescendant,
    Deep,
}

#[derive(Debug, PartialEq)]
pub enum NamespaceConstraint<'a> {
    Any,
    Specific { prefix: &'a str, url: &'a str },
}

#[derive(Debug, PartialEq)]
pub enum ParsedCaseSensitivity {
    ExplicitCaseSensitive,
    AsciiCaseInsensitive,
    CaseSensitive,
    AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument,
}

#[derive(Debug, PartialEq)]
pub enum AttrSelectorOperator {
    Equal,
    Includes,
    DashMatch,
    Prefix,
    Substring,
    Suffix,
}

#[derive(Debug, PartialEq)]
pub enum TSPseudoClass<'a> {
    Object {
        kind: &'a str,
        selectors: Vec<'a, Box<'a, Selector<'a>>>,
    },
    Object2 {
        kind: &'a str,
    },
    Object3 {
        kind: &'a str,
    },
    Object4 {
        kind: &'a str,
    },
    Object5 {
        kind: &'a str,
    },
    Object6 {
        kind: &'a str,
    },
    Object7 {
        kind: &'a str,
    },
    Object8 {
        a: f64,
        b: f64,
        kind: &'a str,
        of: Option<Vec<'a, Box<'a, Selector<'a>>>>,
    },
    Object9 {
        a: f64,
        b: f64,
        kind: &'a str,
        of: Option<Vec<'a, Box<'a, Selector<'a>>>>,
    },
    Object10 {
        a: f64,
        b: f64,
        kind: &'a str,
    },
    Object11 {
        a: f64,
        b: f64,
        kind: &'a str,
    },
    Object12 {
        a: f64,
        b: f64,
        kind: &'a str,
    },
    Object13 {
        a: f64,
        b: f64,
        kind: &'a str,
    },
    Object14 {
        kind: &'a str,
    },
    Object15 {
        kind: &'a str,
    },
    Object16 {
        kind: &'a str,
    },
    Object17 {
        kind: &'a str,
        selectors: Option<Box<'a, Selector<'a>>>,
    },
    Object18 {
        kind: &'a str,
        selectors: Vec<'a, Box<'a, Selector<'a>>>,
    },
    Object19 {
        kind: &'a str,
        selectors: Vec<'a, Box<'a, Selector<'a>>>,
    },
    Object20 {
        kind: &'a str,
        selectors: Vec<'a, Box<'a, Selector<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object21 {
        kind: &'a str,
        selectors: Vec<'a, Box<'a, Selector<'a>>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum PseudoClass<'a> {
    Object {
        kind: &'a str,
        languages: Vec<'a, &'a str>,
    },
    Object2 {
        direction: Box<'a, Direction>,
        kind: &'a str,
    },
    Object3 {
        kind: &'a str,
    },
    Object4 {
        kind: &'a str,
    },
    Object5 {
        kind: &'a str,
    },
    Object6 {
        kind: &'a str,
    },
    Object7 {
        kind: &'a str,
    },
    Object8 {
        kind: &'a str,
    },
    Object9 {
        kind: &'a str,
    },
    Object10 {
        kind: &'a str,
    },
    Object11 {
        kind: &'a str,
    },
    Object12 {
        kind: &'a str,
    },
    Object13 {
        kind: &'a str,
    },
    Object14 {
        kind: &'a str,
    },
    Object15 {
        kind: &'a str,
    },
    Object16 {
        kind: &'a str,
    },
    Object17 {
        kind: &'a str,
    },
    Object18 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object19 {
        kind: &'a str,
    },
    Object20 {
        kind: &'a str,
    },
    Object21 {
        kind: &'a str,
    },
    Object22 {
        kind: &'a str,
    },
    Object23 {
        kind: &'a str,
    },
    Object24 {
        kind: &'a str,
    },
    Object25 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object26 {
        kind: &'a str,
    },
    Object27 {
        kind: &'a str,
    },
    Object28 {
        kind: &'a str,
    },
    Object29 {
        kind: &'a str,
    },
    Object30 {
        kind: &'a str,
    },
    Object31 {
        kind: &'a str,
    },
    Object32 {
        kind: &'a str,
    },
    Object33 {
        kind: &'a str,
    },
    Object34 {
        kind: &'a str,
    },
    Object35 {
        kind: &'a str,
    },
    Object36 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object37 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object38 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object39 {
        kind: &'a str,
    },
    Object40 {
        kind: &'a str,
    },
    Object41 {
        kind: &'a str,
    },
    Object42 {
        kind: &'a str,
    },
    Object43 {
        kind: &'a str,
    },
    Object44 {
        kind: &'a str,
    },
    Object45 {
        kind: &'a str,
    },
    Object46 {
        kind: &'a str,
    },
    Object47 {
        kind: &'a str,
    },
    Object48 {
        kind: &'a str,
    },
    Object49 {
        kind: &'a str,
    },
    Object50 {
        kind: &'a str,
    },
    Object51 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object52 {
        kind: &'a str,
    },
    Object53 {
        kind: &'a str,
    },
    Object54 {
        kind: &'a str,
        state: &'a str,
    },
    Object55 {
        kind: &'a str,
        selector: Box<'a, Selector<'a>>,
    },
    Object56 {
        kind: &'a str,
        selector: Box<'a, Selector<'a>>,
    },
    Object57 {
        kind: &'a str,
        value: Box<'a, WebKitScrollbarPseudoClass>,
    },
    Object58 {
        kind: &'a str,
        name: &'a str,
    },
    Object59 {
        arguments: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        kind: &'a str,
        name: &'a str,
    },
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum BuiltinPseudoElement<'a> {
    Object {
        kind: &'a str,
        selector: Box<'a, Selector<'a>>,
    },
    Object2 {
        kind: &'a str,
        names: Vec<'a, &'a str>,
    },
}

#[derive(Debug, PartialEq)]
pub enum PseudoElement<'a> {
    Object {
        kind: &'a str,
    },
    Object2 {
        kind: &'a str,
    },
    Object3 {
        kind: &'a str,
    },
    Object4 {
        kind: &'a str,
    },
    Object5 {
        kind: &'a str,
    },
    Object6 {
        kind: &'a str,
    },
    Object7 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object8 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object9 {
        kind: &'a str,
    },
    Object10 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object11 {
        kind: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object12 {
        kind: &'a str,
        value: Box<'a, WebKitScrollbarPseudoElement>,
    },
    Object13 {
        kind: &'a str,
    },
    Object14 {
        kind: &'a str,
    },
    Object15 {
        kind: &'a str,
        selector: Box<'a, Selector<'a>>,
    },
    Object16 {
        kind: &'a str,
        selector: Box<'a, Selector<'a>>,
    },
    Object17 {
        kind: &'a str,
    },
    Object18 {
        kind: &'a str,
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    Object19 {
        kind: &'a str,
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    Object20 {
        kind: &'a str,
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    Object21 {
        kind: &'a str,
        part: Box<'a, ViewTransitionPartSelector<'a>>,
    },
    Object22 {
        identifier: &'a str,
        kind: &'a str,
    },
    Object23 {
        kind: &'a str,
    },
    Object24 {
        kind: &'a str,
    },
    Object25 {
        kind: &'a str,
    },
    Object26 {
        kind: &'a str,
    },
    Object27 {
        kind: &'a str,
        name: &'a str,
    },
    Object28 {
        arguments: Vec<'a, Box<'a, TokenOrValue<'a>>>,
        kind: &'a str,
        name: &'a str,
    },
}

#[derive(Debug, PartialEq)]
pub enum WebKitScrollbarPseudoElement {
    Scrollbar,
    Button,
    Track,
    TrackPiece,
    Thumb,
    Corner,
    Resizer,
}

pub type ViewTransitionPartName<'a> = &'a str;

pub type Selector<'a> = Vec<'a, Box<'a, SelectorComponent<'a>>>;

pub type SelectorList<'a> = Vec<'a, Box<'a, Selector<'a>>>;

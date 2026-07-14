use crate::*;

use bitflags::bitflags;
use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug, Default, PartialEq, Visit)]
pub struct DefaultAtRule;

#[derive(Debug, PartialEq, Visit)]
pub struct StyleSheet<'a> {
    pub license_comments: Vec<'a, &'a str>,
    pub rules: Vec<'a, CssRule<'a>>,
    pub source_map_urls: Vec<'a, Option<&'a str>>,
    pub sources: Vec<'a, &'a str>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaRule<'a> {
    pub span: Span,
    pub query: MediaList<'a>,
    pub rules: Vec<'a, CssRule<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaList<'a> {
    pub media_queries: Vec<'a, MediaQuery<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct MediaQuery<'a> {
    pub condition: Option<Box<'a, MediaCondition<'a>>>,
    pub media_type: MediaType<'a>,
    pub qualifier: Option<Qualifier>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct LengthValue {
    pub unit: LengthUnit,
    pub value: f32,
}

#[derive(Debug, PartialEq, Visit)]
pub struct EnvironmentVariable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub indices: Vec<'a, i32>,
    pub name: Box<'a, EnvironmentVariableName<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Url<'a> {
    pub span: Span,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Variable<'a> {
    pub fallback: Option<Vec<'a, TokenOrValue<'a>>>,
    pub name: Box<'a, DashedIdentReference<'a>>,
}

#[derive(Debug, PartialEq, Visit)]
pub struct DashedIdentReference<'a> {
    pub from: Option<Box<'a, Specifier<'a>>>,
    pub ident: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct Function<'a> {
    pub arguments: Vec<'a, TokenOrValue<'a>>,
    #[visit(skip)]
    flags: FunctionFlags,
    pub name: &'a str,
    /// A simple value serialized from this existing function node.
    pub replacement: Option<FunctionReplacement>,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct FunctionFlags: u8 {
        /// This node was reduced to an identifier during minification.
        ///
        /// Keeping the replacement in the existing function allocation avoids
        /// allocating a new token solely to change the surrounding enum variant.
        const IS_IDENTIFIER = 1 << 0;
        /// Emit a quoted `url()` argument directly when it is safe to unquote.
        const UNQUOTED_URL = 1 << 1;
    }
}

impl<'a> Function<'a> {
    /// Creates a function with no minifier serialization state.
    #[inline]
    pub fn new(name: &'a str, arguments: Vec<'a, TokenOrValue<'a>>) -> Self {
        Self {
            arguments,
            flags: FunctionFlags::empty(),
            name,
            replacement: None,
        }
    }

    /// Returns whether this function serializes as an identifier.
    #[inline]
    pub const fn is_identifier(&self) -> bool {
        self.flags.contains(FunctionFlags::IS_IDENTIFIER)
    }

    /// Controls whether this function serializes as an identifier.
    #[inline]
    pub fn set_identifier(&mut self, is_identifier: bool) {
        self.flags.set(FunctionFlags::IS_IDENTIFIER, is_identifier);
    }

    /// Returns whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub const fn is_unquoted_url(&self) -> bool {
        self.flags.contains(FunctionFlags::UNQUOTED_URL)
    }

    /// Controls whether this function's quoted URL argument serializes unquoted.
    #[inline]
    pub fn set_unquoted_url(&mut self, unquoted_url: bool) {
        self.flags.set(FunctionFlags::UNQUOTED_URL, unquoted_url);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Visit)]
pub enum FunctionReplacement {
    GrayAlpha {
        alpha: f32,
        lightness: f32,
    },
    Number(f32),
    Dimension {
        unit: Unit,
        value: f32,
    },
    Percentage(f32),
    Rgb {
        blue: u8,
        green: u8,
        red: u8,
    },
    Rgba {
        alpha: f32,
        blue: u8,
        green: u8,
        red: u8,
        use_hex: bool,
    },
}

#[derive(Debug, PartialEq, Visit)]
pub struct ImportRule<'a> {
    pub layer: Option<Vec<'a, &'a str>>,
    pub span: Span,
    pub media: Option<Box<'a, MediaList<'a>>>,
    pub supports: Option<Box<'a, SupportsCondition<'a>>>,
    pub url: &'a str,
}

#[derive(Debug, PartialEq, Visit)]
pub struct StyleRule<'a> {
    pub declarations: Pin<Box<'a, DeclarationBlock<'a>>>,
    pub span: Span,
    pub rules: Vec<'a, CssRule<'a>>,
    pub selectors: Box<'a, SelectorList<'a>>,
    pub vendor_prefix: VendorPrefix,
}

#[derive(Debug, PartialEq, Visit)]
#[visit(pinned)]
pub struct DeclarationBlock<'a> {
    pub declarations: Vec<'a, Declaration<'a>>,
    #[visit(skip)]
    pub declarations_importance: BitVec<'a>,
    #[visit(skip)]
    _pin: PhantomPinned,
}

impl<'a> DeclarationBlock<'a> {
    #[inline]
    pub fn new(allocator: &'a rocketcss_allocator::Allocator) -> Self {
        Self {
            declarations: allocator.vec(),
            declarations_importance: BitVec::new(allocator),
            _pin: PhantomPinned,
        }
    }

    #[inline]
    pub fn push(&mut self, declaration: Declaration<'a>, important: bool) {
        self.declarations.push(declaration);
        self.declarations_importance.push(important);
    }

    #[inline]
    pub fn push_pinned(mut self: Pin<&mut Self>, declaration: Declaration<'a>, important: bool) {
        // SAFETY: pushing into the declaration vector does not move the block.
        unsafe { self.as_mut().get_unchecked_mut() }.push(declaration, important);
    }

    #[inline]
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn is_important(&self, index: usize) -> bool {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations_importance.is_set(index)
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&Declaration<'a>, bool)> {
        debug_assert_eq!(self.declarations.len(), self.declarations_importance.len());
        self.declarations
            .iter()
            .zip(self.declarations_importance.iter())
    }
}

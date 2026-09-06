use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use crate::{
    CloneIn, FromIn,
    wtf8::{Wtf8, Wtf8Buf},
};

/// A UTF-8 range in one StringPool. Equality compares ranges, not contents.
/// Text ordering requires resolving through the owning pool; offsets do not
/// define lexical order.
///
/// ```compile_fail
/// use rocketcss_common::AstStr;
/// let mut strings = [AstStr::EMPTY];
/// strings.sort();
/// ```
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AstStr<'a> {
    pub(crate) start: u32,
    pub(crate) end: u32,
    marker: PhantomData<&'a str>,
}

impl<'a> AstStr<'a> {
    pub const EMPTY: Self = Self::new(0, 0);

    pub(crate) const fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
            marker: PhantomData,
        }
    }

    pub const fn len(self) -> u32 {
        self.end - self.start
    }
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Canonical string identity within one pool. Resolve text through that pool.
/// Interning order is not lexical order, so atoms cannot be sorted directly.
///
/// ```compile_fail
/// use rocketcss_common::Atom;
/// let mut atoms = [Atom::empty()];
/// atoms.sort();
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Atom<'a>(pub(crate) AstStr<'a>);

impl<'a> Atom<'a> {
    pub const fn empty() -> Self {
        Self(AstStr::EMPTY)
    }
    pub const fn len(self) -> u32 {
        self.0.len()
    }
    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> From<Atom<'a>> for AstStr<'a> {
    fn from(value: Atom<'a>) -> Self {
        value.0
    }
}

const _: () = {
    assert!(std::mem::size_of::<Atom<'static>>() == 8);
    assert!(std::mem::size_of::<AstStr<'static>>() == 8);
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wtf8Atom<'a>(&'a Wtf8);

impl Wtf8Atom<'static> {
    #[inline]
    pub fn new_const(s: &'static str) -> Self {
        Self(Wtf8::from_str(s))
    }

    #[inline]
    pub fn empty() -> Self {
        Self::new_const("")
    }
}

impl<'a> Wtf8Atom<'a> {
    #[inline]
    pub fn new<S>(s: S) -> Self
    where
        Self: From<S>,
    {
        Self::from(s)
    }

    #[inline]
    pub fn new_in<S>(s: S, allocator: &'a crate::Allocator) -> Wtf8Atom<'a>
    where
        Wtf8Atom<'a>: FromIn<'a, S>,
    {
        Wtf8Atom::from_in(s, allocator)
    }

    #[inline]
    pub fn as_wtf8(&self) -> &Wtf8 {
        self.0
    }
}

impl<'a> From<&'a str> for Wtf8Atom<'a> {
    #[expect(clippy::inline_always)]
    #[inline(always)]
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl<'a> From<&'a Wtf8> for Wtf8Atom<'a> {
    #[expect(clippy::inline_always)]
    #[inline(always)]
    fn from(s: &'a Wtf8) -> Self {
        Self(s)
    }
}

impl<'a, 'b> FromIn<'a, &'b str> for Wtf8Atom<'a> {
    #[inline]
    fn from_in(s: &'b str, allocator: &'a crate::Allocator) -> Self {
        Self(allocator.alloc_wtf8(Wtf8::from_str(s)))
    }
}

impl<'a, 'b> FromIn<'a, &'b Wtf8> for Wtf8Atom<'a> {
    #[inline]
    fn from_in(s: &'b Wtf8, allocator: &'a crate::Allocator) -> Self {
        Self(allocator.alloc_wtf8(s))
    }
}

impl<'a> FromIn<'a, String> for Wtf8Atom<'a> {
    #[inline]
    fn from_in(s: String, allocator: &'a crate::Allocator) -> Self {
        Self(allocator.alloc_wtf8(Wtf8::from_str(s.as_str())))
    }
}

impl<'a> FromIn<'a, Wtf8Buf> for Wtf8Atom<'a> {
    #[inline]
    fn from_in(s: Wtf8Buf, allocator: &'a crate::Allocator) -> Self {
        Self(allocator.alloc_wtf8(&s))
    }
}

impl<'a> CloneIn<'a> for Wtf8Atom<'_> {
    type Cloned = Wtf8Atom<'a>;

    #[inline]
    fn clone_in(&self, allocator: &'a crate::Allocator) -> Self::Cloned {
        Wtf8Atom(allocator.alloc_wtf8(self.as_wtf8()))
    }
}

impl Default for Wtf8Atom<'_> {
    #[inline]
    fn default() -> Self {
        Wtf8Atom::empty()
    }
}

impl AsRef<Wtf8> for Wtf8Atom<'_> {
    #[inline]
    fn as_ref(&self) -> &Wtf8 {
        self.as_wtf8()
    }
}

impl Deref for Wtf8Atom<'_> {
    type Target = Wtf8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_wtf8()
    }
}

impl Debug for Wtf8Atom<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_wtf8(), f)
    }
}

impl PartialEq<Wtf8> for Wtf8Atom<'_> {
    #[inline]
    fn eq(&self, other: &Wtf8) -> bool {
        self.as_wtf8() == other
    }
}

impl PartialEq<&Wtf8> for Wtf8Atom<'_> {
    #[inline]
    fn eq(&self, other: &&Wtf8) -> bool {
        self.as_wtf8() == *other
    }
}

impl PartialEq<str> for Wtf8Atom<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_wtf8().as_str() == Some(other)
    }
}

impl PartialEq<&str> for Wtf8Atom<'_> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_wtf8().as_str() == Some(*other)
    }
}

use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, LazyLock},
};

/// An interned string identity.
///
/// Equality and hashing use the string allocation's pointer rather than its
/// contents. Values compared as atoms must therefore originate from the same
/// [`StringPool`](crate::StringPool). Use [`Atom::as_str`] when comparing
/// values from different pools.
#[derive(Clone)]
pub struct Atom<'a> {
    value: Arc<str>,
    marker: PhantomData<&'a str>,
}

static EMPTY_ATOM: LazyLock<Arc<str>> = LazyLock::new(|| Arc::from(""));

impl Atom<'static> {
    #[inline]
    pub fn empty() -> Self {
        Self::from_owned(Arc::clone(&EMPTY_ATOM))
    }
}

impl<'a> Atom<'a> {
    #[inline]
    pub(crate) fn from_owned(value: Arc<str>) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for Atom<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for Atom<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Default for Atom<'_> {
    #[inline]
    fn default() -> Self {
        Atom::empty()
    }
}

impl PartialEq for Atom<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl Eq for Atom<'_> {}

impl PartialOrd for Atom<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Atom<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().as_ptr().cmp(&other.as_str().as_ptr())
    }
}

impl Hash for Atom<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().as_ptr().hash(state);
    }
}

impl Display for Atom<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl Debug for Atom<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl PartialEq<str> for Atom<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Atom<'_> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

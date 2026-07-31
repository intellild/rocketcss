use std::sync::Arc;

use rustc_hash::FxHashSet;

use crate::atom::Atom;

/// Compilation-scoped storage that canonicalizes strings into pointer-comparable atoms.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct StringPool {
    strings: FxHashSet<Arc<str>>,
}

impl StringPool {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern<'a>(&mut self, value: &str) -> Atom<'a> {
        if value.is_empty() {
            return Atom::empty();
        }

        if let Some(value) = self.strings.get(value) {
            return Atom::from_owned(Arc::clone(value));
        }

        let value: Arc<str> = value.into();
        self.strings.insert(Arc::clone(&value));
        Atom::from_owned(value)
    }

    pub fn intern_ascii_lowercase<'a>(&mut self, value: &str) -> Atom<'a> {
        if value.bytes().all(|byte| !byte.is_ascii_uppercase()) {
            self.intern(value)
        } else {
            self.intern(&value.to_ascii_lowercase())
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use super::StringPool;

    fn hash(value: impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn canonicalizes_equal_strings_within_one_pool() {
        let mut pool = StringPool::new();

        let first = pool.intern("selector");
        let owned = String::from("selector");
        let second = pool.intern(&owned);

        assert_eq!(first, second);
        assert_eq!(hash(first), hash(second));
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn atom_identity_is_scoped_to_a_pool() {
        let mut first_pool = StringPool::new();
        let mut second_pool = StringPool::new();

        let first = first_pool.intern("selector");
        let second = second_pool.intern("selector");

        assert_ne!(first, second);
        assert_eq!(first.as_str(), second.as_str());
    }
}

use crate::{Allocator, atom::Atom, hash_map::HashMap};

/// Compilation-scoped storage that canonicalizes strings into pointer-comparable atoms.
pub struct StringPool<'alloc> {
    allocator: &'alloc Allocator,
    strings: HashMap<'alloc, &'alloc str, ()>,
}

impl<'alloc> StringPool<'alloc> {
    #[inline]
    pub fn new_in(allocator: &'alloc Allocator) -> Self {
        Self {
            allocator,
            strings: HashMap::new_in(allocator),
        }
    }

    pub fn intern(&mut self, value: &str) -> Atom<'alloc> {
        if value.is_empty() {
            return Atom::empty();
        }

        if let Some((&value, ())) = self.strings.get_key_value(value) {
            return Atom::from_interned(value);
        }

        let value = self.allocator.alloc_str(value);
        self.strings.insert(value, ());
        Atom::from_interned(value)
    }

    pub fn intern_ascii_lowercase(&mut self, value: &str) -> Atom<'alloc> {
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
    use crate::Allocator;

    fn hash(value: impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn canonicalizes_equal_strings_within_one_pool() {
        let allocator = Allocator::new();
        let mut pool = StringPool::new_in(&allocator);

        let first = pool.intern("selector");
        let owned = String::from("selector");
        let second = pool.intern(&owned);

        assert_eq!(first, second);
        assert_eq!(hash(first), hash(second));
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn atom_identity_is_scoped_to_a_pool() {
        let first_allocator = Allocator::new();
        let second_allocator = Allocator::new();
        let mut first_pool = StringPool::new_in(&first_allocator);
        let mut second_pool = StringPool::new_in(&second_allocator);

        let first = first_pool.intern("selector");
        let second = second_pool.intern("selector");

        assert_ne!(first, second);
        assert_eq!(first.as_str(), second.as_str());
    }
}

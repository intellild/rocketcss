use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;

use crate::atom::Atom;

/// The per-allocator index of interned strings.
pub type StringPool<'a> = HashMap<&'a str, Atom<'a>, FxBuildHasher>;

#[cfg(test)]
mod tests {
    use crate::Allocator;

    #[test]
    fn deduplicates_strings_in_an_allocator() {
        let allocator = Allocator::new();
        let first = allocator.alloc_str("custom-name");
        let second = allocator.alloc_str(&String::from("custom-name"));

        assert_eq!(first, second);
        assert!(std::ptr::eq(first.as_str(), second.as_str()));
        assert_eq!(allocator.string_pool_len(), 1);
    }

    #[test]
    fn atoms_from_different_allocators_have_distinct_identity() {
        let first_allocator = Allocator::new();
        let second_allocator = Allocator::new();

        let first = first_allocator.alloc_str("custom-name");
        let second = second_allocator.alloc_str("custom-name");

        assert_ne!(first, second);
        assert_eq!(first.as_str(), second.as_str());
    }

    #[test]
    fn reset_clears_the_string_index() {
        let mut allocator = Allocator::new();
        assert_eq!(allocator.alloc_str("name"), "name");
        allocator.reset();
        assert_eq!(allocator.string_pool_len(), 0);
    }
}

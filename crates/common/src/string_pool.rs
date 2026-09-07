use crate::{Allocator, AstStr, Atom, hash_map::HashMap, vec::Vec};

/// One immutable source and an append-only buffer sharing a u32 address space.
/// Deduplication keys live in the allocator independently of the growing buffer.
/// Ranges remain valid until the pool is dropped, including across parser rollback.
pub struct StringPool<'alloc> {
    allocator: &'alloc Allocator,
    source: &'alloc str,
    extra: Vec<'alloc, u8>,
    strings: HashMap<'alloc, &'alloc str, Atom<'alloc>>,
}

impl<'alloc> StringPool<'alloc> {
    pub fn new_in(allocator: &'alloc Allocator) -> Self {
        Self::with_source_in(allocator, "")
    }

    pub fn with_source_in(allocator: &'alloc Allocator, source: &'alloc str) -> Self {
        assert!(
            source.len() <= u32::MAX as usize,
            "string pool source exceeds u32"
        );
        Self {
            allocator,
            source,
            extra: Vec::new_in(allocator),
            strings: HashMap::new_in(allocator),
        }
    }

    /// Creates a validated source range without allocating or interning.
    pub fn source_range(&self, start: u32, end: u32) -> AstStr<'alloc> {
        let value = &self.source[start as usize..end as usize];
        if value.is_empty() {
            AstStr::EMPTY
        } else {
            AstStr::new(start, end)
        }
    }

    /// Takes a checked byte subrange without copying text or interning it.
    pub fn slice(&self, value: AstStr<'alloc>, start: usize, end: usize) -> AstStr<'alloc> {
        let text = &self.get(value)[start..end];
        if text.is_empty() {
            return AstStr::EMPTY;
        }
        AstStr::new(
            value.start + u32::try_from(start).unwrap(),
            value.start + u32::try_from(end).unwrap(),
        )
    }

    /// Text borrows the pool, so growing extra cannot invalidate a live borrow.
    #[inline]
    pub fn get<'range>(&self, value: impl Into<AstStr<'range>>) -> &str {
        let value = value.into();
        let start = value.start as usize;
        let end = value.end as usize;
        if end <= self.source.len() {
            &self.source[start..end]
        } else {
            // SAFETY: extra is private and only grows by appending complete
            // UTF-8 strings. Slice the str so even a foreign range checks its
            // bounds and character boundaries without rescanning the text.
            let extra = unsafe { std::str::from_utf8_unchecked(&self.extra) };
            &extra[start - self.source.len()..end - self.source.len()]
        }
    }

    /// Recognizes root-source subslices, including with_source parser inputs.
    fn source_slice(&self, value: &str) -> Option<AstStr<'alloc>> {
        let start = (value.as_ptr() as usize).checked_sub(self.source.as_ptr() as usize)?;
        let end = start.checked_add(value.len())?;
        (end <= self.source.len()).then(|| {
            // Both inputs are valid str slices. A nonempty slice contained in
            // source already has UTF-8 boundaries; source's length fits u32.
            // Avoid slicing source again just to validate the same boundaries.
            if value.is_empty() {
                AstStr::EMPTY
            } else {
                AstStr::new(start as u32, end as u32)
            }
        })
    }

    pub fn intern(&mut self, value: &str) -> Atom<'alloc> {
        if value.is_empty() {
            return Atom::empty();
        }
        if let Some(&atom) = self.strings.get(value) {
            return atom;
        }
        let range = self.add(value);
        let atom = Atom(range);
        self.strings.insert(self.allocator.alloc_str(value), atom);
        atom
    }

    /// Canonicalizes an existing range without copying its text into extra.
    pub fn intern_range(&mut self, range: AstStr<'alloc>) -> Atom<'alloc> {
        if range.is_empty() {
            return Atom::empty();
        }
        let text = self.get(range);
        if let Some(&atom) = self.strings.get(text) {
            return atom;
        }
        let key = self.allocator.alloc_str(text);
        let atom = Atom(range);
        self.strings.insert(key, atom);
        atom
    }

    /// Stores ordinary text without consulting or extending the intern table.
    pub fn add(&mut self, value: &str) -> AstStr<'alloc> {
        if value.is_empty() {
            return AstStr::EMPTY;
        }
        if let Some(range) = self.source_slice(value) {
            range
        } else {
            let start = self
                .source
                .len()
                .checked_add(self.extra.len())
                .expect("string pool overflow");
            let end = start
                .checked_add(value.len())
                .expect("string pool overflow");
            let end = u32::try_from(end).expect("string pool exceeds u32");
            self.extra.extend_from_slice(value.as_bytes());
            AstStr::new(start as u32, end)
        }
    }

    pub fn intern_ascii_lowercase(&mut self, value: &str) -> Atom<'alloc> {
        if value.bytes().all(|byte| !byte.is_ascii_uppercase()) {
            self.intern(value)
        } else {
            self.intern(&value.to_ascii_lowercase())
        }
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
    pub fn extra_len(&self) -> usize {
        self.extra.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_subslices_use_root_offsets_and_external_slices_append() {
        let allocator = Allocator::new();
        let backing = String::from("[aé😀z]");
        let source = &backing[1..backing.len() - 1];
        let mut pool = StringPool::with_source_in(&allocator, source);
        let boundaries = [0, 1, 3, 7, 8];
        for (index, &start) in boundaries.iter().enumerate() {
            for &end in &boundaries[index..] {
                let text = &source[start..end];
                let range = pool.add(text);
                assert_eq!(range, pool.source_range(start as u32, end as u32));
                assert_eq!(pool.get(range), text);
            }
        }
        assert_eq!(pool.extra_len(), 0);
        assert_eq!(pool.len(), 0);

        let copy = source.to_owned();
        // Include slices from the same backing allocation which cross either
        // root boundary, as well as distinct allocations with identical text.
        for text in [&backing[..backing.len() - 1], &backing[1..], &copy, &copy] {
            let before = pool.extra_len();
            let range = pool.add(text);
            assert_eq!(range.start as usize, source.len() + before);
            assert_eq!(range.end as usize, source.len() + before + text.len());
            assert_eq!(pool.get(range), text);
            assert_eq!(pool.extra_len(), before + text.len());
        }
        assert_eq!(pool.len(), 0);
    }

    #[test]
    fn subranges_retain_source_and_extra_storage_without_copying() {
        let allocator = Allocator::new();
        let mut pool = StringPool::with_source_in(&allocator, "aéz");
        let source = pool.source_range(0, 4);
        let extra = pool.add(&String::from("aéz"));
        let bytes = pool.extra_len();
        for original in [source, extra] {
            let middle = pool.slice(original, 1, 3);
            assert_eq!(pool.get(middle), "é");
            assert_eq!(pool.slice(original, 1, 1), AstStr::EMPTY);
        }
        assert_eq!(pool.extra_len(), bytes);
        assert_eq!(pool.len(), 0);
    }

    #[test]
    fn ordinary_strings_are_not_interned() {
        let allocator = Allocator::new();
        let mut pool = StringPool::new_in(&allocator);
        let first = pool.add("same");
        let second = pool.add("same");
        assert_ne!(first, second);
        assert_eq!(pool.get(first), pool.get(second));
        assert_eq!(pool.len(), 0);
        assert_eq!(pool.extra_len(), 8);
        let atom = pool.intern_range(first);
        assert_eq!(atom, pool.intern_range(second));
        assert_eq!(atom, pool.intern("same"));
        assert_eq!(pool.extra_len(), 8);
    }

    #[test]
    fn source_interning_retains_ranges_and_canonicalizes_contents() {
        let allocator = Allocator::new();
        let mut pool = StringPool::with_source_in(&allocator, "abc abc");
        let first = pool.source_range(0, 3);
        let second = pool.source_range(4, 7);
        assert_ne!(first, second);
        let atom = pool.intern_range(first);
        assert_eq!(atom, pool.intern_range(second));
        assert_eq!(atom, pool.intern("abc"));
        assert_eq!(pool.extra_len(), 0);
        assert_eq!(pool.get(atom), "abc");
    }

    #[test]
    fn extra_growth_and_cross_pool_copy() {
        let allocator = Allocator::new();
        let mut pool = StringPool::with_source_in(&allocator, "source");
        let atom = pool.intern("é");
        let before = pool.extra_len();
        assert_eq!(atom, pool.intern("é"));
        assert_eq!(before, pool.extra_len());
        pool.intern(&"x".repeat(8192));
        assert_eq!(pool.get(atom), "é");
        let mut other = StringPool::new_in(&allocator);
        let copied = other.intern(pool.get(atom));
        assert_eq!(other.get(copied), pool.get(atom));
        assert_eq!(pool.intern(""), Atom::empty());
        assert_eq!(pool.get(Atom::empty()), "");
    }

    #[test]
    fn cross_pool_transfer_preserves_text_and_interning_policy() {
        let destination_allocator = Allocator::new();
        let mut destination =
            StringPool::with_source_in(&destination_allocator, "different root source");
        let (first, second, atom) = {
            let source_allocator = Allocator::new();
            let source = String::from("é é");
            let mut pool = StringPool::with_source_in(&source_allocator, &source);
            let left = pool.source_range(0, 2);
            let right = pool.source_range(3, 5);
            let atom = pool.intern_range(left);
            let first = destination.add(pool.get(left));
            let second = destination.add(pool.get(right));
            assert_ne!(first, second);
            assert_eq!(destination.len(), 0);
            let copied_atom = destination.intern(pool.get(atom));
            (first, second, copied_atom)
        };
        // The source text, pool and allocator have all been dropped.
        destination.add(&"x".repeat(8192));
        assert_eq!(destination.get(first), "é");
        assert_eq!(destination.get(second), "é");
        assert_eq!(destination.get(atom), "é");
        assert_eq!(destination.intern_range(first), atom);
        assert_eq!(destination.intern_range(second), atom);
        assert_eq!(destination.len(), 1);
    }

    #[test]
    fn source_subslice_and_lowercase() {
        let allocator = Allocator::new();
        let source = String::from("aBc DEF");
        let mut pool = StringPool::with_source_in(&allocator, &source);
        let atom = pool.intern(&source[4..]);
        assert_eq!(pool.extra_len(), 0);
        assert_eq!(pool.get(atom), "DEF");
        let lower = pool.intern_ascii_lowercase(&source[..3]);
        assert_eq!(pool.get(lower), "abc");
    }

    #[test]
    #[should_panic]
    fn foreign_range_cannot_create_invalid_utf8_in_extra() {
        let allocator = Allocator::new();
        let source_pool = StringPool::with_source_in(&allocator, "ab");
        let mut other_pool = StringPool::new_in(&allocator);
        other_pool.add("é");
        other_pool.get(source_pool.source_range(1, 2));
    }

    #[test]
    #[should_panic]
    fn rejects_partial_utf8_source_range() {
        let allocator = Allocator::new();
        StringPool::with_source_in(&allocator, "é").source_range(0, 1);
    }
}

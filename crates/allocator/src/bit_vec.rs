use std::fmt;

use crate::{Allocator, CloneIn, vec::Vec};

const BITS_PER_WORD: usize = usize::BITS as usize;

/// A growable arena-backed bit vector.
pub struct BitVec<'a> {
    len: u32,
    words: Vec<'a, usize>,
}

impl<'a> BitVec<'a> {
    #[inline]
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            len: 0,
            words: allocator.vec(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn push(&mut self, value: bool) {
        let index = self.len as usize;
        self.len = self.len.checked_add(1).expect("BitVec length overflow");

        if index.is_multiple_of(BITS_PER_WORD) {
            self.words.push(0);
        }
        if value {
            self.words[index / BITS_PER_WORD] |= 1 << (index % BITS_PER_WORD);
        }
    }

    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        assert!(index < self.len(), "BitVec index out of bounds");
        self.words[index / BITS_PER_WORD] & (1 << (index % BITS_PER_WORD)) != 0
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.len(), "BitVec index out of bounds");

        let word = &mut self.words[index / BITS_PER_WORD];
        let mask = 1 << (index % BITS_PER_WORD);
        if value {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator + use<'_> {
        (0..self.len()).map(|index| self.is_set(index))
    }
}

impl fmt::Debug for BitVec<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for BitVec<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl Eq for BitVec<'_> {}

impl<'a> CloneIn<'a> for BitVec<'_> {
    type Cloned = BitVec<'a>;

    fn clone_in(&self, allocator: &'a Allocator) -> Self::Cloned {
        let mut cloned = BitVec::new(allocator);
        for value in self.iter() {
            cloned.push(value);
        }
        cloned
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::{BITS_PER_WORD, BitVec};
    use crate::Allocator;

    #[test]
    fn stores_bits_across_arena_words() {
        let allocator = Allocator::new();
        let mut bits = BitVec::new(&allocator);

        for index in 0..BITS_PER_WORD + 3 {
            bits.push(index % 3 == 0);
        }

        assert_eq!(bits.len(), BITS_PER_WORD + 3);
        assert!(!bits.is_empty());
        assert!(bits.is_set(0));
        assert_eq!(
            bits.is_set(BITS_PER_WORD + 2),
            (BITS_PER_WORD + 2).is_multiple_of(3)
        );
        assert_eq!(
            bits.iter().rev().collect::<std::vec::Vec<_>>(),
            (0..BITS_PER_WORD + 3)
                .rev()
                .map(|index| index % 3 == 0)
                .collect::<std::vec::Vec<_>>()
        );
        bits.set(0, false);
        bits.set(1, true);
        bits.set(BITS_PER_WORD + 1, true);
        assert!(!bits.is_set(0));
        assert!(bits.is_set(1));
        assert!(bits.is_set(BITS_PER_WORD + 1));
    }

    #[test]
    fn does_not_need_drop() {
        assert!(!mem::needs_drop::<BitVec<'_>>());
    }
}

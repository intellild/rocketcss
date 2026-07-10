use std::fmt;

use crate::{Allocator, CloneIn, vec::Vec};

const BITS_PER_WORD: usize = usize::BITS as usize;

/// A growable bit vector optimized for a small number of entries.
///
/// The first machine word of bits is stored inline. Additional words are
/// allocated in the arena, so this type does not need to run `Drop` and can be
/// embedded in arena-allocated AST nodes.
pub struct SmallBitVec<'a> {
    len: usize,
    inline: usize,
    overflow: Vec<'a, usize>,
}

impl<'a> SmallBitVec<'a> {
    #[inline]
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            len: 0,
            inline: 0,
            overflow: allocator.vec(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn push(&mut self, value: bool) {
        let index = self.len;
        self.len = self
            .len
            .checked_add(1)
            .expect("SmallBitVec length overflow");

        if index < BITS_PER_WORD {
            if value {
                self.inline |= 1 << index;
            }
            return;
        }

        let overflow_index = index - BITS_PER_WORD;
        let word_index = overflow_index / BITS_PER_WORD;
        let bit_index = overflow_index % BITS_PER_WORD;

        if bit_index == 0 {
            debug_assert_eq!(word_index, self.overflow.len());
            self.overflow.push(0);
        }

        if value {
            self.overflow[word_index] |= 1 << bit_index;
        }
    }

    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        assert!(index < self.len, "SmallBitVec index out of bounds");

        if index < BITS_PER_WORD {
            return self.inline & (1 << index) != 0;
        }

        let overflow_index = index - BITS_PER_WORD;
        let word_index = overflow_index / BITS_PER_WORD;
        let bit_index = overflow_index % BITS_PER_WORD;
        self.overflow[word_index] & (1 << bit_index) != 0
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator + use<'_> {
        (0..self.len).map(|index| self.is_set(index))
    }
}

impl fmt::Debug for SmallBitVec<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for SmallBitVec<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl Eq for SmallBitVec<'_> {}

impl<'a> CloneIn<'a> for SmallBitVec<'_> {
    type Cloned = SmallBitVec<'a>;

    fn clone_in(&self, allocator: &'a Allocator) -> Self::Cloned {
        let mut cloned = SmallBitVec::new(allocator);
        for value in self.iter() {
            cloned.push(value);
        }
        cloned
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::{BITS_PER_WORD, SmallBitVec};
    use crate::Allocator;

    #[test]
    fn stores_inline_and_overflow_bits() {
        let allocator = Allocator::new();
        let mut bits = SmallBitVec::new(&allocator);

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
    }

    #[test]
    fn does_not_need_drop() {
        assert!(!mem::needs_drop::<SmallBitVec<'_>>());
    }
}

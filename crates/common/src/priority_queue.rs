use crate::{Allocator, vec::Vec};

/// An arena-backed max-priority queue.
///
/// Values are ordered by [`Ord`], and [`PriorityQueue::pop`] removes the
/// greatest value first. Wrap values in [`std::cmp::Reverse`] for min-priority
/// behavior.
#[derive(Debug)]
pub struct PriorityQueue<'arena, T: Unpin> {
    values: Vec<'arena, T>,
}

impl<'arena, T: Unpin + Ord> PriorityQueue<'arena, T> {
    #[inline]
    pub fn new_in(allocator: &'arena Allocator) -> Self {
        Self {
            values: Vec::new_in(allocator),
        }
    }

    #[inline]
    pub fn with_capacity_in(capacity: usize, allocator: &'arena Allocator) -> Self {
        Self {
            values: Vec::with_capacity_in(capacity, allocator),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.values.first()
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
        self.sift_up(self.values.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let last = self.values.pop()?;
        if self.values.is_empty() {
            return Some(last);
        }
        let value = std::mem::replace(&mut self.values[0], last);
        self.sift_down(0);
        Some(value)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    /// Mutates every queued value and restores the heap invariant afterward.
    pub fn update_all(&mut self, mut update: impl FnMut(&mut T)) {
        for value in &mut self.values {
            update(value);
        }
        self.rebuild();
    }

    fn rebuild(&mut self) {
        for index in (0..self.values.len() / 2).rev() {
            self.sift_down(index);
        }
    }

    fn sift_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent = (index - 1) / 2;
            if self.values[parent] >= self.values[index] {
                break;
            }
            self.values.swap(parent, index);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = index * 2 + 1;
            let Some(right) = left.checked_add(1) else {
                break;
            };
            let mut greatest = index;
            if self
                .values
                .get(left)
                .is_some_and(|value| value > &self.values[greatest])
            {
                greatest = left;
            }
            if self
                .values
                .get(right)
                .is_some_and(|value| value > &self.values[greatest])
            {
                greatest = right;
            }
            if greatest == index {
                break;
            }
            self.values.swap(index, greatest);
            index = greatest;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use super::PriorityQueue;
    use crate::Allocator;

    #[test]
    fn pops_greatest_values_first() {
        let allocator = Allocator::new();
        let mut queue = PriorityQueue::with_capacity_in(4, &allocator);
        queue.push(2);
        queue.push(4);
        queue.push(1);
        queue.push(3);

        assert_eq!(queue.peek(), Some(&4));
        assert_eq!(queue.len(), 4);
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn reverse_values_form_a_min_priority_queue() {
        let allocator = Allocator::new();
        let mut queue = PriorityQueue::new_in(&allocator);
        queue.push(Reverse(3));
        queue.push(Reverse(1));
        queue.push(Reverse(2));

        assert_eq!(queue.pop(), Some(Reverse(1)));
        assert_eq!(queue.pop(), Some(Reverse(2)));
        assert_eq!(queue.pop(), Some(Reverse(3)));
    }

    #[test]
    fn update_all_rebuilds_the_heap() {
        let allocator = Allocator::new();
        let mut queue = PriorityQueue::new_in(&allocator);
        queue.push(3);
        queue.push(2);
        queue.push(1);

        queue.update_all(|value| {
            if *value == 1 {
                *value = 4;
            }
        });

        assert_eq!(queue.peek(), Some(&4));
        let values = queue.iter().copied().collect::<std::vec::Vec<_>>();
        assert_eq!(values.len(), 3);

        let capacity = queue.capacity();
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), capacity);
    }
}

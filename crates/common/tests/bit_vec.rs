use rocketcss_common::{Allocator, bit_vec::BitVec};

const BITS_PER_WORD: usize = usize::BITS as usize;

#[test]
fn reset_clears_less_than_one_word() {
    let allocator = Allocator::new();
    let mut bits = BitVec::new(&allocator);

    bits.reset(7);
    assert_eq!(bits.len(), 7);
    assert!(bits.iter().all(|bit| !bit));

    bits.set(3, true);
    bits.reset(7);
    assert!(bits.iter().all(|bit| !bit));
}

#[test]
fn reset_clears_across_word_boundaries() {
    let allocator = Allocator::new();
    let mut bits = BitVec::new(&allocator);
    let len = BITS_PER_WORD + 3;

    bits.reset(len);
    bits.set(BITS_PER_WORD - 1, true);
    bits.set(BITS_PER_WORD, true);
    bits.set(len - 1, true);
    bits.reset(len);

    assert_eq!(bits.len(), len);
    assert!(bits.iter().all(|bit| !bit));
}

#[test]
fn reset_reuses_large_storage_without_leaking_bits() {
    let allocator = Allocator::new();
    let mut bits = BitVec::new(&allocator);
    let large_len = BITS_PER_WORD * 3 + 5;

    bits.reset(large_len);
    bits.set(1, true);
    bits.set(BITS_PER_WORD * 2 + 1, true);
    bits.set(large_len - 1, true);

    bits.reset(3);
    assert_eq!(bits.len(), 3);
    assert!(bits.iter().all(|bit| !bit));

    bits.set(2, true);
    bits.reset(large_len);
    assert_eq!(bits.len(), large_len);
    assert!(bits.iter().all(|bit| !bit));
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic(expected = "BitVec length exceeds u32::MAX")]
fn reset_rejects_lengths_larger_than_u32() {
    let allocator = Allocator::new();
    let mut bits = BitVec::new(&allocator);

    bits.reset(u32::MAX as usize + 1);
}

use std::mem::size_of;
use smallvec::SmallVec;

#[test]
fn arrays() {
    assert_eq!(100*1, size_of::<[bool; 100]>());
    assert_eq!(100*1, size_of::<[u8; 100]>());
    assert_eq!(100*2, size_of::<[u16; 100]>());
    assert_eq!(100*4, size_of::<[u32; 100]>());
    assert_eq!(100*8, size_of::<[u64; 100]>());
    assert_eq!(100*8, size_of::<[usize; 100]>());
}





















#[test]
fn slices() {
    assert_eq!(8, size_of::<&[bool; 100]>());
    assert_eq!(8, size_of::<&[u8; 100]>());
    assert_eq!(8, size_of::<&[u16; 100]>());
    assert_eq!(8, size_of::<&[u32; 100]>());
    assert_eq!(8, size_of::<&[u64; 100]>());
    assert_eq!(8, size_of::<&[usize; 100]>());
}






















#[test]
fn vec() {
    assert_eq!(24, size_of::<Vec<bool>>());
    assert_eq!(24, size_of::<Vec<u8>>());
    assert_eq!(24, size_of::<Vec<u16>>());
    assert_eq!(24, size_of::<Vec<u32>>());
    assert_eq!(24, size_of::<Vec<u64>>());
    assert_eq!(24, size_of::<Vec<usize>>());

    assert_eq!(24, size_of::<String>());
    assert_eq!(16, size_of::<&str>());
}
























#[test]
fn smallvec() {
    assert_eq!(32, size_of::<SmallVec<[bool; 0]>>());
    assert_eq!(32, size_of::<SmallVec<[bool; 23]>>());
    assert_eq!(40, size_of::<SmallVec<[bool; 24]>>());

    assert_eq!(32, size_of::<SmallVec<[u8; 23]>>());
    assert_eq!(40, size_of::<SmallVec<[u8; 24]>>());

    assert_eq!(32, size_of::<SmallVec<[u16; 11]>>());
    assert_eq!(40, size_of::<SmallVec<[u16; 12]>>());

    assert_eq!(32, size_of::<SmallVec<[u32; 5]>>());
    assert_eq!(40, size_of::<SmallVec<[u32; 6]>>());

    assert_eq!(32, size_of::<SmallVec<[u64; 2]>>());
    assert_eq!(40, size_of::<SmallVec<[u64; 3]>>());

    assert_eq!(32, size_of::<SmallVec<[usize; 2]>>());
    assert_eq!(40, size_of::<SmallVec<[usize; 3]>>());
}

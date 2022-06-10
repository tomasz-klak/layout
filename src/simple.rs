use std::mem::size_of;

#[test]
fn simple_types() {
    assert_eq!(1, size_of::<bool>());
    assert_eq!(1, size_of::<u8>());
    assert_eq!(2, size_of::<u16>());
    assert_eq!(4, size_of::<u32>());
    assert_eq!(8, size_of::<u64>());
    assert_eq!(8, size_of::<usize>());
}

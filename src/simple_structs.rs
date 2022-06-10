use std::mem::size_of;

struct S<T>(T);

#[test]
fn simple_structs() {
    assert_eq!(1, size_of::<S<bool>>());
    assert_eq!(1, size_of::<S<u8>>());
    assert_eq!(2, size_of::<S<u16>>());
    assert_eq!(4, size_of::<S<u32>>());
    assert_eq!(8, size_of::<S<u64>>());
    assert_eq!(8, size_of::<S<usize>>());
}

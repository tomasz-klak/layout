use std::mem::size_of;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn references() {
    assert_eq!(8, size_of::<&bool>());
    assert_eq!(8, size_of::<&u8>());
    assert_eq!(8, size_of::<&u16>());
    assert_eq!(8, size_of::<&u32>());
    assert_eq!(8, size_of::<&u64>());
    assert_eq!(8, size_of::<&usize>());
}
















#[test]
fn r#box() {
    // identifiers can be equal to keywords (for,loop etc)

    assert_eq!(8, size_of::<Box<bool>>());
    assert_eq!(8, size_of::<Box<u8>>());
    assert_eq!(8, size_of::<Box<u16>>());
    assert_eq!(8, size_of::<Box<u32>>());
    assert_eq!(8, size_of::<Box<u64>>());
    assert_eq!(8, size_of::<Box<usize>>());
}






















#[test]
fn rc() {
    assert_eq!(8, size_of::<Rc<bool>>());
    assert_eq!(8, size_of::<Rc<u8>>());
    assert_eq!(8, size_of::<Rc<u16>>());
    assert_eq!(8, size_of::<Rc<u32>>());
    assert_eq!(8, size_of::<Rc<u64>>());
    assert_eq!(8, size_of::<Rc<usize>>());
}


























#[test]
fn arc() {
    assert_eq!(8, size_of::<Arc<bool>>());
    assert_eq!(8, size_of::<Arc<u8>>());
    assert_eq!(8, size_of::<Arc<u16>>());
    assert_eq!(8, size_of::<Arc<u32>>());
    assert_eq!(8, size_of::<Arc<u64>>());
    assert_eq!(8, size_of::<Arc<usize>>());
}



























#[test]
fn traits() {
    trait Marker {};

    assert_eq!(16, size_of::<&dyn Marker>());
    assert_eq!(16, size_of::<Rc<dyn Marker>>());
    assert_eq!(16, size_of::<Arc<dyn Marker>>());

    impl Marker for bool {}

    fn test<T: Marker>() {
        assert_eq!(8, size_of::<&T>());
        assert_eq!(8, size_of::<Rc<T>>());
        assert_eq!(8, size_of::<Arc<T>>());
    }

    test::<bool>();
}

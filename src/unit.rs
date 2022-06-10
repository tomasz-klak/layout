use std::marker::PhantomData;
use std::mem::{align_of, size_of};

#[test]
fn unit() {
    assert_eq!(0, size_of::<()>());
    assert_eq!(1, align_of::<()>());
}

#[test]
fn phantom() {
    assert_eq!(0, size_of::<PhantomData<u64>>());
    assert_eq!(1, align_of::<PhantomData<u64>>());
}

#[test]
fn in_struct() {
    struct S((), PhantomData<u64>, (), PhantomData<u64>);
    assert_eq!(0, size_of::<S>());
    assert_eq!(1, align_of::<S>());

    struct S2(u64, (), PhantomData<u64>, (), PhantomData<u64>);
    assert_eq!(8, size_of::<S2>());
    assert_eq!(8, align_of::<S2>());
}

#[test]
fn in_array() {
    type A = [u64; 10];
    assert_eq!(80, size_of::<A>());
    assert_eq!(8, align_of::<A>());

    type B = [(); 10];
    assert_eq!(0, size_of::<B>());
    assert_eq!(1, align_of::<B>());

    type C = [PhantomData<u64>; 10];
    assert_eq!(0, size_of::<C>());
    assert_eq!(1, align_of::<C>());

    struct S((), PhantomData<u64>, (), PhantomData<u64>);
    type D = [S; 10];
    assert_eq!(0, size_of::<D>());
    assert_eq!(1, align_of::<D>());

    struct S2(u64, (), PhantomData<u64>, (), PhantomData<u64>);
    type E = [S2; 10];
    assert_eq!(80, size_of::<E>());
    assert_eq!(8, align_of::<E>());
}

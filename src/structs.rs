use memoffset::offset_of;
use std::mem::{align_of, size_of};

#[test]
fn structs() {
    struct U8_and_U64 {
        small: u8,
        big: u64,
    }
    struct U64_and_U8 {
        big: u64,
        small: u8,
    }
    struct U8_U8_U8(u8, u8, u8);

    assert_eq!(16, size_of::<U8_and_U64>());
    assert_eq!(8, align_of::<U8_and_U64>());
    assert_eq!(0, offset_of!(U8_and_U64, big));
    assert_eq!(8, offset_of!(U8_and_U64, small));

    assert_eq!(16, size_of::<U64_and_U8>());
    assert_eq!(8, align_of::<U64_and_U8>());
    assert_eq!(0, offset_of!(U64_and_U8, big));
    assert_eq!(8, offset_of!(U64_and_U8, small));

    assert_eq!(3, size_of::<U8_U8_U8>());
    assert_eq!(1, align_of::<U8_U8_U8>());
}

#[test]
fn structs_packed() {
    #[repr(packed)]
    struct U8_and_U64 {
        small: u8,
        big: u64,
    }
    #[repr(packed)]
    struct U64_and_U8 {
        big: u64,
        small: u8,
    }
    #[repr(packed)]
    struct U8_U8_U8(u8, u8, u8);

    assert_eq!(9, size_of::<U8_and_U64>());
    assert_eq!(9, size_of::<U64_and_U8>());
    assert_eq!(3, size_of::<U8_U8_U8>());
}

#[test]
fn structs_repr_c() {
    #[repr(C)]
    struct U8_U8_U16 {
        small1: u8,
        small2: u8,
        big: u16,
    }
    #[repr(C)]
    struct U16_U8_U8 {
        big: u16,
        small1: u8,
        small2: u8,
    }
    #[repr(C)]
    struct U8_U16_U8 {
        small1: u8,
        big: u16,
        small2: u8,
    }

    assert_eq!(4, size_of::<U8_U8_U16>());
    assert_eq!(0, offset_of!(U8_U8_U16, small1));
    assert_eq!(1, offset_of!(U8_U8_U16, small2));
    assert_eq!(2, offset_of!(U8_U8_U16, big));

    assert_eq!(4, size_of::<U16_U8_U8>());
    assert_eq!(0, offset_of!(U16_U8_U8, big));
    assert_eq!(2, offset_of!(U16_U8_U8, small1));
    assert_eq!(3, offset_of!(U16_U8_U8, small2));

    assert_eq!(6, size_of::<U8_U16_U8>());
    assert_eq!(0, offset_of!(U8_U16_U8, small1));
    assert_eq!(2, offset_of!(U8_U16_U8, big));
    assert_eq!(4, offset_of!(U8_U16_U8, small2));
}

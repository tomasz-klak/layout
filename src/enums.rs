use std::mem::{align_of, size_of};

#[test]
fn enums() {
    enum E {
        None,
        U64_and_U64(u64, u64),
    }

    assert_eq!(24, size_of::<E>());
    assert_eq!(8, align_of::<E>());
}

#[test]
fn enums2() {
    enum E {
        None,
        U32_and_U64(u32, u64),
    }

    assert_eq!(16, size_of::<E>());
    assert_eq!(8, align_of::<E>());
}

#[test]
fn enums2_inner() {
    struct Inner(u32, u64);
    assert_eq!(16, size_of::<Inner>());

    enum E {
        None,
        U32_and_U64(Inner),
    }

    assert_eq!(24, size_of::<E>());
    assert_eq!(8, align_of::<E>());
}

#[test]
fn enums2_inner_packed() {
    /* Won't compile - packed can be applied to struct or union
    #[repr(packed)]
    enum E {
        None,
        U32_and_U64(u32, u64),
    }

    assert_eq!(16, size_of::<E>());
    assert_eq!(8, align_of::<E>());
    */
    #[repr(packed)]
    struct Inner(u32, u64);
    assert_eq!(12, size_of::<Inner>());

    enum E {
        None,
        U32_and_U64(Inner),
    }

    assert_eq!(13, size_of::<E>());
    assert_eq!(1, align_of::<E>());
}

#[test]
fn enums_nonzero() {
    use std::num::NonZeroU64;
    enum E {
        None,
        U32_and_U64(NonZeroU64, u64),
    }

    assert_eq!(16, size_of::<E>());
    assert_eq!(8, align_of::<E>());
}

#[test]
fn enums_nonzero_bits() {
    use std::num::NonZeroU64;
    enum E {
        A,
        B,
        U32_and_U64(NonZeroU64, u64),
    }

    assert_eq!(24, size_of::<E>());
    assert_eq!(8, align_of::<E>());

    enum E2 {
        A,
        B,
        U32_and_U64(u32, u64),
    }

    assert_eq!(16, size_of::<E2>());
    assert_eq!(8, align_of::<E2>());
}

#[test]
fn option_ref() {
    type T = Option<&'static u64>;

    assert_eq!(8, size_of::<&'static u64>());
    assert_eq!(8, align_of::<&'static u64>());

    assert_eq!(8, size_of::<T>());
    assert_eq!(8, align_of::<T>());
}

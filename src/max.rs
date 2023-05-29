pub trait Max {
    fn max() -> Self;
}

impl Max for i8 {
    fn max() -> Self {
        i8::MAX
    }
}

impl Max for u8 {
    fn max() -> Self {
        u8::MAX
    }
}

impl Max for i16 {
    fn max() -> Self {
        i16::MAX
    }
}

impl Max for u16 {
    fn max() -> Self {
        u16::MAX
    }
}

impl Max for i32 {
    fn max() -> Self {
        i32::MAX
    }
}

impl Max for u32 {
    fn max() -> Self {
        u32::MAX
    }
}

impl Max for i64 {
    fn max() -> Self {
        i64::MAX
    }
}

impl Max for u64 {
    fn max() -> Self {
        u64::MAX
    }
}

impl Max for i128 {
    fn max() -> Self {
        i128::MAX
    }
}

impl Max for u128 {
    fn max() -> Self {
        u128::MAX
    }
}

impl Max for isize {
    fn max() -> Self {
        isize::MAX
    }
}

impl Max for usize {
    fn max() -> Self {
        usize::MAX
    }
}

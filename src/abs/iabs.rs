use std::fmt::Debug;
use std::ops::Neg;

trait IAbs {
    type Output;

    fn iabs(self) -> <Self as IAbs>::Output
    where
        Self: Sized + PartialOrd + Neg + From<i8> + TryInto<<Self as IAbs>::Output>,
        <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
        <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>,
        <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug
    {
        if self >= 0_i8.into() {
            self.try_into().unwrap()
        } else {
            (-self).try_into().unwrap()
        }
    }
}

impl IAbs for i32 {
    type Output = u32;
}

impl IAbs for i8 {
    type Output = u8;
}

impl IAbs for i64 {
    type Output = u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_i32_iabs_plus() {
        let actual = 0_i32.iabs();
        let expected = 0u32;
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_i32_iabs_minus() {
        let actual = (-1_i32).iabs();
        let expected = 1u32;
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_i8_iabs_plus() {
        let actual = 0_i8.iabs();
        let expected = 0u8;
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_i8_iabs_minus() {
        let actual = (-1_i8).iabs();
        let expected = 1u8;
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_i64_iabs_plus() {
        let actual = 0_i64.iabs();
        let expected = 0u64;
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_i64_iabs_minus() {
        let actual = (-1_i64).iabs();
        let expected = 1u64;
        assert_eq!(expected, actual);
    }
}

use std::{fmt::Debug, hash::Hash, num::ParseIntError, str::FromStr};

/// Value for `OrderMethod`. This should be `i8` or `i16`.
pub trait OrderMethodValue:
    Debug
    + Default
    + Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Hash
    + FromStr<Err = ParseIntError> {
    fn zero() -> Self;

    fn one() -> Self;

    fn abs(&self) -> Self;
}

impl OrderMethodValue for i8 {
    #[inline]
    fn zero() -> Self {
        0
    }

    #[inline]
    fn one() -> Self {
        1
    }

    #[inline]
    fn abs(&self) -> Self {
        i8::abs(*self)
    }
}

impl OrderMethodValue for i16 {
    #[inline]
    fn zero() -> Self {
        0
    }

    #[inline]
    fn one() -> Self {
        1
    }

    #[inline]
    fn abs(&self) -> Self {
        i16::abs(*self)
    }
}

/// An integer value for ordering.
///
/// * **0**: Disabled
/// * **> 0**: Ascending
/// * **< 0**: Descending
/// * Absolute value indicates priority; the smaller the value, the more important it is.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrderMethod<T: OrderMethodValue = i8>(pub T);

impl<T: OrderMethodValue> From<T> for OrderMethod<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

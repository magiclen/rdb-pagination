use crate::{OrderMethod, OrderMethodValue};

/// Enum representing the order type.
#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    Asc,
    Desc,
}

impl OrderType {
    /// Constructs an `OrderType` from the given `OrderMethod`.
    ///
    /// # Panics
    ///
    /// Panics if `order_method` is `0`.
    #[inline]
    pub(crate) fn from_order_method<T: OrderMethodValue>(order_method: OrderMethod<T>) -> Self {
        debug_assert!(order_method != OrderMethod(T::zero()));

        if order_method > OrderMethod(T::zero()) {
            Self::Asc
        } else {
            Self::Desc
        }
    }

    /// Returns the string representation of the order type.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

impl AsRef<str> for OrderType {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

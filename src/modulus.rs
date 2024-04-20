use num_traits::Unsigned;

use crate::Integer;

/// Modulus is an unsigned integer that can be cast to some other type `T`.
/// The purpose is to restrict the modulus to unsigned integers yet allow calculations with signed integers
/// when necessary.
pub trait Modulus<T>: Integer + Unsigned
where
    T: TryFrom<Self>,
    <T as TryFrom<Self>>::Error: std::fmt::Debug,
{
    /// Cast the modulus to some other type `T`.
    /// Panics if the modulus cannot be converted to `T`.
    fn cast(self) -> T {
        self.try_into().expect("cannot convert modulus")
    }
}

impl<T, M> Modulus<T> for M
where
    T: TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Integer + Unsigned,
{
}

use num_traits::Unsigned;

use crate::Integer;

pub trait Modulus<T>: Integer + Unsigned
where
    T: TryFrom<Self>,
    <T as TryFrom<Self>>::Error: std::fmt::Debug,
{
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

mod egcd;
mod integer;
mod modulus;

pub use integer::Integer;
pub use modulus::Modulus;

pub trait Constrain<M: Modulus<Self>>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    fn constrain(self, modulus: M) -> Self;
}

impl<T, M> Constrain<M> for T
where
    T: Integer + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    fn constrain(self, modulus: M) -> T {
        let modulus = modulus.cast();
        (self % modulus + modulus) % modulus
    }
}

pub trait AddMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    type Output;
    fn add_mod(self, rhs: Rhs, modulus: M) -> Self::Output;
}

impl<T, M> AddMod<M> for T
where
    T: Integer + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    type Output = T;
    fn add_mod(self, rhs: T, modulus: M) -> T {
        (self + rhs).constrain(modulus)
    }
}

pub trait SubMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    type Output;
    fn sub_mod(self, rhs: Rhs, modulus: M) -> Self::Output;
}

impl<T, M> SubMod<M> for T
where
    T: Integer + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    type Output = T;
    fn sub_mod(self, rhs: T, modulus: M) -> T {
        (self - rhs).constrain(modulus)
    }
}

pub trait MulMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    type Output;
    fn mul_mod(self, rhs: Rhs, modulus: M) -> Self::Output;
}

impl<T, M> MulMod<M> for T
where
    T: Integer + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    type Output = T;
    fn mul_mod(self, rhs: T, modulus: M) -> T {
        (self * rhs).constrain(modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_constrain() {
        assert_eq!(10.constrain(5u8), 0);
        assert_eq!(10.constrain(7u8), 3);
        assert_eq!(10.constrain(11u8), 10);
    }

    #[test]
    fn test_constrain_negative() {
        assert_eq!((-10).constrain(5u8), 0);
        assert_eq!((-10).constrain(7u8), 4);
        assert_eq!((-10).constrain(11u8), 1);
    }

    #[test]
    fn test_add_mod() {
        assert_eq!(10.add_mod(5, 7u8), 1);
        assert_eq!(10.add_mod(5, 11u8), 4);
        assert_eq!(10.add_mod(5, 13u8), 2);
    }

    #[test]
    fn test_add_negative_mod() {
        assert_eq!((-10).add_mod(5, 7u8), 2);
        assert_eq!((-10).add_mod(5, 11u8), 6);
        assert_eq!((-10).add_mod(5, 13u8), 8);
    }

    #[test]
    fn test_sub_mod() {
        assert_eq!(10.sub_mod(5, 7u8), 5);
        assert_eq!(10.sub_mod(5, 11u8), 5);
        assert_eq!(10.sub_mod(5, 13u8), 5);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(10.mul_mod(5, 7u8), 1);
        assert_eq!(10.mul_mod(5, 11u8), 6);
        assert_eq!(10.mul_mod(5, 13u8), 11);
    }
}

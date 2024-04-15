mod egcd;
mod integer;
mod invert;
mod modulus;

pub use egcd::Egcd;
pub use integer::Integer;
pub use invert::Invert;
pub use modulus::Modulus;
use num_traits::FromPrimitive;

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

pub trait DivMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    type Output;
    fn div_mod(self, rhs: Rhs, modulus: M) -> Option<Self::Output>;
}

impl<T, M> DivMod<M> for T
where
    T: Invert + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    type Output = T;
    fn div_mod(self, rhs: T, modulus: M) -> Option<T> {
        let inverse = rhs.invert(modulus)?;
        Some((inverse * self).constrain(modulus))
    }
}

pub trait PowMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    type Output;
    fn pow_mod(self, rhs: Rhs, modulus: M) -> Self::Output;
}

impl<T, M> PowMod<M> for T
where
    T: Integer + TryFrom<M> + Constrain<M> + FromPrimitive + MulMod<M, Output = T> + DivMod<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    type Output = T;
    fn pow_mod(self, mut rhs: T, modulus: M) -> T {
        let two = T::from_i8(2).expect("two");
        let mut result = T::one();
        let mut base = self;
        while rhs != T::zero() {
            if rhs % two == T::one() {
                result = result.mul_mod(base, modulus);
            }
            base = base.mul_mod(base, modulus);
            rhs = rhs / two;
        }
        result
    }
}

pub trait EqMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    fn eq_mod(self, rhs: Rhs, modulus: M) -> bool;
    fn ne_mod(self, rhs: Rhs, modulus: M) -> bool;
}

impl<T, M> EqMod<M> for T
where
    T: Integer + TryFrom<M> + Constrain<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    fn eq_mod(self, rhs: T, modulus: M) -> bool {
        self.constrain(modulus) == rhs.constrain(modulus)
    }
    fn ne_mod(self, rhs: T, modulus: M) -> bool {
        self.constrain(modulus) != rhs.constrain(modulus)
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
    fn test_sub_negative_mod() {
        assert_eq!((-10).sub_mod(5, 7u8), 6);
        assert_eq!((-10).sub_mod(5, 11u8), 7);
        assert_eq!((-10).sub_mod(5, 13u8), 11);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(10.mul_mod(5, 7u8), 1);
        assert_eq!(10.mul_mod(5, 11u8), 6);
        assert_eq!(10.mul_mod(5, 13u8), 11);
    }

    #[test]
    fn test_mul_negative_mod() {
        assert_eq!((-10).mul_mod(5, 7u8), 6);
        assert_eq!((-10).mul_mod(5, 11u8), 5);
        assert_eq!((-10).mul_mod(5, 13u8), 2);
    }

    #[test]
    fn test_div_mod() {
        assert_eq!(10.div_mod(5, 7u8), Some(2));
        assert_eq!(10.div_mod(5, 11u8), Some(2));
        assert_eq!(10.div_mod(5, 13u8), Some(2));
        assert_eq!(10.div_mod(5, 10u8), None);
    }

    #[test]
    fn test_div_negative_mod() {
        assert_eq!((-10).div_mod(5, 7u8), Some(5));
        assert_eq!((-10).div_mod(5, 11u8), Some(9));
        assert_eq!((-10).div_mod(5, 13u8), Some(11));
        assert_eq!((-10).div_mod(5, 10u8), None);
    }

    #[test]
    fn test_eq_mod() {
        assert!(10.eq_mod(3, 7u8));
        assert!(10.eq_mod(10, 11u8));
        assert!(10.eq_mod(10, 13u8));
    }

    #[test]
    fn test_eq_negative_mod() {
        assert!((-10).eq_mod(4, 7u8));
        assert!((-10).eq_mod(0, 10u8));
        assert!((-10).eq_mod(3, 13u8));
    }

    #[test]
    fn test_ne_mod() {
        assert!(!10.ne_mod(3, 7u8));
        assert!(!10.ne_mod(0, 10u8));
        assert!(!10.ne_mod(10, 13u8));
    }

    #[test]
    fn test_ne_negative_mod() {
        assert!(!(-10).ne_mod(4, 7u8));
        assert!(!(-10).ne_mod(0, 10u8));
        assert!(!(-10).ne_mod(3, 13u8));
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(10.pow_mod(3, 3u8), 1);
        assert_eq!(10.pow_mod(3, 7u8), 6);
        assert_eq!(10.pow_mod(3, 10u8), 0);
        assert_eq!(10.pow_mod(3, 11u8), 10);
        assert_eq!(10.pow_mod(3, 13u8), 12);
    }

    #[test]
    fn test_pow_negative_mod() {
        assert_eq!((-10).pow_mod(3, 3u8), 2);
        assert_eq!((-10).pow_mod(3, 7u8), 1);
        assert_eq!((-10).pow_mod(3, 10u8), 0);
        assert_eq!((-10).pow_mod(3, 11u8), 1);
        assert_eq!((-10).pow_mod(3, 13u8), 1);
    }
}

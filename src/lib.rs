//! # Modular arithmetic
//!
//! This crate provides a set of traits to perform modular arithmetic on integer types.
//! The traits are implemented for the standard integer types and can be implemented for custom integer types.
//! The traits are:
//! - `Constrain<M>`: constrain an integer to a modulus.
//! - `AddMod<M>`: add two integers and constrain the result to a modulus.
//! - `SubMod<M>`: subtract two integers and constrain the result to a modulus.
//! - `MulMod<M>`: multiply two integers and constrain the result to a modulus.
//! - `DivMod<M>`: divide two integers and constrain the result to a modulus.
//! - `PowMod<M>`: raise an integer to a power and constrain the result to a modulus.
//! - `EqMod<M>`: check if two integers are congruent modulo a given modulus.
//! - `Invert`: invert an integer with respect to a modulus.
//!
//! # Example
//! ```
//! use modicum::*;
//! use pretty_assertions::assert_eq;
//!
//! let a = 5_i8;
//! let b = 3_i8;
//! let modulus = 7_u32;
//! assert_eq!(a.add_mod(b, modulus), 1);
//! assert_eq!(a.sub_mod(b, modulus), 2);
//! assert_eq!(a.mul_mod(b, modulus), 1);
//! assert_eq!(a.div_mod(b, modulus), Some(4));
//! assert!(a.eq_mod(5, modulus));
//! assert!(!a.ne_mod(5, modulus));
//! assert!(a.ne_mod(6, modulus));
//! assert!(!a.eq_mod(6, modulus));
//! ```

mod egcd;
mod integer;
mod invert;
mod modulus;

pub use egcd::Egcd;
pub use integer::Integer;
pub use invert::Invert;
pub use modulus::Modulus;
use num_traits::FromPrimitive;

/// A trait to constrain an integer to a modulus.
pub trait Constrain<M: Modulus<Self>>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// Constrain an integer to a modulus.
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

/// A trait to add two integers and constrain the result to a modulus.
pub trait AddMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// The output type.
    type Output;

    /// Add two integers and constrain the result to a modulus.
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

/// A trait to subtract two integers and constrain the result to a modulus.
pub trait SubMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// The output type.
    type Output;

    /// Subtract two integers and constrain the result to a modulus.
    fn sub_mod(self, rhs: Rhs, modulus: M) -> Self::Output;
}

impl<T, M> SubMod<M> for T
where
    T: Integer + TryFrom<M>,
    <T as TryFrom<M>>::Error: std::fmt::Debug,
    M: Modulus<T>,
{
    /// The output type.
    type Output = T;

    /// Subtract two integers and constrain the result to a modulus.
    fn sub_mod(self, rhs: T, modulus: M) -> T {
        (self - rhs).constrain(modulus)
    }
}

/// A trait to multiply two integers and constrain the result to a modulus.
pub trait MulMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// The output type.
    type Output;

    /// Multiply two integers and constrain the result to a modulus.
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

/// A trait to divide two integers and constrain the result to a modulus.
pub trait DivMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// The output type.
    type Output;

    /// Divide two integers and constrain the result to a modulus.
    /// If the divisor is not invertible, return `None`.
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

/// A trait to raise an integer to a power and constrain the result to a modulus.
pub trait PowMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// The output type.
    type Output;

    /// Raise an integer to a power and constrain the result to a modulus.
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

/// A trait to check if two integers are congruent, that is, they are equal modulo a given modulus.
pub trait EqMod<M: Modulus<Self>, Rhs = Self>
where
    Self: TryFrom<M>,
    <Self as TryFrom<M>>::Error: std::fmt::Debug,
{
    /// Check if two integers are congruent modulo a given modulus.
    fn eq_mod(self, rhs: Rhs, modulus: M) -> bool;

    /// Check if two integers are not congruent modulo a given modulus.
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

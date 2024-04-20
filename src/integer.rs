use std::ops::{Div, Rem, Sub};

use num_traits::{One, Zero};

/// A trait for integers.
pub trait Integer:
    Zero + One + Eq + Div<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + Copy
{
}

impl<T> Integer for T where
    T: Zero + One + Eq + Div<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy
{
}

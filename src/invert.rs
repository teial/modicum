use super::{Constrain, Egcd, Modulus};

pub trait Invert: Egcd {
    fn invert<P: Modulus<Self>>(self, p: P) -> Option<Self>
    where
        Self: TryFrom<P>,
        <Self as TryFrom<P>>::Error: std::fmt::Debug,
    {
        invert(self, p)
    }
}

impl<T: Egcd> Invert for T {}

fn invert<T: Egcd, P: Modulus<T>>(a: T, p: P) -> Option<T>
where
    T: TryFrom<P>,
    <T as TryFrom<P>>::Error: std::fmt::Debug,
{
    let (d, x, _) = a.constrain(p).egcd(p.cast());
    if d != T::one() {
        return None;
    }
    Some(x.constrain(p))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_invert() {
        let modulus = 11u32;
        assert_eq!(invert(3, modulus), Some(4));
        assert_eq!(invert(5, modulus), Some(9));
        assert_eq!(invert(7, modulus), Some(8));
        assert_eq!(invert(9, modulus), Some(5));
        assert_eq!(invert(10, modulus), Some(10));
        assert_eq!(invert(11, modulus), None);
        assert_eq!(invert(0, modulus), None);
        assert_eq!(invert(-3, modulus), Some(7));
        assert_eq!(invert(-5, modulus), Some(2));
        assert_eq!(invert(-7, modulus), Some(3));
        assert_eq!(invert(-9, modulus), Some(6));
        assert_eq!(invert(-10, modulus), Some(1));
        assert_eq!(invert(-11, modulus), None);
    }
}

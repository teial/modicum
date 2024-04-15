use crate::Integer;

pub trait Egcd: Integer {
    fn egcd(self, other: Self) -> (Self, Self, Self) {
        egcd(self, other)
    }
}

impl<T: Integer> Egcd for T {}

fn egcd<T: Integer>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        return (a, T::one(), T::zero());
    }
    let (d, x, y) = egcd(b, a % b);
    (d, y, x - (a / b) * y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_egcd() {
        assert_eq!(egcd(102, 38), (2, 3, -8));
        assert_eq!(egcd(899, 1914), (29, -17, 8));
        assert_eq!(egcd(1432, 123211), (1, -22973, 267));
        assert_eq!(egcd(14, 28), (14, 1, 0));
        assert_eq!(egcd(28, 14), (14, 0, 1));
    }
}

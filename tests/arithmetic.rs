use modicum::*;
use pretty_assertions::assert_eq;

#[test]
fn test_signed() {
    let a = 5i32;
    let b = 3i32;
    let modulus = 7u32;
    assert_eq!(a.add_mod(b, modulus), 1);
    assert_eq!(a.sub_mod(b, modulus), 2);
    assert_eq!(a.mul_mod(b, modulus), 1);
    assert_eq!(a.div_mod(b, modulus), Some(4));
    assert!(a.eq_mod(5, modulus));
    assert!(!a.ne_mod(5, modulus));
    assert!(a.ne_mod(6, modulus));
    assert!(!a.eq_mod(6, modulus));
}

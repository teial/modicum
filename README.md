# A modular arithmetic library in pure Rust

This library provides a set of traits and implementations for modular arithmetic in pure Rust. It is designed to be flexible and easy to use, with a focus on performance. The library is designed to be used in a variety of contexts, including cryptography, error detection and correction, and other applications where modular arithmetic is useful.

## Features

- Modular arithmetic traits for addition, subtraction, multiplication, division, and exponentiation
- Implementations for all types supported by `num_traits` crate

## Limitations

- The library relies on blanket implementation to work for all integers and possbile big integers as well. This may lead to name conflicts if you use this library with other libraries that also implement the same traits for the same types.
- The operands must be of the same signed type. It is possbile that in future versions this restriction will be lifted.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
modular_arithmetic = "0.1"
```

## Example

```rust
use modular_arithmetic::{ModAdd, ModMul, ModPow};

fn main() {
    let a = 5i32;
    let b = 7i32;
    let m = 11u32;

    let sum = a.mod_add(b, m);
    let difference = a.mod_sub(b, m);
    let product = a.mod_mul(b, m);
    let quotient = a.mod_div(b, m);
    let power = a.mod_pow(b, m);

    println!("{} + {} mod {} = {}", a, b, m, sum);
    println!("{} - {} mod {} = {}", a, b, m, difference);
    println!("{} * {} mod {} = {}", a, b, m, product);
    println!("{} / {} mod {} = {}", a, b, m, quotient);
    println!("{} ^ {} mod {} = {}", a, b, m, power);
}
```

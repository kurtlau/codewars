// https://www.codewars.com/kata/57040e445a726387a1001cf7/rust

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::traits::{One, Zero};
use num::Integer;

#[allow(dead_code)]
fn func(a: BigUint, b: BigUint, n: BigUint) -> BigUint {
    if n == BigUint::zero() {
        return b;
    }
    if n == BigUint::one() {
        return a + b;
    }

    if n.is_odd() {
        func(
            a.clone(),
            a + b,
            (n - BigUint::one()) / 2.to_biguint().unwrap(),
        )
    } else {
        func(a + b.clone(), b, n / 2.to_biguint().unwrap())
    }
}

fn fusc(n: BigUint) -> BigUint {
    let mut a = BigUint::one();
    let mut b = BigUint::zero();

    n.to_str_radix(2).chars().for_each(|c| {
        if c == '1' {
            b += a.clone()
        } else {
            a += b.clone()
        }
    });
    b
    // func(BigUint::one(), BigUint::zero(), n)
}

#[cfg(test)]
mod tests {
    use super::fusc;
    use num::bigint::{BigUint, ToBigUint};
    use num::traits::{One, Zero};

    #[test]
    fn sample_tests() {
        assert_eq!(
            fusc(BigUint::zero()),
            0.to_biguint().unwrap(),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            fusc(BigUint::one()),
            1.to_biguint().unwrap(),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            fusc(85.to_biguint().unwrap()),
            21.to_biguint().unwrap(),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            fusc(9007199254740991_u64.to_biguint().unwrap()),
            53.to_biguint().unwrap(),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            fusc(123456789012345_u64.to_biguint().unwrap()),
            31432586.to_biguint().unwrap(),
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}

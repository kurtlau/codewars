use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::traits::{One, Zero};

fn func(a: BigUint, b: BigUint, n: BigUint) -> BigUint {
    if n == BigUint::zero() { return b; }
    if n == BigUint::one() { return a + b; }

    func(a + b.clone(), b, (n + 1.to_biguint().unwrap()) / 2.to_biguint().unwrap())
}

fn fusc(n: BigUint) -> BigUint {
    func(BigUint::one(), BigUint::zero(), n)
}

#[cfg(test)]
mod tests {
    use super::fusc;
    use num::bigint::{BigUint, ToBigUint};
    use num::traits::{One, Zero};

    #[test]
    fn sample_tests() {
        assert_eq!(fusc(BigUint::zero()), 0.to_biguint().unwrap(), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(fusc(BigUint::one()), 1.to_biguint().unwrap(), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(fusc(85.to_biguint().unwrap()), 21.to_biguint().unwrap(), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(fusc(9007199254740991_u64.to_biguint().unwrap()), 53.to_biguint().unwrap(), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(fusc(123456789012345_u64.to_biguint().unwrap()), 31432586.to_biguint().unwrap(), "\nYour answer (left) is not the expected answer (right).");
    }
}

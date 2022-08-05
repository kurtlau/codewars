use num::bigint::BigUint;
use num::bigint::ToBigUint;

fn func(a: BigUint, b: BigUint, n: BigUint) -> BigUint {
    if n == 0.to_biguint().unwrap() { return b; }
    if n == 1.to_biguint().unwrap() { return a + b; }

    if n.clone() % 2.to_biguint().unwrap() != 0.to_biguint().unwrap() {
        func(a.clone(), a + b, (n - 1.to_biguint().unwrap()) / 2.to_biguint().unwrap())
    }
    else {
        func(a + b.clone(), b,  n / 2.to_biguint().unwrap())
    }
}

fn fusc(n: BigUint) -> BigUint {
    func(1.to_biguint().unwrap(), 0.to_biguint().unwrap(), n)
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

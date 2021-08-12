fn is_prime(n: u32) -> bool {
    let sqrt_n = (n as f64).sqrt() as u32;
    (2u32..=sqrt_n).all(|i| n % i != 0)
}

fn minimum_number(xs: &[u32]) -> u32 {
    let sum: u32 = xs.iter().sum();

    for i in 0u32.. {
        if is_prime(sum + i) {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(minimum_number(&[3, 1, 2]), 1);
        assert_eq!(minimum_number(&[5, 2]), 0);
        assert_eq!(minimum_number(&[1, 1, 1]), 0);
        assert_eq!(minimum_number(&[2, 12, 8, 4, 6]), 5);
        assert_eq!(minimum_number(&[50, 39, 49, 6, 17, 28]), 2);
    }
}

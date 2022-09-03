fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fib = (0u64, 1u64);

    while fib.0 * fib.1 < prod {
        fib = (fib.1, fib.0 + fib.1);
    }

    (fib.0, fib.1, fib.0 * fib.1 == prod)
}

#[cfg(test)]
mod test {
    use super::*;
    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}

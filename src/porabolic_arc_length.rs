fn len_curve(n: i32) -> f64 {
    let mut sum = 0f64;
    let nf64 = n as f64;

    for i in 0..n {
        let if64 = i as f64;
        let t = nf64.powi(2) + 4f64 * if64.powi(2) +4f64 * if64 + 1f64;
        sum += t.sqrt();
    }

    sum / nf64.powi(2)
}

use float_eq::float_eq;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-6;
        let res =
            float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(
            res,
            "Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_float_equals(len_curve(n), exp);
    }

    #[test]
    fn basic_tests_len_curve() {
        dotest(1, 1.414213562);
        dotest(10, 1.478197397);
        dotest(40, 1.478896272);
        dotest(200, 1.478940994);
        dotest(1200, 1.478942805);
    }
}

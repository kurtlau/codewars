fn evaporator(_: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let s = std::iter::successors(Some(100f64), |prev| {
        Some(prev * (1f64 - evap_per_day as f64 * 0.01f64))
    });

    for (index, value) in s.enumerate() {
        if (value as i32) < threshold {
            return index as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
    }
}

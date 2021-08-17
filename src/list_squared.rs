fn squared(n: u64) -> Option<(u64, u64)> {
    let sqrt_n = (n as f64).sqrt() as u64;

    let sum = (1..=sqrt_n)
        .map(|i| {
            let div = n / i;
            if div * i == n {
                if div == i {
                    i.pow(2)
                } else {
                    i.pow(2) + div.pow(2)
                }
            } else {
                0u64
            }
        })
        .sum();

    if ((sum as f64).sqrt() as u64).pow(2) == sum {
        Some((n, sum))
    } else {
        None
    }
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n).filter_map(|i| squared(i)).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(list_squared(m, n), exp)
    }

    #[test]
    fn basics_list_squared() {
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(42, 250, vec![(42, 2500), (246, 84100)]);
        testing(300, 600, vec![]);
    }
}

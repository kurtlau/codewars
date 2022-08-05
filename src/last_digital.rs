#[allow(unreachable_code)]

fn calc_last_digital(num: u64, pow: u64) -> u64 {
    if 0 == pow {
        return 1;
    }

    let num = num % 10;
    let digital = num.pow(pow as u32) % 20;
    println!("{}^^{} last digital: {}", num, pow, digital);
    return digital;

    let tab = vec![
        vec![0u64],
        vec![1],
        vec![6, 2, 4, 8],
        vec![1, 3, 9, 7],
        vec![6, 4],
        vec![5],
        vec![6],
        vec![1, 7, 9, 3],
        vec![6, 8, 4, 2],
        vec![1, 9],
    ];

    let n = num % 10;
    let entry = &tab[n as usize];
    let digital = entry[pow as usize % entry.len()];

    println!("{}^^{} last digital: {}", num, pow, digital);
    digital
}

fn last_digit(lst: &[u64]) -> u64 {
    if lst.is_empty() {
        1
    } else {
        let last = lst.last().unwrap();
        lst.iter()
            .rev()
            .skip(1)
            .fold(*last, |pow, &num| calc_last_digital(num, pow))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            // (vec![2, 2, 2, 0], 4),
            // (vec![937640, 767456, 981242], 0),
            // (vec![123232, 694022, 140249], 6),
            // (vec![499942, 898102, 846073], 6),
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}

fn next_bigger_number(n: i64) -> i64 {
    let mut num: Vec<i64> = vec![];
    let mut n = n;

    while n > 0 {
        let rem = n % 10i64;
        n = n / 10i64;
        num.push(rem);
    }

    for i in 1..num.len() {
        for j in 0..i {
            if num[i] < num[j] {
                num.swap(i, j);
                num[..i].sort_by_key(|&v| std::cmp::Reverse(v));
                return num.iter().rev().fold(0i64, |acc, x| acc * 10 + x);
            }
        }
    }

    -1i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(59884848483559, next_bigger_number(59884848459853));
    }
}

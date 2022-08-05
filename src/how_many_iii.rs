// https://www.codewars.com/kata/5877e7d568909e5ff90017e6

fn find(sum: u8, digs: u8, min: u8) -> usize {
    todo!()
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::find_all;

    #[test]
    fn sample_tests() {
        assert_eq!(find_all(10, 3), Some((8, 118, 334)));
        assert_eq!(find_all(27, 3), Some((1, 999, 999)));
        assert_eq!(find_all(84, 4), None);
        assert_eq!(find_all(35, 6), Some((123, 116999, 566666)));
    }
}

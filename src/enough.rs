fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counter = std::collections::HashMap::<u8, usize>::new();

    lst.into_iter().filter_map(|&num| {
        let entry = counter.entry(num).or_default();
        *entry += 1;
        match *entry <= n {
            true => Some(num),
            false => None,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(
            delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3),
            vec![1, 1, 3, 3, 7, 2, 2, 2]
        );
    }
}

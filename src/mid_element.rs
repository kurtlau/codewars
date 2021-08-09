fn gimme(input_array: [i32; 3]) -> usize {
    let mut a = input_array.iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|&(_, &value)| value);
    a[1].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}

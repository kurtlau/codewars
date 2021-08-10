fn get_count(string: &str) -> usize {
    string.chars().filter(|&ch| "aeiou".contains(ch)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(get_count("abracadabra"), 5);
    }
}

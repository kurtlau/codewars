fn reverse_words(str: &str) -> String {
    str.to_string().split(" ")
    .map(|word| word.chars().rev().collect::<String>())
    .collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(
            reverse_words("The quick brown fox jumps over the lazy dog."),
            "ehT kciuq nworb xof spmuj revo eht yzal .god"
        );
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"), "a b c d");
        assert_eq!(
            reverse_words("double  spaced  words"),
            "elbuod  decaps  sdrow"
        );
    }
}

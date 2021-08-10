fn expanded_form(n: u64) -> String {
    n.to_string()
        .chars()
        .rev()
        .zip(0usize..)
        .filter_map(|(ch, digital)| {
            if ch == '0' {
                None
            } else {
                Some(String::from(ch) + "0".repeat(digital).as_str())
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}

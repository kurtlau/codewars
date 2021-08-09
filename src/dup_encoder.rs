use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    let mut code = HashMap::<char, char>::new();

    word.chars().for_each(|ch| {
        code.entry(ch.to_ascii_lowercase())
            .and_modify(|c| {
                *c = ')';
            })
            .or_insert('(');
    });

    word.chars()
        .map(|ch| code.get(&ch.to_ascii_lowercase()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}

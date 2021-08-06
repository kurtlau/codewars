use std::collections::HashSet;

fn disemvowel(s: &str) -> String {
    let vowel = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
        .into_iter()
        .collect::<HashSet<char>>();
    s.chars()
        .into_iter()
        .filter(|c| !vowel.contains(c))
        .collect()
}

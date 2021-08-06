use std::collections::BTreeSet;

fn longest(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}

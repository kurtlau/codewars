#[allow(dead_code)]
fn no_space(x: String) -> String {
    x.chars()
        .into_iter()
        .filter(|&char| !char.is_whitespace())
        .collect()
}

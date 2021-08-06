fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(idx, ch)| {
            ch.to_ascii_uppercase().to_string()
                + ch.to_ascii_lowercase().to_string().repeat(idx).as_str()
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn even_or_odd(i: i32) -> &'static str {
    ["Even", "Odd"][((i as u32) % 2) as usize]
}
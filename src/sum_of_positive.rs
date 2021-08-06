fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|v| v.is_positive()).sum()
}

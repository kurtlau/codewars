fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|&v| v * v).sum()
}

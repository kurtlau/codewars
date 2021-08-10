fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.is_empty() {
        return "No participants";
    }

    let mut with_weight = st
        .split(',')
        .zip(we)
        .map(|(word, weight)| {
            let score = word
                .chars()
                .map(|ch| ch.to_ascii_lowercase() as i32 - 'a' as i32 + 1)
                .sum::<i32>();
            ((word.len() as i32 + score) * weight, word)
        })
        .collect::<Vec<(i32, &str)>>();

    with_weight.sort_by(|a, b| (-a.0, a.1).cmp(&(-b.0, b.1)));
    with_weight
        .iter()
        .nth(n - 1)
        .unwrap_or(&(-1, "Not enough participants"))
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(rank(st, we, n), exp)
    }

    #[test]
    fn basics_rank() {
        testing(
            "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
            vec![4, 2, 1, 4, 3, 1, 2],
            4,
            "Benjamin",
        );
        testing(
            "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
            vec![1, 3, 5, 5, 3, 6],
            2,
            "Matthew",
        );
        testing(
            "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
            vec![3, 1, 4, 4, 3, 2],
            4,
            "Abigail",
        );
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }
}

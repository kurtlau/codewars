use std::cmp::max;

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }

    let mut l1 = a1.iter().map(|&s| s.len() as i32).collect::<Vec<i32>>();
    let mut l2 = a2.iter().map(|&s| s.len() as i32).collect::<Vec<i32>>();

    l1.sort();
    l2.sort();

    max(
        (l1[0] - l2[l2.len() - 1]).abs(),
        (l2[0] - l1[l1.len() - 1]).abs(),
    )
}

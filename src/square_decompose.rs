fn decompose_recursive(target: i64, max: i64) -> Option<Vec<i64>> {
    for i in (1..max).rev() {
        let remain = target - i.pow(2u32);

        if remain == 0 {
            return Some([i].to_vec());
        } else if remain > 0 {
            if let Some(mut result) = decompose_recursive(remain, i) {
                result.push(i);
                return Some(result);
            }
        }
    }

    None
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    decompose_recursive(n.pow(2u32), n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp)
    }

    #[test]
    fn tests_decompose() {
        testing(5, Some(vec![3, 4]));
        testing(50, Some(vec![1, 3, 5, 8, 49]));
        testing(44, Some(vec![2, 3, 5, 7, 43]));
        testing(625, Some(vec![2, 5, 8, 34, 624]));
        testing(5, Some(vec![3, 4]));
    }
}

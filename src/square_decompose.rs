fn decompose_recursive(remain: i64, max: i64, result: &mut Vec<i64>) -> bool {
    for i in (1..max).rev() {
        let left = remain - i.pow(2u32);

        if left == 0 {
            println!("{} fit", &i);
            result.push(i);
            return true; //found
        } else if left > 0 {
            result.push(i);

            if decompose_recursive(left, i, result) {
                return true;
            } else {
                println!("pop {}", &i);
                result.pop();
            }
        }
    }

    false
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut result: Vec<i64> = vec![];

    if decompose_recursive(n.pow(2u32), n, &mut result) {
        result.reverse();
        return Some(result);
    }

    None
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

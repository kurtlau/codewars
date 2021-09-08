mod solution {
    pub fn range_extraction(a: &[i32]) -> String {
        let mut queue: Vec<(i32, i32)> = vec![];

        for &i in a.iter() {
            if let Some(last) = queue.last_mut() {
                if last.0 + last.1 == i {
                    last.1 = last.1 + 1;
                } else {
                    queue.push((i, 1));
                }
            } else {
                queue.push((i, 1));
            }
        }

        queue.iter().map(|(begin, count)| match count {
            1 => begin.to_string(),
            2 => format!("{},{}", begin, begin + 1),
            count => format!("{}-{}", begin, begin + count - 1),
        }).collect::<Vec<_>>().join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]));
        assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]));
    }
}

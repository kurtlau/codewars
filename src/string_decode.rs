fn decode(word: &str) -> String {
    let mut stack = vec![];
    let mut parse_count = false;
    let mut curr = (1usize, String::new());

    for ch in word.chars() {
        match ch {
            '[' => {
                stack.push(std::mem::take(&mut curr));
                parse_count = true;
            }

            '|' => parse_count = false,

            ']' => if let Some(mut top) = stack.pop() {
                top.1.push_str(curr.1.repeat(curr.0).as_str());
                std::mem::swap(&mut curr, &mut top);
            },

            ch => if parse_count {
                curr.0 = curr.0 * 10 + ch.to_digit(10).unwrap() as usize;
            } else {
                curr.1.push(ch);
            },
        }
    }

    curr.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(decode("ABC"), "ABC");
        assert_eq!(decode("A[2|BC]"), "ABCBC");
        assert_eq!(decode("A[3|B[2|CA]]F"), "ABCACABCACABCACAF");
        assert_eq!(decode("[2|ABC]"), "ABCABC");
        assert_eq!(decode("[12|A]"), "AAAAAAAAAAAA");
        assert_eq!(decode("[0|A]"), "");
    }
}

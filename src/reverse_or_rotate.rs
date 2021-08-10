fn revrot(s: &str, c: usize) -> String {
    if s.is_empty() || c == 0 {
        return String::from("");
    }

    if s.len() <= c {
        return String::from("");
    }

    s.as_bytes()
        .chunks(c)
        .filter_map(|chunk| {
            if chunk.len() < c {
                return None;
            }

            let s = std::str::from_utf8(chunk).unwrap();
            let div_by_2 = 0 == s.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>() % 2;

            if !div_by_2 {
                Some(
                    s.chars().skip(1).collect::<String>()
                        + s.chars().take(1).collect::<String>().as_str(),
                )
            } else {
                Some(s.chars().rev().collect())
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;
    fn testing(s: &str, c: usize, exp: &str) -> () {
        assert_eq!(&revrot(s, c), exp)
    }

    #[test]
    fn basics_revrot() {
        testing("1234", 0, "");
        testing("", 0, "");
        testing("1234", 5, "");
        let s = "733049910872815764";
        testing(s, 5, "330479108928157");
        let s = "73304991087281576455176044327690580265896";
        testing(s, 8, "1994033775182780067155464327690480265895");
    }
}

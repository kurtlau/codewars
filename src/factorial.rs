fn dec2_fact_string(nb: u64) -> String {
    let mut n = nb;
    let mut out = String::new();

    for i in 0..36 {
        let d = n / (i + 1);
        let r = n - d * (i + 1);

        out.push(std::char::from_digit(r as u32, 36).unwrap().to_ascii_uppercase());

        n = d;
        if n == 0 { break; }
    }

    out.chars().rev().collect()
}

fn fact_string_2dec(s: String) -> u64 {
    s.chars().rev().fold(
        (0u64, 1u64, 0u64),
        |acc, ch| (acc.0 + 1, (acc.0 + 1) * acc.1, ch.to_ascii_lowercase().to_digit(36).unwrap() as u64 * acc.1 + acc.2)
    ).2
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn testing1(nb: u64, exp: &str) -> () {
        assert_eq!(&dec2_fact_string(nb), exp)
    }

    fn testing2(s: &str, exp: u64) -> () {
        assert_eq!(fact_string_2dec(s.to_string()), exp)
    }

    #[test]
    fn basics_dec2_fact_string() {
        testing1(2982, "4041000");
        testing1(463, "341010");
    }

    #[test]
    fn basics_fact_string_2dec() {
        testing2("110", 3);
        testing2("4041000", 2982);
        testing2("341010", 463);
    }
}
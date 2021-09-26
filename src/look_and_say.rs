fn say(seq: &str) -> String {
    let mut out = String::new();
    let mut temp = ('0', -1);

    for ch in seq.chars() {
        if ch == temp.0 { temp.1 += 1; } else {
            if temp.0 != '0' {
                out.push_str(temp.1.to_string().as_str());
                out.push(temp.0);
            }
            temp = (ch, 1);
        }
    }

    if temp.0 != '0' {
        out.push_str(temp.1.to_string().as_str());
        out.push(temp.0);
    }

    out
}

fn get_lines(n: usize) -> String {
    if n == 0 { return String::new(); }

    let mut out = vec![ String::from("1")];
    for _ in 1..n {
        out.push(say(out.last().unwrap().as_str()));
    }

    out.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(get_lines(2), "1,11");
        assert_eq!(get_lines(3), "1,11,21");
        assert_eq!(get_lines(5), "1,11,21,1211,111221");
    }
}

// https://www.codewars.com/kata/52e864d1ffb6ac25db00017f/train/rust

#[inline]
#[allow(dead_code)]
fn is_right_ass(c: char) -> bool {
    c == '^'
}

#[inline]
#[allow(dead_code)]
fn op_priority(c: char) -> i32 {
    if c == '^' {
        3
    } else if c == '*' || c == '/' {
        2
    } else {
        1
    }
}

fn to_postfix(infix: &str) -> String {
    let mut out = String::new();
    let mut stack = Vec::<char>::new();

    for ch in infix.chars() {
        if ch.is_numeric() {
            out.push(ch);
            // println!("push: {}", ch);
        } else if stack.is_empty() || ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            while let Some(op) = stack.pop() {
                if op == '(' {
                    break;
                }

                out.push(op);
                // println!("push: {}", op);
            }
        } else {
            while let Some(op) = stack.pop() {
                if op == '(' {
                    stack.push(op);
                    break;
                }

                if op_priority(op) < op_priority(ch) {
                    stack.push(op);
                    break;
                }

                if op_priority(op) == op_priority(ch) && is_right_ass(op) {
                    stack.push(op);
                    break;
                }

                out.push(op);
                // println!("push: {}", op);
            }

            stack.push(ch);
        }
    }

    for ch in stack.into_iter().rev() {
        out.push(ch);
        // println!("push: {}", ch);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::to_postfix;

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(
            actual, expected,
            "\nYour answer (left) is not the correct answer (right)"
        )
    }

    #[test]
    fn fixed_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("(7+1)"), "71+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("1^2^3"), "123^^");
    }
}

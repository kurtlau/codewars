// https://zh.wikipedia.org/wiki/%E6%95%B4%E6%95%B8%E5%88%86%E6%8B%86
fn partitions(n: isize) -> isize {
    if n < 0 {
        return 0;
    }

    let n = n as usize;
    let mut caches = vec![0isize; n + 1];
    caches[0] = 1isize;
    for i in 1..n + 1 {
        for j in i..n + 1 {
            caches[j] += caches[j - i];
        }
    }
    caches[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }
}
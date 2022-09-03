fn inner_spiralize(map: &mut Vec<Vec<i8>>, start: usize, size: usize) {
    if size == 0 {
        return;
    }

    for i in 0..size {
        map[start][start + i] = 1;
    }

    for i in 1..size {
        map[start + i][start + size - 1] = 1;
    }

    if size < 3 {
        return;
    }

    for i in 0..size {
        map[start + size - 1][start + size - i - 1] = 1;
    }

    if size < 4 {
        return;
    }

    for i in 2..size - 1 {
        map[start + size - i][start] = 1;
    }

    if size < 5 {
        return;
    }

    map[start + 2][start] = 1;
    map[start + 2][start + 1] = 1;
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut out = vec![vec![0i8; size]; size];

    let mut size = size;
    let mut start = 0;
    while size > 0 {
        inner_spiralize(&mut out, start, size);
        start += 2;
        if size >= 4 {
            size -= 4;
        } else {
            size = 0;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(spiralize(0), Vec::<Vec<i8>>::new());
        assert_eq!(spiralize(1), [[1]]);
        assert_eq!(spiralize(2), [[1, 1], [0, 1]]);
        assert_eq!(spiralize(3), [[1, 1, 1], [0, 0, 1], [1, 1, 1]]);
        assert_eq!(
            spiralize(4),
            [[1, 1, 1, 1], [0, 0, 0, 1], [1, 0, 0, 1], [1, 1, 1, 1]]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}

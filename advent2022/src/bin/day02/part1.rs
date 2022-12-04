pub(crate) fn solve(bytes: &[u8]) -> usize {
    bytes.chunks_exact(4).map(compute_score).sum()
}

fn compute_score(bytes: &[u8]) -> usize {
    let other = bytes[0] - (b'A' - 1);
    let you = bytes[2] - (b'X' - 1);
    (you + play(you, other)) as usize
}

fn play(you: u8, other: u8) -> u8 {
    match you as i8 - other as i8 {
        -1 | 2 => 0,
        0 => 3,
        _ => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let input = include_bytes!("test.txt");
        let res = solve(input);
        assert_eq!(15, res);
    }
}

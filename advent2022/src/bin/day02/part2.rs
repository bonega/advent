use std::ops::Add;

pub(crate) fn solve(bytes: &[u8]) -> usize {
    bytes.chunks_exact(4).map(compute_score).sum()
}

fn compute_score(bytes: &[u8]) -> usize {
    let other = bytes[0] - b'A';
    let mut you = bytes[2] - b'X';
    dbg!(you);
    you = match you {
        0 => other.wrapping_sub(1) % 3,
        1 => other,
        _ => dbg!(other.add(1) % 3),
    };
    (you + 1 + play(you, other)) as usize
}

fn play(you: u8, other: u8) -> u8 {
    match you as i8 - other as i8 {
        -1 | 2 => 0,
        0 => 3,
        _ => 6,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_example() {
//         let input = include_bytes!("test.txt");
//         let res = solve(input);
//         assert_eq!(15, res);
//     }

//     #[test]
//     fn test_solution() {
//         let input = include_bytes!("input.txt");
//         let res = solve(input);
//         assert_eq!(13924, res);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = b"A Y\nB X\nC Z\n";
        let res = solve(input);
        assert_eq!(12, res);
    }

    #[test]
    fn test_solution() {
        let input = include_bytes!("input.txt");
        let res = solve(input);
        assert_eq!(13448, res);
    }
}

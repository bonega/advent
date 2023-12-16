mod part1;
mod part2;

use std::ops::Add;

fn main() {
    let input = include_bytes!("input.txt");
    let score = part1::solve(input);
    println!("Day02 solution part1: {score}");
    let score = part2::solve(input);
    println!("Day02 solution part2: {score}");
}

fn parse(bytes: &[u8]) -> (u8, u8) {
    (bytes[0] - b'A', bytes[2] - b'X')
}

fn score((other, you): (u8, u8)) -> usize {
    (you + 1 + play(you, other)) as usize
}

fn play(you: u8, other: u8) -> u8 {
    match you as i8 - other as i8 {
        -1 | 2 => 0,
        0 => 3,
        _ => 6,
    }
}

fn choose_play((other, you): (u8, u8)) -> (u8, u8) {
    (
        other,
        match you {
            0 => you.wrapping_sub(1) % 3,
            1 => other,
            _ => you.add(1) % 3,
        },
    )
}

fn solve2(bytes: &[u8]) -> usize {
    bytes
        .chunks_exact(4)
        .map(parse)
        .map(choose_play)
        .map(score)
        .sum()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_example() {
//         let input = include_bytes!("test.txt");
//         let res = solve1(input);
//         assert_eq!(15, res);
//     }

//     #[test]
//     fn test_solution() {
//         let input = include_bytes!("input.txt");
//         let res = solve1(input);
//         assert_eq!(13924, res);
//     }
// }

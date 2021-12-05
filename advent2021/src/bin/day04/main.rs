use std::collections::HashSet;

const WIDTH: usize = 5;

fn main() {
    let s = include_str!("input.txt");
    let (numbers, boards) = parse_bingo(s);
    println!("Part1 {}", part1(&numbers, &boards));
    println!("Part2 {}", part2(&numbers, &boards));
}

fn parse_bingo(s: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = s.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect();

    let mut boards = vec![];
    let mut buffer = [0; WIDTH * WIDTH];

    while lines.next().is_some() {
        for (y, line) in lines.by_ref().take(WIDTH).enumerate() {
            for (x, token) in line.split_ascii_whitespace().enumerate() {
                buffer[x + y * WIDTH] = token.parse().unwrap();
            }
        }

        let board = Board { buffer };
        boards.push(board);
    }
    (numbers, boards)
}

#[derive(Debug)]
struct Board {
    buffer: [u8; WIDTH * WIDTH],
}

impl Board {
    fn index(&self, x: usize, y: usize) -> u8 {
        self.buffer[x + y * WIDTH]
    }

    fn possible_combinations(&self) -> Vec<HashSet<u8>> {
        let mut res = Vec::with_capacity(WIDTH * 2);
        for v in self.buffer.chunks(WIDTH) {
            res.push(v.iter().cloned().collect());
        }
        for x in 0..WIDTH {
            let mut row = HashSet::with_capacity(WIDTH);
            for y in 0..WIDTH {
                row.insert(self.index(x, y));
            }
            res.push(row);
        }

        res
    }
}

fn part1(numbers: &[u8], boards: &[Board]) -> usize {
    let mut numbers = numbers.iter();
    let mut board_combos: Vec<_> = boards.iter().map(|b| b.possible_combinations()).collect();

    loop {
        let n = numbers.next().unwrap();
        for combos in board_combos.iter_mut() {
            for combo in combos.iter_mut() {
                combo.remove(n);
            }
            if combos.iter().any(|combo| combo.is_empty()) {
                let unique: HashSet<u8> = combos.iter().cloned().flatten().collect();
                let sum: usize = unique.into_iter().map(|x| x as usize).sum();
                return sum * (*n as usize);
            }
        }
    }
}

fn part2(numbers: &[u8], boards: &[Board]) -> usize {
    let mut numbers = numbers.iter();
    let mut board_combos: Vec<_> = boards.iter().map(|b| b.possible_combinations()).collect();

    let mut last_score = 0;
    while let Some(n) = numbers.next() {
        for combos in board_combos.iter_mut() {
            for combo in combos.iter_mut() {
                combo.remove(n);
            }
            if combos.iter().any(|combo| combo.is_empty()) {
                let unique: HashSet<u8> = combos.iter().cloned().flatten().collect();
                let sum: usize = unique.into_iter().map(|x| x as usize).sum();
                last_score = sum * (*n as usize);
            }
        }
        board_combos.retain(|combos| !combos.iter().any(|combo| combo.is_empty()));
    }
    last_score
}

#[cfg(test)]
mod tests {
    use crate::{parse_bingo, part1, part2};

    const TEST_DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let expected_numbers = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let (numbers, boards) = parse_bingo(TEST_DATA);
        assert_eq!(expected_numbers, &numbers[..]);
        assert_eq!(4512, part1(&numbers, &boards));
    }

    #[test]
    fn test_part2() {
        let (numbers, boards) = parse_bingo(TEST_DATA);
        assert_eq!(1924, part2(&numbers, &boards));
    }
}

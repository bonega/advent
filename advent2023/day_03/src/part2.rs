use std::ops::Range;

struct Board {
    width: usize,
    height: usize,
    board: String,
}

impl Board {
    fn new(board: &str) -> Self {
        let board = board.trim();
        let width = board.find('\n').unwrap();
        Self {
            width,
            height: board.len() / width,
            board: board.replace('\n', ""),
        }
    }
    fn neighbours(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 8] {
        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        offsets.map(|(dx, dy)| {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0
                && new_x <= self.width as isize
                && new_y >= 0
                && new_y <= self.height as isize
            {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        })
    }

    /// returns a tuple of x-range and y
    fn number_indices<'a>(&'a self) -> Vec<(Range<usize>, usize)> {
        let mut result = vec![];
        let re = regex::Regex::new(r"\d+").unwrap();
        for y in 0..self.height {
            let haystack = &self.board[y * self.width..(y + 1) * self.width];
            let indices = re
                .find_iter(haystack)
                .map(|m| (m.start() % self.width..(m.end() - 1) % self.width + 1, y));
            result.extend(indices);
        }
        result
    }
}

pub fn solve(s: &str) -> usize {
    let board = Board::new(s);
    let mut result = 0;
    let number_indices = board.number_indices();
    for (i, c) in board.board.char_indices() {
        if c == '*' {
            let mut numbers_indices_adjacent = vec![];
            let gear_y = i / board.width;
            let gear_x = i % board.width;
            let neighbours = board.neighbours(gear_x, gear_y);
            let neighbours = neighbours.iter().flatten();

            for (x_range, y) in number_indices.iter() {
                if neighbours
                    .clone()
                    .any(|(gear_neighbour_x, gear_neighbour_y)| {
                        x_range.contains(gear_neighbour_x) && y == gear_neighbour_y
                    })
                {
                    let number = &board.board
                        [x_range.start + y * board.width..x_range.end + y * board.width];
                    numbers_indices_adjacent.push(number);
                }
            }
            if numbers_indices_adjacent.len() == 2 {
                result += numbers_indices_adjacent
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .product::<usize>();
            }
        }
    }
    result
}

#[test]
fn it_solves_example() {
    let input = include_str!("example.txt");

    assert_eq!(467835, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(87449461, solve(input));
}

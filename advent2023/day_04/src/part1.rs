use std::collections::HashSet;

fn numbers_to_hashset(s: &str) -> HashSet<usize> {
    s.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

pub fn solve(s: &str) -> usize {
    let mut result = 0;
    for line in s.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let (winning_numbers, given_numbers) = rest.split_once('|').unwrap();
        let winning_numbers = numbers_to_hashset(winning_numbers);
        let given_numbers = numbers_to_hashset(given_numbers);
        let nr_winners = winning_numbers.intersection(&given_numbers).count();
        if nr_winners != 0 {
            result += 2usize.pow(nr_winners as u32 - 1);
        }
    }
    result
}

#[test]
fn it_solves_example() {
    let input = include_str!("example.txt");

    assert_eq!(13, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(20855, solve(input));
}

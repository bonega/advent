use std::collections::{HashSet, VecDeque};

fn numbers_to_hashset(s: &str) -> HashSet<usize> {
    s.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn line_to_nr_winners(s: &str) -> usize {
    let (_, rest) = s.split_once(':').unwrap();
    let (winning_numbers, given_numbers) = rest.split_once('|').unwrap();
    let winning_numbers = numbers_to_hashset(winning_numbers);
    let given_numbers = numbers_to_hashset(given_numbers);
    winning_numbers.intersection(&given_numbers).count()
}

pub fn solve(s: &str) -> usize {
    let mut result = VecDeque::new();
    for line in s.lines().rev() {
        let nr_winners = line_to_nr_winners(line);
        let sum_of_cards_selected = result.iter().take(nr_winners).sum::<usize>();
        result.push_front(sum_of_cards_selected + 1);
    }
    result.into_iter().sum()
}

#[test]
fn it_solves_example() {
    let input = include_str!("example.txt");

    assert_eq!(30, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(5489600, solve(input));
}

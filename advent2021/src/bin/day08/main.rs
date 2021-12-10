use std::collections::BTreeSet;

fn main() {
    let s = include_str!("input.txt");
    let entries: Vec<Entry> = parse(s);
    println!("Part1: {}", part1(&entries));
    println!("Part2: {}", part2(&entries));
}

type Digit = BTreeSet<char>;

#[derive(Debug)]
struct Entry {
    inputs: Vec<Digit>,
    outputs: Vec<Digit>,
}

impl Entry {
    fn parse(input: &str) -> Self {
        let (input, output) = input.trim().split_once('|').unwrap();
        let inputs = input
            .trim()
            .split_whitespace()
            .map(|token| token.chars().collect())
            .collect();
        let outputs = output
            .trim()
            .split_whitespace()
            .map(|token| token.chars().collect())
            .collect();
        Self { inputs, outputs }
    }

    fn digits(&self) -> Vec<Digit> {
        let mut res: Vec<_> = self
            .inputs
            .iter()
            .cloned()
            .chain(self.outputs.iter().cloned())
            .collect();
        res.sort();
        res.dedup();
        res
    }
}

fn parse(s: &str) -> Vec<Entry> {
    s.lines().map(|line| Entry::parse(line)).collect()
}

fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|e| {
            e.outputs.iter().filter(|d| match d.len() {
                2 | 4 | 7 | 3 => true,
                _ => false,
            })
        })
        .count()
}

fn part2(entries: &[Entry]) -> usize {
    let mut sum = 0;
    for ent in entries {
        let digs = ent.digits();

        let one = digs.iter().find(|d| d.len() == 2).unwrap();
        let four = digs.iter().find(|d| d.len() == 4).unwrap();
        let eight = digs.iter().find(|d| d.len() == 7).unwrap();
        let seven = digs.iter().find(|d| d.len() == 3).unwrap();
        let three = digs
            .iter()
            .find(|d| one.is_subset(&d) && d.len() == 5)
            .unwrap();
        let nine = digs
            .iter()
            .find(|d| four.is_subset(&d) && d.len() == 6)
            .unwrap();
        let a = *seven.difference(&one).next().unwrap();
        let b = *nine.difference(&three).next().unwrap();
        let d = *four.difference(&one).filter(|c| **c != b).next().unwrap();
        let e = *eight.difference(&nine).next().unwrap();
        let four_n_seven = four.iter().chain(seven).cloned().collect();
        let g = *nine.difference(&four_n_seven).next().unwrap();
        let mask_2 = Digit::from([a, d, e, g]);
        let two = digs
            .iter()
            .find(|d| mask_2.is_subset(&d) && d.len() == 5)
            .unwrap();
        let c = *two.difference(&mask_2).next().unwrap();
        let f = *one.difference(&two).next().unwrap();
        let five = Digit::from([a, b, d, f, g]);
        let six = Digit::from([a, b, d, e, f, g]);
        let zero = Digit::from([a, b, c, e, f, g]);
        sum += ent
            .outputs
            .iter()
            .map(|d| match d {
                d if *d == zero => 0,
                d if d == one => 1,
                d if d == two => 2,
                d if d == three => 3,
                d if d == four => 4,
                d if *d == five => 5,
                d if *d == six => 6,
                d if d == seven => 7,
                d if d == eight => 8,
                d if d == nine => 9,
                _ => panic!(),
            })
            .fold(0, |acc, b| acc * 10 + b);
    }
    sum
}

// d1 ->           CF ->   L2
// d7 ->           ACF ->  L3
// d4 ->           BCDF -> L4
// (d2 | d3 | d5) -> ADG & (CE || CF || BF) ->  L5
// (d0 | d6 | d9) -> ABFG & (CE || DE || CD) -> L6

// (d2 | d3 | d5) & d1 -> d3
// (d2 | d3 | d5) & d7 -> d3
// (d0 | d6 | d9) & d4 -> d9

#[cfg(test)]
mod test {

    use crate::{parse, part1, part2};
    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse() {
        let entries = parse(INPUT);
        assert_eq!(10, entries.len());
    }

    #[test]
    fn test_part1() {
        let entries = parse(INPUT);
        assert_eq!(26, part1(&entries));
    }

    #[test]
    fn test_part2() {
        let entries = parse(INPUT);
        assert_eq!(61229, part2(&entries));
    }
}

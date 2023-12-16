fn main() {
    let input = include_str!("input.txt");
    let res1 = solve1(input);
    println!("part1 solution: {res1}");
    let res2 = solve2(input);
    println!("part2 solution: {res2}");
}

fn solve1(s: &str) -> u64 {
    *parse(s).iter().max().unwrap()
}

fn solve2(s: &str) -> u64 {
    let mut nums = parse(s);
    nums.sort();
    nums.iter().rev().take(3).sum()
}

fn parse(s: &str) -> Vec<u64> {
    let mut res = vec![];
    let mut sum = 0;
    for line in s.lines() {
        if line.is_empty() {
            res.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u64>().unwrap();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let input = include_str!("test.txt");
        let res = solve1(input);
        assert_eq!(24000, res);
    }
}

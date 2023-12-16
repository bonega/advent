use nom::{
    self,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::many0,
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let res = solve(input);
    println!("Day01 solution: {res}");
}

fn solve(s: &str) -> u64 {
    *parse_elves(s).iter().max().unwrap()
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_calories(s: &str) -> IResult<&str, u64> {
    let calories = many0(terminated(parse_number, newline));
    nom::combinator::map(calories, |x: Vec<u64>| x.iter().sum())(s)
}

fn parse_elves(s: &str) -> Vec<u64> {
    many0(terminated(parse_calories, newline))(s).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let input = include_str!("test.txt");
        let res = solve(input);
        assert_eq!(24000, res);
    }
}

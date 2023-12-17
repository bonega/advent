use regex::Regex;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"\d").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let mut matches = re.find_iter(line);
        let mut integers = matches.next().unwrap().as_str().to_owned();
        if let Some(m) = matches.last() {
            integers.push_str(m.as_str());
        } else {
            integers.push_str(&integers.clone());
        }
        result += integers.parse::<usize>().unwrap();
    }
    result
}

#[test]
fn it_solves_example() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";

    assert_eq!(142, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(54573, solve(input));
}

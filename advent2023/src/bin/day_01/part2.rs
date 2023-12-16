pub fn find_first_and_last_number_optimized(s: &str) -> usize {
    let mut first_number = None;
    let mut last_number = None;
    for start in 0..s.len() {
        let first_match = match &s[start..start + 1] {
            "1" => Some(1),
            "2" => Some(2),
            "3" => Some(3),
            "4" => Some(4),
            "5" => Some(5),
            "6" => Some(6),
            "7" => Some(7),
            "8" => Some(8),
            "9" => Some(9),
            _ => None,
        };
        if let Some(digit) = first_match {
            if first_number.is_none() {
                first_number = Some(digit)
            } else {
                last_number = Some(digit);
            }
            continue;
        }
        if start + 3 > s.len() {
            continue;
        }
        let second_match = match &s[start..start + 3] {
            "one" => Some(1),
            "two" => Some(2),
            "six" => Some(6),
            _ => None,
        };
        if let Some(digit) = second_match {
            if first_number.is_none() {
                first_number = Some(digit)
            } else {
                last_number = Some(digit);
            }
            continue;
        }

        if start + 4 > s.len() {
            continue;
        }
        let third_match = match &s[start..start + 4] {
            "four" => Some(4),
            "five" => Some(5),
            "nine" => Some(9),
            _ => None,
        };
        if let Some(digit) = third_match {
            if first_number.is_none() {
                first_number = Some(digit)
            } else {
                last_number = Some(digit);
            }

            continue;
        }

        if start + 5 > s.len() {
            continue;
        }
        let fourth_match = match &s[start..start + 5] {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None,
        };

        if let Some(digit) = fourth_match {
            if first_number.is_none() {
                first_number = Some(digit)
            } else {
                last_number = Some(digit);
            }
            continue;
        }
    }

    let first_number = first_number.unwrap();
    first_number * 10 + last_number.unwrap_or(first_number)
}

pub fn find_first_and_last_number(s: &str) -> usize {
    let mut tokens = vec![];
    for start in 0..s.len() {
        for end in start..s.len() + 1 {
            let token = match &s[start..end] {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => continue,
            };
            tokens.push(token);
        }
    }

    let first_token: usize = tokens[0];
    first_token * 10 + tokens.last().unwrap_or(&first_token)
}

pub fn solve(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        result += find_first_and_last_number(line);
        // result += find_first_and_last_number_optimized(line);
    }
    result
}

#[test]
fn it_solves_example() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    assert_eq!(281, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(54591, solve(input));
}

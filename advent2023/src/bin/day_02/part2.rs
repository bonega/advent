pub fn solve(s: &str) -> usize {
    let mut result = 0;
    for line in s.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let draws = rest.split(';');
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in draws {
            let color_sections = draw.split(',');
            for color_section in color_sections {
                let (count, color) = color_section.trim().split_once(' ').unwrap();
                let count: usize = count.parse().unwrap();
                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => unreachable!(),
                }
            }
        }
        result += min_red * min_green * min_blue;
    }
    result
}

#[test]
fn it_solves_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(2286, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(62811, solve(input));
}

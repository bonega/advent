pub fn solve(s: &str) -> usize {
    let mut result = 0;
    'read_line: for line in s.lines() {
        let (header, rest) = line.split_once(':').unwrap();
        let game_id: usize = header[5..].parse().unwrap();
        let draws = rest.split(';');
        for draw in draws {
            let color_sections = draw.split(',');
            for color_section in color_sections {
                let (count, color) = color_section.trim().split_once(' ').unwrap();
                let count: usize = count.parse().unwrap();
                match color {
                    "red" if count > 12 => continue 'read_line,
                    "green" if count > 13 => continue 'read_line,
                    "blue" if count > 14 => continue 'read_line,
                    _ => {}
                }
            }
        }
        result += game_id;
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

    assert_eq!(8, solve(input));
}

#[test]
fn it_solves_problem() {
    let input = include_str!("input.txt");
    assert_eq!(2551, solve(input));
}

mod part1;

fn main() {
    let input = include_bytes!("input.txt");
    let score = part1::solve(input);
    println!("Day02 solution part1: {score}");
}

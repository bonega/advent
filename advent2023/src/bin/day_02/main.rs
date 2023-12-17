mod part1;
mod part2;
pub fn main() {
    let input = include_str!("input.txt");
    let part1_solution = part1::solve(input);
    println!("Part1: {part1_solution}");
    let part2_solution = part2::solve(input);
    println!("Part2: {part2_solution}");
}

fn main() {
    let s = include_str!("input.txt");
    let instructions = parse_input(s);
    println!("Problem1: {}", problem1(&instructions));
    println!("Problem2: {}", problem2(&instructions));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse_input(s: &str) -> Vec<Instruction> {
    let mut res = vec![];
    for line in s.lines() {
        let mut tokens = line.split(' ');
        let dir = tokens.next().unwrap();
        let n = tokens.next().unwrap().parse().unwrap();
        res.push(match dir {
            "forward" => Instruction::Forward(n),
            "down" => Instruction::Down(n),
            "up" => Instruction::Up(n),
            _ => unreachable!(),
        });
    }
    res
}

fn problem1(instructions: &[Instruction]) -> usize {
    let mut hor = 0;
    let mut depth = 0;
    for instr in instructions {
        match instr {
            Instruction::Forward(x) => hor += x,
            Instruction::Down(x) => depth += x,
            Instruction::Up(x) => depth -= x,
        }
    }
    hor * depth
}

fn problem2(instructions: &[Instruction]) -> usize {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instr in instructions {
        match instr {
            Instruction::Forward(x) => {
                hor += x;
                depth += aim * x;
            }
            Instruction::Down(x) => aim += x,
            Instruction::Up(x) => aim -= x,
        }
    }
    hor * depth
}

#[cfg(test)]
mod test {
    use crate::Instruction::{self, *};
    const INSTRUCTIONS: &[Instruction] =
        &[Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];

    #[test]
    fn problem1() {
        assert_eq!(150, crate::problem1(INSTRUCTIONS));
    }

    #[test]
    fn problem2() {
        assert_eq!(900, crate::problem2(INSTRUCTIONS));
    }

    #[test]
    fn parse() {
        let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(INSTRUCTIONS, crate::parse_input(s));
    }
}

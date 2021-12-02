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

#[test]
fn test_problem1() {
    use Instruction::*;
    let instructions = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
    assert_eq!(150, problem1(&instructions));
}

#[test]
fn test_problem2() {
    use Instruction::*;
    let instructions = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
    assert_eq!(900, problem2(&instructions));
}

#[test]
fn test_parse() {
    use Instruction::*;
    let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let result = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
    assert_eq!(result, parse_input(s));
}

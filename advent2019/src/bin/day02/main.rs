fn main() {
    let s = include_str!("input.txt");
    let mem = parse(s);
    println!("Part1: {}", part1(&mem));
    println!("Part2: {}", part2(&mem));
}

fn parse(s: &str) -> Vec<usize> {
    s.trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}

fn execute(mem: &mut [usize]) {
    let mut ip = 0;
    let mut instruction = mem[ip];
    while instruction != 99 {
        let par1 = mem[mem[ip + 1]];
        let par2 = mem[mem[ip + 2]];
        let out = mem[ip + 3];
        match instruction {
            1 => mem[out] = par1 + par2,
            2 => mem[out] = par1 * par2,
            _ => panic!(),
        }
        ip += 4;
        instruction = mem[ip];
    }
}

fn part1(mem: &[usize]) -> usize {
    let mut mem = mem.to_vec();
    mem[1] = 12;
    mem[2] = 2;
    execute(&mut mem);
    mem[0]
}

fn part2(mem_original: &[usize]) -> usize {
    let mut mem = mem_original.to_vec();
    loop {
        for i in 0..100 {
            for j in 0..100 {
                mem[1] = i;
                mem[2] = j;
                execute(&mut mem);
                if mem[0] == 19690720 {
                    return 100 * mem[1] + mem[2];
                } else {
                    mem.copy_from_slice(mem_original)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::execute;

    #[test]
    fn parse() {
        let s = "1,9,10,3,2,3,11,0,99,30,40,50";
        let positions = crate::parse(s);
        assert_eq!([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], &positions[..]);
    }

    #[test]
    fn part1() {
        let mut mem = vec![1, 0, 0, 0, 99];
        execute(&mut mem);
        assert_eq!([2, 0, 0, 0, 99], &mem[..]);

        let mut mem = vec![2, 3, 0, 3, 99];
        execute(&mut mem);
        assert_eq!([2, 3, 0, 6, 99], &mem[..]);

        let mut mem = vec![2, 4, 4, 5, 99, 0];
        execute(&mut mem);
        assert_eq!([2, 4, 4, 5, 99, 9801], &mem[..]);

        let mut mem = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut mem);
        assert_eq!([30, 1, 1, 4, 2, 5, 6, 0, 99], &mem[..]);
    }
}

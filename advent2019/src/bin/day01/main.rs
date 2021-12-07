fn main() {
    let s = include_str!("input.txt");
    let masses:Vec<isize> = s.lines().map(|line| line.parse().unwrap()).collect();
    println!("Part1: {}", part1(&masses));
    println!("Part2: {}", part2(&masses));
}

fn part1(masses: &[isize]) -> isize {
    masses.iter().map(|&m| fuel(m)).sum()

}

fn part2(masses: &[isize]) -> isize {
    masses.iter().map(|&m| fuel_per_module(m)).sum()
}

fn fuel_per_module(mass: isize) -> isize {
    let mut fuel_req = fuel(mass);
    let mut res = 0;
    while fuel_req > 0 {
        res += fuel_req;
        fuel_req = fuel(fuel_req);
    }
    res
}

fn fuel(mass: isize) -> isize {
    mass / 3 - 2
}

#[cfg(test)]
mod test {

    #[test]
    fn fuel() {
        assert_eq!(2, crate::fuel(12));
        assert_eq!(2, crate::fuel(14));
        assert_eq!(654, crate::fuel(1969));
        assert_eq!(33583, crate::fuel(100756))
    }

    #[test]
    fn part2() {
        assert_eq!(966, crate::fuel_per_module(1969));
        assert_eq!(50346, crate::fuel_per_module(100756));
    }
}

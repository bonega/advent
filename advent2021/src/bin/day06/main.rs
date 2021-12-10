fn main() {
    let s = include_str!("input.txt");
    let population = parse(s);
    println!("Part1: {}", solve(population.clone(), 80));
    println!("Part2: {}", solve(population, 256));
}

type Population = [usize; 9];

fn solve(mut population: Population, generations: usize) -> usize {
    for _ in 0..generations {
        tick(&mut population)
    }
    population.iter().sum()
}

fn parse(s: &str) -> Population {
    let mut res = Population::default();
    for tok in s.trim().split(',') {
        let i: usize = tok.parse().unwrap();
        res[i] += 1;
    }
    res
}

fn tick(population: &mut Population) {
    population.rotate_left(1);
    population[6] += population[8];
}

#[cfg(test)]
mod test {

    const TEST_DATA: &str = "3,4,3,1,2";
    #[test]
    fn parse() {
        assert_eq!([0, 1, 1, 2, 1, 0, 0, 0, 0], crate::parse(TEST_DATA));
    }

    #[test]
    fn tick() {
        let mut population = crate::parse(TEST_DATA);
        for _ in 0..18 {
            crate::tick(&mut population);
        }
        assert_eq!(26, population.iter().sum::<usize>());
        for _ in 0..62 {
            crate::tick(&mut population);
        }
        assert_eq!(5934, population.iter().sum::<usize>());
        for _ in 0..176 {
            crate::tick(&mut population);
        }
        assert_eq!(26984457539, population.iter().sum::<usize>());
    }
}

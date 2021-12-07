fn main() {
    let s = include_str!("input.txt");
    let crabs = parse(s);
    println!("Part1: {}", part1(&crabs));
    println!("Part1: {}", part2(&crabs));
}

fn parse(s: &str) -> Vec<isize> {
    s.trim()
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect()
}

fn distance(i: isize, crabs: &[isize]) -> isize {
    crabs.iter().map(|&c| (c - i).abs()).sum()
}

fn distance_2(i: isize, crabs: &[isize]) -> isize {
    crabs
        .iter()
        .map(|&c| {
            let n = (c - i).abs();
            n * (n + 1) / 2
        })
        .sum()
}

fn part1(crabs: &[isize]) -> isize {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut smallest_distance = isize::MAX;
    let mut found_position = None;
    for i in min..max {
        let dist = distance(i, crabs);
        if dist < smallest_distance {
            smallest_distance = dist;
            let _ = found_position.insert(i);
        }
    }
    distance(found_position.unwrap(), crabs)
}

fn part2(crabs: &[isize]) -> isize {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut smallest_distance = isize::MAX;
    let mut found_position = None;
    for i in min..max {
        let dist = distance_2(i, crabs);
        if dist < smallest_distance {
            smallest_distance = dist;
            let _ = found_position.insert(i);
        }
    }
    distance_2(found_position.unwrap(), crabs)
}

#[cfg(test)]
mod test {

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn parse() {
        assert_eq!([16, 1, 2, 0, 4, 2, 7, 1, 2, 14], &crate::parse(INPUT)[..]);
    }

    #[test]
    fn part1() {
        let crabs = crate::parse(INPUT);
        assert_eq!(37, crate::part1(&crabs));
    }

    #[test]
    fn distance() {
        let crabs = crate::parse(INPUT);
        assert_eq!(37, crate::distance(2, &crabs));
        assert_eq!(41, crate::distance(1, &crabs));
        assert_eq!(71, crate::distance(10, &crabs));
    }

    #[test]
    fn distance_2() {
        let crabs = crate::parse(INPUT);
        assert_eq!(168, crate::distance_2(5, &crabs));
        assert_eq!(206, crate::distance_2(2, &crabs))
    }
}

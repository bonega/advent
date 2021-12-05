use std::collections::HashMap;

use regex::Regex;

type Pos = (usize, usize);

#[derive(Debug)]
struct Segment {
    start: Pos,
    end: Pos,
}

impl Segment {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let m = re.captures(s.lines().next().unwrap()).unwrap();
        let start = (m[1].parse().unwrap(), m[2].parse().unwrap());
        let end = (m[3].parse().unwrap(), m[4].parse().unwrap());
        Self { start, end }
    }

    fn to_points(&self) -> Vec<Pos> {
        let mut res = vec![];
        let x_step: isize = match self.start.0.cmp(&self.end.0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let y_step: isize = match self.start.1.cmp(&self.end.1) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let mut x = self.start.0;
        let mut y = self.start.1;
        loop {
            res.push((x, y));
            if x == self.end.0 && y == self.end.1 {
                break;
            }
            x = (x as isize + x_step) as usize;
            y = (y as isize + y_step) as usize;
        }
        res
    }
}

fn parse(s: &str) -> Vec<Segment> {
    let segments = s.lines().map(Segment::new).collect();
    segments
}

fn count_intersections(points: &[Pos]) -> usize {
    let mut inter_map: HashMap<Pos, usize> = HashMap::new();
    for point in points {
        let entry = inter_map.entry(*point).or_default();
        *entry += 1;
    }
    inter_map.values().filter(|v| **v > 1).count()
}

fn main() {
    let s = include_str!("input.txt");
    let segments = parse(s);
    println!("Part1: {}", part1(&segments));
    println!("Part2: {}", part2(&segments));
}

fn part1(segments: &[Segment]) -> usize {
    let points: Vec<Pos> = segments
        .iter()
        .filter(|seg| !(seg.start.0 != seg.end.0 && seg.start.1 != seg.end.1))
        .flat_map(|s| s.to_points())
        .collect();
    count_intersections(&points)
}

fn part2(segments: &[Segment]) -> usize {
    let points: Vec<Pos> = segments.iter().flat_map(|s| s.to_points()).collect();
    count_intersections(&points)
}

#[cfg(test)]
mod test {
    use crate::Segment;

    const TEST_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    #[test]
    fn to_points() {
        assert_eq!(
            [(1, 1), (1, 2), (1, 3)],
            Segment {
                start: (1, 1),
                end: (1, 3)
            }
            .to_points()[..]
        );
        assert_eq!(
            [(9, 7), (8, 7), (7, 7)],
            Segment {
                start: (9, 7),
                end: (7, 7)
            }
            .to_points()[..]
        );
    }

    #[test]
    fn part1() {
        let segments = crate::parse(TEST_DATA);
        assert_eq!(5, crate::part1(&segments));
    }

    #[test]
    fn part2() {
        let segments = crate::parse(TEST_DATA);
        assert_eq!(12, crate::part2(&segments));
    }
}

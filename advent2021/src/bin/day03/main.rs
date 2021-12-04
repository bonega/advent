fn main() {
    let s = include_str!("input.txt");
    println!("Part1: {}", part1(s));
    println!("Part2: {}", part2(s));
}

pub fn part1(s: &str) -> usize {
    let row_len = s.lines().next().unwrap().len();
    let mut weights = vec![0isize; row_len];
    for row in s.as_bytes().chunks(row_len + 1) {
        for (i, b) in row.iter().take(row_len).enumerate() {
            weights[i] += match b {
                b'1' => 1,
                b'0' => -1,
                _ => unreachable!(),
            }
        }
    }

    let mut div = 1 << (row_len - 1);
    let mut gamma = 0;
    let mut epsilon = 0;
    for weight in weights {
        match weight.is_positive() {
            true => gamma += div,
            false => epsilon += div,
        };
        div /= 2;
    }
    gamma * epsilon
}

fn bytes_to_binary(bytes: &[u8]) -> usize {
    usize::from_str_radix(std::str::from_utf8(bytes).unwrap(), 2).unwrap()
}

fn part2(s: &str) -> usize {
    let col_len = s.lines().next().unwrap().len();
    let bytes = s.as_bytes();

    let candidates_len = s.lines().count();
    let index = |col, row| row * (col_len + 1) + col;

    let filter_candidates = |f: &dyn Fn(isize) -> bool| {
        let mut pos = 0;
        let mut candidates: Vec<_> = (0..candidates_len).collect();
        while candidates.len() > 1 {
            let weight: isize = candidates
                .iter()
                .map(|&row| match bytes[index(pos, row)] {
                    b'1' => 1,
                    b'0' => -1,
                    _ => unreachable!(),
                })
                .sum();
            candidates.retain(|row| {
                bytes[index(pos, *row)]
                    == match f(weight) {
                        true => b'1',
                        false => b'0',
                    }
            });
            pos += 1;
        }
        *candidates.first().unwrap()
    };
    let oxy_index = filter_candidates(&|x| x >= 0);
    let co2_index = filter_candidates(&|x| x.is_negative());
    let oxy = bytes_to_binary(&bytes[index(0, oxy_index)..index(col_len, oxy_index)]);
    let co2 = bytes_to_binary(&bytes[index(0, co2_index)..index(col_len, co2_index)]);
    oxy * co2
}

#[test]
fn test_part1() {
    let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(198, part1(s));
}

#[test]
fn test_part2() {
    let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(230, part2(s));
}

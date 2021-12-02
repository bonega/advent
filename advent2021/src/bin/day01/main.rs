use anyhow::Result;

fn problem1(data: &[usize]) -> usize {
    data.windows(2).filter(|data| data[1] > data[0]).count()
}
fn problem2(data: &[usize]) -> usize {
    let summed: Vec<usize> = data.windows(3).map(|data| data.iter().sum()).collect();
    summed.windows(2).filter(|data| data[1] > data[0]).count()
}
fn main() -> Result<()> {
    let data1 = include_str!("input.txt");
    let measures = data1
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()?;
    println!("Problem1: {}", problem1(&measures));
    println!("Problem2: {}", problem2(&measures));
    Ok(())
}

#[test]
fn test() {
    let data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, problem1(&data));
}

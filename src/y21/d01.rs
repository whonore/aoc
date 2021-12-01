fn part1(depths: &[u64]) -> usize {
    depths.windows(2).filter(|ds| ds[0] < ds[1]).count()
}

fn part2(depths: &[u64]) -> usize {
    depths
        .windows(3)
        .map(|ds| ds.iter().sum())
        .collect::<Vec<u64>>()
        .as_slice()
        .windows(2)
        .filter(|ds| ds[0] < ds[1])
        .count()
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d01.txt");
    let depths = input
        .lines()
        .map(|d| d.parse().unwrap())
        .collect::<Vec<_>>();
    let out1 = part1(&depths);
    let out2 = part2(&depths);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(&depths), 7);
    }

    #[test]
    fn test02() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part2(&depths), 5);
    }
}

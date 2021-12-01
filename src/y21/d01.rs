fn part1(depths: &[u64]) -> usize {
    depths.windows(2).filter(|xs| xs[0] < xs[1]).count()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d01.txt");
    let depths = input.lines().map(|d| d.parse().unwrap()).collect::<Vec<_>>();
    let out1 = part1(&depths);
    let out2 = part2();
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
}

fn median(pos: &mut [u64]) -> u64 {
    pos.sort_unstable();
    pos[pos.len() / 2]
}

fn part1(pos: &mut [u64]) -> u64 {
    let c = median(pos);
    pos.iter().map(|&x| if x > c { x - c } else { c - x }).sum()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d07.txt");
    let pos = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Invalid position")?;
    let out1 = part1(&mut pos.clone());
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut pos = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part1(&mut pos), 37);
    }
}

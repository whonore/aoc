fn median(pos: &mut [u64]) -> u64 {
    pos.sort_unstable();
    pos[pos.len() / 2]
}

fn part1(pos: &mut [u64]) -> u64 {
    let c = median(pos);
    pos.iter().map(|&x| if x > c { x - c } else { c - x }).sum()
}

fn dist(x: u64, y: u64) -> u64 {
    assert!(x <= y);
    let n = y - x;
    n * (n + 1) / 2
}

fn part2(pos: &[u64]) -> u64 {
    (*pos.iter().min().unwrap()..*pos.iter().max().unwrap())
        .map(|c| {
            pos.iter()
                .map(|&x| if x > c { dist(c, x) } else { dist(x, c) })
                .sum()
        })
        .min()
        .unwrap()
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
    let out2 = part2(&pos);
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

    #[test]
    fn test02() {
        let pos = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part2(&pos), 168);
    }
}

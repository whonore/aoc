fn part1(heights: &[Vec<u32>]) -> u64 {
    heights
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &h)| {
                [
                    r.checked_sub(1).zip(Some(c)),
                    Some((r + 1, c)),
                    Some(r).zip(c.checked_sub(1)),
                    Some((r, c + 1)),
                ]
                .iter()
                .filter_map(|n| *n)
                .filter_map(|(r, c)| heights.get(r).and_then(|row| row.get(c)))
                .copied()
                .all(|n| h < n)
                .then(|| u64::from(h) + 1)
            })
        })
        .sum()
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d09.txt");
    let heights = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let out1 = part1(&heights);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let heights = [
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(part1(&heights), 15);
    }
}

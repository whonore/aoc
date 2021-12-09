use std::collections::HashSet;

use itertools::Itertools;

fn neighbors(r: usize, c: usize) -> Vec<(usize, usize)> {
    [
        r.checked_sub(1).zip(Some(c)),
        Some((r + 1, c)),
        Some(r).zip(c.checked_sub(1)),
        Some((r, c + 1)),
    ]
    .iter()
    .copied()
    .flatten()
    .collect()
}

fn find_low(heights: &[Vec<u32>]) -> Vec<(usize, usize)> {
    heights
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &h)| {
                neighbors(r, c)
                    .iter()
                    .filter_map(|&(r, c)| heights.get(r).and_then(|row| row.get(c)))
                    .copied()
                    .all(|n| h < n)
                    .then(|| (r, c))
            })
        })
        .collect()
}

fn find_basin(heights: &[Vec<u32>], low: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut basin = [low].iter().copied().collect::<HashSet<(usize, usize)>>();
    let ns = neighbors(low.0, low.1);
    let mut unchecked = ns.iter().copied().collect::<HashSet<(usize, usize)>>();

    while !unchecked.is_empty() {
        let &(r, c) = unchecked.iter().next().unwrap();
        unchecked.remove(&(r, c));
        if heights
            .get(r)
            .and_then(|row| row.get(c))
            .copied()
            .map_or(false, |h| h != 9)
        {
            basin.insert((r, c));
            for n in neighbors(r, c).iter().filter(|n| !basin.contains(n)) {
                unchecked.insert(*n);
            }
        }
    }
    basin
}

fn part1(heights: &[Vec<u32>]) -> u64 {
    find_low(heights)
        .iter()
        .map(|&(r, c)| u64::from(heights[r][c]) + 1)
        .sum()
}

fn part2(heights: &[Vec<u32>]) -> usize {
    let lows = find_low(heights);
    let basins = lows.iter().map(|&low| find_basin(heights, low));
    basins
        .map(|basin| basin.len())
        .sorted_by(|x, y| x.cmp(y).reverse())
        .take(3)
        .product()
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
    let out2 = part2(&heights);
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

    #[test]
    fn test02() {
        let heights = [
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(part2(&heights), 1134);
    }
}

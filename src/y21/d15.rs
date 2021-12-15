use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PointDist((usize, usize), u64);

impl Ord for PointDist {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for PointDist {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(risks: &[Vec<u64>]) -> u64 {
    let size = risks.len();
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    let mut temp_dists: BinaryHeap<PointDist> = BinaryHeap::new();
    let mut dists: HashMap<(usize, usize), u64> = HashMap::new();
    for row in 0..size {
        for col in 0..size {
            let risk = if (row, col) == (0, 0) { 0 } else { u64::MAX };
            unvisited.insert((row, col));
            temp_dists.push(PointDist((row, col), risk));
            dists.insert((row, col), risk);
        }
    }

    while !unvisited.is_empty() {
        let cur = temp_dists.pop().unwrap();
        let (row, col) = cur.0;
        let cur_dist = cur.1;
        if !unvisited.contains(&cur.0) {
            continue;
        }
        unvisited.remove(&cur.0);
        let neighbors = [
            (row + 1, col),
            (row.saturating_sub(1), col),
            (row, col + 1),
            (row, col.saturating_sub(1)),
        ];
        for &(r, c) in neighbors.iter().filter(|pos| unvisited.contains(pos)) {
            dists
                .entry((r, c))
                .and_modify(|dist| *dist = cmp::min(*dist, cur_dist + risks[r][c]));
            temp_dists.push(PointDist((r, c), *dists.get(&(r, c)).unwrap()));
        }
    }

    *dists.get(&(size - 1, size - 1)).unwrap()
}

fn part1(risks: &[Vec<u64>]) -> u64 {
    find_path(risks)
}

fn part2() -> u64 {
    0
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d15.txt");
    let risks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(u64::from)
                        .ok_or_else(|| format!("Invalid digit: {}", c))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&risks);
    let out2 = part2();
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let risks = [
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];
        assert_eq!(part1(&risks), 40);
    }
}

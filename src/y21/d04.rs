use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Board {
    unmarked: HashMap<u8, Pos>,
    marked: HashMap<u8, Pos>,
    width: usize,
    height: usize,
}

impl FromStr for Board {
    type Err = String;

    fn from_str(board: &str) -> Result<Self, Self::Err> {
        let unmarked = board
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(move |(col, val)| (val.parse().unwrap(), Pos { row, col }))
            })
            .collect::<HashMap<_, _>>();
        let width = unmarked.values().map(|pos| pos.col + 1).max().unwrap();
        let height = unmarked.values().map(|pos| pos.row + 1).max().unwrap();
        Ok(Self {
            unmarked,
            marked: HashMap::new(),
            width,
            height,
        })
    }
}

impl Board {
    fn mark(&mut self, v: u8) {
        if let Some(pos) = self.unmarked.remove(&v) {
            self.marked.insert(v, pos);
        }
    }

    fn won(&self) -> bool {
        let rows = self.marked.values().map(|pos| pos.row).counts();
        let cols = self.marked.values().map(|pos| pos.col).counts();
        rows.values().any(|&c| c == self.height) || cols.values().any(|&c| c == self.width)
    }

    fn score(&self) -> u64 {
        self.unmarked.keys().map(|c| u64::from(*c)).sum()
    }
}

fn part1(draws: &[u8], mut boards: Vec<Board>) -> Option<u64> {
    for draw in draws {
        for b in &mut boards {
            b.mark(*draw);
        }
        if let Some(b) = boards.iter().find(|b| b.won()) {
            return Some(b.score() * u64::from(*draw));
        }
    }
    None
}

fn part2(draws: &[u8], mut boards: Vec<Board>) -> Option<u64> {
    for draw in draws {
        for b in &mut boards {
            b.mark(*draw);
        }
        if boards.len() > 1 {
            boards.retain(|b| !b.won());
        }
        if let Some(b) = boards.iter().find(|b| b.won()) {
            return Some(b.score() * u64::from(*draw));
        }
    }
    None
}

#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<String, String> {
    let input = include_str!("input/d04.txt");
    let (draws, boards) = input.split_once("\n\n").unwrap();
    let draws = draws
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Failed to parse draw")?;
    let boards = boards
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let out1 = part1(&draws, boards.clone()).ok_or("No winning board")?;
    let out2 = part2(&draws, boards).ok_or("No winning board")?;
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let draws = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = [
            "22 13 17 11  0\n\
              8  2 23  4 24\n\
             21  9 14 16  7\n\
              6 10  3 18  5\n\
              1 12 20 15 19",
            " 3 15  0  2 22\n\
              9 18 13 17  5\n\
             19  8  7 25 23\n\
             20 11 10 24  4\n\
             14 21 16 12  6",
            "14 21 17 24  4\n\
             10 16 15  9 19\n\
             18  8 23 26 20\n\
             22 11 13  6  5\n\
              2  0 12  3  7",
        ]
        .iter()
        .map(|lines| lines.parse().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(part1(&draws, boards), Some(4512));
    }

    #[test]
    fn test02() {
        let draws = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = [
            "22 13 17 11  0\n\
              8  2 23  4 24\n\
             21  9 14 16  7\n\
              6 10  3 18  5\n\
              1 12 20 15 19",
            " 3 15  0  2 22\n\
              9 18 13 17  5\n\
             19  8  7 25 23\n\
             20 11 10 24  4\n\
             14 21 16 12  6",
            "14 21 17 24  4\n\
             10 16 15  9 19\n\
             18  8 23 26 20\n\
             22 11 13  6  5\n\
              2  0 12  3  7",
        ]
        .iter()
        .map(|lines| lines.parse().unwrap())
        .collect::<Vec<_>>();
        assert_eq!(part2(&draws, boards), Some(1924));
    }
}

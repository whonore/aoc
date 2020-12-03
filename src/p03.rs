use std::ops::Index;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct Slope {
    right: usize,
    down: usize,
    count: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope {
            right,
            down,
            count: 0,
        }
    }
}

impl Iterator for Slope {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.down * self.count;
        let r = self.right * self.count;
        self.count += 1;
        Some((d, r))
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Cell {
    Open,
    Tree,
}
use Cell::*;

#[derive(PartialEq, Eq, Debug)]
struct Grid {
    map: Vec<Vec<Cell>>,
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(map: &str) -> Result<Self, Self::Err> {
        let map = map
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Ok(Open),
                        '#' => Ok(Tree),
                        _ => Err("Invalid cell"),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { map })
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let row = &self.map[idx.0];
        &row[idx.1 % self.width()]
    }
}

fn solve(grid: &Grid, slope: Slope) -> usize {
    slope
        .take_while(|(row, _)| *row < grid.height())
        .map(|(row, col)| &grid[(row, col)])
        .filter(|cell| **cell == Tree)
        .count()
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p03.txt");
    let grid = input.parse()?;
    let out1 = solve(&grid, Slope::new(3, 1));
    let out2 = "";
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        assert_eq!(
            Slope::new(1, 2)
                .take_while(|(row, _)| *row < 5)
                .collect::<Vec<_>>(),
            vec![(0, 0), (2, 1), (4, 2)]
        );
    }

    #[test]
    fn test_wrap() {
        let g: Grid = Grid {
            map: vec![vec![Open, Tree, Open], vec![Tree, Open, Tree]],
        };
        assert_eq!(g[(0, 0)], Open);
        assert_eq!(g[(0, 1)], Tree);
        assert_eq!(g[(0, 2)], Open);
        assert_eq!(g[(0, 3)], Open);
        assert_eq!(g[(1, 0)], Tree);
        assert_eq!(g[(1, 1)], Open);
        assert_eq!(g[(1, 2)], Tree);
        assert_eq!(g[(1, 3)], Tree);
    }

    #[test]
    fn test01() {
        let grid = "..##.......\n\
                    #...#...#..\n\
                    .#....#..#.\n\
                    ..#.#...#.#\n\
                    .#...##..#.\n\
                    ..#.##.....\n\
                    .#.#.#....#\n\
                    .#........#\n\
                    #.##...#...\n\
                    #...##....#\n\
                    .#..#...#.#"
            .parse()
            .unwrap();
        assert_eq!(solve(&grid, Slope::new(3, 1)), 7);
    }
}

use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
use Seat::*;

impl Seat {
    const fn flip(self) -> Self {
        match self {
            Floor => Floor,
            Empty => Occupied,
            Occupied => Empty,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Dir {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}
const DIRS: [Dir; 8] = [
    Dir::NW,
    Dir::N,
    Dir::NE,
    Dir::E,
    Dir::SE,
    Dir::S,
    Dir::SW,
    Dir::W,
];

struct DirIter<'a, A> {
    grid: &'a Vec<Vec<A>>,
    dir: Dir,
    row: usize,
    col: usize,
}

impl<A: Copy> Iterator for DirIter<'_, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Dir::NW => {
                self.row = self.row.checked_sub(1)?;
                self.col = self.col.checked_sub(1)?;
            }
            Dir::N => {
                self.row = self.row.checked_sub(1)?;
            }
            Dir::NE => {
                self.row = self.row.checked_sub(1)?;
                self.col += 1;
            }
            Dir::E => {
                self.col += 1;
            }
            Dir::SE => {
                self.row += 1;
                self.col += 1;
            }
            Dir::S => {
                self.row += 1;
            }
            Dir::SW => {
                self.row += 1;
                self.col = self.col.checked_sub(1)?;
            }
            Dir::W => {
                self.col = self.col.checked_sub(1)?;
            }
        };
        self.grid
            .get(self.row)
            .and_then(|row| row.get(self.col))
            .copied()
    }
}

enum Mode {
    Adj,
    Visible,
}
use Mode::*;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Grid(Vec<Vec<Seat>>);

impl FromStr for Grid {
    type Err = String;

    fn from_str(grid: &str) -> Result<Self, Self::Err> {
        let grid = grid
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Ok(Floor),
                        'L' => Ok(Empty),
                        '#' => Ok(Occupied),
                        _ => Err(format!("Invalid seat {}", c)),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(grid))
    }
}

impl Grid {
    const fn dir_iter(&self, dir: Dir, row: usize, col: usize) -> DirIter<Seat> {
        DirIter {
            grid: &self.0,
            dir,
            row,
            col,
        }
    }

    fn step(&mut self, mode: &Mode) -> bool {
        let mut flip: Vec<(usize, usize)> = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, seat) in row.iter().enumerate() {
                match (seat, mode) {
                    (Empty, Adj) => {
                        if self.count_adj(r, c) == 0 {
                            flip.push((r, c));
                        }
                    }
                    (Occupied, Adj) => {
                        if 4 <= self.count_adj(r, c) {
                            flip.push((r, c));
                        }
                    }
                    (Empty, Visible) => {
                        if self.count_vis(r, c) == 0 {
                            flip.push((r, c));
                        }
                    }
                    (Occupied, Visible) => {
                        if 5 <= self.count_vis(r, c) {
                            flip.push((r, c));
                        }
                    }
                    _ => {}
                }
            }
        }
        for (r, c) in &flip {
            self.0[*r][*c] = self.0[*r][*c].flip();
        }
        !flip.is_empty()
    }

    fn stabilize(&mut self, mode: &Mode) {
        while self.step(mode) {}
    }

    fn count_adj(&self, r: usize, c: usize) -> usize {
        DIRS.iter()
            .filter_map(|dir| self.dir_iter(*dir, r, c).next())
            .filter(|seat| *seat == Occupied)
            .count()
    }

    fn count_vis(&self, r: usize, c: usize) -> usize {
        DIRS.iter()
            .filter_map(|dir| self.dir_iter(*dir, r, c).find(|seat| *seat != Floor))
            .filter(|seat| *seat == Occupied)
            .count()
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|seat| **seat == Occupied).count())
            .sum()
    }
}

fn solve(grid: &mut Grid) -> (usize, usize) {
    let mut grid2 = grid.clone();
    grid.stabilize(&Adj);
    grid2.stabilize(&Visible);
    (grid.count_occupied(), grid2.count_occupied())
}

pub fn run() -> Result<String, String> {
    let input = include_str!("input/p11.txt");
    let mut grid = input.parse::<Grid>()?;
    let (out1, out2) = solve(&mut grid);
    Ok(format!("{} {}", out1, out2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let grid = ".......#.\n\
                    ...#.....\n\
                    .#.......\n\
                    .........\n\
                    ..#L....#\n\
                    ....#....\n\
                    .........\n\
                    #........\n\
                    ...#....."
            .parse::<Grid>()
            .unwrap();
        assert_eq!(grid.count_vis(4, 3), 8);
        let grid = ".............\n\
                    .L.L.#.#.#.#.\n\
                    ............."
            .parse::<Grid>()
            .unwrap();
        assert_eq!(grid.count_vis(1, 1), 0);
        let grid = ".##.##.\n\
                    #.#.#.#\n\
                    ##...##\n\
                    ...L...\n\
                    ##...##\n\
                    #.#.#.#\n\
                    .##.##."
            .parse::<Grid>()
            .unwrap();
        assert_eq!(grid.count_vis(3, 3), 0);
    }

    #[test]
    fn test01() {
        let mut grid = "L.LL.LL.LL\n\
                        LLLLLLL.LL\n\
                        L.L.L..L..\n\
                        LLLL.LL.LL\n\
                        L.LL.LL.LL\n\
                        L.LLLLL.LL\n\
                        ..L.L.....\n\
                        LLLLLLLLLL\n\
                        L.LLLLLL.L\n\
                        L.LLLLL.LL"
            .parse::<Grid>()
            .unwrap();
        assert_eq!(solve(&mut grid), (37, 26));
    }
}
